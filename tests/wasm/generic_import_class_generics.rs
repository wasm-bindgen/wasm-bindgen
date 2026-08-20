//! Runtime coverage for *class-level* generic parameters on
//! `#[wasm_bindgen(generic_per_mono)]` imports: a type or lifetime parameter
//! of the function that also parameterises the receiver/return *class* type
//! itself (`this: &Holder<T>`, `-> Holder<T>`), rather than only the method's
//! own signature. This is the shape used throughout `js-sys` (`Array<T>`,
//! `Map<K, V>`, `Promise<T>`, ...), supported by *hoisting* the relevant
//! function generics onto the enclosing `impl` block's own generic header
//! (`get_fn_generics` / `class_return_path` in `codegen.rs`).
//!
//! `crates/macro/pass-tests/generic-per-mono-class-generics.rs` pins that
//! these shapes *compile*, and `crates/cli/tests/reference/generic-import.rs`
//! pins a subset of the JS they emit, but neither executes anything: a
//! hoisted parameter resolved against the wrong receiver, a swapped
//! `Pair<K, V>` argument order, or a bound that silently failed to reach the
//! manufactured shim would all produce perfectly plausible-looking glue.
//! These tests call each shape and assert on observable JS-side state.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_class_generics.js")]
extern "C" {
    type Holder<T>;

    // Constructor returning the parameterised class: `class_return_path`
    // retargets the generated `impl` block to `impl<T> Holder<T>`, hoisting
    // `T` off the constructor's own parameter list.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new<T>(value: T) -> Holder<T>;

    // Instance method: `T` is hoisted from the receiver `&Holder<T>` and
    // reused for the return type.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn get<T>(this: &Holder<T>) -> T;

    // Self-returning static method: `class_return_path` retargets the `impl`
    // block the same way it does for the constructor above.
    #[wasm_bindgen(static_method_of = Holder, generic_per_mono, js_name = of)]
    fn holder_of<T>(value: T) -> Holder<T>;

    // A hoisted class-level parameter (`T`) mixed with an additional,
    // non-hoisted function-only parameter (`U`).
    #[wasm_bindgen(method, generic_per_mono, js_name = combine)]
    fn combine<T, U>(this: &Holder<T>, other: U);

    // Deliberately plain (non-generic) inspection helpers: `Holder<T>` is
    // `#[repr(transparent)]` over a `JsValue` handle regardless of `T`, so
    // `.as_ref()` hands one of these a stable, ungeneric view.
    #[wasm_bindgen(js_name = holderValue)]
    fn holder_value(h: &JsValue) -> JsValue;
    #[wasm_bindgen(js_name = holderIsFromStatic)]
    fn holder_is_from_static(h: &JsValue) -> bool;
    #[wasm_bindgen(js_name = holderCombined)]
    fn holder_combined(h: &JsValue) -> JsValue;

    // A class-level *lifetime* parameter, rather than a type parameter.
    type LifetimeHolder<'a>;

    #[wasm_bindgen(constructor)]
    fn new_lifetime_holder<'a>(value: JsValue) -> LifetimeHolder<'a>;

    // The receiver's own lifetime `'a` is also the generic argument to the
    // class (`&'a LifetimeHolder<'a>`), so it is hoisted onto the `impl`
    // block's own header the same way a class-level type parameter is.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn lifetime_holder_get<'a, T>(this: &'a LifetimeHolder<'a>) -> T;

    // A class parameterised by *both* a lifetime and a type parameter of the
    // function, with both hoisted onto the same `impl` header.
    type LtHolder<'a, T>;

    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_lt_holder<'a, T>(v: &'a T) -> LtHolder<'a, T>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn lt_holder_get<'a, T>(this: &'a LtHolder<'a, T>) -> T;

    // Two class-level generic parameters (mirrors `Map<K, V>`).
    type Pair<K, V>;

    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_pair<K, V>(k: K, v: V) -> Pair<K, V>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn pair_get<K, V>(this: &Pair<K, V>) -> V;

    // Declaration order deliberately reversed relative to the class argument
    // list, pinning that `class_generic_params` (alphabetically ordered) and
    // `class_generic_exprs` (declaration-ordered) cannot desync.
    #[wasm_bindgen(method, generic_per_mono, js_name = swap)]
    fn pair_swap<V, K>(this: &Pair<V, K>) -> K;

    // The same parameter used twice: `class_generic_params` dedups to one
    // entry while `class_generic_exprs` stays two elements long.
    #[wasm_bindgen(method, generic_per_mono, js_name = both)]
    fn pair_both<T>(this: &Pair<T, T>) -> T;

    // A *concrete* class argument mixed with a hoisted one: `u32` is carried
    // through as written while `V` is hoisted, so the generated header is
    // `impl<V> Pair<u32, V>` — the method exists only for pairs keyed by a
    // `u32`.
    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn pair_u32_key_get<V>(this: &Pair<u32, V>) -> V;

    // A *fully* concrete class argument list: nothing is hoisted, so the
    // header is the bare `impl Pair<u32, String>` and only the function's own
    // `T` is monomorphised. Dropping the arguments instead would bind the
    // class's own parameter defaults (`Pair<JsValue, JsValue>`), and the
    // receiver below would not type-check against the generated method.
    #[wasm_bindgen(method, generic_per_mono, js_name = key)]
    fn pair_concrete_key<T>(this: &Pair<u32, String>, witness: T) -> u32;

    // The same fully concrete list reached through the constructor route
    // (`class_return_path`) rather than through a receiver.
    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_concrete_pair<T>(k: u32, v: T) -> Pair<u32, String>;

    type Boxed<T>;

    #[wasm_bindgen(constructor, generic_per_mono)]
    fn new_boxed<T>(tag: u32, value: T) -> Boxed<T>;

    // A hoisted class-level parameter that appears in no argument or return
    // position: the self type is the only thing constraining it.
    #[wasm_bindgen(method, generic_per_mono, js_name = tag)]
    fn boxed_tag<T>(this: &Boxed<T>) -> u32;

    // A *composed* class argument rather than a bare parameter: `T` is still
    // determined by the self type, so it can be hoisted.
    #[wasm_bindgen(method, generic_per_mono, js_name = nestedGet)]
    fn boxed_nested_get<T>(this: &Boxed<Option<T>>) -> u32;

    // An explicit `where`-clause bound on a hoisted parameter, re-emitted as a
    // predicate on the generated `impl` header (left on the wrapper's own
    // `where` clause it would not constrain the impl's parameter, per
    // RFC 447).
    #[wasm_bindgen(method, generic_per_mono, js_name = whereBound)]
    fn boxed_where_bound<T>(this: &Boxed<T>) -> u32
    where
        T: Clone;

    // The bound is load-bearing on the manufactured shim: its ABI signature
    // projects an associated type off the hoisted parameter (`T::Item`),
    // which only resolves with `T: IntoIterator` in scope. The shim is a
    // nested item and inherits nothing from the `impl` header the bound was
    // hoisted onto, so it is restated on the shim's own `where` clause too.
    #[wasm_bindgen(method, generic_per_mono, js_name = first)]
    fn boxed_where_projection<T>(this: &Boxed<T>, v: T::Item) -> u32
    where
        T: IntoIterator;

    // The same bound written inline, pinning that the two spellings stay
    // equivalent.
    #[wasm_bindgen(method, generic_per_mono, js_name = first)]
    fn boxed_inline_projection<T: IntoIterator>(this: &Boxed<T>, v: T::Item) -> u32;

    type Fallible<T>;

    // `catch` + constructor + class generics: the `Result` is unwrapped at
    // parse time, so `class_return_path` sees `Fallible<T>` through a
    // different route than the plain constructors above.
    #[wasm_bindgen(constructor, generic_per_mono, catch)]
    fn new_fallible<T>(value: T, fail: bool) -> Result<Fallible<T>, JsValue>;

    #[wasm_bindgen(method, generic_per_mono, js_name = get)]
    fn fallible_get<T>(this: &Fallible<T>) -> T;
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_constructor_and_get() {
    let holder_u32 = Holder::new(7u32);
    assert_eq!(holder_u32.get(), 7u32);
    assert_eq!(holder_value(holder_u32.as_ref()), JsValue::from(7u32));

    // A second monomorphisation must produce a distinct shim, not reuse the
    // first.
    let holder_string = Holder::new(String::from("hi"));
    assert_eq!(holder_string.get(), "hi");
    assert_eq!(holder_value(holder_string.as_ref()), JsValue::from("hi"));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_static_self_returning_method() {
    let holder_u32 = Holder::holder_of(5u32);
    assert_eq!(holder_u32.get(), 5u32);
    // Distinguishes the static-method route from the constructor route: this
    // fails if the call was routed to the constructor instead.
    assert!(holder_is_from_static(holder_u32.as_ref()));

    let holder_string = Holder::holder_of(String::from("s"));
    assert_eq!(holder_string.get(), "s");
    assert!(holder_is_from_static(holder_string.as_ref()));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_mixed_hoisted_and_non_hoisted_param() {
    let holder_u32 = Holder::new(1u32);
    holder_u32.combine(2.5f64);
    assert_eq!(holder_combined(holder_u32.as_ref()), JsValue::from(2.5f64));

    let holder_string = Holder::new(String::from("x"));
    holder_string.combine(String::from("y"));
    assert_eq!(holder_combined(holder_string.as_ref()), JsValue::from("y"));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_lifetime_only_parameter() {
    let holder = LifetimeHolder::new_lifetime_holder(JsValue::from(9u32));

    // Two distinct monomorphisations of `get` off the same lifetime-only
    // receiver.
    let as_u32: u32 = holder.lifetime_holder_get();
    assert_eq!(as_u32, 9);

    let as_value: JsValue = holder.lifetime_holder_get();
    assert_eq!(as_value, JsValue::from(9u32));
}

#[wasm_bindgen_test]
fn generic_per_mono_class_lifetime_and_type_param() {
    let value = JsValue::from(11u32);
    let holder = LtHolder::new_lt_holder(&value);
    let as_value: JsValue = holder.lt_holder_get();
    assert_eq!(as_value, JsValue::from(11u32));

    // The lifetime and type parameter are both erased at the ABI boundary
    // (`LtHolder<T>` is `#[repr(transparent)]` over a `JsValue` handle for
    // every `T`), so reinterpreting the same handle at a different `T` here
    // is legitimate, and exercises `get`'s own combined lifetime-and-type
    // hoist at a second, non-handle-shaped `T`.
    let holder_u32: &LtHolder<'_, u32> = holder.unchecked_ref();
    assert_eq!(holder_u32.lt_holder_get(), 11u32);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_two_generic_params() {
    let pair = Pair::new_pair(1u32, String::from("one"));
    assert_eq!(pair.pair_get(), "one");

    // `pair_swap<V, K>(this: &Pair<V, K>) -> K` declares its own generics in
    // reversed order relative to the class's `Pair<K, V>`: against this
    // receiver (`Pair<String, u32>`), that unifies `V = String`, `K = u32`,
    // so the return type `K` names the *second* positional class argument,
    // same as `pair_get`'s `V` does. If `class_generic_params`'s
    // alphabetical ordering ever desynced from `class_generic_exprs`'s
    // declaration order, this would instead try to marshal the first
    // positional argument (a `String`) as a `u32`.
    let flipped = Pair::new_pair(String::from("k"), 2u32);
    assert_eq!(flipped.pair_swap(), 2u32);

    // The same parameter used twice (`Pair<T, T>`).
    let same = Pair::new_pair(3u32, 3u32);
    assert_eq!(same.pair_both(), 3u32);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_concrete_generic_arguments() {
    let pair = Pair::new_pair(1u32, String::from("one"));

    // Mixed: the concrete `u32` key is re-emitted verbatim, `V` is hoisted.
    assert_eq!(pair.pair_u32_key_get(), "one");

    // Fully concrete receiver, two distinct monomorphisations of the
    // function's own (non-hoisted) parameter.
    assert_eq!(pair.pair_concrete_key(0u32), 1u32);
    assert_eq!(pair.pair_concrete_key(String::from("witness")), 1u32);

    // Fully concrete return type on the constructor route.
    let built = Pair::new_concrete_pair(2u32, String::from("two"));
    assert_eq!(built.pair_concrete_key(0u32), 2u32);
    assert_eq!(built.pair_get(), "two");
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_param_unused_in_signature() {
    let boxed = Boxed::new_boxed(1u32, 42u32);
    assert_eq!(boxed.boxed_tag(), 1);

    let boxed_string = Boxed::new_boxed(2u32, String::from("s"));
    assert_eq!(boxed_string.boxed_tag(), 2);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_composed_argument() {
    let boxed: Boxed<Option<u32>> = Boxed::new_boxed(3u32, Some(5u32));
    assert_eq!(boxed.boxed_nested_get(), 3);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_where_bound() {
    let boxed = Boxed::new_boxed(4u32, 42u32);
    assert_eq!(boxed.boxed_where_bound(), 4);

    let boxed_string = Boxed::new_boxed(5u32, String::from("s"));
    assert_eq!(boxed_string.boxed_where_bound(), 5);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_where_bound_projection() {
    let boxed: Boxed<Vec<String>> =
        Boxed::new_boxed(6u32, vec![String::from("a"), String::from("b")]);

    assert_eq!(boxed.boxed_where_projection(String::from("hi")), 2);
    assert_eq!(boxed.boxed_inline_projection(String::from("hey")), 3);
}

#[wasm_bindgen_test]
fn generic_per_mono_class_generic_catch_constructor() {
    let ok = Fallible::new_fallible(9u32, false).unwrap();
    assert_eq!(ok.fallible_get(), 9u32);

    let ok_string = Fallible::new_fallible(String::from("k"), false).unwrap();
    assert_eq!(ok_string.fallible_get(), "k");

    assert!(Fallible::new_fallible(9u32, true).is_err());
}
