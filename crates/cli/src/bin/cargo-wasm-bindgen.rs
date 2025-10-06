use cargo_metadata::{Message, TargetKind};
use clap::Parser;
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

#[derive(clap::ValueEnum, Clone, Debug)]
enum Cmd {
    Build,
    Test,
}

#[derive(Parser, Debug)]
#[command(
    name = "cargo wasm-bindgen",
    version,
    about = "cargo subcommand for the wasm-bindgen CLI"
)]
struct Cli {
    command: Cmd,

    /// Arguments passed to wasm-bindgen.
    #[arg(allow_hyphen_values = true)]
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

    let mut cli =
        Cli::parse_from(std::iter::once(OsString::from("cargo wasm-bindgen")).chain(args));

    // clap reaaaallly doesn't like this scheme with two variadic lists of arbitrary args
    // we need to patch up the parsed struct manually it seems
    if let Some(split_pos) = cli.wasm_bindgen_args.iter().position(|s| s == "--") {
        cli.cargo_args
            .extend(cli.wasm_bindgen_args.drain(split_pos..).skip(1));
    }

    let (command_str, command_exe, artifact_kind) = match &cli.command {
        Cmd::Build => ("build", "wasm-bindgen", TargetKind::CDyLib),
        Cmd::Test => ("test", "wasm-bindgen-test-runner", TargetKind::Test),
    };

    let mut cargo_cmd = Command::new("cargo");
    // Use environment variable so that we don't override user-provided `--target` if any.
    cargo_cmd.env("CARGO_BUILD_TARGET", "wasm32-unknown-unknown");
    cargo_cmd.arg(command_str);
    cargo_cmd.args(cli.cargo_args);

    // Get relevant wasm-bindgen binary from the same directory as the current one.
    let command_exe = std::env::current_exe()?.with_file_name(command_exe);

    match cli.command {
        // For the build command we need to communicate with Cargo over its JSON format.
        Cmd::Build => {
            let mut cargo_output = cargo_cmd
                .arg("--message-format=json-render-diagnostics")
                .stdout(Stdio::piped())
                .spawn()?;

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
                // for each of those, run wasm-bindgen with provided parameters
                .try_for_each(|(manifest_dir, wasm)| {
                    Command::new(&command_exe)
                        .current_dir(manifest_dir)
                        .args(&cli.wasm_bindgen_args)
                        .arg(wasm)
                        .status()
                        .and_then(check_status)
                })?;

            cargo_output.wait()
        }

        // For the test command we can just set up the Cargo runner and delegate the rest to Cargo itself.
        Cmd::Test => cargo_cmd
            .env("CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER", command_exe)
            .arg("--")
            .args(cli.wasm_bindgen_args)
            .status(),
    }
    .and_then(check_status)
}
