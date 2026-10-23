export class Opt {
    static __wrap(ptr) {
        const obj = Object.create(Opt.prototype);
        obj.__wbg_ptr = ptr;
        OptFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
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
     * @returns {number | null}
     */
    get n() {
        const ret = wasm.__wbg_get_opt_n(this.__wbg_ptr);
        return ret === Number.MAX_SAFE_INTEGER - 1 ? null : ret;
    }
    /**
     * @returns {number | null | undefined}
     */
    get o() {
        const ret = wasm.__wbg_get_opt_o(this.__wbg_ptr);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret === Number.MAX_SAFE_INTEGER - 1 ? null : ret;
    }
    /**
     * @param {number | null} arg0
     */
    set n(arg0) {
        wasm.__wbg_set_opt_n(this.__wbg_ptr, arg0 === null ? Number.MAX_SAFE_INTEGER - 1 : (arg0) >>> 0);
    }
    /**
     * @param {number | null} [arg0]
     */
    set o(arg0) {
        wasm.__wbg_set_opt_o(this.__wbg_ptr, arg0 === undefined ? Number.MAX_SAFE_INTEGER : arg0 === null ? Number.MAX_SAFE_INTEGER - 1 : (arg0) >>> 0);
    }
}
if (Symbol.dispose) Opt.prototype[Symbol.dispose] = Opt.prototype.free;

/**
 * @param {number | null} a
 * @param {number | null} [b]
 * @param {number | null} [c]
 */
export function all_nullable(a, b, c) {
    wasm.all_nullable(a === null ? Number.MAX_SAFE_INTEGER - 1 : (a) >>> 0, b === undefined ? Number.MAX_SAFE_INTEGER : b === null ? Number.MAX_SAFE_INTEGER - 1 : (b) >>> 0, c === undefined ? Number.MAX_SAFE_INTEGER : c === null ? Number.MAX_SAFE_INTEGER - 1 : (c) >>> 0);
}

/**
 * @param {Uint8Array | number[] | null} b
 * @param {boolean | null} c
 * @param {string | null} d
 * @returns {Uint8Array | null}
 */
export function nullable_bytes(b, c, d) {
    const char1 = d === null ? 0xFFFFFE : d.codePointAt(0);
    if (char1 !== 0xFFFFFE) { _assertChar(char1); }
    var ptr0 = b === null ? 0 : passArray8ToWasm0(b, wasm.__wbindgen_malloc);
    var len0 = b === null ? 1025 : WASM_VECTOR_LEN;
    const ret = wasm.nullable_bytes(ptr0, len0, c === null ? 0xFFFFFE : c ? 1 : 0, char1);
    let v3 = null;
    if (ret[0] !== 0) {
        v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v3;
}

/**
 * @param {bigint | null} a
 * @param {bigint | null} [b]
 * @returns {bigint | null | undefined}
 */
export function nullable_i64(a, b) {
    const ret = wasm.nullable_i64(a === null ? 2 : 1, a === null ? BigInt(0) : a, b === undefined ? 0 : b === null ? 2 : 1, isLikeNone(b) ? BigInt(0) : b);
    return ret[0] === 0 ? undefined : ret[0] === 2 ? null : ret[1];
}

/**
 * @param {string | null} s
 * @returns {string | null}
 */
export function nullable_string(s) {
    var ptr0 = s === null ? 0 : passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = s === null ? 1025 : WASM_VECTOR_LEN;
    const ret = wasm.nullable_string(ptr0, len0);
    let v2 = null;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]);
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {Flag | null} f
 * @returns {Flag | null}
 */
export function nullable_string_enum(f) {
    const ret = wasm.nullable_string_enum(f === null ? 4 : ((__wbindgen_enum_Flag.indexOf(f) + 1 || 3) - 1));
    return ret === 4 ? null : __wbindgen_enum_Flag[ret];
}

/**
 * @param {Opt | null} o
 * @param {any | null} v
 * @returns {Opt | null}
 */
export function nullable_struct(o, v) {
    let ptr0 = o === null ? 1 : 0;
    if (o !== null) {
        _assertClass(o, Opt);
        ptr0 = o.__destroy_into_raw();
    }
    const ret = wasm.nullable_struct(ptr0, v === null ? 1025 : addToExternrefTable0(v));
    return ret === 1 ? null : Opt.__wrap(ret);
}

/**
 * @param {Uint8Array | number[] | null} [b]
 * @param {boolean | null} [c]
 * @param {string | null} [d]
 * @returns {Uint8Array | null | undefined}
 */
export function optional_nullable_bytes(b, c, d) {
    const char1 = d === undefined ? 0xFFFFFF : d === null ? 0xFFFFFE : d.codePointAt(0);
    if (char1 !== 0xFFFFFF && char1 !== 0xFFFFFE) { _assertChar(char1); }
    var ptr0 = isLikeNone(b) ? 0 : passArray8ToWasm0(b, wasm.__wbindgen_malloc);
    var len0 = b === undefined ? 0 : b === null ? 1025 : WASM_VECTOR_LEN;
    const ret = wasm.optional_nullable_bytes(ptr0, len0, c === undefined ? 0xFFFFFF : c === null ? 0xFFFFFE : c ? 1 : 0, char1);
    let v3 = (ret[1] === 1025 ? null : undefined);
    if (ret[0] !== 0) {
        v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v3;
}

/**
 * @param {string | null} [s]
 * @returns {string | null | undefined}
 */
export function optional_nullable_string(s) {
    var ptr0 = isLikeNone(s) ? 0 : passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = s === undefined ? 0 : s === null ? 1025 : WASM_VECTOR_LEN;
    const ret = wasm.optional_nullable_string(ptr0, len0);
    let v2 = (ret[1] === 1025 ? null : undefined);
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]);
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {Flag | null} [f]
 * @returns {Flag | null | undefined}
 */
export function optional_nullable_string_enum(f) {
    const ret = wasm.optional_nullable_string_enum(f === undefined ? 3 : f === null ? 4 : ((__wbindgen_enum_Flag.indexOf(f) + 1 || 3) - 1));
    return ret === 4 ? null : __wbindgen_enum_Flag[ret];
}

/**
 * @param {Opt | null} [o]
 * @param {any | null} [v]
 * @returns {Opt | null | undefined}
 */
export function optional_nullable_struct(o, v) {
    let ptr0 = o === null ? 1 : 0;
    if (!isLikeNone(o)) {
        _assertClass(o, Opt);
        ptr0 = o.__destroy_into_raw();
    }
    const ret = wasm.optional_nullable_struct(ptr0, v === undefined ? 0 : v === null ? 1025 : addToExternrefTable0(v));
    return ret === 0 ? undefined : ret === 1 ? null : Opt.__wrap(ret);
}

/**
 * @param {number | null | undefined} a
 * @param {number | null} b
 * @param {number | null} [c]
 */
export function some_nullable(a, b, c) {
    wasm.some_nullable(a === undefined ? Number.MAX_SAFE_INTEGER : a === null ? Number.MAX_SAFE_INTEGER - 1 : (a) >>> 0, b === null ? Number.MAX_SAFE_INTEGER - 1 : (b) >>> 0, c === undefined ? Number.MAX_SAFE_INTEGER : c === null ? Number.MAX_SAFE_INTEGER - 1 : (c) >>> 0);
}
export function __wbg___wbindgen_is_undefined_be5b34249ff1fc15(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
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

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
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
