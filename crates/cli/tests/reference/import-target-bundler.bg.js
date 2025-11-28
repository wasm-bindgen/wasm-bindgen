import { default as _default } from 'tests/wasm/import_class.js';

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
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

export function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function __wbg___wbindgen_throw_dd24417ed36fc46e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg_catch_me_1d18acaa34acb005() { return handleError(function () {
    catch_me();
}, arguments) };

export function __wbg_get_c871386e44ba8c35(arg0) {
    const ret = arg0.get();
    return ret;
};

export function __wbg_my_function_597f96bc4719408a() {
    b.my_function();
};

export function __wbg_new_c30895ccee9479d4(arg0) {
    const ret = new _default(arg0);
    return ret;
};

export function __wbg_no_catch_757175fbf9e08b9e() {
    no_catch();
};

export function __wbg_reload_b091d4dc4b1b3a74() {
    window.location.reload();
};

export function __wbg_static_accessor_CONST_85b96acb48be57e1() {
    const ret = a.CONST;
    return ret;
};

export function __wbg_write_691fc0d693f0c7b5(arg0, arg1) {
    window.document.write(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
};
