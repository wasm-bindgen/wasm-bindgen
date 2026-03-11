/* @ts-self-types="./reference_test.d.ts" */

export function __wbg_reset_state () {
    __wbg_instance_id++;

    const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
    if (__wbg_reinit_hook !== null) {
        __wbg_reinit_hook(wasm);
    }
}

export function __wbg_set_reinit_hook (callback) {
    if (callback !== null && typeof callback !== 'function') {
        throw new Error('expected function or null, got ' + typeof callback);
    }
    __wbg_reinit_hook = callback;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_random_fcf922927450bad6: function() {
            const ret = Math.random();
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

let __wbg_instance_id = 0;


let __wbg_reinit_hook = null;

import source wasmModule from "./reference_test_bg.wasm";
const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;
wasm.__wbindgen_start();
