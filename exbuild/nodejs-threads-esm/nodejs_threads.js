/* @ts-self-types="./nodejs_threads.d.ts" */

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a, b) {
    const ret = wasm.add(a, b);
    return ret >>> 0;
}

/**
 * @param {number} size
 * @returns {number}
 */
export function allocate_and_sum(size) {
    const ret = wasm.allocate_and_sum(size);
    return ret >>> 0;
}

/**
 * @returns {number}
 */
export function get_counter() {
    const ret = wasm.get_counter();
    return ret >>> 0;
}

/**
 * @returns {number}
 */
export function increment() {
    const ret = wasm.increment();
    return ret >>> 0;
}

function __wbg_get_imports(memory) {
    const import0 = {
        __proto__: null,
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
        "./nodejs_threads_bg.js": import0,
    };
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
        const wasmUrl = new URL('nodejs_threads_bg.wasm', import.meta.url);
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
    memory = wasmImports['./nodejs_threads_bg.js'].memory;

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
