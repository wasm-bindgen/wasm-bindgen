
let imports = {};
imports['./reference_test_bg.js'] = module.exports;

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

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}
exports.add_that_might_fail = add_that_might_fail;

exports.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

exports.__wbg_random_e2b253f0e987bd7c = function() {
    const ret = Math.random();
    return ret;
};

exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
};

exports.memory = new WebAssembly.Memory({initial:18,maximum:16384,shared:true});

let __wbg_memory;
exports.__wbg_get_imports = function(customMemory) {
    __wbg_memory = customMemory !== undefined ? customMemory : new WebAssembly.Memory({initial:18,maximum:16384,shared:true});
    const imports = {};
    const mod = {};
    for (const key of Object.keys(exports)) {
        if (key.startsWith('__wbg_') || key.startsWith('__wbindgen_')) {
            mod[key] = exports[key];
        }
    }
    mod.memory = __wbg_memory;
    imports['./reference_test_bg.js'] = mod;
    return imports;
};

let wasm;
let wasmModule;
let __initialized = false;

exports.initSync = function(opts) {
    if (opts === undefined) opts = {};
    if (__initialized) return wasm;

    let module = opts.module;
    let memory = opts.memory;
    let thread_stack_size = opts.thread_stack_size;

    if (module === undefined) {
        const wasmPath = `${__dirname}/reference_test_bg.wasm`;
        module = require('fs').readFileSync(wasmPath);
    }

    if (!(module instanceof WebAssembly.Module)) {
        wasmModule = new WebAssembly.Module(module);
    } else {
        wasmModule = module;
    }

    const wasmImports = exports.__wbg_get_imports(memory);
    const instance = new WebAssembly.Instance(wasmModule, wasmImports);
    wasm = instance.exports;
    exports.__wasm = wasm;
    exports.__wbindgen_wasm_module = wasmModule;
    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) { throw new Error('invalid stack size'); }
    wasm.__wbindgen_start(thread_stack_size);

    __initialized = true;
    return wasm;
};

// Auto-initialize for backwards compatibility (only on main thread)
// Worker threads should call initSync({ module, memory }) explicitly
if (require('worker_threads').isMainThread) {
    exports.initSync();
}
