/* tslint:disable */
/* eslint-disable */

export class NamespaceConsumer {
    free(): void;
    [Symbol.dispose](): void;
    duplicate_bar_points(points: bar__Point[]): bar__Point[];
    duplicate_foo_points(points: foo__Point[]): foo__Point[];
    constructor(foo_point: foo__Point, bar_point: bar__Point, foo_status: foo__Status, bar_status: bar__Status);
    next_bar_status(status: bar__Status): bar__Status;
    next_foo_status(status: foo__Status): foo__Status;
    normalize_bar(point: bar__Point): bar__Point;
    rotate_foo(point: foo__Point): foo__Point;
    bar_point: bar__Point;
    bar_points: bar__Point[];
    bar_status: bar__Status;
    foo_point: foo__Point;
    foo_points: foo__Point[];
    foo_status: foo__Status;
}

/**
 * A top-level export colliding with an inner namespace export should not collide.
 */
export class Point {
    free(): void;
    [Symbol.dispose](): void;
    constructor(value: number);
    value: number;
}

/**
 * A top-level enum colliding with an inner namespace export should not collide.
 */
export enum Status {
    Ready = 0,
    Done = 1,
}

declare class bar__Point {
    free(): void;
    [Symbol.dispose](): void;
    constructor(x: number, y: number);
    x: number;
    y: number;
}

declare class bar__RefToFoo {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

declare enum bar__Status {
    Pending = 0,
    Complete = 1,
    Failed = 2,
}

declare function bar__greet(): string;

declare class bar__nested__Point {
    free(): void;
    [Symbol.dispose](): void;
    constructor(magnitude: number);
    magnitude: number;
}

export let bar: {
    Point: typeof bar__Point,
    RefToFoo: typeof bar__RefToFoo,
    Status: typeof bar__Status,
    greet: typeof bar__greet,
    nested: {
        Point: typeof bar__nested__Point,
    },
};

/**
 * Two structs with the same js_name in different namespaces should not collide.
 */
declare class foo__Point {
    free(): void;
    [Symbol.dispose](): void;
    constructor(x: number);
    x: number;
}

declare class foo__RefToBar {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * Two enums with the same js_name in different namespaces should not collide.
 */
declare enum foo__Status {
    Active = 0,
    Inactive = 1,
}

/**
 * Two functions with the same js_name in different namespaces should not collide.
 */
declare function foo__greet(): string;

/**
 * Two structs with the same js_name in nested namespaces should not collide.
 */
declare class foo__nested__Point {
    free(): void;
    [Symbol.dispose](): void;
    constructor(z: number);
    z: number;
}

/**
 * Same js_name reused across different namespace depths should not collide.
 */
declare enum foo__nested__Status {
    Cold = 0,
    Warm = 1,
}

/**
 * Different exported kinds with the same js_name across namespace depths should not collide.
 */
declare function foo__nested__deep__Status(): string;

declare function foo__nested__greet(): string;

export let foo: {
    Point: typeof foo__Point,
    RefToBar: typeof foo__RefToBar,
    Status: typeof foo__Status,
    greet: typeof foo__greet,
    nested: {
        Point: typeof foo__nested__Point,
        Status: typeof foo__nested__Status,
        deep: {
            Status: typeof foo__nested__deep__Status,
        },
        greet: typeof foo__nested__greet,
    },
};

/**
 * A top-level function colliding with an inner namespace export should not collide.
 */
export function greet(): string;
