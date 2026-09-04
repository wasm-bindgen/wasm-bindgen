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
export function __wbg___wbindgen_is_undefined_8c687d0b90d5b524(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_5d9e815e6fdf150f(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_997e73d32238e655(arg0) {
    arg0._wbg_cb_unref();
}
export function __wbg_call_scoped_343c79dd3d3f172b(arg0) {
    call_scoped(arg0);
}
export function __wbg_forEach_a94541f60e671002(arg0, arg1, arg2) {
    try {
        var state0 = {a: arg1, b: arg2};
        var cb0 = (arg0, arg1, arg2) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen_0000000000000003___convert__closures_____invoke___wasm_bindgen_0000000000000003___JsValue__u32__js_sys_0000000000000004___Array______true_(a, state0.b, arg0, arg1, arg2);
            } finally {
                state0.a = a;
            }
        };
        arg0.forEach(cb0);
    } finally {
        state0.a = 0;
    }
}
export function __wbg_instanceof_Window_a3b8566f0a9c5d1a(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_log_363d83b9114c8831(arg0) {
    console.log(arg0);
}
export function __wbg_setTimeout_f511da46f7279b2b() { return handleError(function (arg0, arg1) {
    const ret = arg0.setTimeout(arg1);
    return ret;
}, arguments); }
export function __wbg_static_accessor_GLOBAL_8eb4cd83130a11a0() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_GLOBAL_THIS_1e7044f654e934db() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_SELF_d8b50611246a6d92() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_WINDOW_fd0bc376bf0f8b42() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbindgen_generic_0000000000000000(arg0, arg1) {
    var state0 = {a: arg0, b: arg1};
    var cb0 = () => {
        const a = state0.a;
        state0.a = 0;
        try {
            return wasm_bindgen_0000000000000003___convert__closures_____invoke_______false_(a, state0.b, );
        } finally {
            state0.a = a;
        }
    };
    cb0._wbg_cb_unref = () => state0.a = 0;
    // Cast intrinsic for `Closure(Closure { owned: false, function: Function { arguments: [], shim_idx: 5, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = cb0;
    return ret;
}
export function __wbindgen_generic_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 6, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen_0000000000000003___convert__closures_____invoke_______true_);
    return ret;
}
export function __wbindgen_generic_0000000000000002(arg0, arg1) {
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
function wasm_bindgen_0000000000000003___convert__closures_____invoke_______false_(arg0, arg1) {
    wasm.wasm_bindgen_0000000000000003___convert__closures_____invoke_______false_(arg0, arg1);
}

function wasm_bindgen_0000000000000003___convert__closures_____invoke_______true_(arg0, arg1) {
    wasm.wasm_bindgen_0000000000000003___convert__closures_____invoke_______true_(arg0, arg1);
}

function wasm_bindgen_0000000000000003___convert__closures_____invoke___wasm_bindgen_0000000000000003___JsValue__u32__js_sys_0000000000000004___Array______true_(arg0, arg1, arg2, arg3, arg4) {
    wasm.wasm_bindgen_0000000000000003___convert__closures_____invoke___wasm_bindgen_0000000000000003___JsValue__u32__js_sys_0000000000000004___Array______true_(arg0, arg1, arg2, arg3, arg4);
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
    return decodeText(ptr >>> 0, len);
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
