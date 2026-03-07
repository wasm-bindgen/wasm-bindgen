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
    fn getElementById(this: &HTMLDocument, id: &str) -> Option<Element>;

    type Element;
    #[wasm_bindgen(method, getter = textContent, structural)]
    fn text_content(this: &Element) -> String;
    #[wasm_bindgen(method, setter = textContent, structural)]
    fn set_text_content(this: &Element, text: &str);

    type BrowserError;
    #[wasm_bindgen(method, getter, structural)]
    fn stack(this: &BrowserError) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout")]
    fn set_timeout(closure: JsValue, millis: i32);
}

fn delay_promise(millis: i32) -> js_sys::Promise {
    js_sys::Promise::new_typed(&mut |resolve, _| {
        set_timeout(resolve.into(), millis);
    })
}

/// Request the test runner to take a screenshot and save it to the given path.
///
/// This works by writing the requested filename to a hidden DOM element
/// (`#__wbgtest_screenshot`). The headless test runner detects this, takes a
/// screenshot via the WebDriver protocol, saves it, and clears the element.
/// This function polls until the element is cleared, then returns.
///
/// The path is relative to the crate root (where `cargo test` is run).
///
/// # Panics
///
/// Panics if the `#__wbgtest_screenshot` element is not present in the page
/// (i.e. when not running under the headless test runner).
pub async fn screenshot(path: &str) {
    let el = DOCUMENT
        .with(|doc| doc.getElementById("__wbgtest_screenshot"))
        .expect("#__wbgtest_screenshot element not found; are you running under the headless test runner?");
    el.set_text_content(path);

    loop {
        wasm_bindgen_futures::JsFuture::from(delay_promise(50))
            .await
            .unwrap_throw();

        if el.text_content().is_empty() {
            break;
        }
    }
}

impl Browser {
    /// Creates a new instance of `Browser`, assuming that its APIs will work
    /// (requires `Node::new()` to have return `None` first).
    pub fn new() -> Browser {
        let pre = DOCUMENT
            .with(|document| document.getElementById("output"))
            .expect("#output element not found");
        // Append a newline to separate any existing content (e.g., "Loading Wasm module...")
        // from the test output. This matches the worker behavior and allows the headless
        // runner to stream output correctly.
        let mut content = pre.text_content();
        content.push('\n');
        pre.set_text_content(&content);
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
