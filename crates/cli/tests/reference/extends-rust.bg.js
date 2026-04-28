export class InheritanceParent {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_InheritanceParent = 0;
        InheritanceParentFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_inheritanceparent_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.inheritanceparent_name(this.__wbg_ptr_InheritanceParent);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} name
     */
    constructor(name) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.inheritanceparent_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_InheritanceParent = ret >>> 0;
        InheritanceParentFinalization.register(this, { __wbg_ptr_InheritanceParent: ret >>> 0 }, this);
        return this;
    }
}
if (Symbol.dispose) InheritanceParent.prototype[Symbol.dispose] = InheritanceParent.prototype.free;

/**
 * @param {InheritanceParent} p
 * @returns {string}
 */
export function inheritance_borrow_parent(p) {
    let deferred1_0;
    let deferred1_1;
    try {
        _assertClass(p, InheritanceParent);
        const ret = wasm.inheritance_borrow_parent(p.__wbg_ptr_InheritanceParent);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

class ns__NsChild extends ns__NsParent {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_ns__NsChild = 0;
        const __anc_ns__NsParent = this.__wbg_ptr_ns__NsParent;
        this.__wbg_ptr_ns__NsParent = 0;
        if (__anc_ns__NsParent !== 0) wasm.__wbg_ns__nsparent_free(__anc_ns__NsParent >>> 0, 1);
        ns__NsChildFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ns__nschild_free(ptr, 0);
    }
    /**
     * @param {string} label
     * @param {string} note
     */
    constructor(label, note) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ptr0 = passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(note, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.nschild_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_ns__NsChild = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_ns__nschild_to_nsparent(ret >>> 0) >>> 0;
        this.__wbg_ptr_ns__NsParent = __wbg_anc_0;
        ns__NsChildFinalization.register(this, { __wbg_ptr_ns__NsChild: ret >>> 0, __wbg_ptr_ns__NsParent: __wbg_anc_0 }, this);
        return this;
    }
    /**
     * @returns {string}
     */
    note() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.nschild_note(this.__wbg_ptr_ns__NsChild);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) ns__NsChild.prototype[Symbol.dispose] = ns__NsChild.prototype.free;

class ns__NsParent {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_ns__NsParent = 0;
        ns__NsParentFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ns__nsparent_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    label() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.nsparent_label(this.__wbg_ptr_ns__NsParent);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} label
     */
    constructor(label) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ptr0 = passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.nsparent_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_ns__NsParent = ret >>> 0;
        ns__NsParentFinalization.register(this, { __wbg_ptr_ns__NsParent: ret >>> 0 }, this);
        return this;
    }
}
if (Symbol.dispose) ns__NsParent.prototype[Symbol.dispose] = ns__NsParent.prototype.free;

export const ns = {};
ns.NsChild = ns__NsChild;
ns.NsParent = ns__NsParent;

export class InheritanceChild extends InheritanceParent {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_InheritanceChild = 0;
        const __anc_InheritanceParent = this.__wbg_ptr_InheritanceParent;
        this.__wbg_ptr_InheritanceParent = 0;
        if (__anc_InheritanceParent !== 0) wasm.__wbg_inheritanceparent_free(__anc_InheritanceParent >>> 0, 1);
        InheritanceChildFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_inheritancechild_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    extra() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.inheritancechild_extra(this.__wbg_ptr_InheritanceChild);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} name
     * @param {string} extra
     */
    constructor(name, extra) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(extra, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.inheritancechild_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_InheritanceChild = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_inheritancechild_to_inheritanceparent(ret >>> 0) >>> 0;
        this.__wbg_ptr_InheritanceParent = __wbg_anc_0;
        InheritanceChildFinalization.register(this, { __wbg_ptr_InheritanceChild: ret >>> 0, __wbg_ptr_InheritanceParent: __wbg_anc_0 }, this);
        return this;
    }
}
if (Symbol.dispose) InheritanceChild.prototype[Symbol.dispose] = InheritanceChild.prototype.free;

export class InheritanceGrandchild extends InheritanceChild {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_InheritanceGrandchild = 0;
        const __anc_InheritanceChild = this.__wbg_ptr_InheritanceChild;
        this.__wbg_ptr_InheritanceChild = 0;
        if (__anc_InheritanceChild !== 0) wasm.__wbg_inheritancechild_free(__anc_InheritanceChild >>> 0, 1);
        const __anc_InheritanceParent = this.__wbg_ptr_InheritanceParent;
        this.__wbg_ptr_InheritanceParent = 0;
        if (__anc_InheritanceParent !== 0) wasm.__wbg_inheritanceparent_free(__anc_InheritanceParent >>> 0, 1);
        InheritanceGrandchildFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_inheritancegrandchild_free(ptr, 0);
    }
    /**
     * @param {string} name
     * @param {string} extra
     * @param {string} tag
     */
    constructor(name, extra, tag) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(extra, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(tag, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.inheritancegrandchild_new(ptr0, len0, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_InheritanceGrandchild = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_inheritancegrandchild_to_inheritancechild(ret >>> 0) >>> 0;
        this.__wbg_ptr_InheritanceChild = __wbg_anc_0;
        const __wbg_anc_1 = wasm.__wbg_upcast_inheritancechild_to_inheritanceparent(__wbg_anc_0) >>> 0;
        this.__wbg_ptr_InheritanceParent = __wbg_anc_1;
        InheritanceGrandchildFinalization.register(this, { __wbg_ptr_InheritanceGrandchild: ret >>> 0, __wbg_ptr_InheritanceChild: __wbg_anc_0, __wbg_ptr_InheritanceParent: __wbg_anc_1 }, this);
        return this;
    }
    /**
     * @returns {string}
     */
    tag() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.inheritancegrandchild_tag(this.__wbg_ptr_InheritanceGrandchild);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) InheritanceGrandchild.prototype[Symbol.dispose] = InheritanceGrandchild.prototype.free;
export function __wbg___wbindgen_throw_9c75d47bf9e7731e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
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
const __wbgSuperSkip = Symbol('wasm-bindgen.super-skip');
const InheritanceChildFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_inheritancechild_free(tok.__wbg_ptr_InheritanceChild >>> 0, 1);
wasm.__wbg_inheritanceparent_free(tok.__wbg_ptr_InheritanceParent >>> 0, 1);
});
const InheritanceGrandchildFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_inheritancegrandchild_free(tok.__wbg_ptr_InheritanceGrandchild >>> 0, 1);
wasm.__wbg_inheritancechild_free(tok.__wbg_ptr_InheritanceChild >>> 0, 1);
wasm.__wbg_inheritanceparent_free(tok.__wbg_ptr_InheritanceParent >>> 0, 1);
});
const InheritanceParentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_inheritanceparent_free(tok.__wbg_ptr_InheritanceParent >>> 0, 1);
});
const ns__NsChildFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_ns__nschild_free(tok.__wbg_ptr_ns__NsChild >>> 0, 1);
wasm.__wbg_ns__nsparent_free(tok.__wbg_ptr_ns__NsParent >>> 0, 1);
});
const ns__NsParentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_ns__nsparent_free(tok.__wbg_ptr_ns__NsParent >>> 0, 1);
});

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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
