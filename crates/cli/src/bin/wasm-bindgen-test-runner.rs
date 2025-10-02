use std::{env, process};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    wasm_bindgen_cli::wasm_bindgen_test_runner::run_cli_with_args(env::args_os())?;
    // Returning cleanly has the strange effect of outputting
    // an additional empty line with spaces in it.
    process::exit(0);
}
