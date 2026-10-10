/* tslint:disable */
/* eslint-disable */

export function add(a: number, b: number): number;

export function call_doesnt_throw_n_times(n: number): void;

export function call_doesnt_throw_with_catch_n_times(n: number): void;

export function call_first_child_final_n_times(n: number, element: any): void;

export function call_first_child_structural_n_times(n: number, element: any): void;

export function call_foo_bar_final_n_times(n: number, js_foo: any): void;

export function call_foo_bar_structural_n_times(n: number, js_foo: any): void;

export function call_js_add_n_times(n: number, a: number, b: number): void;

export function call_js_thunk_n_times(n: number): void;

export function call_node_first_child_n_times(n: number, elements: any[]): void;

export function call_node_has_child_nodes_n_times(n: number, elements: any[]): void;

export function call_node_node_type_n_times(n: number, elements: any[]): void;

export function call_use_baz_n_times(n: number): void;

export function count_node_types(element: Node): void;

export function fibonacci(n: number): number;

export function fibonacci_high(): number;

export function str_roundtrip(s: string): string;

export function thunk(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly add: (a: number, b: number) => number;
    readonly call_doesnt_throw_n_times: (a: number) => void;
    readonly call_doesnt_throw_with_catch_n_times: (a: number) => void;
    readonly call_first_child_final_n_times: (a: number, b: any) => void;
    readonly call_first_child_structural_n_times: (a: number, b: any) => void;
    readonly call_foo_bar_final_n_times: (a: number, b: any) => void;
    readonly call_foo_bar_structural_n_times: (a: number, b: any) => void;
    readonly call_js_add_n_times: (a: number, b: number, c: number) => void;
    readonly call_js_thunk_n_times: (a: number) => void;
    readonly call_node_first_child_n_times: (a: number, b: number, c: number) => void;
    readonly call_node_has_child_nodes_n_times: (a: number, b: number, c: number) => void;
    readonly call_node_node_type_n_times: (a: number, b: number, c: number) => void;
    readonly call_use_baz_n_times: (a: number) => void;
    readonly count_node_types: (a: any) => void;
    readonly fibonacci: (a: number) => number;
    readonly fibonacci_high: () => number;
    readonly str_roundtrip: (a: number, b: number) => [number, number];
    readonly thunk: () => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
