//! @ts-gen --lib-name node:url -e "node:*=node_sys::*" node:url
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use ts_gen_integration_tests::url::*;

#[wasm_bindgen_test]
fn test_parse_url() {
    let result = parse("https://example.com:8080/path?query=1#hash");
    assert_eq!(result.protocol(), Some("https:".to_string()));
    assert_eq!(result.hostname(), Some("example.com".to_string()));
    assert_eq!(result.port(), Some("8080".to_string()));
    assert_eq!(result.pathname(), Some("/path".to_string()));
    assert_eq!(result.search(), Some("?query=1".to_string()));
    assert_eq!(result.hash(), Some("#hash".to_string()));
    assert_eq!(result.href(), "https://example.com:8080/path?query=1#hash");
}

#[wasm_bindgen_test]
fn test_resolve_url() {
    let result = resolve("https://example.com/one/two/three", "four");
    assert_eq!(result, "https://example.com/one/two/four");

    let result2 = resolve("https://example.com/one/two/three", "/four");
    assert_eq!(result2, "https://example.com/four");
}

#[wasm_bindgen_test]
fn test_file_url_to_path() {
    let path = file_url_to_path("file:///home/user/file.txt");
    assert_eq!(path, "/home/user/file.txt");
}

#[wasm_bindgen_test]
fn test_path_to_file_url() {
    let url = path_to_file_url("/home/user/file.txt");
    // URL.href is accessible — the return type is URL (the WHATWG URL)
    // which extends Object, so we can access it via JsValue methods
    assert_eq!(
        js_sys::Reflect::get(&url, &JsValue::from_str("href"))
            .unwrap()
            .as_string()
            .unwrap(),
        "file:///home/user/file.txt"
    );
}

#[wasm_bindgen_test]
fn test_parse_with_query_string() {
    // parse with parseQueryString=true returns UrlWithParsedQuery
    let result = parse_with_parse_query_string("https://example.com?foo=bar", true);
    assert_eq!(result.pathname(), Some("/".to_string()));
    assert_eq!(result.hostname(), Some("example.com".to_string()));
}
