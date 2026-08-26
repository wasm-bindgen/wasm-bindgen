/* tslint:disable */
/* eslint-disable */

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

export function statuses(): any[];
