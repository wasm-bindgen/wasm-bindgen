export class NamespaceConsumer {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NamespaceConsumerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_namespaceconsumer_free(ptr, 0);
    }
    /**
     * @returns {bar__Point}
     */
    get bar_point() {
        const ret = wasm.namespaceconsumer_bar_point(this.__wbg_ptr);
        return bar__Point.__wrap(ret);
    }
    /**
     * @returns {bar__Point[]}
     */
    get bar_points() {
        const ret = wasm.namespaceconsumer_bar_points(this.__wbg_ptr);
        const v1 = [];
        let p1 = ret[0];
        p1 = p1 >>> 0;
        const b1 = p1;
        try {
            const view1 = getDataViewMemory0();
            for (const end1 = p1 + ret[1] * 4; p1 < end1; p1 += 4) {
                const w1_0 = view1.getInt32(p1 + 0, true);
                const obj2 = bar__Point.__wrap(w1_0);
                getDataViewMemory0().setInt32(p1 + 0, 0, true);
                v1.push(obj2);
            }
        } finally {
            if (b1) {
                for (let k3 = 0; k3 < ret[1]; k3++) {
                    const cell3 = b1 + k3 * 4;
                    const w3_0 = getDataViewMemory0().getInt32(cell3 + 0, true) >>> 0;
                    if (w3_0) wasm.__wbg_bar__point_free(w3_0, 1);
                }
                wasm.__wbindgen_free(b1, ret[1] * 4, 4);
            }
        }
        return v1;
    }
    /**
     * @returns {bar__Status}
     */
    get bar_status() {
        const ret = wasm.namespaceconsumer_bar_status(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {bar__Point[]} points
     * @returns {bar__Point[]}
     */
    duplicate_bar_points(points) {
        for (let n0 = 0; n0 < points.length; n0++) {
            const e0 = points[n0];
            if (!(e0 instanceof bar__Point)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, bar__Point);
        }
        let j0 = 0, len0, ptr0;
        try {
            len0 = points.length;
            ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
            for (; j0 < len0; j0++) {
                const e0 = points[j0];
                _assertClass(e0, bar__Point);
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
                    if (w2_0) wasm.__wbg_bar__point_free(w2_0, 1);
                }
                wasm.__wbindgen_free(ptr0, len0 * 4, 4);
            }
            throw e;
        }
        const ret = wasm.namespaceconsumer_duplicate_bar_points(this.__wbg_ptr, ptr0, len0);
        const v4 = [];
        let p4 = ret[0];
        p4 = p4 >>> 0;
        const b4 = p4;
        try {
            const view4 = getDataViewMemory0();
            for (const end4 = p4 + ret[1] * 4; p4 < end4; p4 += 4) {
                const w4_0 = view4.getInt32(p4 + 0, true);
                const obj5 = bar__Point.__wrap(w4_0);
                getDataViewMemory0().setInt32(p4 + 0, 0, true);
                v4.push(obj5);
            }
        } finally {
            if (b4) {
                for (let k6 = 0; k6 < ret[1]; k6++) {
                    const cell6 = b4 + k6 * 4;
                    const w6_0 = getDataViewMemory0().getInt32(cell6 + 0, true) >>> 0;
                    if (w6_0) wasm.__wbg_bar__point_free(w6_0, 1);
                }
                wasm.__wbindgen_free(b4, ret[1] * 4, 4);
            }
        }
        return v4;
    }
    /**
     * @param {foo__Point[]} points
     * @returns {foo__Point[]}
     */
    duplicate_foo_points(points) {
        for (let n0 = 0; n0 < points.length; n0++) {
            const e0 = points[n0];
            if (!(e0 instanceof foo__Point)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, foo__Point);
        }
        let j0 = 0, len0, ptr0;
        try {
            len0 = points.length;
            ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
            for (; j0 < len0; j0++) {
                const e0 = points[j0];
                _assertClass(e0, foo__Point);
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
                    if (w2_0) wasm.__wbg_foo__point_free(w2_0, 1);
                }
                wasm.__wbindgen_free(ptr0, len0 * 4, 4);
            }
            throw e;
        }
        const ret = wasm.namespaceconsumer_duplicate_foo_points(this.__wbg_ptr, ptr0, len0);
        const v4 = [];
        let p4 = ret[0];
        p4 = p4 >>> 0;
        const b4 = p4;
        try {
            const view4 = getDataViewMemory0();
            for (const end4 = p4 + ret[1] * 4; p4 < end4; p4 += 4) {
                const w4_0 = view4.getInt32(p4 + 0, true);
                const obj5 = foo__Point.__wrap(w4_0);
                getDataViewMemory0().setInt32(p4 + 0, 0, true);
                v4.push(obj5);
            }
        } finally {
            if (b4) {
                for (let k6 = 0; k6 < ret[1]; k6++) {
                    const cell6 = b4 + k6 * 4;
                    const w6_0 = getDataViewMemory0().getInt32(cell6 + 0, true) >>> 0;
                    if (w6_0) wasm.__wbg_foo__point_free(w6_0, 1);
                }
                wasm.__wbindgen_free(b4, ret[1] * 4, 4);
            }
        }
        return v4;
    }
    /**
     * @returns {foo__Point}
     */
    get foo_point() {
        const ret = wasm.namespaceconsumer_foo_point(this.__wbg_ptr);
        return foo__Point.__wrap(ret);
    }
    /**
     * @returns {foo__Point[]}
     */
    get foo_points() {
        const ret = wasm.namespaceconsumer_foo_points(this.__wbg_ptr);
        const v1 = [];
        let p1 = ret[0];
        p1 = p1 >>> 0;
        const b1 = p1;
        try {
            const view1 = getDataViewMemory0();
            for (const end1 = p1 + ret[1] * 4; p1 < end1; p1 += 4) {
                const w1_0 = view1.getInt32(p1 + 0, true);
                const obj2 = foo__Point.__wrap(w1_0);
                getDataViewMemory0().setInt32(p1 + 0, 0, true);
                v1.push(obj2);
            }
        } finally {
            if (b1) {
                for (let k3 = 0; k3 < ret[1]; k3++) {
                    const cell3 = b1 + k3 * 4;
                    const w3_0 = getDataViewMemory0().getInt32(cell3 + 0, true) >>> 0;
                    if (w3_0) wasm.__wbg_foo__point_free(w3_0, 1);
                }
                wasm.__wbindgen_free(b1, ret[1] * 4, 4);
            }
        }
        return v1;
    }
    /**
     * @returns {foo__Status}
     */
    get foo_status() {
        const ret = wasm.namespaceconsumer_foo_status(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {foo__Point} foo_point
     * @param {bar__Point} bar_point
     * @param {foo__Status} foo_status
     * @param {bar__Status} bar_status
     */
    constructor(foo_point, bar_point, foo_status, bar_status) {
        _assertClass(foo_point, foo__Point);
        _assertClass(bar_point, bar__Point);
        const val3 = +foo_status;
        const val4 = +bar_status;
        let ptr0, ptr1;
        try {
            ptr0 = foo_point.__destroy_into_raw();
            ptr1 = bar_point.__destroy_into_raw();
        } catch (e) {
            if (ptr1) wasm.__wbg_bar__point_free(ptr1, 1);
            if (ptr0) wasm.__wbg_foo__point_free(ptr0, 1);
            throw e;
        }
        const ret = wasm.namespaceconsumer_new(ptr0, ptr1, val3, val4);
        this.__wbg_ptr = ret;
        NamespaceConsumerFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {bar__Status} status
     * @returns {bar__Status}
     */
    next_bar_status(status) {
        const ret = wasm.namespaceconsumer_next_bar_status(this.__wbg_ptr, status);
        return ret;
    }
    /**
     * @param {foo__Status} status
     * @returns {foo__Status}
     */
    next_foo_status(status) {
        const ret = wasm.namespaceconsumer_next_foo_status(this.__wbg_ptr, status);
        return ret;
    }
    /**
     * @param {bar__Point} point
     * @returns {bar__Point}
     */
    normalize_bar(point) {
        _assertClass(point, bar__Point);
        var ptr0 = point.__destroy_into_raw();
        const ret = wasm.namespaceconsumer_normalize_bar(this.__wbg_ptr, ptr0);
        return bar__Point.__wrap(ret);
    }
    /**
     * @param {foo__Point} point
     * @returns {foo__Point}
     */
    rotate_foo(point) {
        _assertClass(point, foo__Point);
        var ptr0 = point.__destroy_into_raw();
        const ret = wasm.namespaceconsumer_rotate_foo(this.__wbg_ptr, ptr0);
        return foo__Point.__wrap(ret);
    }
    /**
     * @param {bar__Point} bar_point
     */
    set bar_point(bar_point) {
        _assertClass(bar_point, bar__Point);
        var ptr0 = bar_point.__destroy_into_raw();
        wasm.namespaceconsumer_set_bar_point(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {bar__Point[]} bar_points
     */
    set bar_points(bar_points) {
        for (let n0 = 0; n0 < bar_points.length; n0++) {
            const e0 = bar_points[n0];
            if (!(e0 instanceof bar__Point)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, bar__Point);
        }
        let j0 = 0, len0, ptr0;
        try {
            len0 = bar_points.length;
            ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
            for (; j0 < len0; j0++) {
                const e0 = bar_points[j0];
                _assertClass(e0, bar__Point);
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
                    if (w2_0) wasm.__wbg_bar__point_free(w2_0, 1);
                }
                wasm.__wbindgen_free(ptr0, len0 * 4, 4);
            }
            throw e;
        }
        wasm.namespaceconsumer_set_bar_points(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {bar__Status} bar_status
     */
    set bar_status(bar_status) {
        wasm.namespaceconsumer_set_bar_status(this.__wbg_ptr, bar_status);
    }
    /**
     * @param {foo__Point} foo_point
     */
    set foo_point(foo_point) {
        _assertClass(foo_point, foo__Point);
        var ptr0 = foo_point.__destroy_into_raw();
        wasm.namespaceconsumer_set_foo_point(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {foo__Point[]} foo_points
     */
    set foo_points(foo_points) {
        for (let n0 = 0; n0 < foo_points.length; n0++) {
            const e0 = foo_points[n0];
            if (!(e0 instanceof foo__Point)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, foo__Point);
        }
        let j0 = 0, len0, ptr0;
        try {
            len0 = foo_points.length;
            ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
            for (; j0 < len0; j0++) {
                const e0 = foo_points[j0];
                _assertClass(e0, foo__Point);
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
                    if (w2_0) wasm.__wbg_foo__point_free(w2_0, 1);
                }
                wasm.__wbindgen_free(ptr0, len0 * 4, 4);
            }
            throw e;
        }
        wasm.namespaceconsumer_set_foo_points(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {foo__Status} foo_status
     */
    set foo_status(foo_status) {
        wasm.namespaceconsumer_set_foo_status(this.__wbg_ptr, foo_status);
    }
}
if (Symbol.dispose) NamespaceConsumer.prototype[Symbol.dispose] = NamespaceConsumer.prototype.free;

/**
 * A top-level export colliding with an inner namespace export should not collide.
 */
export class Point {
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
    get value() {
        const ret = wasm.__wbg_get_point_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} value
     */
    constructor(value) {
        const ret = wasm.point_new(value);
        this.__wbg_ptr = ret;
        PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} arg0
     */
    set value(arg0) {
        wasm.__wbg_set_point_value(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

/**
 * A top-level enum colliding with an inner namespace export should not collide.
 * @enum {0 | 1}
 */
export const Status = Object.freeze({
    Ready: 0, "0": "Ready",
    Done: 1, "1": "Done",
});

class bar__Point {
    static __wrap(ptr) {
        const obj = Object.create(bar__Point.prototype);
        obj.__wbg_ptr = ptr;
        bar__PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        bar__PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bar__point_free(ptr, 0);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    constructor(x, y) {
        const ret = wasm.bar__point_new(x, y);
        this.__wbg_ptr = ret;
        bar__PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_bar__point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_bar__point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_bar__point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_bar__point_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) bar__Point.prototype[Symbol.dispose] = bar__Point.prototype.free;

class bar__RefToFoo {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        bar__RefToFooFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bar__reftofoo_free(ptr, 0);
    }
    /**
     * @returns {foo__Point}
     */
    get foo_point() {
        const ret = wasm.bar__reftofoo_foo_point(this.__wbg_ptr);
        return foo__Point.__wrap(ret);
    }
    /**
     * @returns {foo__Status}
     */
    get foo_status() {
        const ret = wasm.bar__reftofoo_foo_status(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {foo__Point} foo_point
     * @param {foo__Status} foo_status
     */
    constructor(foo_point, foo_status) {
        _assertClass(foo_point, foo__Point);
        const val2 = +foo_status;
        var ptr0 = foo_point.__destroy_into_raw();
        const ret = wasm.bar__reftofoo_new(ptr0, val2);
        this.__wbg_ptr = ret;
        bar__RefToFooFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {foo__Point} point
     * @returns {foo__Point}
     */
    reflect_point(point) {
        _assertClass(point, foo__Point);
        var ptr0 = point.__destroy_into_raw();
        const ret = wasm.bar__reftofoo_reflect_point(this.__wbg_ptr, ptr0);
        return foo__Point.__wrap(ret);
    }
    /**
     * @param {foo__Status} status
     * @returns {foo__Status}
     */
    reflect_status(status) {
        const ret = wasm.bar__reftofoo_reflect_status(this.__wbg_ptr, status);
        return ret;
    }
    /**
     * @param {foo__Point} foo_point
     */
    set foo_point(foo_point) {
        _assertClass(foo_point, foo__Point);
        var ptr0 = foo_point.__destroy_into_raw();
        wasm.bar__reftofoo_set_foo_point(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {foo__Status} foo_status
     */
    set foo_status(foo_status) {
        wasm.bar__reftofoo_set_foo_status(this.__wbg_ptr, foo_status);
    }
}
if (Symbol.dispose) bar__RefToFoo.prototype[Symbol.dispose] = bar__RefToFoo.prototype.free;

/**
 * @enum {0 | 1 | 2}
 */
const bar__Status = Object.freeze({
    Pending: 0, "0": "Pending",
    Complete: 1, "1": "Complete",
    Failed: 2, "2": "Failed",
});

/**
 * @returns {string}
 */
function bar__greet() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.bar__greet();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

class bar__nested__Point {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        bar__nested__PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bar__nested__point_free(ptr, 0);
    }
    /**
     * @param {number} magnitude
     */
    constructor(magnitude) {
        const ret = wasm.bar__nested__point_new(magnitude);
        this.__wbg_ptr = ret;
        bar__nested__PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get magnitude() {
        const ret = wasm.__wbg_get_bar__nested__point_magnitude(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set magnitude(arg0) {
        wasm.__wbg_set_bar__nested__point_magnitude(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) bar__nested__Point.prototype[Symbol.dispose] = bar__nested__Point.prototype.free;

export const bar = {};
bar.Point = bar__Point;
bar.RefToFoo = bar__RefToFoo;
bar.Status = bar__Status;
bar.greet = bar__greet;
bar.nested = {};
bar.nested.Point = bar__nested__Point;

/**
 * Two structs with the same js_name in different namespaces should not collide.
 */
class foo__Point {
    static __wrap(ptr) {
        const obj = Object.create(foo__Point.prototype);
        obj.__wbg_ptr = ptr;
        foo__PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        foo__PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo__point_free(ptr, 0);
    }
    /**
     * @param {number} x
     */
    constructor(x) {
        const ret = wasm.foo__point_new(x);
        this.__wbg_ptr = ret;
        foo__PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_foo__point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_foo__point_x(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) foo__Point.prototype[Symbol.dispose] = foo__Point.prototype.free;

class foo__RefToBar {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        foo__RefToBarFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo__reftobar_free(ptr, 0);
    }
    /**
     * @returns {bar__Point}
     */
    get bar_point() {
        const ret = wasm.foo__reftobar_bar_point(this.__wbg_ptr);
        return bar__Point.__wrap(ret);
    }
    /**
     * @returns {bar__Status}
     */
    get bar_status() {
        const ret = wasm.foo__reftobar_bar_status(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {bar__Point} bar_point
     * @param {bar__Status} bar_status
     */
    constructor(bar_point, bar_status) {
        _assertClass(bar_point, bar__Point);
        const val2 = +bar_status;
        var ptr0 = bar_point.__destroy_into_raw();
        const ret = wasm.foo__reftobar_new(ptr0, val2);
        this.__wbg_ptr = ret;
        foo__RefToBarFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {bar__Point} point
     * @returns {bar__Point}
     */
    reflect_point(point) {
        _assertClass(point, bar__Point);
        var ptr0 = point.__destroy_into_raw();
        const ret = wasm.foo__reftobar_reflect_point(this.__wbg_ptr, ptr0);
        return bar__Point.__wrap(ret);
    }
    /**
     * @param {bar__Status} status
     * @returns {bar__Status}
     */
    reflect_status(status) {
        const ret = wasm.foo__reftobar_reflect_status(this.__wbg_ptr, status);
        return ret;
    }
    /**
     * @param {bar__Point} bar_point
     */
    set bar_point(bar_point) {
        _assertClass(bar_point, bar__Point);
        var ptr0 = bar_point.__destroy_into_raw();
        wasm.foo__reftobar_set_bar_point(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {bar__Status} bar_status
     */
    set bar_status(bar_status) {
        wasm.foo__reftobar_set_bar_status(this.__wbg_ptr, bar_status);
    }
}
if (Symbol.dispose) foo__RefToBar.prototype[Symbol.dispose] = foo__RefToBar.prototype.free;

/**
 * Two enums with the same js_name in different namespaces should not collide.
 * @enum {0 | 1}
 */
const foo__Status = Object.freeze({
    Active: 0, "0": "Active",
    Inactive: 1, "1": "Inactive",
});

/**
 * Two functions with the same js_name in different namespaces should not collide.
 * @returns {string}
 */
function foo__greet() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.foo__greet();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Two structs with the same js_name in nested namespaces should not collide.
 */
class foo__nested__Point {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        foo__nested__PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo__nested__point_free(ptr, 0);
    }
    /**
     * @param {number} z
     */
    constructor(z) {
        const ret = wasm.foo__nested__point_new(z);
        this.__wbg_ptr = ret;
        foo__nested__PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_foo__nested__point_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_foo__nested__point_z(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) foo__nested__Point.prototype[Symbol.dispose] = foo__nested__Point.prototype.free;

/**
 * Same js_name reused across different namespace depths should not collide.
 * @enum {0 | 1}
 */
const foo__nested__Status = Object.freeze({
    Cold: 0, "0": "Cold",
    Warm: 1, "1": "Warm",
});

/**
 * Different exported kinds with the same js_name across namespace depths should not collide.
 * @returns {string}
 */
function foo__nested__deep__Status() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.foo__nested__deep__Status();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
function foo__nested__greet() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.foo__nested__greet();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export const foo = {};
foo.Point = foo__Point;
foo.RefToBar = foo__RefToBar;
foo.Status = foo__Status;
foo.greet = foo__greet;
foo.nested = {};
foo.nested.Point = foo__nested__Point;
foo.nested.Status = foo__nested__Status;
foo.nested.deep = {};
foo.nested.deep.Status = foo__nested__deep__Status;
foo.nested.greet = foo__nested__greet;

/**
 * A top-level function colliding with an inner namespace export should not collide.
 * @returns {string}
 */
export function greet() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.greet();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
const NamespaceConsumerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_namespaceconsumer_free(ptr, 1));
const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr, 1));
const bar__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__point_free(ptr, 1));
const bar__RefToFooFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__reftofoo_free(ptr, 1));
const bar__nested__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__nested__point_free(ptr, 1));
const foo__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__point_free(ptr, 1));
const foo__RefToBarFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__reftobar_free(ptr, 1));
const foo__nested__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__nested__point_free(ptr, 1));

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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
