use wasm_bindgen_test::*;

/// Test that forward_console_to_test_runner correctly wraps console methods
/// and sends messages in the expected format.
///
/// This test mocks the worker environment by:
/// 1. Saving the original console.log
/// 2. Setting up a mock self.postMessage that captures calls
/// 3. Calling forward_console_to_test_runner()
/// 4. Calling console.log and verifying postMessage was called correctly
/// 5. Restoring the original state
#[wasm_bindgen_test]
fn forward_console_to_test_runner_sends_correct_message() {
    use js_sys::{Array, Reflect};

    // Set up a mock environment and capture postMessage calls
    let result = js_sys::eval(
        r#"
        (function() {
            // Save originals
            const originalLog = console.log;
            const originalPostMessage = self.postMessage;

            // Capture postMessage calls
            let capturedMessages = [];
            self.postMessage = function(msg) {
                capturedMessages.push(msg);
            };

            // Now call the forward function (it's already been set up by the test harness,
            // but we'll call it again to test - it should be idempotent or we test fresh)
            // Actually, we need to test the actual function...

            // Reset console.log to original first so we can test the wrapping
            console.log = originalLog;

            return { capturedMessages, originalLog, originalPostMessage };
        })()
        "#,
    )
    .expect("Failed to set up mock environment");

    let captured_messages = Reflect::get(&result, &"capturedMessages".into()).unwrap();
    let original_log = Reflect::get(&result, &"originalLog".into()).unwrap();
    let original_post_message = Reflect::get(&result, &"originalPostMessage".into()).unwrap();

    // Call the forward function
    wasm_bindgen_test::forward_console_to_test_runner();

    // Now call console.log
    js_sys::eval(r#"console.log("TEST_MARKER_12345")"#).expect("Failed to call console.log");

    // Check captured messages
    let messages = Array::from(&captured_messages);
    assert!(
        messages.length() >= 1,
        "Expected at least 1 captured message, got {}",
        messages.length()
    );

    // Get the last message (in case there were others)
    let last_msg = messages.get(messages.length() - 1);
    let msg_array = Array::from(&last_msg);

    assert_eq!(
        msg_array.length(),
        2,
        "Message should have 2 elements [method, args]"
    );

    let method = msg_array.get(0);
    assert_eq!(
        method.as_string().unwrap(),
        "__wbgtest_log",
        "First element should be '__wbgtest_log'"
    );

    let args = Array::from(&msg_array.get(1));
    assert_eq!(args.length(), 1, "Args should have 1 element");
    assert_eq!(
        args.get(0).as_string().unwrap(),
        "TEST_MARKER_12345",
        "Logged message should be 'TEST_MARKER_12345'"
    );

    // Restore originals
    Reflect::set(
        &js_sys::global(),
        &"postMessage".into(),
        &original_post_message,
    )
    .ok();

    // Note: We can't easily restore console.log since it's been wrapped,
    // but that's okay for test purposes - subsequent tests will have forwarding enabled
    // which is fine since they run in the browser context anyway
    let _ = original_log; // suppress unused warning
}

/// Test that all console methods (debug, log, info, warn, error) are forwarded correctly.
#[wasm_bindgen_test]
fn forward_console_all_methods() {
    use js_sys::{Array, Reflect};

    // Set up mock environment capturing postMessage calls
    // Note: forward_console_to_test_runner may have been called by previous tests,
    // so console methods may already be wrapped. That's fine - we just verify the
    // messages contain what we expect.
    let setup = js_sys::eval(
        r#"
        (function() {
            const originalPostMessage = self.postMessage;

            let capturedMessages = [];
            self.postMessage = function(msg) {
                capturedMessages.push(msg);
            };

            return { capturedMessages, originalPostMessage };
        })()
        "#,
    )
    .expect("Failed to set up mock environment");

    let captured_messages = Reflect::get(&setup, &"capturedMessages".into()).unwrap();
    let original_post_message = Reflect::get(&setup, &"originalPostMessage".into()).unwrap();

    // Call the forward function (may be a no-op if already wrapped, that's ok)
    wasm_bindgen_test::forward_console_to_test_runner();

    // Call each console method
    js_sys::eval(
        r#"
        console.debug("DEBUG_MSG");
        console.log("LOG_MSG");
        console.info("INFO_MSG");
        console.warn("WARN_MSG");
        console.error("ERROR_MSG");
        "#,
    )
    .expect("Failed to call console methods");

    // Check captured messages - there should be at least 5, possibly more if
    // console was wrapped multiple times
    let messages = Array::from(&captured_messages);
    assert!(
        messages.length() >= 5,
        "Expected at least 5 captured messages, got {}",
        messages.length()
    );

    // Verify that we find each expected message type in the captured messages
    let expected = [
        ("__wbgtest_debug", "DEBUG_MSG"),
        ("__wbgtest_log", "LOG_MSG"),
        ("__wbgtest_info", "INFO_MSG"),
        ("__wbgtest_warn", "WARN_MSG"),
        ("__wbgtest_error", "ERROR_MSG"),
    ];

    for (expected_method, expected_arg) in expected.iter() {
        let mut found = false;
        for i in 0..messages.length() {
            let msg = messages.get(i);
            let msg_array = Array::from(&msg);
            if msg_array.length() != 2 {
                continue;
            }
            let method = match msg_array.get(0).as_string() {
                Some(m) => m,
                None => continue,
            };
            if method != *expected_method {
                continue;
            }
            let args = Array::from(&msg_array.get(1));
            if args.length() == 0 {
                continue;
            }
            let arg = match args.get(0).as_string() {
                Some(a) => a,
                None => continue,
            };
            if arg == *expected_arg {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "Expected to find message [{}, [{}]] in captured messages",
            expected_method, expected_arg
        );
    }

    // Restore postMessage
    Reflect::set(
        &js_sys::global(),
        &"postMessage".into(),
        &original_post_message,
    )
    .ok();
}
