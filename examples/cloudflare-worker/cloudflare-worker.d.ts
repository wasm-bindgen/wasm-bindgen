// Minimal Cloudflare Workers types for AI image generation example.
// In production, use the full @cloudflare/workers-types package.

// === Console ===
declare namespace console {
  function log(...args: any[]): void;
  function error(...args: any[]): void;
  function warn(...args: any[]): void;
  function info(...args: any[]): void;
  function debug(...args: any[]): void;
}

declare class ReadableStream {
  readonly locked: boolean;
  cancel(): Promise<void>;
  getReader(): any;
}

declare class Headers {
  constructor(init?: HeadersInit);
  append(name: string, value: string): void;
  delete(name: string): void;
  get(name: string): string | null;
  has(name: string): boolean;
  set(name: string, value: string): void;
}

type HeadersInit = Headers | Record<string, string> | [string, string][];

declare class Request {
  constructor(input: RequestInfo, init?: RequestInit);
  readonly method: string;
  readonly url: string;
  readonly headers: Headers;
  readonly body: ReadableStream | null;
  clone(): Request;
  arrayBuffer(): Promise<ArrayBuffer>;
  text(): Promise<string>;
}

type RequestInfo = Request | string;

interface RequestInit {
  method?: string;
  headers?: HeadersInit;
  body?: string | ArrayBuffer | null;
  redirect?: string;
}

declare class Response {
  constructor(body?: string | ArrayBuffer | ReadableStream | null, init?: ResponseInit);
  static redirect(url: string, status?: number): Response;
  static json(data: any, init?: ResponseInit): Response;
  readonly status: number;
  readonly statusText: string;
  readonly ok: boolean;
  readonly headers: Headers;
  readonly body: ReadableStream | null;
  clone(): Response;
  arrayBuffer(): Promise<ArrayBuffer>;
  text(): Promise<string>;
}

interface ResponseInit {
  status?: number;
  statusText?: string;
  headers?: HeadersInit;
}

declare class ExecutionContext {
  waitUntil(promise: Promise<any>): void;
  passThroughOnException(): void;
}

interface ExportedHandler {
  fetch?(request: Request, env: any, ctx: ExecutionContext): Promise<Response>;
}

declare function fetch(input: RequestInfo, init?: RequestInit): Promise<Response>;

// === Workers AI ===

interface Ai {
  run(model: string, inputs: AiTextToImageInput): Promise<ReadableStream>;
  run(model: string, inputs: AiTextGenerationInput): Promise<AiTextGenerationOutput>;
}

interface AiTextToImageInput {
  prompt: string;
  negative_prompt?: string;
  height?: number;
  width?: number;
  image?: number[];
  image_b64?: string;
  mask?: number[];
  num_steps?: number;
  strength?: number;
  guidance?: number;
  seed?: number;
}

interface AiTextGenerationInput {
  prompt: string;
  system_prompt?: string;
  max_tokens?: number;
  temperature?: number;
}

interface AiTextGenerationOutput {
  response: string;
}

// === User environment ===

interface Env {
  AI: Ai;
}
