/* tslint:disable */
/* eslint-disable */
export function web_main(): Promise<void>;
export class WasmAudioProcessor {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  pack(): number;
  static unpack(val: number): WasmAudioProcessor;
  process(buf: Float32Array): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly __wbg_wasmaudioprocessor_free: (a: number, b: number) => void;
  readonly wasmaudioprocessor_pack: (a: number) => number;
  readonly wasmaudioprocessor_process: (a: number, b: number, c: number, d: any) => number;
  readonly wasmaudioprocessor_unpack: (a: number) => number;
  readonly web_main: () => any;
  readonly wasm_bindgen__convert__closures_____invoke__h0416d909e16000eb: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h6ff53e0cec6eac6a: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h471e4107e0c49baa: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h22a87733bfb541df: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h270873f5f46b067b: (a: number, b: number, c: any, d: any) => void;
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
