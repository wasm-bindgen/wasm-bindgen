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
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
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
        const ptr0 = passArrayJsValueToWasm0(points, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.namespaceconsumer_duplicate_bar_points(this.__wbg_ptr, ptr0, len0);
        var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v2;
    }
    /**
     * @param {foo__Point[]} points
     * @returns {foo__Point[]}
     */
    duplicate_foo_points(points) {
        const ptr0 = passArrayJsValueToWasm0(points, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.namespaceconsumer_duplicate_foo_points(this.__wbg_ptr, ptr0, len0);
        var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v2;
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
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
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
        var ptr0 = foo_point.__destroy_into_raw();
        _assertClass(bar_point, bar__Point);
        var ptr1 = bar_point.__destroy_into_raw();
        const ret = wasm.namespaceconsumer_new(ptr0, ptr1, foo_status, bar_status);
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
        const ptr0 = passArrayJsValueToWasm0(bar_points, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
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
        const ptr0 = passArrayJsValueToWasm0(foo_points, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
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
     * @param {number} arg0
     */
    set value(arg0) {
        wasm.__wbg_set_point_value(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} value
     */
    constructor(value) {
        const ret = wasm.toplevelpoint_new(value);
        this.__wbg_ptr = ret;
        PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export class RefToBar {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RefToBarFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_reftobar_free(ptr, 0);
    }
    /**
     * @returns {bar__Point}
     */
    get bar_point() {
        const ret = wasm.reftobar_bar_point(this.__wbg_ptr);
        return bar__Point.__wrap(ret);
    }
    /**
     * @returns {bar__Status}
     */
    get bar_status() {
        const ret = wasm.reftobar_bar_status(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {bar__Point} bar_point
     * @param {bar__Status} bar_status
     */
    constructor(bar_point, bar_status) {
        _assertClass(bar_point, bar__Point);
        var ptr0 = bar_point.__destroy_into_raw();
        const ret = wasm.reftobar_new(ptr0, bar_status);
        return foo__RefToBar.__wrap(ret);
    }
    /**
     * @param {bar__Point} point
     * @returns {bar__Point}
     */
    reflect_point(point) {
        _assertClass(point, bar__Point);
        var ptr0 = point.__destroy_into_raw();
        const ret = wasm.reftobar_reflect_point(this.__wbg_ptr, ptr0);
        return bar__Point.__wrap(ret);
    }
    /**
     * @param {bar__Status} status
     * @returns {bar__Status}
     */
    reflect_status(status) {
        const ret = wasm.reftobar_reflect_status(this.__wbg_ptr, status);
        return ret;
    }
    /**
     * @param {bar__Point} bar_point
     */
    set bar_point(bar_point) {
        _assertClass(bar_point, bar__Point);
        var ptr0 = bar_point.__destroy_into_raw();
        wasm.reftobar_set_bar_point(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {bar__Status} bar_status
     */
    set bar_status(bar_status) {
        wasm.reftobar_set_bar_status(this.__wbg_ptr, bar_status);
    }
}
if (Symbol.dispose) RefToBar.prototype[Symbol.dispose] = RefToBar.prototype.free;

export class RefToFoo {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RefToFooFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_reftofoo_free(ptr, 0);
    }
    /**
     * @returns {foo__Point}
     */
    get foo_point() {
        const ret = wasm.reftofoo_foo_point(this.__wbg_ptr);
        return foo__Point.__wrap(ret);
    }
    /**
     * @returns {foo__Status}
     */
    get foo_status() {
        const ret = wasm.reftofoo_foo_status(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {foo__Point} foo_point
     * @param {foo__Status} foo_status
     */
    constructor(foo_point, foo_status) {
        _assertClass(foo_point, foo__Point);
        var ptr0 = foo_point.__destroy_into_raw();
        const ret = wasm.reftofoo_new(ptr0, foo_status);
        return bar__RefToFoo.__wrap(ret);
    }
    /**
     * @param {foo__Point} point
     * @returns {foo__Point}
     */
    reflect_point(point) {
        _assertClass(point, foo__Point);
        var ptr0 = point.__destroy_into_raw();
        const ret = wasm.reftofoo_reflect_point(this.__wbg_ptr, ptr0);
        return foo__Point.__wrap(ret);
    }
    /**
     * @param {foo__Status} status
     * @returns {foo__Status}
     */
    reflect_status(status) {
        const ret = wasm.reftofoo_reflect_status(this.__wbg_ptr, status);
        return ret;
    }
    /**
     * @param {foo__Point} foo_point
     */
    set foo_point(foo_point) {
        _assertClass(foo_point, foo__Point);
        var ptr0 = foo_point.__destroy_into_raw();
        wasm.reftofoo_set_foo_point(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {foo__Status} foo_status
     */
    set foo_status(foo_status) {
        wasm.reftofoo_set_foo_status(this.__wbg_ptr, foo_status);
    }
}
if (Symbol.dispose) RefToFoo.prototype[Symbol.dispose] = RefToFoo.prototype.free;

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
    static __unwrap(jsValue) {
        if (!(jsValue instanceof bar__Point)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
        const ret = wasm.barpoint_new(x, y);
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
    static __wrap(ptr) {
        const obj = Object.create(bar__RefToFoo.prototype);
        obj.__wbg_ptr = ptr;
        bar__RefToFooFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
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
        const ret = wasm.barnestedpoint_new(magnitude);
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
    static __unwrap(jsValue) {
        if (!(jsValue instanceof foo__Point)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
        const ret = wasm.foopoint_new(x);
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
    static __wrap(ptr) {
        const obj = Object.create(foo__RefToBar.prototype);
        obj.__wbg_ptr = ptr;
        foo__RefToBarFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
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
        const ret = wasm.foonestedpoint_new(z);
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
export function __wbg___wbindgen_debug_string_07cb72cfcc952e2b(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_9c75d47bf9e7731e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_bar__point_new(arg0) {
    const ret = bar__Point.__wrap(arg0);
    return ret;
}
export function __wbg_bar__point_unwrap(arg0) {
    const ret = bar__Point.__unwrap(arg0);
    return ret;
}
export function __wbg_foo__point_new(arg0) {
    const ret = foo__Point.__wrap(arg0);
    return ret;
}
export function __wbg_foo__point_unwrap(arg0) {
    const ret = foo__Point.__unwrap(arg0);
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
const bar__RefToFooFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__reftofoo_free(ptr, 1));
const bar__nested__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__nested__point_free(ptr, 1));
const bar__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bar__point_free(ptr, 1));
const foo__RefToBarFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__reftobar_free(ptr, 1));
const foo__nested__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__nested__point_free(ptr, 1));
const foo__PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo__point_free(ptr, 1));
const NamespaceConsumerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_namespaceconsumer_free(ptr, 1));
const RefToBarFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_reftobar_free(ptr, 1));
const RefToFooFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_reftofoo_free(ptr, 1));
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

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
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
