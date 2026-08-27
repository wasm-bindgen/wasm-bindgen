declare namespace custom_global {
    /* tslint:disable */
    /* eslint-disable */

    export function add(a: number, b: number): number;

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly add: (a: number, b: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

declare type SyncInitInput = BufferSource | WebAssembly.Module;

declare namespace custom_global {
    /**
     * Instantiates the given `module`, which can either be bytes or
     * a precompiled `WebAssembly.Module`.
     *
     * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
     *
     * @returns {InitOutput}
     */
    export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;
}

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
declare function custom_global (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
