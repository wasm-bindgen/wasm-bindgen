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

    fn take_generic_log() -> Vec<JsValue>;
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
