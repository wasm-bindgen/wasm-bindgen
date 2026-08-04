// Reference test for `&T` arguments where `T` is a scalar.
//
// The blanket `impl<T: ScalarIntoWasmAbi> IntoWasmAbi for &T` lets a shared
// reference to a scalar be passed to JS by copying the value, so the wire form is
// identical to passing `T` by value.
//
// Every type in the `scalar_into_wasm_abi!` list in `src/convert/impls.rs`
// appears below, on purpose. That list and `is_scalar_by_shared_ref` in
// `crates/cli-support/src/wit/outgoing.rs` have to stay in lockstep: if the Rust
// side admits a `T` the CLI does not, the program compiles and then fails at
// wasm-bindgen time with "unsupported type behind a reference". Binding all
// sixteen here turns that class of drift into a test failure. Adding a type to
// `scalar_into_wasm_abi!` means adding it here too.
//
// `i128`/`u128` are the interesting cases, since they span multiple ABI
// primitives and are therefore the most likely to be marshalled wrongly behind a
// reference. `isize`/`usize` are included because they describe differently by
// target word size, so the CLI has to accept both forms.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Signed integers.
    #[wasm_bindgen(js_name = takeSignedRefs)]
    fn take_signed_refs(a: &i8, b: &i16, c: &i32, d: &i64, e: &i128, f: &isize);

    // Unsigned integers.
    #[wasm_bindgen(js_name = takeUnsignedRefs)]
    fn take_unsigned_refs(a: &u8, b: &u16, c: &u32, d: &u64, e: &u128, f: &usize);

    // Floats, plus the two non-numeric scalars.
    #[wasm_bindgen(js_name = takeOtherRefs)]
    fn take_other_refs(a: &f32, b: &f64, c: &bool, d: &char);
}

// A `&T` hidden behind a type alias. The macro's reference detection is
// necessarily syntactic — a proc macro cannot resolve a type alias — so the alias
// form reaches the ABI layer and returns the pointee by copy. Pinned because it
// is reachable from ordinary user code and entirely silent: there is no
// diagnostic either way, so a change in the emitted ABI would go unnoticed.
type ScalarRef = &'static u32;

#[wasm_bindgen]
pub fn return_scalar_ref_via_alias() -> ScalarRef {
    &42
}

#[wasm_bindgen]
pub fn driver() {
    take_signed_refs(&-1i8, &-2i16, &-3i32, &-4i64, &-5i128, &-6isize);
    take_unsigned_refs(&1u8, &2u16, &3u32, &4u64, &5u128, &6usize);
    take_other_refs(&1.5f32, &2.5f64, &true, &'x');
}
