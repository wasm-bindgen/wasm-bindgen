export class ReinitCounter {
    static __wrap(ptr) {
        const obj = Object.create(ReinitCounter.prototype);
        obj.__wbg_ptr = ptr;
        Object.defineProperty(obj, '__wbg_inst', { value: __wbg_instance_id, writable: true });

        ReinitCounterFinalization.register(obj, { ptr, instance: __wbg_instance_id }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ReinitCounterFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        __wbg_call_guard();
        wasm.__wbg_reinitcounter_free(ptr, 0);
    }
    __wbg_reconstruct() {
        const obj = new ReinitCounter(...this.__wbg_args);
        this.__wbg_ptr = obj.__wbg_ptr;
        this.__wbg_inst = __wbg_instance_id;
        ReinitCounterFinalization.unregister(obj);
        ReinitCounterFinalization.unregister(this);
        ReinitCounterFinalization.register(this, { ptr: this.__wbg_ptr, instance: __wbg_instance_id }, this);
    }
    /**
     * @returns {number}
     */
    get count() {
        __wbg_call_guard();
        if (this.__wbg_inst !== undefined && this.__wbg_inst !== __wbg_instance_id) {
            _reconstructStale(this);
        }
        const ret = wasm.reinitcounter_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    increment() {
        __wbg_call_guard();
        if (this.__wbg_inst !== undefined && this.__wbg_inst !== __wbg_instance_id) {
            _reconstructStale(this);
        }
        const ret = wasm.reinitcounter_increment(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} start
     * @param {string} label
     */
    constructor(start, label) {
        if (arguments[0] === __wbgSuperSkip) return;
        __wbg_call_guard();
        const ptr0 = passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reinitcounter_new(start, ptr0, len0);
        this.__wbg_ptr = ret;
        Object.defineProperty(this, '__wbg_inst', { value: __wbg_instance_id, writable: true });
        Object.defineProperty(this, '__wbg_args', { value: [...arguments] });
        ReinitCounterFinalization.register(this, { ptr: ret, instance: __wbg_instance_id }, this);
        return this;
    }
    /**
     * @param {number} count
     */
    set count(count) {
        __wbg_call_guard();
        if (this.__wbg_inst !== undefined && this.__wbg_inst !== __wbg_instance_id) {
            _reconstructStale(this);
        }
        wasm.reinitcounter_set_count(this.__wbg_ptr, count);
    }
    /**
     * @param {number} start
     * @returns {ReinitCounter}
     */
    static wrapped(start) {
        __wbg_call_guard();
        const ret = wasm.reinitcounter_wrapped(start);
        return ReinitCounter.__wrap(ret);
    }
}
if (Symbol.dispose) ReinitCounter.prototype[Symbol.dispose] = ReinitCounter.prototype.free;

export class ReinitParent {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_ReinitParent = 0;
        ReinitParentFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_ReinitParent) { throw new TypeError('ReinitParent: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        __wbg_call_guard();
        wasm.__wbg_reinitparent_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    name() {
        let deferred1_0;
        let deferred1_1;
        try {
            __wbg_call_guard();
            if (this.__wbg_inst !== undefined && this.__wbg_inst !== __wbg_instance_id) {
                _reconstructStale(this);
            }
            const ret = wasm.reinitparent_name(this.__wbg_ptr_ReinitParent);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            __wbg_call_guard();
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} name
     */
    constructor(name) {
        if (arguments[0] === __wbgSuperSkip) return;
        __wbg_call_guard();
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reinitparent_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_ReinitParent = ret >>> 0;
        Object.defineProperty(this, '__wbg_inst', { value: __wbg_instance_id, writable: true });
        ReinitParentFinalization.register(this, { __wbg_ptr_ReinitParent: ret >>> 0, instance: __wbg_instance_id }, this);
        return this;
    }
}
if (Symbol.dispose) ReinitParent.prototype[Symbol.dispose] = ReinitParent.prototype.free;

export class ReinitPlain {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ReinitPlainFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        __wbg_call_guard();
        wasm.__wbg_reinitplain_free(ptr, 0);
    }
    /**
     * @param {number} value
     */
    constructor(value) {
        if (arguments[0] === __wbgSuperSkip) return;
        __wbg_call_guard();
        const ret = wasm.reinitplain_new(value);
        this.__wbg_ptr = ret;
        Object.defineProperty(this, '__wbg_inst', { value: __wbg_instance_id, writable: true });
        ReinitPlainFinalization.register(this, { ptr: ret, instance: __wbg_instance_id }, this);
        return this;
    }
    /**
     * @returns {number}
     */
    value() {
        __wbg_call_guard();
        if (this.__wbg_inst !== undefined && this.__wbg_inst !== __wbg_instance_id) {
            _reconstructStale(this);
        }
        const ret = wasm.reinitplain_value(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) ReinitPlain.prototype[Symbol.dispose] = ReinitPlain.prototype.free;

function __wbg_reset_state () {
    __wbg_instance_id++;
    cachedUint8ArrayMemory0 = null;
    if (typeof numBytesDecoded !== 'undefined') numBytesDecoded = 0;
    if (typeof WASM_VECTOR_LEN !== 'undefined') WASM_VECTOR_LEN = 0;
    __wbg_reinit_scheduled = false;
    wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
}

/**
 * @param {ReinitCounter} counter
 * @returns {number}
 */
export function reinit_read_counter(counter) {
    __wbg_call_guard();
    _assertClass(counter, ReinitCounter);
    if (counter.__wbg_inst !== undefined && counter.__wbg_inst !== __wbg_instance_id) {
        _reconstructStale(counter);
    }
    const ret = wasm.reinit_read_counter(counter.__wbg_ptr);
    return ret >>> 0;
}

export function reinit_schedule() {
    __wbg_call_guard();
    wasm.reinit_schedule();
}

export class ReinitChild extends ReinitParent {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_ReinitChild = 0;
        const __anc_ReinitParent = this.__wbg_ptr_ReinitParent;
        this.__wbg_ptr_ReinitParent = 0;
        if (__anc_ReinitParent !== 0) wasm.__wbg_reinitparent_free(__anc_ReinitParent >>> 0, 1);
        ReinitChildFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_ReinitChild) { throw new TypeError('ReinitChild: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        __wbg_call_guard();
        wasm.__wbg_reinitchild_free(ptr, 0);
    }
    __wbg_reconstruct() {
        const obj = new ReinitChild(...this.__wbg_args);
        this.__wbg_ptr = obj.__wbg_ptr;
        this.__wbg_ptr_ReinitChild = obj.__wbg_ptr_ReinitChild;
        this.__wbg_ptr_ReinitParent = obj.__wbg_ptr_ReinitParent;
        this.__wbg_inst = __wbg_instance_id;
        ReinitChildFinalization.unregister(obj);
        ReinitChildFinalization.unregister(this);
        ReinitChildFinalization.register(this, { __wbg_ptr_ReinitChild: this.__wbg_ptr_ReinitChild, __wbg_ptr_ReinitParent: this.__wbg_ptr_ReinitParent, instance: __wbg_instance_id }, this);
    }
    /**
     * @returns {number}
     */
    extra() {
        __wbg_call_guard();
        if (this.__wbg_inst !== undefined && this.__wbg_inst !== __wbg_instance_id) {
            _reconstructStale(this);
        }
        const ret = wasm.reinitchild_extra(this.__wbg_ptr_ReinitChild);
        return ret >>> 0;
    }
    /**
     * @param {string} name
     * @param {number} extra
     */
    constructor(name, extra) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        __wbg_call_guard();
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reinitchild_new(ptr0, len0, extra);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_ReinitChild = ret >>> 0;
        Object.defineProperty(this, '__wbg_inst', { value: __wbg_instance_id, writable: true });
        Object.defineProperty(this, '__wbg_args', { value: [...arguments] });
        const __wbg_anc_0 = wasm.__wbg_upcast_reinitchild_to_reinitparent(ret >>> 0) >>> 0;
        this.__wbg_ptr_ReinitParent = __wbg_anc_0;
        ReinitChildFinalization.register(this, { __wbg_ptr_ReinitChild: ret >>> 0, __wbg_ptr_ReinitParent: __wbg_anc_0, instance: __wbg_instance_id }, this);
        return this;
    }
}
if (Symbol.dispose) ReinitChild.prototype[Symbol.dispose] = ReinitChild.prototype.free;
export function __wbg___wbindgen_reinit_3f79fdb34dee3ea9() {
    __wbg_reinit_scheduled = true;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
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


function __wbg_call_guard() {
    if (__wbg_reinit_scheduled) {
        __wbg_reset_state();
        return;
    }
}


let __wbg_instance_id = 0;
const ReinitChildFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { if (tok.instance === __wbg_instance_id) { wasm.__wbg_reinitchild_free(tok.__wbg_ptr_ReinitChild >>> 0, 1);
wasm.__wbg_reinitparent_free(tok.__wbg_ptr_ReinitParent >>> 0, 1);
} });
const ReinitCounterFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(({ ptr, instance }) => {
    if (instance === __wbg_instance_id) wasm.__wbg_reinitcounter_free(ptr, 1);
});
const ReinitParentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { if (tok.instance === __wbg_instance_id) { wasm.__wbg_reinitparent_free(tok.__wbg_ptr_ReinitParent >>> 0, 1);
} });
const ReinitPlainFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(({ ptr, instance }) => {
    if (instance === __wbg_instance_id) wasm.__wbg_reinitplain_free(ptr, 1);
});

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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

function _reconstructStale(obj) {
    if (obj.__wbg_args === undefined) {
        throw new Error('Invalid stale object from previous Wasm instance');
    }
    obj.__wbg_reconstruct();
}

let __wbg_reinit_scheduled = false;

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
