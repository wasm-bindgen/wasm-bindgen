/* @ts-self-types="./reference_test.d.ts" */

function __wbg_reset_state () {
    __wbg_instance_id++;
    __wbg_aborted = false;
    cachedUint8ArrayMemory0 = null;
    if (typeof numBytesDecoded !== 'undefined') numBytesDecoded = 0;

    const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add_that_might_fail(a, b) {
    let ret;
    if (__wbg_aborted) {
        __wbg_reset_state();
    }
    try {
        ret = wasm.add_that_might_fail(a, b);
    } catch(e) {
        debugger;
        console.log('ABORT', e);
        // wasm.__wbindgen_set_abort_flag(1);
        // __wbg_aborted = true;
        throw e;
    }
    return ret >>> 0;
}
exports.add_that_might_fail = add_that_might_fail;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_be289d5034ed271b: function() { return handleError(function (arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_random_ae0b2256206ad108: function() { return handleError(function () {
            const ret = Math.random();
            return ret;
        }, arguments); },
        __wbindgen_init_externref_table: function() { return handleError(function () {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        }, arguments); },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

let __wbg_instance_id = 0;

let __wbg_aborted = false;

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

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
let wasm = new WebAssembly.Instance(wasmModule, __wbg_get_imports()).exports;
wasm.__wbindgen_start();
