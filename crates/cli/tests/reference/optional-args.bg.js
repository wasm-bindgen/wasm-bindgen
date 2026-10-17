/**
 * @param {number | null} [a]
 * @param {number | null} [b]
 * @param {number | null} [c]
 */
export function all_optional(a, b, c) {
    wasm.all_optional(isLikeNone(a) ? Number.MAX_SAFE_INTEGER : (a) >>> 0, isLikeNone(b) ? Number.MAX_SAFE_INTEGER : (b) >>> 0, isLikeNone(c) ? Number.MAX_SAFE_INTEGER : (c) >>> 0);
}

/**
 * @param {number | null | undefined} a
 * @param {number} b
 * @param {number | null} [c]
 */
export function some_optional(a, b, c) {
    wasm.some_optional(isLikeNone(a) ? Number.MAX_SAFE_INTEGER : (a) >>> 0, b, isLikeNone(c) ? Number.MAX_SAFE_INTEGER : (c) >>> 0);
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
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
