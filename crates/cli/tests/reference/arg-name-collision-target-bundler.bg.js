export class Foo {
    static __wrap(ptr) {
        const obj = Object.create(Foo.prototype);
        obj.__wbg_ptr = ptr;
        FooFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FooFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo_free(ptr, 0);
    }
    /**
     * Only appears in an error message of the debug glue, so stays unchanged.
     * @param {number} value
     * @returns {number}
     */
    add(value) {
        const ret = wasm.foo_add(this.__wbg_ptr, value);
        return ret >>> 0;
    }
    /**
     * Named like the `ptr` local of methods taking `self` by value.
     * @param {number} ptr2
     * @returns {number}
     */
    consume(ptr2) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.foo_consume(ptr, ptr2);
        return ret >>> 0;
    }
    /**
     * @param {number} value
     */
    constructor(value) {
        const ret = wasm.foo_new(value);
        this.__wbg_ptr = ret;
        FooFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) Foo.prototype[Symbol.dispose] = Foo.prototype.free;

/**
 * Named like a class the generated function body refers to.
 * @param {Foo} Foo2
 * @returns {Foo}
 */
export function class_arg(Foo2) {
    _assertClass(Foo2, Foo);
    const ret = wasm.class_arg(Foo2.__wbg_ptr);
    return Foo.__wrap(ret);
}

/**
 * Named like a helper function and a JS global the body references.
 * @param {number | null | undefined} isLikeNone2
 * @param {number} undefined2
 * @returns {number | undefined}
 */
export function global_args(isLikeNone2, undefined2) {
    const ret = wasm.global_args(isLikeNone(isLikeNone2) ? Number.MAX_SAFE_INTEGER : (isLikeNone2) >>> 0, undefined2);
    return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
}

/**
 * Named like locals of the generated function body.
 * @param {string} ptr02
 * @param {number} len02
 * @param {number} ret2
 * @returns {string}
 */
export function local_args(ptr02, len02, ret2) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(ptr02, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.local_args(ptr0, len0, len02, ret2);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Named like the module-level `wasm` object; `wasm2` is already taken.
 * @param {Uint8Array} wasm3
 * @param {number} wasm2
 * @returns {string}
 */
export function wasm_args(wasm3, wasm2) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passArray8ToWasm0(wasm3, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_args(ptr0, len0, wasm2);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}
export function __wbg___wbindgen_throw_5d9e815e6fdf150f(arg0, arg1) {
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
const FooFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo_free(ptr, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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
