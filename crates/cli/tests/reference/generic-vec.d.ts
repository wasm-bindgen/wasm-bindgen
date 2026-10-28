/* tslint:disable */
/* eslint-disable */

export class Point {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    x: number;
}

export function bytes(v: Uint8Array | number[]): Uint8Array;

export function floats(v: Float64Array | number[]): Float64Array;

export function foos(v: any[]): any[];

export function give_nested_strings(): (string[])[];

export function imports(): void;

export function nested(v: (Uint8Array | number[])[]): Uint8Array[];

export function opt_bytes(v: (number | null | undefined)[]): (number | undefined)[];

export function opt_foos(v: (any | null | undefined)[]): (any | undefined)[];

export function points(v: Point[]): Point[];

export function strings(v: string[]): string[];

export function values(v: any[]): any[];

export function wide(v: bigint[]): bigint[];
