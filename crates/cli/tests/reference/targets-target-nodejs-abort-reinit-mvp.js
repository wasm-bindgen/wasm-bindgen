/* @ts-self-types="./reference_test.d.ts" */

function __wbg_reset_state () {
    wasm.__wbindgen_set_abort_flag(1);
    __wbg_instance_id++;
    __wbg_aborted = false;

    const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
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
    if (__wbg_aborted) {
        __wbg_reset_state();
    }
    try {
        ret = wasm.add_that_might_fail(a, b);
    } catch(e) {
        __wbg_aborted = true;
        throw e;
    }
    return ret >>> 0;
}
exports.add_that_might_fail = add_that_might_fail;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_random_9526caf33df4270d: function() {
            const ret = Math.random();
            return ret;
        },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

let __wbg_instance_id = 0;


let __wbg_aborted = false;

const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
let wasm = new WebAssembly.Instance(wasmModule, __wbg_get_imports()).exports;
