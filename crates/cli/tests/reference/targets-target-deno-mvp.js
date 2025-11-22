
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

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasm = (await WebAssembly.instantiateStreaming(fetch(wasmUrl), imports)).instance.exports;
export { wasm as __wasm };

