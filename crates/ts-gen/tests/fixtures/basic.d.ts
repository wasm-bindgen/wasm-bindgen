// Basic test fixture covering core patterns from CF Workers types

/** Represents an error from a DOM operation. */
declare class DOMException {
  constructor(message?: string, name?: string);
  /** The error message. */
  readonly message: string;
  readonly name: string;
  readonly code: number;
}

// === var + interface merge pattern ===
/**
 * The Response interface of the Fetch API represents the response to a request.
 */
declare var Response: {
  prototype: Response;
  new(body?: BodyInit | null, init?: ResponseInit): Response;
  /** Returns a new Response with a network error. */
  error(): Response;
  redirect(url: string, status?: number): Response;
  json(any: any, maybeInit?: ResponseInit | Response): Response;
};

interface Response extends Body {
  /** Creates a clone of this response. */
  clone(): Response;
  /** The HTTP status code. */
  readonly status: number;
  readonly statusText: string;
  readonly headers: Headers;
  readonly ok: boolean;
  readonly url: string;
  readonly body: ReadableStream | null;
}

/**
 * Provides methods to read the body of a request or response.
 */
interface Body {
  readonly body: ReadableStream | null;
  readonly bodyUsed: boolean;
  /** Returns the body as an ArrayBuffer. */
  arrayBuffer(): Promise<ArrayBuffer>;
  text(): Promise<string>;
  json<T>(): Promise<T>;
  blob(): Promise<Blob>;
  formData(): Promise<FormData>;
}

/** Options for constructing a Response. */
interface ResponseInit {
  status?: number;
  statusText?: string;
  headers?: HeadersInit;
}

// === String enum ===
type QueueContentType = "text" | "bytes" | "json" | "v8";

// === Type alias ===
type BodyInit = ReadableStream | string | ArrayBuffer | Blob | URLSearchParams | FormData;
type HeadersInit = Headers | string[][] | Record<string, string>;

// === declare module ===
declare module "cloudflare:sockets" {
  function connect(address: string, options?: SocketOptions): Socket;

  interface Socket {
    close(): Promise<void>;
    readonly closed: Promise<void>;
    readonly opened: Promise<void>;
    readonly readable: ReadableStream;
    readonly writable: WritableStream;
    startTls(): Socket;
  }

  interface SocketOptions {
    secureTransport?: string;
    allowHalfOpen?: boolean;
  }
}

// === Abstract class ===
declare abstract class DurableObject {
  ctx: DurableObjectState;
  env: Record<string, unknown>;
  alarm?(): Promise<void>;
  webSocketMessage?(ws: WebSocket, message: string | ArrayBuffer): Promise<void>;
}

// === Namespace ===
declare namespace WebAssembly {
  class Module {
    constructor(bytes: ArrayBuffer);
  }
  class Instance {
    constructor(module: Module, imports?: object);
    readonly exports: Record<string, unknown>;
  }
  function compile(bytes: ArrayBuffer): Promise<Module>;
  function instantiate(module: Module, imports?: object): Promise<Instance>;
}

// === Global function ===
declare function fetch(input: RequestInfo, init?: RequestInit): Promise<Response>;
declare function atob(data: string): string;
declare function btoa(data: string): string;

// === Const ===
declare const navigator: Navigator;
declare const self: ServiceWorkerGlobalScope;
