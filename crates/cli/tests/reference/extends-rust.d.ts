/* tslint:disable */
/* eslint-disable */

export class InheritanceParent {
    free(): void;
    [Symbol.dispose](): void;
    name(): string;
    constructor(name: string);
}

export function inheritance_borrow_parent(p: InheritanceParent): string;

declare class ns__NsChild extends ns__NsParent {
    free(): void;
    [Symbol.dispose](): void;
    constructor(label: string, note: string);
    note(): string;
}

declare class ns__NsParent {
    free(): void;
    [Symbol.dispose](): void;
    label(): string;
    constructor(label: string);
}

export let ns: {
    NsChild: typeof ns__NsChild,
    NsParent: typeof ns__NsParent,
};

export class InheritanceChild extends InheritanceParent {
    free(): void;
    [Symbol.dispose](): void;
    extra(): string;
    constructor(name: string, extra: string);
}

export class InheritanceGrandchild extends InheritanceChild {
    free(): void;
    [Symbol.dispose](): void;
    constructor(name: string, extra: string, tag: string);
    tag(): string;
}
