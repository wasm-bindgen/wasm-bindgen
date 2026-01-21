//! Tests for the `--experimental-abort-reinit` and `--experimental-reset-state-function` CLI flags.
//!
//! These tests verify the runtime behavior of the panic re-initialization feature:
//! - When a Wasm panic occurs, the module enters an "aborted" state
//! - Subsequent calls automatically re-initialize the module
//! - Objects and closures from previous instances are detected as stale
//! - With `panic=unwind`, `PanicError` does NOT trigger re-initialization

use crate::{Project, REPO_ROOT};
use assert_cmd::Command;
use std::fs;

/// Helper to create a project configured for panic=abort
fn project_panic_abort(name: &str) -> Project {
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
            panic = \"abort\"
            ",
            root = REPO_ROOT.display()
        ),
    );
    project
}

/// Test: Basic panic triggers abort state, next call re-initializes
#[test]
fn abort_reinit_basic() {
    let mut project = project_panic_abort("abort_reinit_basic");

    project.file(
        "src/lib.rs",
        "
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn maybe_panic(should_panic: bool) -> u32 {
            if should_panic {
                panic!(\"test panic\");
            }
            42
        }

        #[wasm_bindgen]
        pub fn simple_add(a: u32, b: u32) -> u32 {
            a + b
        }
        ",
    );

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-abort-reinit")
        .unwrap();

    // Write JS test file
    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./abort_reinit_basic.js');

        // Normal operation works
        assert.strictEqual(wasm.simple_add(1, 2), 3);
        assert.strictEqual(wasm.maybe_panic(false), 42);

        // Trigger panic - should throw
        let panicked = false;
        try {
            wasm.maybe_panic(true);
        } catch (e) {
            panicked = true;
            // Verify it's a RuntimeError from Wasm
            assert(e instanceof WebAssembly.RuntimeError || e.message.includes('unreachable'), 
                   `Expected RuntimeError, got: ${e}`);
        }
        assert(panicked, 'Expected panic to throw');

        // After panic, next call should work (module was re-initialized)
        assert.strictEqual(wasm.simple_add(1, 2), 3, 'Module should work after re-init');
        assert.strictEqual(wasm.maybe_panic(false), 42, 'Module should work after re-init');

        console.log('abort_reinit_basic: PASSED');
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

/// Test: Rust static state is reset after re-initialization
#[test]
fn abort_reinit_state_reset() {
    let mut project = project_panic_abort("abort_reinit_state_reset");

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

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-abort-reinit")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./abort_reinit_state_reset.js');

        // Increment counter a few times
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);
        assert.strictEqual(wasm.increment_and_get(), 2);

        // Trigger panic
        try {
            wasm.trigger_panic();
        } catch (e) {
            // Expected
        }

        // After re-init, counter should be back to 0
        assert.strictEqual(wasm.increment_and_get(), 0, 'Counter should be reset after re-init');
        assert.strictEqual(wasm.increment_and_get(), 1);

        console.log('abort_reinit_state_reset: PASSED');
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

/// Test: Objects from previous Wasm instance are detected as stale
#[test]
fn abort_reinit_stale_object() {
    let mut project = project_panic_abort("abort_reinit_stale_object");

    project.file(
        "src/lib.rs",
        "
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub struct Counter {
            value: u32,
        }

        #[wasm_bindgen]
        impl Counter {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Counter {
                Counter { value: 0 }
            }

            pub fn increment(&mut self) -> u32 {
                self.value += 1;
                self.value
            }

            pub fn get(&self) -> u32 {
                self.value
            }
        }

        #[wasm_bindgen]
        pub fn trigger_panic() {
            panic!();
        }
        ",
    );

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-abort-reinit")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./abort_reinit_stale_object.js');

        // Create an object before panic
        const counter = new wasm.Counter();
        assert.strictEqual(counter.increment(), 1);
        assert.strictEqual(counter.increment(), 2);

        // Trigger panic to force re-initialization
        try {
            wasm.trigger_panic();
        } catch (e) {
            // Expected
        }

        // Force module re-init by calling any function
        // (the re-init happens on next call after panic)

        // Using stale object should throw
        let threwStaleError = false;
        try {
            counter.increment();
        } catch (e) {
            threwStaleError = true;
            assert(e.message.includes('stale') || e.message.includes('previous'), 
                   `Expected stale object error, got: ${e.message}`);
        }
        assert(threwStaleError, 'Expected stale object to throw');

        // New objects should work fine
        const newCounter = new wasm.Counter();
        assert.strictEqual(newCounter.increment(), 1);
        assert.strictEqual(newCounter.get(), 1);
        newCounter.free();

        console.log('abort_reinit_stale_object: PASSED');
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

/// Test: Closures from previous Wasm instance are detected as stale
#[test]
fn abort_reinit_stale_closure() {
    let mut project = project_panic_abort("abort_reinit_stale_closure");

    project.file(
        "src/lib.rs",
        "
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn create_adder(base: u32) -> js_sys::Function {
            let closure = Closure::wrap(Box::new(move |x: u32| -> u32 {
                base + x
            }) as Box<dyn Fn(u32) -> u32>);
            let func = closure.as_ref().clone();
            closure.forget(); // Leak to keep alive
            func.into()
        }

        #[wasm_bindgen]
        pub fn trigger_panic() {
            panic!();
        }

        #[wasm_bindgen]
        pub fn simple_op() -> u32 {
            123
        }
        ",
    );

    project.dep("js-sys = { path = '{root}/crates/js-sys' }");

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-abort-reinit")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./abort_reinit_stale_closure.js');

        // Create a closure before panic
        const adder = wasm.create_adder(10);
        assert.strictEqual(adder(5), 15);
        assert.strictEqual(adder(20), 30);

        // Trigger panic
        try {
            wasm.trigger_panic();
        } catch (e) {
            // Expected
        }

        // Force re-init by calling something
        wasm.simple_op();

        // Using stale closure should throw
        let threwStaleError = false;
        try {
            adder(5);
        } catch (e) {
            threwStaleError = true;
            assert(e.message.includes('closure') || e.message.includes('previous') || e.message.includes('stale'),
                   `Expected stale closure error, got: ${e.message}`);
        }
        assert(threwStaleError, 'Expected stale closure to throw');

        // New closures should work fine
        const newAdder = wasm.create_adder(100);
        assert.strictEqual(newAdder(1), 101);

        console.log('abort_reinit_stale_closure: PASSED');
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

/// Test: Manual __wbg_reset_state() function works
#[test]
fn reset_state_manual() {
    let mut project = project_panic_abort("reset_state_manual");

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
        ",
    );

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-reset-state-function")
        .unwrap();

    fs::write(
        out_dir.join("test.js"),
        "
        const assert = require('assert');
        const wasm = require('./reset_state_manual.js');

        // Increment counter
        assert.strictEqual(wasm.increment_and_get(), 0);
        assert.strictEqual(wasm.increment_and_get(), 1);
        assert.strictEqual(wasm.increment_and_get(), 2);

        // Manually reset state (no panic needed)
        wasm.__wbg_reset_state();

        // Counter should be reset
        assert.strictEqual(wasm.increment_and_get(), 0, 'Counter should be reset after manual reset');
        assert.strictEqual(wasm.increment_and_get(), 1);

        console.log('reset_state_manual: PASSED');
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

/// Test: With panic=unwind, PanicError does NOT trigger re-initialization,
/// but other exceptions DO trigger re-initialization.
///
/// This test requires nightly Rust with -Zbuild-std for panic=unwind on wasm32.
#[test]
#[ignore = "requires nightly Rust with -Zbuild-std"]
fn unwind_panic_no_reinit() {
    let name = "unwind_panic_no_reinit";
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

            [lib]
            crate-type = ['cdylib']

            [workspace]

            [profile.dev]
            panic = \"unwind\"
            ",
            root = REPO_ROOT.display()
        ),
    );

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
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_abort");

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-abort-reinit")
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
#[ignore = "requires nightly Rust with -Zbuild-std"]
fn unwind_js_exception_reinit() {
    let name = "unwind_js_exception_reinit";
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

            [lib]
            crate-type = ['cdylib']

            [workspace]

            [profile.dev]
            panic = \"unwind\"
            ",
            root = REPO_ROOT.display()
        ),
    );

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
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("-Zbuild-std=std,panic_abort");

    let out_dir = project
        .wasm_bindgen("--target nodejs --experimental-abort-reinit")
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
