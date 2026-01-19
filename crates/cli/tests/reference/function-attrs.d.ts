/* tslint:disable */
/* eslint-disable */

/**
 * Description for HoldsNumber
 */
export class HoldsNumber {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Description for method_with_attr
     * @param firstArg - some number
     * @param secondArg
     * @returns returns arg1 if arg2 is true, or holding value of self if arg2 is undefined or false
     */
    method_with_attr(firstArg: number, secondArg: boolean | undefined): number;
    /**
     * Description for static_fn_with_attr
     * @param firstArg - some number
     * @param secondArg
     * @returns returns an instance of HoldsNumber, holding arg1 if arg2 is undefined and holding arg2 if not
     */
    static static_fn_with_attr(firstArg: number, secondArg: number | undefined): HoldsNumber;
    /**
     * Inner value
     */
    readonly inner: number;
}

/**
 * Description for fn_with_attr
 * @param firstArg - some number
 * @param secondArg
 * @returns returns 1 if arg2 is true, or arg1 if arg2 is undefined or false
 */
export function fn_with_attr(firstArg: number, secondArg: boolean | undefined): Promise<number>;
