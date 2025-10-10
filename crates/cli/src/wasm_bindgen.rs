use anyhow::{bail, Error};
use clap::{Parser, ValueEnum};
use std::ffi::OsString;
use std::path::PathBuf;
use wasm_bindgen_cli_support::{Bindgen, EncodeInto};

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum Target {
    Bundler,
    Web,
    Nodejs,
    NoModules,
    Deno,
    ExperimentalNodejsModule,
    Module,
}

#[derive(Debug, Parser)]
#[command(
    name = "wasm-bindgen",
    version,
    about,
    after_help = "Additional documentation: https://wasm-bindgen.github.io/wasm-bindgen/reference/cli.html"
)]
struct Args {
    input: PathBuf,
    #[arg(
        long,
        value_name = "TARGET",
        value_enum,
        default_value_t = Target::Bundler,
        help = "What type of output to generate",
        group = "target-group"
    )]
    target: Target,
    #[arg(long, value_name = "DIR", help = "Output directory")]
    out_dir: PathBuf,
    #[arg(
        long,
        help = "Hint that JS should only be compatible with a browser",
        group = "target-group"
    )]
    browser: bool,
    #[arg(long, help = "Don't emit a *.d.ts file", conflicts_with = "typescript")]
    no_typescript: bool,
    #[arg(long, help = "Don't emit imports in generated JavaScript")]
    omit_imports: bool,
    #[arg(
        long,
        value_name = "VAR",
        help = "Set a custom output filename (Without extension. Defaults to crate name)"
    )]
    out_name: Option<String>,
    #[arg(long, help = "Include otherwise-extraneous debug checks in output")]
    debug: bool,
    #[arg(long, help = "Don't demangle Rust symbol names")]
    no_demangle: bool,
    #[arg(
        long,
        value_name = "VAR",
        help = "Name of the global variable to initialize"
    )]
    no_modules_global: Option<String>,
    #[arg(long, help = "Remove the debugging `name` section of the file")]
    remove_name_section: bool,
    #[arg(long, help = "Remove the telemetry `producers` section")]
    remove_producers_section: bool,
    #[arg(long, help = "Keep exports synthesized by LLD")]
    keep_lld_exports: bool,
    #[arg(long, help = "Keep debug sections in Wasm files")]
    keep_debug: bool,
    #[arg(
        long,
        value_name = "MODE",
        help = "Whether or not to use TextEncoder#encodeInto",
        value_parser = ["test", "always", "never"]
    )]
    encode_into: Option<String>,
    #[arg(
        long,
        help = "Don't add WebAssembly fallback imports in generated JavaScript"
    )]
    omit_default_module_path: bool,
    #[arg(
        long,
        help = "Split linked modules out into their own files. Recommended if possible.\n\
                If a bundler is used, it needs to be set up accordingly."
    )]
    split_linked_modules: bool,
    #[arg(
        long = "experimental-reset-state-function",
        help = "Generate __wbg_reset_state function for WASM reinitialization (experimental)"
    )]
    generate_reset_state: bool,
    // The options below are deprecated. They're still parsed for backwards compatibility,
    // but we don't want to show them in `--help` to avoid distracting users.
    #[arg(long, hide = true)]
    #[deprecated(note = "implied default, only `--no-typescript` is needed")]
    typescript: bool,
    #[arg(long, hide = true, group = "target-group")]
    #[deprecated(note = "use `Args::target` instead")]
    nodejs: bool,
    #[arg(long, hide = true, group = "target-group")]
    #[deprecated(note = "use `Args::target` instead")]
    web: bool,
    #[arg(long, hide = true, group = "target-group")]
    #[deprecated(note = "use `Args::target` instead")]
    no_modules: bool,
    #[arg(long, hide = true)]
    #[deprecated(note = "runtime-detected")]
    #[allow(dead_code)]
    weak_refs: bool,
    #[arg(long, hide = true)]
    #[deprecated(note = "automatically inferred from the Wasm features")]
    reference_types: bool,
}

pub fn run_cli_with_args<I, T>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = match Args::try_parse_from(args) {
        Ok(a) => a,
        Err(e) => match e.kind() {
            // Passing --version and --help should not result in a failure.
            clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                print!("{e}");
                return Ok(());
            }
            _ => bail!(e),
        },
    };
    rmain(&args)
}

fn rmain(args: &Args) -> Result<(), Error> {
    let mut b = Bindgen::new();
    match &args.target {
        Target::Bundler => b.bundler(true)?,
        Target::Web => b.web(true)?,
        Target::NoModules => b.no_modules(true)?,
        Target::Nodejs => b.nodejs(true)?,
        Target::Deno => b.deno(true)?,
        Target::ExperimentalNodejsModule => b.nodejs_module(true)?,
        Target::Module => b.module(true)?,
    };
    #[allow(deprecated)]
    b.input_path(&args.input)
        .nodejs(args.nodejs)?
        .web(args.web)?
        .browser(args.browser)?
        .no_modules(args.no_modules)?
        .debug(args.debug)
        .demangle(!args.no_demangle)
        .keep_lld_exports(args.keep_lld_exports)
        .keep_debug(args.keep_debug)
        .remove_name_section(args.remove_name_section)
        .remove_producers_section(args.remove_producers_section)
        .typescript(!args.no_typescript)
        .omit_imports(args.omit_imports)
        .omit_default_module_path(args.omit_default_module_path)
        .split_linked_modules(args.split_linked_modules)
        .reference_types(args.reference_types)
        .reset_state_function(args.generate_reset_state);

    if let Some(ref name) = args.no_modules_global {
        b.no_modules_global(name)?;
    }
    if let Some(ref name) = args.out_name {
        b.out_name(name);
    }

    if let Some(mode) = &args.encode_into {
        let mode = match mode.as_str() {
            "test" => EncodeInto::Test,
            "always" => EncodeInto::Always,
            "never" => EncodeInto::Never,
            // clap guarantees
            _ => unreachable!(),
        };
        b.encode_into(mode);
    }

    b.generate(&args.out_dir)
}
