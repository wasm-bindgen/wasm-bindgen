//! Tests for headless browser output streaming behavior.
//!
//! These tests verify that console output is handled correctly in headless browser mode,
//! including proper output streaming and avoiding duplicate messages.

use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); // current exe
    if dir.ends_with("deps") {
        dir.pop();
    }
    dir.pop(); // debug and/or release
    dir
});

static REPO_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut repo_root = env::current_dir().unwrap();
    repo_root.pop(); // remove 'cli'
    repo_root.pop(); // remove 'crates'
    repo_root
});

struct Project {
    root: PathBuf,
    name: String,
    deps: String,
    dev_deps: String,
}

impl Project {
    fn new(name: impl Into<String>) -> Project {
        let name = name.into();
        let root = TARGET_DIR.join("cli-tests").join(&name);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        Project {
            root,
            name,
            deps: "wasm-bindgen = { path = '{root}' }\n".to_owned(),
            dev_deps: "wasm-bindgen-test = { path = '{root}/crates/test' }\n".to_owned(),
        }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut Project {
        let dst = self.root.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::write(&dst, contents).unwrap();
        self
    }

    fn cargo_toml(&mut self) {
        if !self.root.join("Cargo.toml").is_file() {
            self.file(
                "Cargo.toml",
                &format!(
                    "
                        [package]
                        name = \"{}\"
                        authors = []
                        version = \"1.0.0\"
                        edition = '2021'

                        [dependencies]
                        {}


                        [dev-dependencies]
                        {}

                        [lib]
                        crate-type = ['cdylib']

                        [workspace]

                        [profile.dev]
                        codegen-units = 1
                    ",
                    self.name,
                    self.deps.replace("{root}", REPO_ROOT.to_str().unwrap()),
                    self.dev_deps.replace("{root}", REPO_ROOT.to_str().unwrap())
                ),
            );
        }
    }
}

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

#[test]
fn test_headless_worker_output_not_garbled() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_headless_worker_output_not_garbled");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_dedicated_worker);

            #[wasm_bindgen_test]
            fn test_1() {}
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // The output should contain the proper test output, not garbled text
    // Correct: "running 1 test" and "test test_1 ... ok"
    // Garbled: "Loading Wasm module...st_1 ... ok" (missing "running 1 test")
    assert!(
        stdout.contains("running 1 test") || stderr.contains("running 1 test"),
        "Expected 'running 1 test' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Make sure the test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Regression test for `WASM_BINDGEN_TEST_NO_STREAM`.
/// Even when streaming is disabled, final harness output should still be printed.
#[test]
fn test_headless_worker_output_visible_with_no_stream() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_headless_worker_output_visible_with_no_stream");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_dedicated_worker);

            #[wasm_bindgen_test]
            fn test_1() {}
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env("WASM_BINDGEN_TEST_NO_STREAM", "1")
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("running 1 test") || stderr.contains("running 1 test"),
        "Expected 'running 1 test' in output with WASM_BINDGEN_TEST_NO_STREAM=1.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output with WASM_BINDGEN_TEST_NO_STREAM=1.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears exactly once for a failing test in headless mode.
/// When a test panics, the console output should be shown exactly once.
#[test]
fn test_worker_console_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_worker_console_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            #[cfg(test)]
            mod tests {
                use wasm_bindgen_test::console_log;
                wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_dedicated_worker);

                #[wasm_bindgen_test::wasm_bindgen_test]
                fn test() {
                    console_log!("hello");
                    panic!()
                }
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // Count occurrences of "hello" - should be exactly 1 for a failing test
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output does NOT appear for a passing test in headless mode.
/// When a test passes, the console output should be captured and not shown.
#[test]
fn test_worker_console_no_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_worker_console_no_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            #[cfg(test)]
            mod tests {
                use wasm_bindgen_test::console_log;
                wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_dedicated_worker);

                #[wasm_bindgen_test::wasm_bindgen_test]
                fn test() {
                    console_log!("hello");
                }
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // Count occurrences of "hello" - should be 0 for a passing test (output captured)
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Verify test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears exactly once for a failing test with --nocapture.
#[test]
fn test_worker_console_panic_nocapture() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_worker_console_panic_nocapture");
    project.file(
        "src/lib.rs",
        r#"
            #[cfg(test)]
            mod tests {
                use wasm_bindgen_test::console_log;
                wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_dedicated_worker);

                #[wasm_bindgen_test::wasm_bindgen_test]
                fn test() {
                    console_log!("hello");
                    panic!()
                }
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--")
        .arg("--nocapture")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // Count occurrences of "hello" - should be exactly 2 (1 from nocapture, 1 from panic)
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 2,
        "Expected 'hello' to appear exactly twice, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears exactly twice for a passing test with --nocapture.
#[test]
fn test_worker_console_no_panic_nocapture() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_worker_console_no_panic_nocapture");
    project.file(
        "src/lib.rs",
        r#"
            #[cfg(test)]
            mod tests {
                use wasm_bindgen_test::console_log;
                wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_dedicated_worker);

                #[wasm_bindgen_test::wasm_bindgen_test]
                fn test() {
                    console_log!("hello");
                }
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--")
        .arg("--nocapture")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // Count occurrences of "hello" - should be exactly 1 with --nocapture
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Verify test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

// ============================================================================
// Default mode tests (no wasm_bindgen_test_configure)
// ============================================================================

/// Test that output is not garbled in default mode (main thread).
#[test]
fn test_default_output_not_garbled() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_default_output_not_garbled");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            #[wasm_bindgen_test]
            fn test_1() {}
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("running 1 test") || stderr.contains("running 1 test"),
        "Expected 'running 1 test' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears exactly once for a failing test in default mode.
#[test]
fn test_default_console_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_default_console_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
                panic!()
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output does NOT appear for a passing test in default mode.
#[test]
fn test_default_console_no_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_default_console_no_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that output is not garbled in run_in_browser mode.
#[test]
fn test_browser_output_not_garbled() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_browser_output_not_garbled");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            fn test_1() {}
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("running 1 test") || stderr.contains("running 1 test"),
        "Expected 'running 1 test' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears for a failing test in run_in_browser mode.
/// Note: In browser mode, "hello" appears twice - once in the test output's "log output:" section
/// and once in the "console.log div contained:" section that the runner prints for debugging.
/// This is pre-existing behavior in the headless runner.
#[test]
fn test_browser_console_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_browser_console_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
                panic!()
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // In browser mode, "hello" appears twice: once in "log output:" and once in
    // "console.log div contained:" (pre-existing runner behavior for debugging).
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 2,
        "Expected 'hello' to appear exactly twice for failing test in browser mode, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output does NOT appear for a passing test in run_in_browser mode.
#[test]
fn test_browser_console_no_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_browser_console_no_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that output is not garbled in run_in_shared_worker mode.
#[test]
fn test_shared_worker_output_not_garbled() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_shared_worker_output_not_garbled");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_shared_worker);

            #[wasm_bindgen_test]
            fn test_1() {}
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("running 1 test") || stderr.contains("running 1 test"),
        "Expected 'running 1 test' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears exactly once for a failing test in run_in_shared_worker mode.
#[test]
fn test_shared_worker_console_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_shared_worker_console_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_shared_worker);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
                panic!()
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output does NOT appear for a passing test in run_in_shared_worker mode.
#[test]
fn test_shared_worker_console_no_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_shared_worker_console_no_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_shared_worker);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that output is not garbled in run_in_service_worker mode.
#[test]
fn test_service_worker_output_not_garbled() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_service_worker_output_not_garbled");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_service_worker);

            #[wasm_bindgen_test]
            fn test_1() {}
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("running 1 test") || stderr.contains("running 1 test"),
        "Expected 'running 1 test' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output appears exactly once for a failing test in run_in_service_worker mode.
#[test]
fn test_service_worker_console_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_service_worker_console_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_service_worker);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
                panic!()
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console output does NOT appear for a passing test in run_in_service_worker mode.
#[test]
fn test_service_worker_console_no_panic_headless() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_service_worker_console_no_panic_headless");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_service_worker);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {count} times.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// Test that console.log from a user-spawned worker_thread is captured in Node.js CJS mode.
#[test]
fn test_user_spawned_worker_logs_node_cjs() {
    let mut project = Project::new("test_user_spawned_worker_logs_node_cjs");

    // Add wasm-bindgen-futures for async support
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");

    // For Node.js, we need to create a JS file that spawns the worker
    // Test all console log levels
    project.file(
        "worker_spawner.cjs",
        r#"
const { Worker } = require('worker_threads');

globalThis.spawnWorkerWithLog = function() {
    return new Promise((resolve, reject) => {
        const worker = new Worker(
            `
            console.debug("NODE_WORKER_DEBUG_MARKER_CJS_5K2N9");
            console.log("NODE_WORKER_LOG_MARKER_CJS_5K2N9");
            console.info("NODE_WORKER_INFO_MARKER_CJS_5K2N9");
            console.warn("NODE_WORKER_WARN_MARKER_CJS_5K2N9");
            console.error("NODE_WORKER_ERROR_MARKER_CJS_5K2N9");
            `,
            { eval: true }
        );
        worker.on('exit', () => resolve());
        worker.on('error', reject);
    });
};
"#,
    );

    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_test::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_name = spawnWorkerWithLog)]
                async fn spawn_worker_with_log();
            }

            #[wasm_bindgen_test]
            async fn test_spawned_worker_logs() {
                spawn_worker_with_log().await;
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");

    // First, we need to set up NODE_PATH to include the worker_spawner.cjs
    // The test runner should load this file before running tests
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        // Set NODE_ARGS to require our worker spawner before the test
        .env(
            "NODE_ARGS",
            format!("--require={}/worker_spawner.cjs", project.root.display()),
        )
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should pass
    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Check all log levels - each should appear exactly once
    let levels = [
        ("debug", "NODE_WORKER_DEBUG_MARKER_CJS_5K2N9"),
        ("log", "NODE_WORKER_LOG_MARKER_CJS_5K2N9"),
        ("info", "NODE_WORKER_INFO_MARKER_CJS_5K2N9"),
        ("warn", "NODE_WORKER_WARN_MARKER_CJS_5K2N9"),
        ("error", "NODE_WORKER_ERROR_MARKER_CJS_5K2N9"),
    ];

    // Print combined output for debugging
    eprintln!("=== Combined output ===\n{combined}\n=== End combined output ===");

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n"),
    );
}

/// Test that console.log from a user-spawned worker_thread is captured in Node.js ESM mode.
#[test]
fn test_user_spawned_worker_logs_node_esm() {
    let mut project = Project::new("test_user_spawned_worker_logs_node_esm");

    // Add wasm-bindgen-futures for async support
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");

    // For Node.js ESM, we need to create a JS module that spawns the worker
    project.file(
        "worker_spawner.mjs",
        r#"
import { Worker } from 'worker_threads';

globalThis.spawnWorkerWithLog = function() {
    return new Promise((resolve, reject) => {
        const worker = new Worker(
            `
            console.debug("NODE_WORKER_DEBUG_MARKER_ESM_8T4R2");
            console.log("NODE_WORKER_LOG_MARKER_ESM_8T4R2");
            console.info("NODE_WORKER_INFO_MARKER_ESM_8T4R2");
            console.warn("NODE_WORKER_WARN_MARKER_ESM_8T4R2");
            console.error("NODE_WORKER_ERROR_MARKER_ESM_8T4R2");
            `,
            { eval: true }
        );
        worker.on('exit', () => resolve());
        worker.on('error', reject);
    });
};
"#,
    );

    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_node_experimental);

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_name = spawnWorkerWithLog)]
                async fn spawn_worker_with_log();
            }

            #[wasm_bindgen_test]
            async fn test_spawned_worker_logs() {
                spawn_worker_with_log().await;
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");

    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        // Set NODE_ARGS to import our worker spawner before the test
        .env(
            "NODE_ARGS",
            format!("--import={}/worker_spawner.mjs", project.root.display()),
        )
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should pass
    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Check all 5 log levels - each should appear exactly once
    let levels = [
        ("debug", "NODE_WORKER_DEBUG_MARKER_ESM_8T4R2"),
        ("log", "NODE_WORKER_LOG_MARKER_ESM_8T4R2"),
        ("info", "NODE_WORKER_INFO_MARKER_ESM_8T4R2"),
        ("warn", "NODE_WORKER_WARN_MARKER_ESM_8T4R2"),
        ("error", "NODE_WORKER_ERROR_MARKER_ESM_8T4R2"),
    ];

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n"),
    );
}

/// Test that console.log and error from a user-spawned worker_thread appears when the worker fails in Node.js.
#[test]
fn test_user_spawned_worker_logs_on_failure_node() {
    let mut project = Project::new("test_user_spawned_worker_logs_on_failure_node");

    // Add wasm-bindgen-futures for async support
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");

    // Create a JS file that spawns a worker that logs then throws
    project.file(
        "worker_spawner.cjs",
        r#"
const { Worker } = require('worker_threads');

globalThis.spawnWorkerWithLogThenFail = function() {
    return new Promise((resolve, reject) => {
        const worker = new Worker(
            `
            console.log("NODE_WORKER_FAILURE_MARKER_7X9K3");
            throw new Error("Intentional node worker failure");
            `,
            { eval: true }
        );
        worker.on('exit', (code) => {
            if (code !== 0) {
                reject(new Error('Worker exited with code ' + code));
            } else {
                resolve();
            }
        });
        worker.on('error', reject);
    });
};
"#,
    );

    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_test::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_name = spawnWorkerWithLogThenFail)]
                async fn spawn_worker_with_log_then_fail();
            }

            #[wasm_bindgen_test]
            async fn test_spawned_worker_logs_then_fails() {
                spawn_worker_with_log_then_fail().await;
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");

    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(
            "NODE_ARGS",
            format!("--require={}/worker_spawner.cjs", project.root.display()),
        )
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should FAIL (worker throws)
    assert!(
        !output.status.success(),
        "Inner test should fail.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Verify the log marker appears
    let count = combined.matches("NODE_WORKER_FAILURE_MARKER_7X9K3").count();
    assert_eq!(
        count, 1,
        "Expected worker log marker to appear exactly once in failure output, but it appeared {count} times.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Verify the worker's error message appears in the output
    assert!(
        combined.contains("Intentional node worker failure"),
        "Expected worker error message 'Intentional node worker failure' to appear in output.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

// Additional headless streaming tests ported from origin/worker-logs-capture
#[test]
fn test_no_carriage_return_in_output() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_no_carriage_return_in_output");
    project.file(
        "src/lib.rs",
        r#"
            #[cfg(test)]
            mod tests {
                use wasm_bindgen_test::*;

                wasm_bindgen_test_configure!(run_in_dedicated_worker);

                #[wasm_bindgen_test]
                fn test() {
                    console_log!("hello");
                }
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that there are no carriage returns in the output.
    // Progress updates use \r to update in-place, but they should be cleared before
    // printing the final output.
    let stdout_cr_count = stdout.matches('\r').count();
    let stderr_cr_count = stderr.matches('\r').count();

    assert_eq!(
        stdout_cr_count, 0,
        "Expected no carriage returns in stdout, but found {stdout_cr_count}.\nstdout (escaped):\n{stdout:?}"
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {stderr_cr_count}.\nstderr (escaped):\n{stderr:?}"
    );

    // Verify test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Test that console output appears exactly twice for a passing test with --nocapture.
#[test]
fn test_default_no_carriage_return_in_output() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_default_no_carriage_return_in_output");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let stdout_cr_count = stdout.matches('\r').count();
    let stderr_cr_count = stderr.matches('\r').count();

    assert_eq!(
        stdout_cr_count, 0,
        "Expected no carriage returns in stdout, but found {stdout_cr_count}.\nstdout (escaped):\n{stdout:?}"
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {stderr_cr_count}.\nstderr (escaped):\n{stderr:?}"
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

// ============================================================================
// run_in_browser mode tests (explicit main thread)
// ============================================================================

/// Test that output is not garbled in run_in_browser mode.
#[test]
fn test_browser_no_carriage_return_in_output() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_browser_no_carriage_return_in_output");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let stdout_cr_count = stdout.matches('\r').count();
    let stderr_cr_count = stderr.matches('\r').count();

    assert_eq!(
        stdout_cr_count, 0,
        "Expected no carriage returns in stdout, but found {stdout_cr_count}.\nstdout (escaped):\n{stdout:?}"
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {stderr_cr_count}.\nstderr (escaped):\n{stderr:?}"
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

// ============================================================================
// run_in_shared_worker mode tests
// ============================================================================

/// Test that output is not garbled in run_in_shared_worker mode.
#[test]
fn test_shared_worker_no_carriage_return_in_output() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_shared_worker_no_carriage_return_in_output");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_shared_worker);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let stdout_cr_count = stdout.matches('\r').count();
    let stderr_cr_count = stderr.matches('\r').count();

    assert_eq!(
        stdout_cr_count, 0,
        "Expected no carriage returns in stdout, but found {stdout_cr_count}.\nstdout (escaped):\n{stdout:?}"
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {stderr_cr_count}.\nstderr (escaped):\n{stderr:?}"
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

// ============================================================================
// run_in_service_worker mode tests
// ============================================================================

/// Test that output is not garbled in run_in_service_worker mode.
#[test]
fn test_service_worker_no_carriage_return_in_output() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_service_worker_no_carriage_return_in_output");
    project.file(
        "src/lib.rs",
        r#"
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_service_worker);

            #[wasm_bindgen_test]
            fn test() {
                console_log!("hello");
            }
        "#,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let stdout_cr_count = stdout.matches('\r').count();
    let stderr_cr_count = stderr.matches('\r').count();

    assert_eq!(
        stdout_cr_count, 0,
        "Expected no carriage returns in stdout, but found {stdout_cr_count}.\nstdout (escaped):\n{stdout:?}"
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {stderr_cr_count}.\nstderr (escaped):\n{stderr:?}"
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

// ============================================================================
// User-spawned worker log capture tests
// These tests verify that console.log from workers spawned by user code
// (not the test runner's own workers) are captured in CLI output.
// ============================================================================

/// Test that console.log from a user-spawned dedicated worker is captured in browser mode.
/// This test spawns a Worker from within the WASM test code and verifies its logs appear.
#[test]
fn test_user_spawned_dedicated_worker_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_dedicated_worker_logs_browser");

    // Add additional dependencies needed for spawning workers
    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "Worker", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_worker_logs() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};

                // Create a worker script that logs all 5 console levels
                let script = r#"
                    console.debug("DEDICATED_WORKER_DEBUG_MARKER_7X9K2");
                    console.log("DEDICATED_WORKER_LOG_MARKER_7X9K2");
                    console.info("DEDICATED_WORKER_INFO_MARKER_7X9K2");
                    console.warn("DEDICATED_WORKER_WARN_MARKER_7X9K2");
                    console.error("DEDICATED_WORKER_ERROR_MARKER_7X9K2");
                    postMessage("done");
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = Worker::new(&url).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should pass (it just spawns a worker)
    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Check all 5 log levels - each should appear exactly once
    let levels = [
        ("debug", "DEDICATED_WORKER_DEBUG_MARKER_7X9K2"),
        ("log", "DEDICATED_WORKER_LOG_MARKER_7X9K2"),
        ("info", "DEDICATED_WORKER_INFO_MARKER_7X9K2"),
        ("warn", "DEDICATED_WORKER_WARN_MARKER_7X9K2"),
        ("error", "DEDICATED_WORKER_ERROR_MARKER_7X9K2"),
    ];

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n")
    );
}

/// Test that a user-installed `addEventListener("message", ...)` handler
/// coexists with console forwarding for a spawned dedicated worker.
#[test]
fn test_user_spawned_worker_add_event_listener_coexists_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_worker_add_event_listener_coexists_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "Worker", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_worker_add_event_listener_coexists() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};

                let script = r#"
                    console.log("DEDICATED_WORKER_ADD_EVENT_LISTENER_MARKER_5J3R8");
                    postMessage("done");
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = Worker::new(&url).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.add_event_listener_with_callback(
                        "message",
                        onmessage.unchecked_ref(),
                    ).unwrap();

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let count = combined
        .matches("DEDICATED_WORKER_ADD_EVENT_LISTENER_MARKER_5J3R8")
        .count();

    assert_eq!(
        count, 1,
        "Expected dedicated worker log marker to appear exactly once, but it appeared {count} times.\n\
         This test verifies that user addEventListener(message) handling coexists with console forwarding.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Test that console.log from a user-spawned module worker (type: 'module') is captured.
#[test]
fn test_user_spawned_module_worker_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_module_worker_logs_browser");

    // Add additional dependencies needed for spawning workers
    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "Worker", "WorkerOptions", "WorkerType", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_module_worker_logs() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker, WorkerOptions, WorkerType};

                // Create a module worker script that logs all 5 console levels
                let script = r#"
                    console.debug("MODULE_WORKER_DEBUG_MARKER_4M8N3");
                    console.log("MODULE_WORKER_LOG_MARKER_4M8N3");
                    console.info("MODULE_WORKER_INFO_MARKER_4M8N3");
                    console.warn("MODULE_WORKER_WARN_MARKER_4M8N3");
                    console.error("MODULE_WORKER_ERROR_MARKER_4M8N3");
                    postMessage("done");
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();

                // Create worker with type: 'module'
                let worker_opts = WorkerOptions::new();
                worker_opts.set_type(WorkerType::Module);
                let worker = Worker::new_with_options(&url, &worker_opts).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for module worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should pass
    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Check all 5 log levels - each should appear exactly once
    let levels = [
        ("debug", "MODULE_WORKER_DEBUG_MARKER_4M8N3"),
        ("log", "MODULE_WORKER_LOG_MARKER_4M8N3"),
        ("info", "MODULE_WORKER_INFO_MARKER_4M8N3"),
        ("warn", "MODULE_WORKER_WARN_MARKER_4M8N3"),
        ("error", "MODULE_WORKER_ERROR_MARKER_4M8N3"),
    ];

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n")
    );
}

/// Test that console.log from a user-spawned module worker created from a URL
/// (not a blob URL) is captured. This exercises the `await import(...)`
/// wrapper path for module workers.
#[test]
fn test_user_spawned_url_module_worker_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_url_module_worker_logs_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["MessageEvent", "Worker", "WorkerOptions", "WorkerType", "Window"]
"#,
    );

    project.file(
        "worker_script.js",
        r#"
console.debug("URL_MODULE_WORKER_DEBUG_MARKER_1Z8V4");
console.log("URL_MODULE_WORKER_LOG_MARKER_1Z8V4");
console.info("URL_MODULE_WORKER_INFO_MARKER_1Z8V4");
console.warn("URL_MODULE_WORKER_WARN_MARKER_1Z8V4");
console.error("URL_MODULE_WORKER_ERROR_MARKER_1Z8V4");
postMessage("done");
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_url_module_worker_logs() {
                use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType};

                let worker_opts = WorkerOptions::new();
                worker_opts.set_type(WorkerType::Module);
                let worker =
                    Worker::new_with_options("worker_script.js", &worker_opts).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for URL module worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let levels = [
        ("debug", "URL_MODULE_WORKER_DEBUG_MARKER_1Z8V4"),
        ("log", "URL_MODULE_WORKER_LOG_MARKER_1Z8V4"),
        ("info", "URL_MODULE_WORKER_INFO_MARKER_1Z8V4"),
        ("warn", "URL_MODULE_WORKER_WARN_MARKER_1Z8V4"),
        ("error", "URL_MODULE_WORKER_ERROR_MARKER_1Z8V4"),
    ];

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n")
    );
}

/// Regression test for non-cloneable worker console arguments.
/// Logging a function should not crash the worker before it can continue.
#[test]
fn test_user_spawned_worker_non_cloneable_log_does_not_crash_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project =
        Project::new("test_user_spawned_worker_non_cloneable_log_does_not_crash_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "ErrorEvent", "MessageEvent", "Url", "Worker", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_non_cloneable_worker_log_does_not_crash() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, ErrorEvent, MessageEvent, Url, Worker};

                let script = r#"
                    console.log(function nonCloneable() {});
                    postMessage("after-log");
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = Worker::new(&url).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_clone = reject.clone();
                    let onerror = Closure::once_into_js(move |e: ErrorEvent| {
                        reject_clone
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str(&format!("worker error: {}", e.message())),
                            )
                            .unwrap();
                    });
                    worker.set_onerror(Some(onerror.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("after-log"));

                worker.terminate();
                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Inner test should pass; non-cloneable console args should not crash the worker.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Test that console.log from a worker created via URL (not blob) is captured.
/// This tests the importScripts wrapper path in the Worker constructor patch.
#[test]
fn test_user_spawned_url_worker_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_url_worker_logs_browser");

    // Add additional dependencies needed for spawning workers
    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "Worker", "Window"]
"#,
    );

    // Create a worker JS file that will be served by the test server
    project.file(
        "worker_script.js",
        r#"
console.debug("URL_WORKER_DEBUG_MARKER_9K2P7");
console.log("URL_WORKER_LOG_MARKER_9K2P7");
console.info("URL_WORKER_INFO_MARKER_9K2P7");
console.warn("URL_WORKER_WARN_MARKER_9K2P7");
console.error("URL_WORKER_ERROR_MARKER_9K2P7");
postMessage("done");
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_url_worker_logs() {
                use web_sys::{MessageEvent, Worker};

                // Create worker from URL (not blob) - this tests importScripts wrapper
                let worker = Worker::new("worker_script.js").unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for URL worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should pass
    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Check all 5 log levels - each should appear exactly once
    let levels = [
        ("debug", "URL_WORKER_DEBUG_MARKER_9K2P7"),
        ("log", "URL_WORKER_LOG_MARKER_9K2P7"),
        ("info", "URL_WORKER_INFO_MARKER_9K2P7"),
        ("warn", "URL_WORKER_WARN_MARKER_9K2P7"),
        ("error", "URL_WORKER_ERROR_MARKER_9K2P7"),
    ];

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n")
    );
}

/// Test that logs from a nested worker are forwarded as well.
/// The outer worker spawns an inner worker and relays a completion message.
#[test]
fn test_user_spawned_nested_worker_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_nested_worker_logs_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "Worker", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_nested_worker_logs() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};

                let script = r#"
                    const innerBlob = new Blob(
                        ['onmessage=function(){console.log("NESTED_WORKER_LOG_MARKER_2P6L1");postMessage("done")};postMessage("ready")'],
                        { type: "application/javascript" }
                    );
                    const inner = new Worker(URL.createObjectURL(innerBlob));
                    inner.onmessage = function(e) {
                        if (e.data === "ready") {
                            inner.postMessage("start");
                            return;
                        }
                        postMessage(e.data);
                        inner.terminate();
                    };
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = Worker::new(&url).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for nested worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let count = combined.matches("NESTED_WORKER_LOG_MARKER_2P6L1").count();
    assert_eq!(
        count, 1,
        "Expected nested worker log marker to appear exactly once, but it appeared {count} times.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Regression test for non-cloneable shared worker console arguments.
/// Logging a function should not crash the shared worker before it can continue.
#[test]
fn test_user_spawned_shared_worker_non_cloneable_log_does_not_crash_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project =
        Project::new("test_user_spawned_shared_worker_non_cloneable_log_does_not_crash_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "ErrorEvent", "MessageEvent", "Url", "SharedWorker", "MessagePort", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_non_cloneable_shared_worker_log_does_not_crash() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, ErrorEvent, MessageEvent, SharedWorker, Url};

                let script = r#"
                    onconnect = function(e) {
                        const port = e.ports[0];
                        console.log(function nonCloneable() {});
                        port.postMessage("after-log");
                    };
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = SharedWorker::new(&url).unwrap();
                let port = worker.port();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    port.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_clone = reject.clone();
                    let onerror = Closure::once_into_js(move |e: ErrorEvent| {
                        reject_clone
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str(&format!("worker error: {}", e.message())),
                            )
                            .unwrap();
                    });
                    worker.set_onerror(Some(onerror.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for shared worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("after-log"));

                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Inner test should pass; non-cloneable console args should not crash the shared worker.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Regression test for script injection in the non-blob Worker wrapper path.
/// The `data:` payload is harmless JavaScript followed by attacker-looking text
/// inside a JS comment. If the wrapper concatenates the URL directly into the
/// bootstrap source, the comment breaks out and the injected marker executes.
#[test]
fn test_user_spawned_data_url_worker_does_not_inject_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_data_url_worker_does_not_inject_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["MessageEvent", "Worker", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_data_url_worker_does_not_inject() {
                use web_sys::{MessageEvent, Worker};

                // Escaped correctly, this URL evaluates as the worker script
                // `postMessage("done")//");console.log("DATA_URL_INJECTED_MARKER_6R2M8");//`
                // which emits no logs. Vulnerable wrapper generation turns it
                // into a second top-level console.log in the bootstrap blob.
                let url = r#"data:text/javascript,postMessage("done")//");console.log("DATA_URL_INJECTED_MARKER_6R2M8");//"#;
                let worker = Worker::new(url).unwrap();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    worker.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for data URL worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                worker.terminate();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let injected_count = combined.matches("DATA_URL_INJECTED_MARKER_6R2M8").count();

    assert_eq!(
        injected_count, 0,
        "Injected marker should not appear. Expected 0, got {injected_count}.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Test that console.log from a user-spawned worker appears in console_log div when test fails.
/// This tests the non-nocapture code path where console_log is shown on failure.
#[test]
fn test_user_spawned_worker_logs_on_failure_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_worker_logs_on_failure_browser");

    // Add additional dependencies needed for spawning workers
    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "ErrorEvent", "Url", "Worker", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_worker_logs_then_fails() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, Url, Worker};

                // Create a worker script that logs a unique marker then throws an error
                let script = r#"
                    console.log("SPAWNED_WORKER_FAILURE_TEST_MARKER_8Y3M4");
                    throw new Error("Intentional worker failure");
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = Worker::new(&url).unwrap();

                // Wait for the worker error via a promise that rejects on error
                let error_promise = js_sys::Promise::new(&mut |_resolve, reject| {
                    let reject_clone = reject.clone();
                    let onerror = Closure::once_into_js(move |e: web_sys::ErrorEvent| {
                        reject_clone.call1(&JsValue::NULL, &e.message().into()).unwrap();
                    });
                    worker.set_onerror(Some(onerror.unchecked_ref()));
                });

                // This will reject when the worker throws
                wasm_bindgen_futures::JsFuture::from(error_promise).await.unwrap();

                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    // Note: NO --nocapture here - we want to test the failure path
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should FAIL (it panics intentionally)
    assert!(
        !output.status.success(),
        "Inner test should fail.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Count occurrences of the marker - should be exactly 1 in console_log div output
    let count = combined
        .matches("SPAWNED_WORKER_FAILURE_TEST_MARKER_8Y3M4")
        .count();

    assert_eq!(
        count, 1,
        "Expected worker log marker to appear exactly once in failure output, but it appeared {count} times.\n\
         This test verifies that console.log from user-spawned workers is shown when tests fail.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Verify the worker's panic/error message appears in the output
    assert!(
        combined.contains("Intentional worker failure"),
        "Expected worker panic message 'Intentional worker failure' to appear in output.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Test that console.log from a user-spawned shared worker is captured in browser mode.
#[test]
fn test_user_spawned_shared_worker_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_shared_worker_logs_browser");

    // Add additional dependencies needed for spawning workers
    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "SharedWorker", "MessagePort", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_shared_worker_logs() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, MessageEvent, SharedWorker, Url};

                // Create a shared worker script that logs all 5 console levels on connect
                let script = r#"
                    onconnect = function(e) {
                        console.debug("SPAWNED_SHARED_WORKER_DEBUG_MARKER_3M8P1");
                        console.log("SPAWNED_SHARED_WORKER_LOG_MARKER_3M8P1");
                        console.info("SPAWNED_SHARED_WORKER_INFO_MARKER_3M8P1");
                        console.warn("SPAWNED_SHARED_WORKER_WARN_MARKER_3M8P1");
                        console.error("SPAWNED_SHARED_WORKER_ERROR_MARKER_3M8P1");
                        e.ports[0].postMessage("done");
                    };
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = SharedWorker::new(&url).unwrap();
                let port = worker.port();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    port.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for shared worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should pass
    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Check all 5 log levels - each should appear exactly once
    let levels = [
        ("debug", "SPAWNED_SHARED_WORKER_DEBUG_MARKER_3M8P1"),
        ("log", "SPAWNED_SHARED_WORKER_LOG_MARKER_3M8P1"),
        ("info", "SPAWNED_SHARED_WORKER_INFO_MARKER_3M8P1"),
        ("warn", "SPAWNED_SHARED_WORKER_WARN_MARKER_3M8P1"),
        ("error", "SPAWNED_SHARED_WORKER_ERROR_MARKER_3M8P1"),
    ];

    let mut failures = Vec::new();
    for (level, marker) in &levels {
        let count = combined.matches(*marker).count();
        if count != 1 {
            failures.push(format!("console.{level}: expected 1, got {count}"));
        }
    }

    assert!(
        failures.is_empty(),
        "Some console log levels were not captured correctly:\n{}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
        failures.join("\n")
    );
}

/// Regression test for shared worker logs emitted before the first port is connected.
/// Top-level logs should still be captured even though no `connect` event has fired yet.
#[test]
fn test_user_spawned_shared_worker_top_level_logs_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_shared_worker_top_level_logs_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "MessageEvent", "Url", "SharedWorker", "MessagePort", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_shared_worker_top_level_logs() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, MessageEvent, SharedWorker, Url};

                let script = r#"
                    console.log("SPAWNED_SHARED_WORKER_TOP_LEVEL_MARKER_7Q2H5");
                    onconnect = function(e) {
                        e.ports[0].postMessage("done");
                    };
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = SharedWorker::new(&url).unwrap();
                let port = worker.port();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    port.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for shared worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));

                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let count = combined
        .matches("SPAWNED_SHARED_WORKER_TOP_LEVEL_MARKER_7Q2H5")
        .count();

    assert_eq!(
        count, 1,
        "Expected top-level shared worker log marker to appear exactly once, but it appeared {count} times.\n\
         This test verifies that logs emitted before the first port connection are still captured.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Regression test for script injection in the non-blob SharedWorker wrapper path.
/// The `data:` payload is harmless JavaScript followed by attacker-looking text
/// inside a JS comment. If the wrapper concatenates the URL directly into the
/// bootstrap source, the comment breaks out and the injected connect listener
/// executes.
#[test]
fn test_user_spawned_data_url_shared_worker_does_not_inject_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project =
        Project::new("test_user_spawned_data_url_shared_worker_does_not_inject_browser");

    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["MessageEvent", "SharedWorker", "MessagePort", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_data_url_shared_worker_does_not_inject() {
                use web_sys::{MessageEvent, SharedWorker};

                // Escaped correctly, this URL evaluates as the shared worker
                // script `addEventListener("connect",e=>e.ports[0].postMessage("done"))//");addEventListener("connect",()=>console.log("..."));//`
                // which is inert. Vulnerable wrapper generation registers a new
                // connect listener in the bootstrap blob that logs the marker.
                let url = r#"data:text/javascript,addEventListener("connect",e=>e.ports[0].postMessage("done"))//");addEventListener("connect",()=>console.log("SHARED_URL_INJECTED_MARKER_4T1Q6"));//"#;
                let worker = SharedWorker::new(url).unwrap();
                let port = worker.port();
                port.start();

                let completion = js_sys::Promise::new(&mut |resolve, reject| {
                    let resolve_clone = resolve.clone();
                    let onmessage = Closure::once_into_js(move |e: MessageEvent| {
                        resolve_clone.call1(&JsValue::NULL, &e.data()).unwrap();
                    });
                    port.set_onmessage(Some(onmessage.unchecked_ref()));

                    let reject_timeout = reject.clone();
                    let timeout = Closure::once_into_js(move || {
                        reject_timeout
                            .call1(
                                &JsValue::NULL,
                                &JsValue::from_str("timed out waiting for data URL shared worker completion"),
                            )
                            .unwrap();
                    });
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout.unchecked_ref(),
                            500,
                        )
                        .unwrap();
                });

                let result = wasm_bindgen_futures::JsFuture::from(completion)
                    .await
                    .unwrap();
                assert_eq!(result.as_string().as_deref(), Some("done"));
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner -- --nocapture",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        output.status.success(),
        "Inner test should pass.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let injected_count = combined.matches("SHARED_URL_INJECTED_MARKER_4T1Q6").count();

    assert_eq!(
        injected_count, 0,
        "Injected shared worker marker should not appear. Expected 0, got {injected_count}.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

/// Test that console.log from a user-spawned shared worker appears in console_log div when test fails.
#[test]
fn test_user_spawned_shared_worker_logs_on_failure_browser() {
    let Some((driver_env, driver_path)) = find_webdriver() else {
        eprintln!("Skipping headless test: no webdriver found");
        return;
    };

    let mut project = Project::new("test_user_spawned_shared_worker_logs_on_failure_browser");

    // Add additional dependencies needed for spawning workers
    project
        .deps
        .push_str("js-sys = { path = '{root}/crates/js-sys' }\n");
    project
        .deps
        .push_str("wasm-bindgen-futures = { path = '{root}/crates/futures' }\n");
    project.deps.push_str(
        r#"
[dependencies.web-sys]
path = '{root}/crates/web-sys'
features = ["Blob", "BlobPropertyBag", "ErrorEvent", "Url", "SharedWorker", "MessagePort", "Window"]
"#,
    );

    project.file(
        "src/lib.rs",
        r##"
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_test::*;

            wasm_bindgen_test_configure!(run_in_browser);

            #[wasm_bindgen_test]
            async fn test_spawned_shared_worker_logs_then_fails() {
                use js_sys::Array;
                use web_sys::{Blob, BlobPropertyBag, Url, SharedWorker};

                // Create a shared worker script that logs a unique marker on connect then throws
                let script = r#"
                    onconnect = function(e) {
                        console.log("SPAWNED_SHARED_WORKER_FAILURE_MARKER_5K9N2");
                        throw new Error("Intentional shared worker failure");
                    };
                "#;
                let arr = Array::new();
                arr.push(&JsValue::from_str(script));
                let opts = BlobPropertyBag::new();
                opts.set_type("application/javascript");
                let blob = Blob::new_with_str_sequence_and_options(&arr, &opts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let worker = SharedWorker::new(&url).unwrap();
                let port = worker.port();
                port.start();

                // Wait for the worker error via a promise that rejects on error
                let error_promise = js_sys::Promise::new(&mut |_resolve, reject| {
                    let reject_clone = reject.clone();
                    let onerror = Closure::once_into_js(move |e: web_sys::ErrorEvent| {
                        reject_clone.call1(&JsValue::NULL, &e.message().into()).unwrap();
                    });
                    worker.set_onerror(Some(onerror.unchecked_ref()));
                });

                // This will reject when the worker throws
                wasm_bindgen_futures::JsFuture::from(error_promise).await.unwrap();

                Url::revoke_object_url(&url).unwrap();
            }
        "##,
    );

    project.cargo_toml();
    let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
    // Note: NO --nocapture here - we want to test the failure path
    let output = Command::new("cargo")
        .current_dir(&project.root)
        .arg("test")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .env(
            "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
            format!(
                "cargo run --manifest-path {} --bin wasm-bindgen-test-runner",
                runner.display()
            ),
        )
        .env(driver_env, driver_path)
        .output()
        .expect("failed to execute cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // The inner test should FAIL (it panics intentionally)
    assert!(
        !output.status.success(),
        "Inner test should fail.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Count occurrences of the marker - should be exactly 1 in console_log div output
    let count = combined
        .matches("SPAWNED_SHARED_WORKER_FAILURE_MARKER_5K9N2")
        .count();

    assert_eq!(
        count, 1,
        "Expected shared worker log marker to appear exactly once in failure output, but it appeared {count} times.\n\
         This test verifies that console.log from user-spawned shared workers is shown when tests fail.\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
