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
//
// The `use_*` helpers below exist purely to *instantiate* each import, which is
// what forces per-monomorphisation codegen to run at all. Nothing calls them,
// and the imported types cannot be constructed here to call them with, so
// dead-code warnings are expected and suppressed.
#![allow(dead_code)]

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

#[wasm_bindgen]
extern "C" {
    // Two class-level generic parameters (mirrors `Map<K, V>`).
    type Pair<K, V>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn pair_get<K, V>(this: &Pair<K, V>) -> V;

    // Declaration order deliberately non-alphabetical and reversed relative to
    // the class argument list. `class_generic_params` is a `BTreeSet`, so it is
    // ordered alphabetically, while the self type's arguments come from the
    // declaration-ordered `class_generic_exprs`; this pins that the two cannot
    // desync into `impl<K, V> Pair<V, K>`-style mismatches.
    #[wasm_bindgen(method, generic_per_mono, js_name = swap)]
    fn pair_swap<V, K>(this: &Pair<V, K>) -> K;

    // The same parameter used twice: `class_generic_params` dedups to one entry
    // while `class_generic_exprs` stays two elements long.
    #[wasm_bindgen(method, generic_per_mono, js_name = both)]
    fn pair_both<T>(this: &Pair<T, T>) -> T;
}

#[wasm_bindgen]
extern "C" {
    type Boxed<T>;

    // A hoisted class-level parameter that appears in no argument or return
    // position, so the self type is the only thing constraining it.
    #[wasm_bindgen(method, generic_per_mono, js_name = tag)]
    fn boxed_tag<T>(this: &Boxed<T>) -> u32;

    // A *composed* class argument rather than a bare parameter: `T` is still
    // determined by the self type, so it can be hoisted.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn boxed_nested_get<T>(this: &Boxed<Option<T>>) -> u32;

    // An explicit `where`-clause bound on a hoisted parameter. This is the
    // exact shape that requires the bound to be re-emitted as a predicate on
    // the generated `impl` header: left on the wrapper method's own `where`
    // clause it would not constrain the impl's parameter (RFC 447), and any
    // parameter hoisted transitively out of such a bound would be an
    // unconstrained impl parameter (E0207).
    #[wasm_bindgen(method, generic_per_mono, js_name = dup)]
    fn boxed_where_bound<T>(this: &Boxed<T>) -> u32
    where
        T: Clone;
}

#[wasm_bindgen]
extern "C" {
    type Defaulted<T>;

    // A constructor whose return type's argument does not *determine* the
    // parameter (`T::Item`). `class_return_path` declines to hoist here, so the
    // arguments are stripped and the method hangs off the class's own defaults
    // (`impl Defaulted`). That is the established behaviour shared with the
    // type-erasure path, and is relied on by real imports such as
    // `js_sys::Promise::new_typed<T: Promising>(..) -> Promise<<T as
    // Promising>::Resolution>` — so it must keep compiling rather than being
    // rejected as an unhoistable class argument list.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_defaulted<T: IntoIterator>(v: u32) -> Defaulted<T::Item>;
}

#[wasm_bindgen]
extern "C" {
    type Fallible<T>;

    // `catch` + constructor + class generics: the `Result` is unwrapped at
    // parse time, so `class_return_path` sees `Fallible<T>` through a different
    // route than the plain constructor above.
    #[wasm_bindgen(constructor, generic_per_mono, catch)]
    fn new_fallible<T>(value: T) -> Result<Fallible<T>, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    type Bounded;

    // An inline lifetime bound (`'a: 'b`) relating two of the function's own
    // lifetimes. The generated shim redeclares its lifetimes with bounds
    // intact, so the wrapper has to carry the bound too or its declaration is
    // strictly weaker than the shim it calls, and the call fails with
    // "lifetime may not live long enough" against generated code. Inline
    // bounds have no parameter-list slot once a parameter may be hoisted onto
    // the `impl` header, so they are reified into `where` predicates by
    // `generics::generic_bounds`.
    //
    // Note the bound only becomes load-bearing when the lifetimes reach the
    // shim through a *generic* projection (`&'a T`): with a concrete `&'a
    // JsValue` the implied bounds of the argument position are enough to prove
    // the predicate on their own, and dropping `'a: 'b` goes unnoticed.
    #[wasm_bindgen(method, generic_per_mono, js_name = take)]
    fn take_bounded<'a: 'b, 'b, T: AsRef<JsValue>>(this: &Bounded, a: &'a T, b: &'b T);
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

fn use_pair(pair: &Pair<u32, String>, flipped: &Pair<String, u32>, same: &Pair<u32, u32>) {
    let _: String = pair.pair_get();
    let _: u32 = flipped.pair_swap();
    let _: u32 = same.pair_both();
}

fn use_boxed(boxed: &Boxed<u32>, nested: &Boxed<Option<u32>>) {
    let _: u32 = boxed.boxed_tag();
    let _: u32 = nested.boxed_nested_get();
    let _: u32 = boxed.boxed_where_bound();
}

fn use_fallible() -> Result<(), JsValue> {
    let _: Fallible<u32> = Fallible::new_fallible(1u32)?;
    let _: Fallible<String> = Fallible::new_fallible(String::from("hi"))?;
    Ok(())
}

fn use_bounded<'a: 'b, 'b>(bounded: &Bounded, a: &'a JsValue, b: &'b JsValue) {
    bounded.take_bounded(a, b);
}

fn main() {}
