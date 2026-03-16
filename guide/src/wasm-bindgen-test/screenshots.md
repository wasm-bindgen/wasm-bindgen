# Screenshots

When running browser tests in headless mode, you can capture screenshots using
the `wasm_bindgen_test::screenshot` function. This is useful for debugging
failures on CI runners where you cannot open a browser, or for saving visual
snapshots of your rendered output.

## Basic Usage

The `screenshot` function is async and saves a PNG to a path relative to the
crate root:

```rust
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn my_screenshot_test() {
    // ... set up your DOM or canvas ...

    wasm_bindgen_test::screenshot("screenshots/my_test.png")
        .await
        .unwrap();
}
```

The file path is relative to the directory where `cargo test` is invoked (i.e.
the crate root). Intermediate directories are created automatically.

## Return Type

`screenshot` returns `Result<Screenshot, ScreenshotError>`. The error type has
two variants:

- **`NotSupported`** -- the `#__wbgtest_screenshot` DOM element is not present,
  meaning the test is not running under the headless test runner. This happens
  when tests run in a real browser or in Node.js.
- **`Failed(String)`** -- the runner attempted the screenshot but something went
  wrong (e.g. an I/O error writing the file).

### Failing on Error

Use `.unwrap()` or `?` when a screenshot is required and the test should fail
without one:

```rust
#[wasm_bindgen_test]
async fn must_screenshot() {
    wasm_bindgen_test::screenshot("required.png").await.unwrap();
}
```

### Optional Screenshots

Use `.ok()` when the test should pass regardless of whether the screenshot
succeeds. This is handy when the same test runs both in headless mode (where
screenshots work) and in a real browser or Node.js (where they do not):

```rust
#[wasm_bindgen_test]
async fn works_everywhere() {
    // Screenshot is taken in headless mode, silently skipped otherwise.
    wasm_bindgen_test::screenshot("optional.png").await.ok();
}
```

## How It Works

The screenshot feature uses a hidden DOM element (`#__wbgtest_screenshot`) as a
communication channel between the Wasm test and the headless test runner:

1. The test writes `filename:<path>` to the element.
2. The test runner polls the element, detects the request, and uses the
   WebDriver screenshot API to capture the page as a PNG.
3. The runner saves the PNG to the requested path and writes `ok` (or
   `err:<message>`) back to the element.
4. The test polls until it sees the response, then clears the element and
   returns the result.

Because this relies on WebDriver, screenshots are only available when running
under the headless test runner (e.g. `wasm-pack test --headless --chrome`).

## Example: Capturing a Screenshot on CI

A common use case is grabbing a screenshot from a headless CI runner to see what
the browser actually rendered:

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn render_and_capture() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let div = document.create_element("div").unwrap();
    div.set_text_content(Some("Hello from wasm-bindgen!"));
    body.append_child(&div).unwrap();

    wasm_bindgen_test::screenshot("ci_output/render_test.png")
        .await
        .unwrap();
}
```

Then in your CI workflow:

```yaml
- name: Run tests
  run: wasm-pack test --headless --chrome

- name: Upload screenshots
  uses: actions/upload-artifact@v6
  if: always()
  with:
    name: screenshots
    path: ci_output/
```

## Limitations

- Screenshots capture the full page as rendered by the browser at that moment.
  The output is browser- and platform-dependent (fonts, rendering engine, DPI).
- This feature does not include built-in image comparison. If you need visual
  regression testing, save the screenshots and compare them with an external tool
  suited to your platform strategy.
- Only one screenshot can be in flight at a time per test. Concurrent screenshot
  calls within a single test are not supported.
