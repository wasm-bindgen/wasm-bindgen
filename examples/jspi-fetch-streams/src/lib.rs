//! # JSPI + Fetch Streaming Example
//!
//! Demonstrates using JSPI to drive `ReadableStream` I/O from plain
//! (non-`async`) Rust:
//!
//! * **Response streaming** — `fetch_stream` fetches a URL and reads the
//!   response body chunk-by-chunk via a `ReadableStreamDefaultReader`.
//!   Works on Chrome, Firefox, and Safari.
//!
//! * **Stream reading** — `read_stream` accepts any JavaScript
//!   `ReadableStream` (e.g. a request body handed in from a Service Worker)
//!   and drains it from plain Rust, suspending the JSPI fiber on each
//!   `reader.read()` Promise.  Works on Chrome, Firefox, and Safari.
//!
//! * **Request body streaming** — sending a `ReadableStream` as a `fetch`
//!   request body requires `duplex: "half"` and is **Chrome-only**;
//!   Firefox and Safari do not support it.  See the JavaScript side of this
//!   example for the feature-detection pattern.
//!
//! Both exports return `[total_bytes: u32, chunk_count: u32]` as a
//! `Uint32Array`-style `Array` so the JavaScript caller can verify the
//! transfer.

use js_sys::futures::jspi::block_on_promise;
use wasm_bindgen::prelude::*;
use web_sys::{ReadableStream, ReadableStreamDefaultReader, ReadableStreamReadResult, Response};

/// Drain `stream` chunk by chunk, suspending the JSPI fiber on each
/// `reader.read()` Promise.  Returns `[total_bytes, chunk_count]`.
fn drain_stream(stream: ReadableStream) -> Result<(u32, u32), JsValue> {
    let reader: ReadableStreamDefaultReader = stream.get_reader().unchecked_into();
    let (mut total, mut chunks) = (0u32, 0u32);

    loop {
        let result: ReadableStreamReadResult =
            block_on_promise(&reader.read())?.unchecked_into();

        if result.get_done().unwrap_or(true) {
            break;
        }

        let chunk: js_sys::Uint8Array = result.get_value().unchecked_into();
        total += chunk.length();
        chunks += 1;
    }

    Ok((total, chunks))
}

fn make_result(total: u32, chunks: u32) -> js_sys::Array {
    let out = js_sys::Array::new();
    out.push(&JsValue::from(total));
    out.push(&JsValue::from(chunks));
    out
}

/// Read a JavaScript `ReadableStream` from plain Rust via JSPI.
///
/// Each `reader.read()` suspends the WASM fiber until the next chunk
/// arrives — no `async fn`, no `.await`, no event-loop blocking.
///
/// Accepts any `ReadableStream`: a `fetch` response body, a request body
/// forwarded from a Service Worker, a synthetic stream, etc.
///
/// Returns `[total_bytes, chunk_count]`.
#[wasm_bindgen(jspi)]
pub fn read_stream(stream: ReadableStream) -> Result<js_sys::Array, JsValue> {
    let (total, chunks) = drain_stream(stream)?;
    Ok(make_result(total, chunks))
}

/// Fetch `url` and stream-read the response body via JSPI.
///
/// The fiber suspends once while waiting for the response headers, then
/// once per body chunk — all from plain, non-`async` Rust.
///
/// Works on Chrome, Firefox, and Safari (response streaming is universally
/// supported).  Returns `[total_bytes, chunk_count]`.
#[wasm_bindgen(jspi)]
pub fn fetch_stream(url: String) -> Result<js_sys::Array, JsValue> {
    let response: Response = block_on_promise(
        &web_sys::window()
            .unwrap_throw()
            .fetch_with_str(&url),
    )?
    .unchecked_into();

    if !response.ok() {
        return Err(JsValue::from_str(&format!("HTTP {}", response.status())));
    }

    let body = response
        .body()
        .ok_or_else(|| JsValue::from_str("response has no body"))?;

    let (total, chunks) = drain_stream(body)?;
    Ok(make_result(total, chunks))
}
