#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TestArrays , typescript_type = "TestArrays")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TestArrays` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub type TestArrays;
    # [wasm_bindgen (structural , method , getter , js_class = "TestArrays" , js_name = octetArray)]
    #[doc = "Getter for the `octetArray` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/octetArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn octet_array(this: &TestArrays) -> ::js_sys::Array;
    # [wasm_bindgen (structural , method , getter , js_class = "TestArrays" , js_name = octetSequence)]
    #[doc = "Getter for the `octetSequence` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/octetSequence)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn octet_sequence(this: &TestArrays) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "TestArrays")]
    #[doc = "The `new TestArrays(..)` constructor, creating a new instance of `TestArrays`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/TestArrays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn new() -> Result<TestArrays, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = byteStrings)]
    #[doc = "The `byteStrings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/byteStrings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn byte_strings(this: &TestArrays, arg1: &str) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = f32)]
    #[doc = "The `f32()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/f32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn f32_with_f32_slice(this: &TestArrays, a: &mut [f32]) -> ::alloc::vec::Vec<f32>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = f32)]
    #[doc = "The `f32()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/f32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn f32_with_f32_array(
        this: &TestArrays,
        a: &::js_sys::Float32Array,
    ) -> ::alloc::vec::Vec<f32>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = f64)]
    #[doc = "The `f64()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/f64)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn f64_with_f64_slice(this: &TestArrays, a: &mut [f64]) -> ::alloc::vec::Vec<f64>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = f64)]
    #[doc = "The `f64()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/f64)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn f64_with_f64_array(
        this: &TestArrays,
        a: &::js_sys::Float64Array,
    ) -> ::alloc::vec::Vec<f64>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = i16)]
    #[doc = "The `i16()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/i16)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn i16_with_i16_slice(this: &TestArrays, a: &mut [i16]) -> ::alloc::vec::Vec<i16>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = i16)]
    #[doc = "The `i16()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/i16)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn i16_with_i16_array(
        this: &TestArrays,
        a: &::js_sys::Int16Array,
    ) -> ::alloc::vec::Vec<i16>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = i32)]
    #[doc = "The `i32()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/i32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn i32_with_i32_slice(this: &TestArrays, a: &mut [i32]) -> ::alloc::vec::Vec<i32>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = i32)]
    #[doc = "The `i32()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/i32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn i32_with_i32_array(
        this: &TestArrays,
        a: &::js_sys::Int32Array,
    ) -> ::alloc::vec::Vec<i32>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = i8)]
    #[doc = "The `i8()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/i8)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn i8_with_i8_slice(this: &TestArrays, a: &mut [i8]) -> ::alloc::vec::Vec<i8>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = i8)]
    #[doc = "The `i8()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/i8)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn i8_with_i8_array(this: &TestArrays, a: &::js_sys::Int8Array) -> ::alloc::vec::Vec<i8>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = strings)]
    #[doc = "The `strings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/strings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn strings(this: &TestArrays, arg1: &str) -> ::alloc::string::String;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = u16)]
    #[doc = "The `u16()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u16)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u16_with_u16_slice(this: &TestArrays, a: &mut [u16]) -> ::alloc::vec::Vec<u16>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = u16)]
    #[doc = "The `u16()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u16)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u16_with_u16_array(
        this: &TestArrays,
        a: &::js_sys::Uint16Array,
    ) -> ::alloc::vec::Vec<u16>;
    # [wasm_bindgen (catch , method , structural , js_class = "TestArrays" , js_name = u32)]
    #[doc = "The `u32()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u32_with_u32_slice(
        this: &TestArrays,
        a: &mut [u32],
    ) -> Result<::alloc::vec::Vec<u32>, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "TestArrays" , js_name = u32)]
    #[doc = "The `u32()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u32_with_u32_array(
        this: &TestArrays,
        a: &::js_sys::Uint32Array,
    ) -> Result<::alloc::vec::Vec<u32>, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = u8)]
    #[doc = "The `u8()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u8)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u8_with_u8_slice(this: &TestArrays, a: &mut [u8]) -> ::alloc::vec::Vec<u8>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = u8)]
    #[doc = "The `u8()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u8)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u8_with_u8_array(this: &TestArrays, a: &::js_sys::Uint8Array) -> ::alloc::vec::Vec<u8>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = u8Clamped)]
    #[doc = "The `u8Clamped()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u8Clamped)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u8_clamped_with_u8_clamped_slice(
        this: &TestArrays,
        a: ::wasm_bindgen::Clamped<&mut [u8]>,
    ) -> ::wasm_bindgen::Clamped<::alloc::vec::Vec<u8>>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = u8Clamped)]
    #[doc = "The `u8Clamped()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/u8Clamped)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn u8_clamped_with_u8_clamped_array(
        this: &TestArrays,
        a: &::js_sys::Uint8ClampedArray,
    ) -> ::wasm_bindgen::Clamped<::alloc::vec::Vec<u8>>;
    # [wasm_bindgen (method , structural , js_class = "TestArrays" , js_name = usvStrings)]
    #[doc = "The `usvStrings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TestArrays/usvStrings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TestArrays`*"]
    pub fn usv_strings(this: &TestArrays, arg1: &str) -> ::alloc::string::String;
}
