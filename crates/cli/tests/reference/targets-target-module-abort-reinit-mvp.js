/* @ts-self-types="./reference_test.d.ts" */

function __wbg_reset_state () {
    __wbg_instance_id++;
    __wbg_aborted = false;
    cachedUint8ArrayMemory0 = null;
    if (typeof numBytesDecoded !== 'undefined') numBytesDecoded = 0;
    if (typeof heap !== 'undefined') {
        heap = new Array(128).fill(undefined);
        heap = heap.concat([undefined, null, true, false]);
        if (typeof heap_next !== 'undefined')
        heap_next = heap.length;
    }

    const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
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

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_be289d5034ed271b: function() { return handleError(function (arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_random_9526caf33df4270d: function() { return handleError(function () {
            const ret = Math.random();
            return ret;
        }, arguments); },
        __wbindgen_object_drop_ref: function() { return handleError(function (arg0) {
            takeObject(arg0);
        }, arguments); },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

let __wbg_instance_id = 0;

let __wbg_aborted = false;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

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

function getObject(idx) { return heap[idx]; }

let heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

import source wasmModule from "./reference_test_bg.wasm";
const wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;
