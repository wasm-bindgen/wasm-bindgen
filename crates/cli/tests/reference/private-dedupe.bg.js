/**
 * @enum {0 | 1}
 */
const Level = Object.freeze({
    Low: 0, "0": "Low",
    High: 1, "1": "High",
});

/**
 * @enum {0 | 1}
 */
const Level2 = Object.freeze({
    Low: 0, "0": "Low",
    High: 1, "1": "High",
});

class Status {
    static __wrap(ptr) {
        const obj = Object.create(Status.prototype);
        obj.__wbg_ptr = ptr;
        StatusFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatusFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_status_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get code() {
        const ret = wasm.__wbg_get_status_code(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set code(arg0) {
        wasm.__wbg_set_status_code(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    describe() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.status_describe(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Status.prototype[Symbol.dispose] = Status.prototype.free;

class Status2 {
    static __wrap(ptr) {
        const obj = Object.create(Status2.prototype);
        obj.__wbg_ptr = ptr;
        Status2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Status2Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_status2_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get code() {
        const ret = wasm.__wbg_get_status2_code(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set code(arg0) {
        wasm.__wbg_set_status2_code(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    describe() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.status2_describe(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Status2.prototype[Symbol.dispose] = Status2.prototype.free;

/**
 * @returns {Level[]}
 */
export function a_levels() {
    const ret = wasm.a_levels();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]);
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @returns {Level2[]}
 */
export function b_levels() {
    const ret = wasm.b_levels();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]);
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @returns {Status2[]}
 */
export function b_statuses() {
    const ret = wasm.b_statuses();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]);
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @returns {any[]}
 */
export function statuses() {
    const ret = wasm.statuses();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]);
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}
export function __wbg___wbindgen_throw_5d9e815e6fdf150f(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_status2_new(arg0) {
    const ret = Status2.__wrap(arg0);
    return ret;
}
export function __wbg_status_new(arg0) {
    const ret = Status.__wrap(arg0);
    return ret;
}
export function __wbindgen_generic_0000000000000000(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
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
const StatusFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_status_free(ptr, 1));
const Status2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_status2_free(ptr, 1));

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
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
