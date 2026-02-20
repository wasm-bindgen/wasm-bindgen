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
pub struct Browser {
    pre: Element,
}

#[wasm_bindgen]
extern "C" {
    type HTMLDocument;
    #[wasm_bindgen(thread_local_v2, js_name = document)]
    static DOCUMENT: HTMLDocument;
    #[wasm_bindgen(method, structural)]
    fn getElementById(this: &HTMLDocument, id: &str) -> Element;

    type Element;
    #[wasm_bindgen(method, getter = textContent, structural)]
    fn text_content(this: &Element) -> String;
    #[wasm_bindgen(method, setter = textContent, structural)]
    fn set_text_content(this: &Element, text: &str);

    type BrowserError;
    #[wasm_bindgen(method, getter, structural)]
    fn stack(this: &BrowserError) -> JsValue;
}

impl Browser {
    /// Creates a new instance of `Browser`, assuming that its APIs will work
    /// (requires `Node::new()` to have return `None` first).
    pub fn new() -> Browser {
        let pre = DOCUMENT.with(|document| document.getElementById("output"));
        pre.set_text_content("");
        Browser { pre }
    }
}

impl super::Formatter for Browser {
    fn writeln(&self, line: &str) {
        let mut html = self.pre.text_content();
        html.extend(line.chars().chain(Some('\n')));
        self.pre.set_text_content(&html);
    }

    fn stringify_error(&self, err: &JsValue) -> String {
        // TODO: this should be a checked cast to `Error`
        let err = Error::from(err.clone());
        let name = String::from(err.name());
        let message = String::from(err.message());
        let err = BrowserError::from(JsValue::from(err));
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
