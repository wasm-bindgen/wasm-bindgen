export function give() {
    wasm.give();
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

function getObject(idx) { return heap[idx]; }

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
