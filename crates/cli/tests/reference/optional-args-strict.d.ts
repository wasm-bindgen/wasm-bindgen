/* tslint:disable */
/* eslint-disable */

export type Flag = "yes" | "no";

export class Opt {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    n?: number;
}

export function all_optional(a?: number, b?: number, c?: number): void;

export function optional_bytes(b?: Uint8Array | number[], c?: boolean, d?: string): void;

export function optional_string(s?: string): string | undefined;

export function optional_string_enum(f?: Flag): void;

export function optional_struct(o?: Opt, v?: any): void;

export function some_optional(a: number | undefined, b: number, c?: number): void;
