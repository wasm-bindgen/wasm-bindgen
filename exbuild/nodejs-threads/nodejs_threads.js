/* @ts-self-types="./nodejs_threads.d.ts" */

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {
    const ret = wasm.add(a, b);
    return ret >>> 0;
}
exports.add = add;

/**
 * @param {number} size
 * @returns {number}
 */
function allocate_and_sum(size) {
    const ret = wasm.allocate_and_sum(size);
    return ret >>> 0;
}
exports.allocate_and_sum = allocate_and_sum;

/**
 * @returns {number}
 */
function get_counter() {
    const ret = wasm.get_counter();
    return ret >>> 0;
}
exports.get_counter = get_counter;

/**
 * @returns {number}
 */
function increment() {
    const ret = wasm.increment();
    return ret >>> 0;
}
exports.increment = increment;

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

let wasm;
let wasmModule;
let memory;
let __initialized = false;

// Export __wbg_get_imports for workers to use
exports.__wbg_get_imports = __wbg_get_imports;

exports.initSync = function(opts) {
    if (opts === undefined) opts = {};
    if (__initialized) return wasm;

    let module = opts.module;
    let mem = opts.memory;
    let thread_stack_size = opts.thread_stack_size;

    if (module === undefined) {
        const wasmPath = `${__dirname}/nodejs_threads_bg.wasm`;
        module = require('fs').readFileSync(wasmPath);
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
    exports.__wasm = wasm;
    exports.__wbg_wasm_module = wasmModule;
    exports.__wbg_memory = memory;

    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) {
        throw new Error('invalid stack size');
    }

    wasm.__wbindgen_start(thread_stack_size);
    __initialized = true;
    return wasm;
};

// Auto-initialize for backwards compatibility (only on main thread)
// Worker threads should call initSync({ module, memory }) explicitly
if (require('worker_threads').isMainThread) {
    exports.initSync();
}
