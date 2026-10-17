export function driver() {
    wasm.driver();
}
export function __wbg_js_block_mixed_2c5572af51372175(arg0, arg1, arg2, arg3, arg4, arg5) {
    js_block_mixed(getArrayU16FromWasm0(arg0, arg1).slice(), Array.from(getArrayU16FromWasm0(arg2, arg3)), getArrayU16FromWasm0(arg4, arg5));
}
export function __wbg_js_copy_f64_ee4f91cb0c300621(arg0, arg1) {
    js_copy_f64(getArrayF64FromWasm0(arg0, arg1).slice());
}
export function __wbg_js_copy_optional_u16_ae5d2263643d02de(arg0, arg1) {
    js_copy_optional_u16(arg0 === 0 ? undefined : getArrayU16FromWasm0(arg0, arg1).slice());
}
export function __wbg_js_copy_u8_aa48a1ab3a44ac8e(arg0, arg1) {
    js_copy_u8(getArrayU8FromWasm0(arg0, arg1).slice());
}
export function __wbg_js_shapes_0af45ab96e7304c4(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    var v0 = Array.from(getArrayU8FromWasm0(arg0, arg1));
    wasm.__wbindgen_free(arg0, arg1 * 1, 1);
    var v1 = Array.from(getArrayF64FromWasm0(arg2, arg3));
    wasm.__wbindgen_free(arg2, arg3 * 8, 8);
    const v2 = [];
    let p2 = arg6;
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + arg7 * 8; p2 < end2; p2 += 8) {
        v2.push(getStringFromWasm0(view2.getInt32(p2 + 0, true), view2.getInt32(p2 + 4, true)));
    }
    wasm.__wbindgen_free(arg6, arg7 * 8, 4);
    js_shapes(v0, v1, Array.from(getClampedArrayU8FromWasm0(arg4, arg5)), v2);
}
export function __wbg_js_view_u8_121c4880f6a160aa(arg0, arg1) {
    js_view_u8(getArrayU8FromWasm0(arg0, arg1));
}
export function __wbindgen_generic_0000000000000000(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    // generic import `js_generic_slice`: [Ref(Slice(Encoded(String)))] -> Unit
    js_generic_slice(v0);
}
export function __wbindgen_generic_0000000000000001(arg0, arg1) {
    // generic import `js_generic_slice`: [Ref(Slice(U16))] -> Unit
    js_generic_slice(getArrayU16FromWasm0(arg0, arg1));
}
export function __wbindgen_generic_0000000000000002(arg0, arg1) {
    // generic import `js_generic_slice`: [SliceMode(Copy, Option(Ref(Slice(U8))))] -> Unit
    js_generic_slice(arg0 === 0 ? undefined : getArrayU8FromWasm0(arg0, arg1).slice());
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
function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint16ArrayMemory0 = null;
function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;
function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
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
