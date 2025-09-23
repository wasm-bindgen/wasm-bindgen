
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
module.exports.add_that_might_fail = function(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
};

module.exports.__wbg_random_8be0a899673d8681 = function() {
    const ret = Math.random();
    return ret;
};

module.exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

const path = require('path').join(__dirname, 'reference_test_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

wasm.__wbindgen_start();

