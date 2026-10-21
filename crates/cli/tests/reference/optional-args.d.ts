/* tslint:disable */
/* eslint-disable */

export type Flag = "yes" | "no";

export class Opt {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    get n(): number | undefined;
    set n(value: number | null | undefined);
}

export function all_optional(a?: number | null, b?: number | null, c?: number | null): void;

export function optional_bytes(b?: Uint8Array | number[] | null, c?: boolean | null, d?: string | null): void;

export function optional_string(s?: string | null): string | undefined;

export function optional_string_enum(f?: Flag | null): void;

export function optional_struct(o?: Opt | null, v?: any | null): void;

export function some_optional(a: number | null | undefined, b: number, c?: number | null): void;
