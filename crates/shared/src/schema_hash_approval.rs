// Whenever any of the schema files change, the SCHEMA_FILE_HASH environment variable will
// change and the schema_version test below will fail. The covered set is listed in
// `SCHEMA_FILES` in this crate's `build.rs`: currently `src/lib.rs` (the encoded AST) and
// `src/tys.rs` (the descriptor opcode numbers). Note that this covers those *definitions*,
// not the order in which the root crate's `describe()` impls emit words -- see the note in
// `build.rs` for why that is out of reach.
//
// If the change was incidental -- a comment, a rustfmt reflow, a new helper that is not part
// of the encoded form -- then the wire contract is unaffected and it is enough to update
// `APPROVED_SCHEMA_FILE_HASH` to the new hash.
//
// If the encoded schema really did change, then additionally set `SCHEMA_VERSION` in this
// library to the version of the *next, unreleased* wasm-bindgen release.
//
// `SCHEMA_VERSION` is a schema identity, not a release version, so it is deliberately *not*
// kept in step with `crates/shared/Cargo.toml`: it is bumped only when the schema changes,
// and releases that leave the schema alone leave it untouched. That is why it is normally
// *behind* the current Cargo.toml version, and being behind is not a bug.
//
// What it must never do is name a version whose released artifacts used a different schema.
// `./publish bump` only rewrites Cargo.toml versions at release time, so on a development
// branch Cargo.toml still names the last *released* version; setting `SCHEMA_VERSION` to it
// would claim an identity that a released artifact already used under the old schema, and the
// CLI's exact-string check (`verify_schema_matches` in `wasm-bindgen-cli-support`) would then
// wave through a genuinely incompatible macro/CLI pair. Hence "next unreleased version", and
// hence only one bump per release cycle however many schema changes land in it.
const APPROVED_SCHEMA_FILE_HASH: &str = "17878099116568217446";

#[test]
fn schema_version() {
    assert_eq!(env!("SCHEMA_FILE_HASH"), APPROVED_SCHEMA_FILE_HASH)
}
