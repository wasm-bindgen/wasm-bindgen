
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
exports.add_that_might_fail = function(a, b) {
    return wasm.add_that_might_fail(a, b) >>> 0;
};

exports.__wbg_random_9526caf33df4270d = function() {
    return Math.random();
};

const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = exports.__wasm = new WebAssembly.Instance(wasmModule, imports).exports;

