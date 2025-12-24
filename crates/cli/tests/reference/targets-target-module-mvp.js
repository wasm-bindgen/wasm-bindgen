/* @ts-self-types="./reference_test.d.ts" */
//#region exports

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


//#region wasm loading
import source wasmModule from "./reference_test_bg.wasm";
const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;

//#endregion

