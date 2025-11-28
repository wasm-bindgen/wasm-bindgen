
let imports = {};
imports['./snippets/import_reftest-a82831e16a4c30f1/inline0.js'] = require('./snippets/import_reftest-a82831e16a4c30f1/inline0.js');
imports['__wbindgen_placeholder__'] = module.exports;

imports['foo-raw'] = require('foo-raw');
imports['pure-extern'] = require('pure-extern');
imports['tests/wasm/imports.js'] = require('tests/wasm/imports.js');
const { default: _default } = require(`tests/wasm/import_class.js`);

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

function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}
exports.exported = exported;

exports.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

exports.__wbg_catch_me_1d18acaa34acb005 = function() { return handleError(function () {
    catch_me();
}, arguments) };

exports.__wbg_get_c871386e44ba8c35 = function(arg0) {
    const ret = arg0.get();
    return ret;
};

exports.__wbg_my_function_597f96bc4719408a = function() {
    b.my_function();
};

exports.__wbg_new_c30895ccee9479d4 = function(arg0) {
    const ret = new _default(arg0);
    return ret;
};

exports.__wbg_no_catch_757175fbf9e08b9e = function() {
    no_catch();
};

exports.__wbg_reload_b091d4dc4b1b3a74 = function() {
    window.location.reload();
};

exports.__wbg_static_accessor_CONST_85b96acb48be57e1 = function() {
    const ret = a.CONST;
    return ret;
};

exports.__wbg_write_691fc0d693f0c7b5 = function(arg0, arg1) {
    window.document.write(getStringFromWasm0(arg0, arg1));
};

exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
};

const wasmPath = `${__dirname}/reference_test_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = exports.__wasm = new WebAssembly.Instance(wasmModule, imports).exports;

wasm.__wbindgen_start();
