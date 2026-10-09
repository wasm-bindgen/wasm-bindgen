import { Other } from 'other';


/**
 * @enum {0 | 1}
 */
export const Enum = Object.freeze({
    A: 0, "0": "A",
    B: 1, "1": "B",
});

export class Test {
    static __wrap(ptr) {
        const obj = Object.create(Test.prototype);
        obj.__wbg_ptr = ptr;
        TestFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TestFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_test_free(ptr, 0);
    }
    /**
     * @param {number} test
     * @returns {Test}
     */
    static test1(test) {
        const ret = wasm.test_test1(test);
        return Test.__wrap(ret);
    }
    /**
     * @param {number} test
     */
    test2(test) {
        wasm.test_test2(this.__wbg_ptr, test);
    }
}
if (Symbol.dispose) Test.prototype[Symbol.dispose] = Test.prototype.free;

/**
 * @param {number} test
 * @returns {number}
 */
export function test1(test) {
    const ret = wasm.test1(test);
    return ret >>> 0;
}
export function __wbg___wbindgen_throw_41e9ee4f547fc59a(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_do_a0ba2606225d4465(arg0) {
    arg0.do();
}
export function __wbg_new_b4330c6bd880e041() {
    const ret = new Other();
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
const TestFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_test_free(ptr, 1));

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
