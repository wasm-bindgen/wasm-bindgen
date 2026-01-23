/* tslint:disable */
/* eslint-disable */

export class WasmAudioProcessor {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    pack(): number;
    process(buf: Float32Array): boolean;
    static unpack(val: number): WasmAudioProcessor;
}

export function web_main(): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly __wbg_wasmaudioprocessor_free: (a: number, b: number) => void;
    readonly wasmaudioprocessor_pack: (a: number) => number;
    readonly wasmaudioprocessor_process: (a: number, b: number, c: number, d: any) => number;
    readonly wasmaudioprocessor_unpack: (a: number) => number;
    readonly web_main: () => any;
    readonly wasm_bindgen_1c53a5e6b3b75beb___closure__destroy___dyn_core_86059c030ec27b87___ops__function__FnMut__web_sys_54e13e2fa7474746___features__gen_Event__Event____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_1c53a5e6b3b75beb___closure__destroy___dyn_core_86059c030ec27b87___ops__function__FnMut__wasm_bindgen_1c53a5e6b3b75beb___JsValue____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_1c53a5e6b3b75beb___convert__closures_____invoke___wasm_bindgen_1c53a5e6b3b75beb___JsValue__wasm_bindgen_1c53a5e6b3b75beb___JsValue_____: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_1c53a5e6b3b75beb___convert__closures_____invoke___web_sys_54e13e2fa7474746___features__gen_Event__Event_____: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_1c53a5e6b3b75beb___convert__closures_____invoke___wasm_bindgen_1c53a5e6b3b75beb___JsValue_____: (a: number, b: number, c: any) => void;
    readonly memory: WebAssembly.Memory;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
    readonly __wbindgen_start: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number }} module - Passing `SyncInitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number } | SyncInitInput, memory?: WebAssembly.Memory): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;
