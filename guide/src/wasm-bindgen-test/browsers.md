# Testing in Headless Browsers

## Configure via Environment Variables

By default tests run on Node.js. To target browsers you can use the `WASM_BINDGEN_USE_BROWSER` environment variable:

```sh
WASM_BINDGEN_USE_BROWSER=1 cargo test --target wasm32-unknown-unknown
```

The following configurations are available:
- `WASM_BINDGEN_USE_DEDICATED_WORKER`: for dedicated workers
- `WASM_BINDGEN_USE_SHARED_WORKER`: for shared workers
- `WASM_BINDGEN_USE_SERVICE_WORKER`: for service workers
- `WASM_BINDGEN_USE_DENO`: for Deno
- `WASM_BINDGEN_USE_NODE_EXPERIMENTAL`: for Node.js but as an ES module

## Force Configuration

Tests can also be forced to run in a certain environment by using the
`wasm_bindgen_test_configure!` macro:

```rust
use wasm_bindgen_test::wasm_bindgen_test_configure;

// Run in a browser.
wasm_bindgen_test_configure!(run_in_browser);
// Or run in a dedicated worker.
wasm_bindgen_test_configure!(run_in_dedicated_worker);
// Or run in a shared worker.
wasm_bindgen_test_configure!(run_in_shared_worker);
// Or run in a service worker.
wasm_bindgen_test_configure!(run_in_service_worker);
// Or run in Node.js but as an ES module.
wasm_bindgen_test_configure!(run_in_node_experimental);
```

Note that this will ignore any environment variable set.

## Configuring Which Browser is Used

To control which browser is used for headless testing, use the appropriate flag
with `wasm-pack test`:

* `wasm-pack test --chrome` &mdash; Run the tests in Chrome. This machine must
  have Chrome installed.

* `wasm-pack test --firefox` &mdash; Run the tests in Firefox. This machine must
  have Firefox installed.

* `wasm-pack test --safari` &mdash; Run the tests in Safari. This machine must
  have Safari installed.

If multiple browser flags are passed, the tests will be run under each browser.

## Running the Tests in the Headless Browser

Once the tests are configured to run in a headless browser, just run `wasm-pack
test` with the appropriate browser flags and `--headless`:

```bash
wasm-pack test --headless --chrome --firefox --safari
```

## Configuring Headless Browser capabilities

Either add the file `webdriver.json` to the root of your crate or ensure the environment
variable `WASM_BINDGEN_TEST_WEBDRIVER_JSON` points to one.
Each browser has own section for capabilities. For example:

```json
{
  "moz:firefoxOptions": {
    "prefs": {
      "media.navigator.streams.fake": true,
      "media.navigator.permission.disabled": true
    },
    "args": []
  },
  "goog:chromeOptions": {
    "args": [
      "--use-fake-device-for-media-stream",
      "--use-fake-ui-for-media-stream"
    ]
  }
}
```
Full list supported capabilities can be found:

* for Chrome - [here](https://peter.sh/experiments/chromium-command-line-switches/)
* for Firefox - [here](https://developer.mozilla.org/en-US/docs/Web/WebDriver/Capabilities/firefoxOptions)

Note that the `headless` argument is always enabled for both browsers.

### Debugging Headless Browser Tests

Omitting the `--headless` flag will disable headless mode, and allow you to
debug failing tests in your browser's devtools.

## Screenshots

When running browser tests in headless mode, you can capture screenshots using
the `wasm_bindgen_test::screenshot` function. This is useful for debugging
failures on CI runners where you cannot open a browser, or for saving visual
snapshots of your rendered output.

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

### Return Type

`screenshot` returns `Result<Screenshot, ScreenshotError>`. The error type has
two variants:

- **`NotSupported`** -- the test is not running under the headless test runner.
  This happens when tests run in a real browser or in Node.js.
- **`Failed(String)`** -- the runner attempted the screenshot but something went
  wrong (e.g. an I/O error writing the file).

Use `.unwrap()` or `?` when a screenshot is required and the test should fail
without one. Use `.ok()` when the test should pass regardless of whether the
screenshot succeeds, which is handy when the same test runs both in headless
mode (where screenshots work) and in a real browser or Node.js (where they do
not):

```rust
#[wasm_bindgen_test]
async fn works_everywhere() {
    wasm_bindgen_test::screenshot("optional.png").await.ok();
}
```

### Example: Capturing a Screenshot on CI

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

### Limitations

- Screenshots capture the full page as rendered by the browser at that moment.
  The output is browser- and platform-dependent (fonts, rendering engine, DPI).
- This feature does not include built-in image comparison. If you need visual
  regression testing, save the screenshots and compare them with an external tool
  suited to your platform strategy.
- Only one screenshot can be in flight at a time per test. Concurrent screenshot
  calls within a single test are not supported.

--------------------------------------------------------------------------------

## Appendix: Testing in headless browsers without `wasm-pack`

**⚠️ The recommended way to use `wasm-bindgen-test` is with `wasm-pack`, since it
will handle installing the test runner, installing a WebDriver client for your
browser, and informing `cargo` how to use the custom test runner.** However, you
can also manage those tasks yourself, if you wish.

### Configuring Which Browser is Used

If one of the following environment variables is set, then the corresponding
WebDriver and browser will be used. If none of these environment variables are
set, then the `$PATH` is searched for a suitable WebDriver implementation.

#### `GECKODRIVER=path/to/geckodriver`

Use Firefox for headless browser testing, and `geckodriver` as its
WebDriver.

The `firefox` binary must be on your `$PATH`.

[Get `geckodriver` here](https://github.com/mozilla/geckodriver/releases)

#### `CHROMEDRIVER=path/to/chromedriver`

Use Chrome for headless browser testing, and `chromedriver` as its
WebDriver.

The `chrome` binary must be on your `$PATH`.

[Get `chromedriver` here](http://chromedriver.chromium.org/downloads)

#### `SAFARIDRIVER=path/to/safaridriver`

Use Safari for headless browser testing, and `safaridriver` as its
WebDriver.

This is installed by default on Mac OS. It should be able to find your Safari
installation by default.

### Running the Tests in the Remote Headless Browser

Tests can be run on a remote webdriver. To do this, the above environment
variables must be set as URL to the remote webdriver. For example:

```
CHROMEDRIVER_REMOTE=http://remote.host/
```

### Running the Tests in the Headless Browser

Once the tests are configured to run in a headless browser and the appropriate
environment variables are set, executing the tests for headless browsers is the
same as executing them for Node.js:

```bash
cargo test --target wasm32-unknown-unknown
```

#### Debugging Headless Browser Tests

Set the `NO_HEADLESS=1` environment variable and the browser tests will not run
headless. Instead, the tests will start a local server that you can visit in
your Web browser of choices, and headless testing should not be used. You can
then use your browser's devtools to debug.
