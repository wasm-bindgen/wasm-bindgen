export class Renamed {
    static __wrap(ptr) {
        const obj = Object.create(Renamed.prototype);
        obj.__wbg_ptr = ptr;
        RenamedFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RenamedFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_renamed_free(ptr, 0);
    }
    /**
     * @param {number} value
     */
    constructor(value) {
        const ret = wasm.renamed_new(value);
        this.__wbg_ptr = ret;
        RenamedFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get value() {
        const ret = wasm.renamed_value(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) Renamed.prototype[Symbol.dispose] = Renamed.prototype.free;

/**
 * @param {number} value
 * @returns {any}
 */
export function makeRenamed(value) {
    const ret = wasm.makeRenamed(value);
    return ret;
}

/**
 * @param {Renamed[]} renameds
 * @returns {number}
 */
export function readRenameds(renameds) {
    for (let n0 = 0; n0 < renameds.length; n0++) {
        const e0 = renameds[n0];
        if (!(e0 instanceof Renamed)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Renamed);
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = renameds.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = renameds[j0];
            if (!(e0 instanceof Renamed)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, Renamed);
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_renamed_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readRenameds(ptr0, len0);
    return ret >>> 0;
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_renamed_new(arg0) {
    const ret = Renamed.__wrap(arg0);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
const RenamedFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_renamed_free(ptr, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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
