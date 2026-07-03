let wasm_bindgen = (function(exports) {
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }

    /**
     * @param {any} promise
     * @returns {Promise<any>}
     */
    async function drive(promise) {
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
    exports.drive = drive;
    function __wbg_get_imports() {
        const import0 = {
            __proto__: null,
            __wbg___wbindgen_jspi_cleanup_485cc4f59821d0c3: function(arg0) {
                _jspiResolved[arg0 >>> 0] = undefined;
                _jspiRejected[arg0 >>> 0] = false;
                _jspiPending[arg0 >>> 0] = undefined;
            },
            __wbg___wbindgen_jspi_get_resolved_ae482262158d0e4e: function(arg0) {
                const ret = _jspiResolved[arg0 >>> 0];
                return ret;
            },
            __wbg___wbindgen_jspi_is_rejected_ad8e8b2df7ac1bf8: function(arg0) {
                const ret = _jspiRejected[arg0 >>> 0];
                return ret;
            },
            __wbg___wbindgen_jspi_set_pending_eb0737ebb950d87d: function(arg0, arg1) {
                _jspiPending[arg0 >>> 0] = arg1;
            },
            __wbg___wbindgen_jspi_suspend_a72c0d026c006e17: ((__inner) => new WebAssembly.Suspending(async function(...args) {
                const __sp = wasm.__stack_pointer.value;
                const __floor = __jspi_active_floor;
                if (__sp <= __floor) throw new RangeError('JSPI fiber stack overflow');
                try { return await __inner(...args); }
                finally { wasm.__stack_pointer.value = __sp; __jspi_active_floor = __floor; }
            }))(function(arg0) {
                return _jspiPending[arg0 >>> 0].then(v => { _jspiRejected[arg0 >>> 0] = false; _jspiResolved[arg0 >>> 0] = v; }, e => { _jspiRejected[arg0 >>> 0] = true; _jspiResolved[arg0 >>> 0] = e; });
            }),
            __wbg___wbindgen_jspi_waker_cleanup_e24dd9d90266971f: function(arg0) {
                _jspiWakerMap.delete(arg0 >>> 0);
            },
            __wbg___wbindgen_jspi_waker_create_ea0ae813d51107c9: function(arg0) {
                const ret = new Promise(resolve => _jspiWakerMap.set(arg0 >>> 0, resolve));
                return ret;
            },
            __wbg___wbindgen_jspi_waker_wake_18ccfe9f06cb07b3: function(arg0) {
                const resolve = _jspiWakerMap.get(arg0 >>> 0);
                resolve && resolve();
            },
            __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
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
        };
        return {
            __proto__: null,
            "./reference_test_bg.js": import0,
        };
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
    function decodeText(ptr, len) {
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

        if (module_or_path === undefined && script_src !== undefined) {
            module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module);
    }

    return Object.assign(__wbg_init, { initSync }, exports);
})({ __proto__: null });
