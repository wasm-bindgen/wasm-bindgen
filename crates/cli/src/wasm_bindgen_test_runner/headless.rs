use super::shell::Shell;
use anyhow::{bail, format_err, Context, Error};
use log::{debug, warn};
use rouille::url::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value as Json};
use std::env;
use std::fs::File;
use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use ureq::Agent;

/// Options that can use to customize and configure a WebDriver session.
type Capabilities = Map<String, Json>;

/// Wrapper for [`Capabilities`] used in `--w3c` mode.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecNewSessionParameters {
    #[serde(rename = "alwaysMatch", default = "Capabilities::default")]
    pub always_match: Capabilities,
    #[serde(rename = "firstMatch", default = "first_match_default")]
    pub first_match: Vec<Capabilities>,
}

impl Default for SpecNewSessionParameters {
    fn default() -> Self {
        Self {
            always_match: Capabilities::new(),
            first_match: vec![Capabilities::new()],
        }
    }
}

fn first_match_default() -> Vec<Capabilities> {
    vec![Capabilities::default()]
}

/// Wrapper for [`Capabilities`] used in `--legacy` mode.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacyNewSessionParameters {
    #[serde(rename = "desiredCapabilities", default = "Capabilities::default")]
    pub desired: Capabilities,
    #[serde(rename = "requiredCapabilities", default = "Capabilities::default")]
    pub required: Capabilities,
}

/// Execute a headless browser tests against a server running on `server`
/// address.
///
/// This function will take care of everything from spawning the WebDriver
/// binary, controlling it, running tests, scraping output, displaying output,
/// etc. It will return `Ok` if all tests finish successfully, and otherwise it
/// will return an error if some tests failed.
pub fn run(
    server: &SocketAddr,
    shell: &Shell,
    driver_timeout: u64,
    test_timeout: u64,
) -> Result<(), Error> {
    let driver = Driver::find()?;
    let mut drop_log: Box<dyn FnMut()> = Box::new(|| ());
    let driver_url = match driver.location() {
        Locate::Remote(url) => Ok(url.clone()),
        Locate::Local((path, args)) => {
            // Wait for the driver to come online and bind its port before we try to
            // connect to it.
            let start = Instant::now();
            let max = Duration::new(driver_timeout, 0);

            let (driver_addr, mut child) = 'outer: loop {
                // Allow tests to run in parallel (in theory) by finding any open port
                // available for our driver. We can't bind the port for the driver, but
                // hopefully the OS gives this invocation unique ports across processes
                let driver_addr = TcpListener::bind("127.0.0.1:0")?.local_addr()?;
                // Spawn the driver binary, collecting its stdout/stderr in separate
                // threads. We'll print this output later.
                let mut cmd = Command::new(path);
                cmd.args(args).arg(format!("--port={}", driver_addr.port()));
                let mut child = BackgroundChild::spawn(path, &mut cmd, shell)?;

                // Wait for the driver to come online and bind its port before we try to
                // connect to it.
                loop {
                    if child.has_failed() {
                        if start.elapsed() >= max {
                            bail!("driver failed to start")
                        }

                        println!("Failed to start driver, trying again ...");

                        thread::sleep(Duration::from_millis(100));
                        break;
                    } else if TcpStream::connect(driver_addr).is_ok() {
                        break 'outer (driver_addr, child);
                    } else if start.elapsed() >= max {
                        bail!("driver failed to bind port during startup")
                    } else {
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            };

            drop_log = Box::new(move || {
                let _ = &child;
                child.print_stdio_on_drop = false;
            });

            Url::parse(&format!("http://{driver_addr}")).map_err(Error::from)
        }
    }?;
    println!(
        "Running headless tests in {} on `{}`",
        driver.browser(),
        driver_url.as_str(),
    );

    let mut client = Client {
        agent: Agent::new_with_defaults(),
        driver_url,
        session: None,
    };
    println!("Try find `webdriver.json` for configure browser's capabilities:");
    let capabilities: Capabilities = match File::open(
        std::env::var("WASM_BINDGEN_TEST_WEBDRIVER_JSON").unwrap_or("webdriver.json".to_string()),
    ) {
        Ok(file) => {
            println!("Ok");
            serde_json::from_reader(file)
        }
        Err(_) => {
            println!("Not found");
            Ok(Capabilities::new())
        }
    }?;
    shell.status("Starting new webdriver session...");
    // Allocate a new session with the webdriver protocol, and once we've done
    // so schedule the browser to get closed with a call to `close_window`.
    let id = client.new_session(&driver, capabilities)?;
    client.session = Some(id.clone());

    // Visit our local server to open up the page that runs tests, and then get
    // some handles to objects on the page which we'll be scraping output from.
    //
    // If WASM_BINDGEN_TEST_ADDRESS is set, use it as the local server URL,
    // trying to inherit the port from the server if it isn't specified.
    let url = match std::env::var("WASM_BINDGEN_TEST_ADDRESS") {
        Ok(u) => {
            let mut url = Url::parse(&u)?;
            if url.port().is_none() {
                url.set_port(Some(server.port())).unwrap();
            }
            url.to_string()
        }
        Err(_) => format!("http://{server}"),
    };

    shell.status(&format!("Visiting {url}..."));
    client.goto(&id, &url)?;
    shell.status("Loading page elements...");
    let output = client.element(&id, "#output")?;
    let logs = client.element(&id, "#console_log")?;
    let errors = client.element(&id, "#console_error")?;

    // At this point we need to wait for the test to finish before we can take a
    // look at what happened. There appears to be no great way to do this with
    // the webdriver protocol today (in terms of synchronization), so for now we
    // just go with a loop.
    //
    // We periodically check the page to see if the output contains a known
    // string to only be printed when tests have finished running.
    //
    // TODO: harness anyhows aren't well handled here, they always force a
    //       timeout. These sorts of anyhows could be "you typo'd the path to a
    //       local script" which is pretty bad to time out for, we should detect
    //       this on the page and look for such output here, printing diagnostic
    //       information.
    shell.status("Waiting for test to finish...");
    let start = Instant::now();
    let max = Duration::new(test_timeout, 0);
    while start.elapsed() < max {
        if client.text(&id, &output)?.contains("test result: ") {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    shell.clear();

    // Tests have now finished or have timed out. At this point we need to print
    // what happened on the console. Currently we just do this by scraping the
    // output of various fields and printing them out, hopefully providing
    // enough diagnostic info to see what went wrong (if anything).
    let output = client.text(&id, &output)?;
    let logs = client.text(&id, &logs)?;
    let errors = client.text(&id, &errors)?;

    if output.contains("test result: ") {
        println!("{output}");

        // If the tests harness finished (either successfully or unsuccessfully)
        // then in theory all the info needed to debug the failure is in its own
        // output, so we shouldn't need the driver logs to get printed.
        drop_log();
    } else {
        println!("Failed to detect test as having been run. It might have timed out.");
        if !output.is_empty() {
            println!("output div contained:\n{}", tab(&output));
        }
    }

    if !output.contains("test result: ok") {
        if !logs.is_empty() {
            println!("console.log div contained:\n{}", tab(&logs));
        }
        if !errors.is_empty() {
            println!("console.log div contained:\n{}", tab(&errors));
        }

        bail!("some tests failed")
    }

    Ok(())
}

enum Driver {
    Gecko(Locate),
    Safari(Locate),
    Chrome(Locate),
    Edge(Locate),
}

enum Locate {
    Local((PathBuf, Vec<String>)),
    Remote(Url),
}

impl Driver {
    /// Attempts to find an appropriate remote WebDriver server or server binary
    /// to execute tests with.
    /// Performs a number of heuristics to find one available, including:
    ///
    /// * Env vars like `GECKODRIVER_REMOTE` address of remote webdriver.
    /// * Env vars like `GECKODRIVER` point to the path to a binary to execute.
    /// * Otherwise, `PATH` is searched for an appropriate binary.
    ///
    /// In the last two cases a list of auxiliary arguments is also returned
    /// which is configured through env vars like `GECKODRIVER_ARGS` to support
    /// extra arguments to the driver's invocation.
    fn find() -> Result<Driver, Error> {
        let env_args = |name: &str| {
            let var = env::var(format!("{}_ARGS", name.to_uppercase())).unwrap_or_default();

            shlex::split(&var)
                .unwrap_or_else(|| var.split_whitespace().map(|s| s.to_string()).collect())
        };

        let drivers = [
            ("geckodriver", Driver::Gecko as fn(Locate) -> Driver),
            ("safaridriver", Driver::Safari as fn(Locate) -> Driver),
            ("chromedriver", Driver::Chrome as fn(Locate) -> Driver),
            ("msedgedriver", Driver::Edge as fn(Locate) -> Driver),
        ];

        // First up, if env vars like GECKODRIVER_REMOTE are present, use those
        // to allow forcing usage of a particular remote driver.
        for (driver, ctor) in drivers.iter() {
            let env = format!("{}_REMOTE", driver.to_uppercase());
            let url = match env::var(&env) {
                Ok(var) => Url::parse(&var).context(format!("failed to parse `{env}`"))?,
                Err(_) => continue,
            };
            return Ok(ctor(Locate::Remote(url)));
        }

        // Next, if env vars like GECKODRIVER are present, use those to
        // allow forcing usage of a particular local driver.
        for (driver, ctor) in drivers.iter() {
            let env = driver.to_uppercase();
            let path = match env::var_os(&env) {
                Some(path) => path,
                None => continue,
            };
            return Ok(ctor(Locate::Local((path.into(), env_args(driver)))));
        }

        // Next, check PATH. If we can find any supported driver, use that by
        // default.
        for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
            let found = drivers.iter().find(|(name, _)| {
                path.join(name)
                    .with_extension(env::consts::EXE_EXTENSION)
                    .exists()
            });
            let (driver, ctor) = match found {
                Some(p) => p,
                None => continue,
            };
            return Ok(ctor(Locate::Local((driver.into(), env_args(driver)))));
        }

        // TODO: download an appropriate driver? How to know which one to
        //       download?

        bail!(
            "\
failed to find a suitable WebDriver binary or remote running WebDriver to drive
headless testing; to configure the location of the webdriver binary you can use
environment variables like `GECKODRIVER=/path/to/geckodriver` or make sure that
the binary is in `PATH`; to configure the address of remote webdriver you can
use environment variables like `GECKODRIVER_REMOTE=http://remote.host/`

This crate currently supports `geckodriver`, `chromedriver`, `safaridriver`, and
`msedgedriver`, although more driver support may be added! You can download these at:

    * geckodriver - https://github.com/mozilla/geckodriver/releases
    * chromedriver - https://chromedriver.chromium.org/downloads
    * msedgedriver - https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/
    * safaridriver - should be preinstalled on OSX

If you would prefer to not use headless testing and would instead like to do
interactive testing in a web browser then you can specify `NO_HEADLESS=1` as
an environment variable. When rerun the tests will start a server that you can
visit in a web browser, and headless testing should not be used.

If you're still having difficulty resolving this error, please feel free to open
an issue against wasm-bindgen/wasm-bindgen!
    "
        )
    }

    fn browser(&self) -> &str {
        match self {
            Driver::Gecko(_) => "Firefox",
            Driver::Safari(_) => "Safari",
            Driver::Chrome(_) => "Chrome",
            Driver::Edge(_) => "Edge",
        }
    }

    fn location(&self) -> &Locate {
        match self {
            Driver::Gecko(locate) => locate,
            Driver::Safari(locate) => locate,
            Driver::Chrome(locate) => locate,
            Driver::Edge(locate) => locate,
        }
    }
}

struct Client {
    agent: Agent,
    driver_url: Url,
    session: Option<String>,
}

enum Method<'a> {
    Get,
    Post(&'a str),
    Delete,
}

// Below here is a bunch of details of the WebDriver protocol implementation.
// I'm not too familiar with them myself, but these seem to work! I mostly
// copied the `webdriver-client` crate when writing the below bindings.

impl Client {
    fn new_session(&mut self, driver: &Driver, mut cap: Capabilities) -> Result<String, Error> {
        match driver {
            Driver::Gecko(_) => {
                #[derive(Deserialize)]
                struct Response {
                    value: ResponseValue,
                }

                #[derive(Deserialize)]
                struct ResponseValue {
                    #[serde(rename = "sessionId")]
                    session_id: String,
                }
                cap.entry("moz:firefoxOptions".to_string())
                    .or_insert_with(|| Json::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .expect("moz:firefoxOptions wasn't a JSON object")
                    .entry("args".to_string())
                    .or_insert_with(|| Json::Array(vec![]))
                    .as_array_mut()
                    .expect("args wasn't a JSON array")
                    .extend(vec![Json::String("-headless".to_string())]);
                let session_config = SpecNewSessionParameters {
                    always_match: cap,
                    first_match: vec![Capabilities::new()],
                };
                let request = json!({
                    "capabilities": session_config,
                });
                let x: Response = self.post("/session", &request)?;
                Ok(x.value.session_id)
            }
            Driver::Safari(_) => {
                #[derive(Clone, Deserialize)]
                struct Response {
                    // returned by `--legacy` or by default on High Sierra and lower.
                    #[serde(rename = "sessionId")]
                    session_id: Option<String>,
                    // returned by the now-default `--w3c` mode
                    value: Option<Value>,
                }
                #[derive(Clone, Deserialize)]
                struct Value {
                    // This needs to be optional because both `--legacy` and High Sierra do not
                    // include a session id in the value entry.
                    #[serde(rename = "sessionId")]
                    session_id: Option<String>,
                }
                let request = json!({
                    // this is needed for the now `--legacy` mode
                    "desiredCapabilities": {
                    },
                    // this is needed for the now `--w3c` (default) mode
                    "capabilities": {
                    }
                });
                let x: Response = self.post("/session", &request)?;
                Ok(x.clone()
                    .session_id
                    .or_else(|| x.value.map(|v| v.session_id.unwrap()))
                    .unwrap())
            }
            Driver::Chrome(_) => {
                #[derive(Deserialize)]
                struct Response {
                    #[serde(rename = "sessionId")]
                    session_id: String,
                }
                cap.entry("goog:chromeOptions".to_string())
                    .or_insert_with(|| Json::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .expect("goog:chromeOptions wasn't a JSON object")
                    .entry("args".to_string())
                    .or_insert_with(|| Json::Array(vec![]))
                    .as_array_mut()
                    .expect("args wasn't a JSON array")
                    .extend(vec![
                        Json::String("headless".to_string()),
                        // See https://stackoverflow.com/questions/50642308/
                        // for what this funky `disable-dev-shm-usage`
                        // option is
                        Json::String("disable-dev-shm-usage".to_string()),
                        Json::String("no-sandbox".to_string()),
                    ]);
                let request = LegacyNewSessionParameters {
                    desired: cap,
                    required: Capabilities::new(),
                };
                let x: Response = self.post("/session", &request)?;
                Ok(x.session_id)
            }
            Driver::Edge(_) => {
                #[derive(Deserialize)]
                struct Response {
                    #[serde(rename = "sessionId")]
                    session_id: String,
                }
                cap.entry("ms:edgeOptions".to_string())
                    .or_insert_with(|| Json::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .expect("ms:edgeOptions wasn't a JSON object")
                    .entry("args".to_string())
                    .or_insert_with(|| Json::Array(vec![]))
                    .as_array_mut()
                    .expect("args wasn't a JSON array")
                    .extend(vec![
                        Json::String("headless".to_string()),
                        // See https://stackoverflow.com/questions/50642308/
                        // for what this funky `disable-dev-shm-usage`
                        // option is
                        Json::String("disable-dev-shm-usage".to_string()),
                        Json::String("no-sandbox".to_string()),
                    ]);
                let request = LegacyNewSessionParameters {
                    desired: cap,
                    required: Capabilities::new(),
                };
                let x: Response = self.post("/session", &request)?;
                Ok(x.session_id)
            }
        }
    }

    fn close_window(&mut self, id: &str) -> Result<(), Error> {
        #[derive(Deserialize)]
        struct Response {}
        let _: Response = self.delete(&format!("/session/{id}/window"))?;
        Ok(())
    }

    fn goto(&mut self, id: &str, url: &str) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Request {
            url: String,
        }
        #[derive(Deserialize)]
        struct Response {}

        let request = Request {
            url: url.to_string(),
        };
        let _: Response = self.post(&format!("/session/{id}/url"), &request)?;
        Ok(())
    }

    fn element(&mut self, id: &str, selector: &str) -> Result<String, Error> {
        #[derive(Serialize)]
        struct Request {
            using: String,
            value: String,
        }
        #[derive(Deserialize)]
        struct Response {
            value: Reference,
        }
        #[derive(Deserialize)]
        struct Reference {
            #[serde(rename = "element-6066-11e4-a52e-4f735466cecf")]
            gecko_reference: Option<String>,
            #[serde(rename = "ELEMENT")]
            safari_reference: Option<String>,
        }

        let request = Request {
            using: "css selector".to_string(),
            value: selector.to_string(),
        };
        let x: Response = self.post(&format!("/session/{id}/element"), &request)?;
        x.value
            .gecko_reference
            .or(x.value.safari_reference)
            .ok_or(format_err!("failed to find element reference in response"))
    }

    fn text(&mut self, id: &str, element: &str) -> Result<String, Error> {
        #[derive(Deserialize)]
        struct Response {
            value: String,
        }
        let x: Response = self.get(&format!("/session/{id}/element/{element}/text"))?;
        Ok(x.value)
    }

    fn get<U>(&mut self, path: &str) -> Result<U, Error>
    where
        U: for<'a> Deserialize<'a>,
    {
        debug!("GET {path}");
        let result = self.doit(path, Method::Get)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn post<T, U>(&mut self, path: &str, data: &T) -> Result<U, Error>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
    {
        let input = serde_json::to_string(data)?;
        debug!("POST {path} {input}");
        let result = self.doit(path, Method::Post(&input))?;
        Ok(serde_json::from_str(&result)?)
    }

    fn delete<U>(&mut self, path: &str) -> Result<U, Error>
    where
        U: for<'a> Deserialize<'a>,
    {
        debug!("DELETE {path}");
        let result = self.doit(path, Method::Delete)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn doit(&mut self, path: &str, method: Method) -> Result<String, Error> {
        let url = self.driver_url.join(path)?;
        let mut response = match method {
            Method::Post(data) => self
                .agent
                .post(url.as_str())
                .content_type("application/json")
                .send(data.as_bytes())?,
            Method::Delete => self.agent.delete(url.as_str()).call()?,
            Method::Get => self.agent.get(url.as_str()).call()?,
        };

        let response_code = response.status();
        let result = response.body_mut().read_to_string()?;

        if response_code != 200 {
            bail!("non-200 response code: {response_code}\n{result}");
        }
        debug!("got: {result}");
        Ok(result)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let id = match &self.session {
            Some(id) => id.clone(),
            None => return,
        };
        if let Err(e) = self.close_window(&id) {
            warn!("failed to close window {e:?}");
        }
    }
}

fn tab(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        result.push_str("    ");
        result.push_str(line);
        result.push('\n');
    }
    result
}

struct BackgroundChild<'a> {
    child: Child,
    stdout: Option<thread::JoinHandle<io::Result<Vec<u8>>>>,
    stderr: Option<thread::JoinHandle<io::Result<Vec<u8>>>>,
    any_stderr: Arc<AtomicBool>,
    shell: &'a Shell,
    print_stdio_on_drop: bool,
}

impl<'a> BackgroundChild<'a> {
    fn spawn(
        path: &Path,
        cmd: &mut Command,
        shell: &'a Shell,
    ) -> Result<BackgroundChild<'a>, Error> {
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());
        log::debug!("executing {cmd:?}");
        let mut child = cmd
            .spawn()
            .context(format!("failed to spawn {path:?} binary"))?;
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        let stdout = Some(thread::spawn(move || {
            let mut dst = Vec::new();
            stdout.read_to_end(&mut dst)?;
            Ok(dst)
        }));
        let any_stderr = Arc::new(AtomicBool::new(false));
        let any_stderr_clone = Arc::clone(&any_stderr);
        let stderr = Some(thread::spawn(move || {
            let mut dst = Cursor::new(Vec::new());
            let mut buffer = [0];

            match stderr.read_exact(&mut buffer) {
                Ok(()) => {
                    dst.write_all(&buffer).unwrap();
                    any_stderr_clone.store(true, Ordering::Relaxed);
                }
                Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                    return Ok(dst.into_inner())
                }
                Err(error) => return Err(error),
            }

            io::copy(&mut stderr, &mut dst)?;
            Ok(dst.into_inner())
        }));
        Ok(BackgroundChild {
            child,
            stdout,
            stderr,
            any_stderr,
            shell,
            print_stdio_on_drop: true,
        })
    }

    fn has_failed(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(status)) => !status.success(),
            Ok(None) => self.any_stderr.load(Ordering::Relaxed),
            Err(_) => true,
        }
    }
}

impl Drop for BackgroundChild<'_> {
    fn drop(&mut self) {
        self.child.kill().unwrap();
        let status = self.child.wait().unwrap();
        if !self.print_stdio_on_drop {
            return;
        }

        self.shell.clear();
        println!("driver status: {status}");

        let stdout = self.stdout.take().unwrap().join().unwrap().unwrap();
        if !stdout.is_empty() {
            println!("driver stdout:\n{}", tab(&String::from_utf8_lossy(&stdout)));
        }
        let stderr = self.stderr.take().unwrap().join().unwrap().unwrap();
        if !stderr.is_empty() {
            println!("driver stderr:\n{}", tab(&String::from_utf8_lossy(&stderr)));
        }
    }
}
