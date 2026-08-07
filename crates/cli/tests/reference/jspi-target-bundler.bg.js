/**
 * Export returning a primitive: TypeScript becomes `(): Promise<number>`.
 * @returns {Promise<number>}
 */
export async function compute() {
    const ret = await (__wbg_jspi_compute ??= WebAssembly.promising(wasm.compute))();
    return ret >>> 0;
}

/**
 * Export returning void: wrapped with `WebAssembly.promising` in JS.
 * TypeScript signature becomes `(): Promise<void>`.
 * @returns {Promise<void>}
 */
export async function do_work() {
    await (__wbg_jspi_do_work ??= WebAssembly.promising(wasm.do_work))();
}
export function __wbg___wbindgen_throw_344f42d3211c4765(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}

export const __wbg_fetch_number_65eabd7e4b01732a = new WebAssembly.Suspending(function() {
    const ret = fetch_number();
    return ret;
});

export const __wbg_sleep_319b371bcbeaac51 = new WebAssembly.Suspending(function(arg0) {
    return sleep(arg0 >>> 0);
});
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
let __wbg_jspi_compute;

let __wbg_jspi_do_work;

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
