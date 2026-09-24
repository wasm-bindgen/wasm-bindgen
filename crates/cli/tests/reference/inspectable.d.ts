/* tslint:disable */
/* eslint-disable */

export class Pair {
    /**
     ** Return copy of self without private attributes.
     */
    toJSON(): Object;
    /**
     * Return stringified version of self.
     */
    toString(): string;
    free(): void;
    [Symbol.dispose](): void;
    constructor(a: number, b: number);
    0: number;
    1: number;
}

export class Point {
    /**
     ** Return copy of self without private attributes.
     */
    toJSON(): Object;
    /**
     * Return stringified version of self.
     */
    toString(): string;
    free(): void;
    [Symbol.dispose](): void;
    constructor(x: number, y: number);
    x: number;
    y: number;
}
