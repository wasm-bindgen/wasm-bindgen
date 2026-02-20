/* @ts-self-types="./reference_test.d.ts" */

/**
 * Runtime test harness support instantiated in JS.
 *
 * The node.js entry script instantiates a `Context` here which is used to
 * drive test execution.
 */
export class WasmBindgenTestContext {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmBindgenTestContextFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmbindgentestcontext_free(ptr, 0);
    }
    /**
     * Handle filter argument.
     * @param {number} filtered
     */
    filtered_count(filtered) {
        wasm.wasmbindgentestcontext_filtered_count(this.__wbg_ptr, filtered);
    }
    /**
     * Handle `--include-ignored` flag.
     * @param {boolean} include_ignored
     */
    include_ignored(include_ignored) {
        wasm.wasmbindgentestcontext_include_ignored(this.__wbg_ptr, include_ignored);
    }
    /**
     * Creates a new context ready to run tests.
     *
     * A `Context` is the main structure through which test execution is
     * coordinated, and this will collect output and results for all executed
     * tests.
     * @param {boolean} is_bench
     */
    constructor(is_bench) {
        const ret = wasm.wasmbindgentestcontext_new(is_bench);
        this.__wbg_ptr = ret >>> 0;
        WasmBindgenTestContextFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Executes a list of tests, returning a promise representing their
     * eventual completion.
     *
     * This is the main entry point for executing tests. All the tests passed
     * in are the JS `Function` object that was plucked off the
     * `WebAssembly.Instance` exports list.
     *
     * The promise returned resolves to either `true` if all tests passed or
     * `false` if at least one test failed.
     * @param {any[]} tests
     * @returns {Promise<any>}
     */
    run(tests) {
        const ptr0 = passArrayJsValueToWasm0(tests, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmbindgentestcontext_run(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
}
if (Symbol.dispose) WasmBindgenTestContext.prototype[Symbol.dispose] = WasmBindgenTestContext.prototype.free;

/**
 * Used to read benchmark data, and then the runner stores it on the local disk.
 * @returns {Uint8Array | undefined}
 */
export function __wbgbench_dump() {
    const ret = wasm.__wbgbench_dump();
    let v1;
    if (ret[0] !== 0) {
        v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v1;
}

/**
 * Used to write previous benchmark data before the benchmark, for later comparison.
 * @param {Uint8Array} baseline
 */
export function __wbgbench_import(baseline) {
    const ptr0 = passArray8ToWasm0(baseline, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbgbench_import(ptr0, len0);
}

/**
 * Handler for `console.debug` invocations. See above.
 * @param {Array<any>} args
 */
export function __wbgtest_console_debug(args) {
    wasm.__wbgtest_console_debug(args);
}

/**
 * Handler for `console.error` invocations. See above.
 * @param {Array<any>} args
 */
export function __wbgtest_console_error(args) {
    wasm.__wbgtest_console_error(args);
}

/**
 * Handler for `console.info` invocations. See above.
 * @param {Array<any>} args
 */
export function __wbgtest_console_info(args) {
    wasm.__wbgtest_console_info(args);
}

/**
 * Handler for `console.log` invocations.
 *
 * If a test is currently running it takes the `args` array and stringifies
 * it and appends it to the current output of the test. Otherwise it passes
 * the arguments to the original `console.log` function, psased as
 * `original`.
 * @param {Array<any>} args
 */
export function __wbgtest_console_log(args) {
    wasm.__wbgtest_console_log(args);
}

/**
 * Handler for `console.warn` invocations. See above.
 * @param {Array<any>} args
 */
export function __wbgtest_console_warn(args) {
    wasm.__wbgtest_console_warn(args);
}

/**
 * @returns {Uint8Array | undefined}
 */
export function __wbgtest_cov_dump() {
    const ret = wasm.__wbgtest_cov_dump();
    let v1;
    if (ret[0] !== 0) {
        v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v1;
}

/**
 * Path to use for coverage data.
 * @param {string | null | undefined} env
 * @param {number} pid
 * @param {string} temp_dir
 * @param {bigint} module_signature
 * @returns {string}
 */
export function __wbgtest_coverage_path(env, pid, temp_dir, module_signature) {
    let deferred3_0;
    let deferred3_1;
    try {
        var ptr0 = isLikeNone(env) ? 0 : passStringToWasm0(env, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(temp_dir, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.__wbgtest_coverage_path(ptr0, len0, pid, ptr1, len1, module_signature);
        deferred3_0 = ret[0];
        deferred3_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @returns {bigint | undefined}
 */
export function __wbgtest_module_signature() {
    const ret = wasm.__wbgtest_module_signature();
    return ret[0] === 0 ? undefined : BigInt.asUintN(64, ret[1]);
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Deno_f60c3c7e825ad418: function(arg0) {
            const ret = arg0.Deno;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_String_233d549434a8caff: function(arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbg_test_invoke_b283828fc66340cb: function() { return handleError(function (arg0) {
            __wbg_test_invoke(arg0);
        }, arguments); },
        __wbg___wbg_test_output_writeln_f828df2032a0b63c: function(arg0) {
            __wbg_test_output_writeln(arg0);
        },
        __wbg___wbgtest_og_console_log_a587bb2e3c630f50: function(arg0, arg1) {
            __wbgtest_og_console_log(getStringFromWasm0(arg0, arg1));
        },
        __wbg___wbindgen_debug_string_a1b3fd0656850da8: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_82aa5b8e9371b250: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_undefined_7b12045c262a3121: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_string_get_aab6399cd8ec8844: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_83ebd457a191bc2a: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_4fc42a417bb095f4: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_call_72a54043615c73e3: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_constructor_f1653f656271dfc0: function(arg0) {
            const ret = arg0.constructor;
            return ret;
        },
        __wbg_createTask_14f5fc2f8fb04e6a: function() { return handleError(function (arg0, arg1) {
            const ret = console.createTask(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_error_7fc56a56fda7d6cc: function(arg0, arg1) {
            console.error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_forEach_b1641681983d2ce1: function(arg0, arg1, arg2) {
            try {
                var state0 = {a: arg1, b: arg2};
                var cb0 = (arg0, arg1, arg2) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen__convert__closures_____invoke__h0000000000000004(a, state0.b, arg0, arg1, arg2);
                    } finally {
                        state0.a = a;
                    }
                };
                arg0.forEach(cb0);
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_getElementById_892d73816f671ce9: function(arg0, arg1, arg2) {
            const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_log_57a4eb1a3075e315: function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        },
        __wbg_message_3e5e69cdfc1ba64d: function(arg0) {
            const ret = arg0.message;
            return ret;
        },
        __wbg_name_1c57541f21fc4ced: function(arg0, arg1) {
            const ret = arg1.name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_name_f5b9516543523a79: function(arg0) {
            const ret = arg0.name;
            return ret;
        },
        __wbg_new_7dd39f4db4be6d0f: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_typed_810c34b0476068fa: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen__convert__closures_____invoke__h0000000000000005(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_now_99905866038a5f6d: function(arg0) {
            const ret = arg0.now();
            return ret;
        },
        __wbg_performance_936bf18854c2faa7: function(arg0) {
            const ret = arg0.performance;
            return ret;
        },
        __wbg_queueMicrotask_17a58d631cc9ab4b: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_queueMicrotask_4114767fcf2790b9: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_resolve_67a1b1ca24efbc5c: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_run_031af009900e4f90: function(arg0, arg1, arg2) {
            try {
                var state0 = {a: arg1, b: arg2};
                var cb0 = () => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen__convert__closures_____invoke__h0000000000000006(a, state0.b, );
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = arg0.run(cb0);
                return ret;
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_self_99ed0c214deaf973: function(arg0) {
            const ret = arg0.self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_set_text_content_2fe7c04d47fe9a32: function(arg0, arg1, arg2) {
            arg0.textContent = getStringFromWasm0(arg1, arg2);
        },
        __wbg_stack_7daa789a7049b374: function(arg0) {
            const ret = arg0.stack;
            return ret;
        },
        __wbg_stack_824c89cc6cf8439b: function(arg0) {
            const ret = arg0.stack;
            return ret;
        },
        __wbg_stack_be3368a487e0e3e0: function(arg0, arg1) {
            const ret = arg1.stack;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_stack_c999b5664536398e: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_static_accessor_DOCUMENT_6c660294d5da59c0: function() {
            const ret = document;
            return ret;
        },
        __wbg_static_accessor_GLOBAL_833a66cb4996dbd8: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_fc74cdbdccd80770: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_066699022f35d48b: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_f821c7eb05393790: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_text_content_9bc544ba431459ba: function(arg0, arg1) {
            const ret = arg1.textContent;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_then_420f698ab0b99678: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_toString_4d08ba77f708483d: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.toString();
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbindgen_cast_0000000000000000: function(arg0, arg1) {
            var state0 = {a: arg0, b: arg1};
            var cb0 = () => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return wasm_bindgen__convert__closures_____invoke__h0000000000000007(a, state0.b, );
                } finally {
                    state0.a = a;
                }
            };
            cb0._wbg_cb_unref = () => { state0.a = state0.b = 0; };
            // Cast intrinsic for `Closure(Closure { dtor_idx: 10, function: Function { arguments: [], shim_idx: 11, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = cb0;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 12, function: Function { arguments: [Externref], shim_idx: 13, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h0000000000000008, wasm_bindgen__convert__closures_____invoke__h0000000000000009);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
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
    };
    return {
        __proto__: null,
        "./reference_test_bg.js": import0,
    };
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000007(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000007(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000006(arg0, arg1) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000006(arg0, arg1);
    return ret !== 0;
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000009(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000009(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000005(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000005(arg0, arg1, arg2, arg3);
}

function wasm_bindgen__convert__closures_____invoke__h0000000000000004(arg0, arg1, arg2, arg3, arg4) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0000000000000004(arg0, arg1, arg2, arg3, arg4);
}

const WasmBindgenTestContextFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmbindgentestcontext_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

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
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
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

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
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
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
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

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
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

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
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
