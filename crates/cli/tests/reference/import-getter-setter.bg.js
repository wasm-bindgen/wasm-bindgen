let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
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
    wasm.exported();
}

export function __wbg___wbindgen_throw_dd24417ed36fc46e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg_another_f4b9a21c6e8a4d1c(arg0) {
    const ret = arg0.prop2;
    return ret;
};

export function __wbg_b_bb1b582d09c61aba(arg0) {
    const ret = arg0.a;
    return ret;
};

export function __wbg_bar2_713391e46a0bb7be() {
    const ret = Bar.bar2();
    return ret;
};

export function __wbg_get_foo_8df4cafb04f0009f() {
    const ret = Bar.get_foo();
    return ret;
};

export function __wbg_new_20883a5bbcc03d8a() {
    const ret = new SomeClass();
    return ret;
};

export function __wbg_set_another_cde5169fcc608e26(arg0, arg1) {
    arg0.prop2 = arg1 >>> 0;
};

export function __wbg_set_b_1453844add10f92c(arg0, arg1) {
    arg0.a = arg1 >>> 0;
};

export function __wbg_set_bar2_e814dd44597f34a8(arg0) {
    Bar.set_bar2(arg0 >>> 0);
};

export function __wbg_set_foo_0f65d059f3230f2a(arg0) {
    Bar.set_foo(arg0 >>> 0);
};

export function __wbg_set_signal_d8e77f065b1f0439(arg0, arg1) {
    arg0.signal = arg1 >>> 0;
};

export function __wbg_set_some_prop_4108a32b244b7d28(arg0, arg1) {
    arg0.some_prop = arg1 >>> 0;
};

export function __wbg_signal_54e2df9a78183990(arg0) {
    const ret = arg0.signal;
    return ret;
};

export function __wbg_some_prop_3c07d616a6648f3b(arg0) {
    const ret = arg0.some_prop;
    return ret;
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
