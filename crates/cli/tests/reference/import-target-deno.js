/* @ts-self-types="./reference_test.d.ts" */
import { default as _default } from 'tests/wasm/import_class.js';

export function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}
import * as import1 from "tests/wasm/imports.js"
import * as import2 from "foo-raw"
import * as import3 from "./snippets/import_reftest-a82831e16a4c30f1/inline0.js"
import * as import4 from "pure-extern"

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_f1861aae416df39d: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_catch_me_687c8ac1b7535045: function() { return handleError(function () {
            catch_me();
        }, arguments); },
        __wbg_get_b0096ab6db85c3a1: function(arg0) {
            const ret = arg0.get();
            return ret;
        },
        __wbg_my_function_d6c76e66f4e6751a: function() {
            b.my_function();
        },
        __wbg_new_625e056221ff6a6e: function(arg0) {
            const ret = new _default(arg0);
            return ret;
        },
        __wbg_no_catch_be4131677910f8cc: function() {
            no_catch();
        },
        __wbg_reload_cbb85f596c80d974: function() {
            window.location.reload();
        },
        __wbg_static_accessor_CONST_85b96acb48be57e1: function() {
            const ret = a.CONST;
            return ret;
        },
        __wbg_write_1411fb0c6d3ebc7b: function(arg0, arg1) {
            window.document.write(getStringFromWasm0(arg0, arg1));
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
        "./reference_test_bg.js": import0,
        "tests/wasm/imports.js": import1,
        "foo-raw": import2,
        "./snippets/import_reftest-a82831e16a4c30f1/inline0.js": import3,
        "pure-extern": import4,
    };
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
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

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const wasmUrl = new URL('reference_test_bg.wasm', import.meta.url);
const wasmInstantiated = await WebAssembly.instantiateStreaming(fetch(wasmUrl), __wbg_get_imports());
const wasm = wasmInstantiated.instance.exports;
wasm.__wbindgen_start();
