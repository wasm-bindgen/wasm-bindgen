export class Opt {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        OptFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_opt_free(ptr, 0);
    }
    /**
     * @returns {number | undefined}
     */
    get n() {
        const ret = wasm.__wbg_get_opt_n(this.__wbg_ptr);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set n(arg0) {
        wasm.__wbg_set_opt_n(this.__wbg_ptr, isLikeNone(arg0) ? Number.MAX_SAFE_INTEGER : (arg0) >>> 0);
    }
}
if (Symbol.dispose) Opt.prototype[Symbol.dispose] = Opt.prototype.free;

/**
 * @param {number | null} [a]
 * @param {number | null} [b]
 * @param {number | null} [c]
 */
export function all_optional(a, b, c) {
    wasm.all_optional(isLikeNone(a) ? Number.MAX_SAFE_INTEGER : (a) >>> 0, isLikeNone(b) ? Number.MAX_SAFE_INTEGER : (b) >>> 0, isLikeNone(c) ? Number.MAX_SAFE_INTEGER : (c) >>> 0);
}

/**
 * @param {Uint8Array | number[] | null} [b]
 * @param {boolean | null} [c]
 * @param {string | null} [d]
 */
export function optional_bytes(b, c, d) {
    const char1 = isLikeNone(d) ? 0xFFFFFF : d.codePointAt(0);
    if (char1 !== 0xFFFFFF) { _assertChar(char1); }
    var ptr0 = isLikeNone(b) ? 0 : passArray8ToWasm0(b, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.optional_bytes(ptr0, len0, isLikeNone(c) ? 0xFFFFFF : c ? 1 : 0, char1);
}

/**
 * @param {string | null} [s]
 * @returns {string | undefined}
 */
export function optional_string(s) {
    var ptr0 = isLikeNone(s) ? 0 : passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.optional_string(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]);
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {Flag | null} [f]
 */
export function optional_string_enum(f) {
    wasm.optional_string_enum(isLikeNone(f) ? 3 : ((__wbindgen_enum_Flag.indexOf(f) + 1 || 3) - 1));
}

/**
 * @param {Opt | null} [o]
 * @param {any | null} [v]
 */
export function optional_struct(o, v) {
    let ptr0 = 0;
    if (!isLikeNone(o)) {
        _assertClass(o, Opt);
        ptr0 = o.__destroy_into_raw();
    }
    wasm.optional_struct(ptr0, isLikeNone(v) ? 0 : addToExternrefTable0(v));
}

/**
 * @param {number | null | undefined} a
 * @param {number} b
 * @param {number | null} [c]
 */
export function some_optional(a, b, c) {
    wasm.some_optional(isLikeNone(a) ? Number.MAX_SAFE_INTEGER : (a) >>> 0, b, isLikeNone(c) ? Number.MAX_SAFE_INTEGER : (c) >>> 0);
}
export function __wbg___wbindgen_is_undefined_8865fb403f8fe9d8(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
const __wbindgen_enum_Flag = ["yes", "no"];
const OptFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_opt_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertChar(c) {
    if (typeof(c) === 'number' && (c >= 0x110000 || (c >= 0xD800 && c < 0xE000))) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
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
