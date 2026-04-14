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

declare class default__Counter {
    free(): void;
    [Symbol.dispose](): void;
    increment(): void;
    constructor(initial: number);
    value: number;
}

declare function default__concat(a: string, b: string): string;

declare function default__uppercase__uppercase(s: string): string;

declare let _default: {
    Counter: typeof default__Counter,
    concat: typeof default__concat,
    uppercase: {
        uppercase: typeof default__uppercase__uppercase,
    },
};
export default _default;

declare function math__add(a: number, b: number): number;

declare function math__divide(a: number, b: number): number;

declare function math__multiply(a: number, b: number): number;

export let math: {
    add: typeof math__add,
    divide: typeof math__divide,
    multiply: typeof math__multiply,
};

declare class models__3d__Point3D {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    x: number;
    y: number;
    z: number;
}

declare class models__Point {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    x: number;
    y: number;
}

export let models: {
    '3d': {
        Point3D: typeof models__3d__Point3D,
    },
    Point: typeof models__Point,
};

export function regular_function(): number;

declare enum types__Status {
    Pending = 0,
    Active = 1,
    Complete = 2,
}

declare enum types__http__HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
}

export let types: {
    Status: typeof types__Status,
    http: {
        HttpStatus: typeof types__http__HttpStatus,
    },
};

declare function utils__string__uppercase(s: string): string;

export let utils: {
    string: {
        uppercase: typeof utils__string__uppercase,
    },
};
