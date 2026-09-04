let wasm_bindgen = (function(exports) {
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }

    class RenderingScene {
        static __wrap(ptr) {
            const obj = Object.create(RenderingScene.prototype);
            obj.__wbg_ptr = ptr;
            RenderingSceneFinalization.register(obj, obj.__wbg_ptr, obj);
            return obj;
        }
        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            RenderingSceneFinalization.unregister(this);
            return ptr;
        }
        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_renderingscene_free(ptr, 0);
        }
        /**
         * Return a progressive rendering of the image so far
         * @returns {ImageData}
         */
        imageSoFar() {
            const ret = wasm.renderingscene_imageSoFar(this.__wbg_ptr);
            return ret;
        }
        /**
         * Returns the JS promise object which resolves when the render is complete
         * @returns {Promise<any>}
         */
        promise() {
            const ret = wasm.renderingscene_promise(this.__wbg_ptr);
            return ret;
        }
    }
    if (Symbol.dispose) RenderingScene.prototype[Symbol.dispose] = RenderingScene.prototype.free;
    exports.RenderingScene = RenderingScene;

    class Scene {
        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            SceneFinalization.unregister(this);
            return ptr;
        }
        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_scene_free(ptr, 0);
        }
        /**
         * Creates a new scene from the JSON description in `object`, which we
         * deserialize here into an actual scene.
         * @param {any} object
         */
        constructor(object) {
            const ret = wasm.scene_new(object);
            if (ret[2]) {
                throw takeFromExternrefTable0(ret[1]);
            }
            this.__wbg_ptr = ret[0];
            SceneFinalization.register(this, this.__wbg_ptr, this);
            return this;
        }
        /**
         * Renders this scene with the provided concurrency and worker pool.
         *
         * This will spawn up to `concurrency` workers which are loaded from or
         * spawned into `pool`. The `RenderingScene` state contains information to
         * get notifications when the render has completed.
         * @param {number} concurrency
         * @param {WorkerPool} pool
         * @returns {RenderingScene}
         */
        render(concurrency, pool) {
            const ptr = this.__destroy_into_raw();
            _assertClass(pool, WorkerPool);
            const ret = wasm.scene_render(ptr, concurrency, pool.__wbg_ptr);
            if (ret[2]) {
                throw takeFromExternrefTable0(ret[1]);
            }
            return RenderingScene.__wrap(ret[0]);
        }
    }
    if (Symbol.dispose) Scene.prototype[Symbol.dispose] = Scene.prototype.free;
    exports.Scene = Scene;

    class WorkerPool {
        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            WorkerPoolFinalization.unregister(this);
            return ptr;
        }
        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_workerpool_free(ptr, 0);
        }
        /**
         * Creates a new `WorkerPool` which immediately creates `initial` workers.
         *
         * The pool created here can be used over a long period of time, and it
         * will be initially primed with `initial` workers. Currently workers are
         * never released or gc'd until the whole pool is destroyed.
         *
         * # Errors
         *
         * Returns any error that may happen while a JS web worker is created and a
         * message is sent to it.
         * @param {number} initial
         */
        constructor(initial) {
            const ret = wasm.workerpool_new(initial);
            if (ret[2]) {
                throw takeFromExternrefTable0(ret[1]);
            }
            this.__wbg_ptr = ret[0];
            WorkerPoolFinalization.register(this, this.__wbg_ptr, this);
            return this;
        }
    }
    if (Symbol.dispose) WorkerPool.prototype[Symbol.dispose] = WorkerPool.prototype.free;
    exports.WorkerPool = WorkerPool;

    /**
     * Entry point invoked by `worker.js`, a bit of a hack but see the "TODO" above
     * about `worker.js` in general.
     * @param {number} ptr
     */
    function child_entry_point(ptr) {
        const ret = wasm.child_entry_point(ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    exports.child_entry_point = child_entry_point;
    function __wbg_get_imports(memory) {
        const import0 = {
            __proto__: null,
            __wbg_Error_67e7344beaa85059: function(arg0, arg1) {
                const ret = Error(getStringFromWasm0(arg0, arg1));
                return ret;
            },
            __wbg_Number_c54e7112a3fa7e3e: function(arg0) {
                const ret = Number(arg0);
                return ret;
            },
            __wbg_String_8564e559799eccda: function(arg0, arg1) {
                const ret = String(arg1);
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_boolean_get_7a12af2b3f899c5a: function(arg0) {
                const v = arg0;
                const ret = typeof(v) === 'boolean' ? v : undefined;
                return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
            },
            __wbg___wbindgen_debug_string_0e68cf47c9cbd9b0: function(arg0, arg1) {
                const ret = debugString(arg1);
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_in_50072d4d6e45c193: function(arg0, arg1) {
                const ret = arg0 in arg1;
                return ret;
            },
            __wbg___wbindgen_is_function_fcda5e3902d732fe: function(arg0) {
                const ret = typeof(arg0) === 'function';
                return ret;
            },
            __wbg___wbindgen_is_object_edb6b15aa3afe12e: function(arg0) {
                const val = arg0;
                const ret = typeof(val) === 'object' && val !== null;
                return ret;
            },
            __wbg___wbindgen_is_string_c4f7cb494a2a21f1: function(arg0) {
                const ret = typeof(arg0) === 'string';
                return ret;
            },
            __wbg___wbindgen_is_undefined_8c687d0b90d5b524: function(arg0) {
                const ret = arg0 === undefined;
                return ret;
            },
            __wbg___wbindgen_jsval_eq_9fdcd3c0a860dd3b: function(arg0, arg1) {
                const ret = arg0 === arg1;
                return ret;
            },
            __wbg___wbindgen_jsval_loose_eq_3c30021c243b64cd: function(arg0, arg1) {
                const ret = arg0 == arg1;
                return ret;
            },
            __wbg___wbindgen_memory_3f8442e22540244f: function() {
                const ret = wasm.memory;
                return ret;
            },
            __wbg___wbindgen_module_c2aeeb3b14bad631: function() {
                const ret = wasmModule;
                return ret;
            },
            __wbg___wbindgen_number_get_1dc732b810cb937c: function(arg0, arg1) {
                const obj = arg1;
                const ret = typeof(obj) === 'number' ? obj : undefined;
                getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
            },
            __wbg___wbindgen_rethrow_dba7bb2caa14ba21: function(arg0) {
                throw arg0;
            },
            __wbg___wbindgen_string_get_92ab86bb19cbc12f: function(arg0, arg1) {
                const obj = arg1;
                const ret = typeof(obj) === 'string' ? obj : undefined;
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_throw_5d9e815e6fdf150f: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
            },
            __wbg__wbg_cb_unref_997e73d32238e655: function(arg0) {
                arg0._wbg_cb_unref();
            },
            __wbg_async_50fcc3f740d37ed6: function(arg0) {
                const ret = arg0.async;
                return ret;
            },
            __wbg_buffer_60eec204070b2802: function(arg0) {
                const ret = arg0.buffer;
                return ret;
            },
            __wbg_call_269c5566fbede3eb: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.call(arg1);
                return ret;
            }, arguments); },
            __wbg_call_6bcf8d3e20937e46: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.call(arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_data_d607e3ca86acb68c: function(arg0) {
                const ret = arg0.data;
                return ret;
            },
            __wbg_done_cffed884d87aa22e: function(arg0) {
                const ret = arg0.done;
                return ret;
            },
            __wbg_entries_972a87586902cf87: function(arg0) {
                const ret = Object.entries(arg0);
                return ret;
            },
            __wbg_error_757e9472f8410341: function(arg0, arg1) {
                let deferred0_0;
                let deferred0_1;
                try {
                    deferred0_0 = arg0;
                    deferred0_1 = arg1;
                    console.error(getStringFromWasm0(arg0, arg1));
                } finally {
                    wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
                }
            },
            __wbg_get_6cf5a4d4d8ad3c5a: function() { return handleError(function (arg0, arg1) {
                const ret = Reflect.get(arg0, arg1);
                return ret;
            }, arguments); },
            __wbg_get_b1f0ab13c737f856: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return ret;
            },
            __wbg_get_unchecked_363572bdd397d473: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return ret;
            },
            __wbg_get_with_ref_key_6412cf3094599694: function(arg0, arg1) {
                const ret = arg0[arg1];
                return ret;
            },
            __wbg_instanceof_ArrayBuffer_d4ff01f8247925ae: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ArrayBuffer;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_ErrorEvent_7c9f5160f3a47d83: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ErrorEvent;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_MessageEvent_feafc259c6e9c5f1: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof MessageEvent;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Uint8Array_598adc0fef426aa8: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Uint8Array;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_isArray_5674713bb7b79043: function(arg0) {
                const ret = Array.isArray(arg0);
                return ret;
            },
            __wbg_isSafeInteger_8f51c743827d1ec5: function(arg0) {
                const ret = Number.isSafeInteger(arg0);
                return ret;
            },
            __wbg_iterator_22ddeb808cf55a6f: function() {
                const ret = Symbol.iterator;
                return ret;
            },
            __wbg_length_31bdaf014f5fbde2: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_4e1adc0d42e23620: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_log_940fb95e88f2483d: function(arg0) {
                console.log(arg0);
            },
            __wbg_log_95348e8e4f492b7e: function(arg0, arg1) {
                console.log(getStringFromWasm0(arg0, arg1));
            },
            __wbg_message_247942253a4ea78c: function(arg0, arg1) {
                const ret = arg1.message;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_new_0700aaf827082758: function(arg0) {
                const ret = new Int32Array(arg0);
                return ret;
            },
            __wbg_new_1da3429bc3c4541c: function(arg0) {
                const ret = new Uint8Array(arg0);
                return ret;
            },
            __wbg_new_227d7c05414eb861: function() {
                const ret = new Error();
                return ret;
            },
            __wbg_new_22cf7c73b2c52d34: function(arg0) {
                const ret = new Uint8ClampedArray(arg0);
                return ret;
            },
            __wbg_new_8f72ed7c652bf5a2: function(arg0, arg1) {
                try {
                    var state0 = {a: arg0, b: arg1};
                    var cb0 = (arg0, arg1) => {
                        const a = state0.a;
                        state0.a = 0;
                        try {
                            return wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined_______true_(a, state0.b, arg0, arg1);
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
            __wbg_new_a313bf260f9bdcc3: function() { return handleError(function (arg0, arg1) {
                const ret = new Worker(getStringFromWasm0(arg0, arg1));
                return ret;
            }, arguments); },
            __wbg_new_ffa92086ea89f79c: function() {
                const ret = new Array();
                return ret;
            },
            __wbg_new_typed_6f8b0d724fe26c07: function(arg0, arg1) {
                try {
                    var state0 = {a: arg0, b: arg1};
                    var cb0 = (arg0, arg1) => {
                        const a = state0.a;
                        state0.a = 0;
                        try {
                            return wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined_______true_(a, state0.b, arg0, arg1);
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
            __wbg_new_with_js_u8_clamped_array_and_sh_9b02d4d6460873d5: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = new ImageData(arg0, arg1 >>> 0, arg2 >>> 0);
                return ret;
            }, arguments); },
            __wbg_new_worker_81c08e70aadb3771: function(arg0, arg1) {
                const ret = new Worker(getStringFromWasm0(arg0, arg1));
                return ret;
            },
            __wbg_next_95053e306b1c3aed: function(arg0) {
                const ret = arg0.next;
                return ret;
            },
            __wbg_next_f31ecb8646d2c605: function() { return handleError(function (arg0) {
                const ret = arg0.next();
                return ret;
            }, arguments); },
            __wbg_of_d9a05aa72bb023a2: function(arg0, arg1, arg2) {
                const ret = Array.of(arg0, arg1, arg2);
                return ret;
            },
            __wbg_postMessage_14c07434f6b7e150: function() { return handleError(function (arg0, arg1) {
                arg0.postMessage(arg1);
            }, arguments); },
            __wbg_postMessage_1f51e50254674143: function() { return handleError(function (arg0, arg1) {
                arg0.postMessage(arg1);
            }, arguments); },
            __wbg_postMessage_8105a302ed002474: function() { return handleError(function (arg0, arg1) {
                arg0.postMessage(arg1);
            }, arguments); },
            __wbg_prototypesetcall_ae9f5e7459250748: function(arg0, arg1, arg2) {
                Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
            },
            __wbg_push_bfdf956ba476f65b: function(arg0, arg1) {
                const ret = arg0.push(arg1);
                return ret;
            },
            __wbg_queueMicrotask_85c90f6987555d65: function(arg0) {
                const ret = arg0.queueMicrotask;
                return ret;
            },
            __wbg_queueMicrotask_f6a1fa10b81d1fc0: function(arg0) {
                queueMicrotask(arg0);
            },
            __wbg_resolve_35ec7e0c6af4c82c: function(arg0) {
                const ret = Promise.resolve(arg0);
                return ret;
            },
            __wbg_set_onerror_30dd5b507c57e96f: function(arg0, arg1) {
                arg0.onerror = arg1;
            },
            __wbg_set_onmessage_2244c2525c4e0c38: function(arg0, arg1) {
                arg0.onmessage = arg1;
            },
            __wbg_set_onmessage_d7e69fe35723df1e: function(arg0, arg1) {
                arg0.onmessage = arg1;
            },
            __wbg_slice_39d07f70c162f831: function(arg0, arg1, arg2) {
                const ret = arg0.slice(arg1 >>> 0, arg2 >>> 0);
                return ret;
            },
            __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
                const ret = arg1.stack;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_static_accessor_GLOBAL_8eb4cd83130a11a0: function() {
                const ret = typeof global === 'undefined' ? null : global;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_GLOBAL_THIS_1e7044f654e934db: function() {
                const ret = typeof globalThis === 'undefined' ? null : globalThis;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_SELF_d8b50611246a6d92: function() {
                const ret = typeof self === 'undefined' ? null : self;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_WINDOW_fd0bc376bf0f8b42: function() {
                const ret = typeof window === 'undefined' ? null : window;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_then_8ebf7f4d2b402f98: function(arg0, arg1) {
                const ret = arg0.then(arg1);
                return ret;
            },
            __wbg_then_b830475380919203: function(arg0, arg1) {
                const ret = arg0.then(arg1);
                return ret;
            },
            __wbg_type_9a9a7042e88baa94: function(arg0, arg1) {
                const ret = arg1.type;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_value_a06131499b246ab0: function(arg0) {
                const ret = arg0.value;
                return ret;
            },
            __wbg_value_c227f843d21da141: function(arg0) {
                const ret = arg0.value;
                return ret;
            },
            __wbg_waitAsync_80b2e954334c3fdf: function(arg0, arg1, arg2) {
                const ret = Atomics.waitAsync(arg0, arg1 >>> 0, arg2);
                return ret;
            },
            __wbg_waitAsync_ac27a6bec1171c41: function() {
                const ret = Atomics.waitAsync;
                return ret;
            },
            __wbindgen_generic_0000000000000001: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 133, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue__core_77def4dfb9471a96___result__Result_____wasm_bindgen_e5fbcddd152687f3___JsError___true_);
                return ret;
            },
            __wbindgen_generic_0000000000000002: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 153, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___futures__task__wait_async_polyfill__MessageEvent______true_);
                return ret;
            },
            __wbindgen_generic_0000000000000003: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 56, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue______true_);
                return ret;
            },
            __wbindgen_generic_0000000000000004: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 56, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue______true__10);
                return ret;
            },
            __wbindgen_generic_0000000000000005: function(arg0) {
                // Cast intrinsic for `F64 -> Externref`.
                const ret = arg0;
                return ret;
            },
            __wbindgen_generic_0000000000000006: function(arg0, arg1) {
                // Cast intrinsic for `Ref(String) -> Externref`.
                const ret = getStringFromWasm0(arg0, arg1);
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
            __wbindgen_link_e7dbf70342428633: function(arg0) {
                const val = `onmessage = function (ev) {
                    let [ia, index, value] = ev.data;
                    ia = new Int32Array(ia.buffer);
                    let result = Atomics.wait(ia, index, value);
                    postMessage(result);
                };
                `;
                const ret = typeof URL.createObjectURL === 'undefined' ? "data:application/javascript," + encodeURIComponent(val) : URL.createObjectURL(new Blob([val], { type: "text/javascript" }));
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            memory: memory || new WebAssembly.Memory({initial:18,maximum:16384,shared:true}),
        };
        return {
            __proto__: null,
            "./raytrace_parallel_bg.js": import0,
        };
    }

    function wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___futures__task__wait_async_polyfill__MessageEvent______true_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___futures__task__wait_async_polyfill__MessageEvent______true_(arg0, arg1, arg2);
    }

    function wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue______true_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue______true_(arg0, arg1, arg2);
    }

    function wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue______true__10(arg0, arg1, arg2) {
        wasm.wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue______true__10(arg0, arg1, arg2);
    }

    function wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue__core_77def4dfb9471a96___result__Result_____wasm_bindgen_e5fbcddd152687f3___JsError___true_(arg0, arg1, arg2) {
        const ret = wasm.wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___wasm_bindgen_e5fbcddd152687f3___JsValue__core_77def4dfb9471a96___result__Result_____wasm_bindgen_e5fbcddd152687f3___JsError___true_(arg0, arg1, arg2);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }

    function wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
        wasm.wasm_bindgen_e5fbcddd152687f3___convert__closures_____invoke___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined___js_sys_56313336cd80145c___Function_fn_wasm_bindgen_e5fbcddd152687f3___JsValue_____wasm_bindgen_e5fbcddd152687f3___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
    }

    const RenderingSceneFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_renderingscene_free(ptr, 1));
    const SceneFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_scene_free(ptr, 1));
    const WorkerPoolFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_workerpool_free(ptr, 1));

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_externrefs.set(idx, obj);
        return idx;
    }

    function _assertClass(instance, klass) {
        if (!(instance instanceof klass)) {
            throw new Error(`expected instance of ${klass.name}`);
        }
    }

    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches && builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    function getArrayU8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
    }

    let cachedDataViewMemory0 = null;
    function getDataViewMemory0() {
        if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
            cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
        }
        return cachedDataViewMemory0;
    }

    function getStringFromWasm0(ptr, len) {
        return decodeText(ptr >>> 0, len);
    }

    let cachedUint8ArrayMemory0 = null;
    function getUint8ArrayMemory0() {
        if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
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

    function makeMutClosure(arg0, arg1, f) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (...args) => {

            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                state.a = a;
                real._wbg_cb_unref();
            }
        };
        real._wbg_cb_unref = () => {
            if (--state.cnt === 0) {
                wasm.__wbindgen_destroy_closure(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        };
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

        const mem = getUint8ArrayMemory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }
        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
            ptr = realloc(ptr, len, offset, 1) >>> 0;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    function takeFromExternrefTable0(idx) {
        const value = wasm.__wbindgen_externrefs.get(idx);
        wasm.__externref_table_dealloc(idx);
        return value;
    }

    let cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : undefined);
    if (cachedTextDecoder) cachedTextDecoder.decode();

    function decodeText(ptr, len) {
        return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
    }

    const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder() : undefined);

    if (cachedTextEncoder) {
        cachedTextEncoder.encodeInto = function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        };
    }

    let WASM_VECTOR_LEN = 0;

    let wasmModule, wasmInstance, wasm;
    function __wbg_finalize_init(instance, module, thread_stack_size) {
        wasmInstance = instance;
        wasm = instance.exports;
        wasmModule = module;
        cachedDataViewMemory0 = null;
        cachedUint8ArrayMemory0 = null;
        if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) {
            throw new Error('invalid stack size');
        }

        wasm.__wbindgen_start(thread_stack_size);
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

    function initSync(module, memory) {
        if (wasm !== undefined) return wasm;

        let thread_stack_size
        if (module !== undefined) {
            if (Object.getPrototypeOf(module) === Object.prototype) {
                ({module, memory, thread_stack_size} = module)
            } else {
                console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
            }
        }

        const imports = __wbg_get_imports(memory);
        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }
        const instance = new WebAssembly.Instance(module, imports);
        return __wbg_finalize_init(instance, module, thread_stack_size);
    }

    async function __wbg_init(module_or_path, memory) {
        if (wasm !== undefined) return wasm;

        let thread_stack_size
        if (module_or_path !== undefined) {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path, memory, thread_stack_size} = module_or_path)
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead')
            }
        }

        if (module_or_path === undefined && script_src !== undefined) {
            module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
        }
        const imports = __wbg_get_imports(memory);

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module, thread_stack_size);
    }

    return Object.assign(__wbg_init, { initSync }, exports);
})({ __proto__: null });
