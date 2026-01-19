use super::measurement::Measurement;
use crate::__rt::web_time::Instant;
use core::future::Future;
use core::hint::black_box;
use core::time::Duration;

// ================================== MAINTENANCE NOTE =============================================
// Any changes made to either Bencher or AsyncBencher will have to be replicated to the other!
// ================================== MAINTENANCE NOTE =============================================

/// Timer struct used to iterate a benchmarked function and measure the runtime.
///
/// This struct provides different timing loops as methods. Each timing loop provides a different
/// way to time a routine and each has advantages and disadvantages.
///
/// * If you want to do the iteration and measurement yourself (eg. passing the iteration count
///   to a separate process), use [`iter_custom`].
/// * If your routine requires no per-iteration setup and returns a value with an expensive `drop`
///   method, use [`iter_with_large_drop`].
/// * If your routine requires some per-iteration setup that shouldn't be timed, use [`iter_batched`]
///   or [`iter_batched_ref`]. See [`BatchSize`] for a discussion of batch sizes.
///   If the setup value implements `Drop` and you don't want to include the `drop` time in the
///   measurement, use [`iter_batched_ref`], otherwise use [`iter_batched`]. These methods are also
///   suitable for benchmarking routines which return a value with an expensive `drop` method,
///   but are more complex than [`iter_with_large_drop`].
/// * Otherwise, use [`iter`].
///
/// [`iter`]: Bencher::iter
/// [`iter_custom`]: Bencher::iter_custom
/// [`iter_future`]: Bencher::iter_future
/// [`iter_custom_future`]: Bencher::iter_custom_future
pub struct Bencher<'a, M: Measurement> {
    pub(crate) iterated: bool,         // Have we iterated this benchmark?
    pub(crate) iters: u64,             // Number of times to iterate this benchmark
    pub(crate) value: Duration,        // The measured value
    pub(crate) measurement: &'a M,     // Reference to the measurement object
    pub(crate) elapsed_time: Duration, // How much time did it take to perform the iteration? Used for the warmup period.
}

impl<'a, M: Measurement> Bencher<'a, M> {
    /// Times a `routine` by executing it many times and timing the total elapsed time.
    ///
    /// Prefer this timing loop when `routine` returns a value that doesn't have a destructor.
    ///
    /// # Timing model
    ///
    /// Note that the `Bencher` also times the time required to destroy the output of `routine()`.
    /// Therefore prefer this timing loop when the runtime of `mem::drop(O)` is negligible compared
    /// to the runtime of the `routine`.
    ///
    /// ```text
    /// elapsed = Instant::now + iters * (routine + mem::drop(O) + Range::next)
    /// ```
    #[inline(never)]
    pub fn iter<O, R>(&mut self, mut routine: R)
    where
        R: FnMut() -> O,
    {
        self.iterated = true;
        let start = self.measurement.start();
        for _ in 0..self.iters {
            black_box(routine());
        }
        let end = self.measurement.end(start);
        self.value = end;
        self.elapsed_time = end;
    }

    /// Times a `routine` by executing it many times and relying on `routine` to measure its own execution time.
    ///
    /// # Timing model
    /// Custom, the timing model is whatever is returned as the [`Duration`] from `routine`.
    ///
    /// # Example
    /// ```rust
    /// use wasm_bindgen_test::{Criterion, wasm_bindgen_bench, Instant};
    ///
    /// fn foo() {
    ///     // ...
    /// }
    ///
    /// #[wasm_bindgen_bench]
    /// fn bench(c: &mut Criterion) {
    ///     c.bench_function("iter", move |b| {
    ///         b.iter_custom(|iters| {
    ///             let start = Instant::now();
    ///             for _i in 0..iters {
    ///                 std::hint::black_box(foo());
    ///             }
    ///             start.elapsed()
    ///         })
    ///     });
    /// }
    /// ```
    ///
    #[inline(never)]
    pub fn iter_custom<R>(&mut self, mut routine: R)
    where
        R: FnMut(u64) -> Duration,
    {
        self.iterated = true;
        let time_start = Instant::now();
        self.value = routine(self.iters);
        self.elapsed_time = time_start.elapsed();
    }

    /// Times a `routine` by executing it many times and timing the total elapsed time.
    ///
    /// Prefer this timing loop when `routine` returns a value that doesn't have a destructor.
    ///
    /// # Timing model
    ///
    /// Note that the `Bencher` also times the time required to destroy the output of `routine()`.
    /// Therefore prefer this timing loop when the runtime of `mem::drop(O)` is negligible compared
    /// to the runtime of the `routine`.
    ///
    /// ```text
    /// elapsed = Instant::now + iters * (routine + mem::drop(O) + Range::next)
    /// ```
    #[inline(never)]
    pub async fn iter_future<O, R, Fut>(&mut self, mut routine: R)
    where
        R: FnMut() -> Fut,
        Fut: Future<Output = O>,
    {
        self.iterated = true;
        let start = self.measurement.start();
        for _ in 0..self.iters {
            black_box(routine().await);
        }
        let end = self.measurement.end(start);
        self.value = end;
        self.elapsed_time = end;
    }

    /// Times a `routine` by executing it many times and relying on `routine` to measure its own execution time.
    ///
    /// # Timing model
    /// Custom, the timing model is whatever is returned as the [`Duration`] from `routine`.
    ///
    /// # Example
    /// ```rust
    /// use wasm_bindgen_test::{Criterion, wasm_bindgen_bench, Instant};
    ///
    /// async fn foo() {
    ///     // ...
    /// }
    ///
    /// #[wasm_bindgen_bench]
    /// async fn bench(c: &mut Criterion) {
    ///     c.bench_async_function("iter", move |b| {
    ///         Box::pin(
    ///             b.iter_custom_future(async |iters| {
    ///                 let start = Instant::now();
    ///                 for _i in 0..iters {
    ///                     std::hint::black_box(foo().await);
    ///                 }
    ///                 start.elapsed()
    ///             })
    ///         )
    ///     }).await;
    /// }
    /// ```
    ///
    #[inline(never)]
    pub async fn iter_custom_future<R, Fut>(&mut self, mut routine: R)
    where
        R: FnMut(u64) -> Fut,
        Fut: Future<Output = Duration>,
    {
        self.iterated = true;
        let time_start = Instant::now();
        self.value = routine(self.iters).await;
        self.elapsed_time = time_start.elapsed();
    }
}
