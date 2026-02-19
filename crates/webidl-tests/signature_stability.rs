// This tests that a stable interface method can have an unstable overload added
// via a partial interface in the unstable directory.

use crate::generated::*;
use wasm_bindgen_test::*;

#[cfg(not(web_sys_unstable_apis))]
#[wasm_bindgen_test]
fn stable_method_is_available() {
    let sig = SignatureStability::new().unwrap();
    let result = sig.process();
    assert_eq!(result, "stable");
}

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_overload_is_available_with_flag() {
    let sig = SignatureStability::new().unwrap();

    let options = SignatureStabilityOptions::new();
    options.set_mode(SignatureStabilityMode::Fast);

    // With authoritative expansion model, the unstable process(options)
    // overrides the stable process(). Both are named `process` within their
    // respective sets, and are cfg-gated alternatives.
    let result = sig.process(&options);
    assert_eq!(result, "fast");
}

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_types_available_with_flag() {
    let _mode = SignatureStabilityMode::Safe;
    let _options = SignatureStabilityOptions::new();
}

// Test that the stable mixin method retains its original name when an unstable
// override exists with different parameter types. This is the critical test -
// if the fix is not applied, the method would be renamed to
// put_image_data_with_f64_and_f64 which would break existing code.
#[cfg(not(web_sys_unstable_apis))]
#[wasm_bindgen_test]
fn stable_mixin_method_has_original_name() {
    let canvas = CanvasLike::new().unwrap();
    let image_data = ImageDataLike::new().unwrap();

    // The method should be called put_image_data, NOT put_image_data_with_f64_and_f64
    canvas.put_image_data(&image_data, 0.0, 0.0);
}

// Test that the unstable mixin method uses the correct (spec) types
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_mixin_method_uses_correct_types() {
    let canvas = CanvasLike::new().unwrap();
    let image_data = ImageDataLike::new().unwrap();

    // The unstable method uses i32 parameters (spec-correct)
    canvas.put_image_data(&image_data, 0i32, 0i32);
}

// Test identical Rust signatures with different attributes ([Throws] vs no Throws).
// Stable version returns Result, unstable returns plain ().
#[cfg(not(web_sys_unstable_apis))]
#[wasm_bindgen_test]
fn stable_throws_method_returns_result() {
    let geo = GeolocationLike::new().unwrap();
    let callback = js_sys::Function::new_no_args("");

    // Stable version has [Throws], so it returns Result<(), JsValue>
    let result: Result<(), wasm_bindgen::JsValue> = geo.get_current_position(&callback);
    let _ = result;
}

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_no_throws_method_returns_unit() {
    use wasm_bindgen::closure::ScopedClosure;

    let geo = GeolocationLike::new().unwrap();

    // Unstable version has no [Throws], so it returns ()
    let callback =
        js_sys::Function::from_closure(ScopedClosure::<dyn Fn(Position)>::new(|_pos: Position| {}));
    let result: () = geo.get_current_position(&callback);
    let _ = result;
}

// Test that read() without arguments is available in stable mode.
// The optional param's type (UnstableOptions) is unstable,
// so stable only gets read() with no args.
#[cfg(not(web_sys_unstable_apis))]
#[wasm_bindgen_test]
fn stable_read_without_args_is_available() {
    let obj = TestOptionalUnstableArg::new().unwrap();
    // read() should be available without unstable flag
    let _promise = obj.read();
}

// Test that read() without arguments is NOT cfg-gated as having an
// unstable override, even though read_with_unstable_options() is unstable.
// These are different signatures so read() should always be available.
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_read_both_signatures_available() {
    let obj = TestOptionalUnstableArg::new().unwrap();
    // read() should be available (not gated behind not(web_sys_unstable_apis))
    let _promise = obj.read();
    // read_with_options() should also be available
    let options = UnstableOptions::new();
    let _promise = obj.read_with_options(&options);
}
