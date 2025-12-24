/* tslint:disable */
/* eslint-disable */

export class Foo {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly lone_getter: number | undefined;
    set lone_setter(value: number | null | undefined);
    /**
     * You will only read numbers.
     */
    get weird(): number;
    /**
     * But you must write strings.
     *
     * Yes, this is totally fine in JS.
     */
    set weird(value: string | null | undefined);
    /**
     * There can be static getters and setters too, and they can even have the
     * same name as instance getters and setters.
     */
    static get x(): boolean | undefined;
    static set x(value: boolean | null | undefined);
    get z(): number | undefined;
    set z(value: number | null | undefined);
    x: number;
    get y(): number | undefined;
    set y(value: number | null | undefined);
}
