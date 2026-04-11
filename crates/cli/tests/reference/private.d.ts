/* tslint:disable */
/* eslint-disable */

/**
 * A hidden enum that is not exported
 */
declare enum HiddenEnum {
    Variant1 = 0,
    Variant2 = 1,
}

/**
 * A hidden struct that is not exported but can be used as an argument type
 */
declare class HiddenStruct {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    value: number;
}
export type { HiddenStruct };

/**
 * A public enum that is exported
 */
export enum PublicEnum {
    A = 0,
    B = 1,
}

/**
 * A public struct that is exported
 */
export class PublicStruct {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    value: number;
}

/**
 * Function that returns a public struct
 */
export function get_public_struct(): PublicStruct;

declare class internal__NamespacedHidden {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    data: number;
}
export type { internal__NamespacedHidden };

declare function internal__create_namespaced(): internal__NamespacedHidden;

export let internal: {
    NamespacedHidden: typeof internal__NamespacedHidden,
    create_namespaced: typeof internal__create_namespaced,
};

/**
 * Function that takes a hidden enum as an argument
 */
export function use_hidden_enum(hidden: HiddenEnum): number;

/**
 * Function that takes a hidden struct as an argument
 */
export function use_hidden_struct(hidden: HiddenStruct): number;
