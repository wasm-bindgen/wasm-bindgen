use js_sys::Promise;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub struct Timeout {
    id: JsValue,
    inner: JsFuture,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(closure: JsValue, millis: f64) -> JsValue;

    #[wasm_bindgen(js_name = clearTimeout)]
    fn clear_timeout(id: &JsValue);
}

impl Timeout {
    pub fn new(dur: Duration) -> Timeout {
        let millis = dur
            .as_secs()
            .checked_mul(1000)
            .unwrap()
            .checked_add(dur.subsec_millis() as u64)
            .unwrap() as f64; // TODO: checked cast

        let mut id = None;
        let promise = Promise::new_typed(ImmediateClosure::new_mut_aborting(
            &mut |resolve, _reject| {
                id = Some(set_timeout(resolve.into(), millis));
            },
        ));

        Timeout {
            id: id.unwrap(),
            inner: JsFuture::from(promise),
        }
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        Pin::new(&mut self.inner).poll(cx).map(|_| ())
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        clear_timeout(&self.id);
    }
}
