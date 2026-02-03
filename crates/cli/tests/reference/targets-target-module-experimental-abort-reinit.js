/* @ts-self-types="./reference_test.d.ts" */

export function __wbg_reset_state () {
    __wbg_instance_id++;
    __wbg_aborted = false;

    const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    let ret;
    if (__wbg_aborted === true) {
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

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_random_ae0b2256206ad108: function() {
            let ret;
            if (__wbg_aborted === true) {
                __wbg_reset_state();
            }
            try {
                ret = Math.random();
            } catch(e) {
                __wbg_aborted = true;
                throw e;
            }
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            if (__wbg_aborted === true) {
                __wbg_reset_state();
            }
            try {
                const table = wasm.__wbindgen_externrefs;
                const offset = table.grow(4);
                table.set(0, undefined);
                table.set(offset + 0, undefined);
                table.set(offset + 1, null);
                table.set(offset + 2, true);
                table.set(offset + 3, false)
            } catch(e) {
                __wbg_aborted = true;
                throw e;
            }
        },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

let __wbg_instance_id = 0;


let __wbg_aborted = false;

import source wasmModule from "./reference_test_bg.wasm";
const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;
wasm.__wbindgen_start();
