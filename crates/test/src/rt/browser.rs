//! Support for printing status information of a test suite in a browser.
//!
//! Currently this is quite simple, rendering the same as the console tests in
//! node.js. Output here is rendered in a `pre`, however.

use alloc::format;
use alloc::string::String;
use js_sys::Error;
use wasm_bindgen::prelude::*;

/// Implementation of `Formatter` for browsers.
pub struct Browser;

#[wasm_bindgen]
extern "C" {
    type BrowserError;
    #[wasm_bindgen(method, getter, structural)]
    fn stack(this: &BrowserError) -> JsValue;

    #[wasm_bindgen(js_name = "__wbg_test_output_writeln")]
    fn write_output_line(data: JsValue);
}

impl Browser {
    /// Creates a new instance of `Browser`, assuming that its APIs will work
    /// (requires `Node::new()` to have return `None` first).
    pub fn new() -> Browser {
        Browser {}
    }
}

impl super::Formatter for Browser {
    fn writeln(&self, line: &str) {
        write_output_line(JsValue::from(line));
    }

    fn stringify_error(&self, err: &JsValue) -> String {
        // TODO: this should be a checked cast to `Error`
        let err = Error::from(err.clone());
        let name = String::from(err.name());
        let message = String::from(err.message());
        let err = BrowserError::from(JsValue::from(err));
        let stack = err.stack();

        let header = format!("{}: {}", name, message);
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
        format!("{}\n{}", header, stack)
    }
}
