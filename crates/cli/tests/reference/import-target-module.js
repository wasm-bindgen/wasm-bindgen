import source wasmModule from "./reference_test_bg.wasm";

let wasm;
import { default as default1 } from 'tests/wasm/import_class.js';
import * as import0 from 'tests/wasm/imports.js'
import * as import1 from 'foo-raw'
import * as import2 from './snippets/reference-test-ddc0ab9a51c9d25f/inline0.js'
import * as import3 from 'pure-extern'

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
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

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

export function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

const imports = {
    __wbindgen_placeholder__: {
        __wbg_catchme_f7d87ea824a61e87: function() { return handleError(function () {
            catch_me();
        }, arguments) },
        __wbg_get_56ba567010fb9959: function(arg0) {
            const ret = arg0.get();
            return ret;
        },
        __wbg_myfunction_8c7b624429f78550: function() {
            b.my_function();
        },
        __wbg_new_d21827b66c7fd25d: function(arg0) {
            const ret = new default1(arg0);
            return ret;
        },
        __wbg_nocatch_be850a8dddd9599d: function() {
            no_catch();
        },
        __wbg_reload_84c12f152ad689f0: function() {
            window.location.reload();
        },
        __wbg_static_accessor_CONST_9e9d5ae758197645: function() {
            const ret = a.CONST;
            return ret;
        },
        __wbg_wbindgenthrow_4c11a24fca429ccf: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_write_c2ce0ce33a6087d5: function(arg0, arg1) {
            window.document.write(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_export_2;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
            ;
        },
    },
    'tests/wasm/imports.js': import0,  'foo-raw': import1,  './snippets/reference-test-ddc0ab9a51c9d25f/inline0.js': import2,  'pure-extern': import3,
};

const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;

wasm.__wbindgen_start();

