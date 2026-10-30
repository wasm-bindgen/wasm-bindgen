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
    let len0, w0, ptr0;
    try {
        len0 = v.length;
        w0 = [];
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const a0_0 = isLikeNone(e0) ? 0 : addToExternrefTable0(e0);
            w0.push(a0_0);
        }
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, w0[k0++], true);
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 1) {
            wasm.__externref_table_dealloc(w0[k0]);
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.maybe_foos(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    const w2 = [];
    for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
        w2.push(view2.getInt32(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    for (let k2 = 0; k2 < w2.length; k2 += 1) {
        v2.push(takeFromExternrefTable0(w2[k2]));
    }
    return v2;
}

/**
 * @param {(string | null | undefined)[] | null} [v]
 * @returns {(string | undefined)[] | undefined}
 */
export function maybe_names(v) {
    let ptr0 = 0, len0 = 0, w0;
    try {
        w0 = [];
        if (!isLikeNone(v)) {
            len0 = v.length;
            for (let j0 = 0; j0 < len0; j0++) {
                const e0 = v[j0];
                var ptr1 = isLikeNone(e0) ? 0 : passStringToWasm0(e0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                w0.push(ptr1, len1);
            }
            ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
            const view0 = getDataViewMemory0();
            for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
                const p0 = ptr0 + j0 * 8;
                view0.setInt32(p0 + 0, w0[k0++], true);
                view0.setInt32(p0 + 4, w0[k0++], true);
            }
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 2) {
            const ptr1 = w0[k0];
            const len1 = w0[k0 + 1];
            if (ptr1) { wasm.__wbindgen_free(ptr1, len1 * 1, 1); }
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        throw e;
    }
    const ret = wasm.maybe_names(ptr0, len0);
    let v3;
    if (ret[0] !== 0) {
        v3 = [];
        let p3 = ret[0];
        p3 = p3 >>> 0;
        const view3 = getDataViewMemory0();
        const w3 = [];
        for (const end3 = p3 + ret[1] * 8; p3 < end3; p3 += 8) {
            w3.push(view3.getInt32(p3 + 0, true), view3.getInt32(p3 + 4, true));
        }
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
        for (let k3 = 0; k3 < w3.length; k3 += 2) {
            let v4;
            if (w3[k3] !== 0) {
                v4 = getStringFromWasm0(w3[k3], w3[k3 + 1]);
                wasm.__wbindgen_free(w3[k3], w3[k3 + 1] * 1, 1);
            }
            v3.push(v4);
        }
    }
    return v3;
}

/**
 * @param {(Point | null | undefined)[]} v
 * @returns {(Point | null | undefined)[]}
 */
export function maybe_nullable_points(v) {
    let len0, w0, ptr0;
    try {
        len0 = v.length;
        w0 = [];
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            let ptr1 = e0 === null ? 1 : 0;
            if (!isLikeNone(e0)) {
                _assertClass(e0, Point);
                ptr1 = e0.__destroy_into_raw();
            }
            w0.push(ptr1);
        }
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, w0[k0++], true);
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 1) {
            const ptr1 = w0[k0];
            if (ptr1 > 1) wasm.__wbg_point_free(ptr1, 1);
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.maybe_nullable_points(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    const view3 = getDataViewMemory0();
    const w3 = [];
    for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
        w3.push(view3.getInt32(p3 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    for (let k3 = 0; k3 < w3.length; k3 += 1) {
        v3.push(w3[k3] === 0 ? undefined : w3[k3] === 1 ? null : Point.__wrap(w3[k3]));
    }
    return v3;
}

/**
 * @param {(Point | null | undefined)[]} v
 * @returns {(Point | undefined)[]}
 */
export function maybe_points(v) {
    let len0, w0, ptr0;
    try {
        len0 = v.length;
        w0 = [];
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            let ptr1 = 0;
            if (!isLikeNone(e0)) {
                _assertClass(e0, Point);
                ptr1 = e0.__destroy_into_raw();
            }
            w0.push(ptr1);
        }
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, w0[k0++], true);
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 1) {
            const ptr1 = w0[k0];
            if (ptr1) wasm.__wbg_point_free(ptr1, 1);
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.maybe_points(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    const view3 = getDataViewMemory0();
    const w3 = [];
    for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
        w3.push(view3.getInt32(p3 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    for (let k3 = 0; k3 < w3.length; k3 += 1) {
        v3.push(w3[k3] === 0 ? undefined : Point.__wrap(w3[k3]));
    }
    return v3;
}

/**
 * @param {string[]} v
 * @returns {string[]}
 */
export function names(v) {
    let len0, w0, ptr0;
    try {
        len0 = v.length;
        w0 = [];
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const ptr1 = passStringToWasm0(e0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            w0.push(ptr1, len1);
        }
        ptr0 = wasm.__wbindgen_malloc(len0 * 8, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
            const p0 = ptr0 + j0 * 8;
            view0.setInt32(p0 + 0, w0[k0++], true);
            view0.setInt32(p0 + 4, w0[k0++], true);
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 2) {
            const ptr1 = w0[k0];
            const len1 = w0[k0 + 1];
            if (ptr1) { wasm.__wbindgen_free(ptr1, len1 * 1, 1); }
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 8, 4);
        throw e;
    }
    const ret = wasm.names(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    const view3 = getDataViewMemory0();
    const w3 = [];
    for (const end3 = p3 + ret[1] * 8; p3 < end3; p3 += 8) {
        w3.push(view3.getInt32(p3 + 0, true), view3.getInt32(p3 + 4, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 4);
    for (let k3 = 0; k3 < w3.length; k3 += 2) {
        let deferred4_0;
        let deferred4_1;
        deferred4_0 = w3[k3];
        deferred4_1 = w3[k3 + 1];
        v3.push(getStringFromWasm0(w3[k3], w3[k3 + 1]));
        wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
    }
    return v3;
}

/**
 * @param {(any | null)[]} v
 * @returns {(any | null)[]}
 */
export function nullable_foos(v) {
    let len0, w0, ptr0;
    try {
        len0 = v.length;
        w0 = [];
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            const a0_0 = e0 === null ? 1025 : addToExternrefTable0(e0);
            w0.push(a0_0);
        }
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, w0[k0++], true);
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 1) {
            wasm.__externref_table_dealloc(w0[k0]);
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.nullable_foos(ptr0, len0);
    const v2 = [];
    let p2 = ret[0];
    p2 = p2 >>> 0;
    const view2 = getDataViewMemory0();
    const w2 = [];
    for (const end2 = p2 + ret[1] * 4; p2 < end2; p2 += 4) {
        w2.push(view2.getInt32(p2 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    for (let k2 = 0; k2 < w2.length; k2 += 1) {
        v2.push(takeFromExternrefTable0(w2[k2]));
    }
    return v2;
}

/**
 * @param {(Point | null)[]} v
 * @returns {(Point | null)[]}
 */
export function nullable_points(v) {
    let len0, w0, ptr0;
    try {
        len0 = v.length;
        w0 = [];
        for (let j0 = 0; j0 < len0; j0++) {
            const e0 = v[j0];
            let ptr1 = e0 === null ? 1 : 0;
            if (e0 !== null) {
                _assertClass(e0, Point);
                ptr1 = e0.__destroy_into_raw();
            }
            w0.push(ptr1);
        }
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        const view0 = getDataViewMemory0();
        for (let j0 = 0, k0 = 0; j0 < len0; j0++) {
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, w0[k0++], true);
        }
    } catch (e) {
        if (w0) for (let k0 = 0; k0 < w0.length; k0 += 1) {
            const ptr1 = w0[k0];
            if (ptr1 > 1) wasm.__wbg_point_free(ptr1, 1);
        }
        if (ptr0) wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        throw e;
    }
    const ret = wasm.nullable_points(ptr0, len0);
    const v3 = [];
    let p3 = ret[0];
    p3 = p3 >>> 0;
    const view3 = getDataViewMemory0();
    const w3 = [];
    for (const end3 = p3 + ret[1] * 4; p3 < end3; p3 += 4) {
        w3.push(view3.getInt32(p3 + 0, true));
    }
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    for (let k3 = 0; k3 < w3.length; k3 += 1) {
        v3.push(w3[k3] === 1 ? null : Point.__wrap(w3[k3]));
    }
    return v3;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_maybe_foos_0a6988fff2da253a(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    const w0 = [];
    for (const end0 = p0 + arg2 * 4; p0 < end0; p0 += 4) {
        w0.push(view0.getInt32(p0 + 0, true));
    }
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    for (let k0 = 0; k0 < w0.length; k0 += 1) {
        v0.push(takeFromExternrefTable0(w0[k0]));
    }
    const ret = js_maybe_foos(v0);
    let len2, w2, ptr2;
    try {
        len2 = ret.length;
        w2 = [];
        for (let j2 = 0; j2 < len2; j2++) {
            const e2 = ret[j2];
            const a2_0 = isLikeNone(e2) ? 0 : addToExternrefTable0(e2);
            w2.push(a2_0);
        }
        ptr2 = wasm.__wbindgen_malloc(len2 * 4, 4) >>> 0;
        const view2 = getDataViewMemory0();
        for (let j2 = 0, k2 = 0; j2 < len2; j2++) {
            const p2 = ptr2 + j2 * 4;
            view2.setInt32(p2 + 0, w2[k2++], true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len2, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr2, true);
    } catch (e) {
        if (w2) for (let k2 = 0; k2 < w2.length; k2 += 1) {
            wasm.__externref_table_dealloc(w2[k2]);
        }
        if (ptr2) wasm.__wbindgen_free(ptr2, len2 * 4, 4);
        throw e;
    }
}
export function __wbg_js_maybe_points_a502c96988d34bc0(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    const w0 = [];
    for (const end0 = p0 + arg2 * 4; p0 < end0; p0 += 4) {
        w0.push(view0.getInt32(p0 + 0, true));
    }
    wasm.__wbindgen_free(arg1, arg2 * 4, 4);
    for (let k0 = 0; k0 < w0.length; k0 += 1) {
        v0.push(w0[k0] === 0 ? undefined : Point.__wrap(w0[k0]));
    }
    const ret = js_maybe_points(v0);
    let len2, w2, ptr2;
    try {
        len2 = ret.length;
        w2 = [];
        for (let j2 = 0; j2 < len2; j2++) {
            const e2 = ret[j2];
            let ptr3 = 0;
            if (!isLikeNone(e2)) {
                _assertClass(e2, Point);
                ptr3 = e2.__destroy_into_raw();
            }
            w2.push(ptr3);
        }
        ptr2 = wasm.__wbindgen_malloc(len2 * 4, 4) >>> 0;
        const view2 = getDataViewMemory0();
        for (let j2 = 0, k2 = 0; j2 < len2; j2++) {
            const p2 = ptr2 + j2 * 4;
            view2.setInt32(p2 + 0, w2[k2++], true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len2, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr2, true);
    } catch (e) {
        if (w2) for (let k2 = 0; k2 < w2.length; k2 += 1) {
            const ptr3 = w2[k2];
            if (ptr3) wasm.__wbg_point_free(ptr3, 1);
        }
        if (ptr2) wasm.__wbindgen_free(ptr2, len2 * 4, 4);
        throw e;
    }
}
export function __wbg_js_names_8202f1528d646c9a(arg0, arg1, arg2) {
    const v0 = [];
    let p0 = arg1;
    p0 = p0 >>> 0;
    const view0 = getDataViewMemory0();
    const w0 = [];
    for (const end0 = p0 + arg2 * 8; p0 < end0; p0 += 8) {
        w0.push(view0.getInt32(p0 + 0, true), view0.getInt32(p0 + 4, true));
    }
    wasm.__wbindgen_free(arg1, arg2 * 8, 4);
    for (let k0 = 0; k0 < w0.length; k0 += 2) {
        let deferred1_0;
        let deferred1_1;
        deferred1_0 = w0[k0];
        deferred1_1 = w0[k0 + 1];
        v0.push(getStringFromWasm0(w0[k0], w0[k0 + 1]));
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
    const ret = js_names(v0);
    let len3, w3, ptr3;
    try {
        len3 = ret.length;
        w3 = [];
        for (let j3 = 0; j3 < len3; j3++) {
            const e3 = ret[j3];
            const ptr4 = passStringToWasm0(e3, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len4 = WASM_VECTOR_LEN;
            w3.push(ptr4, len4);
        }
        ptr3 = wasm.__wbindgen_malloc(len3 * 8, 4) >>> 0;
        const view3 = getDataViewMemory0();
        for (let j3 = 0, k3 = 0; j3 < len3; j3++) {
            const p3 = ptr3 + j3 * 8;
            view3.setInt32(p3 + 0, w3[k3++], true);
            view3.setInt32(p3 + 4, w3[k3++], true);
        }
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len3, true);
        getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr3, true);
    } catch (e) {
        if (w3) for (let k3 = 0; k3 < w3.length; k3 += 2) {
            const ptr4 = w3[k3];
            const len4 = w3[k3 + 1];
            if (ptr4) { wasm.__wbindgen_free(ptr4, len4 * 1, 1); }
        }
        if (ptr3) wasm.__wbindgen_free(ptr3, len3 * 8, 4);
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
