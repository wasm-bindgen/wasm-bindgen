use cargo_metadata::{Message, TargetKind};
use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::io::BufReader;
use std::io::{self};
use std::process::ExitStatus;
use std::process::{Command, Stdio};

fn check_status(status: ExitStatus) -> io::Result<()> {
    match status.code() {
        Some(0) => Ok(()),
        Some(code) => std::process::exit(code),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "process terminated by signal",
        )),
    }
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Build,
    Test,
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,

    /// Arguments passed to wasm-bindgen.
    wasm_bindgen_args: Vec<OsString>,

    /// Arguments passed to cargo.
    #[arg(last = true)]
    cargo_args: Vec<OsString>,
}

fn main() -> io::Result<()> {
    let mut args = std::env::args_os().skip(1).peekable();

    // We can be invoked as either `cargo wasm-bindgen` or `cargo-wasm-bindgen`.
    // If the former, skip the `wasm-bindgen` argument to normalize parsing.
    args.next_if_eq("wasm-bindgen");

    let cli = Cli::parse_from(std::iter::once(OsString::from("cargo wasm-bindgen")).chain(args));

    let (command_str, command_exe, artifact_kind) = match &cli.command {
        Cmd::Build => ("build", "wasm-bindgen", TargetKind::CDyLib),
        Cmd::Test => ("test", "wasm-bindgen-test-runner", TargetKind::Test),
    };

    let mut cargo_output = Command::new("cargo")
        .arg(command_str)
        .args(cli.cargo_args)
        .arg("--target=wasm32-unknown-unknown")
        .arg("--message-format=json-render-diagnostics")
        .args(matches!(cli.command, Cmd::Test).then_some("--no-run"))
        .stdout(Stdio::piped())
        .spawn()?;

    // Get relevant wasm-bindgen binary from the same directory as the current one.
    let wasm_bindgen = std::env::current_exe()?.with_file_name(command_exe);

    Message::parse_stream(BufReader::new(cargo_output.stdout.take().unwrap()))
        // we're only interested in compiler artifacts
        .filter_map(|message| match message {
            Ok(Message::CompilerArtifact(artifact))
                if artifact.target.kind.contains(&artifact_kind) =>
            {
                Some(artifact)
            }
            _ => None,
        })
        // collect filenames from compiler artifacts
        .flat_map(|artifact| {
            artifact
                .filenames
                .into_iter()
                // we're only interested in wasm files
                .filter(|path| path.extension() == Some("wasm"))
                .map(move |path| {
                    (
                        // we need to save the manifest dir or wasm-bindgen won't be able to resolve local paths
                        artifact
                            .manifest_path
                            .parent()
                            .map(ToOwned::to_owned)
                            .unwrap_or_default(),
                        path,
                    )
                })
        })
        .inspect(|x| eprintln!("Processing: {:?}", x))
        // for each of those, run wasm-bindgen with provided parameters
        .try_for_each(|(manifest_dir, wasm)| {
            Command::new(&wasm_bindgen)
                // .current_dir(manifest_dir)
                .args(&cli.wasm_bindgen_args)
                .arg(wasm)
                .status()
                .and_then(check_status)
        })?;

    cargo_output.wait().and_then(check_status)
}
