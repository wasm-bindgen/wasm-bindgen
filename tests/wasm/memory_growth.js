const wasm = require("wasm-bindgen-test");

// Get the current WebAssembly memory buffer size in bytes
exports.get_memory_byte_length = function () {
  return wasm.__wasm.memory.buffer.byteLength;
};
