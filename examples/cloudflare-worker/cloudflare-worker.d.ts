/*! *****************************************************************************
Copyright (c) Cloudflare. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
THIS CODE IS PROVIDED ON AN *AS IS* BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, EITHER EXPRESS OR IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED
WARRANTIES OR CONDITIONS OF TITLE, FITNESS FOR A PARTICULAR PURPOSE,
MERCHANTABLITY OR NON-INFRINGEMENT.
See the Apache Version 2.0 License for specific language governing permissions
and limitations under the License.
***************************************************************************** */
/* eslint-disable */
// noinspection JSUnusedGlobalSymbols

/**
 * The **`console`** object provides access to the debugging console.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/console)
 */
declare namespace console {
  /**
   * The **`console.log()`** static method outputs a message to the console.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/log_static)
   */
  function log(...args: any[]): void;
  /**
   * The **`console.error()`** static method outputs a message to the console at the 'error' log level.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/error_static)
   */
  function error(...args: any[]): void;
  /**
   * The **`console.warn()`** static method outputs a warning message to the console at the 'warning' log level.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/warn_static)
   */
  function warn(...args: any[]): void;
  /**
   * The **`console.info()`** static method outputs a message to the console at the 'info' log level.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/info_static)
   */
  function info(...args: any[]): void;
  /**
   * The **`console.debug()`** static method outputs a message to the console at the 'debug' log level.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/console/debug_static)
   */
  function debug(...args: any[]): void;
}

/**
 * The `ReadableStream` interface of the Streams API represents a readable stream of byte data.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream)
 */
declare class ReadableStream {
  /**
   * The **`locked`** read-only property of the ReadableStream interface returns whether or not the readable stream is locked to a reader.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/locked)
   */
  readonly locked: boolean;
  /**
   * The **`cancel()`** method of the ReadableStream interface returns a Promise that resolves when the stream is canceled.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/cancel)
   */
  cancel(): Promise<void>;
  /**
   * The **`getReader()`** method of the ReadableStream interface creates a reader and locks the stream to it.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/ReadableStream/getReader)
   */
  getReader(): any;
}

/**
 * The **`Headers`** interface of the Fetch API allows you to perform various actions on HTTP request and response headers.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers)
 */
declare class Headers {
  /**
   * The **`Headers()`** constructor creates a new Headers object.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/Headers)
   */
  constructor(init?: HeadersInit);
  /**
   * The **`append()`** method of the Headers interface appends a new value onto an existing header inside a `Headers` object, or adds the header if it does not already exist.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/append)
   */
  append(name: string, value: string): void;
  /**
   * The **`delete()`** method of the Headers interface deletes a header from the current `Headers` object.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/delete)
   */
  delete(name: string): void;
  /**
   * The **`get()`** method of the Headers interface returns a byte string of all the values of a header within a `Headers` object with a given name.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/get)
   */
  get(name: string): string | null;
  /**
   * The **`has()`** method of the Headers interface returns a boolean stating whether a `Headers` object contains a certain header.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/has)
   */
  has(name: string): boolean;
  /**
   * The **`set()`** method of the Headers interface sets a new value for an existing header inside a `Headers` object, or adds the header if it does not already exist.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Headers/set)
   */
  set(name: string, value: string): void;
}

type HeadersInit = Headers | Record<string, string> | [string, string][];

/**
 * The **`Request`** interface of the Fetch API represents a resource request.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request)
 */
declare class Request {
  /**
   * The **`Request()`** constructor creates a new Request object.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/Request)
   */
  constructor(input: RequestInfo, init?: RequestInit);
  /**
   * The **`method`** read-only property of the Request interface contains the request's method (`GET`, `POST`, etc.)
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/method)
   */
  readonly method: string;
  /**
   * The **`url`** read-only property of the Request interface contains the URL of the request.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/url)
   */
  readonly url: string;
  /**
   * The **`headers`** read-only property of the Request interface contains the Headers object associated with the request.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/headers)
   */
  readonly headers: Headers;
  /**
   * The **`body`** read-only property of the Request interface is a ReadableStream of the body contents.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/body)
   */
  readonly body: ReadableStream | null;
  /**
   * The **`clone()`** method of the Request interface creates a copy of the current `Request` object.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/clone)
   */
  clone(): Request;
  /**
   * The **`arrayBuffer()`** method of the Request interface reads the request body and returns it as a promise that resolves with an ArrayBuffer.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/arrayBuffer)
   */
  arrayBuffer(): Promise<ArrayBuffer>;
  /**
   * The **`text()`** method of the Request interface reads the request body and returns it as a promise that resolves with a string.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Request/text)
   */
  text(): Promise<string>;
}

type RequestInfo = Request | string;

interface RequestInit {
  /** A string to set request's method. */
  method?: string;
  /** A Headers object, an object literal, or an array of two-item arrays to set request's headers. */
  headers?: HeadersInit;
  /** A BodyInit object or null to set request's body. */
  body?: string | ArrayBuffer | null;
  /** A string indicating how the request will interact with the browser's cache to set request's redirect. */
  redirect?: string;
}

/**
 * The **`Response`** interface of the Fetch API represents the response to a request.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response)
 */
declare class Response {
  /**
   * The **`Response()`** constructor creates a new Response object.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/Response)
   */
  constructor(body?: string | ArrayBuffer | ReadableStream | null, init?: ResponseInit);
  /**
   * The **`Response.redirect()`** static method returns a `Response` resulting in a redirect to the specified URL.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/redirect_static)
   */
  static redirect(url: string, status?: number): Response;
  /**
   * The **`Response.json()`** static method returns a `Response` that contains the provided JSON data as body, and a `Content-Type` header which is set to `application/json`.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/json_static)
   */
  static json(data: any, init?: ResponseInit): Response;
  /**
   * The **`status`** read-only property of the Response interface contains the HTTP status codes of the response.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/status)
   */
  readonly status: number;
  /**
   * The **`statusText`** read-only property of the Response interface contains the status message corresponding to the HTTP status code in Response.status.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/statusText)
   */
  readonly statusText: string;
  /**
   * The **`ok`** read-only property of the Response interface contains a Boolean stating whether the response was successful (status in the range 200-299) or not.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/ok)
   */
  readonly ok: boolean;
  /**
   * The **`headers`** read-only property of the Response interface contains the Headers object associated with the response.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/headers)
   */
  readonly headers: Headers;
  /**
   * The **`body`** read-only property of the Response interface is a ReadableStream of the body contents.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/body)
   */
  readonly body: ReadableStream | null;
  /**
   * The **`clone()`** method of the Response interface creates a clone of a response object, identical in every way, but stored in a different variable.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/clone)
   */
  clone(): Response;
  /**
   * The **`arrayBuffer()`** method of the Response interface reads the response body and returns it as a promise that resolves with an ArrayBuffer.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/arrayBuffer)
   */
  arrayBuffer(): Promise<ArrayBuffer>;
  /**
   * The **`text()`** method of the Response interface reads the response body and returns it as a promise that resolves with a string.
   *
   * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Response/text)
   */
  text(): Promise<string>;
}

interface ResponseInit {
  /** The status code for the response. */
  status?: number;
  /** The status message associated with the status code. */
  statusText?: string;
  /** A Headers object, an object literal, or an array of two-item arrays to set response's headers. */
  headers?: HeadersInit;
}

/**
 * The ExecutionContext interface provides methods to manage the lifecycle of a Worker invocation.
 *
 * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/)
 */
declare class ExecutionContext {
  /**
   * The **`waitUntil()`** method extends the lifetime of the event. It accepts a Promise-based task which the Workers runtime will execute before the handler terminates but without blocking the response.
   *
   * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/#waituntil)
   */
  waitUntil(promise: Promise<any>): void;
  /**
   * The **`passThroughOnException()`** method prevents a runtime error response when the Worker script throws an unhandled exception. Instead, the request will be forwarded to the origin server as if the Worker did not exist.
   *
   * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/context/#passthroughonexception)
   */
  passThroughOnException(): void;
}

/**
 * The ExportedHandler interface defines the handlers that a Worker can export to respond to incoming events.
 *
 * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/handlers/)
 */
interface ExportedHandler {
  /**
   * The **`fetch()`** handler is called when a Worker receives an HTTP request. It is the main entry point for handling requests.
   *
   * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers/runtime-apis/handlers/fetch/)
   */
  fetch?(request: Request, env: any, ctx: ExecutionContext): Promise<Response>;
}

/**
 * The global **`fetch()`** method starts the process of fetching a resource from the network, returning a promise that is fulfilled once the response is available.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch)
 */
declare function fetch(input: RequestInfo, init?: RequestInit): Promise<Response>;

// AI

/**
 * The Workers AI binding allows you to run machine learning models from your Worker.
 *
 * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/)
 */
interface Ai {
  /**
   * Run a text-to-image AI model with the given inputs. Returns a ReadableStream containing the generated image data.
   *
   * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-to-image)
   */
  run(model: string, inputs: AiTextToImageInput): Promise<ReadableStream>;
  /**
   * Run a text generation AI model with the given inputs. Returns an AiTextGenerationOutput containing the generated text.
   *
   * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-generation)
   */
  run(model: string, inputs: AiTextGenerationInput): Promise<AiTextGenerationOutput>;
}

/**
 * Input parameters for text-to-image AI models.
 *
 * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-to-image)
 */
interface AiTextToImageInput {
  /** A text description of the image you want to generate. */
  prompt: string;
  /** Specify what to exclude from the generated images. */
  negative_prompt?: string;
  /** The height of the generated image in pixels. */
  height?: number;
  /** The width of the generated image in pixels. */
  width?: number;
  /** An array of integers that represent an input image for img2img. */
  image?: number[];
  /** Base64-encoded string of an input image for img2img. */
  image_b64?: string;
  /** An array of integers that represent a mask image for inpainting. */
  mask?: number[];
  /** The number of diffusion steps; higher values can improve quality but take longer. */
  num_steps?: number;
  /** How much the generated image should be similar to the input image for img2img. A value between 0 and 1. */
  strength?: number;
  /** Controls how closely the generated image should adhere to the prompt; higher values make the image more aligned with the prompt. */
  guidance?: number;
  /** Random seed for reproducibility of the image generation. */
  seed?: number;
}

/**
 * Input parameters for text generation AI models.
 *
 * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-generation)
 */
interface AiTextGenerationInput {
  /** The input text prompt for the model to generate a response. */
  prompt: string;
  /** A system-level prompt that provides context or instructions to the model. */
  system_prompt?: string;
  /** The maximum number of tokens to generate in the response. */
  max_tokens?: number;
  /** Controls the randomness of the output; higher values produce more random results. */
  temperature?: number;
}

/**
 * Output from a text generation AI model.
 *
 * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/models/#text-generation)
 */
interface AiTextGenerationOutput {
  /** The generated text response from the model. */
  response: string;
}

// USER BINDINGS

interface Env {
  /**
   * The Workers AI binding for running machine learning models.
   *
   * [Cloudflare Docs Reference](https://developers.cloudflare.com/workers-ai/configuration/bindings/)
   */
  AI: Ai;
}
