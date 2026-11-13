export class Point {
    static __wrap(ptr) {
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_point_x(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point_x(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

/**
 * @param {Uint8Array | number[]} v
 * @returns {Uint8Array}
 */
export function bytes(v) {
    const ptr0 = passArray8ToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.bytes(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {boolean[]} v
 * @returns {boolean[]}
 */
export function flags(v) {
    let len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, e0, true);
        }
    } catch (e) {
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.flags(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const b2 = p2;
    try {
        const view2 = getDataViewMemory0();
        for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
            const w2_0 = view2.getInt32(p2 + 0, true);
            v2.push(w2_0 !== 0);
        }
    } finally {
        if (b2) wasm.__wbindgen_free(b2, ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {Float64Array | number[]} v
 * @returns {Float64Array}
 */
export function floats(v) {
    const ptr0 = passArrayF64ToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.floats(ptr0, len0);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {any[]} v
 * @returns {any[]}
 */
export function foos(v) {
    const ptr0 = passArrayJsValueToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.foos(ptr0, len0);
    var v2;
    try {
        v2 = getArrayJsValueFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @returns {(string[])[]}
 */
export function give_nested_strings() {
    const ret = wasm.give_nested_strings();
    const v1 = [];
    let p1 = ret[0];
    p1 = p1 >>> 0;
    const b1 = p1;
    try {
        const view1 = getDataViewMemory0();
        for (const end1 = p1 + ret[1] * 8; p1 < end1; p1 += 8) {
            const w1_0 = view1.getInt32(p1 + 0, true);
            const w1_1 = view1.getInt32(p1 + 4, true);
            const v2 = [];
            let p2 = w1_0;
            p2 = p2 >>> 0;
            const view2 = getDataViewMemory0();
            for (const end2 = p2 + w1_1 * 8; p2 < end2; p2 += 8) {
                const w2_0 = view2.getInt32(p2 + 0, true);
                const w2_1 = view2.getInt32(p2 + 4, true);
                v2.push(getStringFromWasm0(w2_0, w2_1));
            }
            v1.push(v2);
        }
    } finally {
        if (b1) {
            for (let k3 = 0; k3 < ret[1]; k3++) {
                const cell3 = b1 + k3 * 8;
                const w3_0 = getDataViewMemory0().getInt32(cell3 + 0, true) >>> 0;
                const w3_1 = getDataViewMemory0().getInt32(cell3 + 4, true) >>> 0;
                if (w3_0) {
                    for (let k4 = 0; k4 < w3_1; k4++) {
                        const cell4 = w3_0 + k4 * 8;
                        const w4_0 = getDataViewMemory0().getInt32(cell4 + 0, true) >>> 0;
                        const w4_1 = getDataViewMemory0().getInt32(cell4 + 4, true) >>> 0;
                        if (w4_0) { wasm.__wbindgen_free(w4_0, w4_1 * 1, 1); }
                    }
                    wasm.__wbindgen_free(w3_0, w3_1 * 8, 4);
                }
            }
            wasm.__wbindgen_free(b1, ret[1] * 8, 4);
        }
    }
    return v1;
}

export function imports() {
    wasm.imports();
}

/**
 * @param {(Uint8Array | number[])[]} v
 * @returns {Uint8Array[]}
 */
export function nested(v) {
    for (let n0 = 0; n0 < v.length; n0++) {
        const e0 = v[n0];
        if (!Array.isArray(e0) && !ArrayBuffer.isView(e0)) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            const ptr1 = passArray8ToWasm0(e0, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 8;
            view0.setInt32(p0 + 0, ptr1, true);
            view0.setInt32(p0 + 4, len1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 8;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                const w2_1 = getDataViewMemory0().getInt32(cell2 + 4, true) >>> 0;
                if (w2_0) { wasm.__wbindgen_free(w2_0, w2_1 * 1, 1); }
            }
            wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        }
        throw e;
    }
    const ret = wasm.nested(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    const b4 = p4;
    try {
        const view4 = getDataViewMemory0();
        for (const end4 = p4 + ret[1] * 8; p4 < end4; p4 += 8) {
            const w4_0 = view4.getInt32(p4 + 0, true);
            const w4_1 = view4.getInt32(p4 + 4, true);
            v4.push(getArrayU8FromWasm0(w4_0, w4_1).slice());
        }
    } finally {
        if (b4) {
            for (let k5 = 0; k5 < ret[1]; k5++) {
                const cell5 = b4 + k5 * 8;
                const w5_0 = getDataViewMemory0().getInt32(cell5 + 0, true) >>> 0;
                const w5_1 = getDataViewMemory0().getInt32(cell5 + 4, true) >>> 0;
                if (w5_0) { wasm.__wbindgen_free(w5_0, w5_1 * 1, 1); }
            }
            wasm.__wbindgen_free(b4, ret[1] * 8, 4);
        }
    }
    return v4;
}

/**
 * @param {(number | null | undefined)[]} v
 * @returns {(number | undefined)[]}
 */
export function opt_bytes(v) {
    let len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, isLikeNone(e0) ? 0xFFFFFF : e0, true);
        }
    } catch (e) {
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.opt_bytes(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const b2 = p2;
    try {
        const view2 = getDataViewMemory0();
        for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
            const w2_0 = view2.getInt32(p2 + 0, true);
            v2.push(w2_0 === 0xFFFFFF ? undefined : w2_0);
        }
    } finally {
        if (b2) wasm.__wbindgen_free(b2, ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {(any | null | undefined)[]} v
 * @returns {(any | undefined)[]}
 */
export function opt_foos(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            const w0_0 = isLikeNone(e0) ? 0 : addToExternrefTable0(e0);
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, w0_0, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k1 = 0; k1 < j0; k1++) {
                const cell1 = ptr0 + k1 * 4;
                const w1_0 = getDataViewMemory0().getInt32(cell1 + 0, true) >>> 0;
                wasm.__externref_table_dealloc(w1_0);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.opt_foos(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    const b3 = p3;
    try {
        const view3 = getDataViewMemory0();
        for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
            const w3_0 = view3.getInt32(p3 + 0, true);
            v3.push(wasm.__wbindgen_externrefs.get(w3_0));
        }
    } finally {
        if (b3) {
            for (let k4 = 0; k4 < ret[1]; k4++) {
                const cell4 = b3 + k4 * 4;
                const w4_0 = getDataViewMemory0().getInt32(cell4 + 0, true) >>> 0;
                wasm.__externref_table_dealloc(w4_0);
            }
            wasm.__wbindgen_free(b3, ret[1] * 4, 4);
        }
    }
    return v3;
}

/**
 * @param {Point[]} v
 * @returns {Point[]}
 */
export function points(v) {
    for (let n0 = 0; n0 < v.length; n0++) {
        const e0 = v[n0];
        if (!(e0 instanceof Point)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Point);
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            _assertClass(e0, Point);
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_point_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.points(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    const b4 = p4;
    try {
        const view4 = getDataViewMemory0();
        for (const end4 = p4 + ret[1] * 4; p4 < end4; p4 += 4) {
            const w4_0 = view4.getInt32(p4 + 0, true);
            const obj5 = Point.__wrap(w4_0);
            getDataViewMemory0().setInt32(p4 + 0, 0, true);
            v4.push(obj5);
        }
    } finally {
        if (b4) {
            for (let k6 = 0; k6 < ret[1]; k6++) {
                const cell6 = b4 + k6 * 4;
                const w6_0 = getDataViewMemory0().getInt32(cell6 + 0, true) >>> 0;
                if (w6_0) wasm.__wbg_point_free(w6_0, 1);
            }
            wasm.__wbindgen_free(b4, ret[1] * 4, 4);
        }
    }
    return v4;
}

/**
 * @param {string[]} v
 * @returns {string[]}
 */
export function strings(v) {
    for (let n0 = 0; n0 < v.length; n0++) {
        const e0 = v[n0];
        if (typeof e0 !== 'string') throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            const ptr1 = passStringToWasm0(e0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 8;
            view0.setInt32(p0 + 0, ptr1, true);
            view0.setInt32(p0 + 4, len1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 8;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                const w2_1 = getDataViewMemory0().getInt32(cell2 + 4, true) >>> 0;
                if (w2_0) { wasm.__wbindgen_free(w2_0, w2_1 * 1, 1); }
            }
            wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        }
        throw e;
    }
    const ret = wasm.strings(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    const b4 = p4;
    try {
        const view4 = getDataViewMemory0();
        for (const end4 = p4 + ret[1] * 8; p4 < end4; p4 += 8) {
            const w4_0 = view4.getInt32(p4 + 0, true);
            const w4_1 = view4.getInt32(p4 + 4, true);
            v4.push(getStringFromWasm0(w4_0, w4_1));
        }
    } finally {
        if (b4) {
            for (let k5 = 0; k5 < ret[1]; k5++) {
                const cell5 = b4 + k5 * 8;
                const w5_0 = getDataViewMemory0().getInt32(cell5 + 0, true) >>> 0;
                const w5_1 = getDataViewMemory0().getInt32(cell5 + 4, true) >>> 0;
                if (w5_0) { wasm.__wbindgen_free(w5_0, w5_1 * 1, 1); }
            }
            wasm.__wbindgen_free(b4, ret[1] * 8, 4);
        }
    }
    return v4;
}

/**
 * @param {any[]} v
 * @returns {any[]}
 */
export function values(v) {
    const ptr0 = passArrayJsValueToWasm0(v, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.values(ptr0, len0);
    var v2;
    try {
        v2 = getArrayJsValueFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    }
    return v2;
}

/**
 * @param {bigint[]} v
 * @returns {bigint[]}
 */
export function wide(v) {
    let len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 16, 8) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const p0 = ptr0 + j0 * 16;
            view0.setBigInt64(p0 + 0, e0, true);
            view0.setBigInt64(p0 + 8, e0 >> BigInt(64), true);
        }
    } catch (e) {
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 16, 8);
        throw e;
    }
    const ret = wasm.wide(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const b2 = p2;
    try {
        const view2 = getDataViewMemory0();
        for (const end2 = p2 + ret[1] * 16; p2 < end2; p2 += 16) {
            const w2_0 = view2.getBigInt64(p2 + 0, true);
            const w2_1 = view2.getBigInt64(p2 + 8, true);
            v2.push((BigInt.asUintN(64, w2_0) | (BigInt.asUintN(64, w2_1) << BigInt(64))));
        }
    } finally {
        if (b2) wasm.__wbindgen_free(b2, ret[1] * 16, 8);
    }
    return v2;
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_09736879a3f67559(arg0) {
    arg0._wbg_cb_unref();
}
export function __wbg_give_bytes_c33f23bab560542b(arg0) {
    const ret = give_bytes();
    const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
}
export function __wbg_give_foos_957d8725438c15aa(arg0) {
    const ret = give_foos();
    const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
    getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
}
export function __wbg_give_opt_bytes_60d73cb3d49a34f5(arg0) {
    const ret = give_opt_bytes();
    let len1, ptr1;
    try {
        len1 = ret.length;
        ptr1 = wasm.__wbindgen_malloc(len1 * 4, 4) >>> 0;
        const view1 = getDataViewMemory0();
        for (let j1 = 0; j1 < len1; j1++) {
            const e1 = ret[j1];
            const p1 = ptr1 + j1 * 4;
            view1.setInt32(p1 + 0, isLikeNone(e1) ? 0xFFFFFF : e1, true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
    } catch (e) {
        if (ptr1) wasm.__wbindgen_free(ptr1, len1 * 4, 4);
        throw e;
    }
}
export function __wbg_give_opt_foos_89f792e16b863db6(arg0) {
    const ret = give_opt_foos();
    let j1 = 0, len1, ptr1;
    try {
        len1 = ret.length;
        ptr1 = wasm.__wbindgen_malloc(len1 * 4, 4) >>> 0;
        for (; j1 < len1; j1++) {
            const e1 = ret[j1];
            const w1_0 = isLikeNone(e1) ? 0 : addToExternrefTable0(e1);
            const view1 = getDataViewMemory0();
            const p1 = ptr1 + j1 * 4;
            view1.setInt32(p1 + 0, w1_0, true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
    } catch (e) {
        if (ptr1) {
            for (let k2 = 0; k2 < j1; k2++) {
                const cell2 = ptr1 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                wasm.__externref_table_dealloc(w2_0);
            }
            wasm.__wbindgen_free(ptr1, len1 * 4, 4);
        }
        throw e;
    }
}
export function __wbg_give_strings_6da2627b8b5e4e91(arg0) {
    const ret = give_strings();
    let j1 = 0, len1, ptr1;
    try {
        for (let n1 = 0; n1 < ret.length; n1++) {
            const e1 = ret[n1];
            if (typeof e1 !== 'string') throw new Error('array contains a value of the wrong type');
        }
        len1 = ret.length;
        ptr1 = wasm.__wbindgen_malloc(len1 * 8, 4) >>> 0;
        for (; j1 < len1; j1++) {
            const e1 = ret[j1];
            const ptr2 = passStringToWasm0(e1, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            const view1 = getDataViewMemory0();
            const p1 = ptr1 + j1 * 8;
            view1.setInt32(p1 + 0, ptr2, true);
            view1.setInt32(p1 + 4, len2, true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
    } catch (e) {
        if (ptr1) {
            for (let k3 = 0; k3 < j1; k3++) {
                const cell3 = ptr1 + k3 * 8;
                const w3_0 = getDataViewMemory0().getInt32(cell3 + 0, true) >>> 0;
                const w3_1 = getDataViewMemory0().getInt32(cell3 + 4, true) >>> 0;
                if (w3_0) { wasm.__wbindgen_free(w3_0, w3_1 * 1, 1); }
            }
            wasm.__wbindgen_free(ptr1, len1 * 8, 4);
        }
        throw e;
    }
}
export function __wbg_take_borrowed_bytes_6797222e0fd108d2(arg0, arg1) {
    take_borrowed_bytes(getArrayU8FromWasm0(arg0, arg1));
}
export function __wbg_take_borrowed_closures_ecfd86a190534640(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const b0 = p0;
    try {
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            v0.push(wasm.__wbindgen_externrefs.get(w0_0));
        }
    } finally {
        if (b0) wasm.__wbindgen_free(b0, arg1 * 4, 4);
    }
    take_borrowed_closures(v0);
}
export function __wbg_take_borrowed_floats_30475c7407955193(arg0, arg1) {
    take_borrowed_floats(getArrayF64FromWasm0(arg0, arg1));
}
export function __wbg_take_borrowed_foos_7e4a629107e7f1a8(arg0, arg1) {
    take_borrowed_foos(getArrayJsValueViewFromWasm0(arg0, arg1));
}
export function __wbg_take_borrowed_strings_d2f788fd08c3a59d(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const b0 = p0;
    try {
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            const w0_1 = view0.getInt32(p0 + 4, true);
            v0.push(getStringFromWasm0(w0_0, w0_1));
        }
    } finally {
        if (b0) wasm.__wbindgen_free(b0, arg1 * 8, 4);
    }
    take_borrowed_strings(v0);
}
export function __wbg_take_borrowed_values_32b41a9c5f1b820f(arg0, arg1) {
    take_borrowed_values(getArrayJsValueViewFromWasm0(arg0, arg1));
}
export function __wbg_take_bytes_0be694c4de362f6b(arg0, arg1) {
    var v0 = getArrayU8FromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 1, 1);
    take_bytes(v0);
}
export function __wbg_take_foos_a6f1ce7bf0b6785b(arg0, arg1) {
    var v0;
    try {
        v0 = getArrayJsValueFromWasm0(arg0, arg1);
    } finally {
        wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    }
    take_foos(v0);
}
export function __wbg_take_opt_bytes_f444cd63983812da(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const b0 = p0;
    try {
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            v0.push(w0_0 === 0xFFFFFF ? undefined : w0_0);
        }
    } finally {
        if (b0) wasm.__wbindgen_free(b0, arg1 * 4, 4);
    }
    take_opt_bytes(v0);
}
export function __wbg_take_opt_foos_cba1deacf9e0e96b(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const b0 = p0;
    try {
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 4; p0 < end0; p0 += 4) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            v0.push(wasm.__wbindgen_externrefs.get(w0_0));
        }
    } finally {
        if (b0) {
            for (let k1 = 0; k1 < arg1; k1++) {
                const cell1 = b0 + k1 * 4;
                const w1_0 = getDataViewMemory0().getInt32(cell1 + 0, true) >>> 0;
                wasm.__externref_table_dealloc(w1_0);
            }
            wasm.__wbindgen_free(b0, arg1 * 4, 4);
        }
    }
    take_opt_foos(v0);
}
export function __wbg_take_strings_7ffdcb3cae111236(arg0, arg1) {
    const v0 = [];
    let p0 = arg0;
    p0 = p0 >>> 0;
    const b0 = p0;
    try {
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            const w0_1 = view0.getInt32(p0 + 4, true);
            v0.push(getStringFromWasm0(w0_0, w0_1));
        }
    } finally {
        if (b0) {
            for (let k1 = 0; k1 < arg1; k1++) {
                const cell1 = b0 + k1 * 8;
                const w1_0 = getDataViewMemory0().getInt32(cell1 + 0, true) >>> 0;
                const w1_1 = getDataViewMemory0().getInt32(cell1 + 4, true) >>> 0;
                if (w1_0) { wasm.__wbindgen_free(w1_0, w1_1 * 1, 1); }
            }
            wasm.__wbindgen_free(b0, arg1 * 8, 4);
        }
    }
    take_strings(v0);
}
export function __wbindgen_generic_0000000000000000(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 2, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
    const ret = makeClosure(arg0, arg1, wasm_bindgen_0000000000000001___convert__closures_____invoke_______true_);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
function wasm_bindgen_0000000000000001___convert__closures_____invoke_______true_(arg0, arg1) {
    wasm.wasm_bindgen_0000000000000001___convert__closures_____invoke_______true_(arg0, arg1);
}

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr, 1));

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

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    try {
        for (let i = ptr; i < ptr + 4 * len; i += 4) {
            result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
        }
    } finally {
        wasm.__externref_drop_slice(ptr, len);
    }
    return result;
}

function getArrayJsValueViewFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    return result;
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

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
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

function makeClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
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

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
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

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(bytes) {
    numBytesDecoded += bytes.length;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
        cachedTextDecoder.decode();
        numBytesDecoded = bytes.length;
    }
    return cachedTextDecoder.decode(bytes);
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
