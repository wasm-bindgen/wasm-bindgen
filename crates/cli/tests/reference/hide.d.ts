/* tslint:disable */
/* eslint-disable */
/**
 * Function that takes a hidden struct as an argument
 */
export function use_hidden_struct(hidden: HiddenStruct): number;
/**
 * Function that takes a hidden enum as an argument
 */
export function use_hidden_enum(hidden: HiddenEnum): number;
/**
 * Function that returns a public struct
 */
export function get_public_struct(): PublicStruct;
declare function create_namespaced(): NamespacedHidden;
/**
 * A hidden enum that is not exported
 */
enum HiddenEnum {
  Variant1 = 0,
  Variant2 = 1,
}
export type { HiddenEnum };
/**
 * A public enum that is exported
 */
export enum PublicEnum {
  A = 0,
  B = 1,
}
/**
 * A hidden struct that is not exported but can be used as an argument type
 */
class HiddenStruct {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  value: number;
}
export type { HiddenStruct };
class NamespacedHidden {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  data: number;
}
export type { NamespacedHidden };
/**
 * A public struct that is exported
 */
export class PublicStruct {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  value: number;
}
export declare const internal: {
  create_namespaced: typeof create_namespaced,
};
