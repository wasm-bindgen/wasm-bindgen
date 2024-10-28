/* tslint:disable */
/* eslint-disable */
export class Foo {
  free(): void;
  /**
   * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toPrimitive
   */
  [Symbol.toPrimitive](): string;
/**
 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag
 */
  readonly [Symbol.toStringTag]: string;
}
