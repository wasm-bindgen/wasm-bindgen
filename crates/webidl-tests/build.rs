use std::env;
use std::path::PathBuf;

fn main() {
    let mut out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Check if compat mode is enabled
    let compat_mode = env::var("CARGO_FEATURE_IDL_GENERICS_COMPAT").is_ok();

    // Use different output directories for compat and non-compat builds
    if compat_mode {
        out_dir.push("compat");
    } else {
        out_dir.push("non-compat");
    }

    wasm_bindgen_webidl::generate(
        "webidls".as_ref(),
        &out_dir,
        wasm_bindgen_webidl::Options {
            features: false,
            generics_compat: compat_mode,
        },
    )
    .unwrap();
}
