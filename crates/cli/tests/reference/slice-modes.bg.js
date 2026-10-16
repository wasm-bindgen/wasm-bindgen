export function driver() {
    wasm.driver();
}
export function __wbg_js_block_mixed_90b242452a847672(arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getArrayU16FromWasm0(arg0, arg1).slice();
    var v1 = Array.from(getArrayU16FromWasm0(arg2, arg3));
    js_block_mixed(v0, v1, getArrayU16FromWasm0(arg4, arg5));
}
export function __wbg_js_copy_f64_dffe8df981aad359(arg0, arg1) {
    var v0 = getArrayF64FromWasm0(arg0, arg1).slice();
    js_copy_f64(v0);
}
export function __wbg_js_copy_optional_u16_1a0519dcdf06b916(arg0, arg1) {
    let v0;
    if (arg0 !== 0) {
        v0 = getArrayU16FromWasm0(arg0, arg1).slice();
    }
    js_copy_optional_u16(v0);
}
export function __wbg_js_copy_u8_156faece81e47d65(arg0, arg1) {
    var v0 = getArrayU8FromWasm0(arg0, arg1).slice();
    js_copy_u8(v0);
}
export function __wbg_js_view_u8_121c4880f6a160aa(arg0, arg1) {
    js_view_u8(getArrayU8FromWasm0(arg0, arg1));
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

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
