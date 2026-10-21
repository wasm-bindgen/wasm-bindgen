//! A small test suite for the `wasm-bindgen-test-runner` CLI command itself
//!
//! Every scenario is a test binary in the `fixture` crate, which is built once with a
//! single `cargo test --no-run`. Each binary is then run by `wasm-bindgen-test-runner`
//! directly, and a run is shared by every test that inspects its output, so each browser
//! session is started only once. To add a scenario, prefer adding a test to an existing
//! binary with the same mode, arguments and outcome (tests in a binary must log unique
//! markers), and only add a new file in `fixture/tests` when none fits.

mod headless_streaming_tests;

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

const RUNNER: &str = env!("CARGO_BIN_EXE_wasm-bindgen-test-runner");

static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); // current exe
    if dir.ends_with("deps") {
        dir.pop();
    }
    dir.pop(); // debug and/or release
    dir
});

static FIXTURE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wasm-bindgen-test-runner")
        .join("fixture")
});

/// The fixture's test binaries, keyed by target name.
static FIXTURE: LazyLock<HashMap<String, PathBuf>> = LazyLock::new(|| {
    let output = Command::new("cargo")
        .current_dir(&*FIXTURE_DIR)
        .args(["test", "--target", "wasm32-unknown-unknown", "--no-run"])
        .arg("--message-format=json-render-diagnostics")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .output()
        .expect("failed to execute cargo");
    assert!(
        output.status.success(),
        "failed to build the test runner fixture:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let message: serde_json::Value = serde_json::from_str(line).ok()?;
            if message["reason"] != "compiler-artifact" || message["profile"]["test"] != true {
                return None;
            }
            let name = message["target"]["name"].as_str()?;
            let executable = message["executable"].as_str()?;
            Some((name.to_owned(), PathBuf::from(executable)))
        })
        .collect()
});

/// The output of running one fixture binary.
struct Run {
    stdout: String,
    stderr: String,
    success: bool,
}

impl Run {
    fn count(&self, needle: &str) -> usize {
        self.stdout.matches(needle).count() + self.stderr.matches(needle).count()
    }

    fn dump(&self) -> String {
        format!("stdout:\n{}\nstderr:\n{}", self.stdout, self.stderr)
    }

    #[track_caller]
    fn assert_success(&self) {
        assert!(self.success, "Test should pass.\n{}", self.dump());
    }

    #[track_caller]
    fn assert_failure(&self) {
        assert!(!self.success, "Test should fail.\n{}", self.dump());
    }

    #[track_caller]
    fn assert_contains(&self, needle: &str) {
        assert!(
            self.count(needle) > 0,
            "Expected {needle:?} in output.\n{}",
            self.dump(),
        );
    }

    #[track_caller]
    fn assert_count(&self, needle: &str, expected: usize) {
        let count = self.count(needle);
        assert_eq!(
            count,
            expected,
            "Expected {needle:?} to appear {expected} times, but it appeared {count} times.\n{}",
            self.dump(),
        );
    }
}

/// Runs the fixture binary `target` with `wasm-bindgen-test-runner`.
fn run(target: &str, args: &[&str], envs: &[(&str, &OsStr)]) -> Run {
    let wasm = FIXTURE
        .get(target)
        .unwrap_or_else(|| panic!("the fixture has no test binary named `{target}`"));
    let output = Command::new(RUNNER)
        .current_dir(&*FIXTURE_DIR)
        .arg(wasm)
        .args(args)
        .envs(envs.iter().copied())
        .output()
        .expect("failed to execute wasm-bindgen-test-runner");
    Run {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        success: output.status.success(),
    }
}

#[test]
fn test_wasm_bindgen_test_runner_list() {
    let run = run("list", &["--list"], &[]);
    run.assert_success();
    let mut lines = run.stdout.lines();
    assert_eq!(lines.next(), Some("tests::test_foo: test"));
    assert_eq!(lines.next(), None);
}
