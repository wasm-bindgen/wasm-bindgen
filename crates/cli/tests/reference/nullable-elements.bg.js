/**
 * @param {Function} f
 */
export function use_imports(f) {
    wasm.use_imports(f);
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_call_nullable_61a2ccea214cfd68(arg0) {
    call_nullable(arg0);
}
export function __wbg_nullable_js_strings_4d25a7d2d74571c9(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
        v0.push(wasm.__wbindgen_externrefs.get(view0.getInt32(p0 + 0, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    nullable_js_strings(v0);
}
export function __wbg_nullable_strings_e1dfa29465202c22(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(view0.getInt32(p0 + 0, true) === 0 ? null : getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    nullable_strings(v0);
}
export function __wbg_opt_f64s_95d8affd5b395def(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 16; p0 < end0; p0 += 16) {
        v0.push(view0.getInt32(p0 + 0, true) === 0 ? undefined : view0.getFloat64(p0 + 8, true));
    }
    wasm.__wbindgen_free(arg0, arg1 * 16, 8);
    opt_f64s(v0);
}
export function __wbg_opt_ref_nullable_string_6d266382e66f94b3(arg0, arg1) {
    opt_ref_nullable_string(arg0 === 0 ? (arg1 === 1025 ? null : undefined) : getStringFromWasm0(arg0, arg1));
}
export function __wbg_ref_f64_4483fcdd9da682a2(arg0) {
    ref_f64(arg0);
}
export function __wbg_ref_ref_str_08905e79d5436aa5(arg0, arg1) {
    ref_ref_str(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
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
