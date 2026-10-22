/* tslint:disable */
/* eslint-disable */

export type Flag = "yes" | "no";

export class Opt {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    n?: number | null;
}

export function all_nullable(a?: number | null, b?: number | null, c?: number | null): void;

export function nullable_bytes(b?: Uint8Array | number[] | null, c?: boolean | null, d?: string | null): Uint8Array | null | undefined;

export function nullable_string(s?: string | null): string | null | undefined;

export function nullable_string_enum(f?: Flag | null): Flag | null | undefined;

export function nullable_struct(o?: Opt | null, v?: any | null): Opt | null | undefined;

export function some_nullable(a: number | null | undefined, b: number, c?: number | null): void;
