use std::{env, process};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    wasm_bindgen_cli::wasm_bindgen_test_runner::run_cli_with_args(&args_ref)?;
    process::exit(0);
}
