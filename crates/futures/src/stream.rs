//! Converting JavaScript `AsyncIterator`s to Rust `Stream`s.
//!
//! Analogous to the promise to future conversion, this module allows
//! turning objects implementing the async iterator protocol into `Stream`s
//! that produce values that can be awaited from.
//!

use crate::JsFuture;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::stream::Stream;
use js_sys::{AsyncIterator, IteratorNext};
use wasm_bindgen::prelude::*;

/// A `Stream` that yields values from an underlying `AsyncIterator`.
pub struct JsStream {
    iter: AsyncIterator,
    next: Option<JsFuture>,
    done: bool,
}

impl JsStream {
    fn next_future(&self) -> Result<JsFuture, JsValue> {
        self.iter.next().map(JsFuture::from)
    }
}

impl From<AsyncIterator> for JsStream {
    fn from(iter: AsyncIterator) -> Self {
        JsStream {
            iter,
            next: None,
            done: false,
        }
    }
}

impl Stream for JsStream {
    type Item = Result<JsValue, JsValue>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        let future = match self.next.as_mut() {
            Some(val) => val,
            None => match self.next_future() {
                Ok(val) => {
                    self.next = Some(val);
                    self.next.as_mut().unwrap()
                }
                Err(e) => {
                    self.done = true;
                    return Poll::Ready(Some(Err(e)));
                }
            },
        };

        match Pin::new(future).poll(cx) {
            Poll::Ready(res) => match res {
                Ok(iter_next) => {
                    let next = iter_next.unchecked_into::<IteratorNext>();
                    if next.done() {
                        self.done = true;
                        Poll::Ready(None)
                    } else {
                        self.next.take();
                        Poll::Ready(Some(Ok(next.value())))
                    }
                }
                Err(e) => {
                    self.done = true;
                    Poll::Ready(Some(Err(e)))
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(feature = "std")]
mod to_iter {
    use super::*;
    use crate::future_to_promise;
    use core::ops::DerefMut;
    use js_sys::{Boolean, Object, Promise, Reflect};
    use std::cell::RefCell;
    use std::rc::Rc;

    struct RefCellStreamFuture<S>(Rc<RefCell<S>>);

    impl<S> Clone for RefCellStreamFuture<S> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<S> RefCellStreamFuture<S> {
        fn new(stream: S) -> Self {
            Self(Rc::new(RefCell::new(stream)))
        }
    }

    impl<S> Future for RefCellStreamFuture<S>
    where
        S: Stream + Unpin,
    {
        type Output = Option<S::Item>;
        fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            Pin::new(self.0.borrow_mut().deref_mut()).poll_next(cx)
        }
    }

    /// converts a Rust `Stream` into a JavaScript `AsyncIterator`
    pub fn stream_to_async_iterator<S>(stream: S) -> AsyncIterator
    where
        S: Stream<Item = Result<JsValue, JsValue>> + Unpin + 'static,
    {
        let next = RefCellStreamFuture::new(stream);
        let closure = Closure::<dyn FnMut() -> Promise>::new(move || {
            let cloned = next.clone();
            future_to_promise(async move {
                match cloned.await.transpose() {
                    Ok(value) => {
                        let result = Object::new();
                        Reflect::set(&result, &"done".into(), &Boolean::from(value.is_none()))
                            .unwrap();
                        if let Some(value) = value {
                            Reflect::set(&result, &"value".into(), &value).unwrap();
                        }
                        Ok(result.into())
                    }
                    Err(err) => Err(err),
                }
            })
        });
        let out = Object::new();
        Reflect::set(&out, &"next".into(), &closure.into_js_value()).unwrap();
        out.unchecked_into()
    }
}

#[cfg(feature = "std")]
pub use to_iter::stream_to_async_iterator;
