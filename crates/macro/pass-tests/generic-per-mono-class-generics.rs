// Compile-pass coverage for class-level generics under the experimental
// per-monomorphisation generic import path (`#[wasm_bindgen(generic_per_mono)]`).
//
// A class-level generic is a type or lifetime parameter of the function that
// also parameterises the receiver/return *class* type itself (e.g.
// `this: &Holder<T>`, or `-> Holder<T>` for a constructor / self-returning
// static method). This is the shape used throughout `js-sys` (`Array<T>`,
// `Map<K, V>`, `Promise<T>`, ...), and is supported by *hoisting* the
// function's own generic parameters that the class type's argument list uses
// onto the enclosing `impl` block's own generic header, reusing the same
// `get_fn_generics` hoisting analysis the type-erasure generic path already
// relies on for the same shape. See `try_to_tokens_generic` /
// `get_fn_generics` in `codegen.rs`.
//
// This only needs to *compile*; `crates/cli/tests/reference/generic-import.rs`
// pins the actual generated JS/Wasm output end-to-end.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Holder<T>;

    // Constructor returning the parameterised class: the generated `impl`
    // becomes `impl<T> Holder<T>` rather than a bare `impl Holder`, since `T`
    // is hoisted off the constructor's own parameter list.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new<T>(value: T) -> Holder<T>;

    // Instance method: `T` is hoisted from the receiver `&Holder<T>` and
    // reused for the return type (mirrors `Array::at`/`Array::get`).
    #[wasm_bindgen(method, generic_per_mono)]
    fn get<T>(this: &Holder<T>) -> T;

    // Self-returning static method (mirrors `Array::of`): `class_return_path`
    // retargets the `impl` block to the return type's class the same way it
    // does for the constructor above.
    #[wasm_bindgen(static_method_of = Holder, generic_per_mono, js_name = of)]
    fn holder_of<T>(value: T) -> Holder<T>;

    // A hoisted class-level parameter (`T`) mixed with an additional,
    // non-hoisted function-only parameter (`U`): proves the split between
    // `class_generic_params` and `fn_generic_params` in `get_fn_generics`
    // works, and that both still end up correctly marshalled.
    #[wasm_bindgen(method, generic_per_mono)]
    fn combine<T, U>(this: &Holder<T>, other: U);
}

#[wasm_bindgen]
extern "C" {
    // A class-level *lifetime* parameter, rather than a type parameter.
    type LifetimeHolder<'a>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn get_lifetime<'a, T>(this: &'a LifetimeHolder<'a>) -> T;
}

fn use_holder(holder_u32: &Holder<u32>, holder_string: &Holder<String>) {
    // Two distinct instantiations of `get` prove two distinct manufactured
    // shims are generated and both marshal correctly.
    let _: u32 = holder_u32.get();
    let _: String = holder_string.get();

    // Two distinct instantiations of the constructor.
    let _: Holder<u32> = Holder::new(1u32);
    let _: Holder<String> = Holder::new(String::from("hi"));

    // Two distinct instantiations of the self-returning static method.
    let _: Holder<u32> = Holder::holder_of(2u32);
    let _: Holder<String> = Holder::holder_of(String::from("bye"));

    // Mixed hoisted (`T`) / non-hoisted (`U`) parameters, called with
    // distinct `T`/`U` combinations.
    holder_u32.combine(3.0f64);
    holder_string.combine(4u32);
}

fn use_lifetime_holder<'a>(holder: &'a LifetimeHolder<'a>) {
    let _: u32 = holder.get_lifetime();
    let _: String = holder.get_lifetime();
}

fn main() {}
