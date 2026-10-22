/**
 * @enum {1 | 2}
 */
export const Level = Object.freeze({
    Low: 1, "1": "Low",
    High: 2, "2": "High",
});

export function driver() {
    wasm.driver();
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_bools_9e72c19b74d918e2(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
        v0.push(view0.getInt32(p0 + 0, true) !== 0);
    }
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    js_bools(v0);
}
export function __wbg_js_handles_394bed7813e915ae(arg0, arg1) {
    js_handles(getArrayJsValueViewFromWasm0(arg0, arg1));
}
export function __wbg_js_levels_92d6261cd8377562(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(view0.getFloat64(p0 + 0, true));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 8);
    js_levels(v0);
}
export function __wbg_js_modes_bb53fb3d0fbfa09c(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
        v0.push(__wbindgen_enum_Mode[view0.getInt32(p0 + 0, true)]);
    }
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    js_modes(v0);
}
export function __wbg_js_opt_chars_43eba37f0b6dc71f(arg0, arg1) {
    let v0;
    if (arg0 !== 0) {
        v0 = [];
        let p0 = arg0;
        p0 = p0 >>> 0;
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
            v0.push(String.fromCodePoint(view0.getInt32(p0 + 0, true)));
        }
        wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    }
    js_opt_chars(v0);
}
export function __wbg_js_strings_2558d2768768ed5f(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    js_strings(v0);
}
export function __wbg_js_strs_4204ef305fb644c2(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    js_strs(v0);
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
const __wbindgen_enum_Mode = ["read", "write"];

function getArrayJsValueViewFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    return result;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
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
