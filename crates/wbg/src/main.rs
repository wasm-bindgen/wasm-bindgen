use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use log::debug;
use log::info;
use log::warn;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[clap(name = "wbg", bin_name = "wbg", version)]
struct Opt {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List installed toolchains and binaries
    #[clap(name = "--list")]
    List,

    /// Run a command
    #[clap(external_subcommand)]
    Command(Vec<String>),
}

#[derive(Debug)]
struct PackageInfo {
    name: String,
    versions: Vec<VersionInfo>,
}

#[derive(Debug)]
struct VersionInfo {
    version: String,
    binaries: Vec<String>,
}

fn get_cache_dir() -> Result<PathBuf> {
    if let Ok(val) = env::var("CARGO_WASM_BINDGEN_CACHE_DIR") {
        return Ok(PathBuf::from(val));
    }
    let base = if let Ok(xdg) = env::var("XDG_CACHE_HOME") {
        PathBuf::from(xdg)
    } else {
        home::home_dir()
            .context("Could not determine home directory")?
            .join(".cache")
    };
    Ok(base.join("wbg"))
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::parse();
    if let Err(e) = run(opt) {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Commands::List => {
            let packages = list_packages()?;
            if packages.is_empty() {
                let cache_dir = get_cache_dir()?;
                println!("No packages installed in {}", cache_dir.display());
            } else {
                let cache_dir = get_cache_dir()?;
                println!("Installed packages in {}:", cache_dir.display());
                for pkg in packages {
                    for ver in pkg.versions {
                        println!("- {} v{}", pkg.name, ver.version);
                        for bin in ver.binaries {
                            print!("    Command: {}", bin);
                            println!();
                        }
                    }
                }
            }
        }
        Commands::Command(args) => {
            if args.is_empty() {
                bail!("No command provided.");
            }
            let command_name = &args[0];
            let command_args = &args[1..];
            let binary_name = format!("wasm-bindgen-{command_name}");
            let code = run_package(
                "wasm-bindgen",
                "wasm-bindgen-toolchain",
                None,
                &binary_name,
                command_args,
            )?;
            if code != 0 {
                std::process::exit(code);
            }
        }
    }
    Ok(())
}

fn list_packages() -> Result<Vec<PackageInfo>> {
    let cache_dir = get_cache_dir()?;
    if !cache_dir.exists() {
        return Ok(Vec::new());
    }
    let mut packages = Vec::new();
    let entries = fs::read_dir(&cache_dir).context("Failed to read cache directory")?;
    for entry in entries {
        let entry = entry?;
        let pkg_path = entry.path();
        if !pkg_path.is_dir() {
            continue;
        }
        let pkg_name = pkg_path.file_name().unwrap().to_string_lossy().to_string();
        if pkg_name == "config.toml" {
            continue;
        }
        let versions = get_versions(&pkg_path)?;
        if !versions.is_empty() {
            packages.push(PackageInfo {
                name: pkg_name,
                versions,
            });
        }
    }
    Ok(packages)
}

fn get_versions(pkg_path: &Path) -> Result<Vec<VersionInfo>> {
    let mut versions = Vec::new();
    let entries = fs::read_dir(pkg_path)
        .with_context(|| format!("Failed to read package directory {:?}", pkg_path))?;
    for entry in entries {
        let entry = entry?;
        let ver_path = entry.path();
        if !ver_path.is_dir() {
            continue;
        }
        let ver_str = ver_path.file_name().unwrap().to_string_lossy().to_string();
        if ver_path.join("bin").exists() {
            let bin_names = get_installed_binaries(&ver_path)?;
            let mut binaries = Vec::new();
            for bin_name in bin_names {
                binaries.push(bin_name);
            }
            versions.push(VersionInfo {
                version: ver_str,
                binaries,
            });
        }
    }
    Ok(versions)
}

fn get_installed_binaries(ver_path: &Path) -> Result<Vec<String>> {
    let mut binaries = Vec::new();
    let crates2_path = ver_path.join(".crates2.json");
    if !crates2_path.exists() {
        bail!("missing .crates2.json for {ver_path:?}");
    }
    let file =
        fs::File::open(&crates2_path).with_context(|| format!("cannot open {ver_path:?}"))?;
    let json = serde_json::from_reader::<_, serde_json::Value>(file)
        .with_context(|| format!("cannot read json at {ver_path:?}"))?;
    if let Some(installs) = json.get("installs").and_then(|v| v.as_object()) {
        for (_, val) in installs {
            if let Some(bins) = val.get("bins").and_then(|v| v.as_array()) {
                for b in bins {
                    if let Some(s) = b.as_str() {
                        binaries.push(s.to_string());
                    }
                }
            }
        }
    }
    binaries.sort();
    binaries.dedup();
    Ok(binaries)
}

fn run_package(
    dependency_package: &str,
    install_package: &str,
    explicit_version: Option<String>,
    binary_name: &str,
    args: &[String],
) -> Result<i32> {
    let version = if let Some(v) = explicit_version {
        v
    } else {
        resolve_version(dependency_package)
            .context("Failed to resolve version from dependencies")?
    };
    let cache_dir = get_cache_dir()?.join(install_package).join(&version);
    let bin_dir = cache_dir.join("bin");
    ensure_installed(install_package, &version, &cache_dir)?;
    let binary_path = find_binary(&bin_dir, binary_name).map_err(|available| {
        let mut msg = format!(
            "binary `{}` not found in package `{}` version `{}`.\nAvailable binaries:\n",
            binary_name, install_package, version
        );
        for name in available {
            msg.push_str(&format!("  - {}\n", name));
        }
        msg.push_str(
            "\nThe requested command might not be available in this version of the package.",
        );
        anyhow::anyhow!(msg)
    })?;
    let mut cmd = Command::new(&binary_path);
    cmd.args(args);
    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute binary at {:?}", binary_path))?;
    if let Some(code) = status.code() {
        Ok(code)
    } else {
        // Terminated by signal
        bail!("Process terminated by signal");
    }
}

fn resolve_version(package_name: &str) -> Result<String> {
    debug!("resolving {package_name}");
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .context("Failed to load cargo metadata")?;
    let resolve = metadata
        .resolve
        .ok_or_else(|| anyhow::anyhow!("No resolve graph found in metadata"))?;
    for node in resolve.nodes {
        let pkg = &metadata
            .packages
            .iter()
            .find(|p| p.id == node.id)
            .ok_or_else(|| anyhow::anyhow!("Package not found in metadata packages"))?;
        if pkg.name == package_name {
            return Ok(pkg.version.to_string());
        }
    }
    bail!(
        "Package `{}` not found in project dependencies.",
        package_name
    )
}

fn ensure_installed(package: &str, version: &str, cache_dir: &Path) -> Result<()> {
    debug!("ensuring install {package}:{version} in {cache_dir:?}");
    let bin_dir = cache_dir.join("bin");
    if bin_dir.exists() {
        debug!("already installed");
        // TODO actually ensure binaries are installed here
        return Ok(());
    }
    info!("Installing {} version {}...", package, version);
    fs::create_dir_all(cache_dir).context("Failed to create cache directory for package")?;
    // TODO take binary install code from workers-rs
    // fallback to full install
    let mut cmd = Command::new("cargo");
    cmd.arg("install")
        .arg("--quiet")
        .arg(package)
        .arg("--version")
        .arg(version)
        .arg("--root")
        .arg(cache_dir);
    // TODO find a way to get rid of this - hidden argument maybe
    if let Ok(force_local_dir) = std::env::var("WBG_FORCE_LOCAL_INSTALL_FOR_TESTING") {
        warn!("installing from local version at {force_local_dir}!");
        cmd.arg("--path").arg(force_local_dir);
    } else {
        bail!("wtf");
    }
    // TODO suppress output and only barf it out on error
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status = cmd.status().context("Failed to run cargo install")?;
    if !status.success() {
        let _ = fs::remove_dir_all(cache_dir);
        bail!("Failed to install {}.", package);
    }
    Ok(())
}

fn find_binary(bin_dir: &Path, binary_name: &str) -> Result<PathBuf, Vec<String>> {
    let path = bin_dir.join(binary_name);
    if path.exists() {
        return Ok(path);
    }
    let path_exe = bin_dir.join(format!("{}.exe", binary_name));
    if path_exe.exists() {
        return Ok(path_exe);
    }
    let entries: Vec<_> = match fs::read_dir(bin_dir) {
        Ok(read_dir) => read_dir.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
        Err(_) => Vec::new(),
    };
    if entries.len() == 1 {
        return Ok(entries[0].clone());
    }
    let names = entries
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();
    Err(names)
}
