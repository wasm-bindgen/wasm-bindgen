//! Tests for the `--abort-reinit` and `--experimental-reset-state-function` CLI flags.
//!
//! These tests verify the runtime behavior of the panic re-initialization feature:
//! - When a Wasm panic occurs, the module enters an "aborted" state
//! - Subsequent calls automatically re-initialize the module
//! - Objects and closures from previous instances are detected as stale
//! - With `panic=unwind`, `PanicError` does NOT trigger re-initialization

use crate::{Project, REPO_ROOT};
use assert_cmd::Command;
use std::fs;

/// Helper to create a project configured for panic=unwind
fn project_panic_unwind(name: &str) -> Project {
    let mut project = Project::new(name);
    project.file(
        "Cargo.toml",
        &format!(
            "
            [package]
            name = \"{name}\"
            version = \"1.0.0\"
            edition = \"2021\"

            [dependencies]
            wasm-bindgen = {{ path = '{root}' }}
            js-sys = {{ path = '{root}/crates/js-sys' }}

            [lib]
            crate-type = ['cdylib']

            [workspace]

            [profile.dev]
            panic = \"unwind\"
            ",
            root = REPO_ROOT.display()
        ),
    );
    project
}

/// Test: With panic=unwind, PanicError does NOT trigger re-initialization
#[test]
fn unwind_panic_no_reinit() {
    let name = "unwind_panic_no_reinit";
    let mut project = project_panic_unwind(name);
    project.file(
        "src/lib.rs",
        "
            use wasm_bindgen::prelude::*;
            use std::sync::atomic::{AtomicU32, Ordering};

            static COUNTER: AtomicU32 = AtomicU32::new(0);

            #[wasm_bindgen]
            pub fn increment_and_get() -> u32 {
                COUNTER.fetch_add(1, Ordering::SeqCst)
            }

            #[wasm_bindgen]
            pub fn trigger_panic() {
                panic!();
            }
            ",
    );

    // Build with nightly and -Zbuild-std for panic=unwind support
    project
        .cargo_cmd
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --abort-reinit")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./unwind_panic_no_reinit.js');

        // Increment counter
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);

        // Trigger panic - with unwind, this throws PanicError
        let caughtPanic = false;
        try {
            wasm.trigger_panic();
        } catch (e) {
            caughtPanic = true;
            // Should be a PanicError, not a RuntimeError
            assert.strictEqual(e.name, 'PanicError', `Expected PanicError, got: ${e.name}`);
        }
        assert(caughtPanic, 'Expected panic to throw');

        // With panic=unwind, the module should NOT be re-initialized
        // Counter should continue from where it was
        assert.strictEqual(wasm.increment_and_get(), 2, 'Counter should NOT be reset with panic=unwind');
        assert.strictEqual(wasm.increment_and_get(), 3);

        console.log('unwind_panic_no_reinit: PASSED');
        ",
    )
    .unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("PASSED"));
}

/// Test: With panic=unwind, non-PanicError exceptions DO trigger re-initialization.
#[test]
fn unwind_js_exception_reinit() {
    let name = "unwind_js_exception_reinit";
    let mut project = project_panic_unwind(name);
    project.file(
        "src/lib.rs",
        "
            use wasm_bindgen::prelude::*;
            use std::sync::atomic::{AtomicU32, Ordering};

            static COUNTER: AtomicU32 = AtomicU32::new(0);

            #[wasm_bindgen]
            pub fn increment_and_get() -> u32 {
                COUNTER.fetch_add(1, Ordering::SeqCst)
            }

            #[wasm_bindgen]
            extern \"C\" {
                #[wasm_bindgen(js_name = throwError)]
                fn throw_error();
            }

            #[wasm_bindgen]
            pub fn call_throwing_js() {
                throw_error();
            }
            ",
    );

    project
        .cargo_cmd
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --abort-reinit")
        .unwrap();

    // Provide the JS function that throws
    fs::write(
        out_dir.join("setup.js"),
        "
        // This must be called before requiring the wasm module
        global.throwError = function() {
            throw new Error('JS exception');
        };
        ",
    )
    .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');

        // Setup global throwing function first
        require('./setup.js');

        const wasm = require('./unwind_js_exception_reinit.js');

        // Increment counter
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);

        // Trigger JS exception (not a Rust panic)
        let caughtError = false;
        try {
            wasm.call_throwing_js();
        } catch (e) {
            caughtError = true;
            // Should be a regular Error, not PanicError
            assert.strictEqual(e.name, 'Error', `Expected Error, got: ${e.name}`);
        }
        assert(caughtError, 'Expected JS exception to throw');

        // With a JS exception (not PanicError), the module SHOULD be re-initialized
        // Counter should be reset to 0
        assert.strictEqual(wasm.increment_and_get(), 0, 'Counter should be reset after JS exception');

        console.log('unwind_js_exception_reinit: PASSED');
        ",
    )
    .unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("PASSED"));
}

/// Test: Striped call stacks that cross the FFI boundary multiple times.
/// Tests Rust -> JS -> Rust -> panic, ensuring unwinding works across FFI boundaries.
#[test]
fn unwind_striped_call_stack() {
    let name = "unwind_striped_call_stack";
    let mut project = project_panic_unwind(name);
    project.file(
        "src/lib.rs",
        "
            use wasm_bindgen::prelude::*;
            use std::sync::atomic::{AtomicU32, Ordering};

            static COUNTER: AtomicU32 = AtomicU32::new(0);
            static DEPTH: AtomicU32 = AtomicU32::new(0);

            #[wasm_bindgen]
            extern \"C\" {
                #[wasm_bindgen(js_name = jsCallbackDepth1)]
                fn js_callback_depth_1();
                #[wasm_bindgen(js_name = jsCallbackDepth2)]
                fn js_callback_depth_2();
            }

            #[wasm_bindgen]
            pub fn increment_and_get() -> u32 {
                COUNTER.fetch_add(1, Ordering::SeqCst)
            }

            /// Entry point: Rust -> JS (depth 1)
            #[wasm_bindgen]
            pub fn start_striped_call() {
                DEPTH.store(0, Ordering::SeqCst);
                js_callback_depth_1();
            }

            /// Called from JS (depth 1) -> Rust -> JS (depth 2)
            #[wasm_bindgen]
            pub fn rust_callback_depth_1() {
                DEPTH.fetch_add(1, Ordering::SeqCst);
                js_callback_depth_2();
            }

            /// Called from JS (depth 2) -> Rust -> panic
            #[wasm_bindgen]
            pub fn rust_callback_depth_2() {
                DEPTH.fetch_add(1, Ordering::SeqCst);
                panic!(\"panic at depth 2\");
            }

            #[wasm_bindgen]
            pub fn get_depth() -> u32 {
                DEPTH.load(Ordering::SeqCst)
            }
            ",
    );

    project
        .cargo_cmd
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --abort-reinit")
        .unwrap();

    // Setup JS callbacks that call back into Rust
    fs::write(
        out_dir.join("setup.js"),
        "
        // Depth 1: JS calls back into Rust
        global.jsCallbackDepth1 = function() {
            const wasm = require('./unwind_striped_call_stack.js');
            wasm.rust_callback_depth_1();
        };

        // Depth 2: JS calls back into Rust (which will panic)
        global.jsCallbackDepth2 = function() {
            const wasm = require('./unwind_striped_call_stack.js');
            wasm.rust_callback_depth_2();
        };
        ",
    )
    .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');

        // Setup callbacks first
        require('./setup.js');

        const wasm = require('./unwind_striped_call_stack.js');

        // Increment counter to establish state
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);

        // Start the striped call stack: Rust -> JS -> Rust -> JS -> Rust -> panic
        let caughtPanic = false;
        try {
            wasm.start_striped_call();
        } catch (e) {
            caughtPanic = true;
            // Should be a PanicError with the message
            assert.strictEqual(e.name, 'PanicError', `Expected PanicError, got: ${e.name}`);
            assert(e.message.includes('panic at depth 2'), `Expected panic message, got: ${e.message}`);
        }
        assert(caughtPanic, 'Expected panic to be thrown');

        // With panic=unwind and successful unwinding across FFI, module should NOT be re-initialized
        // Counter should continue from where it was
        assert.strictEqual(wasm.increment_and_get(), 2, 'Counter should NOT be reset after successful unwind');

        console.log('unwind_striped_call_stack: PASSED');
        ",
    )
    .unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("PASSED"));
}

/// Test: Panic during unwinding (double panic) should cause an abort and trigger re-initialization.
#[test]
fn unwind_double_panic_abort() {
    let name = "unwind_double_panic_abort";
    let mut project = project_panic_unwind(name);
    project.file(
        "src/lib.rs",
        "
            use wasm_bindgen::prelude::*;
            use std::sync::atomic::{AtomicU32, Ordering};

            static COUNTER: AtomicU32 = AtomicU32::new(0);

            struct PanicOnDrop;

            impl Drop for PanicOnDrop {
                fn drop(&mut self) {
                    // This panic during unwinding will cause an abort
                    panic!(\"panic during drop\");
                }
            }

            #[wasm_bindgen]
            pub fn increment_and_get() -> u32 {
                COUNTER.fetch_add(1, Ordering::SeqCst)
            }

            #[wasm_bindgen]
            pub fn trigger_double_panic() {
                let _guard = PanicOnDrop;
                panic!(\"first panic\");
                // When unwinding from 'first panic', PanicOnDrop::drop will panic,
                // causing an abort (double panic)
            }
            ",
    );

    project
        .cargo_cmd
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --abort-reinit")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./unwind_double_panic_abort.js');

        // Increment counter to establish state
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);

        // Trigger double panic - this should abort (not a recoverable unwind)
        let caughtError = false;
        try {
            wasm.trigger_double_panic();
        } catch (e) {
            caughtError = true;
            // Double panic causes abort, which should be a RuntimeError (not PanicError)
            // because the abort is not a normal panic unwind
            assert(
                e.name === 'RuntimeError' || e.message.includes('unreachable'),
                `Expected RuntimeError or unreachable, got: ${e.name}: ${e.message}`
            );
        }
        assert(caughtError, 'Expected double panic to throw');

        // With abort (double panic), the module SHOULD be re-initialized
        // Counter should be reset to 0
        assert.strictEqual(wasm.increment_and_get(), 0, 'Counter should be reset after double panic abort');
        assert.strictEqual(wasm.increment_and_get(), 1);

        console.log('unwind_double_panic_abort: PASSED');
        ",
    )
    .unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("PASSED"));
}

/// Test: Manual abort via std::process::abort() should trigger re-initialization.
#[test]
fn unwind_manual_abort() {
    let name = "unwind_manual_abort";
    let mut project = project_panic_unwind(name);

    project.file(
        "src/lib.rs",
        "
            use wasm_bindgen::prelude::*;
            use std::sync::atomic::{AtomicU32, Ordering};

            static COUNTER: AtomicU32 = AtomicU32::new(0);

            #[wasm_bindgen]
            pub fn increment_and_get() -> u32 {
                COUNTER.fetch_add(1, Ordering::SeqCst)
            }

            #[wasm_bindgen]
            pub fn trigger_manual_abort() {
                // std::process::abort() triggers an unreachable instruction in wasm
                std::process::abort();
            }
            ",
    );

    project
        .cargo_cmd
        .env("RUSTFLAGS", "-Cpanic=unwind")
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_unwind");

    let out_dir = project
        .wasm_bindgen("--target nodejs --abort-reinit")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./unwind_manual_abort.js');

        // Increment counter to establish state
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);

        // Trigger manual abort
        let caughtError = false;
        try {
            wasm.trigger_manual_abort();
        } catch (e) {
            caughtError = true;
            // Manual abort causes RuntimeError with unreachable
            assert(
                e.name === 'RuntimeError',
                `Expected RuntimeError, got: ${e.name}`
            );
            assert(
                e.message.includes('unreachable'),
                `Expected unreachable message, got: ${e.message}`
            );
        }
        assert(caughtError, 'Expected manual abort to throw');

        // With manual abort, the module SHOULD be re-initialized
        // Counter should be reset to 0
        assert.strictEqual(wasm.increment_and_get(), 0, 'Counter should be reset after manual abort');
        assert.strictEqual(wasm.increment_and_get(), 1);

        console.log('unwind_manual_abort: PASSED');
        ",
    )
    .unwrap();

    Command::new("node")
        .arg("test.js")
        .current_dir(&out_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("PASSED"));
}
