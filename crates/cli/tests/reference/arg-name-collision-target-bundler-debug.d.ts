/* tslint:disable */
/* eslint-disable */

export class Foo {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Only appears in an error message of the debug glue, so stays unchanged.
     */
    add(value: number): number;
    /**
     * Named like the `ptr` local of methods taking `self` by value.
     */
    consume(ptr: number): number;
    constructor(value: number);
}

/**
 * Named like a class the generated function body refers to.
 */
export function class_arg(Foo: Foo): Foo;

/**
 * Named like a helper function and a JS global the body references.
 */
export function global_args(isLikeNone: number | null | undefined, undefined: number): number | undefined;

/**
 * Named like locals of the generated function body.
 */
export function local_args(ptr0: string, len0: number, ret: number): string;

/**
 * Named like the module-level `wasm` object; `wasm2` is already taken.
 */
export function wasm_args(wasm: Uint8Array, wasm2: number): string;
