let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function isLikeNone(x) {
    return x === undefined || x === null;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_1.set(idx, obj);
    return idx;
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

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

export function exported() {
    wasm.exported();
}

export function __wbg_static_accessor_NAMESPACE_OPTIONAL_0000000000000000() {
    const ret = typeof test === 'undefined' ? null : test?.NAMESPACE_OPTIONAL;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_NAMESPACE_PLAIN_0000000000000001() {
    const ret = test.NAMESPACE_PLAIN;
    return ret;
};

export function __wbg_static_accessor_NESTED_NAMESPACE_OPTIONAL_0000000000000002() {
    const ret = typeof test1 === 'undefined' ? null : test1?.test2?.NESTED_NAMESPACE_OPTIONAL;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_NESTED_NAMESPACE_PLAIN_0000000000000003() {
    const ret = test1.test2.NESTED_NAMESPACE_PLAIN;
    return ret;
};

export function __wbg_static_accessor_OPTIONAL_0000000000000004() {
    const ret = typeof OPTIONAL === 'undefined' ? null : OPTIONAL;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_PLAIN_0000000000000005() {
    const ret = PLAIN;
    return ret;
};

export function __wbg_wbindgenthrow_0000000000000006(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_1;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

