//! @ts-gen --lib-name node:console ../ts-gen/tests/fixtures/node-console.d.ts
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use ts_gen_integration_tests::console::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_namespace = process, js_name = "stdout")]
    static STDOUT: js_sys::Object;
    #[wasm_bindgen(thread_local_v2, js_namespace = process, js_name = "stderr")]
    static STDERR: js_sys::Object;
}

fn stdout() -> js_sys::Object {
    STDOUT.with(|s| s.clone())
}

fn stderr() -> js_sys::Object {
    STDERR.with(|s| s.clone())
}

/// Helper to create a Console backed by process.stdout.
fn make_console() -> Console {
    Console::new(&stdout()).unwrap()
}

#[wasm_bindgen_test]
fn test_console_log() {
    let console = make_console();

    // Variadic always takes a slice — pass empty for no args
    console.log(&[]);
    console.log(&[JsValue::from("hello"), JsValue::from(42)]);
}

#[wasm_bindgen_test]
fn test_console_warn_error_debug_info() {
    let console = make_console();

    console.warn(&[JsValue::from("warning!")]);
    console.error(&[JsValue::from("error!")]);
    console.debug(&[JsValue::from("debug!")]);
    console.info(&[JsValue::from("info!")]);
}

#[wasm_bindgen_test]
fn test_console_count() {
    let console = make_console();

    // No-arg: uses default label
    console.count();

    // With label
    console.count_with_label("my-counter");

    // Reset
    console.count_reset();
    console.count_reset_with_label("my-counter");
}

#[wasm_bindgen_test]
fn test_console_time() {
    let console = make_console();

    console.time();
    console.time_with_label("my-timer");
    console.time_end();
    console.time_end_with_label("my-timer");
}

#[wasm_bindgen_test]
fn test_console_time_log() {
    let console = make_console();

    // timeLog has optional label + variadic data (always present)
    console.time_log(&[]);
    console.time_log_with_label("my-timer", &[JsValue::from("extra data")]);
}

#[wasm_bindgen_test]
fn test_console_group() {
    let console = make_console();

    console.group(&[]);
    console.group(&[JsValue::from("group label")]);
    console.group_end();

    console.group_collapsed(&[]);
    console.group_collapsed(&[JsValue::from("collapsed")]);
    console.group_end();
}

#[wasm_bindgen_test]
fn test_console_assert() {
    let console = make_console();

    // assert: optional condition + variadic data
    console.assert(&[]);
    console.assert_with_condition(true, &[]);
    console.assert_with_condition(false, &[JsValue::from("assertion failed")]);
}

#[wasm_bindgen_test]
fn test_console_clear() {
    let console = make_console();
    console.clear();
}

#[wasm_bindgen_test]
fn test_console_trace() {
    let console = make_console();
    console.trace(&[]);
    console.trace(&[JsValue::from("stack trace")]);
}

#[wasm_bindgen_test]
fn test_console_table() {
    let console = make_console();

    // No-arg
    console.table();

    // With data
    console.table_with_tabular_data(&JsValue::from(js_sys::Array::new()));
}

// Note: profile(), profileEnd(), and timeStamp() are inspector-only APIs
// that don't exist on Console instances created via `new Console(stdout)`.
// The bindings are generated and compile-checked, but we skip runtime tests.

#[wasm_bindgen_test]
fn test_try_variants() {
    let console = make_console();

    // try_ variants return Result
    console.try_log(&[JsValue::from("try")]).unwrap();
    console.try_count().unwrap();
    console.try_count_with_label("try-counter").unwrap();
    console.try_clear().unwrap();
}

#[wasm_bindgen_test]
fn test_constructor_overloads() {
    let _c1 = Console::new(&stdout()).unwrap();
    let _c2 = Console::new_with_stderr(&stdout(), &stderr()).unwrap();
    let _c3 = Console::new_with_stderr_and_ignore_errors(&stdout(), &stderr(), true).unwrap();
}
