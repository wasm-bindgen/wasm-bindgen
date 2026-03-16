// Test fixture covering patterns missing from other fixtures.
// Each section targets a specific gap from the review's test coverage table.

// === Index signatures ===
interface StringMap {
  [key: string]: string;
}

interface NumberIndexed {
  [index: number]: string;
  length: number;
}

interface MixedWithIndex {
  name: string;
  [key: string]: any;
}

// === Intersection types ===
type HasName = { name: string };
type HasAge = { age: number };
type Person = HasName & HasAge;

interface Serializable {
  serialize(): string;
}

type SerializablePerson = Person & Serializable;

// === Const enum ===
declare const enum Direction {
  Up = 0,
  Down = 1,
  Left = 2,
  Right = 3,
}

declare const enum HttpStatus {
  Ok = 200,
  NotFound = 404,
  InternalServerError = 500,
}

// === declare global {} ===
declare module "my-module" {
  export function doWork(input: string): Promise<string>;
  export interface WorkResult {
    success: boolean;
    data?: string;
  }
}

declare global {
  interface GlobalMixin {
    customMethod(): void;
  }
  function globalHelper(x: number): string;
  const GLOBAL_VERSION: string;
}

// === export default class/function ===
export default class DefaultProcessor {
  constructor(config: object);
  process(input: string): Promise<string>;
  readonly name: string;
}

export default function createProcessor(name: string): DefaultProcessor;

// === Recursive types ===
interface TreeNode {
  value: string;
  children: TreeNode[];
  parent?: TreeNode;
}

interface LinkedList {
  data: any;
  next: LinkedList | null;
}

// === Computed property names / Symbol methods ===
interface Iterable {
  [Symbol.iterator](): Iterator;
  [Symbol.toStringTag]: string;
}

interface AsyncIterable {
  [Symbol.asyncIterator](): AsyncIterator;
}

// === Overloaded standalone functions ===
declare function parse(input: string): object;
declare function parse(input: ArrayBuffer): object;
declare function parse(input: string, reviver: Function): object;

declare function stringify(value: any): string;
declare function stringify(value: any, replacer: Function): string;
declare function stringify(value: any, replacer: Function, space: number): string;

// === Edge cases ===

// Enum with negative values
declare enum SignedValues {
  NegativeOne = -1,
  Zero = 0,
  One = 1,
  Max = 2147483647,
}

// Interface extending multiple bases
interface MultiExtend extends Serializable, GlobalMixin {
  id: string;
}

// Optional methods
interface EventTarget {
  addEventListener(type: string, listener: Function): void;
  removeEventListener(type: string, listener: Function): void;
  dispatchEvent?(event: object): boolean;
}

// Static members on interface (merged var+interface)
declare var EventEmitter: {
  prototype: EventEmitter;
  new(): EventEmitter;
  listenerCount(emitter: EventEmitter, event: string): number;
};

interface EventEmitter {
  on(event: string, listener: Function): this;
  emit(event: string, ...args: any[]): boolean;
  removeAllListeners(event?: string): this;
}

// Nullable and undefined params together
interface Storage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
  clear(): void;
  readonly length: number;
}

// Deeply nested generics
interface Cache {
  get(key: string): Promise<Map<string, Array<string>> | null>;
  set(key: string, value: Map<string, Array<string>>): Promise<void>;
}

// === Dictionary builder pattern ===
// All-optional interfaces should get builder() + new()
interface FetchOptions {
  method?: string;
  headers?: Headers | Record<string, string>;
  body?: string | ArrayBuffer | null;
  redirect?: string;
  signal?: AbortSignal;
}

// Dictionary with a single property
interface SimpleConfig {
  verbose?: boolean;
}

// Dictionary with union-typed properties (setter expansion + builder)
interface NotificationOptions {
  body?: string;
  icon?: string;
  tag?: string;
  data?: any;
}

// === Setter union expansion ===
// Setters on non-dictionary interfaces should also expand unions
interface MutableWidget {
  label: string | number;
  readonly id: string;
  callback: Function;
}
