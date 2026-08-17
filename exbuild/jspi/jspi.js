/* @ts-self-types="./jspi.d.ts" */

/**
 * Stress-tests that `__stack_pointer` is correctly restored after suspension
 * for arbitrary call depth and for heap allocations that occur after resume.
 *
 * The call tree recurses `depth` levels deep (each frame has a 1 KiB
 * shadow-stack local), suspends at the bottom, then allocates a `Vec` on
 * the way back up through every frame.
 *
 * ## Why this validates the shadow-stack invariant
 *
 * `__stack_pointer` is a Wasm *global*; JSPI preserves Wasm locals across
 * fiber switches but **not** globals.  If another fiber runs while this one
 * is suspended it would overwrite both the global and the shadow-stack
 * memory itself.  wasm-bindgen instruments every
 * `#[wasm_bindgen(suspending)]` import with an in-wasm wrapper that
 * evacuates the fiber's live shadow-stack region to the heap before
 * suspending, and copies it back — restoring `__stack_pointer` from a wasm
 * local — as the very first instructions after resume.  By the time any Rust
 * instruction executes after `block_on_promise` returns, the whole stack is
 * already correct — even after 20 nested frames and even when the very next
 * operation allocates from the heap.
 *
 * ## Expected return value
 *
 * `deep_alloc(N)` returns `1000 + N*(N+1)/2`.
 * For `deep_alloc(20)` the expected value is **1210**.
 * @param {number} depth
 * @returns {Promise<number>}
 */
export async function deep_alloc(depth) {
    const ret = await (__wbg_jspi_deep_alloc ??= WebAssembly.promising(wasm.deep_alloc))(depth);
    return ret >>> 0;
}

/**
 * Demonstrates that fibers use the full main shadow stack: ~96 KiB of live
 * stack frames survive a suspension with no size tuning required.
 *
 * wasm-bindgen instruments the module so that on suspension the fiber's live
 * shadow-stack region is evacuated to the heap and copied back — to the same
 * addresses — immediately on resume. Returns `49152`.
 * @returns {Promise<number>}
 */
export async function deep_stack() {
    const ret = await (__wbg_jspi_deep_stack ??= WebAssembly.promising(wasm.deep_stack))();
    return ret >>> 0;
}

/**
 * Sleep for `ms` milliseconds.
 *
 * This is a plain (non-`async`) Rust function, yet it awaits a `setTimeout`
 * promise via JSPI without blocking the browser's event loop.
 *
 * The `#[wasm_bindgen(jspi)]` attribute causes the generated JS glue to wrap
 * this export with `WebAssembly.promising`, so callers receive a `Promise`.
 * @param {number} ms
 * @returns {Promise<void>}
 */
export async function do_sleep(ms) {
    await (__wbg_jspi_do_sleep ??= WebAssembly.promising(wasm.do_sleep))(ms);
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_is_undefined_6cff064c44e0d823: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jspi_spawn_poll_5d3e535b7a2053dd: function(arg0) {
            Promise.resolve().then(() => (__wbg_jspi_task_poll_promising ??= WebAssembly.promising(wasm.__wbg_jspi_task_poll))(arg0 >>> 0));
        },
        __wbg___wbindgen_jspi_suspend_a24992d661c1c360: ((__inner) => new WebAssembly.Suspending(function(...args) {
            try { return __inner.apply(this, args); }
            catch (e) { return Promise.reject(e); }
        }))(function(arg0) {
            const ret = arg0;
            return ret;
        }),
        __wbg___wbindgen_throw_bb96b2010945f0bc: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_instanceof_Window_5625ff9937037a38: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_new_418fb92a013d5930: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_a42b010618f9b99f___convert__closures_____invoke___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined_______true_(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        },
        __wbg_resolve_020f95d838c6ef25: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_setTimeout_8be4960d8ad2bb76: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.setTimeout(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_static_accessor_GLOBAL_THIS_466428f93b4eaa76: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_c7aea38d4de089bc: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_42d4fae05e59267a: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_e0db14a0eba6a812: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbindgen_generic_0000000000000001: function(arg0) {
            // Cast intrinsic for `Externref -> Externref`.
            const ret = arg0;
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
        __wbindgen_jstag: WebAssembly.JSTag,
    };
    return {
        __proto__: null,
        "./jspi_bg.js": import0,
    };
}

function wasm_bindgen_a42b010618f9b99f___convert__closures_____invoke___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen_a42b010618f9b99f___convert__closures_____invoke___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
}

let __wbg_jspi_deep_alloc;

let __wbg_jspi_deep_stack;

let __wbg_jspi_do_sleep;

let __wbg_jspi_task_poll_promising;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
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

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
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

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (!module.ok) {
            throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
        }

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = expectedResponseType(module.type);

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
        module_or_path = new URL('jspi_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
