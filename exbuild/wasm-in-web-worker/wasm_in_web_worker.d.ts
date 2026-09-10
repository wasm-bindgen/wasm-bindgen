declare namespace wasm_bindgen {
    /* tslint:disable */
    /* eslint-disable */

    /**
     * A number evaluation struct
     *
     * This struct will be the main object which responds to messages passed to the
     * worker. It stores the last number which it was passed to have a state. The
     * statefulness is not required in this example but should show how
     * larger, more complex scenarios with statefulness can be set up.
     */
    export class NumberEval {
        private constructor();
        free(): void;
        [Symbol.dispose](): void;
        /**
         * Get last number that was checked - this method is added to work with
         * statefulness.
         */
        get_last_number(): number;
        /**
         * Check if a number is even and store it as last processed number.
         *
         * # Arguments
         *
         * * `number` - The number to be checked for being even/odd.
         */
        is_even(number: number): boolean;
        /**
         * Create new instance.
         */
        static new(): NumberEval;
    }

    /**
     * Run entry point for the main thread.
     */
    export function startup(): void;

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_numbereval_free: (a: number, b: number) => void;
    readonly numbereval_get_last_number: (a: number) => number;
    readonly numbereval_is_even: (a: number, b: number) => number;
    readonly numbereval_new: () => number;
    readonly startup: () => void;
    readonly wasm_bindgen_39f762a0b0ba065a___convert__closures_____invoke___web_sys_8c9a2da38fd826b1___features__gen_MessageEvent__MessageEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_39f762a0b0ba065a___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __wbindgen_start: () => void;
}

declare type SyncInitInput = BufferSource | WebAssembly.Module;

declare namespace wasm_bindgen {
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
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
