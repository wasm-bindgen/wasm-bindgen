use js_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn new() {
    let x = ArrayBuffer::new(42);
    let y: JsValue = x.into();
    assert!(y.is_object());
}

#[wasm_bindgen_test]
fn byte_length() {
    let buf = ArrayBuffer::new(42);
    assert_eq!(buf.byte_length(), 42);
}

#[wasm_bindgen_test]
fn is_view() {
    let x = Uint8Array::new(&JsValue::from(42));
    assert!(ArrayBuffer::is_view(&JsValue::from(x)));
}

#[wasm_bindgen_test]
fn slice() {
    let buf = ArrayBuffer::new(4);
    #[cfg(not(js_sys_unstable_apis))]
    let slice = buf.slice(2);
    #[cfg(js_sys_unstable_apis)]
    let slice = buf.slice(2, 4);
    assert!(JsValue::from(slice).is_object());
}

#[wasm_bindgen_test]
fn slice_with_end() {
    let buf = ArrayBuffer::new(4);
    let slice = buf.slice_with_end(1, 2);
    assert!(JsValue::from(slice).is_object());
}

#[wasm_bindgen_test]
fn arraybuffer_inheritance() {
    let buf = ArrayBuffer::new(4);
    assert!(buf.is_instance_of::<ArrayBuffer>());
    assert!(buf.is_instance_of::<Object>());
    let _: &Object = buf.as_ref();
}

#[wasm_bindgen_test]
fn new_with_options() {
    let options = ArrayBufferOptions::new(100);
    let buf = ArrayBuffer::new_with_options(50, &options);
    assert_eq!(buf.byte_length(), 50);
    assert_eq!(buf.max_byte_length(), 100);
}

#[wasm_bindgen_test]
fn resizable() {
    let options = ArrayBufferOptions::new(100);
    let buf = ArrayBuffer::new_with_options(50, &options);
    assert!(buf.resizable());

    let fixed = ArrayBuffer::new(50);
    assert!(!fixed.resizable());
}

#[wasm_bindgen_test]
fn resize() {
    let options = ArrayBufferOptions::new(100);
    assert_eq!(options.get_max_byte_length(), 100);
    let buf = ArrayBuffer::new_with_options(50, &options);
    assert_eq!(buf.byte_length(), 50);

    buf.resize(75).unwrap();
    assert_eq!(buf.byte_length(), 75);
}

#[wasm_bindgen_test]
#[ignore = "ArrayBuffer.detached is not yet available in all environments"]
fn detached() {
    let buf = ArrayBuffer::new(10);
    assert!(!buf.detached());
}

#[wasm_bindgen_test]
#[ignore = "ArrayBuffer.transfer is not yet available in all environments"]
fn transfer() {
    let buf = ArrayBuffer::new(10);
    let transferred = buf.transfer().unwrap();
    assert!(buf.detached());
    assert_eq!(transferred.byte_length(), 10);
}

#[wasm_bindgen_test]
#[ignore = "ArrayBuffer.transfer is not yet available in all environments"]
fn transfer_with_length() {
    let buf = ArrayBuffer::new(10);
    let transferred = buf.transfer_with_length(5).unwrap();
    assert!(buf.detached());
    assert_eq!(transferred.byte_length(), 5);
}

#[wasm_bindgen_test]
#[ignore = "ArrayBuffer.transferToFixedLength is not yet available in all environments"]
fn transfer_to_fixed_length() {
    let options = ArrayBufferOptions::new(100);
    let buf = ArrayBuffer::new_with_options(10, &options);
    assert!(buf.resizable());
    let transferred = buf.transfer_to_fixed_length().unwrap();
    assert!(buf.detached());
    assert_eq!(transferred.byte_length(), 10);
    assert!(!transferred.resizable());
}

#[wasm_bindgen_test]
#[ignore = "ArrayBuffer.transferToFixedLength is not yet available in all environments"]
fn transfer_to_fixed_length_with_length() {
    let options = ArrayBufferOptions::new(100);
    let buf = ArrayBuffer::new_with_options(10, &options);
    assert!(buf.resizable());
    let transferred = buf.transfer_to_fixed_length_with_length(5).unwrap();
    assert!(buf.detached());
    assert_eq!(transferred.byte_length(), 5);
    assert!(!transferred.resizable());
}
