//! Runtime coverage for `async` and `slice_to_array` per-monomorphisation
//! generic imports (`#[wasm_bindgen(experimental_generic_mono)]`).
//!
//! These assert the *observable* behaviour, which a reference snapshot cannot:
//! the reference test only pins the JS text, so it would happily accept a
//! regression that re-described an `async` return as its inner type (marshalling
//! the `Promise` as if it were a `u32`) or that silently ignored
//! `slice_to_array` (handing JS a typed-array view instead of an `Array`).

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import_async.js")]
extern "C" {
    // An `async` import returns a `Promise` across the ABI whatever it resolves
    // to, so a monomorphised `-> T` works even when `T` is not handle-shaped.
    #[wasm_bindgen(experimental_generic_mono, js_name = asyncEcho)]
    async fn async_echo<T>(x: T) -> T;

    // A concrete, non-handle return type through the same path.
    #[wasm_bindgen(experimental_generic_mono, js_name = asyncLen)]
    async fn async_len<T>(x: T) -> u32;

    // `catch` monomorphises the `Ok` type and hard-codes `JsValue` as the error.
    #[wasm_bindgen(experimental_generic_mono, catch, js_name = asyncTryEcho)]
    async fn async_try_echo<T>(x: T, fail: bool) -> Result<T, JsValue>;

    // An `async` import with no return value still resolves to `JsValue`.
    #[wasm_bindgen(experimental_generic_mono, js_name = asyncRecord)]
    async fn async_record<T>(x: T);

    #[wasm_bindgen(js_name = takeLog)]
    fn take_log() -> String;

    // `slice_to_array` hands JS an owned `Array`, not a view into wasm memory.
    // A primitive element type borrows the caller's slice; a `String` element
    // type hands over a freshly allocated buffer JS must free.
    #[wasm_bindgen(experimental_generic_mono, slice_to_array, js_name = kindOf)]
    fn kind_of_slice<T>(xs: &[u16], other: T) -> String;

    #[wasm_bindgen(experimental_generic_mono, slice_to_array, js_name = kindOf)]
    fn kind_of_str_slice<T>(xs: &[String], other: T) -> String;

    #[wasm_bindgen(experimental_generic_mono, slice_to_array, js_name = kindOf)]
    fn kind_of_opt_slice<T>(xs: Option<&[u16]>, other: T) -> String;

    // `Option<&[String]>` is the allocating-and-freeing element path *and* the
    // `Option` path at once.
    #[wasm_bindgen(experimental_generic_mono, slice_to_array, js_name = joinOf)]
    fn join_of_opt_str_slice<T>(xs: Option<&[String]>, other: T) -> String;

    // Without the attribute the same signature must still hand JS a view, so
    // these two together prove the attribute is what makes the difference.
    #[wasm_bindgen(experimental_generic_mono, js_name = kindOf)]
    fn kind_of_slice_plain<T>(xs: &[u16], other: T) -> String;

    // `slice_to_array` is not slice-shaped-only-by-accident: `Vec<T>` is not a
    // slice, so the attribute is a documented no-op there.
    #[wasm_bindgen(experimental_generic_mono, slice_to_array, js_name = kindOf)]
    fn kind_of_vec<T>(xs: Vec<T>) -> String;
}

// `slice_to_array` is inheritable from the enclosing block.
#[wasm_bindgen(module = "tests/wasm/generic_import_async.js", slice_to_array)]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono, js_name = kindOf)]
    fn kind_of_block_slice<T>(xs: &[u16], other: T) -> String;
}

#[wasm_bindgen_test]
async fn async_generic_return() {
    // A `T` that is *not* handle-shaped is the case that used to be
    // mis-marshalled: the promise was described as a `u32`/string.
    assert_eq!(async_echo(7u32).await, 7);
    assert_eq!(async_echo(2.5f64).await, 2.5);
    assert_eq!(async_echo(String::from("hi")).await, "hi");
    // ...and one that is, to be sure the handle path still works.
    assert_eq!(async_echo(JsValue::from(3u32)).await, JsValue::from(3u32));
}

#[wasm_bindgen_test]
async fn async_concrete_return() {
    assert_eq!(async_len(String::from("abcd")).await, 4);
    assert_eq!(async_len(12u32).await, 2);
}

#[wasm_bindgen_test]
async fn async_option_and_vec_return() {
    assert_eq!(async_echo(Some(5u32)).await, Some(5u32));
    assert_eq!(async_echo(None::<u32>).await, None);
    assert_eq!(async_echo(vec![1u32, 2]).await, vec![1u32, 2]);
}

#[wasm_bindgen_test]
async fn async_catch_return() {
    assert_eq!(async_try_echo(9u32, false).await.unwrap(), 9);
    assert_eq!(async_try_echo(String::from("k"), false).await.unwrap(), "k");
    assert!(async_try_echo(9u32, true).await.is_err());
}

#[wasm_bindgen_test]
async fn async_unit_return() {
    async_record(1u32).await;
    async_record(2.5f64).await;
    assert_eq!(take_log(), "1,2.5");
}

#[wasm_bindgen_test]
fn slice_to_array_hands_js_an_array() {
    // The attribute is what makes the difference...
    assert_eq!(kind_of_slice(&[1u16, 2], 0u32), "Array");
    assert_eq!(kind_of_slice_plain(&[1u16, 2], 0u32), "Uint16Array");

    // ...on the allocating element path too...
    assert_eq!(kind_of_str_slice(&[String::from("a")], 0u32), "Array");

    // ...through `Option<&[T]>`...
    assert_eq!(kind_of_opt_slice(Some(&[1u16]), 0u32), "Array");
    assert_eq!(kind_of_opt_slice(None, 0u32), "undefined");

    // ...and when inherited from the enclosing block.
    assert_eq!(kind_of_block_slice(&[1u16], 0u32), "Array");

    // `Option<&[String]>` exercises the `Option` and the allocating element
    // paths together; check the contents survive, not just the type.
    assert_eq!(
        join_of_opt_str_slice(Some(&[String::from("x"), String::from("y")]), 0u32),
        "Array:x|y"
    );
    assert_eq!(join_of_opt_str_slice(None, 0u32), "undefined");

    // `Vec<T>` is not slice-shaped, so the attribute is a no-op.
    assert_eq!(kind_of_vec(vec![1u16, 2]), "Uint16Array");
}

#[wasm_bindgen_test]
fn slice_to_array_leaves_the_caller_slice_intact() {
    let xs = [3u16, 4, 5];
    assert_eq!(kind_of_slice(&xs, 0u32), "Array");
    assert_eq!(xs, [3u16, 4, 5]);
}

#[wasm_bindgen_test]
fn slice_to_array_leaves_an_allocating_caller_slice_intact() {
    // The allocating element path is the dangerous one: the generated glue reads
    // the externref index buffer and then frees *it* (`len * 4` bytes). Freeing
    // the elements instead would be a use-after-free that assertions on the
    // return value alone cannot see, because the return value is computed before
    // the free. Read the caller's slice back afterwards instead.
    let xs = [String::from("a"), String::from("b")];

    assert_eq!(kind_of_str_slice(&xs, 0u32), "Array");
    assert_eq!(xs, [String::from("a"), String::from("b")]);

    // Through the `Option` path too, and with the contents checked on the JS
    // side so both directions are pinned.
    assert_eq!(join_of_opt_str_slice(Some(&xs), 0u32), "Array:a|b");
    assert_eq!(xs, [String::from("a"), String::from("b")]);

    // Using the slice again after two crossings must still be sound.
    assert_eq!(kind_of_str_slice(&xs, 1.5f64), "Array");
    assert_eq!(xs.join("|"), "a|b");
}
