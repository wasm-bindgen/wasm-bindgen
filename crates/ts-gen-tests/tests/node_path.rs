//! @ts-gen --lib-name node:path node:path
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use ts_gen_integration_tests::path::*;

#[wasm_bindgen_test]
fn test_normalize() {
    let result = normalize("/foo/bar//baz/asdf/quux/..");
    assert_eq!(result, "/foo/bar/baz/asdf");
}

#[wasm_bindgen_test]
fn test_join() {
    let parts: Vec<JsValue> = ["/foo", "bar", "baz/asdf", "quux", ".."]
        .iter()
        .map(|s| JsValue::from_str(s))
        .collect();
    let result = join(&parts);
    assert_eq!(result, "/foo/bar/baz/asdf");
}

#[wasm_bindgen_test]
fn test_resolve() {
    let parts: Vec<JsValue> = ["/foo/bar", "./baz"]
        .iter()
        .map(|s| JsValue::from_str(s))
        .collect();
    let result = resolve(&parts);
    assert_eq!(result, "/foo/bar/baz");
}

#[wasm_bindgen_test]
fn test_is_absolute() {
    assert!(is_absolute("/foo/bar"));
    assert!(!is_absolute("foo/bar"));
}

#[wasm_bindgen_test]
fn test_relative() {
    let result = relative("/data/orandea/test/aaa", "/data/orandea/impl/bbb");
    assert_eq!(result, "../../impl/bbb");
}

#[wasm_bindgen_test]
fn test_dirname() {
    assert_eq!(dirname("/foo/bar/baz/asdf/quux"), "/foo/bar/baz/asdf");
}

#[wasm_bindgen_test]
fn test_basename() {
    assert_eq!(basename("/foo/bar/baz/asdf/quux.html"), "quux.html");
}

#[wasm_bindgen_test]
fn test_basename_with_suffix() {
    assert_eq!(
        basename_with_suffix("/foo/bar/baz/asdf/quux.html", ".html"),
        "quux"
    );
}

#[wasm_bindgen_test]
fn test_extname() {
    assert_eq!(extname("index.html"), ".html");
    assert_eq!(extname("index."), ".");
    assert_eq!(extname("index"), "");
    assert_eq!(extname(".index"), "");
}

#[wasm_bindgen_test]
fn test_parse_and_format() {
    let parsed = parse("/home/user/dir/file.txt");
    assert_eq!(parsed.root(), "/");
    assert_eq!(parsed.dir(), "/home/user/dir");
    assert_eq!(parsed.base(), "file.txt");
    assert_eq!(parsed.ext(), ".txt");
    assert_eq!(parsed.name(), "file");
}
