import source wasmModule from "./reference_test_bg.wasm";

let wasm;

let __wbg_instance_id = 0;

export function __wbg_reset_state () {
    __wbg_instance_id++;
    const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
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
