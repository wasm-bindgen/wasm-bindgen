export function _function() {
    wasm._function();
}

export function _var() {
    wasm._var();
}

export function exported() {
    wasm.exported();
}

/**
 * @param {number} _new
 * @param {number} _var
 * @param {number} _switch
 * @param {number} _default
 * @param {number} _arguments
 */
export function weird_arguments(_new, _var, _switch, _default, _arguments) {
    wasm.weird_arguments(_new, _var, _switch, _default, _arguments);
}
export function __wbg___wbindgen_throw_83ebd457a191bc2a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_await_4ea226770dbb5814() {
    await();
}
export function __wbg_let_bd65bc5e4baee9c0(arg0) {
    arg0.let();
}
export function __wbg_new_8ea43830bd92dbd2() {
    B.new();
}
export function __wbg_new_aa2b5c77eb49997e() {
    const ret = window.__TAURI__.menu.Menu.new();
    return ret;
}
export function __wbg_new_c882a94b69381309() {
    const ret = A.new();
    return ret;
}
export function __wbg_static_accessor_TRUE_c4e736fa46a2ab00() {
    const ret = true;
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
function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
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
