/* @ts-self-types="./wasm_bindgen_benchmark.d.ts" */
import { Foo, jsthunk, use_baz } from '../globals.js';

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a, b) {
    const ret = wasm.add(a, b);
    return ret;
}

/**
 * @param {number} n
 */
export function call_doesnt_throw_n_times(n) {
    wasm.call_doesnt_throw_n_times(n);
}

/**
 * @param {number} n
 */
export function call_doesnt_throw_with_catch_n_times(n) {
    wasm.call_doesnt_throw_with_catch_n_times(n);
}

/**
 * @param {number} n
 * @param {any} element
 */
export function call_first_child_final_n_times(n, element) {
    wasm.call_first_child_final_n_times(n, element);
}

/**
 * @param {number} n
 * @param {any} element
 */
export function call_first_child_structural_n_times(n, element) {
    wasm.call_first_child_structural_n_times(n, element);
}

/**
 * @param {number} n
 * @param {any} js_foo
 */
export function call_foo_bar_final_n_times(n, js_foo) {
    wasm.call_foo_bar_final_n_times(n, js_foo);
}

/**
 * @param {number} n
 * @param {any} js_foo
 */
export function call_foo_bar_structural_n_times(n, js_foo) {
    wasm.call_foo_bar_structural_n_times(n, js_foo);
}

/**
 * @param {number} n
 * @param {number} a
 * @param {number} b
 */
export function call_js_add_n_times(n, a, b) {
    wasm.call_js_add_n_times(n, a, b);
}

/**
 * @param {number} n
 */
export function call_js_thunk_n_times(n) {
    wasm.call_js_thunk_n_times(n);
}

/**
 * @param {number} n
 * @param {any[]} elements
 */
export function call_node_first_child_n_times(n, elements) {
    const ptr0 = passArrayJsValueToWasm0(elements, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.call_node_first_child_n_times(n, ptr0, len0);
}

/**
 * @param {number} n
 * @param {any[]} elements
 */
export function call_node_has_child_nodes_n_times(n, elements) {
    const ptr0 = passArrayJsValueToWasm0(elements, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.call_node_has_child_nodes_n_times(n, ptr0, len0);
}

/**
 * @param {number} n
 * @param {any[]} elements
 */
export function call_node_node_type_n_times(n, elements) {
    const ptr0 = passArrayJsValueToWasm0(elements, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.call_node_node_type_n_times(n, ptr0, len0);
}

/**
 * @param {number} n
 */
export function call_use_baz_n_times(n) {
    wasm.call_use_baz_n_times(n);
}

/**
 * @param {Node} element
 */
export function count_node_types(element) {
    wasm.count_node_types(element);
}

/**
 * @param {number} n
 * @returns {number}
 */
export function fibonacci(n) {
    const ret = wasm.fibonacci(n);
    return ret;
}

/**
 * @returns {number}
 */
export function fibonacci_high() {
    const ret = wasm.fibonacci_high();
    return ret;
}

/**
 * @param {string} s
 * @returns {string}
 */
export function str_roundtrip(s) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.str_roundtrip(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

export function thunk() {
    wasm.thunk();
}
import * as import1 from "../globals.js"
import * as import2 from "../globals.js"
import * as import3 from "../globals.js"

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_rethrow_05525c567f154472: function(arg0) {
            throw arg0;
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_bar_173e74eaa64f6e03: function(arg0) {
            Foo.prototype.bar.call(arg0);
        },
        __wbg_bar_8ee2ad15781ac124: function(arg0) {
            arg0.bar();
        },
        __wbg_firstChild_2950111f6da7246c: function(arg0) {
            const ret = arg0.firstChild;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_firstChild_b492b343d85d0a6d: function(arg0) {
            const ret = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'firstChild').get.call(arg0);
            return ret;
        },
        __wbg_firstChild_e3ec0d08e9322186: function(arg0) {
            const ret = arg0.firstChild;
            return ret;
        },
        __wbg_hasChildNodes_9780b4125d1978d1: function(arg0) {
            const ret = arg0.hasChildNodes();
            return ret;
        },
        __wbg_jsthunk_39201715b453122e: function() { return handleError(function () {
            jsthunk();
        }, arguments); },
        __wbg_nextSibling_2e988d9bbe3e06f0: function(arg0) {
            const ret = arg0.nextSibling;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_nodeType_1a77807cb3800514: function(arg0) {
            const ret = arg0.nodeType;
            return ret;
        },
        __wbg_use_baz_a16d355598539413: function(arg0) {
            use_baz(__wbindgen_enum_Baz[arg0]);
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
        "./wasm_bindgen_benchmark_bg.js": import0,
        "../globals.js": import1,
        "../globals.js": import2,
        "../globals.js": import3,
    };
}

const __wbindgen_enum_Baz = ["variant-1", "variant-2", "variant-3"];

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
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
        module_or_path = new URL('wasm_bindgen_benchmark_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
