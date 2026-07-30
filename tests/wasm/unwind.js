const wasm = require('wasm-bindgen-test');

// Throws an error - used to test that JS throws trigger Rust unwinding
exports.js_throw_error = () => {
  throw new Error('JS throw for unwind test');
};

// Check if drop ran (reads from global set by Rust)
exports.js_check_dropped = () => {
  return globalThis.unwind_drop_ran === true;
};

// Reset the drop flag
exports.js_reset_dropped = () => {
  globalThis.unwind_drop_ran = false;
  globalThis.unwind_continued_after_throw = false;
};

// Trigger the unwind test by calling the Rust function
// This catches the error so we can verify it propagated
exports.js_trigger_unwind_test = () => {
  wasm.rust_call_throwing_js();
};

// How far `__stack_pointer` moves across `n` calls to `f`, each expected to
// throw. Both readings are taken from JS, with no wasm frame live: an
// enclosing wasm frame restores the pointer to the value it saved on entry
// when it returns, which would erase the drift before Rust could observe it.
function spDrift(f, n) {
  const before = wasm.sp_now();
  let caught = 0;
  for (let i = 0; i < n; i++) {
    try {
      f();
    } catch (error) {
      if (error.name !== 'PanicError') {
        throw error;
      }
      caught++;
    }
  }
  if (caught !== n) {
    throw new Error(`expected ${n} panics to escape, got ${caught}`);
  }
  return before - wasm.sp_now();
}

exports.js_closure_sp_drift = (f, n) => spDrift(() => f(), n);

exports.js_export_sp_drift = (n) => spDrift(() => wasm.sp_leak_panic(), n);
