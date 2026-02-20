use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/SharedArrayBuffer.js")]
extern "C" {
    fn is_shared_array_buffer_supported() -> bool;
}

#[wasm_bindgen_test]
fn new() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let x = SharedArrayBuffer::new(42);
    let y: JsValue = x.into();
    assert!(y.is_object());
}

#[wasm_bindgen_test]
fn byte_length() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let buf = SharedArrayBuffer::new(42);
    assert_eq!(buf.byte_length(), 42);
}

#[wasm_bindgen_test]
fn slice() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let buf = SharedArrayBuffer::new(4);
    #[cfg(not(js_sys_unstable_apis))]
    let slice = buf.slice(2);
    #[cfg(js_sys_unstable_apis)]
    let slice = buf.slice(2, 4);
    assert!(JsValue::from(slice).is_object());
}

#[wasm_bindgen_test]
fn slice_with_end() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let buf = SharedArrayBuffer::new(4);
    let slice = buf.slice_with_end(1, 2);
    assert!(JsValue::from(slice).is_object());
}

#[wasm_bindgen_test]
fn sharedarraybuffer_inheritance() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let buf = SharedArrayBuffer::new(4);
    assert!(buf.is_instance_of::<SharedArrayBuffer>());
    assert!(buf.is_instance_of::<Object>());
    let _: &Object = buf.as_ref();
}

#[wasm_bindgen_test]
fn new_with_options() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let options = ArrayBufferOptions::new(100);
    let buf = SharedArrayBuffer::new_with_options(50, &options);
    assert_eq!(buf.byte_length(), 50);
    assert_eq!(buf.max_byte_length(), 100);
}

#[wasm_bindgen_test]
fn growable() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let options = ArrayBufferOptions::new(100);
    let buf = SharedArrayBuffer::new_with_options(50, &options);
    assert!(buf.growable());

    let fixed = SharedArrayBuffer::new(50);
    assert!(!fixed.growable());
}

#[wasm_bindgen_test]
fn grow() {
    if !is_shared_array_buffer_supported() {
        return;
    }
    let options = ArrayBufferOptions::new(100);
    let buf = SharedArrayBuffer::new_with_options(50, &options);
    assert_eq!(buf.byte_length(), 50);
    buf.grow(75).unwrap();
    assert_eq!(buf.byte_length(), 75);
}
