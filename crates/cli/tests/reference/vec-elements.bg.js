/**
 * @param {boolean[]} v
 * @returns {boolean[]}
 */
export function bools(v) {
    const len0 = v.length;
    const ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
    try {
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, e0, true);
        }
    } catch (err0) {
        wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw err0;
    }
    const ret = wasm.bools(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
        v2.push(view2.getInt32(p2 + 0, true) !== 0);
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

export function driver() {
    wasm.driver();
}

/**
 * @param {Mode[]} v
 * @returns {Mode[]}
 */
export function modes(v) {
    const len0 = v.length;
    const ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
    try {
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, (__wbindgen_enum_Mode.indexOf(e0) + 1 || 3) - 1, true);
        }
    } catch (err0) {
        wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw err0;
    }
    const ret = wasm.modes(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
        v2.push(__wbindgen_enum_Mode[view2.getInt32(p2 + 0, true)]);
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {string[] | null} [v]
 * @returns {string[] | undefined}
 */
export function opt_chars(v) {
    let ptr0 = 0;
    let len0 = 0;
    if (!isLikeNone(v)) {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        try {
            const view0 = getDataViewMemory0();
            for (let j0 = 0; j0 < len0; j0++) {
                const e0 = v[j0];
                const p0 = ptr0 + j0 * 4;
                const char1 = e0.codePointAt(0);
                _assertChar(char1);
                view0.setInt32(p0 + 0, char1, true);
            }
        } catch (err0) {
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
            throw err0;
        }
    }
    const ret = wasm.opt_chars(ptr0, len0);
    let v3;
    if (ret[0] !== 0) {
        v3 = [];
        let p3 = ret[0];
        p3 = p3 >>> 0;
        const view3 = getDataViewMemory0();
        for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
            v3.push(String.fromCodePoint(view3.getInt32(p3 + 0, true)));
        }
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v3;
}

/**
 * @returns {string[]}
 */
export function strs() {
    const ret = wasm.strs();
    const v1 = [];
    let p1 = ret[0];
    p1 = p1 >>> 0;
    const view1 = getDataViewMemory0();
    for (const end1 = p1 + ret[1] * 8; p1 < end1; p1 += 8) {
        v1.push(getStringFromWasm0(view1.getInt32(p1 + 0, true), view1.getInt32(p1 + 4, true)));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
    return v1;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_bools_30fc8a2bc11a9f84(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg2 * 4; p0 < end0; p0 += 4) {
        v0.push(view0.getInt32(p0 + 0, true) !== 0);
    }
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    const ret = js_bools(v0);
    const len2 = ret.length;
    const ptr2 = wasm.__wbindgen_malloc(len2 * 4, 4) >>> 0;
    try {
        const view2 = getDataViewMemory0();
        for (let j2 = 0; j2 < len2; j2++) {
            const e2 = ret[j2];
            const p2 = ptr2 + j2 * 4;
            view2.setInt32(p2 + 0, e2, true);
        }
    } catch (err2) {
        wasm.__wbindgen_free(ptr2, len2 * 4, 4);
        throw err2;
    }
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len2, true);
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr2, true);
}
export function __wbg_js_strs_d7bff571d595ef45(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
        v0.push(getStringFromWasm0(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true)));
    }
    wasm.__wbindgen_free(arg0, arg1 * 8, 4);
    js_strs(v0);
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
const __wbindgen_enum_Mode = ["read", "write"];

function _assertChar(c) {
    if (typeof(c) === 'number' && (c >= 0x110000 || (c >= 0xD800 && c < 0xE000))) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
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

function isLikeNone(x) {
    return x === undefined || x === null;
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
