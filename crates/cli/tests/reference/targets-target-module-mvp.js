import source wasmModule from "./reference_test_bg.wasm";

let wasm;

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}

const imports = {
    __wbindgen_placeholder__: {
        __wbg_random_9526caf33df4270d: function() {
            const ret = Math.random();
            return ret;
        },
    },

};

const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
