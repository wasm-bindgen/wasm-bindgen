export function driver() {
    wasm.driver();
}
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_js_each_c2094a133f72a80f(arg0, arg1, arg2, arg3) {
    const lent0 = [];
    const b0 = arg0 >>> 0;
    const lent2 = [];
    const b2 = arg2 >>> 0;
    try {
        const v0 = [];
        let p0 = b0;
        for (const end0 = p0 + arg1 * 8; p0 < end0; p0 += 8) {
            const view0 = getDataViewMemory0();
            const w0_0 = view0.getInt32(p0 + 0, true);
            const w0_1 = view0.getInt32(p0 + 4, true);
            const state1 = {a: w0_0, b: w0_1};
            const cb1 = (arg0) => wasm_bindgen_0000000000000000___convert__closures_____invoke___u32__u32__true_(state1.a, state1.b, arg0);
            v0.push(cb1);
            lent0.push(() => {
                state1.a = 0;
            });
        }
        const v2 = [];
        let p2 = b2;
        for (const end2 = p2 + arg3 * 8; p2 < end2; p2 += 8) {
            const view2 = getDataViewMemory0();
            const w2_0 = view2.getInt32(p2 + 0, true);
            const w2_1 = view2.getInt32(p2 + 4, true);
            const lent3 = [];
            const v3 = [];
            let p3 = w2_0;
            p3 = p3 >>> 0;
            for (const end3 = p3 + w2_1 * 8; p3 < end3; p3 += 8) {
                const view3 = getDataViewMemory0();
                const w3_0 = view3.getInt32(p3 + 0, true);
                const w3_1 = view3.getInt32(p3 + 4, true);
                const state4 = {a: w3_0, b: w3_1};
                const cb4 = () => {
                    const a = state4.a;
                    state4.a = 0;
                    try {
                        return wasm_bindgen_0000000000000000___convert__closures_____invoke_______true_(a, state4.b, );
                    } finally {
                        state4.a = a;
                    }
                };
                v3.push(cb4);
                lent3.push(() => {
                    state4.a = 0;
                });
            }
            v2.push(v3);
            lent2.push(() => {
                for (const f of lent3) f();
            });
        }
        js_each(v0, v2);
    } finally {
        for (const f of lent0) f();
        if (b0) wasm.__wbindgen_free(b0, arg1 * 8, 4);
        for (const f of lent2) f();
        if (b2) {
            let view5 = getDataViewMemory0();
            for (let k5 = 0; k5 < arg3; k5++) {
                const cell5 = b2 + k5 * 8;
                const w5_0 = view5.getInt32(cell5 + 0, true) >>> 0;
                const w5_1 = view5.getInt32(cell5 + 4, true) >>> 0;
                if (w5_0) {
                    wasm.__wbindgen_free(w5_0, w5_1 * 8, 4);
                    view5 = getDataViewMemory0();
                }
            }
            wasm.__wbindgen_free(b2, arg3 * 8, 4);
        }
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
function wasm_bindgen_0000000000000000___convert__closures_____invoke_______true_(arg0, arg1) {
    wasm.wasm_bindgen_0000000000000000___convert__closures_____invoke_______true_(arg0, arg1);
}

function wasm_bindgen_0000000000000000___convert__closures_____invoke___u32__u32__true_(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen_0000000000000000___convert__closures_____invoke___u32__u32__true_(arg0, arg1, arg2);
    return ret >>> 0;
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
