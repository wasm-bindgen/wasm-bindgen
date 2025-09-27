use crate::Bindgen;
use anyhow::{bail, Context};
use std::path::PathBuf;
use std::thread;

mod deno;
mod headless;
mod node;
mod server;
mod shell;

pub use walrus;

struct Tests {
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

struct Test {
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
    /// The basename of the file being tested.
    file_name: String,

    /// The raw contents of the file to test. `cargo test` passes this argument for you.
    file_content: Vec<u8>,

    /// Use [Bindgen::no_modules] when generating test sources.
    no_modules: bool,

    /// Use [Bindgen::debug] when generating test sources.
    debug: bool,

    /// Use [Bindgen::split_linked_modules] when generating test sources.
    split_linked_modules: bool,

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

    /// Disable headless tests.
    no_headless: bool,

    /// A directory to use in place of a generated temp directory.
    temp_dir: Option<PathBuf>,

    /// The timeout to use for webdriver processes.
    driver_timeout: u64,

    /// The timeout to use for browser processes.
    browser_timeout: u64,

    /// The webdriver json config content.
    capabilities_content: Option<String>,

    /// An optional prefix
    coverage_profraw_prefix: Option<String>,

    /// The expected output file or directory for coverage profraw files.
    coverage_profraw_out: Option<PathBuf>,

    /// Skip setting Cross-Origin policies for tests.
    no_origin_isolation: bool,

    /// An optional override address to use for headless tests.
    address: Option<String>,

    /// The test mode to use when the type is not identifiable from [TestRunner::file].
    fallback_test_mode: Option<TestMode>,

    /// Only run web tests.
    test_only_web: bool,

    /// Only run node tests.
    test_only_node: bool,

    /// The path to the desired `nodejs` binary for node tests.
    node_bin: Option<PathBuf>,

    /// Augment `NODE_PATH` so things like `require("tests/my-custom.js")` work
    /// and Rust code can import from custom JS shims. This is a bit of a hack
    /// and should probably be removed at some point.
    node_path: Vec<PathBuf>,

    /// Arguments to pass to nodejs for node tests.
    node_args: Vec<String>,
}

impl TestRunner {
    /// Constructor for the [TestRunner].
    pub fn new(file_name: String, file_content: Vec<u8>) -> Self {
        Self {
            file_name,
            file_content,
            no_modules: false,
            debug: false,
            split_linked_modules: false,
            include_ignored: false,
            ignored: false,
            exact: false,
            skip: Vec::new(),
            list: false,
            nocapture: false,
            format: None,
            filter: None,
            no_headless: false,
            temp_dir: None,
            driver_timeout: 5,
            browser_timeout: 20,
            capabilities_content: None,
            coverage_profraw_prefix: None,
            coverage_profraw_out: None,
            no_origin_isolation: false,
            address: None,
            fallback_test_mode: None,
            test_only_node: false,
            test_only_web: false,
            node_bin: None,
            node_path: Vec::new(),
            node_args: Vec::new(),
        }
    }

    pub fn no_modules(&mut self) -> &mut Self {
        self.no_modules = true;
        self
    }

    pub fn debug(&mut self) -> &mut Self {
        self.debug = true;
        self
    }

    pub fn split_linked_modules(&mut self) -> &mut Self {
        self.split_linked_modules = true;
        self
    }

    pub fn with_include_ignored(&mut self) -> &mut Self {
        self.include_ignored = true;
        self
    }

    pub fn with_ignored(&mut self) -> &mut Self {
        self.ignored = true;
        self
    }

    pub fn with_exact(&mut self) -> &mut Self {
        self.exact = true;
        self
    }

    pub fn with_skip<S: Into<String>>(&mut self, filter: S) -> &mut Self {
        self.skip.push(filter.into());
        self
    }

    pub fn with_list(&mut self) -> &mut Self {
        self.list = true;
        self
    }

    pub fn with_nocapture(&mut self) -> &mut Self {
        self.nocapture = true;
        self
    }

    pub fn with_format(&mut self, format: OutputFormatSetting) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn with_filter<S: Into<String>>(&mut self, filter: S) -> &mut Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn with_no_headless(&mut self) -> &mut Self {
        self.no_headless = true;
        self
    }

    pub fn with_temp_dir(&mut self, path: PathBuf) -> &mut Self {
        self.temp_dir = Some(path);
        self
    }

    pub fn with_driver_timeout(&mut self, timeout: u64) -> &mut Self {
        self.driver_timeout = timeout;
        self
    }

    pub fn with_browser_timeout(&mut self, timeout: u64) -> &mut Self {
        self.browser_timeout = timeout;
        self
    }

    pub fn with_capabilities(&mut self, content: String) -> &mut Self {
        self.capabilities_content = Some(content);
        self
    }

    pub fn with_coverage_profraw_prefix(&mut self, prefix: String) -> &mut Self {
        self.coverage_profraw_prefix = Some(prefix);
        self
    }

    pub fn with_coverage_profraw_out(&mut self, path: PathBuf) -> &mut Self {
        self.coverage_profraw_out = Some(path);
        self
    }

    pub fn with_no_origin_isolation(&mut self) -> &mut Self {
        self.no_origin_isolation = true;
        self
    }

    pub fn with_address(&mut self, address: String) -> &mut Self {
        self.address = Some(address);
        self
    }

    pub fn with_fallback_test_mode(&mut self, test_mode: TestMode) -> &mut Self {
        self.fallback_test_mode = Some(test_mode);
        self
    }

    pub fn with_test_only_web(&mut self) -> &mut Self {
        self.test_only_web = true;
        self
    }

    pub fn with_test_only_node(&mut self) -> &mut Self {
        self.test_only_node = true;
        self
    }

    pub fn with_node_bin(&mut self, binary: PathBuf) -> &mut Self {
        self.node_bin = Some(binary);
        self
    }

    pub fn with_node_path(&mut self, path: Vec<PathBuf>) -> &mut Self {
        self.node_path = path;
        self
    }

    pub fn with_node_args(&mut self, args: Vec<String>) -> &mut Self {
        self.node_args = args;
        self
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        if self.ignored && self.include_ignored {
            bail!("`ignored` is mutually exclusive with `include_ignored`.");
        }

        let shell = shell::Shell::new();

        // Collect all tests that the test harness is supposed to run. We assume
        // that any exported function with the prefix `__wbg_test` is a test we need
        // to execute.
        let mut wasm = walrus::ModuleConfig::new()
            // generate dwarf by default, it can be controlled by debug profile
            //
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            .generate_dwarf(true)
            .parse(&self.file_content)
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
            return Ok(());
        }

        let (_, tmpdir) = match &self.temp_dir {
            Some(p) => (None, p.clone()),
            None => {
                let tmp = tempfile::tempdir()?;
                let path = tmp.path().to_path_buf();
                (Some(tmp), path)
            }
        };

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
        let no_modules = self.no_modules;
        let test_mode = match custom_section {
            Some(section) if section.data.contains(&0x01) => TestMode::Browser { no_modules },
            Some(section) if section.data.contains(&0x02) => {
                TestMode::DedicatedWorker { no_modules }
            }
            Some(section) if section.data.contains(&0x03) => TestMode::SharedWorker { no_modules },
            Some(section) if section.data.contains(&0x04) => TestMode::ServiceWorker { no_modules },
            Some(section) if section.data.contains(&0x05) => TestMode::Node { no_modules },
            Some(_) => bail!("invalid __wasm_bingen_test_unstable value"),
            None => match self.fallback_test_mode {
                Some(fallback) => fallback,
                None => bail!("Unable to determine test mode. No fallback was provided."),
            },
        };

        // Gracefully handle requests to execute only node or only web tests.
        let node = matches!(test_mode, TestMode::Node { .. });

        if self.test_only_node && !node {
            println!(
                "this test suite is only configured to run in a browser, \
             but we're only testing node.js tests so skipping"
            );
            return Ok(());
        }
        if self.test_only_web && node {
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

        let driver_timeout = self.driver_timeout;
        let browser_timeout = self.browser_timeout;

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

        if self.split_linked_modules {
            b.split_linked_modules(true);
        }

        let coverage = self.coverage_args(&self.file_name);

        // The debug here means adding some assertions and some error messages to the generated js
        // code.
        //
        // It has nothing to do with Rust.
        b.debug(self.debug)
            .input_module(module, wasm)
            .emit_start(false)
            .generate(&tmpdir)
            .context("executing `wasm-bindgen` over the Wasm file")?;
        shell.clear();

        match test_mode {
            TestMode::Node { no_modules } => node::execute(
                module,
                &tmpdir,
                tests,
                !no_modules,
                coverage,
                self.nocapture,
                self.include_ignored,
                &self.node_bin,
                &self.node_path,
                &self.node_args,
            )?,
            TestMode::Deno => {
                deno::execute(module, &tmpdir, tests, self.nocapture, self.include_ignored)?
            }
            TestMode::Browser { .. }
            | TestMode::DedicatedWorker { .. }
            | TestMode::SharedWorker { .. }
            | TestMode::ServiceWorker { .. } => {
                let headless = !self.no_headless;
                let srv = server::spawn(
                    &if headless {
                        "127.0.0.1:0".parse().unwrap()
                    } else if let Some(address) = &self.address {
                        address.parse().unwrap()
                    } else {
                        "127.0.0.1:8000".parse().unwrap()
                    },
                    headless,
                    module,
                    &tmpdir,
                    tests,
                    test_mode,
                    self.no_origin_isolation,
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
                    &self.capabilities_content,
                    self.address.clone(),
                )?;
            }
        }
        Ok(())
    }

    fn coverage_args(&self, file_name: &str) -> PathBuf {
        fn generated(file_name: &str, prefix: &str) -> String {
            let res = format!("{prefix}{}.profraw", file_name);
            res
        }

        let prefix = match &self.coverage_profraw_prefix {
            Some(p) => p.clone(),
            None => String::new(),
        };

        match &self.coverage_profraw_out {
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
}
