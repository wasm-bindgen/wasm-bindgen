/* tslint:disable */
/* eslint-disable */

export enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

export class Rectangle {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    height: number;
    width: number;
}

declare class Counter {
    free(): void;
    [Symbol.dispose](): void;
    increment(): void;
    constructor(initial: number);
    value: number;
}

declare function concat(a: string, b: string): string;

declare function uppercase(s: string): string;

declare let _default: {
    Counter: typeof Counter,
    concat: typeof concat,
    uppercase: {
        uppercase: typeof uppercase,
    },
};
export default _default;

declare function add(a: number, b: number): number;

declare function divide(a: number, b: number): number;

declare function multiply(a: number, b: number): number;

export let math: {
    add: typeof add,
    divide: typeof divide,
    multiply: typeof multiply,
};

declare class Point3D {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    x: number;
    y: number;
    z: number;
}

declare class Point {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    x: number;
    y: number;
}

export let models: {
    '3d': {
        Point3D: typeof Point3D,
    },
    Point: typeof Point,
};

export function regular_function(): number;

declare enum Status {
    Pending = 0,
    Active = 1,
    Complete = 2,
}

declare enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
}

export let types: {
    Status: typeof Status,
    http: {
        HttpStatus: typeof HttpStatus,
    },
};

declare function uppercase2(s: string): string;

export let utils: {
    string: {
        uppercase: typeof uppercase2,
    },
};
