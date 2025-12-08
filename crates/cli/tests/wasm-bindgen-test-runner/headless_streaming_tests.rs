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
        "Expected 'running 1 test' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );

    // Make sure the test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    // Count occurrences of "hello" - should be exactly 1 for a failing test
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
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
    let combined = format!("{}{}", stdout, stderr);

    // Count occurrences of "hello" - should be 0 for a passing test (output captured)
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );

    // Verify test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    // Count occurrences of "hello" - should be exactly 2 (1 from nocapture, 1 from panic)
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 2,
        "Expected 'hello' to appear exactly twice, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );
}

/// Test that the test output does not contain embedded carriage returns from progress updates.
/// Bug: Progress status updates use \r to update in-place, but these \r characters leak into
/// the final output causing text corruption like:
///   "   Doc-tests playgrounded; 0 failed; 0 ignored; 0 filtered out; finished in 0.00s"
/// instead of proper separate lines.
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
        "Expected no carriage returns in stdout, but found {}.\nstdout (escaped):\n{:?}",
        stdout_cr_count, stdout
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {}.\nstderr (escaped):\n{:?}",
        stderr_cr_count, stderr
    );

    // Verify test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    // Count occurrences of "hello" - should be exactly 1 with --nocapture
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );

    // Verify test actually passed
    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
        "Expected 'running 1 test' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

/// Test that the output does not contain carriage returns in default mode.
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
        "Expected no carriage returns in stdout, but found {}.\nstdout (escaped):\n{:?}",
        stdout_cr_count, stdout
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {}.\nstderr (escaped):\n{:?}",
        stderr_cr_count, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

// ============================================================================
// run_in_browser mode tests (explicit main thread)
// ============================================================================

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
        "Expected 'running 1 test' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    // In browser mode, "hello" appears twice: once in "log output:" and once in
    // "console.log div contained:" (pre-existing runner behavior for debugging).
    let count = combined.matches("hello").count();

    assert_eq!(
        count, 2,
        "Expected 'hello' to appear exactly twice for failing test in browser mode, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

/// Test that the output does not contain carriage returns in run_in_browser mode.
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
        "Expected no carriage returns in stdout, but found {}.\nstdout (escaped):\n{:?}",
        stdout_cr_count, stdout
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {}.\nstderr (escaped):\n{:?}",
        stderr_cr_count, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

// ============================================================================
// run_in_shared_worker mode tests
// ============================================================================

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
        "Expected 'running 1 test' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

/// Test that the output does not contain carriage returns in run_in_shared_worker mode.
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
        "Expected no carriage returns in stdout, but found {}.\nstdout (escaped):\n{:?}",
        stdout_cr_count, stdout
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {}.\nstderr (escaped):\n{:?}",
        stderr_cr_count, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

// ============================================================================
// run_in_service_worker mode tests
// ============================================================================

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
        "Expected 'running 1 test' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("test test_1 ... ok") || stderr.contains("test test_1 ... ok"),
        "Expected 'test test_1 ... ok' in output.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 1,
        "Expected 'hello' to appear exactly once for failing test, but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
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
    let combined = format!("{}{}", stdout, stderr);

    let count = combined.matches("hello").count();

    assert_eq!(
        count, 0,
        "Expected 'hello' to NOT appear for passing test (output should be captured), but it appeared {} times.\nstdout:\n{}\nstderr:\n{}",
        count, stdout, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

/// Test that the output does not contain carriage returns in run_in_service_worker mode.
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
        "Expected no carriage returns in stdout, but found {}.\nstdout (escaped):\n{:?}",
        stdout_cr_count, stdout
    );
    assert_eq!(
        stderr_cr_count, 0,
        "Expected no carriage returns in stderr, but found {}.\nstderr (escaped):\n{:?}",
        stderr_cr_count, stderr
    );

    assert!(
        output.status.success(),
        "Test should pass.\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}
