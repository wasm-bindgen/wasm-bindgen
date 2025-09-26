import source wasmModule from "./reference_test_bg.wasm";

let wasm;
import { default as default1 } from 'tests/wasm/import_class.js';
import * as import0 from 'tests/wasm/imports.js'
import * as import1 from 'foo-raw'
import * as import2 from './snippets/import_reftest-a82831e16a4c30f1/inline0.js'
import * as import3 from 'pure-extern'

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
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

export function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

let __wbg_instance_id = 0;

export function __wbg_reset_state () {
    __wbg_instance_id++;
    cachedUint8ArrayMemory0 = null;
    if (typeof numBytesDecoded !== 'undefined') numBytesDecoded = 0;
    const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
    wasm = wasmInstance.exports;
    wasm.__wbindgen_start();
}

const imports = {
    __wbindgen_placeholder__: {
        __wbg___wbindgen_throw_451ec1a8469d7eb6: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_catch_me_1d18acaa34acb005: function() { return handleError(function () {
            catch_me();
        }, arguments) },
        __wbg_get_c871386e44ba8c35: function(arg0) {
            const ret = arg0.get();
            return ret;
        },
        __wbg_my_function_597f96bc4719408a: function() {
            b.my_function();
        },
        __wbg_new_c30895ccee9479d4: function(arg0) {
            const ret = new default1(arg0);
            return ret;
        },
        __wbg_no_catch_757175fbf9e08b9e: function() {
            no_catch();
        },
        __wbg_reload_b091d4dc4b1b3a74: function() {
            window.location.reload();
        },
        __wbg_static_accessor_CONST_85b96acb48be57e1: function() {
            const ret = a.CONST;
            return ret;
        },
        __wbg_write_691fc0d693f0c7b5: function(arg0, arg1) {
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
            ;
        },
    },
    'tests/wasm/imports.js': import0,  'foo-raw': import1,  './snippets/import_reftest-a82831e16a4c30f1/inline0.js': import2,  'pure-extern': import3,
};

const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;

wasm.__wbindgen_start();

