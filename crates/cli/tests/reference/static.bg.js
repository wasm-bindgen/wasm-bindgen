let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
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

function isLikeNone(x) {
    return x === undefined || x === null;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

export function exported() {
    wasm.exported();
}

export function __wbg___wbindgen_throw_451ec1a8469d7eb6(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg_static_accessor_NAMESPACE_OPTIONAL_2e93032090b95d76() {
    return isLikeNone(typeof test === 'undefined' ? null : test?.NAMESPACE_OPTIONAL) ? 0 : addToExternrefTable0(typeof test === 'undefined' ? null : test?.NAMESPACE_OPTIONAL);
};

export function __wbg_static_accessor_NAMESPACE_PLAIN_996cfe92d41df43c() {
    return test.NAMESPACE_PLAIN;
};

export function __wbg_static_accessor_NESTED_NAMESPACE_OPTIONAL_c736ff66b2fdaa43() {
    return isLikeNone(typeof test1 === 'undefined' ? null : test1?.test2?.NESTED_NAMESPACE_OPTIONAL) ? 0 : addToExternrefTable0(typeof test1 === 'undefined' ? null : test1?.test2?.NESTED_NAMESPACE_OPTIONAL);
};

export function __wbg_static_accessor_NESTED_NAMESPACE_PLAIN_e9affde7bc139ebd() {
    return test1.test2.NESTED_NAMESPACE_PLAIN;
};

export function __wbg_static_accessor_OPTIONAL_8c4485fd717985fd() {
    return isLikeNone(typeof OPTIONAL === 'undefined' ? null : OPTIONAL) ? 0 : addToExternrefTable0(typeof OPTIONAL === 'undefined' ? null : OPTIONAL);
};

export function __wbg_static_accessor_PLAIN_c0ea7240f2fd9157() {
    return PLAIN;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

