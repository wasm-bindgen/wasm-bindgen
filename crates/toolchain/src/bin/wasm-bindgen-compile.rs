use std::{env, process};

fn main() {
    env_logger::init();
    let err = match wasm_bindgen_cli::wasm_bindgen::run_cli_with_args(env::args_os()) {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {err:?}");
    process::exit(1);
}
