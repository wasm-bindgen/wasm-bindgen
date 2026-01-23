use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/gc.js")]
extern "C" {
    fn drop_callback();

    async fn owned_methods();
    async fn gc_builder();
    async fn gc_constructor();
    async fn no_gc_fn_argument();
    async fn no_gc_method_receiver();
}

#[wasm_bindgen]
pub struct OwnedValue {
    pub n: f64,
}

#[wasm_bindgen]
impl OwnedValue {
    #[wasm_bindgen(constructor)]
    pub fn new(n: f64) -> Self {
        Self { n }
    }

    pub fn build(n: f64) -> Self {
        Self::new(n)
    }

    #[allow(clippy::should_implement_trait)] // traits unsupported by wasm_bindgen
    pub fn add(self, other: OwnedValue) -> Self {
        Self {
            n: self.n + other.n,
        }
    }

    pub fn n(self) -> f64 {
        self.n
    }

    pub async fn borrow_and_wait(&self, promise: Promise) {
        let _ = JsFuture::from(promise).await;
    }
}

impl Drop for OwnedValue {
    fn drop(&mut self) {
        drop_callback();
    }
}

#[wasm_bindgen]
pub async fn borrow_and_wait(_: &OwnedValue, promise: Promise) {
    let _ = JsFuture::from(promise).await;
}

#[wasm_bindgen_test]
#[ignore = "flaky"]
async fn test_all() {
    owned_methods().await;
    gc_builder().await;
    gc_constructor().await;
    no_gc_fn_argument().await;
    no_gc_method_receiver().await;
}
