//#region exports

export class Item {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ItemFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_item_free(ptr, 0);
    }
    /**
     * @param {Item} other
     * @param {Item | null | undefined} third
     * @param {Item} fourth
     */
    eat(other, third, fourth) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertClass(other, Item);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        if (!isLikeNone(third)) {
            _assertClass(third, Item);
            if (third.__wbg_ptr === 0) {
                throw new Error('Attempt to use a moved value');
            }
        }
        _assertClass(fourth, Item);
        if (fourth.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        let ptr, ptr0, ptr1 = 0;
        try {
            ptr = this.__destroy_into_raw();
            _assertNum(ptr);
            if (other.__wbg_ptr === 0) {
                throw new Error('Attempt to use a moved value');
            }
            ptr0 = other.__destroy_into_raw();
            if (!isLikeNone(third)) {
                if (third.__wbg_ptr === 0) {
                    throw new Error('Attempt to use a moved value');
                }
            }
            if (!isLikeNone(third)) {
                ptr1 = third.__destroy_into_raw();
            }
        } catch (e) {
            if (ptr1) wasm.__wbg_item_free(ptr1, 1);
            if (ptr0) wasm.__wbg_item_free(ptr0, 1);
            if (ptr) wasm.__wbg_item_free(ptr, 1);
            throw e;
        }
        wasm.item_eat(ptr, ptr0, ptr1, fourth.__wbg_ptr);
    }
}
if (Symbol.dispose) Item.prototype[Symbol.dispose] = Item.prototype.free;

//#endregion

//#region wasm imports
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

//#endregion
const ItemFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_item_free(ptr, 1));


//#region intrinsics
function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
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

function isLikeNone(x) {
    return x === undefined || x === null;
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


//#endregion

//#region wasm loading

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

//#endregion
