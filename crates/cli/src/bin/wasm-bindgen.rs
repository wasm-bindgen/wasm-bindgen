use anyhow::{bail, Error};
use clap::Parser;
use std::path::PathBuf;
use std::process;
use wasm_bindgen_cli_support::{Bindgen, EncodeInto};

#[derive(Debug, Parser)]
#[command(
    name = "wasm-bindgen",
    version,
    about,
    long_about = None,
    after_help = "Additional documentation: https://rustwasm.github.io/wasm-bindgen/reference/cli.html",
)]
struct Args {
    #[arg(long, help = "Deprecated, use `--target nodejs`")]
    nodejs: bool,
    #[arg(long, help = "Hint that JS should only be compatible with a browser")]
    browser: bool,
    #[arg(long, help = "Deprecated, use `--target web`")]
    web: bool,
    #[arg(long, help = "Deprecated, use `--target no-modules`")]
    no_modules: bool,
    #[arg(long, help = "Output a TypeScript definition file (on by default)")]
    typescript: bool,
    #[arg(long, help = "Don't emit a *.d.ts file")]
    no_typescript: bool,
    #[arg(long, help = "Don't emit imports in generated JavaScript")]
    omit_imports: bool,
    #[arg(long, value_name = "DIR", help = "Output directory")]
    out_dir: Option<PathBuf>,
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
    #[arg(long, help = "Deprecated, is runtime-detected")]
    #[allow(dead_code)]
    weak_refs: bool,
    #[arg(long, help = "Deprecated, use `-Ctarget-feature=+reference-types`")]
    reference_types: bool,
    #[arg(long, help = "Keep exports synthesized by LLD")]
    keep_lld_exports: bool,
    #[arg(long, help = "Keep debug sections in Wasm files")]
    keep_debug: bool,
    #[arg(
        long,
        value_name = "MODE",
        help = "Whether or not to use TextEncoder#encodeInto, valid values are [test, always, never]"
    )]
    encode_into: Option<String>,
    #[arg(
        long,
        value_name = "TARGET",
        help = "What type of output to generate, valid\n\
                values are [web, bundler, nodejs, no-modules, deno, experimental-nodejs-module],\n\
                and the default is [bundler]"
    )]
    target: Option<String>,
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
    input: PathBuf,
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
    let typescript = args.typescript || !args.no_typescript;

    let mut b = Bindgen::new();
    if let Some(name) = &args.target {
        match name.as_str() {
            "bundler" => b.bundler(true)?,
            "web" => b.web(true)?,
            "no-modules" => b.no_modules(true)?,
            "nodejs" => b.nodejs(true)?,
            "deno" => b.deno(true)?,
            "experimental-nodejs-module" => b.nodejs_module(true)?,
            s => bail!("invalid encode-into mode: `{}`", s),
        };
    }
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
        .typescript(typescript)
        .omit_imports(args.omit_imports)
        .omit_default_module_path(args.omit_default_module_path)
        .split_linked_modules(args.split_linked_modules);
    if args.reference_types {
        #[allow(deprecated)]
        b.reference_types(true);
    }
    if let Some(ref name) = args.no_modules_global {
        b.no_modules_global(name)?;
    }
    if let Some(ref name) = args.out_name {
        b.out_name(name);
    }
    if let Some(mode) = &args.encode_into {
        match mode.as_str() {
            "test" => b.encode_into(EncodeInto::Test),
            "always" => b.encode_into(EncodeInto::Always),
            "never" => b.encode_into(EncodeInto::Never),
            s => bail!("invalid encode-into mode: `{}`", s),
        };
    }

    let out_dir = match args.out_dir {
        Some(ref p) => p,
        None => bail!("the `--out-dir` argument is now required"),
    };

    b.generate(out_dir)
}
