mod manifest;
mod wasm_opt;

use anyhow::{anyhow, bail, Context, Error, Result};
use binary_install::Cache;
use clap::Parser;
use clap::{Args, ValueEnum};
use log::info;
use manifest::CrateData;
use path_clean::PathClean;
use serde::Deserialize;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

const DEFAULT_WASM_OPT_VERSION: &str = "125";

#[derive(Parser, Debug)]
#[command(name = "wasm-bindgen-build")]
struct BuildCommand {
    #[clap(flatten)]
    pub opts: BuildOptions,
}

#[derive(Debug, Args)]
#[command(allow_hyphen_values = true, trailing_var_arg = true)]
pub struct BuildOptions {
    /// The path to the Rust crate. If not set, searches up the path from the current directory.
    #[clap(overrides_with = "path")]
    pub path: Option<PathBuf>,

    /// The npm scope to use in package.json, if any.
    #[clap(long = "scope", short = 's', overrides_with = "scope")]
    pub scope: Option<String>,

    #[clap(
        long = "mode",
        short = 'm',
        default_value = "normal",
        overrides_with = "mode"
    )]
    /// Sets steps to be run. [possible values: no-install, normal, force]
    pub mode: InstallMode,

    #[clap(long = "no-typescript", overrides_with = "typescript")]
    /// By default a *.d.ts file is generated for the generated JS file, but
    /// this flag will disable generating this TypeScript file.
    pub disable_dts: bool,
    #[clap(long = "typescript", overrides_with = "disable_dts")]
    pub typescript: bool,

    #[clap(long = "weak-refs", overrides_with = "no_weak_refs")]
    /// Enable usage of the JS weak references proposal.
    pub weak_refs: bool,
    #[clap(long = "no-weak-refs", overrides_with = "weak_refs")]
    pub no_weak_refs: bool,

    #[clap(long = "reference-types", overrides_with = "no_reference_types")]
    /// Enable usage of WebAssembly reference types.
    pub reference_types: bool,
    #[clap(long = "no-reference-types", overrides_with = "reference_types")]
    pub no_reference_types: bool,

    #[clap(
        long = "target",
        short = 't',
        default_value = "bundler",
        overrides_with = "target"
    )]
    /// Sets the target environment. [possible values: bundler, nodejs, web, no-modules, deno]
    pub target: Target,

    #[clap(long = "debug")]
    /// Deprecated. Renamed to `--dev`.
    pub debug: bool,

    #[clap(long = "dev")]
    /// Create a development build. Enable debug info, and disable
    /// optimizations.
    pub dev: bool,

    #[clap(long = "release")]
    /// Create a release build. Enable optimizations and disable debug info.
    pub release: bool,

    #[clap(long = "profiling")]
    /// Create a profiling build. Enable optimizations and debug info.
    pub profiling: bool,

    #[clap(long = "profile", overrides_with = "profile")]
    /// User-defined profile with --profile flag
    pub profile: Option<String>,

    #[clap(long = "out-dir", short = 'd', overrides_with = "out_dir")]
    /// Sets the output directory with a relative path.
    pub out_dir: Option<String>,

    #[clap(long = "out-name", overrides_with = "out_name")]
    /// Sets the output file names. Defaults to package name.
    pub out_name: Option<String>,

    #[clap(long = "no-pack", alias = "no-package", overrides_with = "pack")]
    /// Option to not generate a package.json
    pub no_pack: bool,
    #[clap(long = "pack", overrides_with = "no_pack")]
    pub pack: bool,

    #[clap(long = "no-opt", alias = "no-optimization", overrides_with = "opt")]
    /// Option to skip optimization with wasm-opt
    pub no_opt: bool,
    #[clap(long = "opt", overrides_with = "no_opt")]
    pub opt: bool,

    #[clap(long = "wasm-opt-version", overrides_with = "wasm_opt_version")]
    pub wasm_opt_version: Option<String>,

    /// List of extra options to pass to `cargo build`
    pub extra_options: Vec<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum, Default)]
pub enum InstallMode {
    NoInstall,
    #[default]
    Normal,
    Force,
}

impl InstallMode {
    pub fn install_permitted(&self) -> bool {
        !matches!(self, InstallMode::NoInstall)
    }
}

impl FromStr for InstallMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "no-install" => Ok(InstallMode::NoInstall),
            "normal" => Ok(InstallMode::Normal),
            "force" => Ok(InstallMode::Force),
            _ => bail!("Unknown mode: {}", s),
        }
    }
}

impl fmt::Display for InstallMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstallMode::NoInstall => write!(f, "no-install"),
            InstallMode::Normal => write!(f, "normal"),
            InstallMode::Force => write!(f, "force"),
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Target {
    #[default]
    Bundler,
    Web,
    Nodejs,
    NoModules,
    Deno,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Target::Bundler => "bundler",
            Target::Web => "web",
            Target::Nodejs => "nodejs",
            Target::NoModules => "no-modules",
            Target::Deno => "deno",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Target {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "bundler" | "browser" => Ok(Target::Bundler),
            "web" => Ok(Target::Web),
            "nodejs" => Ok(Target::Nodejs),
            "no-modules" => Ok(Target::NoModules),
            "deno" => Ok(Target::Deno),
            _ => bail!("Unknown target: {}", s),
        }
    }
}

/// The build profile controls whether optimizations, debug info, and assertions
/// are enabled or disabled.
#[derive(Clone, Debug)]
pub enum BuildProfile {
    /// Enable assertions and debug info. Disable optimizations.
    Dev,
    /// Enable optimizations. Disable assertions and debug info.
    Release,
    /// Enable optimizations and debug info. Disable assertions.
    Profiling,
    /// User-defined profile with --profile flag
    Custom(String),
}

#[derive(Parser, Debug)]
#[command(
    name = "wasm-bindgen-build",
    ignore_errors = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
struct PreParseCommand {
    #[clap(flatten)]
    pub opts: PreParseOptions,
}

#[derive(Debug, Args)]
pub struct PreParseOptions {
    #[clap()]
    pub path: Option<PathBuf>,

    #[clap(long = "out-name")]
    pub out_name: Option<String>,
}

pub fn run_cli_with_args<I, T>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = args.into_iter().collect::<Vec<_>>();
    let args_pre = PreParseCommand::parse_from(args.clone());
    let opts_pre = args_pre.opts;
    let crate_path = opts_pre.path.unwrap_or_else(|| PathBuf::from("."));
    let crate_path = fs::canonicalize(&crate_path).with_context(|| {
        anyhow!(
            "failed to canonicalize crate path: {}",
            crate_path.display()
        )
    })?;
    let crate_data = CrateData::new(&crate_path, opts_pre.out_name.clone())?;

    let args = {
        let mut final_args = Vec::<OsString>::new();
        let config_args = args_from_toml(&crate_data)
            .into_iter()
            .map(|x| Into::<OsString>::into(x))
            .collect::<Vec<_>>();
        let mut user_args = args.clone().into_iter().map(|x| x.into());
        if let Some(arg0) = user_args.next() {
            final_args.push(arg0.into());
        }
        final_args.extend(config_args);
        final_args.extend(user_args);
        BuildCommand::parse_from(final_args)
    };
    let mut opts = args.opts;

    if let Some(path) = &opts.path {
        if path.to_string_lossy().starts_with("--") {
            let path = opts.path.take().unwrap();
            opts.extra_options
                .insert(0, path.to_string_lossy().into_owned());
        }
    }

    let dev = opts.dev || opts.debug;
    let profile = match (dev, opts.release, opts.profiling, opts.profile) {
        (false, false, false, None) | (false, true, false, None) => BuildProfile::Release,
        (true, false, false, None) => BuildProfile::Dev,
        (false, false, true, None) => BuildProfile::Profiling,
        (false, false, false, Some(profile)) => BuildProfile::Custom(profile),
        _ => bail!(
            "Can only supply one of the --dev, --release, --profiling, or --profile 'name' flags"
        ),
    };

    let out_dir = match opts.out_dir {
        None => {
            let target_dir = crate_data.target_directory();
            let profile_dir_name = match profile {
                BuildProfile::Dev => "debug",
                BuildProfile::Release | BuildProfile::Profiling => "release",
                BuildProfile::Custom(ref name) => name,
            };
            target_dir.join("wasm-bindgen").join(profile_dir_name)
        }
        Some(out_dir) => crate_path.join(PathBuf::from(out_dir)),
    }
    .clean();

    let target_triple = {
        let mut extra_options_iter = opts.extra_options.iter();
        if extra_options_iter
            .by_ref()
            .any(|option| option == "--target")
        {
            extra_options_iter.next().map(|s| s.as_str())
        } else {
            None
        }
        .unwrap_or("wasm32-unknown-unknown")
    };

    info!("Checking crate configuration");
    crate_data.check_crate_config()?;
    info!("Crate is correctly configured.");

    info!("Building wasm");
    let wasm_path = cargo_build_wasm(
        &crate_path,
        &profile,
        &opts.extra_options,
        target_triple,
        &crate_data,
    )?;
    info!("Wasm built at {wasm_path:#?}.");

    if !out_dir.exists() {
        fs::create_dir_all(&out_dir)?;
    }

    {
        let pkg_file_path = out_dir.join("package.json");
        if pkg_file_path.exists() {
            info!("deleting existing package.json at {pkg_file_path:?}");
            fs::remove_file(pkg_file_path)?;
        }
    }

    info!("Building the wasm bindings");
    run_wasm_bindgen(
        &wasm_path,
        &out_dir,
        &opts.out_name,
        opts.disable_dts,
        opts.weak_refs,
        opts.reference_types,
        opts.target,
        &profile,
        &crate_data,
    )?;

    if !opts.no_opt {
        info!("Running wasm-opt");
        let args = crate_data
            .configured_profile(profile.clone())
            .wasm_opt_args();

        if let Some(mut args) = args {
            if opts.reference_types {
                args.push("--enable-reference-types".into());
            }
            let cache = Cache::new("wasm-bindgen-build")?;
            let version_num = opts
                .wasm_opt_version
                .unwrap_or_else(|| DEFAULT_WASM_OPT_VERSION.to_string());
            let version = format!("version_{}", version_num);
            wasm_opt::run(
                &cache,
                &out_dir,
                &args,
                &version,
                opts.mode.install_permitted(),
            )
            .context("wasm-opt failed")?;
        }
    }

    // TODO is this necessary? feels like it should be merged into wasm-bindgen if so
    if !opts.no_pack {
        info!("Writing package json to {out_dir:?}");
        crate_data.write_package_json(&out_dir, &opts.scope, opts.disable_dts, opts.target)?;
        info!("Wrote package.json to {out_dir:?}");
    }
    Ok(())
}

fn args_from_toml(crate_data: &CrateData) -> Vec<String> {
    let config = crate_data.wasm_bindgen_config();
    let mut config_args = Vec::new();
    if let Some(d) = &config.out_dir {
        config_args.push("--out-dir".to_string());
        config_args.push(d.clone());
    }
    if let Some(t) = config.target {
        config_args.push("--target".to_string());
        config_args.push(t.to_string());
    }
    if let Some(s) = &config.scope {
        config_args.push("--scope".to_string());
        config_args.push(s.clone());
    }
    if config.disable_dts.unwrap_or(false) {
        config_args.push("--no-typescript".to_string());
    }
    if config.weak_refs.unwrap_or(false) {
        config_args.push("--weak-refs".to_string());
    }
    if config.reference_types.unwrap_or(false) {
        config_args.push("--reference-types".to_string());
    }
    if config.no_pack.unwrap_or(false) {
        config_args.push("--no-pack".to_string());
    }
    if config.no_opt.unwrap_or(false) {
        config_args.push("--no-opt".to_string());
    }
    if let Some(v) = &config.wasm_opt_version {
        config_args.push("--wasm-opt-version".to_string());
        config_args.push(v.clone());
    }
    config_args
}

fn cargo_build_wasm(
    crate_path: &Path,
    profile: &BuildProfile,
    extra_options: &[String],
    target_triple: &str,
    crate_data: &CrateData,
) -> Result<PathBuf> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(crate_path);
    cmd.arg("build")
        .arg("--lib")
        .arg("--target")
        .arg(target_triple);

    match profile {
        BuildProfile::Dev => {}
        BuildProfile::Release | BuildProfile::Profiling => {
            cmd.arg("--release");
        }
        BuildProfile::Custom(name) => {
            cmd.arg("--profile").arg(name);
        }
    }

    cmd.args(extra_options);

    info!("Running: {:?}", cmd);
    let status = cmd.status().context("Failed to run cargo build")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    let target_dir = crate_data.target_directory();
    let profile_dir = match profile {
        BuildProfile::Dev => "debug",
        BuildProfile::Release | BuildProfile::Profiling => "release",
        BuildProfile::Custom(name) => name,
    };

    let crate_name = crate_data.crate_name();

    let wasm_path = target_dir
        .join(target_triple)
        .join(profile_dir)
        .join(format!("{}.wasm", crate_name));

    if !wasm_path.exists() {
        bail!("Could not find generated wasm at {:?}.", wasm_path);
    }

    Ok(wasm_path)
}

#[allow(clippy::too_many_arguments)]
fn run_wasm_bindgen(
    wasm_path: &Path,
    out_dir: &Path,
    out_name: &Option<String>,
    disable_dts: bool,
    weak_refs: bool,
    reference_types: bool,
    target: Target,
    profile: &BuildProfile,
    crate_data: &CrateData,
) -> Result<()> {
    let mut args = vec![
        "compile".to_string(),
        wasm_path.to_str().unwrap().to_string(),
        "--out-dir".to_string(),
        out_dir.to_str().unwrap().to_string(),
    ];

    if disable_dts {
        args.push("--no-typescript".to_string());
    } else {
        args.push("--typescript".to_string());
    }

    if weak_refs {
        args.push("--weak-refs".to_string());
    }

    if reference_types {
        args.push("--reference-types".to_string());
    }

    args.push("--target".to_string());
    args.push(target.to_string());

    if let Some(name) = out_name {
        args.push("--out-name".to_string());
        args.push(name.clone());
    }

    let configured_profile = crate_data.configured_profile(profile.clone());
    if configured_profile.wasm_bindgen_debug_js_glue() {
        args.push("--debug".to_string());
    }
    if !configured_profile.wasm_bindgen_demangle_name_section() {
        args.push("--no-demangle".to_string());
    }
    if configured_profile.wasm_bindgen_dwarf_debug_info() {
        args.push("--keep-debug".to_string());
    }
    if configured_profile.wasm_bindgen_omit_default_module_path() {
        args.push("--omit-default-module-path".to_string());
    }
    if configured_profile.wasm_bindgen_split_linked_modules() {
        args.push("--split-linked-modules".to_string());
    }

    info!("Running: cargo {}", args.join(" "));
    // TODO maybe check existence of wbg or have wbg pass an env var so we know who it's being run.
    let status = Command::new("wbg")
        .args(&args)
        .status()
        .context("Failed to run cargo wasm-bindgen cli")?;

    if !status.success() {
        bail!("wasm-bindgen failed");
    }

    Ok(())
}
