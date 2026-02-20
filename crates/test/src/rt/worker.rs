//! Support for printing status information of a test suite in a browser.
//!
//! Currently this is quite simple, rendering the same as the console tests in
//! node.js. Output here is rendered in a `pre`, however.

use alloc::format;
use alloc::string::String;
use js_sys::Error;
use wasm_bindgen::prelude::*;

/// Implementation of `Formatter` for browsers.
///
/// Routes all output to a `pre` on the page currently. Eventually this probably
/// wants to be a pretty table with colors and folding and whatnot.
pub struct Worker {}

#[wasm_bindgen]
extern "C" {
    type WorkerError;
    #[wasm_bindgen(method, getter, structural)]
    fn stack(this: &WorkerError) -> JsValue;

    #[wasm_bindgen(js_name = "__wbg_test_output_writeln")]
    fn write_output_line(data: JsValue);
}

impl Worker {
    /// Attempts to create a new formatter for web worker
    pub fn new() -> Worker {
        Worker {}
    }
}

impl super::Formatter for Worker {
    fn writeln(&self, line: &str) {
        write_output_line(JsValue::from(String::from(line)));
    }

    fn stringify_error(&self, err: &JsValue) -> String {
        // TODO: this should be a checked cast to `Error`
        let error = Error::from(err.clone());
        let name = String::from(error.name());
        let message = String::from(error.message());
        let err = WorkerError::from(err.clone());
        let stack = err.stack();

        let header = format!("{name}: {message}");
        let stack = match stack.as_string() {
            Some(stack) => stack,
            None => return header,
        };

        // If the `stack` variable contains the name/message already, this is
        // probably a chome-like error which is already rendered well, so just
        // return this info
        if stack.contains(&header) {
            return stack;
        }

        // Fallback to make sure we don't lose any info
        format!("{header}\n{stack}")
    }
}
