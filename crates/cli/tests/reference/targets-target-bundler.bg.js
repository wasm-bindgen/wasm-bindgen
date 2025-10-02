let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    return wasm.add_that_might_fail(a, b) >>> 0;
}

export function __wbg_random_ae0b2256206ad108() {
    return Math.random();
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

