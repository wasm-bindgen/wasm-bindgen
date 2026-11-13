export class Item {
    static __wrap(ptr) {
        const obj = Object.create(Item.prototype);
        obj.__wbg_ptr = ptr;
        ItemFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
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
}
if (Symbol.dispose) Item.prototype[Symbol.dispose] = Item.prototype.free;

export function give() {
    wasm.give();
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_takes_beside_117c0a886bf3caab(arg0, arg1, arg2, arg3) {
    let taken0 = false;
    const b1 = arg2 >>> 0;
    try {
        const v1 = [];
        let p1 = b1;
        const view1 = getDataViewMemory0();
        for (const end1 = p1 + arg3 * 8; p1 < end1; p1 += 8) {
            const w1_0 = view1.getInt32(p1 + 0, true);
            const w1_1 = view1.getInt32(p1 + 4, true);
            v1.push(getStringFromWasm0(w1_0, w1_1));
        }
        takes_beside((taken0 = true, Item.__wrap(arg0)), arg1, v1);
    } finally {
        if (!taken0) {
            if (arg0) wasm.__wbg_item_free(arg0, 1);
        }
        if (b1) {
            let view2 = getDataViewMemory0();
            for (let k2 = 0; k2 < arg3; k2++) {
                const cell2 = b1 + k2 * 8;
                const w2_0 = view2.getInt32(cell2 + 0, true) >>> 0;
                const w2_1 = view2.getInt32(cell2 + 4, true) >>> 0;
                if (w2_0) {
                    wasm.__wbindgen_free(w2_0, w2_1 * 1, 1);
                    view2 = getDataViewMemory0();
                }
            }
            wasm.__wbindgen_free(b1, arg3 * 8, 4);
        }
    }
}
export function __wbg_takes_optional_values_d21b5652004b5d69(arg0, arg1) {
    try {
        let v0;
        if (arg0 !== 0) {
            v0 = getArrayJsValueViewFromWasm0(arg0, arg1);
        }
        takes_optional_values(v0);
    } finally {
        if (arg0) { wasm.__externref_drop_slice(arg0, arg1); wasm.__wbindgen_free(arg0, arg1 * 4, 4); }
    }
}
export function __wbg_takes_values_1ab111644c4cbdee(arg0, arg1) {
    try {
        var v0 = getArrayJsValueViewFromWasm0(arg0, arg1);
        takes_values(v0);
    } finally {
        if (arg0) { wasm.__externref_drop_slice(arg0, arg1); wasm.__wbindgen_free(arg0, arg1 * 4, 4); }
    }
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
const ItemFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_item_free(ptr, 1));

function getArrayJsValueViewFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
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
