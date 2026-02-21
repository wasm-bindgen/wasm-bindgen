export function exported() {
    wasm.exported();
}
export function __wbg___wbindgen_throw_df03e93053e0f4bc(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_another_5e391a82904c95f3(arg0) {
    const ret = arg0.prop2;
    return ret;
}
export function __wbg_b_9258c02a5383359c(arg0) {
    const ret = arg0.a;
    return ret;
}
export function __wbg_bar2_a8f90bac60f6a1ef() {
    const ret = Bar.bar2();
    return ret;
}
export function __wbg_get_foo_2cfe584a610f2fe7() {
    const ret = Bar.get_foo();
    return ret;
}
export function __wbg_new_2d06dc6ad956a2e8() {
    const ret = new SomeClass();
    return ret;
}
export function __wbg_set_another_c7bea22c1395f7c7(arg0, arg1) {
    arg0.prop2 = arg1 >>> 0;
}
export function __wbg_set_b_28af687e466c2915(arg0, arg1) {
    arg0.a = arg1 >>> 0;
}
export function __wbg_set_bar2_1f717a0f52a88346(arg0) {
    Bar.set_bar2(arg0 >>> 0);
}
export function __wbg_set_foo_9310b27d3607e1da(arg0) {
    Bar.set_foo(arg0 >>> 0);
}
export function __wbg_set_signal_2da17cf9cd6c4702(arg0, arg1) {
    arg0.signal = arg1 >>> 0;
}
export function __wbg_set_some_prop_1f95e7eaab86103b(arg0, arg1) {
    arg0.some_prop = arg1 >>> 0;
}
export function __wbg_signal_178ef00cfc37cb61(arg0) {
    const ret = arg0.signal;
    return ret;
}
export function __wbg_some_prop_97f412fbfc566107(arg0) {
    const ret = arg0.some_prop;
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
