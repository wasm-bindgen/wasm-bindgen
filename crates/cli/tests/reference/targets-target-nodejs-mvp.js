
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}
exports.add_that_might_fail = add_that_might_fail;

exports.__wbg_random_9526caf33df4270d = function() {
    const ret = Math.random();
    return ret;
};

const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = exports.__wasm = new WebAssembly.Instance(wasmModule, imports).exports;
