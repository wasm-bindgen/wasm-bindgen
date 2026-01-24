/* tslint:disable */
/* eslint-disable */

export function __wbg_reset_state(): void;

export function add_that_might_fail(a: number, b: number): number;

export type SyncInitInput = BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly add_that_might_fail: (a: number, b: number) => number;
    readonly memory: WebAssembly.Memory;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
    readonly __wbindgen_start: (a: number) => void;
}

export interface InitSyncOptions {
    module?: SyncInitInput;
    memory?: WebAssembly.Memory;
    thread_stack_size?: number;
}

/**
 * Initialize the WebAssembly module synchronously.
 *
 * For the main thread, this is called automatically on import.
 * Worker threads should call this explicitly with shared module and memory:
 *
 * ```js
 * initSync({ module: __wbg_wasm_module, memory: __wbg_memory });
 * ```
 *
 * @param opts - Initialization options
 * @returns The exports object
 */
export function initSync(opts?: InitSyncOptions): InitOutput;

/**
 * Get the imports object for WebAssembly instantiation.
 *
 * @param memory - Optional shared memory to use instead of creating new
 * @returns The imports object for WebAssembly.Instance
 */
export function __wbg_get_imports(memory?: WebAssembly.Memory): WebAssembly.Imports;

/** The compiled WebAssembly module. Can be shared with workers. */
export const __wbg_wasm_module: WebAssembly.Module;

/** The shared WebAssembly memory. */
export const __wbg_memory: WebAssembly.Memory;
