//! Promise combinators that delegate to JavaScript's `Promise.all`, `Promise.race`, etc.
//!
//! These provide true concurrent I/O by using the JavaScript event loop rather
//! than cooperative Rust polling. Use these instead of `futures_util::future::join_all`
//! when working with JS-backed async operations (fetch, KV, D1, R2, etc).

use super::future_to_promise_typed;
use crate::*;
use core::future::Future;
use wasm_bindgen::JsGeneric;

/// Trait for types that can be converted into a JavaScript `Promise<T>`.
///
/// Implemented for [`Promise<T>`] (identity conversion) and for Rust `Future`s
/// with output `Result<T, JsValue>` (via [`future_to_promise_typed`]).
pub trait IntoPromise {
    /// The type this promise resolves to.
    type Output: JsGeneric;

    /// Convert this value into a JavaScript [`Promise`].
    fn into_promise(self) -> Promise<Self::Output>;
}

impl<T: JsGeneric> IntoPromise for Promise<T> {
    type Output = T;
    fn into_promise(self) -> Promise<T> {
        self
    }
}

impl<F, T> IntoPromise for F
where
    F: Future<Output = Result<T, JsValue>> + 'static,
    T: JsGeneric + Promising + FromWasmAbi,
    <T as Promising>::Resolution: JsGeneric,
{
    type Output = <T as Promising>::Resolution;
    fn into_promise(self) -> Promise<<T as Promising>::Resolution> {
        future_to_promise_typed(self)
    }
}

/// Awaits multiple JavaScript `Promise`s concurrently using `Promise.all`.
///
/// Unlike `futures_util::future::join_all`, which polls futures cooperatively
/// within the Rust executor and cannot yield to the JavaScript event loop
/// between individual completions, this function delegates concurrency to
/// `Promise.all` on the JavaScript side. This enables true concurrent I/O for
/// JS-backed operations such as `fetch`, KV, D1, R2, etc.
///
/// All promises must resolve to the same type `T`. For heterogeneous promise
/// types, use the [`join!`] macro instead.
///
/// Accepts any iterator of items that implement [`IntoPromise`], which includes
/// both [`Promise<T>`] values and Rust `Future`s whose output is
/// `Result<T, JsValue>`.
///
/// # Example
///
/// ```ignore
/// use js_sys::futures::join_all;
///
/// let promises: Vec<Promise> = (0..10)
///     .map(|_| worker.fetch_with_str_and_init(&url, &init))
///     .collect();
/// let results: Array = join_all(promises).await?;
/// ```
///
/// # Errors
///
/// Rejects with the value of the first promise that rejects, mirroring the
/// behavior of `Promise.all`.
pub async fn join_all<I: IntoIterator>(
    promises: I,
) -> Result<Array<<I::Item as IntoPromise>::Output>, JsValue>
where
    I::Item: IntoPromise,
{
    let array = Array::new_typed();
    for p in promises {
        array.push(&p.into_promise());
    }
    Promise::all_iterable(&array).await
}

/// Awaits multiple JavaScript `Promise`s concurrently using `Promise.allSettled`.
///
/// Unlike [`join_all`], this never rejects early. It waits for every promise to
/// either fulfill or reject, returning an `Array<PromiseState<T>>` where each
/// element can be inspected via `.is_fulfilled()`, `.get_value()`, and
/// `.get_reason()`.
///
/// For heterogeneous promise types, use the [`all_settled!`] macro instead.
///
/// # Example
///
/// ```ignore
/// use js_sys::futures::all_settled;
///
/// let results = all_settled(promises).await?;
/// for state in results.iter() {
///     if state.is_fulfilled() {
///         let value = state.get_value().unwrap();
///     }
/// }
/// ```
pub async fn all_settled<I: IntoIterator>(
    promises: I,
) -> Result<Array<PromiseState<<I::Item as IntoPromise>::Output>>, JsValue>
where
    I::Item: IntoPromise,
{
    let array = Array::new_typed();
    for p in promises {
        array.push(&p.into_promise());
    }
    Promise::all_settled_iterable(&array).await
}

/// Returns the result of the first `Promise` to settle (fulfill or reject),
/// using `Promise.race`.
///
/// This is the JS-native equivalent of `futures_util::future::select`. All
/// promises must resolve to the same type `T`.
///
/// # Example
///
/// ```ignore
/// use js_sys::futures::race;
///
/// let first = race(promises).await?;
/// ```
///
/// # Errors
///
/// Rejects with the value of the first promise to reject, if it settles
/// before any promise fulfills.
pub async fn race<I: IntoIterator>(promises: I) -> Result<<I::Item as IntoPromise>::Output, JsValue>
where
    I::Item: IntoPromise,
    <I::Item as IntoPromise>::Output: FromWasmAbi,
{
    let array = Array::new_typed();
    for p in promises {
        array.push(&p.into_promise());
    }
    Promise::race_iterable(&array).await
}

/// Returns the result of the first `Promise` to fulfill, using `Promise.any`.
///
/// Ignores rejections unless all promises reject, in which case it rejects
/// with an `AggregateError`.
///
/// # Example
///
/// ```ignore
/// use js_sys::futures::any;
///
/// let first_success = any(promises).await?;
/// ```
///
/// # Errors
///
/// Rejects with an `AggregateError` if every promise in the iterator rejects.
pub async fn any<I: IntoIterator>(promises: I) -> Result<<I::Item as IntoPromise>::Output, JsValue>
where
    I::Item: IntoPromise,
    <I::Item as IntoPromise>::Output: FromWasmAbi,
{
    let array = Array::new_typed();
    for p in promises {
        array.push(&p.into_promise());
    }
    Promise::any_iterable(&array).await
}

/// Tuples of `Promise<T>` whose combinators produce single `Promise`s whose
/// resolution shape is fixed by the tuple arity and element types.
///
/// Implemented for every tuple arity 1..=8 of `Promise<T: JsGeneric>`. The
/// associated `Joined` / `Settled` types pin down the `ArrayTuple` shape of
/// the result, so the one `unchecked_into` needed to reinterpret the
/// `Array<JsValue>` returned by `Promise.all` / `Promise.allSettled` is
/// encapsulated inside each method — the caller sees a fully-typed
/// `Promise<ArrayTuple<...>>`.
///
/// The soundness of the `unchecked_into`s here rests on `Promise.all` and
/// `Promise.allSettled` preserving input order and arity, which they do by
/// spec.
pub trait PromiseTuple {
    /// The typed `ArrayTuple` shape the joined promise resolves to.
    ///
    /// For a tuple `(Promise<T1>, Promise<T2>, ...)` this is
    /// `ArrayTuple<(T1, T2, ...)>`.
    type Joined: JsGeneric;

    /// The typed `ArrayTuple` shape the all-settled promise resolves to.
    ///
    /// For a tuple `(Promise<T1>, Promise<T2>, ...)` this is
    /// `ArrayTuple<(PromiseState<T1>, PromiseState<T2>, ...)>`.
    type Settled: JsGeneric;

    /// Join all promises in the tuple via `Promise.all`, returning a single
    /// `Promise` resolving to an `ArrayTuple` whose elements are the
    /// resolutions of the input promises in order.
    fn join_promise(self) -> Promise<Self::Joined>;

    /// Settle all promises in the tuple via `Promise.allSettled`, returning
    /// a single `Promise` resolving to an `ArrayTuple` of `PromiseState`s.
    ///
    /// Never rejects early — every input settles (fulfills or rejects) and is
    /// reflected by its `PromiseState` slot in the result tuple.
    fn all_settled_promise(self) -> Promise<Self::Settled>;
}

// One impl per arity, mirroring `ArrayTuple`'s `new1..=new8`.
//
// Each `T_i` here is already the final resolution type: `IntoPromise::Output`
// projects through `Promising::Resolution` for the future path, and the
// identity impl for `Promise<T>` has `Output = T`. So by the time a tuple
// reaches `PromiseTuple::join_promise`, every element is a `Promise<R_i>`
// where `R_i` is what the caller will actually observe.
macro_rules! impl_promise_tuple {
    ([$($T:ident)+] [$($idx:tt)+]) => {
        impl<$($T: JsGeneric),+> PromiseTuple for ($(Promise<$T>,)+) {
            type Joined = ArrayTuple<($($T,)+)>;
            type Settled = ArrayTuple<($(PromiseState<$T>,)+)>;

            fn join_promise(self) -> Promise<Self::Joined> {
                // Build the heterogeneous ArrayTuple of promises. The
                // existing `From` impl on `ArrayTuple` upcasts each element
                // through `JsGeneric`, which is sound by construction.
                let tuple: ArrayTuple<($(Promise<$T>,)+)> = ($(self.$idx,)+).into();
                // `Promise.all_iterable` preserves order and arity, so we
                // can reinterpret the resulting `Array<JsValue>` as the
                // intended `ArrayTuple<(T1, T2, ...)>`. This is the only
                // unchecked step.
                use wasm_bindgen::JsCast;
                Promise::all_iterable(&tuple).unchecked_into()
            }

            fn all_settled_promise(self) -> Promise<Self::Settled> {
                // Same construction as `join_promise`, but routing through
                // `Promise.allSettled` so each element's final shape is
                // `PromiseState<T_i>` rather than `T_i`.
                let tuple: ArrayTuple<($(Promise<$T>,)+)> = ($(self.$idx,)+).into();
                use wasm_bindgen::JsCast;
                Promise::all_settled_iterable(&tuple).unchecked_into()
            }
        }
    };
}

impl_promise_tuple!([T1][0]);
impl_promise_tuple!([T1 T2] [0 1]);
impl_promise_tuple!([T1 T2 T3] [0 1 2]);
impl_promise_tuple!([T1 T2 T3 T4] [0 1 2 3]);
impl_promise_tuple!([T1 T2 T3 T4 T5] [0 1 2 3 4]);
impl_promise_tuple!([T1 T2 T3 T4 T5 T6] [0 1 2 3 4 5]);
impl_promise_tuple!([T1 T2 T3 T4 T5 T6 T7] [0 1 2 3 4 5 6]);
impl_promise_tuple!([T1 T2 T3 T4 T5 T6 T7 T8] [0 1 2 3 4 5 6 7]);

/// Awaits multiple JavaScript `Promise`s of different types concurrently using
/// `Promise.all`, returning a `Promise<ArrayTuple<(T1, T2, ...)>>`.
///
/// This is the heterogeneous counterpart to [`join_all`]. Each argument may be
/// either a `Promise<T>` or a Rust `Future<Output = Result<T, JsValue>>`; both
/// are lifted to `Promise<T>` via [`IntoPromise`]. The result can be `.await`ed
/// and destructured via `.into_parts()`.
///
/// # Example
///
/// ```ignore
/// use js_sys::join;
///
/// let results = join!(
///     fetch_promise,        // Promise<Response>
///     array_buffer_promise, // Promise<ArrayBuffer>
/// ).await?;
/// let (response, buffer) = results.into_parts();
/// ```
///
/// # Errors
///
/// Returns `Err(JsValue)` if any promise rejects, with the value of the first
/// rejection.
#[macro_export]
macro_rules! join {
    ($($expr:expr),+ $(,)?) => {{
        // Lift every argument to `Promise<T_i>` via `IntoPromise`, then hand
        // the tuple to `PromiseTuple::join_promise`, which names the exact
        // `ArrayTuple<(T_1::Resolution, T_2::Resolution, ...)>` result type
        // via its `Joined` associated type — no inference on the caller,
        // no unchecked casts leaking to the call site.
        $crate::futures::PromiseTuple::join_promise((
            $( $crate::futures::IntoPromise::into_promise($expr), )+
        ))
    }};
}

/// Awaits multiple JavaScript `Promise`s of different types concurrently using
/// `Promise.allSettled`, returning a
/// `Promise<ArrayTuple<(PromiseState<T1>, PromiseState<T2>, ...)>>`.
///
/// Heterogeneous counterpart to [`all_settled`]. Each argument may be either a
/// `Promise<T>` or a Rust `Future<Output = Result<T, JsValue>>`; both are
/// lifted to `Promise<T>` via [`IntoPromise`]. Unlike [`join!`], this never
/// rejects early — every input settles (fulfills or rejects) and is reflected
/// by its `PromiseState` slot.
///
/// # Example
///
/// ```ignore
/// use js_sys::all_settled;
///
/// let results = all_settled!(
///     fetch_promise,        // Promise<Response>
///     array_buffer_promise, // Promise<ArrayBuffer>
/// ).await?;
/// let (response_state, buffer_state) = results.into_parts();
/// ```
#[macro_export]
macro_rules! all_settled {
    ($($expr:expr),+ $(,)?) => {{
        // Lift every argument to `Promise<T_i>` via `IntoPromise`, then hand
        // the tuple to `PromiseTuple::all_settled_promise`, which names the
        // exact `ArrayTuple<(PromiseState<T_1>, ...)>` result type via its
        // `Settled` associated type — no unchecked casts leaking to the call
        // site.
        $crate::futures::PromiseTuple::all_settled_promise((
            $( $crate::futures::IntoPromise::into_promise($expr), )+
        ))
    }};
}
