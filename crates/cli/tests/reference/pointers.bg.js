let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * @param {number} input
 * @returns {number}
 */
export function const_pointer(input) {
    const ret = wasm.const_pointer(input);
    return ret >>> 0;
}

/**
 * @param {number} input
 * @returns {number}
 */
export function mut_pointer(input) {
    const ret = wasm.mut_pointer(input);
    return ret >>> 0;
}

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

