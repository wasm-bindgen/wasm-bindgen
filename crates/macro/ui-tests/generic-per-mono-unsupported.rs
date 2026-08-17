use wasm_bindgen::prelude::*;

// The experimental per-monomorphisation generic import path
// (`#[wasm_bindgen(generic_per_mono)]`) rejects a handful of shapes with a
// clear diagnostic, deferring them to the type-erasure generic path. Each
// function below exercises one of those aborting paths.
#[wasm_bindgen]
extern "C" {
    // `generic_per_mono` requires at least one type parameter.
    #[wasm_bindgen(generic_per_mono)]
    fn without_type_param(x: u32);

    // Argument-position `impl Trait` desugars to an anonymous type
    // parameter, so this function does have one; it's just nameless, and
    // never shows up in `without_type_param`'s check above. It gets its own
    // diagnostic rather than a misleading "at least one type parameter" one.
    #[wasm_bindgen(generic_per_mono)]
    fn impl_trait_arg(x: impl Clone);

    // Same rejection when `impl Trait` is mixed with a real, named type
    // parameter, or nested inside another type.
    #[wasm_bindgen(generic_per_mono)]
    fn impl_trait_arg_with_type_param<T>(x: impl Clone, y: T);

    #[wasm_bindgen(generic_per_mono)]
    fn nested_impl_trait_arg<T>(x: Vec<impl Clone>, y: T);

    // A bare shared reference to a generic type parameter (`&T`) *is* now
    // supported, but a mutable reference (`&mut T`) is not.
    #[wasm_bindgen(generic_per_mono)]
    fn mut_ref_to_generic<T>(x: &mut T);

    // Nor is a reference to a generic parameter nested inside another type
    // (e.g. `Option<&T>`).
    #[wasm_bindgen(generic_per_mono)]
    fn nested_ref_to_generic<T>(x: Option<&T>);

    // Returning a reference is not supported.
    #[wasm_bindgen(generic_per_mono)]
    fn return_ref<T>(x: T) -> &JsValue;

    // A bare generic type parameter cannot be the `variadic` argument, since it
    // may monomorphise to a non-iterable scalar.
    #[wasm_bindgen(generic_per_mono, variadic)]
    fn variadic_scalar<T>(first: u32, rest: T);

    // Nor can a reference to one: `&T` crosses the ABI as whatever `T` does, so
    // it is just as unspreadable.
    #[wasm_bindgen(generic_per_mono, variadic)]
    fn variadic_ref<T>(first: u32, rest: &T);

    // Nor any other non-sequence container. These are rejected by the same
    // allow-list: `Option<T>` is `undefined` or a bare value, `Box<T>` marshals
    // as its pointee, and an associated type is whatever the bound resolves it
    // to — none of which is iterable for every monomorphisation.
    #[wasm_bindgen(generic_per_mono, variadic)]
    fn variadic_option<T>(first: u32, rest: Option<T>);

    #[wasm_bindgen(generic_per_mono, variadic)]
    fn variadic_box<T>(first: u32, rest: Box<T>);

    #[wasm_bindgen(generic_per_mono, variadic)]
    fn variadic_assoc<T: IntoIterator>(first: u32, rest: T::Item);

    // `catch` hard-codes the error type to `JsValue` and monomorphises only the
    // `Ok` type, so a type parameter in the error position is rejected.
    #[wasm_bindgen(generic_per_mono, catch)]
    fn catch_generic_err<T>(x: T) -> Result<JsValue, T>;

    // The rejection is on *any* `&mut` whose referent mentions a type
    // parameter, not just a bare `&mut T`.
    #[wasm_bindgen(generic_per_mono)]
    fn mut_ref_to_nested_generic<T>(x: &mut Vec<T>);

    // The four further nested-reference shapes `references_generic_param`
    // documents. Each has a reference to `T` somewhere inside a larger type, so
    // none of them is a *top-level* shared ref and all need the erasure path.
    #[wasm_bindgen(generic_per_mono)]
    fn ref_ref_to_generic<T>(x: &&T);

    #[wasm_bindgen(generic_per_mono)]
    fn tuple_with_ref_to_generic<T>(x: (T, &T));

    #[wasm_bindgen(generic_per_mono)]
    fn array_of_refs_to_generic<T>(x: [&T; 2]);

    #[wasm_bindgen(generic_per_mono)]
    fn boxed_ref_to_generic<T>(x: Box<&T>);

    // An argument whose pattern is not a plain ident or `_` has no name for the
    // generated shim to forward, so per-mono codegen rejects it.
    #[wasm_bindgen(generic_per_mono)]
    fn tuple_pattern_arg<T>((a, b): (u32, u32), x: T);
}

// Class-level generics on the imported type (`Holder<T>` / `LifetimeHolder<'a>`
// used as a method receiver, constructor return, or self-returning static
// method return) are now supported by hoisting the relevant function generics
// onto the `impl` block's own header — see
// `pass-tests/generic-per-mono-class-generics.rs` and
// `crates/cli/tests/reference/generic-import.rs` for passing coverage.

// The rejections below happen while *parsing* the block rather than while
// generating its tokens, and a parse failure aborts the whole block, swallowing
// every codegen-time diagnostic in it. Keep them in their own block so they
// don't mask the ones above.
#[wasm_bindgen]
extern "C" {
    // `assert_no_shim` asserts that no shim is generated, which per-mono codegen
    // can never satisfy: it manufactures one shim per monomorphisation.
    #[wasm_bindgen(generic_per_mono, assert_no_shim)]
    fn asserted_no_shim<T>(x: T);

    // `reexport` names a single descriptor shim, and a per-mono import has no
    // single shim to name.
    #[wasm_bindgen(generic_per_mono, reexport)]
    fn reexported<T>(x: T);

    // Const generic parameters are rejected by `validate_generics` for *every*
    // wasm-bindgen generic, erased or per-mono, so this surfaces as the shared
    // "unsupported in wasm-bindgen generics" error rather than a per-mono one.
    #[wasm_bindgen(generic_per_mono)]
    fn const_generic<const N: usize, T>(x: T);

    // A `?Sized` bound relaxes an implicit bound rather than adding one, which
    // `validate_generics` rejects for *every* wasm-bindgen generic. Like the
    // const-generic case above this is the shared parse-time error, not a
    // per-mono one -- the erasure path already covers the same message.
    #[wasm_bindgen(generic_per_mono)]
    fn unsized_type_param<T: ?Sized>(x: &T);

    // `slice_to_array` needs a concrete element type: `VectorRefIntoWasmAbi` is
    // implemented per concrete ABI shape, so no bound makes `&[T]` work. This is
    // also a parse-time rejection, so it belongs in this block: leaving it in the
    // codegen-time block above would abort that block and swallow every
    // diagnostic in it.
    #[wasm_bindgen(generic_per_mono, slice_to_array)]
    fn slice_to_array_generic_elem<T>(xs: &[T], other: T);

    // Also rejected when only nested inside the element type...
    #[wasm_bindgen(generic_per_mono, slice_to_array)]
    fn slice_to_array_nested_elem<T>(xs: &[Vec<T>], other: T);

    // ...and through the `Option<&[T]>` form.
    #[wasm_bindgen(generic_per_mono, slice_to_array)]
    fn slice_to_array_option_elem<T>(xs: Option<&[T]>, other: T);
}

// `slice_to_array` is inherited from the enclosing block, and the same rejection
// applies on the type-erasure generic path, which has no `generic_per_mono`.
#[wasm_bindgen(slice_to_array)]
extern "C" {
    fn erased_slice_to_array_generic_elem<T>(xs: &[T]);
}

fn main() {}
