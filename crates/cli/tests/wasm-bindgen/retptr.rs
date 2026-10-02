use crate::Project;
use assert_cmd::Command;
use std::fs;

#[test]
fn shared_worker_high_address_retptr() {
    let mut project = Project::new("shared_worker_high_address_retptr");
    project.cargo_cmd.arg("-Zbuild-std=std,panic_abort").env(
        "RUSTFLAGS",
        "-Ctarget-feature=+atomics,+bulk-memory,-multivalue \
         -Clink-arg=--shared-memory \
         -Clink-arg=--max-memory=4294967296 \
         -Clink-arg=--import-memory \
         -Clink-arg=--export=__wasm_init_tls \
         -Clink-arg=--export=__tls_size \
         -Clink-arg=--export=__tls_align \
         -Clink-arg=--export=__tls_base",
    );
    project.file(
        "src/lib.rs",
        r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn reserve_low_memory() {
            std::mem::forget(Vec::<u8>::with_capacity(0x7fff_ffff));
        }

        #[wasm_bindgen]
        pub fn stack_address() -> usize {
            let local = 0u8;
            &local as *const u8 as usize
        }

        #[wasm_bindgen]
        pub fn number(value: &JsValue) -> Option<f64> {
            value.as_f64()
        }

        #[wasm_bindgen]
        pub fn string(value: &JsValue) -> Option<String> {
            value.as_string()
        }
        "#,
    );
    let output = project.wasm_bindgen("--target nodejs").unwrap();
    fs::write(
        output.join("test.cjs"),
        r#"
        const assert = require('node:assert/strict');
        const { Worker, isMainThread, workerData } = require('node:worker_threads');
        const bindings = require('./shared_worker_high_address_retptr.js');

        function check() {
            assert.equal(bindings.number(42.5), 42.5);
            assert.equal(bindings.number('not a number'), undefined);
            assert.equal(bindings.string('high stack 🦀'), 'high stack 🦀');
            assert.equal(bindings.string(42.5), undefined);
        }

        if (isMainThread) {
            assert.ok(bindings.stack_address() < 0x80000000);
            check();
            bindings.reserve_low_memory();
            const worker = new Worker(__filename, {
                workerData: {
                    module: bindings.__wbg_wasm_module,
                    memory: bindings.__wbg_memory,
                },
            });
            worker.on('error', error => { throw error; });
            worker.on('exit', code => assert.equal(code, 0));
        } else {
            bindings.initSync(workerData);
            assert.ok(bindings.stack_address() >= 0x80000000);
            check();
        }
        "#,
    )
    .unwrap();
    Command::new("node")
        .arg("test.cjs")
        .current_dir(output)
        .assert()
        .success();
}

#[test]
fn memory64_retptr() {
    let mut project = Project::new("memory64_retptr");
    project.target("wasm64-unknown-unknown");
    project.cargo_cmd.arg("-Zbuild-std=std,panic_abort");
    project.file(
        "src/lib.rs",
        r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn number(value: &JsValue) -> Option<f64> { value.as_f64() }

        #[wasm_bindgen]
        pub fn string(value: &JsValue) -> Option<String> { value.as_string() }
        "#,
    );
    let output = project.wasm_bindgen("--target nodejs").unwrap();
    let js = fs::read_to_string(output.join("memory64_retptr.js")).unwrap();
    assert!(js.contains("Number(arg0) +"));
    assert!(!js.contains("(arg0 >>> 0) +"));
}
