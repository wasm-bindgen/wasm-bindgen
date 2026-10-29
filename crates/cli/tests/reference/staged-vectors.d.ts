/* tslint:disable */
/* eslint-disable */

export class Point {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export function driver(): void;

export function maybe_bytes(v: (number | null | undefined)[]): (number | undefined)[];

export function maybe_foos(v: (any | null | undefined)[]): (any | undefined)[];

export function maybe_names(v?: (string | null | undefined)[] | null): (string | undefined)[] | undefined;

export function maybe_nullable_points(v: (Point | null | undefined)[]): (Point | null | undefined)[];

export function maybe_points(v: (Point | null | undefined)[]): (Point | undefined)[];

export function names(v: string[]): string[];

export function nullable_foos(v: (any | null)[]): (any | null)[];

export function nullable_points(v: (Point | null)[]): (Point | null)[];
