export function exported() {
    wasm.exported();
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_another_9901a975ce0a4ca4(arg0) {
    const ret = arg0.prop2;
    return ret;
}
export function __wbg_b_6086af6cfa460743(arg0) {
    const ret = arg0.a;
    return ret;
}
export function __wbg_bar2_27fd601ca9c7705e() {
    const ret = Bar.bar2();
    return ret;
}
export function __wbg_get_foo_225ac333baff72fb() {
    const ret = Bar.get_foo();
    return ret;
}
export function __wbg_new_0a8c8aa0c3831eff() {
    const ret = new SomeClass();
    return ret;
}
export function __wbg_set_another_6313ddfa56a45080(arg0, arg1) {
    arg0.prop2 = arg1 >>> 0;
}
export function __wbg_set_b_78928624b22966c6(arg0, arg1) {
    arg0.a = arg1 >>> 0;
}
export function __wbg_set_bar2_dfbeba9889d2c348(arg0) {
    Bar.set_bar2(arg0 >>> 0);
}
export function __wbg_set_foo_f48c7ea95bc10ade(arg0) {
    Bar.set_foo(arg0 >>> 0);
}
export function __wbg_set_signal_3e79bb805bd15b95(arg0, arg1) {
    arg0.signal = arg1 >>> 0;
}
export function __wbg_set_some_prop_bd06b568caffcf3a(arg0, arg1) {
    arg0.some_prop = arg1 >>> 0;
}
export function __wbg_signal_e9c12b322bf51ce2(arg0) {
    const ret = arg0.signal;
    return ret;
}
export function __wbg_some_prop_0e1ce88c7a71e58f(arg0) {
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
