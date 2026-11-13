export function use_imports() {
    wasm.use_imports();
}
export function __wbg_bytes_29201aac79b58f49(arg0, arg1) {
    bytes(getArrayU8FromWasm0(arg0, arg1));
}
export function __wbg_opt_byte_vec_c3294165014f2387(arg0, arg1) {
    opt_byte_vec(arg0 === 0 ? undefined : getArrayU8FromWasm0(arg0, arg1).slice());
}
export function __wbg_opt_string_vec_9bc7e0bd264b9984(arg0, arg1) {
    const b0 = arg0 >>> 0;
    try {
        let v0;
        if (arg0 !== 0) {
            v0 = [];
            let p0 = b0;
            const view0 = getDataViewMemory0();
            for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
                const w0_0 = view0.getInt32(p0 + 0, true);
                const w0_1 = view0.getInt32(p0 + 4, true);
                v0.push(getStringFromWasm0(w0_0, w0_1));
            }
        }
        opt_string_vec(v0);
    } finally {
        if (b0) wasm.__wbindgen_free(b0, arg1 * 8, 4);
    }
}
export function __wbg_opt_vecs_d5230a392f57f00b(arg0, arg1) {
    const b0 = arg0 >>> 0;
    try {
        const v0 = [];
        let p0 = b0;
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            const w0_1 = view0.getInt32(p0 + 4, true);
            v0.push(w0_0 === 0 ? undefined : getArrayU32FromWasm0(w0_0, w0_1).slice());
        }
        opt_vecs(v0);
    } finally {
        if (b0) wasm.__wbindgen_free(b0, arg1 * 8, 4);
    }
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
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

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
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
