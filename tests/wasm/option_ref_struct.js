const wasm = require("wasm-bindgen-test");

exports.js_call_rust_with_some = function () {
  const s = new wasm.OptRefTestStruct(42);
  return wasm.rust_receive_option_ref(s);
};

exports.js_call_rust_with_none = function () {
  return wasm.rust_receive_option_ref(undefined);
};
