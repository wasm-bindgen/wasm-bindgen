use crate::Bindgen;
use anyhow::{bail, Context};
use std::path::{Path, PathBuf};
use std::thread;
use std::{env, fs};

pub mod deno;
pub mod headless;
pub mod node;
pub mod server;
pub mod shell;

pub use walrus;

pub struct Tests {
    pub tests: Vec<Test>,
    pub filtered: usize,
}

impl Tests {
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            filtered: 0,
        }
    }

    fn as_args(&self, include_ignored: bool) -> String {
        let filtered = self.filtered;

        format!(
            r#"
            // Forward runtime arguments.
            cx.include_ignored({include_ignored:?});
            cx.filtered_count({filtered});
        "#
        )
    }
}

pub struct Test {
    /// The test name.
    pub name: String,

    /// Symbol name.
    pub export: String,

    /// Whether or not the test should be ignored.
    pub ignored: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TestMode {
    Node { no_modules: bool },
    Deno,
    Browser { no_modules: bool },
    DedicatedWorker { no_modules: bool },
    SharedWorker { no_modules: bool },
    ServiceWorker { no_modules: bool },
}

impl TestMode {
    pub fn is_worker(self) -> bool {
        matches!(
            self,
            Self::DedicatedWorker { .. } | Self::SharedWorker { .. } | Self::ServiceWorker { .. }
        )
    }

    pub fn no_modules(self) -> bool {
        match self {
            Self::Deno => true,
            Self::Browser { no_modules }
            | Self::Node { no_modules }
            | Self::DedicatedWorker { no_modules }
            | Self::SharedWorker { no_modules }
            | Self::ServiceWorker { no_modules } => no_modules,
        }
    }
}

/// Possible values for the `--format` option.
#[derive(Debug, Clone, Copy)]
pub enum OutputFormatSetting {
    /// Display one character per test
    Terse,
}

#[derive(Debug)]
pub struct TestRunner {
    /// The file to test. `cargo test` passes this argument for you.
    file: PathBuf,

    /// Run ignored tests
    include_ignored: bool,

    /// Run ignored tests
    ignored: bool,

    /// Exactly match filters rather than by substring
    exact: bool,

    /// Skip tests whose names contain FILTER (this flag can be used multiple times)
    skip: Vec<String>,

    /// List all tests and benchmarks
    list: bool,

    /// Don't capture `console.*()` of each task, allow printing directly.
    nocapture: bool,

    /// Configure formatting of output.
    format: Option<OutputFormatSetting>,

    /// The FILTER string is tested against the name of all tests, and only those tests whose names contain the filter are run.
    filter: Option<String>,
}

impl TestRunner {
    /// Constructor for the [TestRunner].
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            include_ignored: false,
            ignored: false,
            exact: false,
            skip: Vec::new(),
            list: false,
            nocapture: false,
            format: None,
            filter: None,
        }
    }

    pub fn with_include_ignored(mut self) -> anyhow::Result<Self> {
        if self.ignored {
            bail!("`--ignored` is mutually exclusive with `--include-ignored`");
        }
        self.include_ignored = true;
        Ok(self)
    }

    pub fn with_ignored(mut self) -> anyhow::Result<Self> {
        if self.include_ignored {
            bail!("`--ignored` is mutually exclusive with `--include-ignored`");
        }
        self.ignored = true;
        Ok(self)
    }

    pub fn with_exact(mut self) -> Self {
        self.exact = true;
        self
    }

    pub fn with_skip<S: Into<String>>(mut self, filter: S) -> Self {
        self.skip.push(filter.into());
        self
    }

    pub fn with_list(mut self) -> Self {
        self.list = true;
        self
    }

    pub fn with_nocapture(mut self) -> Self {
        self.nocapture = true;
        self
    }

    pub fn with_format(mut self, format: OutputFormatSetting) -> Self {
        self.format = Some(format);
        self
    }

    pub fn with_filter<S: Into<String>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let shell = shell::Shell::new();

        let file_name = self
            .file
            .file_name()
            .map(Path::new)
            .context("file to test is not a valid file, can't extract file name")?;

        // Collect all tests that the test harness is supposed to run. We assume
        // that any exported function with the prefix `__wbg_test` is a test we need
        // to execute.
        let wasm = fs::read(&self.file).context("failed to read Wasm file")?;
        let mut wasm = walrus::ModuleConfig::new()
            // generate dwarf by default, it can be controlled by debug profile
            //
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            .generate_dwarf(true)
            .parse(&wasm)
            .context("failed to deserialize Wasm module")?;
        let mut tests = Tests::new();

        'outer: for export in wasm.exports.iter() {
            let Some(name) = export.name.strip_prefix("__wbgt_") else {
                continue;
            };
            let modifiers = name.split_once('_').expect("found invalid identifier").0;

            let Some(name) = export.name.split_once("::").map(|s| s.1) else {
                continue;
            };

            let test = Test {
                name: name.into(),
                export: export.name.clone(),
                ignored: modifiers.contains('$'),
            };

            if let Some(filter) = &self.filter {
                let matches = if self.exact {
                    name == *filter
                } else {
                    name.contains(filter)
                };

                if !matches {
                    tests.filtered += 1;
                    continue;
                }
            }

            for skip in &self.skip {
                let matches = if self.exact {
                    name == *skip
                } else {
                    name.contains(skip)
                };

                if matches {
                    tests.filtered += 1;
                    continue 'outer;
                }
            }

            if !test.ignored && self.ignored {
                tests.filtered += 1;
            } else {
                tests.tests.push(test);
            }
        }

        if self.list {
            for test in tests.tests {
                println!("{}: test", test.name);
            }

            // Returning cleanly has the strange effect of outputting
            // an additional empty line with spaces in it.
            std::process::exit(0);
        }

        let tmpdir = tempfile::tempdir()?;

        let module = "wasm-bindgen-test";

        // Right now there's a bug where if no tests are present then the
        // `wasm-bindgen-test` runtime support isn't linked in, so just bail out
        // early saying everything is ok.
        if tests.tests.is_empty() {
            println!("no tests to run!");
            return Ok(());
        }

        // Figure out if this tests is supposed to execute in node.js or a browser.
        // That's done on a per-test-binary basis with the
        // `wasm_bindgen_test_configure` macro, which emits a custom section for us
        // to read later on.

        let custom_section = wasm.customs.remove_raw("__wasm_bindgen_test_unstable");
        let no_modules = std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok();
        let test_mode = match custom_section {
            Some(section) if section.data.contains(&0x01) => TestMode::Browser { no_modules },
            Some(section) if section.data.contains(&0x02) => {
                TestMode::DedicatedWorker { no_modules }
            }
            Some(section) if section.data.contains(&0x03) => TestMode::SharedWorker { no_modules },
            Some(section) if section.data.contains(&0x04) => TestMode::ServiceWorker { no_modules },
            Some(section) if section.data.contains(&0x05) => TestMode::Node { no_modules },
            Some(_) => bail!("invalid __wasm_bingen_test_unstable value"),
            None => {
                let mut modes = Vec::new();
                let mut add_mode = |mode: TestMode| {
                    std::env::var(test_mode_env(&mode))
                        .is_ok()
                        .then(|| modes.push(mode))
                };
                add_mode(TestMode::Deno);
                add_mode(TestMode::Browser { no_modules });
                add_mode(TestMode::DedicatedWorker { no_modules });
                add_mode(TestMode::SharedWorker { no_modules });
                add_mode(TestMode::ServiceWorker { no_modules });
                add_mode(TestMode::Node { no_modules });

                match modes.len() {
                    0 => TestMode::Node { no_modules: true },
                    1 => modes[0],
                    _ => {
                        bail!(
                            "only one test mode must be set, found: `{}`",
                            modes
                                .iter()
                                .map(test_mode_env)
                                .collect::<Vec<_>>()
                                .join("`, `")
                        )
                    }
                }
            }
        };

        let headless = env::var("NO_HEADLESS").is_err();
        let debug = env::var("WASM_BINDGEN_NO_DEBUG").is_err();

        // Gracefully handle requests to execute only node or only web tests.
        let node = matches!(test_mode, TestMode::Node { .. });

        if env::var_os("WASM_BINDGEN_TEST_ONLY_NODE").is_some() && !node {
            println!(
                "this test suite is only configured to run in a browser, \
             but we're only testing node.js tests so skipping"
            );
            return Ok(());
        }
        if env::var_os("WASM_BINDGEN_TEST_ONLY_WEB").is_some() && node {
            println!(
                "\
    This test suite is only configured to run in node.js, but we're only running
    browser tests so skipping. If you'd like to run the tests in a browser
    include this in your crate when testing:

        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    You'll likely want to put that in a `#[cfg(test)]` module or at the top of an
    integration test.\
    "
            );
            return Ok(());
        }

        let driver_timeout = env::var("WASM_BINDGEN_TEST_DRIVER_TIMEOUT")
            .map(|timeout| {
                timeout
                    .parse()
                    .expect("Could not parse 'WASM_BINDGEN_TEST_DRIVER_TIMEOUT'")
            })
            .unwrap_or(5);

        let browser_timeout = env::var("WASM_BINDGEN_TEST_TIMEOUT")
            .map(|timeout| {
                let timeout = timeout
                    .parse()
                    .expect("Could not parse 'WASM_BINDGEN_TEST_TIMEOUT'");
                println!("Set timeout to {timeout} seconds...");
                timeout
            })
            .unwrap_or(20);

        // Make the generated bindings available for the tests to execute against.
        shell.status("Executing bindgen...");
        let mut b = Bindgen::new();
        match test_mode {
            TestMode::Node { no_modules: true } => b.nodejs(true)?,
            TestMode::Node { no_modules: false } => b.nodejs_module(true)?,
            TestMode::Deno => b.deno(true)?,
            TestMode::Browser { .. }
            | TestMode::DedicatedWorker { .. }
            | TestMode::SharedWorker { .. }
            | TestMode::ServiceWorker { .. } => {
                if test_mode.no_modules() {
                    b.no_modules(true)?
                } else {
                    b.web(true)?
                }
            }
        };

        if std::env::var("WASM_BINDGEN_SPLIT_LINKED_MODULES").is_ok() {
            b.split_linked_modules(true);
        }

        let coverage = coverage_args(file_name);

        // The debug here means adding some assertions and some error messages to the generated js
        // code.
        //
        // It has nothing to do with Rust.
        b.debug(debug)
            .input_module(module, wasm)
            .emit_start(false)
            .generate(&tmpdir)
            .context("executing `wasm-bindgen` over the Wasm file")?;
        shell.clear();

        match test_mode {
            TestMode::Node { no_modules } => {
                // Augment `NODE_PATH` so things like `require("tests/my-custom.js")` work
                // and Rust code can import from custom JS shims. This is a bit of a hack
                // and should probably be removed at some point.
                let path = env::var("NODE_PATH").unwrap_or_default();
                let path = env::split_paths(&path).collect::<Vec<_>>();

                let extra_node_args = env::var("NODE_ARGS")
                    .unwrap_or_default()
                    .split(',')
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();

                node::execute(
                    module,
                    tmpdir.path(),
                    tests,
                    !no_modules,
                    coverage,
                    self.nocapture,
                    self.include_ignored,
                    path,
                    extra_node_args,
                )?
            }
            TestMode::Deno => deno::execute(
                module,
                tmpdir.path(),
                tests,
                self.nocapture,
                self.include_ignored,
            )?,
            TestMode::Browser { .. }
            | TestMode::DedicatedWorker { .. }
            | TestMode::SharedWorker { .. }
            | TestMode::ServiceWorker { .. } => {
                let capabilities_file = PathBuf::from(
                    std::env::var("WASM_BINDGEN_TEST_WEBDRIVER_JSON")
                        .unwrap_or("webdriver.json".to_string()),
                );
                let srv = server::spawn(
                    &if headless {
                        "127.0.0.1:0".parse().unwrap()
                    } else if let Ok(address) = std::env::var("WASM_BINDGEN_TEST_ADDRESS") {
                        address.parse().unwrap()
                    } else {
                        "127.0.0.1:8000".parse().unwrap()
                    },
                    headless,
                    module,
                    tmpdir.path(),
                    tests,
                    test_mode,
                    std::env::var("WASM_BINDGEN_TEST_NO_ORIGIN_ISOLATION").is_err(),
                    coverage,
                    self.nocapture,
                    self.include_ignored,
                )
                .context("failed to spawn server")?;
                let addr = srv.server_addr();

                // TODO: eventually we should provide the ability to exit at some point
                // (gracefully) here, but for now this just runs forever.
                if !headless {
                    println!("Interactive browsers tests are now available at http://{addr}");
                    println!();
                    println!("Note that interactive mode is enabled because `NO_HEADLESS`");
                    println!("is specified in the environment of this process. Once you're");
                    println!("done with testing you'll need to kill this server with");
                    println!("Ctrl-C.");
                    srv.run();
                    return Ok(());
                }

                thread::spawn(|| srv.run());
                headless::run(
                    &addr,
                    &shell,
                    driver_timeout,
                    browser_timeout,
                    &capabilities_file,
                    std::env::var("WASM_BINDGEN_TEST_ADDRESS").ok(),
                )?;
            }
        }
        Ok(())
    }
}

fn coverage_args(file_name: &Path) -> PathBuf {
    fn generated(file_name: &Path, prefix: &str) -> String {
        let res = format!("{prefix}{}.profraw", file_name.display());
        res
    }

    let prefix = env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX")
        .map(|s| s.to_str().unwrap().to_string())
        .unwrap_or_default();

    match env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT") {
        Some(s) => {
            let mut buf = PathBuf::from(s);
            if buf.is_dir() {
                buf.push(generated(file_name, &prefix));
            }
            buf
        }
        None => PathBuf::from(generated(file_name, &prefix)),
    }
}

fn test_mode_env(mode: &TestMode) -> &'static str {
    match mode {
        TestMode::Node { .. } => "WASM_BINDGEN_USE_NODE_EXPERIMENTAL",
        TestMode::Deno => "WASM_BINDGEN_USE_DENO",
        TestMode::Browser { .. } => "WASM_BINDGEN_USE_BROWSER",
        TestMode::DedicatedWorker { .. } => "WASM_BINDGEN_USE_DEDICATED_WORKER",
        TestMode::SharedWorker { .. } => "WASM_BINDGEN_USE_SHARED_WORKER",
        TestMode::ServiceWorker { .. } => "WASM_BINDGEN_USE_SERVICE_WORKER",
    }
}
