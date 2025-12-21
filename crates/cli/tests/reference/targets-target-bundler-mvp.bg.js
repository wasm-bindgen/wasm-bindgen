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
export function __wbg_random_9526caf33df4270d() {
    const ret = Math.random();
    return ret;
}
//#endregion


//#region wasm loading
let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
//#endregion

