/**
 * Export returning a primitive: TypeScript becomes `(): Promise<number>`.
 * @returns {Promise<number>}
 */
export async function compute() {
    if (__jspi_sync_sp === undefined) __jspi_sync_sp = wasm.__stack_pointer.value;
    else wasm.__stack_pointer.value = __jspi_sync_sp;
    const __jspi_stack = __jspi_stack_alloc();
    __jspi_active_floor = __jspi_stack + __jspi_guard_size;
    wasm.__stack_pointer.value = __jspi_stack + __jspi_stack_size;
    try {
        const ret = await (__wbg_jspi_compute ??= WebAssembly.promising(wasm.compute))();
        return ret >>> 0;
    } finally {
        wasm.__stack_pointer.value = __jspi_sync_sp;
        __jspi_stack_free(__jspi_stack);
        __jspi_active_floor = 0;
    }
}

/**
 * Export returning void: wrapped with `WebAssembly.promising` in JS.
 * TypeScript signature becomes `(): Promise<void>`.
 * @returns {Promise<void>}
 */
export function do_work() {
    if (__jspi_sync_sp === undefined) __jspi_sync_sp = wasm.__stack_pointer.value;
    else wasm.__stack_pointer.value = __jspi_sync_sp;
    const __jspi_stack = __jspi_stack_alloc();
    __jspi_active_floor = __jspi_stack + __jspi_guard_size;
    wasm.__stack_pointer.value = __jspi_stack + __jspi_stack_size;
    return (__wbg_jspi_do_work ??= WebAssembly.promising(wasm.do_work))().finally(() => { wasm.__stack_pointer.value = __jspi_sync_sp; __jspi_stack_free(__jspi_stack); __jspi_active_floor = 0; });
}

export const __wbg_fetch_number_65eabd7e4b01732a = ((__inner) => new WebAssembly.Suspending(async function(...args) {
    const __sp = wasm.__stack_pointer.value;
    const __floor = __jspi_active_floor;
    if (__sp <= __floor) throw new RangeError('JSPI fiber stack overflow');
    try { return await __inner(...args); }
    finally { wasm.__stack_pointer.value = __sp; __jspi_active_floor = __floor; }
}))(function() {
    const ret = fetch_number();
    return ret;
});

export const __wbg_sleep_319b371bcbeaac51 = ((__inner) => new WebAssembly.Suspending(async function(...args) {
    const __sp = wasm.__stack_pointer.value;
    const __floor = __jspi_active_floor;
    if (__sp <= __floor) throw new RangeError('JSPI fiber stack overflow');
    try { return await __inner(...args); }
    finally { wasm.__stack_pointer.value = __sp; __jspi_active_floor = __floor; }
}))(function(arg0) {
    return sleep(arg0 >>> 0);
});
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
let __wbg_jspi_compute;


let __wbg_jspi_do_work;

let __jspi_sync_sp;
let __jspi_active_floor = 0;
const __jspi_stack_size = 65536;
const __jspi_guard_size = 8192;
const __jspi_stack_pool = [];
function __jspi_stack_alloc() {
    if (__jspi_stack_pool.length > 0) return __jspi_stack_pool.pop();
    const ptr = wasm.memory.grow(1);
    if (ptr === -1) throw new RangeError('out of memory allocating JSPI fiber stack');
    return ptr * 65536;
}
function __jspi_stack_free(ptr) { __jspi_stack_pool.push(ptr); }


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
