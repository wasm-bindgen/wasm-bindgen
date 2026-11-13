//#region exports

/**
 * @param {number} a
 * @param {string[]} names
 * @param {undefined[]} units
 * @returns {number}
 */
export function beside(a, names, units) {
    _assertNum(a);
    _assertArrayLike(names);
    if (names.length > 268435455) throw new Error('invalid malloc request');
    for (let n0 = 0; n0 < names.length; n0++) {
        const e0 = names[n0];
        if (typeof e0 !== 'string') throw new Error('array contains a value of the wrong type');
    }
    _assertArrayLike(units);
    if (units.length > 4294967295) throw new Error('invalid malloc request');
    for (let n3 = 0; n3 < units.length; n3++) {
        const e3 = units[n3];
        if (e3 !== undefined) throw new Error('array contains a value of the wrong type');
    }
    const val5 = +a;
    let j0 = 0, len0, ptr0, len3, ptr3 = 1;
    try {
        len0 = names.length >>> 0;
        if (len0 > 268435455) throw new Error('invalid malloc request');
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = names[j0];
            if (typeof e0 !== 'string') throw new Error('array contains a value of the wrong type');
            const ptr1 = passStringToWasm0(e0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 8;
            view0.setInt32(p0 + 0, ptr1, true);
            view0.setInt32(p0 + 4, len1, true);
        }
        len3 = Math.trunc(units.length);
        if (len3 < 0 || len3 > 4294967295) throw new Error('invalid malloc request');
        for (let j3 = 0; j3 < len3; j3++) {
            const e3 = units[j3];
            if (e3 !== undefined) throw new Error('array contains a value of the wrong type');
        }
    } catch (e) {
        if (ptr0) {
            let view2 = getDataViewMemory0();
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 8;
                const w2_0 = view2.getInt32(cell2 + 0, true) >>> 0;
                const w2_1 = view2.getInt32(cell2 + 4, true) >>> 0;
                if (w2_0) {
                    wasm.__wbindgen_free(w2_0, w2_1 * 1, 1);
                    view2 = getDataViewMemory0();
                }
            }
            wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        }
        throw e;
    }
    const ret = wasm.beside(val5, ptr0, len0, ptr3, len3);
    return ret >>> 0;
}

export function driver() {
    wasm.driver();
}

/**
 * @param {(undefined[])[]} v
 * @returns {(undefined[])[]}
 */
export function nested(v) {
    _assertArrayLike(v);
    if (v.length > 268435455) throw new Error('invalid malloc request');
    for (let n0 = 0; n0 < v.length; n0++) {
        const e0 = v[n0];
        if (!Array.isArray(e0) && !ArrayBuffer.isView(e0)) throw new Error('array contains a value of the wrong type');
        if (e0.length > 4294967295) throw new Error('invalid malloc request');
        for (let n1 = 0; n1 < e0.length; n1++) {
            const e1 = e0[n1];
            if (e1 !== undefined) throw new Error('array contains a value of the wrong type');
        }
    }
    let len0, ptr0;
    try {
        len0 = v.length >>> 0;
        if (len0 > 268435455) throw new Error('invalid malloc request');
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const p0 = ptr0 + j0 * 8;
            if (!Array.isArray(e0) && !ArrayBuffer.isView(e0)) throw new Error('array contains a value of the wrong type');
            const len1 = Math.trunc(e0.length);
            if (len1 < 0 || len1 > 4294967295) throw new Error('invalid malloc request');
            const ptr1 = 1;
            for (let j1 = 0; j1 < len1; j1++) {
                const e1 = e0[j1];
                if (e1 !== undefined) throw new Error('array contains a value of the wrong type');
            }
            view0.setInt32(p0 + 0, ptr1, true);
            view0.setInt32(p0 + 4, len1, true);
        }
    } catch (e) {
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        throw e;
    }
    const ret = wasm.nested(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    const b3 = p3;
    try {
        const view3 = getDataViewMemory0();
        for (const end3 = p3 + ret[1] * 8; p3 < end3; p3 += 8) {
            const w3_0 = view3.getInt32(p3 + 0, true);
            const w3_1 = view3.getInt32(p3 + 4, true);
            const v4 = [];
            for (let n4 = w3_1 >>> 0; n4 > 0; n4--) {
                v4.push(undefined);
            }
            v3.push(v4);
        }
    } finally {
        if (b3) wasm.__wbindgen_free(b3, ret[1] * 8, 4);
    }
    return v3;
}

/**
 * @param {undefined[] | null} [v]
 * @returns {undefined[] | undefined}
 */
export function opt_units(v) {
    if (!isLikeNone(v)) {
        _assertArrayLike(v);
    }
    if (!isLikeNone(v) && v.length > 4294967295) throw new Error('invalid malloc request');
    if (!isLikeNone(v)) {
        for (let n0 = 0; n0 < v.length; n0++) {
            const e0 = v[n0];
            if (e0 !== undefined) throw new Error('array contains a value of the wrong type');
        }
    }
    let ptr0 = 0;
    let len0 = 0;
    if (!isLikeNone(v)) {
        len0 = Math.trunc(v.length);
        if (len0 < 0 || len0 > 4294967295) throw new Error('invalid malloc request');
        ptr0 = 1;
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            if (e0 !== undefined) throw new Error('array contains a value of the wrong type');
        }
    }
    const ret = wasm.opt_units(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = [];
        for (let n2 = ret[1] >>> 0; n2 > 0; n2--) {
            v2.push(undefined);
        }
    }
    return v2;
}

/**
 * @param {undefined[]} v
 * @returns {undefined[]}
 */
export function units(v) {
    _assertArrayLike(v);
    if (v.length > 4294967295) throw new Error('invalid malloc request');
    for (let n0 = 0; n0 < v.length; n0++) {
        const e0 = v[n0];
        if (e0 !== undefined) throw new Error('array contains a value of the wrong type');
    }
    const len0 = Math.trunc(v.length);
    if (len0 < 0 || len0 > 4294967295) throw new Error('invalid malloc request');
    const ptr0 = 1;
    for (let j0 = 0; j0 < len0; j0++) {
        const e0 = v[j0];
        if (e0 !== undefined) throw new Error('array contains a value of the wrong type');
    }
    const ret = wasm.units(ptr0, len0);
    const v2 = [];
    for (let n2 = ret[1] >>> 0; n2 > 0; n2--) {
        v2.push(undefined);
    }
    return v2;
}

//#endregion

//#region wasm imports
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_slice_b442f45de0854e9d() { return logError(function (arg0, arg1, arg2, arg3) {
    const v0 = [];
    for (let n0 = arg1 >>> 0; n0 > 0; n0--) {
        v0.push(undefined);
    }
    let v1;
    if (arg2 !== 0) {
        v1 = [];
        for (let n1 = arg3 >>> 0; n1 > 0; n1--) {
            v1.push(undefined);
        }
    }
    js_slice(v0, v1);
}, arguments); }
export function __wbg_js_units_ab3635b1277f1800() { return logError(function (arg0, arg1, arg2) {
    const v0 = [];
    for (let n0 = arg2 >>> 0; n0 > 0; n0--) {
        v0.push(undefined);
    }
    const ret = js_units(v0);
    _assertArrayLike(ret);
    if (ret.length > 4294967295) throw new Error('invalid malloc request');
    for (let n2 = 0; n2 < ret.length; n2++) {
        const e2 = ret[n2];
        if (e2 !== undefined) throw new Error('array contains a value of the wrong type');
    }
    const len2 = Math.trunc(ret.length);
    if (len2 < 0 || len2 > 4294967295) throw new Error('invalid malloc request');
    const ptr2 = 1;
    for (let j2 = 0; j2 < len2; j2++) {
        const e2 = ret[j2];
        if (e2 !== undefined) throw new Error('array contains a value of the wrong type');
    }
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len2, true);
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr2, true);
}, arguments); }
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}

//#endregion

//#region intrinsics
function _assertArrayLike(n, typedArray) {
    if (typeof(n) !== 'object' || n === null || typeof(n.length) !== 'number') {
        const expected = typedArray === undefined ? 'an array' : `an array or ${typedArray}`;
        const found = n === null ? 'null' : typeof(n) === 'object' ? Object.prototype.toString.call(n).slice(8, -1) : typeof(n);
        throw new Error(`expected ${expected}, found ${found}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
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

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function passStringToWasm0(arg, malloc, realloc) {
    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


//#endregion

//#region wasm loading

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

//#endregion
