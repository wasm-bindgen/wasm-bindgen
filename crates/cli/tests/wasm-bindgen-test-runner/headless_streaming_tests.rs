//! Tests for headless browser output streaming behavior.
//!
//! These tests verify that console output is handled correctly in headless browser mode,
//! including proper output streaming and avoiding duplicate messages. The scenarios are
//! binaries in the fixture described in `main.rs`, and each `static` run below happens
//! once, shared by every test that inspects it.

use super::{run, Run, FIXTURE_DIR};
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::sync::LazyLock;

/// Logged by the `passing_test` in each `<mode>_pass` fixture binary.
const PASS_MARKER: &str = "PASSING_TEST_LOG_MARKER";
/// Logged by the `failing_test` in each `<mode>_fail` fixture binary.
const FAIL_MARKER: &str = "FAILING_TEST_LOG_MARKER";

static WEBDRIVER: LazyLock<Option<(&'static str, PathBuf)>> = LazyLock::new(find_webdriver);

/// Returns the path to a webdriver if one is available, or None if headless
/// tests should be skipped.
fn find_webdriver() -> Option<(&'static str, PathBuf)> {
    // Check for explicit env vars first
    if let Ok(path) = env::var("CHROMEDRIVER") {
        return Some(("CHROMEDRIVER", PathBuf::from(path)));
    }
    if let Ok(path) = env::var("GECKODRIVER") {
        return Some(("GECKODRIVER", PathBuf::from(path)));
    }
    if let Ok(path) = env::var("SAFARIDRIVER") {
        return Some(("SAFARIDRIVER", PathBuf::from(path)));
    }

    // Try to find webdrivers in PATH
    for (env_name, binary) in [
        ("CHROMEDRIVER", "chromedriver"),
        ("GECKODRIVER", "geckodriver"),
        ("SAFARIDRIVER", "safaridriver"),
    ] {
        if let Ok(output) = std::process::Command::new("which").arg(binary).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some((env_name, PathBuf::from(path)));
                }
            }
        }
    }

    None
}

impl Run {
    /// Asserts that `{prefix}{LEVEL}{suffix}` appears exactly once for every
    /// console level.
    #[track_caller]
    fn assert_each_level_once(&self, prefix: &str, suffix: &str) {
        let failures = ["DEBUG", "LOG", "INFO", "WARN", "ERROR"]
            .iter()
            .filter_map(|level| {
                let count = self.count(&format!("{prefix}{level}{suffix}"));
                (count != 1).then(|| format!("console.{level}: expected 1, got {count}"))
            })
            .collect::<Vec<_>>();
        assert!(
            failures.is_empty(),
            "Some console log levels were not captured correctly:\n{}\n{}",
            failures.join("\n"),
            self.dump(),
        );
    }

    /// Asserts that a passing `<mode>_pass` run printed the harness output intact.
    #[track_caller]
    fn assert_not_garbled(&self) {
        // Garbled output looked like "Loading Wasm module...st_1 ... ok", with
        // "running 1 test" missing.
        self.assert_contains("running 1 test");
        self.assert_contains("test passing_test ... ok");
        // Progress updates use \r to update in-place, but they should be
        // cleared before printing the final output.
        self.assert_count("\r", 0);
        self.assert_success();
    }
}

/// Runs the fixture binary `target` in a headless browser, or returns `None`
/// if no webdriver is available.
fn browser_run(target: &str, args: &[&str]) -> Option<Run> {
    let (driver_env, driver_path) = WEBDRIVER.as_ref()?;
    Some(run(target, args, &[(*driver_env, driver_path.as_os_str())]))
}

/// Returns a shared browser run, or `None` if headless tests are skipped.
fn browser(run: &'static LazyLock<Option<Run>>) -> Option<&'static Run> {
    let run = run.as_ref();
    if run.is_none() {
        eprintln!("Skipping headless test: no webdriver found");
    }
    run
}

fn node_args(flag: &str, file: &str) -> OsString {
    let mut arg = OsString::from(flag);
    arg.push(FIXTURE_DIR.join(file));
    arg
}

// ============================================================================
// Default mode tests (no wasm_bindgen_test_configure, so they run in Node.js)
// ============================================================================

static NODE_PASS: LazyLock<Run> = LazyLock::new(|| run("node_pass", &[], &[]));
static NODE_FAIL: LazyLock<Run> = LazyLock::new(|| run("node_fail", &[], &[]));

/// Test that output is not garbled in default mode.
#[test]
fn test_default_output_not_garbled() {
    NODE_PASS.assert_not_garbled();
}

/// Test that console output appears exactly once for a failing test in default mode.
#[test]
fn test_default_console_panic_headless() {
    NODE_FAIL.assert_failure();
    NODE_FAIL.assert_count(FAIL_MARKER, 1);
}

/// Test that console output does NOT appear for a passing test in default mode.
#[test]
fn test_default_console_no_panic_headless() {
    NODE_PASS.assert_success();
    NODE_PASS.assert_count(PASS_MARKER, 0);
}

// ============================================================================
// run_in_browser mode tests (explicit main thread)
// ============================================================================

static BROWSER_PASS: LazyLock<Option<Run>> = LazyLock::new(|| browser_run("browser_pass", &[]));
static BROWSER_FAIL: LazyLock<Option<Run>> = LazyLock::new(|| browser_run("browser_fail", &[]));

/// Test that output is not garbled in run_in_browser mode.
#[test]
fn test_browser_output_not_garbled() {
    let Some(run) = browser(&BROWSER_PASS) else {
        return;
    };
    run.assert_not_garbled();
}

/// Test that console output appears for a failing test in run_in_browser mode.
/// Note: In browser mode, the log appears twice - once in the test output's "log output:"
/// section and once in the "console output:" section that the runner prints for debugging.
/// This is pre-existing behavior in the headless runner.
#[test]
fn test_browser_console_panic_headless() {
    let Some(run) = browser(&BROWSER_FAIL) else {
        return;
    };
    run.assert_failure();
    run.assert_count(FAIL_MARKER, 2);
}

/// Test that console output does NOT appear for a passing test in run_in_browser mode.
#[test]
fn test_browser_console_no_panic_headless() {
    let Some(run) = browser(&BROWSER_PASS) else {
        return;
    };
    run.assert_success();
    run.assert_count(PASS_MARKER, 0);
}

// ============================================================================
// run_in_dedicated_worker mode tests
// ============================================================================

static DEDICATED_PASS: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("dedicated_worker_pass", &[]));
static DEDICATED_FAIL: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("dedicated_worker_fail", &[]));
static DEDICATED_PASS_NOCAPTURE: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("dedicated_worker_pass", &["--nocapture"]));
static DEDICATED_FAIL_NOCAPTURE: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("dedicated_worker_fail", &["--nocapture"]));

/// Test that output is not garbled in run_in_dedicated_worker mode.
#[test]
fn test_headless_worker_output_not_garbled() {
    let Some(run) = browser(&DEDICATED_PASS) else {
        return;
    };
    run.assert_not_garbled();
}

/// Regression test for `WASM_BINDGEN_TEST_NO_STREAM`.
/// Even when streaming is disabled, final harness output should still be printed.
#[test]
fn test_headless_worker_output_visible_with_no_stream() {
    let Some((driver_env, driver_path)) = WEBDRIVER.as_ref() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };
    let run = run(
        "dedicated_worker_pass",
        &[],
        &[
            ("WASM_BINDGEN_TEST_NO_STREAM", OsStr::new("1")),
            (*driver_env, driver_path.as_os_str()),
        ],
    );
    run.assert_not_garbled();
}

/// Test that console output appears exactly once for a failing test in headless mode.
#[test]
fn test_worker_console_panic_headless() {
    let Some(run) = browser(&DEDICATED_FAIL) else {
        return;
    };
    run.assert_failure();
    run.assert_count(FAIL_MARKER, 1);
}

/// Test that console output does NOT appear for a passing test in headless mode.
#[test]
fn test_worker_console_no_panic_headless() {
    let Some(run) = browser(&DEDICATED_PASS) else {
        return;
    };
    run.assert_success();
    run.assert_count(PASS_MARKER, 0);
}

/// Test that console output appears exactly twice for a failing test with --nocapture:
/// once as it is logged, and once in the failure report.
#[test]
fn test_worker_console_panic_nocapture() {
    let Some(run) = browser(&DEDICATED_FAIL_NOCAPTURE) else {
        return;
    };
    run.assert_failure();
    run.assert_count(FAIL_MARKER, 2);
}

/// Test that console output appears exactly once for a passing test with --nocapture.
/// See: https://github.com/wasm-bindgen/wasm-bindgen/pull/4845#issuecomment-3660688206
#[test]
fn test_worker_console_no_panic_nocapture() {
    let Some(run) = browser(&DEDICATED_PASS_NOCAPTURE) else {
        return;
    };
    run.assert_success();
    run.assert_count(PASS_MARKER, 1);
}

// ============================================================================
// run_in_shared_worker mode tests
// ============================================================================

static SHARED_PASS: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("shared_worker_pass", &[]));
static SHARED_FAIL: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("shared_worker_fail", &[]));

/// Test that output is not garbled in run_in_shared_worker mode.
#[test]
fn test_shared_worker_output_not_garbled() {
    let Some(run) = browser(&SHARED_PASS) else {
        return;
    };
    run.assert_not_garbled();
}

/// Test that console output appears exactly once for a failing test in run_in_shared_worker mode.
#[test]
fn test_shared_worker_console_panic_headless() {
    let Some(run) = browser(&SHARED_FAIL) else {
        return;
    };
    run.assert_failure();
    run.assert_count(FAIL_MARKER, 1);
}

/// Test that console output does NOT appear for a passing test in run_in_shared_worker mode.
#[test]
fn test_shared_worker_console_no_panic_headless() {
    let Some(run) = browser(&SHARED_PASS) else {
        return;
    };
    run.assert_success();
    run.assert_count(PASS_MARKER, 0);
}

// ============================================================================
// run_in_service_worker mode tests
// ============================================================================

static SERVICE_PASS: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("service_worker_pass", &[]));
static SERVICE_FAIL: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("service_worker_fail", &[]));

/// Test that output is not garbled in run_in_service_worker mode.
#[test]
fn test_service_worker_output_not_garbled() {
    let Some(run) = browser(&SERVICE_PASS) else {
        return;
    };
    run.assert_not_garbled();
}

/// Test that console output appears exactly once for a failing test in run_in_service_worker mode.
#[test]
fn test_service_worker_console_panic_headless() {
    let Some(run) = browser(&SERVICE_FAIL) else {
        return;
    };
    run.assert_failure();
    run.assert_count(FAIL_MARKER, 1);
}

/// Test that console output does NOT appear for a passing test in run_in_service_worker mode.
#[test]
fn test_service_worker_console_no_panic_headless() {
    let Some(run) = browser(&SERVICE_PASS) else {
        return;
    };
    run.assert_success();
    run.assert_count(PASS_MARKER, 0);
}

// ============================================================================
// Node.js user-spawned worker log capture tests
// ============================================================================

/// Test that console.log from a user-spawned worker_thread is captured in Node.js CJS mode.
#[test]
fn test_user_spawned_worker_logs_node_cjs() {
    let node_args = node_args("--require=", "worker_spawner.cjs");
    let run = run("node_user_worker_cjs", &[], &[("NODE_ARGS", &node_args)]);
    run.assert_success();
    run.assert_each_level_once("NODE_WORKER_", "_MARKER_CJS_5K2N9");
}

/// Test that console.log from a user-spawned worker_thread is captured in Node.js ESM mode.
#[test]
fn test_user_spawned_worker_logs_node_esm() {
    let node_args = node_args("--import=", "worker_spawner.mjs");
    let run = run("node_user_worker_esm", &[], &[("NODE_ARGS", &node_args)]);
    run.assert_success();
    run.assert_each_level_once("NODE_WORKER_", "_MARKER_ESM_8T4R2");
}

/// Test that console.log and error from a user-spawned worker_thread appears when the worker fails in Node.js.
#[test]
fn test_user_spawned_worker_logs_on_failure_node() {
    let node_args = node_args("--require=", "worker_spawner.cjs");
    let run = run("node_user_worker_fail", &[], &[("NODE_ARGS", &node_args)]);
    run.assert_failure();
    run.assert_count("NODE_WORKER_FAILURE_MARKER_7X9K3", 1);
    run.assert_contains("Intentional node worker failure");
}

// ============================================================================
// Browser user-spawned worker log capture tests
// These tests verify that console.log from workers spawned by user code
// (not the test runner's own workers) are captured in CLI output.
// ============================================================================

/// All the passing tests in `user_workers.rs`, run with `--nocapture`.
static USER_WORKERS: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("user_workers", &["--nocapture"]));
/// All the failing tests in `user_workers_fail.rs`, run without `--nocapture`.
static USER_WORKERS_FAIL: LazyLock<Option<Run>> =
    LazyLock::new(|| browser_run("user_workers_fail", &[]));

/// Returns the shared `user_workers` run after checking that its test `name` passed.
#[track_caller]
fn user_workers_test(name: &str) -> Option<&'static Run> {
    let run = browser(&USER_WORKERS)?;
    run.assert_contains(&format!("test {name} ... ok"));
    Some(run)
}

/// Test that console.log from a user-spawned dedicated worker is captured in browser mode.
#[test]
fn test_user_spawned_dedicated_worker_logs_browser() {
    let Some(run) = user_workers_test("dedicated_worker_logs") else {
        return;
    };
    run.assert_each_level_once("DEDICATED_WORKER_", "_MARKER_7X9K2");
}

/// Test that a user-installed `addEventListener("message", ...)` handler
/// coexists with console forwarding for a spawned dedicated worker.
#[test]
fn test_user_spawned_worker_add_event_listener_coexists_browser() {
    let Some(run) = user_workers_test("add_event_listener_coexists") else {
        return;
    };
    run.assert_count("DEDICATED_WORKER_ADD_EVENT_LISTENER_MARKER_5J3R8", 1);
}

/// Test that console.log from a user-spawned module worker (type: 'module') is captured.
#[test]
fn test_user_spawned_module_worker_logs_browser() {
    let Some(run) = user_workers_test("module_worker_logs") else {
        return;
    };
    run.assert_each_level_once("MODULE_WORKER_", "_MARKER_4M8N3");
}

/// Test that console.log from a user-spawned module worker created from a URL
/// (not a blob URL) is captured. This exercises the `await import(...)`
/// wrapper path for module workers.
#[test]
fn test_user_spawned_url_module_worker_logs_browser() {
    let Some(run) = user_workers_test("url_module_worker_logs") else {
        return;
    };
    run.assert_each_level_once("URL_MODULE_WORKER_", "_MARKER_1Z8V4");
}

/// Test that console.log from a worker created via URL (not blob) is captured.
/// This tests the importScripts wrapper path in the Worker constructor patch.
#[test]
fn test_user_spawned_url_worker_logs_browser() {
    let Some(run) = user_workers_test("url_worker_logs") else {
        return;
    };
    run.assert_each_level_once("URL_WORKER_", "_MARKER_9K2P7");
}

/// Regression test for non-cloneable worker console arguments.
/// Logging a function should not crash the worker before it can continue.
#[test]
fn test_user_spawned_worker_non_cloneable_log_does_not_crash_browser() {
    user_workers_test("worker_non_cloneable_log_does_not_crash");
}

/// Test that logs from a nested worker are forwarded as well.
/// The outer worker spawns an inner worker and relays a completion message.
#[test]
fn test_user_spawned_nested_worker_logs_browser() {
    let Some(run) = user_workers_test("nested_worker_logs") else {
        return;
    };
    run.assert_count("NESTED_WORKER_LOG_MARKER_2P6L1", 1);
}

/// Regression test for script injection in the non-blob Worker wrapper path.
/// The `data:` payload is harmless JavaScript followed by attacker-looking text
/// inside a JS comment. If the wrapper concatenates the URL directly into the
/// bootstrap source, the comment breaks out and the injected marker executes.
#[test]
fn test_user_spawned_data_url_worker_does_not_inject_browser() {
    let Some(run) = user_workers_test("data_url_worker_does_not_inject") else {
        return;
    };
    run.assert_count("DATA_URL_INJECTED_MARKER_6R2M8", 0);
}

/// Test that console.log from a user-spawned shared worker is captured in browser mode.
#[test]
fn test_user_spawned_shared_worker_logs_browser() {
    let Some(run) = user_workers_test("shared_worker_logs") else {
        return;
    };
    run.assert_each_level_once("SPAWNED_SHARED_WORKER_", "_MARKER_3M8P1");
}

/// Regression test for shared worker logs emitted before the first port is connected.
/// Top-level logs should still be captured even though no `connect` event has fired yet.
#[test]
fn test_user_spawned_shared_worker_top_level_logs_browser() {
    let Some(run) = user_workers_test("shared_worker_top_level_logs") else {
        return;
    };
    run.assert_count("SPAWNED_SHARED_WORKER_TOP_LEVEL_MARKER_7Q2H5", 1);
}

/// Regression test for non-cloneable shared worker console arguments.
/// Logging a function should not crash the shared worker before it can continue.
#[test]
fn test_user_spawned_shared_worker_non_cloneable_log_does_not_crash_browser() {
    user_workers_test("shared_worker_non_cloneable_log_does_not_crash");
}

/// Regression test for script injection in the non-blob SharedWorker wrapper path.
/// The `data:` payload is harmless JavaScript followed by attacker-looking text
/// inside a JS comment. If the wrapper concatenates the URL directly into the
/// bootstrap source, the comment breaks out and the injected connect listener
/// executes.
#[test]
fn test_user_spawned_data_url_shared_worker_does_not_inject_browser() {
    let Some(run) = user_workers_test("data_url_shared_worker_does_not_inject") else {
        return;
    };
    run.assert_count("SHARED_URL_INJECTED_MARKER_4T1Q6", 0);
}

/// Test that console.log from a user-spawned worker appears in console output when test fails.
/// This tests the non-nocapture code path where console output is shown on failure.
#[test]
fn test_user_spawned_worker_logs_on_failure_browser() {
    let Some(run) = browser(&USER_WORKERS_FAIL) else {
        return;
    };
    run.assert_failure();
    run.assert_contains("test dedicated_worker_logs_then_fails ... FAIL");
    run.assert_count("SPAWNED_WORKER_FAILURE_TEST_MARKER_8Y3M4", 1);
    run.assert_contains("Intentional worker failure");
}

/// Test that console.log from a user-spawned shared worker appears in console output when test fails.
#[test]
fn test_user_spawned_shared_worker_logs_on_failure_browser() {
    let Some(run) = browser(&USER_WORKERS_FAIL) else {
        return;
    };
    run.assert_failure();
    run.assert_contains("test shared_worker_logs_then_fails ... FAIL");
    run.assert_count("SPAWNED_SHARED_WORKER_FAILURE_MARKER_5K9N2", 1);
}
