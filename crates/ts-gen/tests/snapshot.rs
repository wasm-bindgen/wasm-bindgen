//! Snapshot tests for ts-gen code generation.
//!
//! Every `.d.ts` file in `tests/fixtures/` is automatically discovered and run
//! through the `ts-gen` CLI binary, then the generated Rust output is compared
//! against a blessed snapshot in `tests/snapshots/<name>.rs`.
//!
//! Fixtures can include `//! @ts-gen <args>` directive comments at the top.
//! The args after `//! @ts-gen` are passed **verbatim** to the CLI, so any
//! flags the CLI supports (e.g. `--lib-name`, `--external`) just work.
//!
//! To add a new snapshot test, just drop a `.d.ts` file into `tests/fixtures/`.
//! To update snapshots after intentional changes:
//!
//! ```sh
//! BLESS=1 cargo test -p ts-gen
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

/// Root of the ts-gen crate (where Cargo.toml lives).
fn crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Path to the compiled `ts-gen` binary.
///
/// `cargo test` compiles the binary into the same target directory.
fn ts_gen_bin() -> PathBuf {
    // env!("CARGO_BIN_EXE_ts-gen") is set by cargo for integration tests
    // when the crate defines a [[bin]] target.
    PathBuf::from(env!("CARGO_BIN_EXE_ts-gen"))
}

/// Collect extra CLI args from `//! @ts-gen <args>` directive lines at the top
/// of a `.d.ts` file. The text after the prefix is split on whitespace and
/// returned as a flat list of arguments — no interpretation, just passthrough.
fn collect_directive_args(fixture: &Path) -> Vec<String> {
    let content = std::fs::read_to_string(fixture).unwrap_or_default();
    let mut args = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("//! @ts-gen ") {
            args.extend(rest.split_whitespace().map(String::from));
        } else if !trimmed.starts_with("//") && !trimmed.is_empty() {
            // Stop scanning once we hit non-comment content
            break;
        }
    }

    args
}

/// Compare `actual` against the snapshot file at `snap_path`.
///
/// - If `BLESS=1` is set, writes `actual` to `snap_path` and returns Ok.
/// - Otherwise, reads the snapshot and asserts equality, printing a diff on mismatch.
fn assert_snapshot(snap_path: &Path, actual: &str) {
    let bless = std::env::var("BLESS").is_ok_and(|v| v == "1" || v == "true");

    if bless {
        if let Some(parent) = snap_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create snapshot directory");
        }
        std::fs::write(snap_path, actual)
            .unwrap_or_else(|e| panic!("Failed to write snapshot {}: {e}", snap_path.display()));
        eprintln!("  blessed: {}", snap_path.display());
        return;
    }

    let expected = std::fs::read_to_string(snap_path).unwrap_or_else(|e| {
        panic!(
            "Snapshot file not found: {}\n\
             Run `BLESS=1 cargo test -p ts-gen` to create it.\n\
             Error: {e}",
            snap_path.display()
        )
    });

    if actual == expected {
        return;
    }

    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(expected.as_str(), actual);
    let mut diff_output = String::new();
    let snap_name = snap_path.file_name().unwrap().to_string_lossy();

    diff_output.push_str(&format!("\n--- expected: {snap_name}\n+++ actual\n"));

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            diff_output.push_str("...\n");
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let sign = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal => " ",
                };
                diff_output.push_str(&format!("{sign}{change}"));
                if change.missing_newline() {
                    diff_output.push('\n');
                }
            }
        }
    }

    panic!(
        "Snapshot mismatch for {}\n\
         Run `BLESS=1 cargo test -p ts-gen` to update.\n\
         {diff_output}",
        snap_path.display()
    );
}

/// Run the `ts-gen` CLI on a `.d.ts` fixture and compare the generated Rust
/// output against the blessed snapshot.
fn snapshot_test(fixture_name: &str) {
    let root = crate_root();
    let fixture = root.join("tests/fixtures").join(fixture_name);
    let base_name = fixture_name.replace(".d.ts", "");
    let snap_path = root
        .join("tests")
        .join("snapshots")
        .join(format!("{base_name}.rs"));

    // Collect any extra CLI flags from directive comments
    let extra_args = collect_directive_args(&fixture);

    // Use a temp file for the CLI output
    let out_dir = std::env::temp_dir().join(format!("ts-gen-snapshot-{base_name}"));
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir).expect("Failed to create temp output dir");
    let out_file = out_dir.join("bindings.rs");

    // Invoke the CLI binary: ts-gen --input <fixture> --output <out_file> [extra_args...]
    let output = Command::new(ts_gen_bin())
        .arg("--input")
        .arg(&fixture)
        .arg("--output")
        .arg(&out_file)
        .args(&extra_args)
        .output()
        .expect("Failed to execute ts-gen binary");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "ts-gen CLI failed for {fixture_name}:\n\
             exit code: {:?}\n\
             stderr:\n{stderr}\n\
             stdout:\n{stdout}",
            output.status.code()
        );
    }

    let rust_code = std::fs::read_to_string(&out_file).unwrap_or_else(|e| {
        panic!(
            "Failed to read generated output {}: {e}",
            out_file.display()
        )
    });

    assert_snapshot(&snap_path, &rust_code);

    // Clean up
    let _ = std::fs::remove_dir_all(&out_dir);
}

/// Discover all `.d.ts` files in `tests/fixtures/` and run a snapshot test
/// for each one. Test names are derived from the file stem.
#[test]
fn snapshots() {
    let fixtures_dir = crate_root().join("tests/fixtures");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", fixtures_dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ts")
                && path.to_string_lossy().ends_with(".d.ts")
            {
                Some(path.file_name()?.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "No .d.ts fixtures found in {}",
        fixtures_dir.display()
    );

    let mut failures = Vec::new();

    for fixture in &fixtures {
        let result = std::panic::catch_unwind(|| {
            snapshot_test(fixture);
        });
        if let Err(e) = result {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown panic".to_string()
            };
            failures.push((fixture.clone(), msg));
        }
    }

    if !failures.is_empty() {
        let mut report = format!("\n{} snapshot(s) failed:\n", failures.len());
        for (name, msg) in &failures {
            report.push_str(&format!("\n--- {name} ---\n{msg}\n"));
        }
        panic!("{report}");
    }

    eprintln!("All {} snapshot tests passed.", fixtures.len());
}
