use super::shell::Shell;
use anyhow::{bail, Context, Error};
use log::{debug, warn};
use rouille::url::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value as Json};
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use ureq::{Agent, RequestBuilder};

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

/// Per-phase request budgets, each settable through the environment.
pub struct Timeouts {
    pub driver: Duration,
    pub startup: Duration,
    pub page_load: Duration,
    pub test: Duration,
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
    timeouts: &Timeouts,
    nocapture: bool,
) -> Result<(), Error> {
    let driver = Driver::find()?;
    let mut drop_log: Box<dyn FnMut()> = Box::new(|| ());
    let driver_url = match driver.location() {
        Locate::Remote(url) => Ok(url.clone()),
        Locate::Local((path, args)) => {
            // Wait for the driver to come online and bind its port before we try to
            // connect to it.
            let start = Instant::now();
            let max = timeouts.driver;

            // Each individual driver spawn gets this long to bind its port
            // before we kill it and retry. This handles drivers that get stuck
            // (e.g. macOS blocking Safari's WebDriver with a permissions dialog)
            // without waiting for the entire `driver_timeout`.
            let per_attempt = Duration::from_secs(10);

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
                let attempt_start = Instant::now();

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
                    } else if attempt_start.elapsed() >= per_attempt {
                        println!(
                            "Driver has not bound port after {}s, restarting ...",
                            per_attempt.as_secs()
                        );
                        break;
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
    client.new_session(&driver, capabilities, timeouts.startup)?;

    let browser_name = client
        .session_browser_name(timeouts.startup)
        .unwrap_or_else(|| driver.browser().to_ascii_lowercase());
    let style_mode = style_mode_for_browser(&browser_name);

    // Visit our local server to open up the page that runs tests.
    //
    // If WASM_BINDGEN_TEST_ADDRESS is set, use it as the local server URL,
    // trying to inherit the port from the server if it isn't specified.
    let mut url = match std::env::var("WASM_BINDGEN_TEST_ADDRESS") {
        Ok(addr) => {
            let mut url = Url::parse(&format!("http://{addr}"))?;
            if url.port().is_none() {
                url.set_port(Some(server.port())).unwrap();
            }
            url
        }
        Err(_) => Url::parse(&format!("http://{server}"))?,
    };
    // The headless template reads this fragment to pick the style.
    let style = match style_mode {
        StyleMode::DisplayNone => "display-none",
        StyleMode::VisibilityHidden => "visibility-hidden",
    };
    url.set_fragment(Some(&format!("wbg_style={style}")));

    shell.status(&format!(
        "Visiting {url} (browser: {browser_name}, sink: append, style: {style_mode:?}, poll: 100ms)..."
    ));
    client.goto(url.as_str(), timeouts.page_load)?;
    shell.status("Loading page elements...");

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
    let max = timeouts.test;
    let no_stream_scrape = env::var_os("WASM_BINDGEN_TEST_NO_STREAM").is_some();
    let mut shell_cleared = false;
    let mut output_buf = String::new();
    let mut output_offset = 0usize;
    while let Some(remaining) = max.checked_sub(start.elapsed()) {
        let budget = remaining.max(MIN_POLL_TIMEOUT);
        if no_stream_scrape {
            let output = client.text_content("#output", 0, budget)?;
            if output.chunk.contains("test result: ") {
                output_buf = output.chunk;
                output_offset = output.next_offset;
                break;
            }
        } else {
            let output = client.text_content("#output", output_offset, budget)?;
            let new_output = output.chunk;
            output_offset = output.next_offset;

            // Print new output as it appears (real-time streaming)
            if !new_output.is_empty() {
                // Clear shell status before first output so they don't mix
                if !shell_cleared {
                    shell.clear();
                    shell_cleared = true;
                }
                io::stdout().lock().write_all(new_output.as_bytes())?;
                output_buf.push_str(&new_output);
            }

            if output_buf.contains("test result: ") {
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    if !shell_cleared {
        shell.clear();
    }

    // Tests have now finished or have timed out. At this point we need to check
    // what happened. In streaming mode output was already printed in real-time.
    // In no-stream mode, emit the buffered output now.
    if no_stream_scrape && !output_buf.is_empty() {
        io::stdout().lock().write_all(output_buf.as_bytes())?;
    }

    // Print any remaining output that might have arrived after the last poll
    let remaining_output = {
        let output = client.text_content("#output", output_offset, timeouts.test)?;
        output.chunk
    };
    if !remaining_output.is_empty() {
        io::stdout().lock().write_all(remaining_output.as_bytes())?;
        output_buf.push_str(&remaining_output);
    }

    if output_buf.contains("test result: ") {
        // If the tests harness finished (either successfully or unsuccessfully)
        // then in theory all the info needed to debug the failure is in its own
        // output, so we shouldn't need the driver logs to get printed.
        drop_log();
    } else {
        println!("Failed to detect test as having been run. It might have timed out.");
    }

    // When --nocapture is active and tests passed, verify that worker console
    // messages were routed to #output (not #console_output). This guards against
    // a scoping regression where the module-loaded run.js can't see the
    // `nocapture` const from the inline classic script.
    if nocapture && output_buf.contains("test result: ok") {
        let console_output = client.text_content("#console_output", 0, timeouts.test)?;
        if !console_output.chunk.is_empty() {
            bail!(
                "with --nocapture, #console_output should be empty but contained:\n{}",
                console_output.chunk
            );
        }
    }

    if !output_buf.contains("test result: ok") {
        // Read console output incrementally to avoid exceeding WebDriver response limits.
        let mut has_output = false;
        let mut offset = 0;
        loop {
            let output = match client.text_content("#console_output", offset, timeouts.test) {
                Ok(output) => output,
                Err(e) => {
                    warn!("failed to read console output {e:?}");
                    break;
                }
            };
            let chunk = output.chunk;
            if chunk.is_empty() {
                break;
            }
            if !has_output {
                println!("console output:");
                has_output = true;
            }
            io::stdout().lock().write_all(tab(&chunk).as_bytes())?;
            offset = output.next_offset;
        }

        bail!("some tests failed")
    }

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum StyleMode {
    DisplayNone,
    VisibilityHidden,
}

fn style_mode_for_browser(browser_name: &str) -> StyleMode {
    let browser = browser_name.to_ascii_lowercase();
    if browser.contains("safari") {
        StyleMode::VisibilityHidden
    } else {
        StyleMode::DisplayNone
    }
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

/// Parse a legacy (JSON Wire Protocol) "new session" response from
/// chromedriver/msedgedriver.
///
/// These drivers reply with HTTP 200 even when session creation fails, encoding
/// the failure in a non-zero `status` field alongside a human-readable
/// `value.message`. We used to read only `sessionId` here, so a failed response
/// (which still carries a placeholder `sessionId`) was treated as a success and
/// every subsequent request 404'd with a baffling `http status: 404`. Instead,
/// surface the driver's own message (e.g. a chromedriver/Chrome version
/// mismatch).
fn parse_legacy_session_response(browser: &str, body: &str) -> Result<String, Error> {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "sessionId")]
        session_id: Option<String>,
        status: Option<i64>,
        value: Option<Json>,
    }

    let response: Response = serde_json::from_str(body)
        .with_context(|| format!("failed to parse {browser} session response: {body}"))?;

    if response.status.unwrap_or(0) != 0 {
        let message = response
            .value
            .as_ref()
            .and_then(|value| value.get("message"))
            .and_then(|message| message.as_str())
            .unwrap_or(body);
        bail!("failed to create a {browser} session: {message}");
    }

    response
        .session_id
        .ok_or_else(|| anyhow::anyhow!("{browser} session response missing `sessionId`: {body}"))
}

struct Client {
    agent: Agent,
    driver_url: Url,
    session: Option<String>,
}

/// Budget for the window close once the outcome is known.
const CLEANUP_TIMEOUT: Duration = Duration::from_secs(5);

/// Floor so the last poll is not cut short by its own deadline.
const MIN_POLL_TIMEOUT: Duration = Duration::from_secs(5);

enum Method<'a> {
    Get,
    Post(&'a str),
    Delete,
}

impl Method<'_> {
    fn verb(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post(_) => "POST",
            Method::Delete => "DELETE",
        }
    }
}

// Below here is a bunch of details of the WebDriver protocol implementation.
// I'm not too familiar with them myself, but these seem to work! I mostly
// copied the `webdriver-client` crate when writing the below bindings.

impl Client {
    fn new_session(
        &mut self,
        driver: &Driver,
        mut cap: Capabilities,
        timeout: Duration,
    ) -> Result<(), Error> {
        let id = match driver {
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
                let x: Response = self.post("/session", &request, timeout)?;
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
                let x: Response = self.post("/session", &request, timeout)?;
                Ok(x.clone()
                    .session_id
                    .or_else(|| x.value.map(|v| v.session_id.unwrap()))
                    .unwrap())
            }
            Driver::Chrome(_) => {
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
                let body = self.post_raw("/session", &request, timeout)?;
                parse_legacy_session_response("Chrome", &body)
            }
            Driver::Edge(_) => {
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
                let body = self.post_raw("/session", &request, timeout)?;
                parse_legacy_session_response("Edge", &body)
            }
        }?;
        self.session = Some(id);
        Ok(())
    }

    fn session_path(&self, suffix: &str) -> Result<String, Error> {
        let id = self
            .session
            .as_deref()
            .context("no active webdriver session")?;
        Ok(format!("/session/{id}{suffix}"))
    }

    fn close_window(&mut self, timeout: Duration) -> Result<(), Error> {
        let Some(id) = self.session.take() else {
            return Ok(());
        };
        #[derive(Deserialize)]
        struct Response {}
        let _: Response = self.delete(&format!("/session/{id}/window"), timeout)?;
        Ok(())
    }

    fn goto(&mut self, url: &str, timeout: Duration) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Request {
            url: String,
        }
        #[derive(Deserialize)]
        struct Response {}

        let request = Request {
            url: url.to_string(),
        };
        let path = self.session_path("/url")?;
        let _: Response = self.post(&path, &request, timeout)?;
        Ok(())
    }

    fn text_content(
        &mut self,
        selector: &str,
        offset: usize,
        timeout: Duration,
    ) -> Result<TextChunk, Error> {
        #[derive(Serialize)]
        struct Request {
            script: String,
            args: Vec<usize>,
        }
        #[derive(Deserialize)]
        struct Response {
            value: serde_json::Value,
        }
        #[derive(Deserialize)]
        struct Value {
            chunk: String,
            next_offset: usize,
        }
        let request = Request {
            script: format!(
                "const el = document.querySelector({}); \
                 if (!el || el.textContent == null) {{ \
                     return {{ chunk: \"\", next_offset: arguments[0] }}; \
                 }} \
                 const text = el.textContent; \
                 const start = Math.min(arguments[0], text.length); \
                 return {{ chunk: text.slice(start), next_offset: text.length }};",
                serde_json::to_string(selector)?
            ),
            args: vec![offset],
        };
        let path = self.session_path("/execute/sync")?;
        let x: Response = self.post(&path, &request, timeout)?;
        match x.value {
            serde_json::Value::Object(_) => {
                let value: Value = serde_json::from_value(x.value)?;
                Ok(TextChunk {
                    chunk: value.chunk,
                    next_offset: value.next_offset,
                })
            }
            serde_json::Value::Null => Ok(TextChunk {
                chunk: String::new(),
                next_offset: offset,
            }),
            other => bail!("unexpected response from execute/sync: {other:?}"),
        }
    }

    fn session_browser_name(&mut self, timeout: Duration) -> Option<String> {
        let path = self.session_path("").ok()?;
        let value: serde_json::Value = match self.get(&path, timeout) {
            Ok(value) => value,
            Err(err) => {
                debug!("failed to read webdriver session capabilities: {err:#}");
                return None;
            }
        };
        value
            .get("value")
            .and_then(|v| {
                v.get("capabilities")
                    .and_then(|caps| caps.get("browserName"))
                    .or_else(|| v.get("browserName"))
            })
            .or_else(|| {
                value
                    .get("capabilities")
                    .and_then(|caps| caps.get("browserName"))
            })
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn get<U>(&mut self, path: &str, timeout: Duration) -> Result<U, Error>
    where
        U: for<'a> Deserialize<'a>,
    {
        debug!("GET {path}");
        let result = self.doit(path, Method::Get, timeout)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn post<T, U>(&mut self, path: &str, data: &T, timeout: Duration) -> Result<U, Error>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
    {
        let input = serde_json::to_string(data)?;
        debug!("POST {path} {input}");
        let result = self.doit(path, Method::Post(&input), timeout)?;
        Ok(serde_json::from_str(&result)?)
    }

    /// Like [`post`](Self::post) but returns the raw response body instead of
    /// deserializing it, so callers can inspect driver-specific error encodings.
    fn post_raw<T>(&mut self, path: &str, data: &T, timeout: Duration) -> Result<String, Error>
    where
        T: Serialize,
    {
        let input = serde_json::to_string(data)?;
        debug!("POST {path} {input}");
        self.doit(path, Method::Post(&input), timeout)
    }

    fn delete<U>(&mut self, path: &str, timeout: Duration) -> Result<U, Error>
    where
        U: for<'a> Deserialize<'a>,
    {
        debug!("DELETE {path}");
        let result = self.doit(path, Method::Delete, timeout)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn doit(&mut self, path: &str, method: Method, timeout: Duration) -> Result<String, Error> {
        let url = endpoint_url(&self.driver_url, path)?;
        let verb = method.verb();
        let fail = |err| request_error(err, verb, path, timeout);
        let mut response = match method {
            Method::Post(data) => with_timeout(self.agent.post(url.as_str()), timeout)
                .content_type("application/json")
                .send(data.as_bytes()),
            Method::Get => with_timeout(self.agent.get(url.as_str()), timeout).call(),
            Method::Delete => with_timeout(self.agent.delete(url.as_str()), timeout).call(),
        }
        .map_err(fail)?;

        let response_code = response.status();
        let result = response.body_mut().read_to_string().map_err(fail)?;

        if response_code != 200 {
            bail!("non-200 response code: {response_code}\n{result}");
        }
        debug!("got: {result}");
        Ok(result)
    }
}

/// Resolves a WebDriver command path such as `/session` against the driver
/// URL, keeping the URL's own path. A remote driver at `http://host/wd/hub`
/// gets `http://host/wd/hub/session`, not `http://host/session`.
fn endpoint_url(driver_url: &Url, path: &str) -> Result<Url, Error> {
    let mut base = driver_url.clone();
    if !base.path().ends_with('/') {
        base.set_path(&format!("{}/", base.path()));
    }
    Ok(base.join(path.trim_start_matches('/'))?)
}

/// Marks a request that ran out of budget.
#[derive(Debug)]
struct WebDriverTimeout(String);

impl fmt::Display for WebDriverTimeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl StdError for WebDriverTimeout {}

fn with_timeout<Any>(builder: RequestBuilder<Any>, timeout: Duration) -> RequestBuilder<Any> {
    builder.config().timeout_global(Some(timeout)).build()
}

fn request_error(err: ureq::Error, verb: &str, path: &str, timeout: Duration) -> Error {
    match err {
        ureq::Error::Timeout(_) => Error::new(WebDriverTimeout(format!(
            "webdriver {verb} {path} timed out after {:.1}s",
            timeout.as_secs_f64()
        ))),
        other => Error::from(other).context(format!("webdriver {verb} {path} request failed")),
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Err(e) = self.close_window(CLEANUP_TIMEOUT) {
            warn!("failed to close window {e:?}");
        }
    }
}

struct TextChunk {
    chunk: String,
    next_offset: usize,
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

/// Cap for reading one driver pipe, which the driver's browser child can hold open.
const STDIO_DUMP_TIMEOUT: Duration = Duration::from_secs(2);

struct BackgroundChild<'a> {
    child: Child,
    stdout: Option<mpsc::Receiver<io::Result<Vec<u8>>>>,
    stderr: Option<mpsc::Receiver<io::Result<Vec<u8>>>>,
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
        let stdout = read_to_channel(child.stdout.take().unwrap());
        let stderr = read_to_channel(child.stderr.take().unwrap());
        Ok(BackgroundChild {
            child,
            stdout: Some(stdout),
            stderr: Some(stderr),
            shell,
            print_stdio_on_drop: true,
        })
    }

    fn has_failed(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(status)) => !status.success(),
            // The child is still running. Stderr output alone does not indicate
            // failure — drivers like ChromeDriver routinely emit warnings (e.g.
            // "[WARNING]: FromSockAddr failed on netmask") during normal startup.
            Ok(None) => false,
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

        dump_stream("stdout", self.stdout.take().unwrap());
        dump_stream("stderr", self.stderr.take().unwrap());
    }
}

fn read_to_channel(mut pipe: impl Read + Send + 'static) -> mpsc::Receiver<io::Result<Vec<u8>>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match pipe.read(&mut buf) {
                Ok(0) => return,
                Ok(n) => {
                    if tx.send(Ok(buf[..n].to_vec())).is_err() {
                        return;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e));
                    return;
                }
            }
        }
    });
    rx
}

/// Drains whatever reached `rx` before the cap, with a notice when the stream did not end.
fn collect_stream(rx: mpsc::Receiver<io::Result<Vec<u8>>>) -> (Vec<u8>, Option<String>) {
    let deadline = Instant::now() + STDIO_DUMP_TIMEOUT;
    let mut bytes = Vec::new();
    loop {
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok(Ok(chunk)) => bytes.extend_from_slice(&chunk),
            Ok(Err(e)) => return (bytes, Some(format!("read failed, {e}"))),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let held = "truncated, a child process still holds the pipe".to_string();
                return (bytes, Some(held));
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return (bytes, None),
        }
    }
}

fn dump_stream(name: &str, rx: mpsc::Receiver<io::Result<Vec<u8>>>) {
    let (bytes, notice) = collect_stream(rx);
    if !bytes.is_empty() {
        println!("driver {name}:\n{}", tab(&String::from_utf8_lossy(&bytes)));
    }
    if let Some(notice) = notice {
        println!("driver {name} {notice}");
    }
}

#[cfg(test)]
mod tests {
    use super::{
        collect_stream, parse_legacy_session_response, read_to_channel, Agent, Client, Error, Url,
        WebDriverTimeout,
    };
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::time::{Duration, Instant};

    const BUDGET: Duration = Duration::from_millis(300);
    const HOLD: Duration = Duration::from_secs(30);
    const SLACK: Duration = Duration::from_secs(3);

    // A real chromedriver 150 response when driving Chrome 149: HTTP 200 with a
    // non-zero JSON Wire Protocol `status` and a placeholder `sessionId`.
    const VERSION_MISMATCH_BODY: &str = r#"{"sessionId":"daa26256195a0a06927e6d7f7b0cfcfc","status":33,"value":{"message":"session not created: This version of ChromeDriver only supports Chrome version 150\nCurrent browser version is 149.0.7827.155 with binary path /opt/google/chrome/chrome"}}"#;

    #[test]
    fn surfaces_driver_error_message() {
        let err = parse_legacy_session_response("Chrome", VERSION_MISMATCH_BODY)
            .expect_err("a non-zero status response must be treated as an error");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("only supports Chrome version 150"),
            "error should surface the driver's own message, got: {msg}"
        );
        assert!(
            msg.contains("149.0.7827.155"),
            "error should include the current browser version, got: {msg}"
        );
    }

    #[test]
    fn returns_session_id_on_success() {
        let body = r#"{"sessionId":"abc123","status":0,"value":{}}"#;
        let id = parse_legacy_session_response("Chrome", body).expect("success should parse");
        assert_eq!(id, "abc123");
    }

    fn bound_listener() -> (TcpListener, Url) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, Url::parse(&format!("http://{addr}/")).unwrap())
    }

    fn client_for(url: Url) -> Client {
        Client {
            agent: Agent::new_with_defaults(),
            driver_url: url,
            session: None,
        }
    }

    fn stalled_driver() -> Url {
        let (listener, url) = bound_listener();
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                thread::spawn(move || {
                    let _stream = stream;
                    thread::sleep(HOLD);
                });
            }
        });
        url
    }

    fn one_shot_driver(response: Vec<u8>) -> Url {
        let (listener, url) = bound_listener();
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            let _ = stream.write_all(&response);
            thread::sleep(HOLD);
        });
        url
    }

    fn timeout_error(url: Url, request: impl FnOnce(&mut Client) -> Result<(), Error>) -> Error {
        let mut client = client_for(url);
        let start = Instant::now();
        let err = request(&mut client).expect_err("the command must fail");
        assert!(start.elapsed() < SLACK, "the request outlived its timeout");
        assert!(
            err.downcast_ref::<WebDriverTimeout>().is_some(),
            "stall must be marked as a timeout, got: {err:#}"
        );
        err
    }

    /// Answers one request with `{}` and returns the request target it saw.
    fn requested_path(base: &str, request: impl FnOnce(&mut Client)) -> String {
        let (listener, url) = bound_listener();
        let base = url.join(base).unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap();
            let head = String::from_utf8_lossy(&buf[..n]).into_owned();
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\n{}");
            head.split(' ').nth(1).unwrap().to_string()
        });
        let mut client = client_for(base);
        request(&mut client);
        server.join().unwrap()
    }

    #[test]
    fn remote_url_path_is_kept() {
        let cases = [
            ("/", "/session"),
            ("/wd/hub/", "/wd/hub/session"),
            ("/wd/hub", "/wd/hub/session"),
        ];
        for (base, expected) in cases {
            let path = requested_path(base, |c| {
                c.get::<serde_json::Value>("/session", BUDGET).unwrap();
            });
            assert_eq!(path, expected, "driver URL path `{base}`");
        }

        let path = requested_path("/wd/hub/", |c| {
            c.session = Some("abc".into());
            c.close_window(BUDGET).unwrap();
        });
        assert_eq!(path, "/wd/hub/session/abc/window");
    }

    #[test]
    fn stalled_get_hits_the_request_timeout() {
        let err = timeout_error(stalled_driver(), |c| {
            c.get::<serde_json::Value>("/session", BUDGET).map(|_| ())
        });
        let msg = format!("{err:#}");
        assert!(
            msg.contains("timed out") && msg.contains("/session"),
            "error must name timeout and request, got: {msg}"
        );
    }

    #[test]
    fn stalled_post_hits_the_request_timeout() {
        timeout_error(stalled_driver(), |c| {
            c.post::<_, serde_json::Value>(
                "/session",
                &serde_json::json!({"capabilities": {}}),
                BUDGET,
            )
            .map(|_| ())
        });
    }

    #[test]
    fn stalled_response_body_hits_the_request_timeout() {
        let url = one_shot_driver(b"HTTP/1.1 200 OK\r\ncontent-length: 1000\r\n\r\n".to_vec());
        timeout_error(url, |c| {
            c.get::<serde_json::Value>("/session/x/execute/sync", BUDGET)
                .map(|_| ())
        });
    }

    #[test]
    fn failed_close_consumes_the_session() {
        let mut client = client_for(stalled_driver());
        client.session = Some("deadbeef".into());
        let err = client
            .close_window(BUDGET)
            .expect_err("a stalled close must fail");
        assert!(err.downcast_ref::<WebDriverTimeout>().is_some());
        let start = Instant::now();
        assert!(client.close_window(BUDGET).is_ok());
        drop(client);
        assert!(
            start.elapsed() < Duration::from_secs(1),
            "consumed session must not re-request"
        );
    }

    #[test]
    fn answered_close_failure_is_not_a_timeout() {
        let body = r#"{"value":{"error":"unable to close window","message":"refused"}}"#;
        let response = format!(
            "HTTP/1.1 500 Internal Server Error\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
            body.len()
        );
        let mut client = client_for(one_shot_driver(response.into_bytes()));
        client.session = Some("deadbeef".into());
        let err = client
            .close_window(BUDGET)
            .expect_err("a 500 close must fail");
        assert!(
            err.downcast_ref::<WebDriverTimeout>().is_none(),
            "answered close is not a stall, got: {err:#}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn drop_does_not_wait_for_a_grandchild_holding_the_pipes() {
        use super::{BackgroundChild, Shell};
        use std::path::Path;
        use std::process::Command;

        let shell = Shell::new();
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg("sleep 10 & exit");
        let child = BackgroundChild::spawn(Path::new("/bin/sh"), &mut cmd, &shell).unwrap();
        let start = Instant::now();
        drop(child);
        assert!(
            start.elapsed() < Duration::from_secs(6),
            "drop waited on stdio held open past the dump cap"
        );
    }

    #[test]
    fn a_held_pipe_still_yields_what_arrived() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.write_all(b"driver said hello").unwrap();
            thread::sleep(HOLD);
        });
        let (bytes, notice) = collect_stream(read_to_channel(TcpStream::connect(addr).unwrap()));
        assert_eq!(bytes, b"driver said hello");
        assert!(notice.is_some(), "a held pipe must be reported");
    }
}
