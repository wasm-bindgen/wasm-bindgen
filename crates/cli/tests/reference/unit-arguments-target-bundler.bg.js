export class Record {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RecordFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_record_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get a() {
        const ret = wasm.__wbg_get_record_a(this.__wbg_ptr);
        return ret >>> 0;
    }
    get unit() {
        wasm.__wbg_get_record_unit(this.__wbg_ptr);
    }
    /**
     * @param {undefined} unit
     */
    set other(unit) {
        wasm.record_set_other(this.__wbg_ptr);
    }
    /**
     * @param {number} arg0
     */
    set a(arg0) {
        wasm.__wbg_set_record_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {undefined} arg0
     */
    set unit(arg0) {
        wasm.__wbg_set_record_unit(this.__wbg_ptr);
    }
}
if (Symbol.dispose) Record.prototype[Symbol.dispose] = Record.prototype.free;

export function driver() {
    wasm.driver();
}

/**
 * @param {number} a
 * @param {undefined} unit
 * @param {string[]} names
 * @param {undefined} last
 * @returns {number}
 */
export function flat(a, unit, names, last) {
    if (names.length > 268435455) throw new Error('invalid malloc request');
    for (let n0 = 0; n0 < names.length; n0++) {
        const e0 = names[n0];
        if (typeof e0 !== 'string') throw new Error('array contains a value of the wrong type');
    }
    const val4 = +a;
    let j0 = 0, len0, ptr0;
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
    const ret = wasm.flat(val4, ptr0, len0);
    return ret >>> 0;
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_call_e4ce143d755e22db(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1, arg2) => wasm_bindgen_0000000000000000___convert__closures_____invoke___u32______u32_______true_(state0.a, state0.b, arg0, arg1, arg2);
        js_call(cb0);
    } finally {
        state0.a = 0;
    }
}
export function __wbg_js_flat_271f7de6e0f1d252(arg0, arg1, arg2) {
    js_flat(arg0 >>> 0, undefined, getStringFromWasm0(arg1, arg2), undefined);
}
export function __wbg_js_rest_9eb91d9a635ba94a(arg0, arg1) {
    var v0 = getArrayU32FromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    js_rest(undefined, ...(v0));
}
export function __wbg_method_bb44ff63a2c4ea44(arg0) {
    arg0.method(undefined);
}
export function __wbg_new_259767d7b8bf0dd9(arg0) {
    const ret = new Thing(undefined, arg0 >>> 0);
    return ret;
}
export function __wbg_set_unit_a320c10b481dfdac(arg0) {
    arg0.unit = undefined;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
function wasm_bindgen_0000000000000000___convert__closures_____invoke___u32______u32_______true_(arg0, arg1, arg2, arg3, arg4) {
    wasm.wasm_bindgen_0000000000000000___convert__closures_____invoke___u32______u32_______true_(arg0, arg1, arg2, arg4);
}

const RecordFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_record_free(ptr, 1));

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
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
