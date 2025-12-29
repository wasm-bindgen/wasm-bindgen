/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function _default(a, b) {
    const ret = wasm.default(a, b);
    return ret;
}
export { _default as default }
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
