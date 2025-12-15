use std::{env, process};

fn main() {
    env_logger::init();
    let err = match wasm_bindgen_cli::wasm_bindgen_build::run_cli_with_args(env::args()) {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {err:?}");
    process::exit(1);
}
