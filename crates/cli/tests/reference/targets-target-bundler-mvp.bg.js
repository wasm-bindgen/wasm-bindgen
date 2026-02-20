/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}
export function __wbg_random_c82d91f28994c195() {
    const ret = Math.random();
    return ret;
}

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
