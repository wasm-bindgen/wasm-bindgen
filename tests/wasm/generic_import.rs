use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import.js")]
extern "C" {
    // Opt-in per-monomorphisation generic import. Each concrete `T`
    // gets its own JS adapter with `T`-specific marshalling, all bound
    // to the same JS `record_generic` function. The `IntoWasmAbi +
    // WasmDescribe` bounds are supplied by the macro expansion.
    #[wasm_bindgen(generic)]
    fn record_generic<T>(x: T);

    // Multiple arguments, mixed concrete + generic, with the generic
    // parameter repeated across two argument positions.
    #[wasm_bindgen(generic)]
    fn record_mixed<T>(a: T, b: u32, c: T);

    // Multiple distinct type parameters.
    #[wasm_bindgen(generic)]
    fn record_two<A, B>(a: A, b: B);

    // Generic argument and generic return (round-trip).
    #[wasm_bindgen(generic)]
    fn groundtrip<T>(x: T) -> T;

    // Zero arguments, generic return position only.
    #[wasm_bindgen(generic)]
    fn gget<T>() -> T;

    // Generic parameter nested inside `Option<_>`.
    #[wasm_bindgen(generic)]
    fn record_opt<T>(x: Option<T>);

    // Generic parameter behind a shared reference.
    #[wasm_bindgen(generic)]
    fn record_ref<T>(x: &T);

    // `catch` with a generic argument and generic `Ok` return.
    #[wasm_bindgen(generic, catch)]
    fn try_maybe<T>(x: T) -> Result<T, JsValue>;

    fn take_generic_log() -> Vec<JsValue>;

    type Recorder;
    #[wasm_bindgen(constructor)]
    fn new() -> Recorder;
    // Generic instance method on a concrete class: each `T` binds to the
    // same JS `pushVal` with `T`-specific marshalling.
    #[wasm_bindgen(method, generic, js_name = "pushVal")]
    fn push_val<T>(this: &Recorder, x: T);
    #[wasm_bindgen(method, getter)]
    fn last(this: &Recorder) -> JsValue;
}

#[wasm_bindgen_test]
fn generic_import_marshals_per_type() {
    record_generic(42u32);
    record_generic(3.5f64);
    record_generic("hello");

    let log = take_generic_log();
    assert_eq!(log.len(), 3);
    // u32 arrives as a JS number.
    assert_eq!(log[0].as_f64(), Some(42.0));
    // f64 arrives as a JS number.
    assert_eq!(log[1].as_f64(), Some(3.5));
    // &str arrives as a JS string (decoded from the two-word ABI) —
    // impossible under the type-erased generics path.
    assert_eq!(log[2].as_string(), Some("hello".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_mixed_and_repeated_params() {
    // `T = &str` at positions 0 and 2 (one hole, used twice), concrete
    // `u32` at position 1.
    record_mixed("x", 9, "y");

    let log = take_generic_log();
    assert_eq!(log.len(), 3);
    assert_eq!(log[0].as_string(), Some("x".to_string()));
    assert_eq!(log[1].as_f64(), Some(9.0));
    assert_eq!(log[2].as_string(), Some("y".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_multiple_type_params() {
    record_two(1u32, "two");

    let log = take_generic_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].as_f64(), Some(1.0));
    assert_eq!(log[1].as_string(), Some("two".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_generic_return() {
    // Generic argument and generic return marshalled per `T`.
    let s: String = groundtrip("round".to_string());
    assert_eq!(s, "round");

    let n: u32 = groundtrip(123u32);
    assert_eq!(n, 123);
}

#[wasm_bindgen_test]
fn generic_import_return_only() {
    let n: u32 = gget();
    assert_eq!(n, 7);
}

#[wasm_bindgen_test]
fn generic_import_option_arg() {
    record_opt(Some(5u32));
    record_opt::<u32>(None);

    let log = take_generic_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].as_f64(), Some(5.0));
    assert!(log[1].is_undefined() || log[1].is_null());
}

#[wasm_bindgen_test]
fn generic_import_ref_arg() {
    let v = JsValue::from(99u32);
    record_ref(&v);

    let log = take_generic_log();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].as_f64(), Some(99.0));
}

mod dobj {
    pub trait DurableObject {
        type Stub;
    }
}

// Generic method on a *generic* class with an associated-type return:
// exercises class-generic hoisting (`impl<T> DoNamespace<T>`) and a hole
// that is a projection (`<T as DurableObject>::Stub`), not a bare param.
// Compile-only: never instantiated, so no courier is monomorphised.
#[wasm_bindgen]
extern "C" {
    type MyDo;
    type MyDoStub;
    type DoNamespace<T: dobj::DurableObject = MyDo>;

    #[wasm_bindgen(method, generic, js_name = "getByName")]
    fn get_by_name<T: dobj::DurableObject = MyDo>(
        this: &DoNamespace<T>,
        name: &str,
    ) -> <T as dobj::DurableObject>::Stub;
}

impl dobj::DurableObject for MyDo {
    type Stub = MyDoStub;
}

#[wasm_bindgen_test]
fn generic_import_method() {
    let r = Recorder::new();
    r.push_val(7u32);
    assert_eq!(r.last().as_f64(), Some(7.0));
    r.push_val("hi");
    assert_eq!(r.last().as_string(), Some("hi".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_catch() {
    let ok: Result<u32, JsValue> = try_maybe(5u32);
    assert_eq!(ok.unwrap(), 5);

    let err: Result<u32, JsValue> = try_maybe(13u32);
    assert!(err.is_err());
}
