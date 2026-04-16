/* @ts-self-types="./reference_test.d.ts" */

function __wbg_reset_state () {
    __wbg_instance_id++;
    __wbg_reinit_scheduled = false;
    wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
}
exports.__wbg_reset_state = __wbg_reset_state;

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add_that_might_fail(a, b) {
    let ret;
    __wbg_call_guard();
    ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}
exports.add_that_might_fail = add_that_might_fail;
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_random_c82d91f28994c195: function() {
            const ret = Math.random();
            return ret;
        },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

function __wbg_call_guard() {
    if (__wbg_reinit_scheduled) {
        __wbg_reset_state();
        return;
    }
}


let __wbg_instance_id = 0;

let __wbg_reinit_scheduled = false;

const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;
