// Mostly no-op `build.rs` so that `[package] links = ...` works in `Cargo.toml`.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(feature = "xxx_debug_only_print_generated_code")]
    {
        println!("cargo:warning=The `xxx_debug_only_print_generated_code` internal feature is deprecated and will be removed in the next major version.");
    }
}
