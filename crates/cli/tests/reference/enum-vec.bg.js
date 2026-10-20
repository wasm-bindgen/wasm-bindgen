/**
 * @enum {0 | 1 | 2}
 */
export const Color = Object.freeze({
    Green: 0, "0": "Green",
    Yellow: 1, "1": "Yellow",
    Red: 2, "2": "Red",
});

/**
 * @enum {0 | 1}
 */
const HiddenErr = Object.freeze({
    X: 0, "0": "X",
    Y: 1, "1": "Y",
});

/**
 * @enum {0 | 1}
 */
export const RenamedErr = Object.freeze({
    One: 0, "0": "One",
    Two: 1, "1": "Two",
});

/**
 * @param {Color[]} values
 * @returns {Color[]}
 */
export function enum_vec_echo(values) {
    const len0 = values.length;
    const ptr0 = wasm.__wbindgen_malloc(len0 * 8, 8) >>> 0;
    try {
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = values[j0];
            const p0 = ptr0 + j0 * 8;
            view0.setFloat64(p0 + 0, e0, true);
        }
    } catch (err0) {
        wasm.__wbindgen_free(ptr0, len0 * 8, 8);
        throw err0;
    }
    const ret = wasm.enum_vec_echo(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 8; p2 < end2; p2 += 8) {
        v2.push(view2.getFloat64(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {HiddenErr[]} values
 * @returns {HiddenErr[]}
 */
export function hidden_err_vec_echo(values) {
    const len0 = values.length;
    const ptr0 = wasm.__wbindgen_malloc(len0 * 8, 8) >>> 0;
    try {
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = values[j0];
            const p0 = ptr0 + j0 * 8;
            view0.setFloat64(p0 + 0, e0, true);
        }
    } catch (err0) {
        wasm.__wbindgen_free(ptr0, len0 * 8, 8);
        throw err0;
    }
    const ret = wasm.hidden_err_vec_echo(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 8; p2 < end2; p2 += 8) {
        v2.push(view2.getFloat64(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @enum {0 | 1}
 */
const ns__NsErr = Object.freeze({
    A: 0, "0": "A",
    B: 1, "1": "B",
});

export const ns = {};
ns.NsErr = ns__NsErr;

/**
 * @param {ns__NsErr[]} values
 * @returns {ns__NsErr[]}
 */
export function ns_err_vec_echo(values) {
    const len0 = values.length;
    const ptr0 = wasm.__wbindgen_malloc(len0 * 8, 8) >>> 0;
    try {
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = values[j0];
            const p0 = ptr0 + j0 * 8;
            view0.setFloat64(p0 + 0, e0, true);
        }
    } catch (err0) {
        wasm.__wbindgen_free(ptr0, len0 * 8, 8);
        throw err0;
    }
    const ret = wasm.ns_err_vec_echo(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 8; p2 < end2; p2 += 8) {
        v2.push(view2.getFloat64(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Color[] | null} [values]
 * @returns {Color[] | undefined}
 */
export function option_enum_vec_echo(values) {
    let ptr0 = 0;
    let len0 = 0;
    if (!isLikeNone(values)) {
        len0 = values.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 8) >>> 0;
        try {
            const view0 = getDataViewMemory0();
            for (let j0 = 0; j0 < len0; j0++) {
                const e0 = values[j0];
                const p0 = ptr0 + j0 * 8;
                view0.setFloat64(p0 + 0, e0, true);
            }
        } catch (err0) {
            wasm.__wbindgen_free(ptr0, len0 * 8, 8);
            throw err0;
        }
    }
    const ret = wasm.option_enum_vec_echo(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = [];
        let p2 = ret[0];
        p2 = p2 >>> 0;
        const view2 = getDataViewMemory0();
        for (const end2 = p2 + ret[1] * 8; p2 < end2; p2 += 8) {
            v2.push(view2.getFloat64(p2 + 0, true));
        }
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    }
    return v2;
}

/**
 * @param {RenamedErr[]} values
 * @returns {RenamedErr[]}
 */
export function renamed_err_vec_echo(values) {
    const len0 = values.length;
    const ptr0 = wasm.__wbindgen_malloc(len0 * 8, 8) >>> 0;
    try {
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = values[j0];
            const p0 = ptr0 + j0 * 8;
            view0.setFloat64(p0 + 0, e0, true);
        }
    } catch (err0) {
        wasm.__wbindgen_free(ptr0, len0 * 8, 8);
        throw err0;
    }
    const ret = wasm.renamed_err_vec_echo(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 8; p2 < end2; p2 += 8) {
        v2.push(view2.getFloat64(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
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
