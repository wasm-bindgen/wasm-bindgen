/* tslint:disable */
/* eslint-disable */
declare function add(a: number, b: number): number;
declare function multiply(a: number, b: number): number;
declare function concat(a: string, b: string): string;
declare function uppercase(s: string): string;
declare function divide(a: number, b: number): number;
export function regular_function(): number;
export enum Color {
  Red = 0,
  Green = 1,
  Blue = 2,
}
declare enum HttpStatus {
  Ok = 200,
  NotFound = 404,
  ServerError = 500,
}
declare enum Status {
  Pending = 0,
  Active = 1,
  Complete = 2,
}
declare class Counter {
  free(): void;
  [Symbol.dispose](): void;
  constructor(initial: number);
  increment(): void;
  value: number;
}
declare class Point {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
}
declare class Point3D {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
  z: number;
}
export class Rectangle {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  width: number;
  height: number;
}
declare const default1: {
  Counter: typeof Counter,
  concat: typeof concat,
};
export { default1 as default }
export declare const math: {
  add: typeof add,
  divide: typeof divide,
  multiply: typeof multiply,
};
export declare const models: {
  '3d': {
    Point3D: typeof Point3D,
  },
  Point: typeof Point,
};
export declare const types: {
  Status: typeof Status,
  http: {
    HttpStatus: typeof HttpStatus,
  },
};
export declare const utils: {
  string: {
    uppercase: typeof uppercase,
  },
};
