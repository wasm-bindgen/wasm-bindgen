/* tslint:disable */
/* eslint-disable */

export class ReinitCounter {
    free(): void;
    [Symbol.dispose](): void;
    increment(): number;
    constructor(start: number, label: string);
    static wrapped(start: number): ReinitCounter;
    count: number;
}

export class ReinitParent {
    free(): void;
    [Symbol.dispose](): void;
    name(): string;
    constructor(name: string);
}

export class ReinitPlain {
    free(): void;
    [Symbol.dispose](): void;
    constructor(value: number);
    value(): number;
}

declare function __wbg_reset_state(): void;

export function reinit_read_counter(counter: ReinitCounter): number;

export function reinit_schedule(): void;

export class ReinitChild extends ReinitParent {
    free(): void;
    [Symbol.dispose](): void;
    extra(): number;
    constructor(name: string, extra: number);
}
