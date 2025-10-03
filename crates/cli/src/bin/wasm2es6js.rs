use std::env;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    wasm_bindgen_cli::wasm2es6js::run_cli_with_args(env::args_os())
}
