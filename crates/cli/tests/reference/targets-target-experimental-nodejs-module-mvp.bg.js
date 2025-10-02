let wasm;
let wasmModule;
export function __wbg_set_wasm(exports, module) {
    wasm = exports;
    wasmModule = module;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    return wasm.add_that_might_fail(a, b) >>> 0;
}

export function __wbg_random_9526caf33df4270d() {
    return Math.random();
};

