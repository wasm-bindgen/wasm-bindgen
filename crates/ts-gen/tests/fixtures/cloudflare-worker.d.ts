//! @ts-gen --lib-name cloudflare-worker
// Minimal TypeScript bindings for a Cloudflare Workers hello world.
// Covers: Request, Response, ResponseInit, Headers, ExportedHandler, and fetch event.

// === Headers ===
declare class Headers {
  constructor(init?: HeadersInit);
  append(name: string, value: string): void;
  delete(name: string): void;
  get(name: string): string | null;
  has(name: string): boolean;
  set(name: string, value: string): void;
  entries(): IterableIterator<[string, string]>;
  keys(): IterableIterator<string>;
  values(): IterableIterator<string>;
}

type HeadersInit = Headers | Record<string, string> | [string, string][];

// === Request ===
declare class Request {
  constructor(input: RequestInfo, init?: RequestInit);
  readonly method: string;
  readonly url: string;
  readonly headers: Headers;
  readonly body: ReadableStream | null;
  readonly bodyUsed: boolean;
  readonly redirect: string;
  readonly signal: AbortSignal;
  clone(): Request;
  arrayBuffer(): Promise<ArrayBuffer>;
  text(): Promise<string>;
  json<T>(): Promise<T>;
  blob(): Promise<Blob>;
  formData(): Promise<FormData>;
}

type RequestInfo = Request | string;

interface RequestInit {
  method?: string;
  headers?: HeadersInit;
  body?: BodyInit | null;
  redirect?: string;
  signal?: AbortSignal;
}

type BodyInit = ReadableStream | string | ArrayBuffer | Blob | URLSearchParams | FormData;

// === Response ===
declare class Response {
  constructor(body?: BodyInit | null, init?: ResponseInit);
  static redirect(url: string, status?: number): Response;
  static json(data: any, init?: ResponseInit): Response;
  readonly status: number;
  readonly statusText: string;
  readonly ok: boolean;
  readonly headers: Headers;
  readonly body: ReadableStream | null;
  readonly bodyUsed: boolean;
  readonly url: string;
  clone(): Response;
  arrayBuffer(): Promise<ArrayBuffer>;
  text(): Promise<string>;
  json<T>(): Promise<T>;
  blob(): Promise<Blob>;
  formData(): Promise<FormData>;
}

interface ResponseInit {
  status?: number;
  statusText?: string;
  headers?: HeadersInit;
}

// === Execution Context ===
declare class ExecutionContext {
  waitUntil(promise: Promise<any>): void;
  passThroughOnException(): void;
}

// === Environment (user-defined, but we provide a minimal shape) ===
interface Env {
  [key: string]: any;
}

// === ExportedHandler (the default export shape) ===
interface ExportedHandler<E = Env> {
  fetch?(request: Request, env: E, ctx: ExecutionContext): Promise<Response>;
  scheduled?(controller: ScheduledController, env: E, ctx: ExecutionContext): Promise<void>;
}

interface ScheduledController {
  readonly scheduledTime: number;
  readonly cron: string;
  noRetry(): void;
}

// === Global fetch ===
declare function fetch(input: RequestInfo, init?: RequestInit): Promise<Response>;
