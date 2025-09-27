//! A "wrapper binary" used to execute Wasm files as tests
//!
//! This binary is intended to be used as a "test runner" for Wasm binaries,
//! being compatible with `cargo test` for the Wasm target. It will
//! automatically execute `wasm-bindgen` (or the equivalent thereof) and then
//! execute either Node.js over the tests or start a server which a browser can
//! be used to run against to execute tests. In a browser mode if `CI` is in the
//! environment then it'll also attempt headless testing, spawning the server in
//! the background and then using the WebDriver protocol to execute tests.
//!
//! For more documentation about this see the `wasm-bindgen-test` crate README
//! and source code.

use anyhow::{bail, Context};
use clap::Parser;
use clap::ValueEnum;
use std::env;
use std::fs;
use std::path::PathBuf;
use wasm_bindgen_cli_support::test_runner::TestMode;
use wasm_bindgen_cli_support::test_runner::{OutputFormatSetting, TestRunner};

/// Possible values for the `--format` option.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FormatSetting {
    /// Display one character per test
    Terse,
}

impl From<FormatSetting> for OutputFormatSetting {
    fn from(f: FormatSetting) -> Self {
        match f {
            FormatSetting::Terse => OutputFormatSetting::Terse,
        }
    }
}

#[derive(Parser)]
#[command(name = "wasm-bindgen-test-runner", version, about, long_about = None)]
struct Cli {
    #[arg(
        index = 1,
        help = "The file to test. `cargo test` passes this argument for you."
    )]
    file: PathBuf,
    #[arg(long, conflicts_with = "ignored", help = "Run ignored tests")]
    include_ignored: bool,
    #[arg(long, conflicts_with = "include_ignored", help = "Run ignored tests")]
    ignored: bool,
    #[arg(long, help = "Exactly match filters rather than by substring")]
    exact: bool,
    #[arg(
        long,
        value_name = "FILTER",
        help = "Skip tests whose names contain FILTER (this flag can be used multiple times)"
    )]
    skip: Vec<String>,
    #[arg(long, help = "List all tests and benchmarks")]
    list: bool,
    #[arg(
        long,
        help = "don't capture `console.*()` of each task, allow printing directly"
    )]
    nocapture: bool,
    #[arg(
        long,
        value_enum,
        value_name = "terse",
        help = "Configure formatting of output"
    )]
    format: Option<FormatSetting>,
    #[arg(
        index = 2,
        value_name = "FILTER",
        help = "The FILTER string is tested against the name of all tests, and only those tests \
                whose names contain the filter are run."
    )]
    filter: Option<String>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let file_name = cli
        .file
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .context("file to test is not a valid file, can't extract file name")?;

    let file_content = fs::read(&cli.file).context("failed to read Wasm file")?;
    let mut runner = TestRunner::new(file_name, file_content);

    if cli.include_ignored {
        runner.with_include_ignored();
    }

    if cli.ignored {
        runner.with_ignored();
    }

    if cli.exact {
        runner.with_exact();
    }

    for skip in cli.skip {
        runner.with_skip(skip);
    }

    if cli.list {
        runner.with_list();
    }

    if cli.nocapture {
        runner.with_nocapture();
    }

    if let Some(fmt) = cli.format {
        runner.with_format(fmt.into());
    }

    if let Some(filter) = cli.filter {
        runner.with_filter(filter);
    }

    let no_modules = std::env::var("WASM_BINDGEN_USE_NO_MODULE").is_ok();
    if no_modules {
        runner.no_modules();
    }

    if env::var("NO_HEADLESS").is_err() {
        runner.with_no_headless();
    }

    if env::var("WASM_BINDGEN_NO_DEBUG").is_err() {
        runner.debug();
    }

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

    runner.with_fallback_test_mode(match modes.len() {
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
    });

    if let Ok(timeout) = env::var("WASM_BINDGEN_TEST_DRIVER_TIMEOUT") {
        runner.with_driver_timeout(
            timeout
                .parse()
                .expect("Could not parse 'WASM_BINDGEN_TEST_DRIVER_TIMEOUT'"),
        );
    }

    if let Ok(timeout) = env::var("WASM_BINDGEN_TEST_TIMEOUT") {
        runner.with_browser_timeout(
            timeout
                .parse()
                .expect("Could not parse 'WASM_BINDGEN_TEST_TIMEOUT'"),
        );
    }

    if std::env::var("WASM_BINDGEN_SPLIT_LINKED_MODULES").is_ok() {
        runner.split_linked_modules();
    }

    let capabilities_file = PathBuf::from(
        std::env::var("WASM_BINDGEN_TEST_WEBDRIVER_JSON").unwrap_or("webdriver.json".to_string()),
    );
    if capabilities_file.exists() {
        let content = std::fs::read_to_string(capabilities_file)
            .context("A capabilities file was found but could not be read")?;
        runner.with_capabilities(content);
    }

    if std::env::var("WASM_BINDGEN_TEST_NO_ORIGIN_ISOLATION").is_err() {
        runner.with_no_origin_isolation();
    }

    if env::var_os("WASM_BINDGEN_TEST_ONLY_NODE").is_some() {
        runner.with_test_only_node();
    }

    if env::var_os("WASM_BINDGEN_TEST_ONLY_WEB").is_some() {
        runner.with_test_only_web();
    }

    if let Ok(value) = env::var("NODE_PATH") {
        let path = env::split_paths(&value).collect::<Vec<_>>();
        runner.with_node_path(path);
    }

    if let Ok(args) = env::var("NODE_ARGS") {
        let extra_node_args = args
            .split(',')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        runner.with_node_args(extra_node_args);
    }

    if let Some(prefix) = env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX") {
        runner.with_coverage_profraw_prefix(prefix.to_str().unwrap().to_string());
    }

    if let Some(out) = env::var_os("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT") {
        runner.with_coverage_profraw_out(PathBuf::from(out));
    }

    runner.execute()
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
