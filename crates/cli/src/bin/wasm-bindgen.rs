use anyhow::Error;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process;
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
    encode_into: Option<EncodeInto>,
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

fn main() {
    env_logger::init();
    let args = Args::parse();

    let err = match rmain(&args) {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {err:?}");
    process::exit(1);
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
        Target::Module => b.source_phase(true)?,
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
        .reference_types(args.reference_types);

    if let Some(ref name) = args.no_modules_global {
        b.no_modules_global(name)?;
    }
    if let Some(ref name) = args.out_name {
        b.out_name(name);
    }
    if let Some(mode) = args.encode_into {
        b.encode_into(mode);
    }

    b.generate(&args.out_dir)
}
