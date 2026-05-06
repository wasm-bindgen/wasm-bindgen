/**
 * @param {number} x
 * @returns {number}
 */
export function maybe_panic(x) {
    let ret;
    __wbg_call_guard();
    try {
        ret = wasm.maybe_panic(x);
    } catch(e) {
        __wbg_handle_catch(e);
    }
    return ret >>> 0;
}
export function __wbg___wbindgen_panic_error_a4429721aaf96b50(arg0) {
    const ret = new PanicError(arg0);
    return ret;
}
export function __wbg___wbindgen_rethrow_8e609956a7b9f4fb(arg0) {
    throw new WebAssembly.Exception(__wbindgen_wrapped_jstag, [arg0]);
}
export function __wbg___wbindgen_throw_9c75d47bf9e7731e(arg0, arg1) {
    throw new WebAssembly.Exception(__wbindgen_wrapped_jstag, [new Error(getStringFromWasm0(arg0, arg1))]);
}
export function __wbindgen_cast_0000000000000000(arg0, arg1) {
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

export const __wbindgen_jstag = WebAssembly.JSTag;

export { __wbindgen_wrapped_jstag };
const __wbindgen_wrapped_jstag = new WebAssembly.Tag({ parameters: ['externref'] });


let __wbg_terminated_addr;
let __wbg_called_abort = false;
function __wbg_call_abort_hook() {
    __wbg_called_abort = true;
    try {
        const idx = getInt32ArrayMemory0()[wasm.__abort_handler.value / 4];
        if (idx) wasm.__wbindgen_export.get(idx)();
    } catch(_) {}
}

function __wbg_handle_catch(e) {
    if (e instanceof WebAssembly.Exception && e.is(__wbindgen_wrapped_jstag)) {
        throw e.getArg(__wbindgen_wrapped_jstag, 0);
    }
    getInt32ArrayMemory0()[__wbg_terminated_addr] = 1;
    __wbg_call_abort_hook();
    throw e;
}


function __wbg_call_guard() {
    __wbg_terminated_addr ??= wasm.__instance_terminated.value / 4;
    const flag = getInt32ArrayMemory0()[__wbg_terminated_addr];
    if (flag) {
        if (!__wbg_called_abort) {
            __wbg_call_abort_hook();
        }throw new Error('Module terminated');
    }
}

let cachedInt32ArrayMemory0 = null;
function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
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

class PanicError extends Error {}
Object.defineProperty(PanicError.prototype, 'name', {
    value: PanicError.name,
});

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
