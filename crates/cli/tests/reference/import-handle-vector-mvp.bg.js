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
export function __wbg_takes_beside_8e50eaf734ab6671(arg0, arg1, arg2, arg3) {
    let taken0 = false;
    let taken1 = false;
    const b2 = arg2 >>> 0;
    try {
        const v2 = [];
        let p2 = b2;
        const view2 = getDataViewMemory0();
        for (const end2 = p2 + arg3 * 8; p2 < end2; p2 += 8) {
            const w2_0 = view2.getInt32(p2 + 0, true);
            const w2_1 = view2.getInt32(p2 + 4, true);
            v2.push(getStringFromWasm0(w2_0, w2_1));
        }
        takes_beside((taken0 = true, Item.__wrap(arg0)), (taken1 = true, takeObject(arg1)), v2);
    } finally {
        if (!taken0) {
            if (arg0) wasm.__wbg_item_free(arg0, 1);
        }
        if (!taken1) {
            dropObject(arg1);
        }
        if (b2) {
            let view3 = getDataViewMemory0();
            for (let k3 = 0; k3 < arg3; k3++) {
                const cell3 = b2 + k3 * 8;
                const w3_0 = view3.getInt32(cell3 + 0, true) >>> 0;
                const w3_1 = view3.getInt32(cell3 + 4, true) >>> 0;
                if (w3_0) {
                    wasm.__wbindgen_free(w3_0, w3_1 * 1, 1);
                    view3 = getDataViewMemory0();
                }
            }
            wasm.__wbindgen_free(b2, arg3 * 8, 4);
        }
    }
}
export function __wbg_takes_nullable_beside_7c4d16b33d8ab86c(arg0, arg1, arg2) {
    const b0 = arg0 >>> 0;
    let taken2 = false;
    try {
        const v0 = [];
        let p0 = b0;
        const view0 = getDataViewMemory0();
        for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
            const w0_0 = view0.getInt32(p0 + 0, true);
            const w0_1 = view0.getInt32(p0 + 4, true);
            v0.push(getStringFromWasm0(w0_0, w0_1));
        }
        takes_nullable_beside(v0, (taken2 = true, arg2 === 1 ? null : Item.__wrap(arg2)));
    } finally {
        if (b0) {
            let view1 = getDataViewMemory0();
            for (let k1 = 0; k1 < arg1; k1++) {
                const cell1 = b0 + k1 * 8;
                const w1_0 = view1.getInt32(cell1 + 0, true) >>> 0;
                const w1_1 = view1.getInt32(cell1 + 4, true) >>> 0;
                if (w1_0) {
                    wasm.__wbindgen_free(w1_0, w1_1 * 1, 1);
                    view1 = getDataViewMemory0();
                }
            }
            wasm.__wbindgen_free(b0, arg1 * 8, 4);
        }
        if (!taken2) {
            if (arg2 >>> 0 > 1) wasm.__wbg_item_free(arg2, 1);
        }
    }
}
export function __wbg_takes_optional_values_81c81c200e75d4b6(arg0, arg1) {
    try {
        let v0;
        if (arg0 !== 0) {
            v0 = getArrayJsValueViewFromWasm0(arg0, arg1);
        }
        takes_optional_values(v0);
    } finally {
        if (arg0) { for (let p = arg0 >>> 0, end = p + 4 * arg1, view = getDataViewMemory0(); p < end; p += 4) dropObject(view.getUint32(p, true)); wasm.__wbindgen_free(arg0, arg1 * 4, 4); }
    }
}
export function __wbg_takes_values_d084bf53be6dc3ec(arg0, arg1) {
    try {
        var v0 = getArrayJsValueViewFromWasm0(arg0, arg1);
        takes_values(v0);
    } finally {
        if (arg0) { for (let p = arg0 >>> 0, end = p + 4 * arg1, view = getDataViewMemory0(); p < end; p += 4) dropObject(view.getUint32(p, true)); wasm.__wbindgen_free(arg0, arg1 * 4, 4); }
    }
}
export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
}
const ItemFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_item_free(ptr, 1));

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayJsValueViewFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(getObject(mem.getUint32(i, true)));
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

function getObject(idx) { return heap[idx]; }

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
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
