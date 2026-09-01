//! Runtime coverage for argument and return shapes in
//! `#[wasm_bindgen(experimental_generic_mono)]` imports whose failure modes are silent —
//! the generated JS looks correct in a snapshot either way.
//!
//! - `&mut [u16]`: must be a live view into wasm memory, so JS's writes reach
//!   the caller. A copy would produce identical-looking glue.
//! - `&mut dyn FnMut`: must be invoked the expected number of times, with the
//!   reentrancy guard released afterwards, including when its own signature is
//!   generic.
//! - non-async `catch` with a *generic* `Ok`: the success value is marshalled at
//!   each monomorphisation's concrete type while the error stays `JsValue`; the
//!   throwing path is a different unwrap from the async one covered elsewhere.
//! - `Option<T>` in the declared signature: composes the `Option` hole and
//!   `isLikeNone` handling with per-monomorphisation descriptor generation.
//! - one bare `T` across `&str`/`String`/`u32`: three wire protocols behind
//!   two JS-visible types, so a shim marshalling the wrong one produces a
//!   wrong *value*, not a crash.
//! - `&[T]` with a generic element type: takes the `&T` HRTB route rather
//!   than the concrete-slice route, both as a plain and a `variadic` argument.

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsStringLike, JsStringLikeOwn};
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_args.js")]
extern "C" {
    // `&mut` to a *concrete* type inside a generic import. Binds exactly as on
    // the non-generic path: a mutable typed-array view that is written back.
    #[wasm_bindgen(experimental_generic_mono, js_name = fillSlice)]
    fn fill_slice<T>(xs: &mut [u16], base: T);

    #[wasm_bindgen(experimental_generic_mono, js_name = withCallback)]
    fn with_callback<T>(f: &mut dyn FnMut(u32), times: T);

    #[wasm_bindgen(experimental_generic_mono)]
    fn transform<T, U>(value: T, f: &mut dyn FnMut(T) -> U) -> U;

    #[wasm_bindgen(experimental_generic_mono, js_name = transform)]
    fn transform_shared<T, U>(value: T, f: &dyn Fn(T) -> U) -> U;

    #[wasm_bindgen(experimental_generic_mono, catch, js_name = tryGet)]
    fn try_get<T>(key: u32) -> Result<T, JsValue>;

    #[wasm_bindgen(experimental_generic_mono, catch, js_name = tryGetString)]
    fn try_get_string<T>(key: u32) -> Result<T, JsValue>;

    #[wasm_bindgen(experimental_generic_mono, js_name = optEcho)]
    fn opt_echo<T>(x: Option<T>) -> Option<T>;

    #[wasm_bindgen(experimental_generic_mono, js_name = optDescribe)]
    fn opt_describe<T>(x: Option<T>) -> String;

    // One import across the three string/number wire shapes: `&str` crosses as
    // a borrowed (ptr, len) pair with no free, `String` as an owned buffer the
    // shim frees, `u32` as a plain scalar. JS reports `typeof` plus the value,
    // so marshalling the wrong wire form surfaces as a mismatched value.
    #[wasm_bindgen(experimental_generic_mono, js_name = describeAny)]
    fn describe_any<T>(x: T) -> String;

    // A shared slice with a *generic* element type: `&[T]` takes the `&T`
    // HRTB route (`for<'a> &'a [T]: IntoWasmAbi`) rather than the
    // concrete-slice route, and each element type gets its own typed-array
    // view.
    #[wasm_bindgen(experimental_generic_mono, js_name = sumSlice)]
    fn sum_slice<T>(xs: &[T]) -> f64;

    // `JsStringLike`/`JsStringLikeOwn` bounds: every string shape crosses at its
    // native wire format (UTF-8 buffer vs handle) but must arrive in JS as a
    // string *value*, and a JS string return must be receivable as either an
    // owned copy or a handle.
    #[wasm_bindgen(experimental_generic_mono, js_name = describeStr)]
    fn describe_str<T: JsStringLike>(s: T) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = makeStr)]
    fn make_str<T: JsStringLikeOwn>() -> T;
}

#[wasm_bindgen_test]
fn experimental_generic_mono_mut_slice_is_written_back() {
    let mut xs = [0u16; 4];

    fill_slice(&mut xs, 10u32);

    // JS wrote through a live view into wasm memory. If the binding copied the
    // slice across, `xs` would still be all zeroes.
    assert_eq!(xs, [10u16, 11, 12, 13]);

    // A second monomorphisation of the same import.
    fill_slice(&mut xs, 100.0f64);
    assert_eq!(xs, [100u16, 101, 102, 103]);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_mut_closure_is_invoked() {
    let mut seen = Vec::new();
    {
        // Mutable captures require an explicit unwind-safety opt-in.
        let mut seen = core::panic::AssertUnwindSafe(&mut seen);
        let mut push = move |v: u32| seen.push(v);
        with_callback(&mut push, 3u32);
    }
    assert_eq!(seen, vec![0, 1, 2]);

    // The reentrancy guard must have been released, so the same import can be
    // used again with a different monomorphisation.
    let mut count = 0u32;
    {
        let mut count = core::panic::AssertUnwindSafe(&mut count);
        let mut bump = move |_: u32| **count += 1;
        with_callback(&mut bump, 2.0f64);
    }
    assert_eq!(count, 2);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_callback_signature() {
    let mut double = |value: u32| value * 2;
    assert_eq!(transform(3u32, &mut double), 6);

    let mut label = |value: f64| format!("value:{value}");
    assert_eq!(transform(2.5f64, &mut label), "value:2.5");

    let increment = |value: u32| value + 1;
    assert_eq!(transform_shared(3u32, &increment), 4);

    let label = |value: f64| format!("shared:{value}");
    assert_eq!(transform_shared(2.5f64, &label), "shared:2.5");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_string_arg_bound() {
    // JS reports `typeof` plus the value, so a shape that crossed as anything
    // other than a string value surfaces as a mismatch.
    assert_eq!(describe_str("borrowed"), "string:borrowed");
    assert_eq!(describe_str(String::from("owned")), "string:owned");

    let js = JsString::from("handle");
    assert_eq!(describe_str(&js), "string:handle");
    assert_eq!(describe_str(js), "string:handle");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_string_ret_bound() {
    assert_eq!(make_str::<String>(), "made");
    assert_eq!(make_str::<JsString>().to_string(), "made");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_catch_ok_path() {
    // Generic `Ok`, marshalled at each concrete type.
    assert_eq!(try_get::<u32>(3).unwrap(), 6);
    assert_eq!(try_get::<f64>(4).unwrap(), 8.0);
    assert_eq!(try_get_string::<String>(7).unwrap(), "v7");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_catch_throw_path() {
    // A JS `throw` must surface as `Err`, not as a trap or a garbage `Ok`.
    let err = try_get::<u32>(0).unwrap_err();
    assert!(
        err.as_string().unwrap_or_default().contains("boom") || format!("{err:?}").contains("boom"),
        "expected the thrown Error to survive as the Err payload, got {err:?}"
    );

    assert!(try_get_string::<String>(0).is_err());

    // The exception slot must be cleared, so a subsequent call still succeeds.
    assert_eq!(try_get::<u32>(5).unwrap(), 10);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_option_in_signature() {
    // `Option<T>` as both argument and return, at two concrete `T`s.
    assert_eq!(opt_echo(Some(5u32)), Some(5u32));
    assert_eq!(opt_echo::<u32>(None), None);

    assert_eq!(opt_echo(Some(2.5f64)), Some(2.5f64));
    assert_eq!(opt_echo::<f64>(None), None);

    // Confirm the JS side really saw a present/absent value rather than a
    // sentinel that happened to round-trip.
    assert_eq!(opt_describe(Some(7u32)), "some:7");
    assert_eq!(opt_describe::<u32>(None), "none");
    assert_eq!(opt_describe(Some(String::from("s"))), "some:s");
}

#[wasm_bindgen(module = "tests/wasm/generic_import_args.js")]
extern "C" {
    // `variadic` spreads the final argument. It must be a sequence shape when it
    // mentions a type parameter, because iterability then depends on the
    // instantiation rather than on the declaration.
    #[wasm_bindgen(experimental_generic_mono, variadic, js_name = variadicJoin)]
    fn variadic_join<T>(first: u32, rest: Vec<T>) -> String;

    // The same JS function fed from a borrowed generic-element slice instead
    // of an owned `Vec<T>` — the other sequence shape the non-sequence
    // variadic diagnostic recommends.
    #[wasm_bindgen(experimental_generic_mono, variadic, js_name = variadicJoin)]
    fn variadic_join_slice<T>(first: u32, rest: &[T]) -> String;
}

#[wasm_bindgen_test]
fn experimental_generic_mono_variadic_actually_spreads() {
    // The JS side reports `rest.length`, so a binding that passed the `Vec` as a
    // single array argument instead of spreading it reports 1 rather than 3.
    assert_eq!(variadic_join(7, vec![1u32, 2, 3]), "7:3:1|2|3");

    // A second monomorphisation, and the empty case.
    assert_eq!(variadic_join(8, vec![1.5f64, 2.5]), "8:2:1.5|2.5");
    assert_eq!(variadic_join::<u32>(9, vec![]), "9:0:");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_string_and_number_wire_shapes() {
    assert_eq!(describe_any("borrowed"), "string:borrowed");
    assert_eq!(describe_any(String::from("owned")), "string:owned");
    assert_eq!(describe_any(13u32), "number:13");
    assert_eq!(describe_any(14.0f64), "number:14");
}

#[wasm_bindgen_test]
fn experimental_generic_mono_generic_element_slice() {
    assert_eq!(sum_slice(&[1u32, 2, 3]), 6.0);
    assert_eq!(sum_slice(&[1.5f64, 2.5]), 4.0);
    assert_eq!(sum_slice::<u32>(&[]), 0.0);
}

#[wasm_bindgen_test]
fn experimental_generic_mono_variadic_slice_spreads() {
    assert_eq!(variadic_join_slice(7, &[1u32, 2, 3]), "7:3:1|2|3");
    assert_eq!(variadic_join_slice(8, &[1.5f64]), "8:1:1.5");
    assert_eq!(variadic_join_slice::<u32>(9, &[]), "9:0:");
}
