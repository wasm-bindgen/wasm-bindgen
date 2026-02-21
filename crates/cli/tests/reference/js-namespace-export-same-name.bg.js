class Point {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bar__point_free(ptr, 0);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    constructor(x, y) {
        const ret = wasm.barpoint_new(x, y);
        this.__wbg_ptr = ret >>> 0;
        PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_bar__point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_bar__point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_bar__point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_bar__point_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

/**
 * @enum {0 | 1 | 2}
 */
const Status = Object.freeze({
    Pending: 0, "0": "Pending",
    Complete: 1, "1": "Complete",
    Failed: 2, "2": "Failed",
});

/**
 * @returns {string}
 */
function greet() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.bar__greet();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export const bar = {};
bar.Point = Point;
bar.Status = Status;
bar.greet = greet;

/**
 * Two structs with the same js_name in different namespaces should not collide.
 */
class Point2 {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Point2Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo__point_free(ptr, 0);
    }
    /**
     * @param {number} x
     */
    constructor(x) {
        const ret = wasm.foopoint_new(x);
        this.__wbg_ptr = ret >>> 0;
        Point2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_foo__point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_foo__point_x(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point2.prototype[Symbol.dispose] = Point2.prototype.free;

/**
 * Two enums with the same js_name in different namespaces should not collide.
 * @enum {0 | 1}
 */
const Status2 = Object.freeze({
    Active: 0, "0": "Active",
    Inactive: 1, "1": "Inactive",
});

/**
 * Two functions with the same js_name in different namespaces should not collide.
 * @returns {string}
 */
function greet2() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.foo__greet();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export const foo = {};
foo.Point = Point2;
foo.Status = Status2;
foo.greet = greet2;
export function __wbg___wbindgen_throw_39bc967c0e5a9b58(arg0, arg1) {
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
const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__point_free(ptr >>> 0, 1));
const Point2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__point_free(ptr >>> 0, 1));

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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
