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

use clap::Parser;
use clap::ValueEnum;
use std::path::PathBuf;
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

    let mut runner = TestRunner::new(cli.file);

    if cli.include_ignored {
        runner = runner.with_include_ignored()?;
    }

    if cli.ignored {
        runner = runner.with_ignored()?;
    }

    if cli.exact {
        runner = runner.with_exact();
    }

    for skip in cli.skip {
        runner = runner.with_skip(skip);
    }

    if cli.list {
        runner = runner.with_list();
    }

    if cli.nocapture {
        runner = runner.with_nocapture();
    }

    if let Some(fmt) = cli.format {
        runner = runner.with_format(fmt.into());
    }

    if let Some(filter) = cli.filter {
        runner = runner.with_filter(filter);
    }

    runner.execute()
}
