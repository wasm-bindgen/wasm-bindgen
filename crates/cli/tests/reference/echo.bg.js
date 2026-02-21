export class Foo {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Foo.prototype);
        obj.__wbg_ptr = ptr;
        FooFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    static __unwrap(jsValue) {
        if (!(jsValue instanceof Foo)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FooFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo_free(ptr, 0);
    }
}
if (Symbol.dispose) Foo.prototype[Symbol.dispose] = Foo.prototype.free;

/**
 * @param {boolean} a
 * @returns {boolean}
 */
export function echo_bool(a) {
    const ret = wasm.echo_bool(a);
    return ret !== 0;
}

/**
 * @param {string} a
 * @returns {string}
 */
export function echo_char(a) {
    const char0 = a.codePointAt(0);
    _assertChar(char0);
    const ret = wasm.echo_char(char0);
    return String.fromCodePoint(ret);
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_f32(a) {
    const ret = wasm.echo_f32(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_f64(a) {
    const ret = wasm.echo_f64(a);
    return ret;
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i128(a) {
    const ret = wasm.echo_i128(a, a >> BigInt(64));
    return (BigInt.asUintN(64, ret[0]) | (ret[1] << BigInt(64)));
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i16(a) {
    const ret = wasm.echo_i16(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i32(a) {
    const ret = wasm.echo_i32(a);
    return ret;
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i64(a) {
    const ret = wasm.echo_i64(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i8(a) {
    const ret = wasm.echo_i8(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_isize(a) {
    const ret = wasm.echo_isize(a);
    return ret;
}

/**
 * @param {boolean | null} [a]
 * @returns {boolean | undefined}
 */
export function echo_option_bool(a) {
    const ret = wasm.echo_option_bool(isLikeNone(a) ? 0xFFFFFF : a ? 1 : 0);
    return ret === 0xFFFFFF ? undefined : ret !== 0;
}

/**
 * @param {string | null} [a]
 * @returns {string | undefined}
 */
export function echo_option_char(a) {
    const char0 = isLikeNone(a) ? 0xFFFFFF : a.codePointAt(0);
    if (char0 !== 0xFFFFFF) { _assertChar(char0); }
    const ret = wasm.echo_option_char(char0);
    return ret === 0xFFFFFF ? undefined : String.fromCodePoint(ret);
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_f32(a) {
    const ret = wasm.echo_option_f32(isLikeNone(a) ? 0x100000001 : Math.fround(a));
    return ret === 0x100000001 ? undefined : ret;
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_f64(a) {
    const ret = wasm.echo_option_f64(!isLikeNone(a), isLikeNone(a) ? 0 : a);
    return ret[0] === 0 ? undefined : ret[1];
}

/**
 * @param {bigint | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i128(a) {
    const ret = wasm.echo_option_i128(!isLikeNone(a), isLikeNone(a) ? BigInt(0) : a, isLikeNone(a) ? BigInt(0) : a >> BigInt(64));
    return ret[0] === 0 ? undefined : (BigInt.asUintN(64, ret[1]) | (ret[2] << BigInt(64)));
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i16(a) {
    const ret = wasm.echo_option_i16(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i32(a) {
    const ret = wasm.echo_option_i32(isLikeNone(a) ? 0x100000001 : (a) >> 0);
    return ret === 0x100000001 ? undefined : ret;
}

/**
 * @param {bigint | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i64(a) {
    const ret = wasm.echo_option_i64(!isLikeNone(a), isLikeNone(a) ? BigInt(0) : a);
    return ret[0] === 0 ? undefined : ret[1];
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i8(a) {
    const ret = wasm.echo_option_i8(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_isize(a) {
    const ret = wasm.echo_option_isize(isLikeNone(a) ? 0x100000001 : (a) >> 0);
    return ret === 0x100000001 ? undefined : ret;
}

/**
 * @param {string | null} [a]
 * @returns {string | undefined}
 */
export function echo_option_string(a) {
    var ptr0 = isLikeNone(a) ? 0 : passStringToWasm0(a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_string(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {Foo | null} [a]
 * @returns {Foo | undefined}
 */
export function echo_option_struct(a) {
    let ptr0 = 0;
    if (!isLikeNone(a)) {
        _assertClass(a, Foo);
        ptr0 = a.__destroy_into_raw();
    }
    const ret = wasm.echo_option_struct(ptr0);
    return ret === 0 ? undefined : Foo.__wrap(ret);
}

/**
 * @param {bigint | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u128(a) {
    const ret = wasm.echo_option_u128(!isLikeNone(a), isLikeNone(a) ? BigInt(0) : a, isLikeNone(a) ? BigInt(0) : a >> BigInt(64));
    return ret[0] === 0 ? undefined : (BigInt.asUintN(64, ret[1]) | (BigInt.asUintN(64, ret[2]) << BigInt(64)));
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u16(a) {
    const ret = wasm.echo_option_u16(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u32(a) {
    const ret = wasm.echo_option_u32(isLikeNone(a) ? 0x100000001 : (a) >>> 0);
    return ret === 0x100000001 ? undefined : ret;
}

/**
 * @param {bigint | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u64(a) {
    const ret = wasm.echo_option_u64(!isLikeNone(a), isLikeNone(a) ? BigInt(0) : a);
    return ret[0] === 0 ? undefined : BigInt.asUintN(64, ret[1]);
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u8(a) {
    const ret = wasm.echo_option_u8(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_usize(a) {
    const ret = wasm.echo_option_usize(isLikeNone(a) ? 0x100000001 : (a) >>> 0);
    return ret === 0x100000001 ? undefined : ret;
}

/**
 * @param {Int16Array | null} [a]
 * @returns {Int16Array | undefined}
 */
export function echo_option_vec_i16(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_i16(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI16FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    }
    return v2;
}

/**
 * @param {Int32Array | null} [a]
 * @returns {Int32Array | undefined}
 */
export function echo_option_vec_i32(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_i32(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {BigInt64Array | null} [a]
 * @returns {BigInt64Array | undefined}
 */
export function echo_option_vec_i64(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_i64(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    }
    return v2;
}

/**
 * @param {Int8Array | null} [a]
 * @returns {Int8Array | undefined}
 */
export function echo_option_vec_i8(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_i8(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {string[] | null} [a]
 * @returns {string[] | undefined}
 */
export function echo_option_vec_string(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_string(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {Foo[] | null} [a]
 * @returns {Foo[] | undefined}
 */
export function echo_option_vec_struct(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_struct(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {Uint16Array | null} [a]
 * @returns {Uint16Array | undefined}
 */
export function echo_option_vec_u16(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_u16(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU16FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    }
    return v2;
}

/**
 * @param {Uint32Array | null} [a]
 * @returns {Uint32Array | undefined}
 */
export function echo_option_vec_u32(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_u32(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {BigUint64Array | null} [a]
 * @returns {BigUint64Array | undefined}
 */
export function echo_option_vec_u64(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_u64(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    }
    return v2;
}

/**
 * @param {Uint8Array | null} [a]
 * @returns {Uint8Array | undefined}
 */
export function echo_option_vec_u8(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_u8(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {Int16Array | null} [a]
 * @returns {Int16Array | undefined}
 */
export function echo_option_vec_uninit_i16(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_i16(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI16FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    }
    return v2;
}

/**
 * @param {Int32Array | null} [a]
 * @returns {Int32Array | undefined}
 */
export function echo_option_vec_uninit_i32(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_i32(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {BigInt64Array | null} [a]
 * @returns {BigInt64Array | undefined}
 */
export function echo_option_vec_uninit_i64(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_i64(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    }
    return v2;
}

/**
 * @param {Int8Array | null} [a]
 * @returns {Int8Array | undefined}
 */
export function echo_option_vec_uninit_i8(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_i8(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayI8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {Uint16Array | null} [a]
 * @returns {Uint16Array | undefined}
 */
export function echo_option_vec_uninit_u16(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_u16(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU16FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    }
    return v2;
}

/**
 * @param {Uint32Array | null} [a]
 * @returns {Uint32Array | undefined}
 */
export function echo_option_vec_uninit_u32(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_u32(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {BigUint64Array | null} [a]
 * @returns {BigUint64Array | undefined}
 */
export function echo_option_vec_uninit_u64(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_u64(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    }
    return v2;
}

/**
 * @param {Uint8Array | null} [a]
 * @returns {Uint8Array | undefined}
 */
export function echo_option_vec_uninit_u8(a) {
    var ptr0 = isLikeNone(a) ? 0 : passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_option_vec_uninit_u8(ptr0, len0);
    let v2;
    if (ret[0] !== 0) {
        v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {string} a
 * @returns {string}
 */
export function echo_string(a) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.echo_string(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {Foo} a
 * @returns {Foo}
 */
export function echo_struct(a) {
    _assertClass(a, Foo);
    var ptr0 = a.__destroy_into_raw();
    const ret = wasm.echo_struct(ptr0);
    return Foo.__wrap(ret);
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u128(a) {
    const ret = wasm.echo_u128(a, a >> BigInt(64));
    return (BigInt.asUintN(64, ret[0]) | (BigInt.asUintN(64, ret[1]) << BigInt(64)));
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u16(a) {
    const ret = wasm.echo_u16(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u32(a) {
    const ret = wasm.echo_u32(a);
    return ret >>> 0;
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u64(a) {
    const ret = wasm.echo_u64(a);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u8(a) {
    const ret = wasm.echo_u8(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_usize(a) {
    const ret = wasm.echo_usize(a);
    return ret >>> 0;
}

/**
 * @param {Int16Array} a
 * @returns {Int16Array}
 */
export function echo_vec_i16(a) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_i16(ptr0, len0);
    var v2 = getArrayI16FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    return v2;
}

/**
 * @param {Int32Array} a
 * @returns {Int32Array}
 */
export function echo_vec_i32(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_i32(ptr0, len0);
    var v2 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {BigInt64Array} a
 * @returns {BigInt64Array}
 */
export function echo_vec_i64(a) {
    const ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_i64(ptr0, len0);
    var v2 = getArrayI64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Int8Array} a
 * @returns {Int8Array}
 */
export function echo_vec_i8(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_i8(ptr0, len0);
    var v2 = getArrayI8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {string[]} a
 * @returns {string[]}
 */
export function echo_vec_string(a) {
    const ptr0 = passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_string(ptr0, len0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {Foo[]} a
 * @returns {Foo[]}
 */
export function echo_vec_struct(a) {
    const ptr0 = passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_struct(ptr0, len0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {Uint16Array} a
 * @returns {Uint16Array}
 */
export function echo_vec_u16(a) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_u16(ptr0, len0);
    var v2 = getArrayU16FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    return v2;
}

/**
 * @param {Uint32Array} a
 * @returns {Uint32Array}
 */
export function echo_vec_u32(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_u32(ptr0, len0);
    var v2 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {BigUint64Array} a
 * @returns {BigUint64Array}
 */
export function echo_vec_u64(a) {
    const ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_u64(ptr0, len0);
    var v2 = getArrayU64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} a
 * @returns {Uint8Array}
 */
export function echo_vec_u8(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_u8(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Int16Array} a
 * @returns {Int16Array}
 */
export function echo_vec_uninit_i16(a) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_i16(ptr0, len0);
    var v2 = getArrayI16FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    return v2;
}

/**
 * @param {Int32Array} a
 * @returns {Int32Array}
 */
export function echo_vec_uninit_i32(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_i32(ptr0, len0);
    var v2 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {BigInt64Array} a
 * @returns {BigInt64Array}
 */
export function echo_vec_uninit_i64(a) {
    const ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_i64(ptr0, len0);
    var v2 = getArrayI64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Int8Array} a
 * @returns {Int8Array}
 */
export function echo_vec_uninit_i8(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_i8(ptr0, len0);
    var v2 = getArrayI8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Uint16Array} a
 * @returns {Uint16Array}
 */
export function echo_vec_uninit_u16(a) {
    const ptr0 = passArray16ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_u16(ptr0, len0);
    var v2 = getArrayU16FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 2, 2);
    return v2;
}

/**
 * @param {Uint32Array} a
 * @returns {Uint32Array}
 */
export function echo_vec_uninit_u32(a) {
    const ptr0 = passArray32ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_u32(ptr0, len0);
    var v2 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {BigUint64Array} a
 * @returns {BigUint64Array}
 */
export function echo_vec_uninit_u64(a) {
    const ptr0 = passArray64ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_u64(ptr0, len0);
    var v2 = getArrayU64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} a
 * @returns {Uint8Array}
 */
export function echo_vec_uninit_u8(a) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.echo_vec_uninit_u8(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}
export function __wbg___wbindgen_debug_string_43c7ccb034739216(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_string_get_d09f733449cbf7a2(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_df03e93053e0f4bc(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_foo_new(arg0) {
    const ret = Foo.__wrap(arg0);
    return ret;
}
export function __wbg_foo_unwrap(arg0) {
    const ret = Foo.__unwrap(arg0);
    return ret;
}
export function __wbindgen_cast_0000000000000000(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
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
const FooFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertChar(c) {
    if (typeof(c) === 'number' && (c >= 0x110000 || (c >= 0xD800 && c < 0xE000))) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

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

function getArrayI16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getBigInt64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayI8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
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

function getArrayU64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getBigUint64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedBigInt64ArrayMemory0 = null;
function getBigInt64ArrayMemory0() {
    if (cachedBigInt64ArrayMemory0 === null || cachedBigInt64ArrayMemory0.byteLength === 0) {
        cachedBigInt64ArrayMemory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64ArrayMemory0;
}

let cachedBigUint64ArrayMemory0 = null;
function getBigUint64ArrayMemory0() {
    if (cachedBigUint64ArrayMemory0 === null || cachedBigUint64ArrayMemory0.byteLength === 0) {
        cachedBigUint64ArrayMemory0 = new BigUint64Array(wasm.memory.buffer);
    }
    return cachedBigUint64ArrayMemory0;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedInt16ArrayMemory0 = null;
function getInt16ArrayMemory0() {
    if (cachedInt16ArrayMemory0 === null || cachedInt16ArrayMemory0.byteLength === 0) {
        cachedInt16ArrayMemory0 = new Int16Array(wasm.memory.buffer);
    }
    return cachedInt16ArrayMemory0;
}

let cachedInt32ArrayMemory0 = null;
function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

let cachedInt8ArrayMemory0 = null;
function getInt8ArrayMemory0() {
    if (cachedInt8ArrayMemory0 === null || cachedInt8ArrayMemory0.byteLength === 0) {
        cachedInt8ArrayMemory0 = new Int8Array(wasm.memory.buffer);
    }
    return cachedInt8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
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

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray16ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 2, 2) >>> 0;
    getUint16ArrayMemory0().set(arg, ptr / 2);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getBigUint64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
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
