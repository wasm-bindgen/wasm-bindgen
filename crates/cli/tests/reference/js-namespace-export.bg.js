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
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {
    const ret = wasm.add(a, b);
    return ret;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function multiply(a, b) {
    const ret = wasm.multiply(a, b);
    return ret;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
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
/**
 * @param {string} a
 * @param {string} b
 * @returns {string}
 */
function concat(a, b) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passStringToWasm0(a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(b, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.concat(ptr0, len0, ptr1, len1);
        deferred3_0 = ret[0];
        deferred3_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @param {string} s
 * @returns {string}
 */
function uppercase(s) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.uppercase(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function divide(a, b) {
    const ret = wasm.divide(a, b);
    return ret;
}

/**
 * @returns {number}
 */
export function regular_function() {
    const ret = wasm.regular_function();
    return ret;
}

/**
 * @enum {0 | 1 | 2}
 */
export const Color = Object.freeze({
    Red: 0, "0": "Red",
    Green: 1, "1": "Green",
    Blue: 2, "2": "Blue",
});
/**
 * @enum {200 | 404 | 500}
 */
const HttpStatus = Object.freeze({
    Ok: 200, "200": "Ok",
    NotFound: 404, "404": "NotFound",
    ServerError: 500, "500": "ServerError",
});
/**
 * @enum {0 | 1 | 2}
 */
const Status = Object.freeze({
    Pending: 0, "0": "Pending",
    Active: 1, "1": "Active",
    Complete: 2, "2": "Complete",
});

const CounterFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_counter_free(ptr >>> 0, 1));

class Counter {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CounterFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_counter_free(ptr, 0);
    }
    /**
     * @param {number} initial
     */
    constructor(initial) {
        const ret = wasm.counter_new(initial);
        this.__wbg_ptr = ret >>> 0;
        CounterFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get value() {
        const ret = wasm.counter_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} val
     */
    set value(val) {
        wasm.counter_set_value(this.__wbg_ptr, val);
    }
    increment() {
        wasm.counter_increment(this.__wbg_ptr);
    }
}
if (Symbol.dispose) Counter.prototype[Symbol.dispose] = Counter.prototype.free;

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr >>> 0, 1));

class Point {

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
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_point_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

const Point3DFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point3d_free(ptr >>> 0, 1));

class Point3D {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Point3DFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point3d_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_point3d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point3d_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_point3d_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_point3d_y(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_point3d_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_point3d_z(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point3D.prototype[Symbol.dispose] = Point3D.prototype.free;

const RectangleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rectangle_free(ptr >>> 0, 1));

export class Rectangle {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RectangleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rectangle_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get width() {
        const ret = wasm.__wbg_get_rectangle_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set width(arg0) {
        wasm.__wbg_set_rectangle_width(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get height() {
        const ret = wasm.__wbg_get_rectangle_height(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set height(arg0) {
        wasm.__wbg_set_rectangle_height(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Rectangle.prototype[Symbol.dispose] = Rectangle.prototype.free;

export default {
    Counter,
    concat,
};

export const math = {
    add,
    divide,
    multiply,
};

export const models = {
    '3d': {
        Point3D,
    },
    Point,
};

export const types = {
    Status,
    http: {
        HttpStatus,
    },
};

export const utils = {
    string: {
        uppercase,
    },
};

export function __wbg___wbindgen_throw_b855445ff6a94295(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
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

