export function exported() {
    wasm.exported();
}
export function __wbg_c_c6c4f07d196bdffe(arg0, arg1) {
    a.b.c(getStringFromWasm0(arg0, arg1));
}
export function __wbg_c_ca0c67a358a3ed69(arg0, arg1) {
    a.c(getStringFromWasm0(arg0, arg1));
}
export function __wbg_log_5e5e4051c9e253ff(arg0, arg1) {
    beta.log(getStringFromWasm0(arg0, arg1));
}
export function __wbg_log_747870ed06c474c8(arg0, arg1) {
    alpha.log(getStringFromWasm0(arg0, arg1));
}
export function __wbg_warn_505bf06fa20f48dc(arg0, arg1) {
    warn(getStringFromWasm0(arg0, arg1));
}
export function __wbg_warn_ac30e50f7e66848f(arg0, arg1) {
    gamma.warn(getStringFromWasm0(arg0, arg1));
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
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
