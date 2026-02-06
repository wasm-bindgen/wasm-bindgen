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
