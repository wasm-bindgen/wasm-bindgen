use wasm_bindgen::prelude::*;

// Trait bounds declared on a per-monomorphisation generic import
// (`#[wasm_bindgen(experimental_generic_mono)]`) are part of its contract: they are
// carried through codegen, so a caller that violates one is rejected, and the
// diagnostic points at the user's own bound. Both an inline bound and a `where`
// predicate are exercised, since they reach the generated wrapper by different
// routes.

// Real library traits are exercised too, so that the diagnostic a caller
// actually sees for a familiar bound is pinned down, and so that the generic
// arguments inside a bound path (`IntoIterator<Item = String>`) are covered:
// carrying only the trait name would let the last call below compile.

// `u32` satisfies the `IntoWasmAbi`/`WasmDescribe` bounds the codegen
// synthesizes, so only the user-written bound can be what fails below. The same
// holds for `Vec<u32>`.
trait Marker {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn inline_bound<T: Marker>(x: T);

    #[wasm_bindgen(experimental_generic_mono)]
    fn where_bound<T>(x: T)
    where
        T: Marker;

    #[wasm_bindgen(experimental_generic_mono)]
    fn iter_bound<T: Iterator>(x: T);

    #[wasm_bindgen(experimental_generic_mono)]
    fn iter_item_bound<T>(x: T)
    where
        T: IntoIterator<Item = String>;
}

fn violates_inline_bound() {
    inline_bound(1u32);
}

fn violates_where_bound() {
    where_bound(2u32);
}

fn violates_real_trait_bound() {
    iter_bound(3u32);
}

fn violates_associated_type_binding() {
    iter_item_bound(vec![4u32]);
}

fn main() {}
