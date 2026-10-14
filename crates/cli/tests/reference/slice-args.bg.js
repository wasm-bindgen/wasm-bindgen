export class Point {
    static __unwrap(jsValue) {
        if (!(jsValue instanceof Point)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export class Samples {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SamplesFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_samples_free(ptr, 0);
    }
    /**
     * @returns {Float64Array}
     */
    get values() {
        const ret = wasm.__wbg_get_samples_values(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {Float64Array} out
     */
    copy_into(out) {
        var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.samples_copy_into(this.__wbg_ptr, ptr0, len0, out);
    }
    /**
     * @param {Float64Array | number[]} more
     */
    extend(more) {
        const ptr0 = passArrayF64ToWasm0(more, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.samples_extend(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {Float64Array | number[]} values
     */
    constructor(values) {
        const ptr0 = passArrayF64ToWasm0(values, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.samples_new(ptr0, len0);
        this.__wbg_ptr = ret;
        SamplesFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Int32Array | number[]} values
     * @returns {number}
     */
    static sum(values) {
        const ptr0 = passArray32ToWasm0(values, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.samples_sum(ptr0, len0);
        return ret;
    }
    /**
     * @param {Float64Array | number[]} arg0
     */
    set values(arg0) {
        const ptr0 = passArrayF64ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_samples_values(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) Samples.prototype[Symbol.dispose] = Samples.prototype.free;

/**
 * @param {Uint8Array | number[]} a
 * @returns {Uint8Array}
 */
export function roundtrip(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.roundtrip(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Uint8ClampedArray} a
 */
export function take_clamped(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_clamped(ptr0, len0);
}

/**
 * @param {Float32Array | number[]} a
 */
export function take_f32(a) {
    const ptr0 = passArrayF32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_f32(ptr0, len0);
}

/**
 * @param {Float64Array | number[]} a
 */
export function take_f64(a) {
    const ptr0 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_f64(ptr0, len0);
}

/**
 * @param {Int16Array | number[]} a
 */
export function take_i16(a) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_i16(ptr0, len0);
}

/**
 * @param {Int32Array | number[]} a
 */
export function take_i32(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_i32(ptr0, len0);
}

/**
 * @param {BigInt64Array | bigint[]} a
 */
export function take_i64(a) {
    const ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_i64(ptr0, len0);
}

/**
 * @param {Int8Array | number[]} a
 */
export function take_i8(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_i8(ptr0, len0);
}

/**
 * @param {Int32Array | number[]} a
 */
export function take_isize(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_isize(ptr0, len0);
}

/**
 * @param {Float32Array} a
 */
export function take_mut_f32(a) {
    var ptr0 = passArrayF32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_f32(ptr0, len0, a);
}

/**
 * @param {Float64Array} a
 */
export function take_mut_f64(a) {
    var ptr0 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_f64(ptr0, len0, a);
}

/**
 * @param {Int16Array} a
 */
export function take_mut_i16(a) {
    var ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_i16(ptr0, len0, a);
}

/**
 * @param {Int32Array} a
 */
export function take_mut_i32(a) {
    var ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_i32(ptr0, len0, a);
}

/**
 * @param {BigInt64Array} a
 */
export function take_mut_i64(a) {
    var ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_i64(ptr0, len0, a);
}

/**
 * @param {Int8Array} a
 */
export function take_mut_i8(a) {
    var ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_i8(ptr0, len0, a);
}

/**
 * @param {Int32Array} a
 */
export function take_mut_isize(a) {
    var ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_isize(ptr0, len0, a);
}

/**
 * @param {Uint16Array} a
 */
export function take_mut_u16(a) {
    var ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_u16(ptr0, len0, a);
}

/**
 * @param {Uint32Array} a
 */
export function take_mut_u32(a) {
    var ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_u32(ptr0, len0, a);
}

/**
 * @param {BigUint64Array} a
 */
export function take_mut_u64(a) {
    var ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_u64(ptr0, len0, a);
}

/**
 * @param {Uint8Array} a
 */
export function take_mut_u8(a) {
    var ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_u8(ptr0, len0, a);
}

/**
 * @param {Uint32Array} a
 */
export function take_mut_usize(a) {
    var ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.take_mut_usize(ptr0, len0, a);
}

/**
 * @param {any[]} a
 * @param {string[]} b
 * @param {Point[]} c
 */
export function take_non_numbers(a, b, c) {
    const ptr0 = passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayJsValueToWasm0(b, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayJsValueToWasm0(c, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.take_non_numbers(ptr0, len0, ptr1, len1, ptr2, len2);
}

/**
 * @param {Float32Array | number[] | null | undefined} a
 * @param {number} b
 * @param {BigUint64Array | bigint[] | null} [c]
 */
export function take_optional(a, b, c) {
    var ptr0 = isLikeNone(a) ? 0 : passArrayF32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(c) ? 0 : passArray64ToWasm0(c, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    wasm.take_optional(ptr0, len0, b, ptr1, len1);
}

/**
 * @param {Uint16Array | number[]} a
 * @param {BigInt64Array | bigint[]} b
 */
export function take_owned(a, b) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.take_owned(ptr0, len0, ptr1, len1);
}

/**
 * @param {Uint16Array | number[]} a
 */
export function take_u16(a) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_u16(ptr0, len0);
}

/**
 * @param {Uint32Array | number[]} a
 */
export function take_u32(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_u32(ptr0, len0);
}

/**
 * @param {BigUint64Array | bigint[]} a
 */
export function take_u64(a) {
    const ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_u64(ptr0, len0);
}

/**
 * @param {Uint8Array | number[]} a
 */
export function take_u8(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_u8(ptr0, len0);
}

/**
 * @param {Uint32Array | number[]} a
 */
export function take_usize(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.take_usize(ptr0, len0);
}
export function __wbg___wbindgen_copy_to_typed_array_88899a52af046901(arg0, arg1, arg2) {
    new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
}
export function __wbg___wbindgen_debug_string_4687d8d8c2017d52(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_string_get_0380ccaa2f57f0d9(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_point_unwrap(arg0) {
    const ret = Point.__unwrap(arg0);
    return ret;
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
const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr, 1));
const SamplesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_samples_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function debugString(val) {
    switch (typeof val) {
        // primitive types
        case 'number':
        case 'boolean':
        case 'undefined':
        return `${val}`;
        case 'string':
        return JSON.stringify(val);
        case 'bigint':
        return `${val}n`;
        case 'symbol':
        return val.description !== undefined
            ? `Symbol(${val.description})`
            : 'Symbol';
        case 'function':
        return typeof val.name === 'string' && val.name !== ''
            ? `Function(${val.name})`
            : 'Function';
        case 'object': {
            if (val === null) {
                return 'null';
            }
            // objects
            if (Array.isArray(val)) {
                return '[' + Array.from(val, debugString).join(', ') + ']';
            }
            // errors
            if (val instanceof Error) {
                return `${val.name}: ${val.message}\n${val.stack}`;
            }

            const toStringOutput = toString.call(val);
            // Test for built-in
            const className = /\[object ([^\]]+)\]/.exec(toStringOutput)?.[1];

            // Failed to match the standard '[object ClassName]'
            if (className === undefined) {
                return toStringOutput;
            }
            if (className === 'Object') {
                // we're a user defined class or Object
                // JSON.stringify avoids problems with cycles, and is generally much
                // easier than looping through ownProperties of `val`.
                try {
                    return 'Object(' + JSON.stringify(val) + ')';
                } catch (_) {
                    return 'Object';
                }
            }
            // TODO we could test for more things here, like `Set`s and `Map`s.
            return className;
        }
        // Per MDN: 'This list of values is exhaustive.'
        // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof#description
    }
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedBigUint64ArrayMemory0 = null;
function getBigUint64ArrayMemory0() {
    if (cachedBigUint64ArrayMemory0 === null || cachedBigUint64ArrayMemory0.byteLength === 0) {
        cachedBigUint64ArrayMemory0 = new BigUint64Array(wasm.memory.buffer);
    }
    return cachedBigUint64ArrayMemory0;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint16ArrayMemory0 = null;
function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
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

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray16ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 2, 2) >>> 0;
    getUint16ArrayMemory0().set(arg, ptr / 2);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getBigUint64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
