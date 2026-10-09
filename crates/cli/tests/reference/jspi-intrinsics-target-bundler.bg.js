/**
 * @param {any} promise
 * @returns {Promise<any>}
 */
export async function drive(promise) {
    const ret = await (__wbg_jspi_drive ??= WebAssembly.promising(wasm.drive))(promise);
    return ret;
}

export const __wbg___wbindgen_jspi_suspend_83a35dabfa04fa34 = ((__inner) => new WebAssembly.Suspending(function(...args) {
    try { return __inner.apply(this, args); }
    catch (e) { return Promise.reject(e); }
}))(function(arg0) {
    const ret = arg0;
    return ret;
});
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_generic_0000000000000000(arg0) {
    // Cast intrinsic for `Externref -> Externref`.
    const ret = arg0;
    return ret;
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

export const __wbindgen_jstag = WebAssembly.JSTag;
let __wbg_jspi_drive;

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
