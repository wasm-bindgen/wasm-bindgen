//! Runtime coverage for lifetime parameters on
//! `#[wasm_bindgen(experimental_generic_mono)]` imports.
//!
//! Lifetimes carry no runtime information — they are erased before the wasm
//! ABI boundary — so the interesting part isn't marshalling, it's that the
//! generated per-monomorphisation shim (a nested `fn` item that inherits none
//! of the wrapper's generics) correctly redeclares whichever lifetime
//! parameters the signature actually names. These mirror
//! `generic_import_ref.rs`'s cases but with an explicit, named lifetime
//! instead of an elided one.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_lifetime.js")]
extern "C" {
    // A named (rather than elided) lifetime on a bare shared reference to a
    // generic type parameter (`&'a T`).
    #[wasm_bindgen(experimental_generic_mono, js_name = takeRefPrimitive)]
    fn take_ref_lifetime<'a, T>(x: &'a T) -> f64;

    // The same lifetime also appears in a `where T: 'a` bound.
    #[wasm_bindgen(experimental_generic_mono, js_name = doubleRef)]
    fn double_ref<'a, T: 'a>(x: &'a T) -> T;

    // A named lifetime on a concrete (non-generic) argument, alongside an
    // unrelated type parameter `T`. Unlike the two above, `'a` here doesn't
    // flow through the bare-shared-ref-to-a-generic-param special case at
    // all — it's just an ordinary borrow of a concrete type — so this
    // exercises the shim redeclaring `'a` purely to name
    // `<&'a str as IntoWasmAbi>::Abi` in its own signature.
    #[wasm_bindgen(experimental_generic_mono, js_name = concatStr)]
    fn concat_str<'a, T>(s: &'a str, suffix: T) -> String;

    // Two lifetimes related by an *inline* outlives predicate (`<'a: 'b, 'b>`),
    // which — unlike a `where` predicate — has no carrier other than the
    // parameter list itself, so the shim has to reproduce the whole
    // declaration rather than just the bare names.
    #[wasm_bindgen(experimental_generic_mono, js_name = joinStrs)]
    fn join_outlives<'a: 'b, 'b, T>(a: &'a str, b: &'b str, suffix: T) -> String;

    type Scaler;

    #[wasm_bindgen(constructor)]
    fn new(factor: f64) -> Scaler;

    // A named lifetime written on the *receiver*. The receiver is marshalled
    // as `<&'a Scaler as IntoWasmAbi>`, so the generated wrapper has to bind
    // it as `&'a self`; a plain `&self` would borrow at an anonymous
    // caller-chosen lifetime that rustc cannot prove outlives `'a`.
    #[wasm_bindgen(method, experimental_generic_mono, js_name = scale)]
    fn scale<'a, T>(this: &'a Scaler, x: T) -> f64;

    // The receiver's lifetime is shared with another argument, tying the two
    // borrows together for the duration of the call.
    #[wasm_bindgen(method, experimental_generic_mono, js_name = scaleRef)]
    fn scale_ref<'a, T>(this: &'a Scaler, x: &'a T) -> f64;
}

#[wasm_bindgen_test]
fn experimental_generic_mono_named_lifetime_ref() {
    assert_eq!(take_ref_lifetime(&JsValue::from(5u32)), 6.0);
    assert_eq!(take_ref_lifetime(&JsValue::from(2.5f64)), 3.5);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_lifetime_bound_on_type_param() {
    assert_eq!(double_ref(&JsValue::from(4u32)), JsValue::from(8u32));
    assert_eq!(double_ref(&JsValue::from(1.5f64)), JsValue::from(3.0f64));
}

#[wasm_bindgen_test]
fn experimental_generic_mono_lifetime_on_unrelated_concrete_arg() {
    assert_eq!(concat_str("hi", 42u32), "hi:42");
    assert_eq!(concat_str("hi", String::from("there")), "hi:there");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_inline_lifetime_outlives_bound() {
    let long = String::from("outer");
    {
        let short = String::from("inner");
        assert_eq!(join_outlives(&long, &short, 1u32), "outer|inner:1");
        assert_eq!(
            join_outlives(&long, &short, String::from("s")),
            "outer|inner:s"
        );
    }
}

#[wasm_bindgen_test]
fn experimental_generic_mono_named_lifetime_on_receiver() {
    let scaler = Scaler::new(3.0);
    assert_eq!(scaler.scale(4u32), 12.0);
    assert_eq!(scaler.scale(0.5f64), 1.5);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_receiver_lifetime_shared_with_argument() {
    let scaler = Scaler::new(10.0);
    let x = JsValue::from(7u32);
    assert_eq!(scaler.scale_ref(&x), 70.0);
    let y = JsValue::from(1.25f64);
    assert_eq!(scaler.scale_ref(&y), 12.5);
}
