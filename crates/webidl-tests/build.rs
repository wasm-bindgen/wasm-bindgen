use std::env;
use std::path::PathBuf;

fn main() {
    env_logger::init();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Read next_unstable from environment variable
    // Set WBG_NEXT_UNSTABLE=1 to generate generics mode output
    let next_unstable = env::var("WBG_NEXT_UNSTABLE").is_ok_and(|v| v == "1");

    // Re-run build script if this env var changes
    println!("cargo:rerun-if-env-changed=WBG_NEXT_UNSTABLE");

    wasm_bindgen_webidl::generate(
        "webidls".as_ref(),
        &out_dir,
        wasm_bindgen_webidl::Options {
            features: false,
            next_unstable: std::cell::Cell::new(next_unstable),
        },
    )
    .unwrap();
}
