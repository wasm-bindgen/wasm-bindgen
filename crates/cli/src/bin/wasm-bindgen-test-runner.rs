use std::{env, process};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    wasm_bindgen_cli::wasm_bindgen_test_runner::run_cli_with_args(env::args())?;
    process::exit(0);
}
