/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}
export function __wbg_random_9526caf33df4270d() {
    const ret = Math.random();
    return ret;
}
export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
}
function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getObject(idx) { return heap[idx]; }

let heap = new Array(128).fill(undefined);
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
