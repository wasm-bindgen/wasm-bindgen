//! Checking for the wasm32 target

use crate::build::utils;
use crate::build::BuildProfile;
use crate::build::PBAR;
use super::emoji;
use super::versions::MIN_RUSTC_VERSION;
use anyhow::{anyhow, bail, Context, Result};
use core::str;
use log::error;
use log::info;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

struct Wasm32Check {
    rustc_path: PathBuf,
    sysroot: PathBuf,
    found: bool,
    is_rustup: bool,
}

impl fmt::Display for Wasm32Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let target = "wasm32-unknown-unknown";

        if !self.found {
            let rustup_string = if self.is_rustup {
                "It looks like Rustup is being used.".to_owned()
            } else {
                format!("It looks like Rustup is not being used. For non-Rustup setups, the {} target needs to be installed manually.", target)
            };

            writeln!(
                f,
                "{} target not found in sysroot: {:?}",
                target, self.sysroot
            )
            .and_then(|_| {
                writeln!(
                    f,
                    "\nUsed rustc from the following path: {:?}",
                    self.rustc_path
                )
            })
            .and_then(|_| writeln!(f, "{}", rustup_string))
        } else {
            write!(
                f,
                "sysroot: {:?}, rustc path: {:?}, was found: {}, isRustup: {}",
                self.sysroot, self.rustc_path, self.found, self.is_rustup
            )
        }
    }
}

/// Ensure that `rustup` has the `wasm32-unknown-unknown` target installed for
/// current toolchain
pub fn check_for_wasm32_target() -> Result<()> {
    let msg = format!("{}Checking for the Wasm target...", emoji::TARGET);
    PBAR.info(&msg);

    // Check if wasm32 target is present, otherwise bail.
    match check_wasm32_target() {
        Ok(ref wasm32_check) if wasm32_check.found => Ok(()),
        Ok(wasm32_check) => bail!("{}", wasm32_check),
        Err(err) => Err(err),
    }
}

/// Get rustc's sysroot as a PathBuf
fn get_rustc_sysroot() -> Result<PathBuf> {
    let command = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()?;

    if command.status.success() {
        Ok(String::from_utf8(command.stdout)?.trim().into())
    } else {
        Err(anyhow!(
            "Getting rustc's sysroot wasn't successful. Got {}",
            command.status
        ))
    }
}

/// Get wasm32-unknown-unknown target libdir
fn get_rustc_wasm32_unknown_unknown_target_libdir() -> Result<PathBuf> {
    let command = Command::new("rustc")
        .args([
            "--target",
            "wasm32-unknown-unknown",
            "--print",
            "target-libdir",
        ])
        .output()?;

    if command.status.success() {
        Ok(String::from_utf8(command.stdout)?.trim().into())
    } else {
        Err(anyhow!(
            "Getting rustc's wasm32-unknown-unknown target wasn't successful. Got {}",
            command.status
        ))
    }
}

fn does_wasm32_target_libdir_exist() -> bool {
    let result = get_rustc_wasm32_unknown_unknown_target_libdir();

    match result {
        Ok(wasm32_target_libdir_path) => {
            if wasm32_target_libdir_path.exists() {
                info!(
                    "Found wasm32-unknown-unknown in {:?}",
                    wasm32_target_libdir_path
                );
                true
            } else {
                info!(
                    "Failed to find wasm32-unknown-unknown in {:?}",
                    wasm32_target_libdir_path
                );
                false
            }
        }
        Err(_) => {
            error!("Some error in getting the target libdir!");
            false
        }
    }
}

fn check_wasm32_target() -> Result<Wasm32Check> {
    let sysroot = get_rustc_sysroot()?;
    let rustc_path = which::which("rustc")?;

    if does_wasm32_target_libdir_exist() {
        Ok(Wasm32Check {
            rustc_path,
            sysroot,
            found: true,
            is_rustup: false,
        })
    // If it doesn't exist, then we need to check if we're using rustup.
    } else {
        // If sysroot contains "rustup", then we can assume we're using rustup
        // and use rustup to add the wasm32-unknown-unknown target.
        if sysroot.to_string_lossy().contains("rustup") {
            rustup_add_wasm_target().map(|()| Wasm32Check {
                rustc_path,
                sysroot,
                found: true,
                is_rustup: true,
            })
        } else {
            Ok(Wasm32Check {
                rustc_path,
                sysroot,
                found: false,
                is_rustup: false,
            })
        }
    }
}

/// Add wasm32-unknown-unknown using `rustup`.
fn rustup_add_wasm_target() -> Result<()> {
    let mut cmd = Command::new("rustup");
    cmd.arg("target").arg("add").arg("wasm32-unknown-unknown");
    utils::run(cmd, "rustup").context("Adding the wasm32-unknown-unknown target with rustup")?;

    Ok(())
}

/// Ensure that `rustc` is present and that it is >= 1.30.0
pub fn check_rustc_version() -> Result<String> {
    let local_minor_version = rustc_minor_version();
    match local_minor_version {
        Some(mv) => {
            if mv < MIN_RUSTC_VERSION.minor as u32 {
                bail!(
                    "Your version of Rust, '1.{}', is not supported. Please install Rust version {} or higher.",
                    mv,
                    *MIN_RUSTC_VERSION
                )
            } else {
                Ok(mv.to_string())
            }
        }
        None => bail!("We can't figure out what your Rust version is- which means you might not have Rust installed. Please install Rust version 1.30.0 or higher."),
    }
}

// from https://github.com/alexcrichton/proc-macro2/blob/79e40a113b51836f33214c6d00228934b41bd4ad/build.rs#L44-L61
fn rustc_minor_version() -> Option<u32> {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => return None,
            }
        };
    }
    let output = otry!(Command::new("rustc").arg("--version").output().ok());
    let version = otry!(str::from_utf8(&output.stdout).ok());
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    otry!(pieces.next()).parse().ok()
}

/// Run `cargo build` targetting `wasm32-unknown-unknown`.
pub fn cargo_build_wasm(
    path: &Path,
    profile: BuildProfile,
    extra_options: &[String],
) -> Result<()> {
    let msg = format!("{}Compiling to Wasm...", emoji::CYCLONE);
    PBAR.info(&msg);

    let mut cmd = Command::new("cargo");
    cmd.current_dir(path).arg("build").arg("--lib");

    if PBAR.quiet() {
        cmd.arg("--quiet");
    }

    match profile {
        BuildProfile::Profiling => {
            // Once there are DWARF debug info consumers, force enable debug
            // info, because builds that use the release cargo profile disables
            // debug info.
            //
            // cmd.env("RUSTFLAGS", "-g");
            cmd.arg("--release");
        }
        BuildProfile::Release => {
            cmd.arg("--release");
        }
        BuildProfile::Dev => {
            // Plain cargo builds use the dev cargo profile, which includes
            // debug info by default.
        }
        BuildProfile::Custom(arg) => {
            cmd.arg("--profile").arg(arg);
        }
    }

    cmd.arg("--target").arg("wasm32-unknown-unknown");

    // The `cargo` command is executed inside the directory at `path`, so relative paths set via extra options won't work.
    // To remedy the situation, all detected paths are converted to absolute paths.
    let mut handle_path = false;
    let extra_options_with_absolute_paths = extra_options
        .iter()
        .map(|option| -> Result<String> {
            let value = if handle_path && Path::new(option).is_relative() {
                std::env::current_dir()?
                    .join(option)
                    .to_str()
                    .ok_or_else(|| anyhow!("path contains non-UTF-8 characters"))?
                    .to_string()
            } else {
                option.to_string()
            };
            handle_path = matches!(&**option, "--target-dir" | "--out-dir" | "--manifest-path");
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()?;
    cmd.args(extra_options_with_absolute_paths);

    utils::run(cmd, "cargo build").context("Compiling your crate to WebAssembly failed")?;
    Ok(())
}
