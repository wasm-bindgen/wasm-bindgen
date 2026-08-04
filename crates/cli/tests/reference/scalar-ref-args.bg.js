export function driver() {
    wasm.driver();
}

/**
 * @returns {number}
 */
export function return_scalar_ref_via_alias() {
    const ret = wasm.return_scalar_ref_via_alias();
    return ret >>> 0;
}
export function __wbg_takeOtherRefs_ac11064eb438d124(arg0, arg1, arg2, arg3) {
    takeOtherRefs(arg0, arg1, arg2 !== 0, String.fromCodePoint(arg3));
}
export function __wbg_takeSignedRefs_403fd2c6a03e5f9e(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    takeSignedRefs(arg0, arg1, arg2, arg3, (BigInt.asUintN(64, arg4) | (arg5 << BigInt(64))), arg6);
}
export function __wbg_takeUnsignedRefs_ed8a3712ea2f6657(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    takeUnsignedRefs(arg0, arg1, arg2 >>> 0, BigInt.asUintN(64, arg3), (BigInt.asUintN(64, arg4) | (BigInt.asUintN(64, arg5) << BigInt(64))), arg6 >>> 0);
}
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
