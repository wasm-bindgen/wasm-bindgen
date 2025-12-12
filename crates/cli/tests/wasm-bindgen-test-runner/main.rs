//! A small test suite for the `wasm-bindgen-test-runner` CLI command itself

use assert_cmd::Command;
use predicates::str;
use std::env;
use std::fs;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Output;
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

    fn wasm_bindgen_test(&mut self, args: &str) -> anyhow::Result<Output> {
        self.cargo_toml();
        let mut cargo_cmd = Command::new("cargo");
        let runner = REPO_ROOT.join("crates").join("cli").join("Cargo.toml");
        let output = cargo_cmd
            .current_dir(&self.root)
            .arg("test")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--")
            .args(args.split_whitespace())
            .env("CARGO_TARGET_DIR", &*TARGET_DIR)
            .env(
                "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER",
                format!(
                    "cargo run --manifest-path {} --bin wasm-bindgen-test-runner --",
                    runner.display()
                ),
            )
            .output()?;
        Ok(output)
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

#[test]
fn test_wasm_bindgen_test_runner_list() {
    let output = Project::new("test_wasm_bindgen_test_runner_list")
        .file(
            "src/lib.rs",
            r#"
            #[cfg(test)]
            mod tests {
                use wasm_bindgen_test::*;

                #[wasm_bindgen_test]
                fn test_foo() {}
            }
        "#,
        )
        .wasm_bindgen_test("--list")
        .unwrap();
    let mut lines = output.stdout.lines().map(|l| l.unwrap());
    assert_eq!(lines.next().as_deref(), Some("tests::test_foo: test"));
    assert_eq!(lines.next(), None);
}
