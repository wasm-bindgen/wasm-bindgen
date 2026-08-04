use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::Hasher;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    set_schema_version_env_var();

    let rev = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()
        .map(|s| s.stdout)
        .and_then(|s| String::from_utf8(s).ok());
    if let Some(rev) = rev {
        if rev.len() >= 9 {
            println!("cargo:rustc-env=WBG_VERSION={}", &rev[..9]);
        }
    }
}

/// Files whose contents define the macro/CLI wire contract, hashed together
/// into `SCHEMA_FILE_HASH` so that changing any of them trips the
/// `schema_version` test and forces a deliberate `SCHEMA_VERSION` bump.
///
/// * `src/lib.rs` -- the `Program`/`Import`/`Export` structs the macro encodes
///   into the `__wasm_bindgen_unstable` custom section.
/// * `src/tys.rs` -- the descriptor opcode numbers. These are consumed by value
///   in `wasm-bindgen-cli-support`'s decoder, so renumbering or reordering them
///   silently repurposes every existing tag: an old `.wasm` decoded by a new
///   CLI yields wrong bindings with no error anywhere. That is exactly the
///   failure this mechanism exists to prevent, and it used to slip through
///   because only `lib.rs` was covered.
///
/// Deliberately *not* covered: the `describe()` impls in the root crate's
/// `src/describe.rs` and `src/rt/mod.rs`, which decide the order words are
/// emitted in. Reaching them means escaping this crate's directory, which
/// breaks packaged and vendored builds. Hashing `tys.rs` closes the
/// realistic hole (an opcode renumber); an emission-order change still relies
/// on the author bumping `SCHEMA_VERSION` by hand.
const SCHEMA_FILES: &[&str] = &["src/lib.rs", "src/tys.rs"];

fn set_schema_version_env_var() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").expect(
        "The `CARGO_MANIFEST_DIR` environment variable is needed to locate the schema file",
    );
    let cargo_manifest_dir = PathBuf::from(cargo_manifest_dir);

    let mut hasher = DefaultHasher::new();
    for relative in SCHEMA_FILES {
        let path = cargo_manifest_dir.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());

        let schema_file = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read schema file {}: {e}", path.display()));
        #[cfg(windows)]
        let schema_file = schema_file.replace("\r\n", "\n");

        hasher.write(schema_file.as_bytes());
    }

    println!("cargo:rustc-env=SCHEMA_FILE_HASH={}", hasher.finish());
}
