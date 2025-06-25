let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function isLikeNone(x) {
    return x == null;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_1.set(idx, obj);
    return idx;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

export function exported() {
    wasm.exported();
}

export function __wbg_static_accessor_NAMESPACE_OPTIONAL_c9a4344c544120f4() {
    const ret = typeof test === 'undefined' ? null : test?.NAMESPACE_OPTIONAL;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_NAMESPACE_PLAIN_784c8d7f5bbac62a() {
    const ret = test.NAMESPACE_PLAIN;
    return ret;
};

export function __wbg_static_accessor_NESTED_NAMESPACE_OPTIONAL_a414abbeb018a35a() {
    const ret = typeof test1 === 'undefined' ? null : test1?.test2?.NESTED_NAMESPACE_OPTIONAL;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_NESTED_NAMESPACE_PLAIN_1121b285cb8479df() {
    const ret = test1.test2.NESTED_NAMESPACE_PLAIN;
    return ret;
};

export function __wbg_static_accessor_OPTIONAL_ade71b6402851d0c() {
    const ret = typeof OPTIONAL === 'undefined' ? null : OPTIONAL;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_PLAIN_c0f08eb2f0db194c() {
    const ret = PLAIN;
    return ret;
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

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

