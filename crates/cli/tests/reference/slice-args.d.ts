/* tslint:disable */
/* eslint-disable */

export class Point {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class Samples {
    free(): void;
    [Symbol.dispose](): void;
    copy_into(out: Float64Array): void;
    extend(more: Float64Array | number[]): void;
    constructor(values: Float64Array | number[]);
    static sum(values: Int32Array | number[]): number;
    get values(): Float64Array;
    set values(value: Float64Array | number[]);
}

export function roundtrip(a: Uint8Array | number[]): Uint8Array;

export function take_clamped(a: Uint8ClampedArray): void;

export function take_f32(a: Float32Array | number[]): void;

export function take_f64(a: Float64Array | number[]): void;

export function take_i16(a: Int16Array | number[]): void;

export function take_i32(a: Int32Array | number[]): void;

export function take_i64(a: BigInt64Array | bigint[]): void;

export function take_i8(a: Int8Array | number[]): void;

export function take_isize(a: Int32Array | number[]): void;

export function take_mut_f32(a: Float32Array): void;

export function take_mut_f64(a: Float64Array): void;

export function take_mut_i16(a: Int16Array): void;

export function take_mut_i32(a: Int32Array): void;

export function take_mut_i64(a: BigInt64Array): void;

export function take_mut_i8(a: Int8Array): void;

export function take_mut_isize(a: Int32Array): void;

export function take_mut_u16(a: Uint16Array): void;

export function take_mut_u32(a: Uint32Array): void;

export function take_mut_u64(a: BigUint64Array): void;

export function take_mut_u8(a: Uint8Array): void;

export function take_mut_usize(a: Uint32Array): void;

export function take_non_numbers(a: any[], b: string[], c: Point[]): void;

export function take_optional(a: Float32Array | number[] | null | undefined, b: number, c?: BigUint64Array | bigint[] | null): void;

export function take_owned(a: Uint16Array | number[], b: BigInt64Array | bigint[]): void;

export function take_u16(a: Uint16Array | number[]): void;

export function take_u32(a: Uint32Array | number[]): void;

export function take_u64(a: BigUint64Array | bigint[]): void;

export function take_u8(a: Uint8Array | number[]): void;

export function take_usize(a: Uint32Array | number[]): void;
