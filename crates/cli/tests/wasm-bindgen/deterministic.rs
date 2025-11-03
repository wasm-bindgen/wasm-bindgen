//! Test that wasm-bindgen produces deterministic output.

use super::*;

#[test]
fn test_different_signatures_get_different_hashes() {
    let mut project = Project::new("different_signatures_test");
    project
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn closure_no_args() -> js_sys::Function {
                    Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>)
                        .into_js_value()
                        .unchecked_into()
                }

                #[wasm_bindgen]
                pub fn closure_i32_arg() -> js_sys::Function {
                    Closure::wrap(Box::new(|_x: i32| {}) as Box<dyn Fn(i32)>)
                        .into_js_value()
                        .unchecked_into()
                }

                #[wasm_bindgen]
                pub fn closure_string_arg() -> js_sys::Function {
                    Closure::wrap(Box::new(|_s: String| {}) as Box<dyn Fn(String)>)
                        .into_js_value()
                        .unchecked_into()
                }
            "#,
        )
        .dep("js-sys = { path = '{root}/crates/js-sys' }");

    let out_dir = project.wasm_bindgen("--target bundler").unwrap();
    let wasm_path = out_dir.join("different_signatures_test_bg.wasm");
    let wasm_bytes = fs::read(&wasm_path).unwrap();

    let module = walrus::Module::from_buffer(&wasm_bytes).unwrap();

    let exports = module
        .exports
        .iter()
        .filter(|e| e.name.contains("wasm_bindgen__"))
        .map(|x| x.name.clone())
        .collect::<Vec<_>>();

    assert_eq!(
        exports,
        vec![
            "wasm_bindgen__convert__closures_____invoke__h0000000000000000",
            "wasm_bindgen__closure__destroy__h0000000000000000",
            "wasm_bindgen__convert__closures_____invoke__h0000000000000001",
            "wasm_bindgen__closure__destroy__h0000000000000001",
            "wasm_bindgen__convert__closures_____invoke__h0000000000000002",
            "wasm_bindgen__closure__destroy__h0000000000000002",
        ]
    );
}

#[test]
fn test_closure_runtime_works() {
    let out_dir = Project::new("closure_runtime_test")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn create_adder(x: i32) -> js_sys::Function {
                    let closure = Closure::wrap(Box::new(move |y: i32| -> i32 {
                        x + y
                    }) as Box<dyn Fn(i32) -> i32>);
                    let func = closure.as_ref().clone();
                    closure.forget();
                    func.unchecked_into()
                }
            "#,
        )
        .dep("js-sys = { path = '{root}/crates/js-sys' }")
        .wasm_bindgen("--target nodejs")
        .unwrap();

    let test_js = r#"
        const wasm = require('./closure_runtime_test.js');
        const adder = wasm.create_adder(10);
        const result = adder(5);
        console.log(result);
    "#;
    fs::write(out_dir.join("test.js"), test_js).unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout("15\n");
}

#[test]
fn test_user_export_with_hash_suffix_preserved() {
    // A user might legitimately have an export ending in h[0-9a-f]{16}
    let out_dir = Project::new("user_hash_export")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                // This function name legitimately ends with a hash-like pattern
                #[wasm_bindgen]
                pub fn compute_h0123456789abcdef() -> i32 { 42 }
            "#,
        )
        .wasm_bindgen("--target nodejs")
        .unwrap();

    let js = fs::read_to_string(out_dir.join("user_hash_export.js")).unwrap();
    assert!(
        js.contains("compute_h0123456789abcdef"),
        "User export should preserve original name, but got: {}",
        js
    );

    // Run the code to make sure the function is callable
    let test_js = r#"
        const wasm = require('./user_hash_export.js');
        const result = wasm.compute_h0123456789abcdef();
        console.log(result);
    "#;
    fs::write(out_dir.join("test.js"), test_js).unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout("42\n");
}
