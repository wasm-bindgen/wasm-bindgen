/* tslint:disable */
/* eslint-disable */

/**
 * Export returning a primitive: TypeScript becomes `(): Promise<number>`.
 */
export function compute(): Promise<number>;

/**
 * Export returning void: wrapped with `WebAssembly.promising` in JS.
 * TypeScript signature becomes `(): Promise<void>`.
 */
export function do_work(): Promise<void>;
