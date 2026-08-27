/* tslint:disable */
/* eslint-disable */

export function async_bytes(): Promise<Uint8Array<ArrayBuffer>>;

export function optional_bytes(): Uint8Array<ArrayBuffer> | undefined;

export function owned_bytes(): Uint8Array<ArrayBuffer>;

export function owned_floats(): Float32Array<ArrayBuffer>;

export function roundtrip(borrowed: Uint8Array, owned: Uint8Array): Uint8Array<ArrayBuffer>;

export function strings(input: string[]): string[];
