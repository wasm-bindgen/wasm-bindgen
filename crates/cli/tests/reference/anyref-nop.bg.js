export function foo() {
    wasm.foo();
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
