//! A shared, batched Cargo workspace for the majority of the plain CLI-behavior
//! tests in `main.rs`/`diagnostics.rs`/`npm.rs`.
//!
//! Like `reference::REFERENCE_WORKSPACE`, this exists so that the many small,
//! independent test crates that only need the default (stable) toolchain,
//! default `wasm32-unknown-unknown` target, and no extra `RUSTFLAGS` can be
//! compiled with a single shared `cargo build --workspace` invocation instead
//! of each test spawning its own `cargo build` subprocess.
//!
//! Unlike reference tests (which are standalone `.rs` files discovered by
//! scanning a directory), these tests' source lives inline in their `#[test]`
//! function bodies today, so there's no natural on-disk artifact to scan for
//! upfront. Instead, each batchable test is described by a [`Fixture`] in the
//! static [`FIXTURES`] table below, and the corresponding `#[test]` function
//! fetches its pre-built `.wasm`/output via [`fixture`] instead of creating
//! its own [`crate::Project`].
//!
//! Tests that need a custom toolchain, target, or `RUSTFLAGS` (e.g. the
//! `-Zbuild-std` nightly termination/panic-unwind tests) can't share this
//! workspace and continue to use `Project` directly, exactly as the
//! `panic-unwind`/`targets` reference tests continue to use
//! `reference::runtest_with_opts`.

use crate::{run_wasm_bindgen, REPO_ROOT, TARGET_DIR};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::LazyLock;

/// A single batchable test fixture.
pub(crate) struct Fixture {
    /// Unique crate/package name; must be a valid Rust identifier and is used
    /// as both the Cargo package name and the workspace member directory.
    pub(crate) name: &'static str,
    /// `(relative path, contents)` pairs written verbatim under the member's
    /// root. Occurrences of the literal `{root}` in `contents` are replaced
    /// with the repository root path, mirroring the `// DEPENDENCY:` comment
    /// convention used by reference tests.
    ///
    /// If no `"Cargo.toml"` entry is present, a default cdylib manifest
    /// depending on `wasm-bindgen` (plus `extra_deps`) is synthesized.
    /// Explicit `"Cargo.toml"` entries must *not* declare their own
    /// `[workspace]` table, since the member joins the shared workspace
    /// declared at its root.
    pub(crate) files: &'static [(&'static str, &'static str)],
    /// Extra `[dependencies]` lines appended when synthesizing a default
    /// Cargo.toml. Ignored if `files` already provides one.
    pub(crate) extra_deps: &'static [&'static str],
}

const fn fixture_def(
    name: &'static str,
    files: &'static [(&'static str, &'static str)],
) -> Fixture {
    Fixture {
        name,
        files,
        extra_deps: &[],
    }
}

include!("fixtures_data.rs");

struct MainWorkspace {
    root: PathBuf,
}

impl MainWorkspace {
    fn wasm_path(&self, name: &str) -> PathBuf {
        let mut built = TARGET_DIR.to_path_buf();
        built.push("wasm32-unknown-unknown");
        built.push("debug");
        built.push(name);
        built.set_extension("wasm");
        built
    }

    fn pkg_root(&self, name: &str) -> PathBuf {
        self.root.join(name).join("pkg")
    }
}

static MAIN_WORKSPACE: LazyLock<Result<MainWorkspace, String>> =
    LazyLock::new(|| build_main_workspace().map_err(|e| format!("{e:#}")));

fn build_main_workspace() -> Result<MainWorkspace> {
    let root = TARGET_DIR.join("cli-tests").join("main-workspace");
    drop(fs::remove_dir_all(&root));
    fs::create_dir_all(&root)?;

    let repo_root = REPO_ROOT.to_str().unwrap();
    let mut members = Vec::new();

    for f in FIXTURES {
        let member_dir = root.join(f.name);
        let mut has_cargo_toml = false;
        for (path, contents) in f.files {
            has_cargo_toml |= *path == "Cargo.toml";
            let dst = member_dir.join(path);
            fs::create_dir_all(dst.parent().unwrap())?;
            fs::write(&dst, contents.replace("{root}", repo_root))?;
        }

        if !has_cargo_toml {
            let name = f.name;
            let deps: String = f
                .extra_deps
                .iter()
                .map(|d| format!("{}\n", d.replace("{root}", repo_root)))
                .collect();
            fs::write(
                member_dir.join("Cargo.toml"),
                format!(
                    "[package]\n\
                     name = \"{name}\"\n\
                     authors = []\n\
                     version = \"1.0.0\"\n\
                     edition = \"2021\"\n\
                     \n\
                     [dependencies]\n\
                     wasm-bindgen = {{ path = '{repo_root}' }}\n\
                     {deps}\n\
                     [lib]\n\
                     crate-type = [\"cdylib\"]\n"
                ),
            )?;
        }

        members.push(f.name);
    }

    let members_toml = members
        .iter()
        .map(|m| format!("    \"{m}\",\n"))
        .collect::<String>();
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[workspace]\n\
             resolver = \"2\"\n\
             members = [\n\
             {members_toml}\
             ]\n\
             \n\
             [profile.dev]\n\
             codegen-units = 1\n"
        ),
    )?;

    let output = Command::new("cargo")
        .current_dir(&root)
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &*TARGET_DIR)
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "failed to build shared main-test workspace:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(MainWorkspace { root })
}

/// A handle to a pre-built [`Fixture`], mirroring the subset of `Project`'s
/// API that batched tests need.
pub(crate) struct BatchedProject {
    name: &'static str,
}

impl BatchedProject {
    /// Path to the fixture's compiled `.wasm`, in case a test needs to
    /// inspect or mutate it directly before calling [`Self::wasm_bindgen`].
    pub(crate) fn wasm(&self) -> Result<PathBuf> {
        let workspace = MAIN_WORKSPACE
            .as_ref()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(workspace.wasm_path(self.name))
    }

    /// The fixture's workspace-member directory, for tests that need to
    /// stage extra files (e.g. a hand-rolled emscripten input `.wasm`)
    /// alongside the build the same way `Project::root` does.
    pub(crate) fn root(&self) -> Result<PathBuf> {
        let workspace = MAIN_WORKSPACE
            .as_ref()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(workspace.root.join(self.name))
    }

    pub(crate) fn wasm_bindgen(&self, args: &str) -> Result<PathBuf> {
        let workspace = MAIN_WORKSPACE
            .as_ref()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        run_wasm_bindgen(
            &workspace.wasm_path(self.name),
            &workspace.pkg_root(self.name),
            args,
        )
    }
}

/// Look up a fixture registered in [`FIXTURES`] by name.
///
/// # Panics
///
/// Panics if `name` isn't present in [`FIXTURES`] (a programmer error: every
/// caller must have a matching entry).
pub(crate) fn fixture(name: &'static str) -> BatchedProject {
    assert!(
        FIXTURES.iter().any(|f| f.name == name),
        "no such fixture {name:?}; add it to FIXTURES in fixtures_data.rs"
    );
    BatchedProject { name }
}
