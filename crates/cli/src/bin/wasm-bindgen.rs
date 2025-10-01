use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let err = match wasm_bindgen_cli::wasm_bindgen::run_cli_with_args(&args_ref) {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {err:?}");
    process::exit(1);
}
