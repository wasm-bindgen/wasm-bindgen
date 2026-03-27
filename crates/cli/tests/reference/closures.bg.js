export function delayed_callback() {
    const ret = wasm.delayed_callback();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function use_scoped_callback() {
    wasm.use_scoped_callback();
}

/**
 * @param {Array<any>} a
 */
export function use_stack_callback(a) {
    wasm.use_stack_callback(a);
}
export function __wbg___wbindgen_is_undefined_87a3a837f331fef5(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_5549492daedad139(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_fbe69bb076c16bad(arg0) {
    arg0._wbg_cb_unref();
}
export function __wbg_call_scoped_343c79dd3d3f172b(arg0) {
    call_scoped(arg0);
}
export function __wbg_forEach_9a13ca674064d22c(arg0, arg1, arg2) {
    try {
        var state0 = {a: arg1, b: arg2};
        var cb0 = (arg0, arg1, arg2) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen__convert__closures_____invoke__h0000000000000003(a, state0.b, arg0, arg1, arg2);
            } finally {
                state0.a = a;
            }
        };
        arg0.forEach(cb0);
    } finally {
        state0.a = 0;
    }
}
export function __wbg_instanceof_Window_2fa8d9c2d5b6104a(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_log_6a75b71d6316e935(arg0) {
    console.log(arg0);
}
export function __wbg_setTimeout_2a9750f034b9bea6() { return handleError(function (arg0, arg1) {
    const ret = arg0.setTimeout(arg1);
    return ret;
}, arguments); }
export function __wbg_static_accessor_GLOBAL_8dfb7f5e26ebe523() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_GLOBAL_THIS_941154efc8395cdd() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_SELF_58dac9af822f561f() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_WINDOW_ee64f0b3d8354c0b() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbindgen_cast_0000000000000000(arg0, arg1) {
    var state0 = {a: arg0, b: arg1};
    var cb0 = () => {
        const a = state0.a;
        state0.a = 0;
        try {
            return wasm_bindgen__convert__closures_____invoke__h0000000000000004(a, state0.b, );
        } finally {
            state0.a = a;
        }
    };
    cb0._wbg_cb_unref = () => state0.a = 0;
    // Cast intrinsic for `Closure(Closure { owned: false, function: Function { arguments: [], shim_idx: 6, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = cb0;
    return ret;
}
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 7, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h0000000000000005);
    return ret;
}
export function __wbindgen_cast_0000000000000002(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
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
function wasm_bindgen__convert__closures_____invoke__h0000000000000004(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000004(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000005(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000005(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000003(arg0, arg1, arg2, arg3, arg4) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000003(arg0, arg1, arg2, arg3, arg4);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

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

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
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
