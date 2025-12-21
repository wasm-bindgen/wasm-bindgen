/* @ts-self-types="./reference_test.d.ts" */
//#region exports

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
const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = new WebAssembly.Instance(wasmModule, __wbg_get_imports()).exports;

//#endregion

