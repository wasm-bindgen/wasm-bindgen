use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[clap(name = "cargo-wasm-bindgen", bin_name = "cargo", version)]
enum CargoOpt {
    #[clap(name = "wasm-bindgen")]
    WasmBindgen(Opt),
}

#[derive(Parser, Debug)]
struct Opt {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List installed versions
    #[clap(name = "--list")]
    List,

    /// List configured shortcuts
    #[clap(name = "--list-shortcuts")]
    ListShortcuts,

    /// Run a specific package version
    #[clap(name = "--run")]
    Run(RunArgs),

    /// Create a new shortcut
    #[clap(name = "--create-shortcut")]
    CreateShortcut(CreateShortcutArgs),

    /// Delete a shortcut
    #[clap(name = "--delete-shortcut")]
    DeleteShortcut(DeleteShortcutArgs),

    /// Run a shortcut
    #[clap(external_subcommand)]
    Shortcut(Vec<String>),
}

#[derive(Args, Debug)]
struct RunArgs {
    /// The install package name (e.g. wasm-bindgen-cli)
    install_package: String,

    #[clap(long)]
    version: Option<String>,

    /// Captures "version-package binary" or just "binary"
    #[clap(allow_hyphen_values = true)]
    rest: Vec<String>,

    /// Arguments to pass to the binary
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Debug)]
struct CreateShortcutArgs {
    shortcut: String,
    install_package: String,
    binary_name: String,
}

#[derive(Args, Debug)]
struct DeleteShortcutArgs {
    shortcut: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Shortcut {
    install_package: String,
    binary_name: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct Config {
    #[serde(default)]
    shortcuts: HashMap<String, Shortcut>,
    #[serde(default)]
    metadata_mappings: HashMap<String, String>,
}

impl Config {
    fn default_config() -> Self {
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "build".to_string(),
            Shortcut {
                // TODO merge into cli package?
                install_package: "wasm-bindgen-build".to_string(),
                // install_package: "wasm-bindgen-cli".to_string(),
                binary_name: "wasm-bindgen-build".to_string(),
            },
        );
        shortcuts.insert(
            "cli".to_string(),
            Shortcut {
                install_package: "wasm-bindgen-cli".to_string(),
                binary_name: "wasm-bindgen".to_string(),
            },
        );
        shortcuts.insert(
            "test-runner".to_string(),
            Shortcut {
                install_package: "wasm-bindgen-cli".to_string(),
                binary_name: "wasm-bindgen-test-runner".to_string(),
            },
        );
        shortcuts.insert(
            "wasm2es6js".to_string(),
            Shortcut {
                install_package: "wasm-bindgen-cli".to_string(),
                binary_name: "wasm2es6js".to_string(),
            },
        );

        let mut metadata_mappings = HashMap::new();
        metadata_mappings.insert("wasm-bindgen-cli".to_string(), "wasm-bindgen".to_string());
        metadata_mappings.insert("wasm-bindgen-build".to_string(), "wasm-bindgen".to_string());

        Config {
            shortcuts,
            metadata_mappings,
        }
    }

    fn merge(&mut self, other: Config) {
        self.shortcuts.extend(other.shortcuts);
        self.metadata_mappings.extend(other.metadata_mappings);
    }
}

#[derive(Debug)]
struct PackageInfo {
    name: String,
    versions: Vec<VersionInfo>,
}

#[derive(Debug)]
struct VersionInfo {
    version: String,
    binaries: Vec<BinaryInfo>,
}

#[derive(Debug)]
struct BinaryInfo {
    name: String,
    shortcuts: Vec<String>,
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
    Ok(base.join("cargo-wasm-bindgen"))
}

fn get_config_dir() -> Result<PathBuf> {
    let base = if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else {
        home::home_dir()
            .context("Could not determine home directory")?
            .join(".config")
    };
    Ok(base.join("cargo-wasm-bindgen"))
}

fn get_config_path() -> Result<PathBuf> {
    if let Ok(path) = env::var("CARGO_WASM_BINDGEN_CONFIG_TOML") {
        return Ok(PathBuf::from(path));
    }
    Ok(get_config_dir()?.join("config.toml"))
}

fn load_config() -> Result<Config> {
    let mut config = Config::default_config();

    let config_path = get_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let loaded = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file at {:?}", config_path))?;
        config.merge(loaded);
    }

    Ok(config)
}

fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory at {:?}", parent))?;
    }
    let content = toml::to_string(config).context("Failed to serialize config")?;
    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, content)
        .with_context(|| format!("Failed to write temp config file at {:?}", tmp_path))?;
    fs::rename(&tmp_path, &path).with_context(|| {
        format!(
            "Failed to move config file from {:?} to {:?}",
            tmp_path, path
        )
    })?;
    Ok(())
}

fn main() -> Result<()> {
    let CargoOpt::WasmBindgen(opt) = CargoOpt::parse();
    if let Err(e) = run(opt) {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run(opt: Opt) -> Result<()> {
    let mut config = load_config()?;
    match opt.command {
        Commands::List => {
            let packages = list_packages(&config)?;
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
                            print!("    Command: {}", bin.name);
                            if !bin.shortcuts.is_empty() {
                                let joined = bin.shortcuts.join(", ");
                                print!(" (shortcuts: {})", joined);
                            }
                            println!();
                        }
                    }
                }
            }
        }
        Commands::ListShortcuts => {
            println!("Shortcuts:");
            for (name, shortcut) in &config.shortcuts {
                println!(
                    "  {} -> package: {}, binary: {}",
                    name, shortcut.install_package, shortcut.binary_name
                );
            }
            println!("\nMetadata Mappings:");
            for (install, meta) in &config.metadata_mappings {
                println!("  {} -> {}", install, meta);
            }
        }
        Commands::CreateShortcut(args) => {
            create_shortcut(args, &mut config)?;
            println!("Shortcut created.");
        }
        Commands::DeleteShortcut(args) => {
            delete_shortcut(args, &mut config)?;
            println!("Shortcut deleted.");
        }
        Commands::Run(args) => {
            let code = handle_run_command(args, &config)?;
            if code != 0 {
                std::process::exit(code);
            }
        }
        Commands::Shortcut(args) => {
            if args.is_empty() {
                bail!("No shortcut or command provided.");
            }
            let shortcut_name = &args[0];
            let shortcut_args = &args[1..];
            if let Some(shortcut) = config.shortcuts.get(shortcut_name) {
                let version_package = config
                    .metadata_mappings
                    .get(&shortcut.install_package)
                    .cloned();
                let code = run_package(
                    &shortcut.install_package,
                    None,
                    version_package.as_deref(),
                    &shortcut.binary_name,
                    shortcut_args,
                )?;
                if code != 0 {
                    std::process::exit(code);
                }
            } else {
                eprintln!("Error: shortcut `{}` not found.", shortcut_name);
                eprintln!(
                    "Available shortcuts: {:?}",
                    config.shortcuts.keys().collect::<Vec<_>>()
                );
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn list_packages(config: &Config) -> Result<Vec<PackageInfo>> {
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
        let versions = get_versions(&pkg_name, &pkg_path, config)?;
        if !versions.is_empty() {
            packages.push(PackageInfo {
                name: pkg_name,
                versions,
            });
        }
    }
    Ok(packages)
}

fn get_versions(pkg_name: &str, pkg_path: &Path, config: &Config) -> Result<Vec<VersionInfo>> {
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
                let shortcuts = get_matching_shortcuts(pkg_name, &bin_name, config);
                binaries.push(BinaryInfo {
                    name: bin_name,
                    shortcuts,
                });
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

fn get_matching_shortcuts(pkg_name: &str, binary_name: &str, config: &Config) -> Vec<String> {
    let mut matches: Vec<_> = config
        .shortcuts
        .iter()
        .filter(|(_, s)| s.install_package == pkg_name && s.binary_name == binary_name)
        .map(|(k, _)| k.clone())
        .collect();
    matches.sort();
    matches
}

fn create_shortcut(args: CreateShortcutArgs, config: &mut Config) -> Result<()> {
    config.shortcuts.insert(
        args.shortcut.clone(),
        Shortcut {
            install_package: args.install_package,
            binary_name: args.binary_name,
        },
    );
    save_config(config).context("Failed to save config")?;
    Ok(())
}

fn delete_shortcut(args: DeleteShortcutArgs, config: &mut Config) -> Result<()> {
    if config.shortcuts.remove(&args.shortcut).is_some() {
        save_config(config).context("Failed to save config")?;
        Ok(())
    } else {
        bail!("Shortcut `{}` not found.", args.shortcut);
    }
}

fn handle_run_command(args: RunArgs, _config: &Config) -> Result<i32> {
    let (version_package, binary_name) = match args.rest.len() {
        1 => (None, args.rest[0].clone()),
        2 => (Some(args.rest[0].clone()), args.rest[1].clone()),
        _ => {
            bail!(
                "Error: expected 1 or 2 positional arguments after install-package (binary-name, or version-package binary-name). Found: {:?}",
                args.rest
            );
        }
    };

    run_package(
        &args.install_package,
        args.version,
        version_package.as_deref(),
        &binary_name,
        &args.args,
    )
}

fn run_package(
    install_package: &str,
    explicit_version: Option<String>,
    version_package: Option<&str>,
    binary_name: &str,
    args: &[String],
) -> Result<i32> {
    let version = if let Some(v) = explicit_version {
        v
    } else if let Some(vp) = version_package {
        resolve_version(vp).context("Failed to resolve version from dependencies")?
    } else {
        resolve_version(install_package).context("Failed to resolve version from dependencies")?
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
    let bin_dir = cache_dir.join("bin");
    if bin_dir.exists() {
        return Ok(());
    }

    println!("Installing {} version {}...", package, version);
    fs::create_dir_all(cache_dir).context("Failed to create cache directory for package")?;

    let mut cmd = Command::new("cargo");
    cmd.arg("install")
        .arg(package)
        .arg("--version")
        .arg(version)
        .arg("--root")
        .arg(cache_dir);

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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::io::Write;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    struct TestContext {
        _temp_dir: TempDir,
        cache_dir: PathBuf,
        #[allow(dead_code)]
        config_dir: PathBuf,
        #[allow(dead_code)]
        cargo_bin: PathBuf,
    }

    fn setup() -> Result<TestContext> {
        let temp_dir = tempfile::tempdir()?;
        let root = temp_dir.path();

        let cache_dir = root.join("cache");
        let config_dir = root.join("config");
        let bin_dir = root.join("bin");

        fs::create_dir_all(&cache_dir)?;
        fs::create_dir_all(&config_dir)?;
        fs::create_dir_all(&bin_dir)?;

        unsafe {
            env::set_var("CARGO_WASM_BINDGEN_CACHE_DIR", &cache_dir);
            env::set_var(
                "CARGO_WASM_BINDGEN_CONFIG_TOML",
                config_dir.join("config.toml"),
            );
            env::remove_var("XDG_CACHE_HOME");
            env::remove_var("XDG_CONFIG_HOME");
        }

        let cargo_bin = bin_dir.join("cargo");
        let mut f = fs::File::create(&cargo_bin)?;

        let script = r###"#!/bin/sh
echo "Called with: $*" >> /tmp/cargo-fake.log
if [ "$1" = "metadata" ]; then
    # Return valid metadata with multiple packages
    echo '{"packages":[{"name":"test-pkg","version":"1.0.0","id":"test-pkg 1.0.0 (registry+...)","dependencies":[],"targets":[],"features":{},"manifest_path":"/tmp/Cargo.toml"},{"name":"wasm-bindgen","version":"0.2.92","id":"wasm-bindgen 0.2.92 (registry+...)","dependencies":[],"targets":[],"features":{},"manifest_path":"/tmp/Cargo.toml"}],"resolve":{"nodes":[{"id":"test-pkg 1.0.0 (registry+...)","dependencies":[],"deps":[],"features":[]},{"id":"wasm-bindgen 0.2.92 (registry+...)","dependencies":[],"deps":[],"features":[]}]},"workspace_members":["test-pkg 1.0.0 (registry+...)"],"workspace_root":"/tmp","target_directory":"/tmp/target","version":1}'
elif [ "$1" = "install" ]; then
    # args: install PKG --version VER --root ROOT
    PKG="$2"
    VER="$4"
    ROOT="$6"
    
    mkdir -p "$ROOT/bin"
    
    # Define binaries based on package
    BINS=""
    if [ "$PKG" = "wasm-bindgen-cli" ]; then
        BINS="wasm-bindgen wasm-bindgen-test-runner wasm2es6js"
    elif [ "$PKG" = "test-pkg" ]; then
        BINS="test-pkg test-bin"
    else
        BINS="$PKG"
    fi
    
    JSON_BINS=""
    for BIN in $BINS; do
        path="$ROOT/bin/$BIN"
        echo "#!/bin/sh" > "$path"
        # If the binary name is 'fail-bin', exit with 1
        if [ "$BIN" = "fail-bin" ]; then
            echo "exit 1" >> "$path"
        else
            echo "echo \"Running $BIN with args: \$*\"" >> "$path"
            echo "exit 0" >> "$path"
        fi
        chmod +x "$path"
        
        if [ -z "$JSON_BINS" ]; then
            JSON_BINS="\"$BIN\""
        else
            JSON_BINS="$JSON_BINS, \"$BIN\""
        fi
    done

    # Generate .crates2.json for list command
    echo "{
      \"installs\": {
        \"$PKG $VER (registry+...)\": {
          \"bins\": [$JSON_BINS]
        }
      }
    }" > "$ROOT/.crates2.json"

else
    echo "Unknown command $1"
    exit 1
fi
"###;
        f.write_all(script.as_bytes())?;

        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&cargo_bin)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&cargo_bin, perms)?;
        }

        let path_env = env::var_os("PATH").unwrap_or_default();
        let mut new_path = vec![bin_dir.clone()];
        new_path.extend(env::split_paths(&path_env));
        let new_path_os = env::join_paths(new_path)?;
        unsafe {
            env::set_var("PATH", new_path_os);
            env::set_var("CARGO", &cargo_bin);
        }

        Ok(TestContext {
            _temp_dir: temp_dir,
            cache_dir,
            config_dir,
            cargo_bin,
        })
    }

    #[test]
    #[serial]
    fn test_list_empty() -> Result<()> {
        let _ctx = setup()?;
        let config = load_config()?;
        let packages = list_packages(&config)?;
        assert!(packages.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_and_delete_shortcut() -> Result<()> {
        let _ctx = setup()?;
        let mut config = load_config()?;

        let args = CreateShortcutArgs {
            shortcut: "my-shortcut".to_string(),
            install_package: "pkg".to_string(),
            binary_name: "bin".to_string(),
        };

        create_shortcut(args, &mut config)?;

        let loaded = load_config()?;
        assert!(loaded.shortcuts.contains_key("my-shortcut"));

        let del_args = DeleteShortcutArgs {
            shortcut: "my-shortcut".to_string(),
        };
        delete_shortcut(del_args, &mut config)?;

        let loaded_after = load_config()?;
        assert!(!loaded_after.shortcuts.contains_key("my-shortcut"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_list_installed() -> Result<()> {
        let _ctx = setup()?;

        let args = RunArgs {
            install_package: "test-pkg".to_string(),
            version: None,
            rest: vec!["test-pkg".to_string()],
            args: vec![],
        };
        let config = load_config()?;
        handle_run_command(args, &config)?;

        let packages = list_packages(&config)?;
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].name, "test-pkg");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_list_shortcuts() -> Result<()> {
        let _ctx = setup()?;
        let config = load_config()?;
        assert!(config.shortcuts.contains_key("build"));
        assert!(config.shortcuts.contains_key("cli"));
        assert!(config.shortcuts.contains_key("test-runner"));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_run_default_shortcut_cli() -> Result<()> {
        let _ctx = setup()?;
        let config = load_config()?;
        let shortcut = config.shortcuts.get("cli").unwrap();
        let version_pkg = config
            .metadata_mappings
            .get(&shortcut.install_package)
            .map(|s| s.as_str());

        let code = run_package(
            &shortcut.install_package,
            None,
            version_pkg,
            &shortcut.binary_name,
            &["--arg1".to_string()],
        )?;

        assert_eq!(code, 0);

        // Verify install
        let ver_dir = _ctx.cache_dir.join("wasm-bindgen-cli").join("0.2.92");
        assert!(ver_dir.exists());
        assert!(ver_dir.join("bin").join("wasm-bindgen").exists());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_run_binary_args_passed() -> Result<()> {
        let _ctx = setup()?;
        let args = RunArgs {
            install_package: "test-pkg".to_string(),
            version: None,
            rest: vec!["test-pkg".to_string()],
            args: vec!["--foo".to_string(), "bar".to_string()],
        };
        let config = load_config()?;
        let code = handle_run_command(args, &config)?;
        assert_eq!(code, 0);
        Ok(())
    }

    #[test]
    #[serial]
    fn test_run_exit_code_failure() -> Result<()> {
        let _ctx = setup()?;
        let args = RunArgs {
            install_package: "fail-bin".to_string(),
            version: Some("1.0.0".to_string()),
            rest: vec!["fail-bin".to_string()],
            args: vec![],
        };
        let config = load_config()?;
        let code = handle_run_command(args, &config)?;
        assert_eq!(code, 1);
        Ok(())
    }

    #[test]
    #[serial]
    fn test_run_version_package_lookup() -> Result<()> {
        let _ctx = setup()?;
        let args = RunArgs {
            install_package: "test-pkg".to_string(),
            version: None,
            rest: vec!["test-pkg".to_string(), "test-bin".to_string()],
            args: vec![],
        };
        let config = load_config()?;
        let code = handle_run_command(args, &config)?;
        assert_eq!(code, 0);

        // Check install dir: cache/test-pkg/1.0.0
        let ver_dir = _ctx.cache_dir.join("test-pkg").join("1.0.0");
        assert!(ver_dir.exists());
        assert!(ver_dir.join("bin").join("test-bin").exists());
        Ok(())
    }

    #[test]
    #[serial]
    fn test_binary_missing_in_version() -> Result<()> {
        let _ctx = setup()?;
        let args = RunArgs {
            install_package: "test-pkg".to_string(),
            version: None,
            rest: vec!["missing-bin".to_string()],
            args: vec![],
        };
        let config = load_config()?;

        // This should return Err because find_binary fails
        let result = handle_run_command(args, &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("binary `missing-bin` not found"));
        assert!(msg.contains("Available binaries"));
        assert!(msg.contains("test-pkg"));
        assert!(msg.contains("test-bin"));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_multiple_versions() -> Result<()> {
        let _ctx = setup()?;
        let config = load_config()?;

        // Install v1.0.0 (via metadata)
        let args1 = RunArgs {
            install_package: "test-pkg".to_string(),
            version: None,
            rest: vec!["test-pkg".to_string()],
            args: vec![],
        };
        handle_run_command(args1, &config)?;

        // Install v2.0.0 (explicit)
        let args2 = RunArgs {
            install_package: "test-pkg".to_string(),
            version: Some("2.0.0".to_string()),
            rest: vec!["test-pkg".to_string()],
            args: vec![],
        };
        handle_run_command(args2, &config)?;

        let packages = list_packages(&config)?;
        let pkg = packages
            .iter()
            .find(|p| p.name == "test-pkg")
            .expect("test-pkg not found");
        assert_eq!(pkg.versions.len(), 2);

        let v1 = pkg.versions.iter().find(|v| v.version == "1.0.0");
        let v2 = pkg.versions.iter().find(|v| v.version == "2.0.0");
        assert!(v1.is_some());
        assert!(v2.is_some());

        Ok(())
    }
}
