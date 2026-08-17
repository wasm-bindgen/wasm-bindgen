/* tslint:disable */
/* eslint-disable */

/**
 * Stress-tests that `__stack_pointer` is correctly restored after suspension
 * for arbitrary call depth and for heap allocations that occur after resume.
 *
 * The call tree recurses `depth` levels deep (each frame has a 1 KiB
 * shadow-stack local), suspends at the bottom, then allocates a `Vec` on
 * the way back up through every frame.
 *
 * ## Why this validates the shadow-stack invariant
 *
 * `__stack_pointer` is a Wasm *global*; JSPI preserves Wasm locals across
 * fiber switches but **not** globals.  If another fiber runs while this one
 * is suspended it would overwrite both the global and the shadow-stack
 * memory itself.  wasm-bindgen instruments every
 * `#[wasm_bindgen(suspending)]` import with an in-wasm wrapper that
 * evacuates the fiber's live shadow-stack region to the heap before
 * suspending, and copies it back — restoring `__stack_pointer` from a wasm
 * local — as the very first instructions after resume.  By the time any Rust
 * instruction executes after `block_on_promise` returns, the whole stack is
 * already correct — even after 20 nested frames and even when the very next
 * operation allocates from the heap.
 *
 * ## Expected return value
 *
 * `deep_alloc(N)` returns `1000 + N*(N+1)/2`.
 * For `deep_alloc(20)` the expected value is **1210**.
 */
export function deep_alloc(depth: number): Promise<number>;

/**
 * Demonstrates that fibers use the full main shadow stack: ~96 KiB of live
 * stack frames survive a suspension with no size tuning required.
 *
 * wasm-bindgen instruments the module so that on suspension the fiber's live
 * shadow-stack region is evacuated to the heap and copied back — to the same
 * addresses — immediately on resume. Returns `49152`.
 */
export function deep_stack(): Promise<number>;

/**
 * Sleep for `ms` milliseconds.
 *
 * This is a plain (non-`async`) Rust function, yet it awaits a `setTimeout`
 * promise via JSPI without blocking the browser's event loop.
 *
 * The `#[wasm_bindgen(jspi)]` attribute causes the generated JS glue to wrap
 * this export with `WebAssembly.promising`, so callers receive a `Promise`.
 */
export function do_sleep(ms: number): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly deep_alloc: (a: number) => number;
    readonly deep_stack: () => number;
    readonly do_sleep: (a: number) => void;
    readonly __wbg_jspi_task_poll: (a: number) => void;
    readonly wasm_bindgen_a42b010618f9b99f___convert__closures_____invoke___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined___js_sys_476ea7fb5b6ab2e8___Function_fn_wasm_bindgen_a42b010618f9b99f___JsValue_____wasm_bindgen_a42b010618f9b99f___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
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
