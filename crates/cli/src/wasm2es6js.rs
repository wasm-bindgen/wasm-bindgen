use anyhow::{bail, Context, Error};
use clap::Parser;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "wasm2es6js",
    version,
    about,
    long_about = None,
    after_help = "Note that this is not intended to produce a production-ready output module but rather\n\
                  is intended purely as a temporary \"hack\" until it's standard in\n\
                  bundlers for working with wasm. Use this program with care!",
)]
struct Args {
    #[arg(long, short, value_name = "FILE", help = "File to place output in")]
    output: Option<PathBuf>,
    #[arg(long, value_name = "DIR", help = "Directory to place output in")]
    out_dir: Option<PathBuf>,
    #[arg(long, help = "Output a `*.d.ts` file next to the JS output")]
    typescript: bool,
    #[arg(long, help = "Inline the Wasm module using base64 encoding")]
    base64: bool,
    #[arg(
        long,
        value_name = "PATH",
        help = "Load module by passing the PATH argument to `fetch()`"
    )]
    fetch: Option<String>,
    input: PathBuf,
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
    rmain(args)
}

fn rmain(args: Args) -> anyhow::Result<()> {
    let wasm = fs::read(&args.input)
        .with_context(|| format!("failed to read `{}`", args.input.display()))?;

    let object = wasm_bindgen_cli_support::wasm2es6js::Config::new()
        .base64(args.base64)
        .fetch(args.fetch.clone())
        .generate(&wasm)?;

    if args.typescript {
        let ts = object.typescript()?;
        write(&args, "d.ts", ts.as_bytes(), false)?;
    }

    let (js, wasm) = object.js_and_wasm()?;

    write(&args, "js", js.as_bytes(), false)?;
    if let Some(wasm) = wasm {
        write(&args, "wasm", &wasm, false)?;
    }
    Ok(())
}

fn write(args: &Args, extension: &str, contents: &[u8], print_fallback: bool) -> Result<(), Error> {
    if let Some(p) = &args.output {
        let dst = p.with_extension(extension);
        fs::write(&dst, contents)
            .with_context(|| format!("failed to write `{}`", dst.display()))?;
    } else if let Some(p) = &args.out_dir {
        let filename = args.input.file_name().unwrap();
        let dst = p.join(filename).with_extension(extension);
        fs::write(&dst, contents)
            .with_context(|| format!("failed to write `{}`", dst.display()))?;
    } else if print_fallback {
        println!("{}", String::from_utf8_lossy(contents))
    }

    Ok(())
}
