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

// Test that unstable IDL overrides use typed Function<fn(...)> parameters.
// The unstable getCurrentPosition takes PositionCallback which is
// `callback PositionCallback = undefined (Position position)`.
// This should generate &Function<fn(Position) -> Undefined>, not &Function.
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_callback_uses_typed_function() {
    use wasm_bindgen::closure::ScopedClosure;

    let geo = GeolocationLike::new().unwrap();

    // from_closure produces Function<fn(Position) -> Undefined>.
    // This only compiles if get_current_position accepts the typed signature.
    let callback =
        js_sys::Function::from_closure(ScopedClosure::<dyn Fn(Position)>::new(|_pos: Position| {}));
    geo.get_current_position(&callback);
}

// Test WebGL pattern: multiple stable overloads where one uses an unstable type.
// Stable overloads that don't use the unstable type should have NO cfg gate.
// They should be available in both stable and unstable modes.
#[wasm_bindgen_test]
fn stable_overloads_not_gated_by_unstable_type_sibling() {
    let gl = WebGlLike::new().unwrap();
    let tex = TextureLike::new().unwrap();

    // These stable overloads should always be available, regardless of unstable flag.
    // They must NOT get cfg(not(web_sys_unstable_apis)) just because a sibling
    // overload (texUpload with UnstableFrame) uses an unstable type.
    let _ = gl.tex_upload_with_x_and_y(&tex, 0i32, 0i32);
    let _ = gl.tex_upload_with_dx_and_dy(&tex, 0.0, 0.0);
    let _ = gl.tex_upload_with_data(&tex, "data");
}

// The unstable-type overload should only be available in unstable mode.
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen_test]
fn unstable_type_overload_available_with_flag() {
    let gl = WebGlLike::new().unwrap();
    let tex = TextureLike::new().unwrap();
    let frame = UnstableFrame::new().unwrap();

    let _ = gl.tex_upload(&tex, &frame);
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
