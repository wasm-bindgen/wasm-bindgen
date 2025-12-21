/* @ts-self-types="./reference_test.d.ts" */
//#region exports

export function __wbg_reset_state () {
    __wbg_instance_id++;
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
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}
//#endregion

//#region wasm imports

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
//#endregion

let __wbg_instance_id = 0;


//#region wasm loading
import source wasmModule from "./reference_test_bg.wasm";
const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;

//#endregion

