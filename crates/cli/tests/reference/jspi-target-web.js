/* @ts-self-types="./reference_test.d.ts" */

/**
 * Export returning a primitive: TypeScript becomes `(): Promise<number>`.
 * @returns {Promise<number>}
 */
export async function compute() {
    if (__jspi_sync_sp === undefined) __jspi_sync_sp = wasm.__stack_pointer.value;
    else wasm.__stack_pointer.value = __jspi_sync_sp;
    const __jspi_stack = __jspi_stack_alloc();
    __jspi_active_floor = __jspi_stack + __jspi_guard_size;
    wasm.__stack_pointer.value = __jspi_stack + __jspi_stack_size;
    try {
        const ret = await (__wbg_jspi_compute ??= WebAssembly.promising(wasm.compute))();
        return ret >>> 0;
    } finally {
        wasm.__stack_pointer.value = __jspi_sync_sp;
        __jspi_stack_free(__jspi_stack);
        __jspi_active_floor = 0;
    }
}

/**
 * Export returning void: wrapped with `WebAssembly.promising` in JS.
 * TypeScript signature becomes `(): Promise<void>`.
 * @returns {Promise<void>}
 */
export function do_work() {
    if (__jspi_sync_sp === undefined) __jspi_sync_sp = wasm.__stack_pointer.value;
    else wasm.__stack_pointer.value = __jspi_sync_sp;
    const __jspi_stack = __jspi_stack_alloc();
    __jspi_active_floor = __jspi_stack + __jspi_guard_size;
    wasm.__stack_pointer.value = __jspi_stack + __jspi_stack_size;
    return (__wbg_jspi_do_work ??= WebAssembly.promising(wasm.do_work))().finally(() => { wasm.__stack_pointer.value = __jspi_sync_sp; __jspi_stack_free(__jspi_stack); __jspi_active_floor = 0; });
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_fetch_number_65eabd7e4b01732a: ((__inner) => new WebAssembly.Suspending(async function(...args) {
            const __sp = wasm.__stack_pointer.value;
            const __floor = __jspi_active_floor;
            if (__sp <= __floor) throw new RangeError('JSPI fiber stack overflow');
            try { return await __inner(...args); }
            finally { wasm.__stack_pointer.value = __sp; __jspi_active_floor = __floor; }
        }))(function() {
            const ret = fetch_number();
            return ret;
        }),
        __wbg_sleep_319b371bcbeaac51: ((__inner) => new WebAssembly.Suspending(async function(...args) {
            const __sp = wasm.__stack_pointer.value;
            const __floor = __jspi_active_floor;
            if (__sp <= __floor) throw new RangeError('JSPI fiber stack overflow');
            try { return await __inner(...args); }
            finally { wasm.__stack_pointer.value = __sp; __jspi_active_floor = __floor; }
        }))(function(arg0) {
            return sleep(arg0 >>> 0);
        }),
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

let __wbg_jspi_compute;


let __wbg_jspi_do_work;

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

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('reference_test_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
