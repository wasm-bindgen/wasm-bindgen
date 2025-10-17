/* tslint:disable */
/* eslint-disable */

/**
 * Runtime test harness support instantiated in JS.
 *
 * The node.js entry script instantiates a `Context` here which is used to
 * drive test execution.
 */
export class WasmBindgenTestContext {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Handle filter argument.
     */
    filtered_count(filtered: number): void;
    /**
     * Handle `--include-ignored` flag.
     */
    include_ignored(include_ignored: boolean): void;
    /**
     * Creates a new context ready to run tests.
     *
     * A `Context` is the main structure through which test execution is
     * coordinated, and this will collect output and results for all executed
     * tests.
     */
    constructor(is_bench: boolean);
    /**
     * Executes a list of tests, returning a promise representing their
     * eventual completion.
     *
     * This is the main entry point for executing tests. All the tests passed
     * in are the JS `Function` object that was plucked off the
     * `WebAssembly.Instance` exports list.
     *
     * The promise returned resolves to either `true` if all tests passed or
     * `false` if at least one test failed.
     */
    run(tests: any[]): Promise<any>;
}

/**
 * Used to read benchmark data, and then the runner stores it on the local disk.
 */
export function __wbgbench_dump(): Uint8Array | undefined;

/**
 * Used to write previous benchmark data before the benchmark, for later comparison.
 */
export function __wbgbench_import(baseline: Uint8Array): void;

/**
 * Handler for `console.debug` invocations. See above.
 */
export function __wbgtest_console_debug(args: Array<any>): void;

/**
 * Handler for `console.error` invocations. See above.
 */
export function __wbgtest_console_error(args: Array<any>): void;

/**
 * Handler for `console.info` invocations. See above.
 */
export function __wbgtest_console_info(args: Array<any>): void;

/**
 * Handler for `console.log` invocations.
 *
 * If a test is currently running it takes the `args` array and stringifies
 * it and appends it to the current output of the test. Otherwise it passes
 * the arguments to the original `console.log` function, psased as
 * `original`.
 */
export function __wbgtest_console_log(args: Array<any>): void;

/**
 * Handler for `console.warn` invocations. See above.
 */
export function __wbgtest_console_warn(args: Array<any>): void;

export function __wbgtest_cov_dump(): Uint8Array | undefined;

/**
 * Path to use for coverage data.
 */
export function __wbgtest_coverage_path(env: string | null | undefined, pid: number, temp_dir: string, module_signature: bigint): string;

export function __wbgtest_module_signature(): bigint | undefined;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly "__wbgt__wasm_export_colon_reftest::colon_test": (a: number) => void;
    readonly __wbg_wasmbindgentestcontext_free: (a: number, b: number) => void;
    readonly __wbgbench_dump: () => [number, number];
    readonly __wbgbench_import: (a: number, b: number) => void;
    readonly __wbgtest_console_debug: (a: any) => void;
    readonly __wbgtest_console_error: (a: any) => void;
    readonly __wbgtest_console_info: (a: any) => void;
    readonly __wbgtest_console_log: (a: any) => void;
    readonly __wbgtest_console_warn: (a: any) => void;
    readonly __wbgtest_cov_dump: () => [number, number];
    readonly __wbgtest_coverage_path: (a: number, b: number, c: number, d: number, e: number, f: bigint) => [number, number];
    readonly __wbgtest_module_signature: () => [number, bigint];
    readonly wasmbindgentestcontext_filtered_count: (a: number, b: number) => void;
    readonly wasmbindgentestcontext_include_ignored: (a: number, b: number) => void;
    readonly wasmbindgentestcontext_new: (a: number) => number;
    readonly wasmbindgentestcontext_run: (a: number, b: number, c: number) => any;
    readonly wasm_bindgen__closure__destroy__h0000000000000008: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0000000000000004: (a: number, b: number, c: any, d: number, e: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0000000000000009: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h0000000000000005: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0000000000000006: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__h0000000000000007: (a: number, b: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
