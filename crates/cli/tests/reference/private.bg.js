/**
 * A hidden enum that is not exported
 * @enum {0 | 1}
 */
const HiddenEnum = Object.freeze({
    Variant1: 0, "0": "Variant1",
    Variant2: 1, "1": "Variant2",
});

/**
 * A hidden struct that is not exported but can be used as an argument type
 */
class HiddenStruct {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HiddenStructFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hiddenstruct_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get value() {
        const ret = wasm.__wbg_get_hiddenstruct_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set value(arg0) {
        wasm.__wbg_set_hiddenstruct_value(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) HiddenStruct.prototype[Symbol.dispose] = HiddenStruct.prototype.free;

/**
 * A public enum that is exported
 * @enum {0 | 1}
 */
export const PublicEnum = Object.freeze({
    A: 0, "0": "A",
    B: 1, "1": "B",
});

/**
 * A public struct that is exported
 */
export class PublicStruct {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PublicStruct.prototype);
        obj.__wbg_ptr = ptr;
        PublicStructFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PublicStructFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_publicstruct_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get value() {
        const ret = wasm.__wbg_get_publicstruct_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set value(arg0) {
        wasm.__wbg_set_publicstruct_value(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) PublicStruct.prototype[Symbol.dispose] = PublicStruct.prototype.free;

/**
 * Function that returns a public struct
 * @returns {PublicStruct}
 */
export function get_public_struct() {
    const ret = wasm.get_public_struct();
    return PublicStruct.__wrap(ret);
}

class NamespacedHidden {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NamespacedHidden.prototype);
        obj.__wbg_ptr = ptr;
        NamespacedHiddenFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NamespacedHiddenFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_namespacedhidden_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get data() {
        const ret = wasm.__wbg_get_namespacedhidden_data(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set data(arg0) {
        wasm.__wbg_set_namespacedhidden_data(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) NamespacedHidden.prototype[Symbol.dispose] = NamespacedHidden.prototype.free;

/**
 * @returns {NamespacedHidden}
 */
function create_namespaced() {
    const ret = wasm.internal_create_namespaced();
    return NamespacedHidden.__wrap(ret);
}

export const internal = {};
internal.NamespacedHidden = NamespacedHidden;
internal.create_namespaced = create_namespaced;

/**
 * Function that takes a hidden enum as an argument
 * @param {HiddenEnum} hidden
 * @returns {number}
 */
export function use_hidden_enum(hidden) {
    const ret = wasm.use_hidden_enum(hidden);
    return ret;
}

/**
 * Function that takes a hidden struct as an argument
 * @param {HiddenStruct} hidden
 * @returns {number}
 */
export function use_hidden_struct(hidden) {
    _assertClass(hidden, HiddenStruct);
    var ptr0 = hidden.__destroy_into_raw();
    const ret = wasm.use_hidden_struct(ptr0);
    return ret;
}
export function __wbg___wbindgen_throw_83ebd457a191bc2a(arg0, arg1) {
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
const HiddenStructFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_hiddenstruct_free(ptr >>> 0, 1));
const NamespacedHiddenFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_namespacedhidden_free(ptr >>> 0, 1));
const PublicStructFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_publicstruct_free(ptr >>> 0, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
