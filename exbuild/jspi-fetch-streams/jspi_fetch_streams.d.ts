/* tslint:disable */
/* eslint-disable */

/**
 * Fetch `url` and stream-read the response body via JSPI.
 *
 * The fiber suspends once while waiting for the response headers, then
 * once per body chunk — all from plain, non-`async` Rust.
 *
 * Works on Chrome, Firefox, and Safari (response streaming is universally
 * supported).  Returns `[total_bytes, chunk_count]`.
 */
export function fetch_stream(url: string): Promise<Array<any>>;

/**
 * Read a JavaScript `ReadableStream` from plain Rust via JSPI.
 *
 * Each `reader.read()` suspends the WASM fiber until the next chunk
 * arrives — no `async fn`, no `.await`, no event-loop blocking.
 *
 * Accepts any `ReadableStream`: a `fetch` response body, a request body
 * forwarded from a Service Worker, a synthetic stream, etc.
 *
 * Returns `[total_bytes, chunk_count]`.
 */
export function read_stream(stream: ReadableStream): Promise<Array<any>>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly fetch_stream: (a: number, b: number) => [number, number, number];
    readonly read_stream: (a: any) => [number, number, number];
    readonly __wbg_jspi_task_poll: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
