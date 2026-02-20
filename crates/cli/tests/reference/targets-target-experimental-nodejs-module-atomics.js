/* @ts-self-types="./reference_test.d.ts" */

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}

function __wbg_get_imports(memory) {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_83ebd457a191bc2a: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_random_6e647071acda68e7: function() {
            const ret = Math.random();
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
        memory: memory || new WebAssembly.Memory({initial:18,maximum:16384,shared:true}),
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : undefined);
if (cachedTextDecoder) cachedTextDecoder.decode();

function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
}

import { readFileSync } from 'node:fs';
import { isMainThread } from 'node:worker_threads';

let wasm;
let wasmModule;
let memory;
let __initialized = false;

export function initSync(opts = {}) {
    if (__initialized) return wasm;

    let { module, memory: mem, thread_stack_size } = opts;

    if (module === undefined) {
        const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
        module = readFileSync(wasmUrl);
    }

    if (!(module instanceof WebAssembly.Module)) {
        wasmModule = new WebAssembly.Module(module);
    } else {
        wasmModule = module;
    }

    const wasmImports = __wbg_get_imports(mem);
    const instance = new WebAssembly.Instance(wasmModule, wasmImports);
    wasm = instance.exports;
    memory = wasmImports['./reference_test_bg.js'].memory;

    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) {
        throw new Error('invalid stack size');
    }

    wasm.__wbindgen_start(thread_stack_size);
    __initialized = true;
    return wasm;
}

// Auto-initialize for backwards compatibility (only on main thread)
// Worker threads should call initSync({ module, memory }) explicitly
if (isMainThread) {
    initSync();
}

export { wasm as __wasm, wasmModule as __wbg_wasm_module, memory as __wbg_memory, __wbg_get_imports };
