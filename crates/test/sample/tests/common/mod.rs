use sample::Timeout;
use std::time::Duration;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    console_log!("DO NOT SEE ME");
}

#[wasm_bindgen_test]
async fn pass_after_2s() {
    console_log!("immediate log");
    Timeout::new(Duration::from_secs(1)).await;
    console_log!("log after 1s");
    Timeout::new(Duration::from_secs(1)).await;
    console_log!("log at end");
}

#[wasm_bindgen_test]
fn fail() {
    console_log!("helpful message, please see me");
    panic!("this is a failing test");
}

#[wasm_bindgen_test]
async fn fail_after_3s() {
    console_log!("immediate log");
    Timeout::new(Duration::from_secs(1)).await;
    console_log!("log after 1s");
    Timeout::new(Duration::from_secs(1)).await;
    console_log!("log after 2s");
    Timeout::new(Duration::from_secs(1)).await;
    panic!("end");
}

#[wasm_bindgen_test]
fn result_fail() -> Result<(), Box<dyn core::error::Error>> {
    Err("failed via Result".into())
}

#[wasm_bindgen_test]
async fn result_fail_after_1s() -> Result<(), Box<dyn core::error::Error>> {
    Timeout::new(Duration::from_secs(1)).await;
    Err("failed via Result".into())
}

#[wasm_bindgen_test]
fn panic_before_result_fail() -> Result<(), Box<dyn core::error::Error>> {
    panic!("panic before Result fail");

    #[allow(unreachable_code)]
    Err("failed via Result".into())
}

#[wasm_bindgen_test]
#[should_panic]
fn should_panic() {
    panic!()
}

#[wasm_bindgen_test]
#[should_panic = "error message"]
fn should_panic_string() {
    panic!("error message")
}

#[wasm_bindgen_test]
#[should_panic(expected = "error message")]
fn should_panic_expected() {
    panic!("error message")
}

#[wasm_bindgen_test]
#[ignore]
async fn ignore() {
    console_log!("IGNORED");
}

#[wasm_bindgen_test]
#[should_panic = "reason"]
async fn ignore_reason() {
    panic!("IGNORED WITH A REASON")
}
