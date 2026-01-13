use anyhow::{bail, Result};
use binary_install::Cache;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn find_wasm_opt(
    cache: &Cache,
    version: &str,
    install_permitted: bool,
) -> Result<Option<PathBuf>> {
    // First attempt to look up in PATH. If found assume it works.
    if let Ok(path) = which::which("wasm-opt") {
        log::info!("found wasm-opt at {:?}", path);
        return Ok(Some(path));
    }

    let url = match prebuilt_url(version) {
        Ok(url) => url,
        Err(e) => {
            log::warn!("no prebuilt wasm-opt binaries available: {}", e);
            return Ok(None);
        }
    };

    let binaries = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "aarch64") | ("linux", "x86_64") => vec!["bin/wasm-opt"],
        ("macos", "x86_64") | ("macos", "aarch64") => vec!["bin/wasm-opt", "lib/libbinaryen.dylib"],
        ("windows", "x86_64") => vec!["bin/wasm-opt.exe"],
        _ => return Ok(None),
    };

    log::info!("Downloading wasm-opt...");
    match cache.download(install_permitted, "wasm-opt", &binaries, &url)? {
        Some(download) => Ok(Some(download.binary("bin/wasm-opt")?)),
        None => {
            log::info!("Skipping wasm-opt as no downloading was requested");
            Ok(None)
        }
    }
}

fn prebuilt_url(version: &str) -> Result<String> {
    let target = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "aarch64") => "aarch64-linux",
        ("linux", "x86_64") => "x86_64-linux",
        ("macos", "x86_64") => "x86_64-macos",
        ("macos", "aarch64") => "arm64-macos",
        ("windows", "x86_64") => "x86_64-windows",
        _ => bail!("Unrecognized target for wasm-opt"),
    };

    Ok(format!(
        "https://github.com/WebAssembly/binaryen/releases/download/{vers}/binaryen-{vers}-{target}.tar.gz",
        vers = version,
        target = target,
    ))
}

pub fn run(
    cache: &Cache,
    out_dir: &Path,
    args: &[String],
    version: &str,
    install_permitted: bool,
) -> Result<()> {
    let wasm_opt_path = match find_wasm_opt(cache, version, install_permitted)? {
        Some(path) => path,
        None => return Ok(()),
    };

    log::info!("Optimizing wasm binaries with `wasm-opt`...");

    for file in out_dir.read_dir()? {
        let file = file?;
        let path = file.path();
        if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }

        let tmp = path.with_extension("wasm-opt.wasm");
        let mut cmd = Command::new(&wasm_opt_path);
        cmd.arg(&path).arg("-o").arg(&tmp).args(args);

        log::info!("Running: {:?}", cmd);
        let status = cmd.status()?;
        if !status.success() {
            bail!("wasm-opt failed");
        }

        std::fs::rename(&tmp, &path)?;
    }

    Ok(())
}
