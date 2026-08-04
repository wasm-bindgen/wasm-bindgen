// `&T` can only be handed to JS by copying `T` across the boundary, which is
// sound for a scalar but not for a type with an identity or an owner. The
// `ScalarIntoWasmAbi` marker keeps the set of types with an `impl IntoWasmAbi for
// &T` in lockstep with the set the CLI can actually bind, so each of these fails
// at compile time with a trait error naming the user's own type, rather than
// aborting `wasm-bindgen` after the build has already succeeded.
//
// Note the spans below: each rejected argument produces two errors, one located
// at the imported function and one at the argument's type. Both are on the
// offending declaration rather than on the block's `#[wasm_bindgen]`, which is
// where an unimplemented-ABI error would otherwise land -- the generated shim
// signature and the generated body each carry the obligation independently, and
// `respan_all`/`abi_span` in `crates/macro-support/src/codegen.rs` place them.
// The pair is redundant but points at the right place, which matters because
// this check makes that path considerably easier to reach.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct CopyCell {
    pub v: f64,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Blue,
}

#[wasm_bindgen]
extern "C" {
    // An exported `#[wasm_bindgen]` struct: passing `&CopyCell` would silently
    // hand JS a distinct copy with its own `free()` obligation.
    fn take_struct_ref(x: &CopyCell);

    // An exported `#[wasm_bindgen]` enum is `Copy` but is not a scalar ABI.
    fn take_enum_ref(x: &Color);

    // `Option<u32>` is `Copy` but has no by-reference wire form.
    fn take_option_ref(x: &Option<u32>);

    // Neither does a reference to a reference.
    fn take_ref_ref(x: &&u32);
}

fn main() {}
