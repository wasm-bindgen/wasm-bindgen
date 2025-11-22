
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
        __wbg_random_ae0b2256206ad108: function() {
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
    },

};

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasm = (await WebAssembly.instantiateStreaming(fetch(wasmUrl), imports)).instance.exports;
export { wasm as __wasm };

wasm.__wbindgen_start();
