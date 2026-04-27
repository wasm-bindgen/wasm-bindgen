//! Apples-to-apples comparison of promise combinators.
//!
//! Both paths join the **exact same** JS `Promise`s — the only thing that
//! differs is the combinator. This isolates combinator overhead from the
//! underlying async work (which is identical on both sides: `setTimeout(0)`
//! backed promises).
//!
//! Paths compared:
//!
//! * `futures_util::future::join_all` over `Vec<JsFuture<T>>` where each
//!   `JsFuture` wraps one of the input `Promise`s. This is the
//!   Rust-executor-cooperative path the PR claims is slower.
//! * `Promise::all_iterable(&Array::from_iter_typed(iter)).await` — the
//!   canonical one-liner. The iterator items are `Promise<T>`, so
//!   `Array::from_iter_typed` infers the target `Array<Promise<T>>`
//!   annotation-free via `IntoJsGeneric`. Delegates to `Promise.all` on the
//!   JS side. No intermediate `Vec`, no `js_sys::futures` wrapper in the hot
//!   loop, no macros, no new combinator crate surface.
//!
//! Varying N exposes any scaling behaviour differences — in particular
//! `futures_util::future::join_all`'s O(N) re-poll of every child on every
//! wake, which becomes O(N^2) total poll work when wakes arrive one-by-one.

use js_sys::{futures::JsFuture, Array, Number, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

// One setTimeout(0)-backed promise per invocation. Kept in JS so the
// construction cost is equal on both sides of the comparison.
#[wasm_bindgen(module = "/benches/promise_combinators.js")]
extern "C" {
    #[wasm_bindgen(js_name = makeTimeoutPromise)]
    fn make_timeout_promise(value: f64) -> Promise<Number>;
}

fn make_timeout_promises(n: usize) -> Vec<Promise<Number>> {
    (0..n).map(|i| make_timeout_promise(i as f64)).collect()
}

async fn bench_futures_join_all(n: usize) {
    let promises = make_timeout_promises(n);
    // Wrap every promise in a `JsFuture`, then join via the Rust executor.
    let futs: Vec<JsFuture<Number>> = promises.into_iter().map(JsFuture::from).collect();
    let results = futures_util::future::join_all(futs).await;
    // Keep results live across the await so nothing is optimised away.
    std::hint::black_box(results);
}

async fn bench_promise_all(n: usize) {
    // Canonical one-liner: collect the promise-producing iterator straight
    // into a typed `Array<Promise<Number>>` via `Array::from_iter_typed`
    // (inference pins the element type through `IntoJsGeneric::JsCanon`),
    // then hand it to `Promise::all_iterable` and await. One JS-side
    // combinator, one wake, no intermediate `Vec`.
    let results = Promise::all_iterable(&Array::from_iter_typed(
        (0..n).map(|i| make_timeout_promise(i as f64)),
    ))
    .await
    .unwrap();
    std::hint::black_box(results);
}

#[wasm_bindgen_bench]
async fn bench_join_combinators(c: &mut Criterion) {
    // N = 10 matches the PR claim's workload size.
    // N = 100, 1000 expose any super-linear scaling of the Rust path.
    for &n in &[10usize, 100, 1000] {
        let label_futures = format!("futures_join_all_timeout_n{n}");
        c.bench_async_function(&label_futures, |b| {
            Box::pin(async move {
                b.iter_custom_future(move |iters| async move {
                    let start = wasm_bindgen_test::Instant::now();
                    for _ in 0..iters {
                        bench_futures_join_all(n).await;
                    }
                    start.elapsed()
                })
                .await;
            })
        })
        .await;

        let label_promise_all = format!("promise_all_timeout_n{n}");
        c.bench_async_function(&label_promise_all, |b| {
            Box::pin(async move {
                b.iter_custom_future(move |iters| async move {
                    let start = wasm_bindgen_test::Instant::now();
                    for _ in 0..iters {
                        bench_promise_all(n).await;
                    }
                    start.elapsed()
                })
                .await;
            })
        })
        .await;
    }
}
