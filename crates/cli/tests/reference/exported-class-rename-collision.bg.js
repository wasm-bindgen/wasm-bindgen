export class BarPoint {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BarPointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_barpoint_free(ptr, 0);
    }
    /**
     * @param {number} y
     */
    constructor(y) {
        const ret = wasm.barpoint_new(y);
        this.__wbg_ptr = ret;
        BarPointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.barpoint_y(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) BarPoint.prototype[Symbol.dispose] = BarPoint.prototype.free;

export class FooPoint {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FooPointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foopoint_free(ptr, 0);
    }
    /**
     * @param {number} x
     */
    constructor(x) {
        const ret = wasm.foopoint_new(x);
        this.__wbg_ptr = ret;
        FooPointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.foopoint_x(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) FooPoint.prototype[Symbol.dispose] = FooPoint.prototype.free;
export function __wbg___wbindgen_throw_ea4887a5f8f9a9db(arg0, arg1) {
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
const BarPointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_barpoint_free(ptr, 1));
const FooPointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foopoint_free(ptr, 1));

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
