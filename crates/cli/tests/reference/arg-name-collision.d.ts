/* tslint:disable */
/* eslint-disable */

export class Foo {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Names that don't collide stay unchanged.
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
 * Named like locals of the generated function body.
 */
export function local_args(ptr0: string, len0: number, ret: number): string;

/**
 * Named like the module-level `wasm` object.
 */
export function wasm_arg(wasm: Uint8Array): string;
