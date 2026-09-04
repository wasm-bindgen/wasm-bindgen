/* tslint:disable */
/* eslint-disable */

declare enum Level {
    Low = 0,
    High = 1,
}

declare enum Level2 {
    Low = 0,
    High = 1,
}

declare class Status {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    describe(): string;
    code: number;
}
export type { Status };

declare class Status2 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    describe(): string;
    code: number;
}
export type { Status2 };

export function a_levels(): Level[];

export function b_levels(): Level2[];

export function b_statuses(): Status2[];

export function statuses(): any[];
