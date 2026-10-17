export function driver() {
    wasm.driver();
}
export function __wbg_js_block_slice_u16_351329232b16999d(arg0, arg1) {
    js_block_slice_u16(Array.from(getArrayU16FromWasm0(arg0, arg1)));
}
export function __wbg_js_slice_optional_string_as_array_ce9ccdb9300131fd(arg0, arg1) {
    let v0;
    if (arg0 !== 0) {
        v0 = [];
        let p0 = arg0;
        p0 = p0 >>> 0;
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
            v0.push(getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
        }
        wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    }
    js_slice_optional_string_as_array(v0);
}
export function __wbg_js_slice_optional_u16_as_array_f119bb99afccb33c(arg0, arg1) {
    js_slice_optional_u16_as_array(arg0 === 0 ? undefined : Array.from(getArrayU16FromWasm0(arg0, arg1)));
}
export function __wbg_js_slice_string_as_array_03aefca66a560a29(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    js_slice_string_as_array(v0);
}
export function __wbg_js_slice_u16_as_array_78e736a5d5adf04b(arg0, arg1) {
    js_slice_u16_as_array(Array.from(getArrayU16FromWasm0(arg0, arg1)));
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
function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
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
