/* tslint:disable */
/* eslint-disable */

export class BarPoint {
    free(): void;
    [Symbol.dispose](): void;
    constructor(y: number);
    readonly y: number;
}

export class FooPoint {
    free(): void;
    [Symbol.dispose](): void;
    constructor(x: number);
    readonly x: number;
}
