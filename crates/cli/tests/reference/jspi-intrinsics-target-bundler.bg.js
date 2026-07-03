/**
 * @param {any} promise
 * @returns {Promise<any>}
 */
export async function drive(promise) {
    if (__jspi_sync_sp === undefined) __jspi_sync_sp = wasm.__stack_pointer.value;
    else wasm.__stack_pointer.value = __jspi_sync_sp;
    const __jspi_stack = __jspi_stack_alloc();
    __jspi_active_floor = __jspi_stack + __jspi_guard_size;
    wasm.__stack_pointer.value = __jspi_stack + __jspi_stack_size;
    try {
        const ret = await (__wbg_jspi_drive ??= WebAssembly.promising(wasm.drive))(promise);
        return ret;
    } finally {
        wasm.__stack_pointer.value = __jspi_sync_sp;
        __jspi_stack_free(__jspi_stack);
        __jspi_active_floor = 0;
    }
}
export function __wbg___wbindgen_jspi_cleanup_485cc4f59821d0c3(arg0) {
    _jspiResolved[arg0 >>> 0] = undefined;
    _jspiRejected[arg0 >>> 0] = false;
    _jspiPending[arg0 >>> 0] = undefined;
}
export function __wbg___wbindgen_jspi_get_resolved_ae482262158d0e4e(arg0) {
    const ret = _jspiResolved[arg0 >>> 0];
    return ret;
}
export function __wbg___wbindgen_jspi_is_rejected_ad8e8b2df7ac1bf8(arg0) {
    const ret = _jspiRejected[arg0 >>> 0];
    return ret;
}
export function __wbg___wbindgen_jspi_set_pending_eb0737ebb950d87d(arg0, arg1) {
    _jspiPending[arg0 >>> 0] = arg1;
}

export const __wbg___wbindgen_jspi_suspend_a72c0d026c006e17 = ((__inner) => new WebAssembly.Suspending(async function(...args) {
    const __sp = wasm.__stack_pointer.value;
    const __floor = __jspi_active_floor;
    if (__sp <= __floor) throw new RangeError('JSPI fiber stack overflow');
    try { return await __inner(...args); }
    finally { wasm.__stack_pointer.value = __sp; __jspi_active_floor = __floor; }
}))(function(arg0) {
    return _jspiPending[arg0 >>> 0].then(v => { _jspiRejected[arg0 >>> 0] = false; _jspiResolved[arg0 >>> 0] = v; }, e => { _jspiRejected[arg0 >>> 0] = true; _jspiResolved[arg0 >>> 0] = e; });
});
export function __wbg___wbindgen_jspi_waker_cleanup_e24dd9d90266971f(arg0) {
    _jspiWakerMap.delete(arg0 >>> 0);
}
export function __wbg___wbindgen_jspi_waker_create_ea0ae813d51107c9(arg0) {
    const ret = new Promise(resolve => _jspiWakerMap.set(arg0 >>> 0, resolve));
    return ret;
}
export function __wbg___wbindgen_jspi_waker_wake_18ccfe9f06cb07b3(arg0) {
    const resolve = _jspiWakerMap.get(arg0 >>> 0);
    resolve && resolve();
}
export function __wbg___wbindgen_throw_344f42d3211c4765(arg0, arg1) {
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
let __wbg_jspi_drive;

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

const _jspiPending = [];
const _jspiResolved = [];
const _jspiRejected = [];
const _jspiWakerMap = new Map();

let __jspi_sync_sp;
let __jspi_active_floor = 0;
const __jspi_stack_size = 65536;
const __jspi_guard_size = 8192;
const __jspi_stack_pool = [];
function __jspi_stack_alloc() {
    if (__jspi_stack_pool.length > 0) return __jspi_stack_pool.pop();
    const ptr = wasm.memory.grow(1);
    if (ptr === -1) throw new RangeError('out of memory allocating JSPI fiber stack');
    return ptr * 65536;
}
function __jspi_stack_free(ptr) { __jspi_stack_pool.push(ptr); }

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
