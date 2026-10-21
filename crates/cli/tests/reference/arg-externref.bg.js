/**
 * @param {number} n
 * @param {any | null | undefined} a
 * @param {number} x
 * @param {number | null} [m]
 * @returns {boolean}
 */
export function option_then_numbers(n, a, x, m) {
    const val1 = +n;
    const val2 = +x;
    const val3 = +(isLikeNone(m) ? Number.MAX_SAFE_INTEGER : (m) >> 0);
    const ret = wasm.option_then_numbers(val1, isLikeNone(a) ? 0 : addToExternrefTable0(a), val2, val3);
    return ret !== 0;
}

/**
 * @param {any | null | undefined} a
 * @param {bigint} n
 * @returns {boolean}
 */
export function option_then_u64(a, n) {
    const val1 = BigInt.asIntN(64, n);
    const ret = wasm.option_then_u64(isLikeNone(a) ? 0 : addToExternrefTable0(a), val1);
    return ret !== 0;
}

/**
 * @param {any | null | undefined} a
 * @param {any} b
 * @returns {boolean}
 */
export function option_without_numbers(a, b) {
    const ret = wasm.option_without_numbers(isLikeNone(a) ? 0 : addToExternrefTable0(a), b);
    return ret !== 0;
}

/**
 * @param {any} a
 * @param {bigint} n
 * @returns {boolean}
 */
export function owned_then_u64(a, n) {
    const ret = wasm.owned_then_u64(a, n);
    return ret !== 0;
}
export function __wbg___wbindgen_is_falsy_16bd49b68658263e(arg0) {
    const ret = !arg0;
    return ret;
}
export function __wbg___wbindgen_is_undefined_8865fb403f8fe9d8(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(bytes) {
    numBytesDecoded += bytes.length;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
        cachedTextDecoder.decode();
        numBytesDecoded = bytes.length;
    }
    return cachedTextDecoder.decode(bytes);
}


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
