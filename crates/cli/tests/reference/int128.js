let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}
/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i128(a) {
    const ret = wasm.echo_i128(a, a >> BigInt(64));
    return (BigInt.asUintN(64, ret[0]) | (ret[1] << BigInt(64)));
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u128(a) {
    const ret = wasm.echo_u128(a, a >> BigInt(64));
    return (BigInt.asUintN(64, ret[0]) | (BigInt.asUintN(64, ret[1]) << BigInt(64)));
}

function isLikeNone(x) {
    return x == null;
}
/**
 * @param {bigint | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i128(a) {
    const ret = wasm.echo_option_i128(!isLikeNone(a), isLikeNone(a) ? BigInt(0) : a, isLikeNone(a) ? BigInt(0) : a >> BigInt(64));
    return ret[0] === 0 ? undefined : (BigInt.asUintN(64, ret[1]) | (ret[2] << BigInt(64)));
}

/**
 * @param {bigint | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u128(a) {
    const ret = wasm.echo_option_u128(!isLikeNone(a), isLikeNone(a) ? BigInt(0) : a, isLikeNone(a) ? BigInt(0) : a >> BigInt(64));
    return ret[0] === 0 ? undefined : (BigInt.asUintN(64, ret[1]) | (BigInt.asUintN(64, ret[2]) << BigInt(64)));
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_0.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * @returns {bigint}
 */
export function throw_i128() {
    const ret = wasm.throw_i128();
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    return (BigInt.asUintN(64, ret[0]) | (ret[1] << BigInt(64)));
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

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

