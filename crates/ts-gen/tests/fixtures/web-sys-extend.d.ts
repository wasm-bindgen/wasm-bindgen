//! @ts-gen --external Response=::web_sys::Response,Request=::web_sys::Request,RequestInit=::web_sys::RequestInit,Headers=::web_sys::Headers,ReadableStream=::web_sys::ReadableStream,WritableStream=::web_sys::WritableStream,AbortSignal=::web_sys::AbortSignal,Blob=::web_sys::Blob,FormData=::web_sys::FormData,Body=::web_sys::Body

// Extending web_sys types without redefining them.
//
// FetchOptions extends RequestInit with extra fields.
// ResponseExt extends Response with extra methods.
// A custom fetch function ties them together.
//
// All base web types are mapped to web_sys via --external flags,
// so the generated Rust code links directly against web_sys.

/** Options for fetch, extending the standard RequestInit. */
interface FetchOptions extends RequestInit {
  /** Number of times to retry on failure. */
  retries?: number;
  /** Timeout in milliseconds. */
  timeout?: number;
  /** Custom priority hint. */
  priority?: string;
}

/**
 * Extended response with additional convenience methods.
 */
interface ResponseExt extends Response {
  /** Parse the body as JSON and return a typed result. */
  jsonExt<T>(): Promise<T>;
  /** Get the response body as a Uint8Array. */
  bytes(): Promise<ArrayBuffer>;
  /** Whether the response was served from cache. */
  readonly cached: boolean;
  /** Timing info in milliseconds. */
  readonly timing: number;
}

/** Perform a fetch with extended options, returning an extended response. */
declare function fetch(input: Request | string, init?: FetchOptions): Promise<ResponseExt>;
