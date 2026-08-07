use wasm_bindgen::prelude::*;

// `slice_to_array` names `<T as VectorRefIntoWasmAbi>` and describes the argument
// through `&Vec<T>`. A bare type parameter can satisfy neither, and no bound the
// user is able to write makes it satisfiable, because the blanket
// `VectorRefIntoWasmAbi` impls are keyed on concrete ABI shapes rather than on a
// public trait. Left unchecked the user gets errors naming private traits, so the
// combination is rejected up front.

#[wasm_bindgen]
extern "C" {
    // Applied to the function, whose element type is the type parameter.
    #[wasm_bindgen(slice_to_array)]
    fn direct<T>(xs: &[T]);

    // The `Option` form must be caught too.
    #[wasm_bindgen(slice_to_array)]
    fn optional<T>(xs: Option<&[T]>);

    // Nested in a concrete constructor, so a plain "is it an ident" test is not
    // enough — the element type still mentions `T`.
    #[wasm_bindgen(slice_to_array)]
    fn nested<T>(xs: &[Option<T>]);

    // A concrete element type in a generic function is fine: the type parameter
    // is not what is being marshalled as a slice.
    #[wasm_bindgen(slice_to_array)]
    fn concrete_elem_is_ok<T>(t: T, xs: &[u16]);
}

// Inherited from the enclosing block, which is the case a user is most likely to
// hit without having written `slice_to_array` near the offending argument.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    fn per_block<T>(xs: &[T]);
}

fn main() {}
