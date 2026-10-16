use futures_channel::oneshot;
use std::future::{poll_fn, Future};
use std::task::Poll;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::{wasm_bindgen_bench, Criterion};

/// Completes after waking its own task `n` times, so every poll but the last
/// goes back through the executor's queue.
fn yield_times(mut n: u32) -> impl Future<Output = ()> {
    poll_fn(move |cx| {
        if n == 0 {
            return Poll::Ready(());
        }
        n -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    })
}

/// Spawns `tasks` tasks that each yield `yields` times, and waits for all of
/// them. All of the tasks are woken within the same ticks, so this measures
/// the executor's per-wake scheduling cost.
async fn spawn_and_yield(tasks: usize, yields: u32) {
    let mut done = Vec::with_capacity(tasks);
    for _ in 0..tasks {
        let (tx, rx) = oneshot::channel();
        spawn_local(async move {
            yield_times(yields).await;
            let _ = tx.send(());
        });
        done.push(rx);
    }
    for rx in done {
        rx.await.unwrap();
    }
}

#[wasm_bindgen_bench]
async fn bench_executor(c: &mut Criterion) {
    c.bench_async_function("bench_spawn_local_1_task_yield_100", |b| {
        Box::pin(b.iter_future(|| spawn_and_yield(1, 100)))
    })
    .await;
    c.bench_async_function("bench_spawn_local_100_tasks_yield_10", |b| {
        Box::pin(b.iter_future(|| spawn_and_yield(100, 10)))
    })
    .await;
}
