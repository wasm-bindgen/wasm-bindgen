use std::env;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    wasm_bindgen_toolchain::wasm_bindgen_build::run_cli_with_args(env::args_os())
}
