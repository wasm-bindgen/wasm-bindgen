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
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export function driver() {
    wasm.driver();
}

/**
 * @param {(number | null | undefined)[]} v
 * @returns {(number | undefined)[]}
 */
export function maybe_bytes(v) {
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
    const ret = wasm.maybe_bytes(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
        v2.push(view2.getInt32(p2 + 0, true) === 0xFFFFFF ? undefined : view2.getInt32(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {(any | null | undefined)[]} v
 * @returns {(any | undefined)[]}
 */
export function maybe_foos(v) {
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
    const ret = wasm.maybe_foos(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
        const view3 = getDataViewMemory0();
        const w3_0 = view3.getInt32(p3 + 0, true);
        v3.push(takeFromExternrefTable0(w3_0));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v3;
}

/**
 * @param {(string | null | undefined)[] | null} [v]
 * @returns {(string | undefined)[] | undefined}
 */
export function maybe_names(v) {
    let j0 = 0, ptr0 = 0, len0 = 0;
    try {
        if (!isLikeNone(v)) {
            len0 = v.length;
            ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
            for (; j0 < len0; j0++) {
                const e0 = v[j0];
                if (!(isLikeNone(e0)) && typeof e0 !== 'string') throw new Error('array contains a value of the wrong type');
                var ptr1 = isLikeNone(e0) ? 0 : passStringToWasm0(e0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                const view0 = getDataViewMemory0();
                const p0 = ptr0 + j0 * 8;
                view0.setInt32(p0 + 0, ptr1, true);
                view0.setInt32(p0 + 4, len1, true);
            }
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
    const ret = wasm.maybe_names(ptr0, len0);
    let v4;
    if (ret[0] !== 0) {
        v4 = [];
        let p4 = ret[0];
        p4 = p4 >>> 0;
        for (const end4 = p4 + ret[1] * 8; p4 < end4; p4 += 8) {
            const view4 = getDataViewMemory0();
            const w4_0 = view4.getInt32(p4 + 0, true);
            const w4_1 = view4.getInt32(p4 + 4, true);
            let v5;
            if (w4_0 !== 0) {
                v5 = getStringFromWasm0(w4_0, w4_1);
                wasm.__wbindgen_free(w4_0, w4_1 * 1, 1);
            }
            v4.push(v5);
        }
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
    }
    return v4;
}

/**
 * @param {(Point | null | undefined)[]} v
 * @returns {(Point | null | undefined)[]}
 */
export function maybe_nullable_points(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            let ptr1 = e0 === null ? 1 : 0;
            if (!isLikeNone(e0)) {
                _assertClass(e0, Point);
                ptr1 = e0.__destroy_into_raw();
            }
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0> 1) wasm.__wbg_point_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.maybe_nullable_points(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    const view4 = getDataViewMemory0();
    for (const end4 = p4 + ret[1] * 4; p4 < end4; p4 += 4) {
        v4.push(view4.getInt32(p4 + 0, true) === 0 ? undefined : view4.getInt32(p4 + 0, true) === 1 ? null : Point.__wrap(view4.getInt32(p4 + 0, true)));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v4;
}

/**
 * @param {(Point | null | undefined)[]} v
 * @returns {(Point | undefined)[]}
 */
export function maybe_points(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            let ptr1 = 0;
            if (!isLikeNone(e0)) {
                _assertClass(e0, Point);
                ptr1 = e0.__destroy_into_raw();
            }
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
    const ret = wasm.maybe_points(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    const view4 = getDataViewMemory0();
    for (const end4 = p4 + ret[1] * 4; p4 < end4; p4 += 4) {
        v4.push(view4.getInt32(p4 + 0, true) === 0 ? undefined : Point.__wrap(view4.getInt32(p4 + 0, true)));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v4;
}

/**
 * @param {string[]} v
 * @returns {string[]}
 */
export function names(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            if (typeof e0 !== 'string') throw new Error('array contains a value of the wrong type');
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
    const ret = wasm.names(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    for (const end4 = p4 + ret[1] * 8; p4 < end4; p4 += 8) {
        const view4 = getDataViewMemory0();
        const w4_0 = view4.getInt32(p4 + 0, true);
        const w4_1 = view4.getInt32(p4 + 4, true);
        let deferred5_0;
        let deferred5_1;
        try {
            deferred5_0 = w4_0;
            deferred5_1 = w4_1;
            v4.push(getStringFromWasm0(w4_0, w4_1));
        } finally {
            wasm.__wbindgen_free(deferred5_0, deferred5_1, 1);
        }
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
    return v4;
}

/**
 * @param {((any | null | undefined)[] | null | undefined)[]} v
 * @returns {((any | undefined)[] | undefined)[]}
 */
export function nested_maybe_foos(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            let j1 = 0, ptr1 = 0, len1 = 0;
            try {
                if (!isLikeNone(e0)) {
                    len1 = e0.length;
                    ptr1 = wasm.__wbindgen_malloc(len1 * 4, 4) >>> 0;
                    for (; j1 < len1; j1++) {
                        const e1 = e0[j1];
                        const w1_0 = isLikeNone(e1) ? 0 : addToExternrefTable0(e1);
                        const view1 = getDataViewMemory0();
                        const p1 = ptr1 + j1 * 4;
                        view1.setInt32(p1 + 0, w1_0, true);
                    }
                }
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
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 8;
            view0.setInt32(p0 + 0, ptr1, true);
            view0.setInt32(p0 + 4, len1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k3 = 0; k3 < j0; k3++) {
                const cell3 = ptr0 + k3 * 8;
                const w3_0 = getDataViewMemory0().getInt32(cell3 + 0, true) >>> 0;
                const w3_1 = getDataViewMemory0().getInt32(cell3 + 4, true) >>> 0;
                if (w3_0) {
                    for (let k4 = 0; k4 < w3_1; k4++) {
                        const cell4 = w3_0 + k4 * 4;
                        const w4_0 = getDataViewMemory0().getInt32(cell4 + 0, true) >>> 0;
                        wasm.__externref_table_dealloc(w4_0);
                    }
                    wasm.__wbindgen_free(w3_0, w3_1 * 4, 4);
                }
            }
            wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        }
        throw e;
    }
    const ret = wasm.nested_maybe_foos(ptr0, len0);
    const v6 = [];
    let p6 = ret[0];
    p6 = p6 >>> 0;
    for (const end6 = p6 + ret[1] * 8; p6 < end6; p6 += 8) {
        const view6 = getDataViewMemory0();
        const w6_0 = view6.getInt32(p6 + 0, true);
        const w6_1 = view6.getInt32(p6 + 4, true);
        let v7;
        if (w6_0 !== 0) {
            v7 = [];
            let p7 = w6_0;
            p7 = p7 >>> 0;
            for (const end7 = p7 + w6_1 * 4; p7 < end7; p7 += 4) {
                const view7 = getDataViewMemory0();
                const w7_0 = view7.getInt32(p7 + 0, true);
                v7.push(takeFromExternrefTable0(w7_0));
            }
            wasm.__wbindgen_free(w6_0, w6_1 * 4, 4);
        }
        v6.push(v7);
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
    return v6;
}

/**
 * @param {(string[])[]} v
 * @returns {(string[])[]}
 */
export function nested_names(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            let j1 = 0, len1, ptr1;
            try {
                len1 = e0.length;
                ptr1 = wasm.__wbindgen_malloc(len1 * 8, 4) >>> 0;
                for (; j1 < len1; j1++) {
                    const e1 = e0[j1];
                    if (typeof e1 !== 'string') throw new Error('array contains a value of the wrong type');
                    const ptr2 = passStringToWasm0(e1, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                    const len2 = WASM_VECTOR_LEN;
                    const view1 = getDataViewMemory0();
                    const p1 = ptr1 + j1 * 8;
                    view1.setInt32(p1 + 0, ptr2, true);
                    view1.setInt32(p1 + 4, len2, true);
                }
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
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 8;
            view0.setInt32(p0 + 0, ptr1, true);
            view0.setInt32(p0 + 4, len1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k4 = 0; k4 < j0; k4++) {
                const cell4 = ptr0 + k4 * 8;
                const w4_0 = getDataViewMemory0().getInt32(cell4 + 0, true) >>> 0;
                const w4_1 = getDataViewMemory0().getInt32(cell4 + 4, true) >>> 0;
                if (w4_0) {
                    for (let k5 = 0; k5 < w4_1; k5++) {
                        const cell5 = w4_0 + k5 * 8;
                        const w5_0 = getDataViewMemory0().getInt32(cell5 + 0, true) >>> 0;
                        const w5_1 = getDataViewMemory0().getInt32(cell5 + 4, true) >>> 0;
                        if (w5_0) { wasm.__wbindgen_free(w5_0, w5_1 * 1, 1); }
                    }
                    wasm.__wbindgen_free(w4_0, w4_1 * 8, 4);
                }
            }
            wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        }
        throw e;
    }
    const ret = wasm.nested_names(ptr0, len0);
    const v7 = [];
    let p7 = ret[0];
    p7 = p7 >>> 0;
    for (const end7 = p7 + ret[1] * 8; p7 < end7; p7 += 8) {
        const view7 = getDataViewMemory0();
        const w7_0 = view7.getInt32(p7 + 0, true);
        const w7_1 = view7.getInt32(p7 + 4, true);
        const v8 = [];
        let p8 = w7_0;
        p8 = p8 >>> 0;
        for (const end8 = p8 + w7_1 * 8; p8 < end8; p8 += 8) {
            const view8 = getDataViewMemory0();
            const w8_0 = view8.getInt32(p8 + 0, true);
            const w8_1 = view8.getInt32(p8 + 4, true);
            let deferred9_0;
            let deferred9_1;
            try {
                deferred9_0 = w8_0;
                deferred9_1 = w8_1;
                v8.push(getStringFromWasm0(w8_0, w8_1));
            } finally {
                wasm.__wbindgen_free(deferred9_0, deferred9_1, 1);
            }
        }
        wasm.__wbindgen_free(w7_0, w7_1 * 8, 4);
        v7.push(v8);
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
    return v7;
}

/**
 * @param {(any | null)[]} v
 * @returns {(any | null)[]}
 */
export function nullable_foos(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            const w0_0 = e0 === null ? 1025 : addToExternrefTable0(e0);
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
    const ret = wasm.nullable_foos(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
        const view3 = getDataViewMemory0();
        const w3_0 = view3.getInt32(p3 + 0, true);
        v3.push(takeFromExternrefTable0(w3_0));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v3;
}

/**
 * @param {(Point | null)[]} v
 * @returns {(Point | null)[]}
 */
export function nullable_points(v) {
    let j0 = 0, len0, ptr0;
    try {
        len0 = v.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = v[j0];
            let ptr1 = e0 === null ? 1 : 0;
            if (e0 !== null) {
                _assertClass(e0, Point);
                ptr1 = e0.__destroy_into_raw();
            }
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0> 1) wasm.__wbg_point_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.nullable_points(ptr0, len0);
    const v4 = [];
    let p4 = ret[0];
    p4 = p4 >>> 0;
    const view4 = getDataViewMemory0();
    for (const end4 = p4 + ret[1] * 4; p4 < end4; p4 += 4) {
        v4.push(view4.getInt32(p4 + 0, true) === 1 ? null : Point.__wrap(view4.getInt32(p4 + 0, true)));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v4;
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_maybe_foos_0a6988fff2da253a(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    for (const end0 = p0 + arg2 * 4; p0 < end0; p0 += 4) {
        const view0 = getDataViewMemory0();
        const w0_0 = view0.getInt32(p0 + 0, true);
        v0.push(takeFromExternrefTable0(w0_0));
    }
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    const ret = js_maybe_foos(v0);
    let j2 = 0, len2, ptr2;
    try {
        len2 = ret.length;
        ptr2 = wasm.__wbindgen_malloc(len2 * 4, 4) >>> 0;
        for (; j2 < len2; j2++) {
            const e2 = ret[j2];
            const w2_0 = isLikeNone(e2) ? 0 : addToExternrefTable0(e2);
            const view2 = getDataViewMemory0();
            const p2 = ptr2 + j2 * 4;
            view2.setInt32(p2 + 0, w2_0, true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len2, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr2, true);
    } catch (e) {
        if (ptr2) {
            for (let k3 = 0; k3 < j2; k3++) {
                const cell3 = ptr2 + k3 * 4;
                const w3_0 = getDataViewMemory0().getInt32(cell3 + 0, true) >>> 0;
                wasm.__externref_table_dealloc(w3_0);
            }
            wasm.__wbindgen_free(ptr2, len2 * 4, 4);
        }
        throw e;
    }
}
export function __wbg_js_maybe_points_a502c96988d34bc0(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    for (const end0 = p0 + arg2 * 4; p0 < end0; p0 += 4) {
        v0.push(view0.getInt32(p0 + 0, true) === 0 ? undefined : Point.__wrap(view0.getInt32(p0 + 0, true)));
    }
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    const ret = js_maybe_points(v0);
    let j2 = 0, len2, ptr2;
    try {
        len2 = ret.length;
        ptr2 = wasm.__wbindgen_malloc(len2 * 4, 4) >>> 0;
        for (; j2 < len2; j2++) {
            const e2 = ret[j2];
            let ptr3 = 0;
            if (!isLikeNone(e2)) {
                _assertClass(e2, Point);
                ptr3 = e2.__destroy_into_raw();
            }
            const view2 = getDataViewMemory0();
            const p2 = ptr2 + j2 * 4;
            view2.setInt32(p2 + 0, ptr3, true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len2, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr2, true);
    } catch (e) {
        if (ptr2) {
            for (let k4 = 0; k4 < j2; k4++) {
                const cell4 = ptr2 + k4 * 4;
                const w4_0 = getDataViewMemory0().getInt32(cell4 + 0, true) >>> 0;
                if (w4_0) wasm.__wbg_point_free(w4_0, 1);
            }
            wasm.__wbindgen_free(ptr2, len2 * 4, 4);
        }
        throw e;
    }
}
export function __wbg_js_names_8202f1528d646c9a(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    for (const end0 = p0 + arg2 * 8; p0 < end0; p0 += 8) {
        const view0 = getDataViewMemory0();
        const w0_0 = view0.getInt32(p0 + 0, true);
        const w0_1 = view0.getInt32(p0 + 4, true);
        let deferred1_0;
        let deferred1_1;
        try {
            deferred1_0 = w0_0;
            deferred1_1 = w0_1;
            v0.push(getStringFromWasm0(w0_0, w0_1));
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    wasm.__wbindgen_free(arg1, arg2 * 8, 4);
    const ret = js_names(v0);
    let j3 = 0, len3, ptr3;
    try {
        len3 = ret.length;
        ptr3 = wasm.__wbindgen_malloc(len3 * 8, 4) >>> 0;
        for (; j3 < len3; j3++) {
            const e3 = ret[j3];
            if (typeof e3 !== 'string') throw new Error('array contains a value of the wrong type');
            const ptr4 = passStringToWasm0(e3, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len4 = WASM_VECTOR_LEN;
            const view3 = getDataViewMemory0();
            const p3 = ptr3 + j3 * 8;
            view3.setInt32(p3 + 0, ptr4, true);
            view3.setInt32(p3 + 4, len4, true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len3, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr3, true);
    } catch (e) {
        if (ptr3) {
            for (let k5 = 0; k5 < j3; k5++) {
                const cell5 = ptr3 + k5 * 8;
                const w5_0 = getDataViewMemory0().getInt32(cell5 + 0, true) >>> 0;
                const w5_1 = getDataViewMemory0().getInt32(cell5 + 4, true) >>> 0;
                if (w5_0) { wasm.__wbindgen_free(w5_0, w5_1 * 1, 1); }
            }
            wasm.__wbindgen_free(ptr3, len3 * 8, 4);
        }
        throw e;
    }
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
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

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
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
