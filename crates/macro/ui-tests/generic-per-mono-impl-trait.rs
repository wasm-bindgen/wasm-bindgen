use wasm_bindgen::prelude::*;

// Argument-position `impl Trait` in a per-monomorphisation generic import
// (`#[wasm_bindgen(experimental_generic_mono)]`) is desugared into a synthesized named
// type parameter with the same bounds, exactly as if the user had written it
// out by hand (`fn f<T: Trait>(x: T)`). The declarations below all take that
// path silently; only the call sites at the bottom, which violate the
// (synthesized) bound, produce a diagnostic — proving the bound is actually
// carried through rather than dropped.
trait Marker {}

#[wasm_bindgen]
extern "C" {
    // A bare `impl Trait` argument on a function with no named type
    // parameter of its own: this is the shape `generic-per-mono-unsupported.rs`
    // notes would otherwise look like it has none.
    #[wasm_bindgen(experimental_generic_mono)]
    fn bare_impl_trait(x: impl Marker);

    // `impl Trait` mixed with a real, named type parameter.
    #[wasm_bindgen(experimental_generic_mono)]
    fn impl_trait_with_type_param<T>(x: impl Marker, y: T);

    // `impl Trait` nested inside another type.
    #[wasm_bindgen(experimental_generic_mono)]
    fn nested_impl_trait<T>(x: Vec<impl Marker>, y: T);

    // Two `impl Trait` arguments in the same function, each with a different
    // bound: both need their own synthesized parameter with distinct names.
    #[wasm_bindgen(experimental_generic_mono)]
    fn two_impl_trait_args(x: impl Marker, y: impl Iterator);
}

// `u32` satisfies the `IntoWasmAbi`/`WasmDescribe` bounds the codegen
// synthesizes, so only the user-written (here, desugared) bound can be what
// fails below.
fn violates_bare_impl_trait() {
    bare_impl_trait(1u32);
}

fn violates_impl_trait_with_type_param() {
    impl_trait_with_type_param(1u32, 2u32);
}

fn violates_nested_impl_trait() {
    nested_impl_trait(vec![1u32], 2u32);
}

fn violates_two_impl_trait_args() {
    two_impl_trait_args(1u32, 2u32);
}

fn main() {}
