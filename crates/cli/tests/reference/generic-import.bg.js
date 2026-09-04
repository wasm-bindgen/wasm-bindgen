/**
 * @param {any} widget
 * @param {any} lifetime_holder
 * @returns {Promise<void>}
 */
export function run(widget, lifetime_holder) {
    const ret = wasm.run(widget, lifetime_holder);
    return ret;
}
export function __wbg___wbindgen_debug_string_0e68cf47c9cbd9b0(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_is_function_fcda5e3902d732fe(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
}
export function __wbg___wbindgen_is_undefined_8c687d0b90d5b524(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_5d9e815e6fdf150f(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_997e73d32238e655(arg0) {
    arg0._wbg_cb_unref();
}
export function __wbg_blockNotGeneric_5e8959bc4144429a(arg0) {
    blockNotGeneric(arg0 >>> 0);
}
export function __wbg_call_6bcf8d3e20937e46() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
}, arguments); }
export function __wbg_new_typed_6f8b0d724fe26c07(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen_000000000000004b___convert__closures_____invoke___js_sys_000000000000004c___Function_fn_wasm_bindgen_000000000000004b___JsValue_____wasm_bindgen_000000000000004b___sys__Undefined___js_sys_000000000000004c___Function_fn_wasm_bindgen_000000000000004b___JsValue_____wasm_bindgen_000000000000004b___sys__Undefined_______true_(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return ret;
    } finally {
        state0.a = 0;
    }
}
export function __wbg_queueMicrotask_85c90f6987555d65(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
}
export function __wbg_queueMicrotask_f6a1fa10b81d1fc0(arg0) {
    queueMicrotask(arg0);
}
export function __wbg_resolve_35ec7e0c6af4c82c(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
}
export function __wbg_run_f19543b086e90d43(arg0, arg1, arg2) {
    try {
        var state0 = {a: arg1, b: arg2};
        var cb0 = () => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen_000000000000004b___convert__closures_____invoke___bool__true_(a, state0.b, );
            } finally {
                state0.a = a;
            }
        };
        const ret = arg0.run(cb0);
        return ret;
    } finally {
        state0.a = 0;
    }
}
export function __wbg_static_accessor_CREATE_TASK_04be768c473932c0() {
    const ret = typeof console === 'undefined' ? null : console?.createTask;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_GLOBAL_8eb4cd83130a11a0() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_GLOBAL_THIS_1e7044f654e934db() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_SELF_d8b50611246a6d92() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_WINDOW_fd0bc376bf0f8b42() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_takeScalars_beca431d0c81162b(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    takeScalars(arg0 >>> 0, arg1, arg2 !== 0, String.fromCodePoint(arg3), arg4, arg5, arg6);
}
export function __wbg_takeWideScalars_ad60f980ecf3c417(arg0, arg1, arg2, arg3, arg4) {
    takeWideScalars((BigInt.asUintN(64, arg0) | (arg1 << BigInt(64))), (BigInt.asUintN(64, arg2) | (BigInt.asUintN(64, arg3) << BigInt(64))), arg4);
}
export function __wbg_then_7a850dae4493f353(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
}
export function __wbg_then_b830475380919203(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
}
export function __wbindgen_generic_0000000000000000(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 79, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen_000000000000004b___convert__closures_____invoke___wasm_bindgen_000000000000004b___JsValue__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_);
    return ret;
}
export function __wbindgen_generic_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [String], shim_idx: 80, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen_000000000000004b___convert__closures_____invoke___alloc_000000000000004e___string__String__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_);
    return ret;
}
export function __wbindgen_generic_0000000000000002(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [U32], shim_idx: 81, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen_000000000000004b___convert__closures_____invoke___u32__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_);
    return ret;
}
export function __wbindgen_generic_0000000000000003(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_generic_0000000000000004(arg0) {
    // generic import `asyncCount`: [U32] -> Externref
    const ret = asyncCount(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000005(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        // generic import `asyncIdentity`: [String] -> Externref
        const ret = asyncIdentity(getStringFromWasm0(arg0, arg1));
        return ret;
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbindgen_generic_0000000000000006(arg0) {
    // generic import `asyncIdentity`: [U32] -> Externref
    const ret = asyncIdentity(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000007() { return handleError(function (arg0) {
    // generic import `asyncTry`: [U32] -> Externref
    const ret = asyncTry(arg0 >>> 0);
    return ret;
}, arguments); }
export function __wbindgen_generic_0000000000000008(arg0, arg1) {
    // generic import `attach`: [Ref(Externref), Ref(Externref)] -> Unit
    arg0.attach(arg1);
}
export function __wbindgen_generic_0000000000000009(arg0) {
    // generic import `blockInheritedImplTrait`: [F64] -> Unit
    blockInheritedImplTrait(arg0);
}
export function __wbindgen_generic_000000000000000a(arg0) {
    // generic import `blockInheritedImplTrait`: [U32] -> Unit
    blockInheritedImplTrait(arg0 >>> 0);
}
export function __wbindgen_generic_000000000000000b(arg0, arg1) {
    // generic import `blockInheritedTwo`: [U32, F64] -> F64
    const ret = blockInheritedTwo(arg0 >>> 0, arg1);
    return ret;
}
export function __wbindgen_generic_000000000000000c(arg0) {
    // generic import `blockInherited`: [F64] -> Unit
    blockInherited(arg0);
}
export function __wbindgen_generic_000000000000000d(arg0) {
    // generic import `blockInherited`: [U32] -> Unit
    blockInherited(arg0 >>> 0);
}
export function __wbindgen_generic_000000000000000e(arg0, arg1) {
    // generic import `combine`: [Ref(Externref), F64] -> Unit
    arg0.combine(arg1);
}
export function __wbindgen_generic_000000000000000f(arg0, arg1) {
    // generic import `combine`: [Ref(Externref), U32] -> Unit
    arg0.combine(arg1 >>> 0);
}
export function __wbindgen_generic_0000000000000010(arg0) {
    // generic import `a.b.deepLog`: [F64] -> Unit
    a.b.deepLog(arg0);
}
export function __wbindgen_generic_0000000000000011(arg0, arg1, arg2) {
    // generic import `delete_indexed`: [Ref(Externref), Ref(String)] -> Unit
    delete arg0[getStringFromWasm0(arg1, arg2)];
}
export function __wbindgen_generic_0000000000000012(arg0, arg1, arg2) {
    // generic import `fillSlice`: [RefMut(Slice(U16)), U32] -> Unit
    fillSlice(getArrayU16FromWasm0(arg0, arg1), arg2 >>> 0);
}
export function __wbindgen_generic_0000000000000013(arg0, arg1, arg2) {
    // generic import `get`: [Ref(Externref), Ref(String)] -> U32
    const ret = arg0[getStringFromWasm0(arg1, arg2)];
    return ret;
}
export function __wbindgen_generic_0000000000000014(arg0, arg1) {
    // generic import `get`: [Ref(Externref)] -> String
    const ret = arg1.get();
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbindgen_generic_0000000000000015(arg0) {
    // generic import `get`: [Ref(Externref)] -> U32
    const ret = arg0.get();
    return ret;
}
export function __wbindgen_generic_0000000000000016(arg0, arg1) {
    // generic import `get`: [Ref(Externref)] -> String
    const ret = arg1.get();
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbindgen_generic_0000000000000017(arg0) {
    // generic import `get`: [Ref(Externref)] -> U32
    const ret = arg0.get();
    return ret;
}
export function __wbindgen_generic_0000000000000018(arg0) {
    // generic import `identity`: [F64] -> F64
    const ret = identity(arg0);
    return ret;
}
export function __wbindgen_generic_0000000000000019(arg0) {
    // generic import `identity`: [U32] -> U32
    const ret = identity(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_000000000000001a(arg0) {
    // generic import `kind`: [Ref(Externref)] -> U32
    const ret = GetOwnOrInheritedPropertyDescriptor(Widget.prototype, 'kind').get.call(arg0);
    return ret;
}
export function __wbindgen_generic_000000000000001b(arg0, arg1, arg2) {
    var v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    // generic import `logBlockSlice`: [Ref(Vector(U16)), U32] -> Unit
    logBlockSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_000000000000001c(arg0, arg1) {
    // generic import `logGenericSlice`: [Ref(Slice(F64))] -> Unit
    logGenericSlice(getArrayF64FromWasm0(arg0, arg1));
}
export function __wbindgen_generic_000000000000001d(arg0, arg1) {
    // generic import `logGenericSlice`: [Ref(Slice(U32))] -> Unit
    logGenericSlice(getArrayU32FromWasm0(arg0, arg1));
}
export function __wbindgen_generic_000000000000001e(arg0) {
    // generic import `logImplTrait`: [F64] -> Unit
    logImplTrait(arg0);
}
export function __wbindgen_generic_000000000000001f(arg0) {
    // generic import `logImplTrait`: [U32] -> Unit
    logImplTrait(arg0 >>> 0);
}
export function __wbindgen_generic_0000000000000020(arg0, arg1) {
    var v0 = getArrayF64FromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 8, 8);
    // generic import `logNestedImplTrait`: [Vector(F64)] -> Unit
    logNestedImplTrait(v0);
}
export function __wbindgen_generic_0000000000000021(arg0, arg1) {
    var v0 = getArrayU32FromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // generic import `logNestedImplTrait`: [Vector(U32)] -> Unit
    logNestedImplTrait(v0);
}
export function __wbindgen_generic_0000000000000022(arg0, arg1, arg2) {
    let v0;
    if (arg0 !== 0) {
        v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    }
    // generic import `logOptSlice`: [Option(Ref(Vector(U16))), U32] -> Unit
    logOptSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_0000000000000023(arg0, arg1, arg2) {
    let v0;
    if (arg0 !== 0) {
        v0 = getArrayJsValueFromWasm0(arg0, arg1);
        wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    }
    // generic import `logOptStrSlice`: [Option(Ref(Vector(NamedExternref("string")))), U32] -> Unit
    logOptStrSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_0000000000000024(arg0) {
    // generic import `logRef`: [Ref(Externref)] -> Unit
    logRef(arg0);
}
export function __wbindgen_generic_0000000000000025(arg0, arg1, arg2) {
    var v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    // generic import `logSlice`: [Ref(Vector(U16)), F64] -> Unit
    logSlice(v0, arg2);
}
export function __wbindgen_generic_0000000000000026(arg0, arg1, arg2) {
    var v0 = Array.from(getArrayU16FromWasm0(arg0, arg1));
    // generic import `logSlice`: [Ref(Vector(U16)), U32] -> Unit
    logSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_0000000000000027(arg0, arg1, arg2) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1);
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // generic import `logStrSlice`: [Ref(Vector(NamedExternref("string"))), U32] -> Unit
    logStrSlice(v0, arg2 >>> 0);
}
export function __wbindgen_generic_0000000000000028(arg0) {
    // generic import `log`: [F64] -> Unit
    log(arg0);
}
export function __wbindgen_generic_0000000000000029(arg0, arg1) {
    // generic import `log`: [Ref(String)] -> Unit
    log(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_generic_000000000000002a(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        // generic import `log`: [String] -> Unit
        log(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbindgen_generic_000000000000002b(arg0) {
    // generic import `log`: [U32] -> Unit
    log(arg0 >>> 0);
}
export function __wbindgen_generic_000000000000002c(arg0) {
    // generic import `console.log`: [U32] -> Unit
    console.log(arg0 >>> 0);
}
export function __wbindgen_generic_000000000000002d(arg0, arg1) {
    // generic import `mixImplTrait`: [F64, U32] -> Unit
    mixImplTrait(arg0, arg1 >>> 0);
}
export function __wbindgen_generic_000000000000002e(arg0, arg1) {
    // generic import `mixImplTrait`: [U32, U32] -> Unit
    mixImplTrait(arg0 >>> 0, arg1 >>> 0);
}
export function __wbindgen_generic_000000000000002f(arg0, arg1) {
    // generic import `mix`: [U32, F64] -> Unit
    mix(arg0 >>> 0, arg1);
}
export function __wbindgen_generic_0000000000000030(arg0, arg1) {
    // generic import `mix`: [U32, U32] -> Unit
    mix(arg0 >>> 0, arg1 >>> 0);
}
export function __wbindgen_generic_0000000000000031(arg0) {
    // generic import `new`: [F64] -> Externref
    const ret = new Widget(arg0);
    return ret;
}
export function __wbindgen_generic_0000000000000032(arg0) {
    // generic import `new`: [U32] -> Externref
    const ret = new Widget(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000033(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        // generic import `new_holder`: [String] -> Externref
        const ret = new Holder(getStringFromWasm0(arg0, arg1));
        return ret;
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbindgen_generic_0000000000000034(arg0) {
    // generic import `new_holder`: [U32] -> Externref
    const ret = new Holder(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000035(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        // generic import `of`: [String] -> Externref
        const ret = Holder.of(getStringFromWasm0(arg0, arg1));
        return ret;
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbindgen_generic_0000000000000036(arg0) {
    // generic import `of`: [U32] -> Externref
    const ret = Holder.of(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000037(arg0) {
    // generic import `of`: [F64] -> Externref
    const ret = Widget.of(arg0);
    return ret;
}
export function __wbindgen_generic_0000000000000038(arg0) {
    // generic import `of`: [U32] -> Externref
    const ret = Widget.of(arg0 >>> 0);
    return ret;
}
export function __wbindgen_generic_0000000000000039(arg0, arg1) {
    // generic import `pair`: [U32, F64] -> Unit
    pair(arg0 >>> 0, arg1);
}
export function __wbindgen_generic_000000000000003a(arg0, arg1) {
    // generic import `setImplTrait`: [Ref(Externref), F64] -> Unit
    arg0.setImplTrait(arg1);
}
export function __wbindgen_generic_000000000000003b(arg0, arg1) {
    // generic import `setImplTrait`: [Ref(Externref), U32] -> Unit
    arg0.setImplTrait(arg1 >>> 0);
}
export function __wbindgen_generic_000000000000003c(arg0, arg1) {
    // generic import `set`: [Ref(Externref), F64] -> Unit
    arg0.set(arg1);
}
export function __wbindgen_generic_000000000000003d(arg0, arg1) {
    // generic import `set`: [Ref(Externref), U32] -> Unit
    arg0.set(arg1 >>> 0);
}
export function __wbindgen_generic_000000000000003e(arg0, arg1, arg2, arg3) {
    // generic import `set_indexed`: [Ref(Externref), Ref(String), U32] -> Unit
    arg0[getStringFromWasm0(arg1, arg2)] = arg3 >>> 0;
}
export function __wbindgen_generic_000000000000003f(arg0, arg1) {
    // generic import `set_tag`: [Ref(Externref), F64] -> Unit
    arg0.tag = arg1;
}
export function __wbindgen_generic_0000000000000040(arg0, arg1) {
    // generic import `set_value`: [Ref(Externref), U32] -> Unit
    arg0.value = arg1 >>> 0;
}
export function __wbindgen_generic_0000000000000041(arg0, arg1, arg2) {
    // generic import `spreadGeneric`: [U32, Ref(Slice(U32))] -> Unit
    spreadGeneric(arg0 >>> 0, ...(getArrayU32FromWasm0(arg1, arg2)));
}
export function __wbindgen_generic_0000000000000042(arg0, arg1) {
    var v0 = getArrayU32FromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    // generic import `sumItems`: [Vector(U32)] -> F64
    const ret = sumItems(v0);
    return ret;
}
export function __wbindgen_generic_0000000000000043(arg0) {
    // generic import `tag`: [Ref(Externref)] -> U32
    const ret = arg0.tag;
    return ret;
}
export function __wbindgen_generic_0000000000000044() { return handleError(function (arg0) {
    // generic import `tryGet`: [U32] -> F64
    const ret = tryGet(arg0 >>> 0);
    return ret;
}, arguments); }
export function __wbindgen_generic_0000000000000045() { return handleError(function (arg0) {
    // generic import `tryGet`: [U32] -> U32
    const ret = tryGet(arg0 >>> 0);
    return ret;
}, arguments); }
export function __wbindgen_generic_0000000000000046() { return handleError(function (arg0) {
    // generic import `tryLog`: [U32] -> Unit
    tryLog(arg0 >>> 0);
}, arguments); }
export function __wbindgen_generic_0000000000000047(arg0) {
    // generic import `value`: [Ref(Externref)] -> F64
    const ret = arg0.value;
    return ret;
}
export function __wbindgen_generic_0000000000000048(arg0) {
    // generic import `value`: [Ref(Externref)] -> U32
    const ret = arg0.value;
    return ret;
}
export function __wbindgen_generic_0000000000000049(arg0, arg1, arg2) {
    var v0 = getArrayU32FromWasm0(arg1, arg2).slice();
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    // generic import `variadicLog`: [U32, Vector(U32)] -> Unit
    variadicLog(arg0 >>> 0, ...(v0));
}
export function __wbindgen_generic_000000000000004a(arg0, arg1, arg2) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen_000000000000004b___convert__closures_____invoke___u32______true_(a, state0.b, arg0);
            } finally {
                state0.a = a;
            }
        };
        // generic import `withCallback`: [RefMut(Function(Function { arguments: [U32], shim_idx: 82, ret: Unit, inner_ret: Some(Unit) })), U32] -> Unit
        withCallback(cb0, arg2 >>> 0);
    } finally {
        state0.a = 0;
    }
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
function wasm_bindgen_000000000000004b___convert__closures_____invoke___bool__true_(arg0, arg1) {
    const ret = wasm.wasm_bindgen_000000000000004b___convert__closures_____invoke___bool__true_(arg0, arg1);
    return ret !== 0;
}

function wasm_bindgen_000000000000004b___convert__closures_____invoke___wasm_bindgen_000000000000004b___JsValue__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen_000000000000004b___convert__closures_____invoke___wasm_bindgen_000000000000004b___JsValue__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen_000000000000004b___convert__closures_____invoke___js_sys_000000000000004c___Function_fn_wasm_bindgen_000000000000004b___JsValue_____wasm_bindgen_000000000000004b___sys__Undefined___js_sys_000000000000004c___Function_fn_wasm_bindgen_000000000000004b___JsValue_____wasm_bindgen_000000000000004b___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen_000000000000004b___convert__closures_____invoke___js_sys_000000000000004c___Function_fn_wasm_bindgen_000000000000004b___JsValue_____wasm_bindgen_000000000000004b___sys__Undefined___js_sys_000000000000004c___Function_fn_wasm_bindgen_000000000000004b___JsValue_____wasm_bindgen_000000000000004b___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
}

function wasm_bindgen_000000000000004b___convert__closures_____invoke___u32______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_000000000000004b___convert__closures_____invoke___u32______true_(arg0, arg1, arg2);
}

function wasm_bindgen_000000000000004b___convert__closures_____invoke___u32__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen_000000000000004b___convert__closures_____invoke___u32__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen_000000000000004b___convert__closures_____invoke___alloc_000000000000004e___string__String__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_(arg0, arg1, arg2) {
    const ptr0 = passStringToWasm0(arg2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_bindgen_000000000000004b___convert__closures_____invoke___alloc_000000000000004e___string__String__core_000000000000004d___result__Result_____wasm_bindgen_000000000000004b___JsError___true_(arg0, arg1, ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
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

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint16ArrayMemory0 = null;
function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
    return {};
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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
