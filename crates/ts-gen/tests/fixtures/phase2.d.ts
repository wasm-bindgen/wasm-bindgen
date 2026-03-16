// Test fixture for Phase 2 fixes

// A locally-defined interface
interface Writable {
  write(data: string): boolean;
}

// 2.1: Local alias via export specifier rename
export { Writable as WritableStream };

// Negative enum (verifies Phase 1 still works)
declare enum Priority {
  Low = -1,
  Normal = 0,
  High = 1,
}

// Regular type alias (local)
type StringOrNumber = string | number;

// 2.10: Union type alias used in function params — must resolve through alias
type BodyInit = ReadableStream | string | ArrayBuffer | Blob;

declare function send(body: BodyInit): void;

// Type alias to a concrete type — must resolve through alias
type RequestInfo = string | Request;

declare function fetch(input: RequestInfo): Promise<Response>;
