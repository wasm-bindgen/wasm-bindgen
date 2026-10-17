//! A throw runs no destructors, so a conversion from JS that rejects one
//! argument, in Rust or in the generated JS, must not leave earlier ones
//! owned or borrowed: each
//! test either repeats a rejected call with a large owned argument and checks
//! that memory stays flat, or checks that an object borrowed by the rejected
//! call is usable and freed afterwards.

use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/arg_leaks.js")]
extern "C" {
    #[wasm_bindgen(js_name = rejectedAfterOwned)]
    fn rejected_after_owned(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedOwnedObject)]
    fn rejected_owned_object(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedReceiver)]
    fn rejected_receiver();
    #[wasm_bindgen(js_name = rejectedSetter)]
    fn rejected_setter(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedFree)]
    fn rejected_free();
    #[wasm_bindgen(js_name = rejectedClosureArg)]
    fn rejected_closure_arg(f: &Closure<dyn FnMut(String, ArgLeakEnum)>, memory: JsValue);
    #[wasm_bindgen(js_name = rejectedClosureCall)]
    fn rejected_closure_call(f: &JsValue, memory: JsValue);
    #[wasm_bindgen(js_name = rejectedOnceCall)]
    fn rejected_once_call(f: &Closure<dyn FnMut(String)>, memory: JsValue);
    #[wasm_bindgen(catch, js_name = rejectedAsync)]
    async fn rejected_async(memory: JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = rejectedInJs)]
    fn rejected_in_js(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedObjectInJs)]
    fn rejected_object_in_js();
    #[wasm_bindgen(catch, js_name = asyncBorrowsAtCall)]
    async fn async_borrows_at_call() -> Result<(), JsValue>;
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ArgLeakEnum {
    A = 0,
    B = 1,
}

#[wasm_bindgen]
pub enum ArgLeakUnion {
    Red = "red",
    Count(u32),
}

thread_local! {
    static DROPS: Cell<u32> = const { Cell::new(0) };
}

#[wasm_bindgen]
pub struct ArgLeakObject {
    value: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
}

impl Drop for ArgLeakObject {
    fn drop(&mut self) {
        DROPS.with(|d| d.set(d.get() + 1));
    }
}

#[wasm_bindgen]
impl ArgLeakObject {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ArgLeakObject {
        ArgLeakObject {
            value: 0,
            name: String::new(),
        }
    }

    pub fn set(&mut self, e: ArgLeakEnum) {
        self.value = e as u32;
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn borrowed(&self, f: &js_sys::Function) {
        f.call0(&JsValue::UNDEFINED).unwrap_throw();
    }

    pub async fn value_later(&self) -> u32 {
        self.value
    }

    pub fn consume(self, _n: u64) {}
}

#[wasm_bindgen]
pub fn arg_leaks_drops() -> u32 {
    DROPS.with(|d| d.get())
}

#[wasm_bindgen]
pub fn arg_leaks_enum(_s: String, _e: ArgLeakEnum) {}

#[wasm_bindgen]
pub fn arg_leaks_option_enum(_s: String, _e: Option<ArgLeakEnum>) {}

#[wasm_bindgen]
pub fn arg_leaks_enum_vec(_s: String, _v: Vec<ArgLeakEnum>) {}

#[wasm_bindgen]
pub fn arg_leaks_union(_s: String, _u: ArgLeakUnion) {}

#[wasm_bindgen]
pub fn arg_leaks_enum_first(_e: ArgLeakEnum, _s: String) {}

#[wasm_bindgen]
pub fn arg_leaks_object(_v: Vec<u8>, _o: ArgLeakObject) {}

#[wasm_bindgen]
pub fn arg_leaks_object_ref(_o: &ArgLeakObject, _e: ArgLeakEnum) {}

#[wasm_bindgen]
pub async fn arg_leaks_async(_s: String, _e: ArgLeakEnum) {}

#[wasm_bindgen]
pub fn arg_leaks_u64(_s: String, _n: u64) {}

#[wasm_bindgen]
pub fn arg_leaks_class_ref(_s: String, _o: &ArgLeakObject) {}

#[wasm_bindgen]
pub fn arg_leaks_bytes(_s: String, _v: Vec<u8>) {}

#[wasm_bindgen]
pub fn arg_leaks_option_u32(_s: String, _n: Option<u32>) {}

#[wasm_bindgen]
pub fn arg_leaks_values(_v: Vec<JsValue>, _o: &ArgLeakObject) {}

#[wasm_bindgen]
pub fn arg_leaks_object_u64(_o: ArgLeakObject, _n: u64) {}

#[wasm_bindgen]
pub fn arg_leaks_owned_and_ref(_o: ArgLeakObject, _r: &ArgLeakObject) {}

#[wasm_bindgen]
pub fn arg_leaks_live_count() -> u32 {
    wasm_bindgen::externref_heap_live_count()
}

#[wasm_bindgen_test]
fn rejected_argument_frees_earlier_ones() {
    rejected_after_owned(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn rejected_owned_object_is_released() {
    rejected_owned_object(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn rejected_argument_releases_receiver() {
    rejected_receiver();
}

#[wasm_bindgen_test]
fn rejected_setter_frees_value() {
    rejected_setter(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn rejected_free_releases_object() {
    rejected_free();
}

#[wasm_bindgen_test]
fn rejected_closure_argument_frees_earlier_ones() {
    let f = Closure::new(|_s: String, _e: ArgLeakEnum| {});
    rejected_closure_arg(&f, wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn dropped_closure_frees_arguments() {
    let f = Closure::<dyn FnMut(String)>::new(|_s: String| {});
    let js = f.as_ref().clone();
    drop(f);
    rejected_closure_call(&js, wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn once_closure_frees_arguments() {
    let f = Closure::once(|_s: String| {});
    rejected_once_call(&f, wasm_bindgen::memory());
}

#[wasm_bindgen_test]
async fn rejected_async_argument_rejects() {
    rejected_async(wasm_bindgen::memory()).await.unwrap();
}

#[wasm_bindgen_test]
async fn async_method_borrows_when_called() {
    async_borrows_at_call().await.unwrap();
}

#[wasm_bindgen_test]
fn rejected_in_js_frees_earlier_arguments() {
    rejected_in_js(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn rejected_in_js_releases_objects() {
    rejected_object_in_js();
}
