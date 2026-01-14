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
