use crate::build::emoji::{CONFIG, DOWN_ARROW};
use crate::build::versions::{CUR_ESBUILD_VERSION, CUR_WASM_BINDGEN_VERSION, CUR_WASM_OPT_VERSION};
use crate::build::PBAR;
use anyhow::{bail, Context, Result};
use flate2::read::GzDecoder;
use heck::ToShoutySnakeCase;
use std::env;
use std::{
    fs::{create_dir_all, read_dir, OpenOptions},
    path::{Path, PathBuf},
};

pub trait GetBinary: BinaryDep {
    /// Get the given binary path for a binary dependency
    /// Returns both the path and a boolean indicating if user provided an override
    fn get_binary(&self, bin_name: Option<&str>) -> Result<(PathBuf, bool)> {
        let full_name = self.full_name();
        let name = self.name();
        let target = self.target();
        let version = self.version();

        // 1. First check {BIN_NAME}_BIN env override
        let check_env_var = bin_name.unwrap_or(name).to_shouty_snake_case() + "_BIN";
        if let Ok(custom_bin) = env::var(&check_env_var) {
            match which::which(&custom_bin) {
                Ok(resolved) => {
                    PBAR.info(&format!(
                        "{CONFIG}Using custom {full_name} from {check_env_var}: {}",
                        resolved.display()
                    ));
                    return Ok((resolved, true));
                }
                Err(_) => {
                    PBAR.warn(&format!("{check_env_var}={custom_bin} not found, falling back to internal {full_name} implementation"));
                }
            }
        }

        // 2. Then check the cache path
        let cache_path = cache_path(name, &version, target)?;
        let bin_path = cache_path.join(self.bin_path(bin_name)?);
        if bin_path.exists() {
            return Ok((bin_path, false));
        }

        // 3. Finally perform a download, clearing cache for this name and target first
        let url = self.download_url();
        let _ = remove_all_versions(name, target);
        PBAR.info(&format!("{DOWN_ARROW}Downloading {full_name}@{version}..."));
        download(&url, &cache_path)?;
        if !bin_path.exists() {
            bail!(
                "Unable to locate binary {} in {full_name}",
                bin_path.to_string_lossy()
            );
        }
        Ok((bin_path, false))
    }
}

pub trait BinaryDep: Sized {
    /// Returns the name of the binary
    fn name(&self) -> &'static str;

    /// Returns the full name of the binary
    fn full_name(&self) -> &'static str;

    /// Returns the target of the binary
    fn target(&self) -> &'static str;

    /// Returns the latest current version of the binary
    fn version(&self) -> String;

    /// Returns the URL for the binary to be downloaded
    /// as well as the path string within the archive to use
    fn download_url(&self) -> String;

    /// Get the relative path of the given binary in the package
    /// If None, returns the default binary
    fn bin_path(&self, name: Option<&str>) -> Result<String>;
}

impl<T: BinaryDep> GetBinary for T {}

const MAYBE_EXE: &str = if cfg!(windows) { ".exe" } else { "" };

/// For clearing the cache, remove all files for the given binary and target
fn remove_all_versions(name: &str, target: &str) -> Result<usize> {
    let prefix_name = format!("{name}-{target}-");
    let dir = dirs_next::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("wasm-bindgen-build");

    let mut deleted_count = 0;
    for entry in read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();

        if let Some(name_str) = file_name.to_str() {
            if name_str.starts_with(&prefix_name) {
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(&path)?;
                } else {
                    std::fs::remove_file(&path)?;
                }
                deleted_count += 1;
            }
        }
    }

    Ok(deleted_count)
}

/// Cache path for this binary instance
fn cache_path(name: &str, version: &str, target: &str) -> Result<PathBuf> {
    let path_name = format!("{name}-{target}-{version}{MAYBE_EXE}");
    let path = dirs_next::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("wasm-bindgen-build")
        .join(&path_name);
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}

#[cfg(target_family = "unix")]
fn fix_permissions(options: &mut OpenOptions) -> &mut OpenOptions {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o755)
}

#[cfg(target_family = "windows")]
fn fix_permissions(options: &mut OpenOptions) -> &mut OpenOptions {
    options
}

/// Download this binary instance into its cache path
fn download(url: &str, bin_dir: &Path) -> Result<()> {
    let mut res = ureq::get(url)
        .call()
        .with_context(|| format!("Failed to fetch URL {url}"))?;
    let body = res.body_mut().as_reader();
    let deflater = GzDecoder::new(body);
    let mut archive = tar::Archive::new(deflater);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path_stripped = entry.path()?.components().skip(1).collect::<PathBuf>();
        let bin_path = bin_dir.join(path_stripped);

        if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&bin_path)?;
        } else {
            if let Some(parent) = bin_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut options = std::fs::OpenOptions::new();
            let options = fix_permissions(&mut options);
            let mut file = options.create(true).write(true).open(&bin_path)?;
            std::io::copy(&mut entry, &mut file)?;
        }
    }

    Ok(())
}

pub struct Esbuild;

impl BinaryDep for Esbuild {
    fn full_name(&self) -> &'static str {
        "Esbuild"
    }
    fn name(&self) -> &'static str {
        "esbuild"
    }
    fn version(&self) -> String {
        CUR_ESBUILD_VERSION.to_string()
    }
    fn target(&self) -> &'static str {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("android", "arm") => "android-arm",
            ("android", "aarch64") => "android-arm64",
            ("android", "x86_64") => "android-x64",
            ("macos", "aarch64") => "darwin-arm64",
            ("macos", "x86_64") => "darwin-x64",
            ("freebsd", "aarch64") => "freebsd-arm64",
            ("freebsd", "x86_64") => "freebsd-x64",
            ("linux", "arm") => "linux-arm",
            ("linux", "aarch64") => "linux-arm64",
            ("linux", "x86") => "linux-ia32",
            ("linux", "powerpc64") => "linux-ppc64",
            ("linux", "s390x") => "linux-s390x",
            ("linux", "x86_64") => "linux-x64",
            ("netbsd", "aarch64") => "netbsd-arm64",
            ("netbsd", "x86_64") => "netbsd-x64",
            ("openbsd", "aarch64") => "openbsd-arm64",
            ("openbsd", "x86_64") => "openbsd-x64",
            ("solaris", "x86_64") => "sunos-x64",
            ("windows", "aarch64") => "win32-arm64",
            ("windows", "x86") => "win32-ia32",
            ("windows", "x86_64") => "win32-x64",
            _ => panic!("Platform unsupported by esbuild."),
        }
    }
    fn download_url(&self) -> String {
        let version = self.version();
        let target = self.target();
        format!("https://registry.npmjs.org/@esbuild/{target}/-/{target}-{version}.tgz")
    }
    fn bin_path(&self, name: Option<&str>) -> Result<String> {
        Ok(match name {
            None | Some("esbuild") => format!("bin/esbuild{MAYBE_EXE}"),
            Some(name) => bail!("Unknown binary {name} in {}", self.full_name()),
        })
    }
}

pub struct WasmOpt;

impl BinaryDep for WasmOpt {
    fn full_name(&self) -> &'static str {
        "Wasm Opt"
    }
    fn name(&self) -> &'static str {
        "wasm-opt"
    }
    fn version(&self) -> String {
        CUR_WASM_OPT_VERSION.to_owned()
    }
    fn target(&self) -> &'static str {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("macos", "aarch64") => "arm64-macos",
            ("macos", "x86_64") => "x86_64-macos",
            ("linux" | "freebsd" | "netbsd" | "openbsd" | "android", "aarch64") => "aarch64-linux",
            ("linux" | "freebsd" | "netbsd" | "openbsd", "x86_64") => "x86_64-linux",
            ("windows", "aarch64") => "arm64-windows",
            ("windows", "x86_64") => "x86_64-windows",
            _ => panic!("Platform unsupported for {}", self.full_name()),
        }
    }
    fn download_url(&self) -> String {
        let version = self.version();
        let target = self.target();
        format!("https://github.com/WebAssembly/binaryen/releases/download/version_{version}/binaryen-version_{version}-{target}.tar.gz")
    }
    fn bin_path(&self, name: Option<&str>) -> Result<String> {
        Ok(match name {
            None | Some("wasm-opt") => format!("bin/wasm-opt{MAYBE_EXE}"),
            Some(name) => bail!("Unknown binary {name} in {}", self.full_name()),
        })
    }
}

pub struct WasmBindgen;

impl BinaryDep for WasmBindgen {
    fn full_name(&self) -> &'static str {
        "Wasm Bindgen"
    }
    fn name(&self) -> &'static str {
        "wasm-bindgen"
    }
    fn version(&self) -> String {
        CUR_WASM_BINDGEN_VERSION.to_string()
    }
    fn target(&self) -> &'static str {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("macos", "aarch64") => "aarch64-apple-darwin",
            ("macos", "x86_64") => "x86_64-apple-darwin",
            ("linux" | "freebsd" | "netbsd" | "openbsd" | "android", "aarch64") => {
                "aarch64-unknown-linux-musl"
            }
            ("linux" | "freebsd" | "netbsd" | "openbsd", "x86_64") => "x86_64-unknown-linux-musl",
            ("windows", "x86_64" | "aarch64") => "x86_64-pc-windows-msvc",
            _ => panic!("Platform unsupported for {}", self.full_name()),
        }
    }
    fn download_url(&self) -> String {
        let version = self.version();
        let target = self.target();
        format!("https://github.com/wasm-bindgen/wasm-bindgen/releases/download/{version}/wasm-bindgen-{version}-{target}.tar.gz")
    }
    fn bin_path(&self, name: Option<&str>) -> Result<String> {
        Ok(match name {
            None | Some("wasm-bindgen") => format!("wasm-bindgen{MAYBE_EXE}"),
            Some("wasm-bindgen-test-runner") => format!("wasm-bindgen-test-runner{MAYBE_EXE}"),
            Some(name) => bail!("Unknown binary {name} in {}", self.full_name()),
        })
    }
}
