use wasm_bindgen_cli_support::build::binary::{GetBinary, WasmOpt};
use wasm_bindgen_cli_support::build::lockfile::{DepCheckError, Lockfile};
use wasm_bindgen_cli_support::build::manifest;
use wasm_bindgen_cli_support::build::target;
use wasm_bindgen_cli_support::build::utils;
use wasm_bindgen_cli_support::build::versions::{
    CUR_WASM_BINDGEN_VERSION, MIN_WASM_BINDGEN_LIB_VERSION,
};
use wasm_bindgen_cli_support::build::PBAR;
use wasm_bindgen_cli_support::build::{emoji, BuildProfile};

use anyhow::{anyhow, bail, Context, Error, Result};
use clap::Args;
use clap::Parser;
use log::info;
use manifest::CrateData;
use path_clean::PathClean;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::time::Instant;
use utils::{create_pkg_dir, get_crate_path};

/// Everything required to configure and run the `wasm-bindgen-build` command.
#[allow(missing_docs)]
pub struct Build {
    pub crate_path: PathBuf,
    pub crate_data: manifest::CrateData,
    pub scope: Option<String>,
    pub disable_dts: bool,
    pub target: Target,
    pub no_opt: bool,
    pub profile: BuildProfile,
    pub out_dir: PathBuf,
    pub out_name: Option<String>,
    pub bindgen: Option<PathBuf>,
    pub bindgen_override: bool,
    pub extra_args: Vec<String>,
    pub extra_options: Vec<String>,
}

// TODO
// -m, --mode <MODE>          Sets steps to be run. [possible values: no-install, normal, force] [default: normal]
// --weak-refs            Enable usage of the JS weak references proposal
// --reference-types      Enable usage of WebAssembly reference types
// --no-pack              Option to not generate a package.json

/// Everything required to configure and run the `wasm-bindgen-build` command.
#[derive(Debug, Args, Default)]
#[command(allow_hyphen_values = true, trailing_var_arg = true)]
pub struct BuildOptions {
    /// The path to the Rust crate. If not set, searches up the path from the current directory.
    #[clap()]
    pub path: Option<PathBuf>,

    /// The npm scope to use in package.json, if any.
    #[clap(long = "scope", short = 's')]
    pub scope: Option<String>,

    #[clap(long = "no-typescript")]
    /// By default a *.d.ts file is generated for the generated JS file, but
    /// this flag will disable generating this TypeScript file.
    pub disable_dts: bool,

    #[clap(long = "target", short = 't', default_value = "bundler")]
    /// Sets the target environment. [possible values: bundler, nodejs, web, no-modules, deno, module]
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

    #[clap(long = "profile")]
    /// User-defined profile with --profile flag
    pub profile: Option<String>,

    #[clap(long = "out-dir", short = 'd', default_value = "pkg")]
    /// Sets the output directory with a relative path.
    pub out_dir: String,

    #[clap(long = "out-name")]
    /// Sets the output file names. Defaults to package name.
    pub out_name: Option<String>,

    #[clap(long = "no-opt", alias = "no-optimization")]
    /// Option to skip optimization with wasm-opt
    pub no_opt: bool,

    /// List of extra options to pass to `cargo build`
    pub extra_options: Vec<String>,

    #[clap(long, hide = true)]
    /// Pass-through for --no-panic-recovery
    pub no_panic_recovery: bool,
}

// TODO nodejs, web, no-modules, deno

/// What sort of output we're going to be generating and flags we're invoking
/// `wasm-bindgen` with.
#[derive(Clone, Copy, Debug, Default)]
pub enum Target {
    /// Default output mode or `--target bundler`, indicates output will be
    /// used with a bundle in a later step.
    #[default]
    Bundler,
    /// Correspond to `--target module` where the output uses ES module syntax
    /// with source phase imports for WebAssembly modules.
    Module,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Target::Bundler => "bundler",
            Target::Module => "module",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Target {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "bundler" | "browser" => Ok(Target::Bundler),
            "module" => Ok(Target::Module),
            _ => bail!("Unknown target: {}", s),
        }
    }
}

type BuildStep = fn(&mut Build) -> Result<()>;

impl Build {
    /// Construct a build command from the given options.
    pub fn try_from_opts(mut build_opts: BuildOptions) -> Result<Self> {
        if let Some(path) = &build_opts.path {
            if path.to_string_lossy().starts_with("--") {
                let path = build_opts.path.take().unwrap();
                build_opts
                    .extra_options
                    .insert(0, path.to_string_lossy().into_owned());
            }
        }
        let crate_path = get_crate_path(build_opts.path)?;
        let crate_data = manifest::CrateData::new(&crate_path, build_opts.out_name.clone())?;
        let out_dir = crate_path.join(PathBuf::from(build_opts.out_dir)).clean();

        let dev = build_opts.dev || build_opts.debug;
        let profile = match (
            dev,
            build_opts.release,
            build_opts.profiling,
            build_opts.profile,
        ) {
            (false, false, false, None) | (false, true, false, None) => BuildProfile::Release,
            (true, false, false, None) => BuildProfile::Dev,
            (false, false, true, None) => BuildProfile::Profiling,
            (false, false, false, Some(profile)) => BuildProfile::Custom(profile),
            // Unfortunately, `clap` doesn't expose clap's `conflicts_with`
            // functionality yet, so we have to implement it ourselves.
            _ => bail!("Can only supply one of the --dev, --release, --profiling, or --profile 'name' flags"),
        };

        Ok(Build {
            crate_path,
            crate_data,
            scope: build_opts.scope,
            disable_dts: build_opts.disable_dts,
            target: build_opts.target,
            no_opt: build_opts.no_opt,
            profile,
            out_dir,
            out_name: build_opts.out_name,
            bindgen: None,
            bindgen_override: false,
            extra_args: Vec::new(),
            extra_options: build_opts.extra_options,
        })
    }

    /// Prepare this `Build` command.
    pub fn init(&mut self) -> Result<()> {
        let process_steps = Build::get_preprocess_steps();
        for (_, process_step) in process_steps {
            process_step(self)?;
        }
        Ok(())
    }

    /// Execute this `Build` command.
    pub fn run(&mut self) -> Result<()> {
        let process_steps = Build::get_process_steps(self.no_opt);

        let started = Instant::now();

        for (_, process_step) in process_steps {
            process_step(self)?;
        }

        let duration = utils::elapsed(started.elapsed());
        info!("Done in {}.", &duration);
        info!(
            "Your wasm pkg is ready to publish at {}.",
            self.out_dir.display()
        );

        PBAR.info(&format!("{}Done in {}", emoji::SPARKLE, &duration));

        PBAR.info(&format!(
            "{} Your wasm pkg is ready to publish at {}.",
            emoji::PACKAGE,
            self.out_dir.display()
        ));
        Ok(())
    }

    #[allow(clippy::vec_init_then_push)]
    fn get_preprocess_steps() -> Vec<(&'static str, BuildStep)> {
        macro_rules! steps {
            ($($name:ident),+) => {
                {
                let mut steps: Vec<(&'static str, BuildStep)> = Vec::new();
                    $(steps.push((stringify!($name), Build::$name));)*
                        steps
                    }
                };
            ($($name:ident,)*) => (steps![$($name),*])
        }
        steps![
            step_check_rustc_version,
            step_check_crate_config,
            step_check_for_wasm_target,
            step_check_lib_versions,
            step_install_wasm_bindgen,
        ]
    }

    #[allow(clippy::vec_init_then_push)]
    fn get_process_steps(no_opt: bool) -> Vec<(&'static str, BuildStep)> {
        macro_rules! steps {
            ($($name:ident),+) => {
                {
                let mut steps: Vec<(&'static str, BuildStep)> = Vec::new();
                    $(steps.push((stringify!($name), Build::$name));)*
                        steps
                    }
                };
            ($($name:ident,)*) => (steps![$($name),*])
        }
        let mut steps = Vec::new();
        steps.extend(steps![
            step_build_wasm,
            step_create_dir,
            step_run_wasm_bindgen,
        ]);

        if !no_opt {
            steps.extend(steps![step_run_wasm_opt]);
        }

        steps.extend(steps![step_create_json,]);
        steps
    }

    fn step_check_rustc_version(&mut self) -> Result<()> {
        info!("Checking rustc version...");
        let version = target::check_rustc_version()?;
        let msg = format!("rustc version is {}.", version);
        info!("{}", &msg);
        Ok(())
    }

    fn step_check_crate_config(&mut self) -> Result<()> {
        info!("Checking crate configuration...");
        self.crate_data.check_crate_config()?;
        info!("Crate is correctly configured.");
        Ok(())
    }

    fn step_check_for_wasm_target(&mut self) -> Result<()> {
        info!("Checking for wasm-target...");
        target::check_for_wasm32_target()?;
        info!("Checking for wasm-target was successful.");
        Ok(())
    }

    fn step_build_wasm(&mut self) -> Result<()> {
        info!("Building wasm...");
        target::cargo_build_wasm(&self.crate_path, self.profile.clone(), &self.extra_options)?;

        info!(
            "wasm built at {:#?}.",
            &self
                .crate_path
                .join("target")
                .join("wasm32-unknown-unknown")
                .join("release")
        );
        Ok(())
    }

    fn step_create_dir(&mut self) -> Result<()> {
        info!("Creating a pkg directory...");
        create_pkg_dir(&self.out_dir)?;
        info!("Created a pkg directory at {:#?}.", &self.crate_path);
        Ok(())
    }

    fn step_create_json(&mut self) -> Result<()> {
        self.crate_data
            .write_package_json(&self.out_dir, &self.scope, self.disable_dts)?;
        info!(
            "Wrote a package.json at {:#?}.",
            &self.out_dir.join("package.json")
        );
        Ok(())
    }

    fn step_check_lib_versions(&mut self) -> Result<()> {
        let lockfile = Lockfile::new(&self.crate_data.data)?;

        lockfile
            .require_lib(
                "wasm-bindgen",
                &MIN_WASM_BINDGEN_LIB_VERSION,
                &CUR_WASM_BINDGEN_VERSION,
            )
            .map_err(|err| match err {
                DepCheckError::VersionError(msg, _) => anyhow!(msg),
                DepCheckError::Error(err) => anyhow!(err),
            })?;
        Ok(())
    }

    fn step_install_wasm_bindgen(&mut self) -> Result<()> {
        info!("Installing wasm-bindgen-cli...");
        use wasm_bindgen_cli_support::build::binary::{GetBinary, WasmBindgen};
        let (bindgen, bindgen_override) = WasmBindgen.get_binary(None)?;
        self.bindgen = Some(bindgen);
        self.bindgen_override = bindgen_override;
        info!("Installing wasm-bindgen-cli was successful.");
        Ok(())
    }

    fn step_run_wasm_bindgen(&mut self) -> Result<()> {
        info!("Building the wasm bindings...");
        wasm_bindgen_build(
            &self.crate_data,
            self.bindgen.as_ref().unwrap(),
            &self.out_dir,
            &self.out_name,
            self.disable_dts,
            self.target,
            self.profile.clone(),
            &self.extra_args,
            &self.extra_options,
        )?;
        info!("wasm bindings were built at {:#?}.", &self.out_dir);
        Ok(())
    }

    fn step_run_wasm_opt(&mut self) -> Result<()> {
        let mut args = match self
            .crate_data
            .configured_profile(self.profile.clone())
            .wasm_opt_args()
        {
            Some(args) => args,
            None => return Ok(()),
        };
        args.push("--all-features".into());
        // Keep the Wasm names section
        args.push("--debuginfo".into());
        info!("executing wasm-opt with {:?}", args);
        wasm_opt_run(&self.out_dir, &args).map_err(|e| {
            anyhow!(
                "{}\nTo disable `wasm-opt`, add `wasm-opt = false` to your package metadata in your `Cargo.toml`.", e
            )
        })
    }
}

/// Run the `wasm-bindgen` CLI to generate bindings for the current crate's
/// `.wasm`.
#[allow(clippy::too_many_arguments)]
pub fn wasm_bindgen_build(
    data: &CrateData,
    bindgen_path: &Path,
    out_dir: &Path,
    out_name: &Option<String>,
    disable_dts: bool,
    target: Target,
    profile: BuildProfile,
    extra_args: &[String],
    extra_options: &[String],
) -> Result<()> {
    let profile_name = match profile.clone() {
        BuildProfile::Release | BuildProfile::Profiling => "release",
        BuildProfile::Dev => "debug",
        BuildProfile::Custom(profile_name) => &profile_name.clone(),
    };

    let out_dir = out_dir.to_str().unwrap();

    let target_directory = {
        let mut has_target_dir_iter = extra_options.iter();
        has_target_dir_iter
            .find(|&it| it == "--target-dir")
            .and_then(|_| has_target_dir_iter.next())
            .map(Path::new)
            .unwrap_or(data.target_directory())
    };

    let wasm_path = target_directory
        .join("wasm32-unknown-unknown")
        .join(profile_name)
        .join(data.crate_name())
        .with_extension("wasm");

    let dts_arg = if disable_dts {
        "--no-typescript"
    } else {
        "--typescript"
    };

    let mut cmd = Command::new(bindgen_path);
    cmd.arg(&wasm_path)
        .arg("--out-dir")
        .arg(out_dir)
        .arg(dts_arg);

    cmd.arg("--target").arg(target.to_string());

    if let Some(value) = out_name {
        cmd.arg("--out-name").arg(value);
    }

    let profile = data.configured_profile(profile);
    if profile.wasm_bindgen_debug_js_glue() {
        cmd.arg("--debug");
    }
    if !profile.wasm_bindgen_demangle_name_section() {
        cmd.arg("--no-demangle");
    }
    if profile.wasm_bindgen_dwarf_debug_info() {
        cmd.arg("--keep-debug");
    }
    if profile.wasm_bindgen_omit_default_module_path() {
        cmd.arg("--omit-default-module-path");
    }
    if profile.wasm_bindgen_split_linked_modules() {
        cmd.arg("--split-linked-modules");
    }

    for arg in extra_args {
        cmd.arg(arg);
    }

    utils::run(cmd, "wasm-bindgen").context("Running the wasm-bindgen CLI")?;
    Ok(())
}

/// Execute `wasm-opt` over wasm binaries found in `out_dir`, downloading if
/// necessary into `cache`. Passes `args` to each invocation of `wasm-opt`.
pub fn wasm_opt_run(out_dir: &Path, args: &[String]) -> Result<()> {
    let wasm_opt_path = WasmOpt.get_binary(None)?.0;

    PBAR.info("Optimizing wasm binaries with `wasm-opt`...");

    for file in out_dir.read_dir()? {
        let file = file?;
        let path = file.path();
        if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }

        let tmp = path.with_extension("wasm-opt.wasm");
        let mut cmd = Command::new(&wasm_opt_path);
        cmd.arg(&path).arg("-o").arg(&tmp).args(args);
        utils::run(cmd, "wasm-opt")?;
        std::fs::rename(&tmp, &path)?;
    }

    Ok(())
}

#[derive(Parser)]
struct BuildArgs {
    #[clap(flatten)]
    pub build_options: BuildOptions,
}

fn parse_wasm_pack_opts<I>(args: I) -> Result<BuildOptions>
where
    I: IntoIterator<Item = String>,
{
    // This is done instead of explicitly constructing
    // BuildOptions to preserve the behavior of appending
    // arbitrary arguments in `args`.
    let mut build_args = vec![
        "--no-typescript".to_owned(),
        "--target".to_owned(),
        "module".to_owned(),
        "--out-dir".to_owned(),
        "build".to_owned(),
        "--out-name".to_owned(),
        "index".to_owned(),
    ];

    build_args.extend(args);

    let command = BuildArgs::try_parse_from(build_args)?;
    Ok(command.build_options)
}

pub fn run_cli_with_args(args: std::env::Args) -> Result<()> {
    let args: Vec<_> = args.collect();
    if args.len() > 1 && (args[1].as_str() == "--version" || args[1].as_str() == "-v") {
        // println!("{}", VERSION);
        return Ok(());
    }
    let wasm_pack_opts = parse_wasm_pack_opts(args.into_iter().skip(1))?;
    // let out_path = output_path("");
    // if out_path.exists() {
    //     fs::remove_dir_all(out_path)?;
    // }
    let mut builder = Build::try_from_opts(wasm_pack_opts)?;
    builder.init()?;
    builder.run()?;
    Ok(())
}
