export function exported() {
    wasm.exported();
}
export function __wbg___wbindgen_throw_5d9e815e6fdf150f(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_c_27eb6c8c34a36e09(arg0, arg1) {
    a.c(getStringFromWasm0(arg0, arg1));
}
export function __wbg_c_62f7c495c0da1f6c(arg0, arg1) {
    a.b.c(getStringFromWasm0(arg0, arg1));
}
export function __wbg_info_219b4a73877b158f(arg0, arg1) {
    epsilon.info(getStringFromWasm0(arg0, arg1));
}
export function __wbg_info_b8c8b37229b5e91d(arg0, arg1) {
    delta.info(getStringFromWasm0(arg0, arg1));
}
export function __wbg_log_aa9c51e79de6434e(arg0, arg1) {
    alpha.log(getStringFromWasm0(arg0, arg1));
}
export function __wbg_log_df735e8b40a03464(arg0, arg1) {
    beta.log(getStringFromWasm0(arg0, arg1));
}
export function __wbg_static_accessor_STATE_21acaf98a2b6b7ca() {
    const ret = iota.STATE;
    return ret;
}
export function __wbg_static_accessor_STATE_73563c4d96c22122() {
    const ret = theta.STATE;
    return ret;
}
export function __wbg_warn_505bf06fa20f48dc(arg0, arg1) {
    warn(getStringFromWasm0(arg0, arg1));
}
export function __wbg_warn_74a4944fa727b481(arg0, arg1) {
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
