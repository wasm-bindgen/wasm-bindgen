/* tslint:disable */
/* eslint-disable */

/**
 * Delete the file at `path`.  Throws if the path does not exist.
 */
export function opfs_delete(path: string): Promise<void>;

/**
 * Return `true` if `path` exists in the Origin Private File System.
 * Returns `false` if any path component (directory or file) is missing.
 */
export function opfs_has(path: string): Promise<boolean>;

/**
 * Read and return the UTF-8 text content of `path`.
 * Throws if the path does not exist.
 */
export function opfs_read(path: string): Promise<string>;

/**
 * Write UTF-8 `content` to `path` in the Origin Private File System.
 *
 * Creates intermediate directories and the file if they do not exist;
 * overwrites the file if it does.
 */
export function opfs_write(path: string, content: string): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly opfs_delete: (a: number, b: number) => void;
    readonly opfs_has: (a: number, b: number) => number;
    readonly opfs_read: (a: number, b: number) => [number, number];
    readonly opfs_write: (a: number, b: number, c: number, d: number) => void;
    readonly __wbg_jspi_task_poll: (a: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
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
