/**
 * A color.
 * @enum {0 | 1 | 2}
 */
export const Color = Object.freeze({
    /**
     * Green as a leaf.
     */
    Green: 0, "0": "Green",
    /**
     * Yellow as the sun.
     */
    Yellow: 1, "1": "Yellow",
    /**
     * Red as a rose.
     */
    Red: 2, "2": "Red",
});

/**
 * @enum {0 | 1 | 42 | 43}
 */
export const ImplicitDiscriminant = Object.freeze({
    A: 0, "0": "A",
    B: 1, "1": "B",
    C: 42, "42": "C",
    D: 43, "43": "D",
});

/**
 * A C-style enum with negative discriminants.
 * @enum {-1 | 0 | 1}
 */
export const Ordering = Object.freeze({
    Less: -1, "-1": "Less",
    Equal: 0, "0": "Equal",
    Greater: 1, "1": "Greater",
});

/**
 * @param {Color} color
 * @returns {Color}
 */
export function enum_echo(color) {
    const ret = wasm.enum_echo(color);
    return ret;
}

/**
 * @param {Color} color
 * @returns {ColorName}
 */
export function get_name(color) {
    const ret = wasm.get_name(color);
    return __wbindgen_enum_ColorName[ret];
}

/**
 * @param {Color | null} [color]
 * @returns {Color | undefined}
 */
export function option_enum_echo(color) {
    const ret = wasm.option_enum_echo(isLikeNone(color) ? 3 : color);
    return ret === 3 ? undefined : ret;
}

/**
 * @param {Ordering | null} [order]
 * @returns {Ordering | undefined}
 */
export function option_order(order) {
    const ret = wasm.option_order(isLikeNone(order) ? 2 : order);
    return ret === 2 ? undefined : ret;
}

/**
 * @param {ColorName | null} [color]
 * @returns {ColorName | undefined}
 */
export function option_string_enum_echo(color) {
    const ret = wasm.option_string_enum_echo(isLikeNone(color) ? 4 : ((__wbindgen_enum_ColorName.indexOf(color) + 1 || 4) - 1));
    return __wbindgen_enum_ColorName[ret];
}
export function __wbg___wbindgen_throw_df03e93053e0f4bc(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
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
const __wbindgen_enum_ColorName = ["green", "yellow", "red"];

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
