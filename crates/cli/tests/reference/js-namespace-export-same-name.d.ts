/* tslint:disable */
/* eslint-disable */

declare class Point {
    free(): void;
    [Symbol.dispose](): void;
    constructor(x: number, y: number);
    x: number;
    y: number;
}

declare enum Status {
    Pending = 0,
    Complete = 1,
    Failed = 2,
}

declare function greet(): string;

export let bar: {
    Point: typeof Point,
    Status: typeof Status,
    greet: typeof greet,
};

/**
 * Two structs with the same js_name in different namespaces should not collide.
 */
declare class Point2 {
    free(): void;
    [Symbol.dispose](): void;
    constructor(x: number);
    x: number;
}

/**
 * Two enums with the same js_name in different namespaces should not collide.
 */
declare enum Status2 {
    Active = 0,
    Inactive = 1,
}

/**
 * Two functions with the same js_name in different namespaces should not collide.
 */
declare function greet2(): string;

export let foo: {
    Point: typeof Point2,
    Status: typeof Status2,
    greet: typeof greet2,
};
