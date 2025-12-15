//! Utility functions for commands.
#![allow(clippy::redundant_closure)]

use anyhow::{bail, Result};
use log::info;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// If an explicit path is given, then use it, otherwise assume the current
/// directory is the crate path.
pub fn get_crate_path(path: Option<PathBuf>) -> Result<PathBuf> {
    match path {
        Some(p) => Ok(p),
        None => find_manifest_from_cwd(),
    }
}

/// Search up the path for the manifest file from the current working directory
/// If we don't find the manifest file then return back the current working directory
/// to provide the appropriate error
fn find_manifest_from_cwd() -> Result<PathBuf> {
    let mut parent_path = std::env::current_dir()?;
    let mut manifest_path = parent_path.join("Cargo.toml");
    loop {
        if !manifest_path.is_file() {
            if parent_path.pop() {
                manifest_path = parent_path.join("Cargo.toml");
            } else {
                return Ok(PathBuf::from("."));
            }
        } else {
            return Ok(parent_path);
        }
    }
}

/// Construct our `pkg` directory in the crate.
pub fn create_pkg_dir(out_dir: &Path) -> Result<()> {
    let _ = fs::remove_file(out_dir.join("package.json")); // Clean up package.json from previous runs
    fs::create_dir_all(out_dir)?;
    fs::write(out_dir.join(".gitignore"), "*")?;
    Ok(())
}

/// Render a `Duration` to a form suitable for display on a console
pub fn elapsed(duration: Duration) -> String {
    let secs = duration.as_secs();

    if secs >= 60 {
        format!("{}m {:02}s", secs / 60, secs % 60)
    } else {
        format!("{}.{:02}s", secs, duration.subsec_nanos() / 10_000_000)
    }
}

/// Run the given command and return on success.
pub fn run(mut command: Command, command_name: &str) -> Result<()> {
    info!("Running {:?}", command);

    let status = command.status()?;

    if status.success() {
        Ok(())
    } else {
        bail!(
            "failed to execute `{}`: exited with {}\n  full command: {:?}",
            command_name,
            status,
            command,
        )
    }
}

/// Run the given command and return its stdout.
pub fn run_capture_stdout(mut command: Command, command_name: &str) -> Result<String> {
    info!("Running {:?}", command);

    let output = command
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        bail!(
            "failed to execute `{}`: exited with {}\n  full command: {:?}",
            command_name,
            output.status,
            command,
        )
    }
}
