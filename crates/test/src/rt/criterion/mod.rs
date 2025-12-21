//! A statistics-driven micro-benchmarking library written in Rust.
//!
//! This crate is a microbenchmarking library which aims to provide strong
//! statistical confidence in detecting and estimating the size of performance
//! improvements and regressions, while also being easy to use.
//!
//! See
//! [the user guide](https://bheisler.github.io/criterion.rs/book/index.html)
//! for examples as well as details on the measurement and analysis process,
//! and the output.
//!
//! ## Features:
//! * Collects detailed statistics, providing strong confidence that changes
//!   to performance are real, not measurement noise.
//! * Produces detailed charts, providing thorough understanding of your code's
//!   performance behavior.

#![warn(clippy::doc_markdown, missing_docs)]
#![warn(bare_trait_objects)]
#![allow(
    clippy::just_underscores_and_digits, // Used in the stats code
    clippy::transmute_ptr_to_ptr, // Used in the stats code
)]

// Needs to be declared before other modules
// in order to be usable there.
mod analysis;
mod baseline;
mod bencher;
mod benchmark;
mod compare;
mod estimate;
mod format;
mod measurement;
mod report;
mod routine;
mod stats;

use core::future::Future;
use core::pin::Pin;
use core::ptr;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use core::time::Duration;
use libm::{ceil, sqrt};
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use benchmark::BenchmarkConfig;
use measurement::WallTime;
use report::WasmReport;

pub use bencher::Bencher;
pub use measurement::Measurement;

/// The benchmark manager
///
/// `Criterion` lets you configure and execute benchmarks
///
/// Each benchmark consists of four phases:
///
/// - **Warm-up**: The routine is repeatedly executed, to let the CPU/OS/JIT/interpreter adapt to
///   the new load
/// - **Measurement**: The routine is repeatedly executed, and timing information is collected into
///   a sample
/// - **Analysis**: The sample is analyzed and distilled into meaningful statistics that get
///   reported to stdout, stored in files, and plotted
/// - **Comparison**: The current sample is compared with the sample obtained in the previous
///   benchmark.
pub struct Criterion<M: Measurement = WallTime> {
    config: BenchmarkConfig,
    report: WasmReport,
    measurement: M,
    location: Option<Location>,
}

pub(crate) struct Location {
    file: String,
    module_path: String,
}

impl Default for Criterion {
    /// Creates a benchmark manager with the following default settings:
    ///
    /// - Sample size: 100 measurements
    /// - Warm-up time: 3 s
    /// - Measurement time: 5 s
    /// - Bootstrap size: 100 000 resamples
    /// - Noise threshold: 0.01 (1%)
    /// - Confidence level: 0.95
    /// - Significance level: 0.05
    fn default() -> Criterion {
        Criterion {
            config: BenchmarkConfig {
                confidence_level: 0.95,
                measurement_time: Duration::from_secs(5),
                noise_threshold: 0.01,
                nresamples: 100_000,
                sample_size: 100,
                significance_level: 0.05,
                warm_up_time: Duration::from_secs(3),
                sampling_mode: SamplingMode::Auto,
            },
            report: WasmReport,
            measurement: WallTime,
            location: None,
        }
    }
}

impl<M: Measurement> Criterion<M> {
    /// Changes the measurement for the benchmarks run with this runner. See the
    /// [`Measurement`] trait for more details
    pub fn with_measurement<M2: Measurement>(self, m: M2) -> Criterion<M2> {
        // Can't use struct update syntax here because they're technically different types.
        Criterion {
            config: self.config,
            report: self.report,
            measurement: m,
            location: self.location,
        }
    }

    /// Configure file and module paths for use with codspeed.
    #[must_use]
    pub fn with_location(self, file: &str, module_path: &str) -> Criterion<M> {
        Criterion {
            location: Some(Location {
                file: file.into(),
                module_path: module_path.into(),
            }),
            ..self
        }
    }

    /// Changes the default size of the sample for benchmarks run with this runner.
    ///
    /// A bigger sample should yield more accurate results if paired with a sufficiently large
    /// measurement time.
    ///
    /// Sample size must be at least 10.
    ///
    /// # Panics
    ///
    /// Panics if n < 10
    #[must_use]
    pub fn sample_size(mut self, n: usize) -> Criterion<M> {
        assert!(n >= 10);

        self.config.sample_size = n;
        self
    }

    /// Changes the default warm up time for benchmarks run with this runner.
    ///
    /// # Panics
    ///
    /// Panics if the input duration is zero
    #[must_use]
    pub fn warm_up_time(mut self, dur: Duration) -> Criterion<M> {
        assert!(dur.as_nanos() > 0);

        self.config.warm_up_time = dur;
        self
    }

    ///
    /// With a longer time, the measurement will become more resilient to transitory peak loads
    /// caused by external programs
    ///
    /// **Note**: If the measurement time is too "low", Criterion will automatically increase it
    ///
    /// # Panics
    ///
    /// Panics if the input duration in zero
    /// Changes the default measurement time for benchmarks run with this runner.
    #[must_use]
    pub fn measurement_time(mut self, dur: Duration) -> Criterion<M> {
        assert!(dur.as_nanos() > 0);

        self.config.measurement_time = dur;
        self
    }

    /// Changes the default number of resamples for benchmarks run with this runner.
    ///
    /// Number of resamples to use for the
    /// [bootstrap](http://en.wikipedia.org/wiki/Bootstrapping_(statistics)#Case_resampling)
    ///
    /// A larger number of resamples reduces the random sampling errors, which are inherent to the
    /// bootstrap method, but also increases the analysis time
    ///
    /// # Panics
    ///
    /// Panics if the number of resamples is set to zero
    #[must_use]
    pub fn nresamples(mut self, n: usize) -> Criterion<M> {
        assert!(n > 0);
        if n <= 1000 {
            console_error!("\nWarning: It is not recommended to reduce nresamples below 1000.");
        }

        self.config.nresamples = n;
        self
    }

    /// Changes the default noise threshold for benchmarks run with this runner. The noise threshold
    /// is used to filter out small changes in performance, even if they are statistically
    /// significant. Sometimes benchmarking the same code twice will result in small but
    /// statistically significant differences solely because of noise. This provides a way to filter
    /// out some of these false positives at the cost of making it harder to detect small changes
    /// to the true performance of the benchmark.
    ///
    /// The default is 0.01, meaning that changes smaller than 1% will be ignored.
    ///
    /// # Panics
    ///
    /// Panics if the threshold is set to a negative value
    #[must_use]
    pub fn noise_threshold(mut self, threshold: f64) -> Criterion<M> {
        assert!(threshold >= 0.0);

        self.config.noise_threshold = threshold;
        self
    }

    /// Changes the default confidence level for benchmarks run with this runner. The confidence
    /// level is the desired probability that the true runtime lies within the estimated
    /// [confidence interval](https://en.wikipedia.org/wiki/Confidence_interval). The default is
    /// 0.95, meaning that the confidence interval should capture the true value 95% of the time.
    ///
    /// # Panics
    ///
    /// Panics if the confidence level is set to a value outside the `(0, 1)` range
    #[must_use]
    pub fn confidence_level(mut self, cl: f64) -> Criterion<M> {
        assert!(cl > 0.0 && cl < 1.0);
        if cl < 0.5 {
            console_error!(
                "\nWarning: It is not recommended to reduce confidence level below 0.5."
            );
        }

        self.config.confidence_level = cl;
        self
    }

    /// Changes the default [significance level](https://en.wikipedia.org/wiki/Statistical_significance)
    /// for benchmarks run with this runner. This is used to perform a
    /// [hypothesis test](https://en.wikipedia.org/wiki/Statistical_hypothesis_testing) to see if
    /// the measurements from this run are different from the measured performance of the last run.
    /// The significance level is the desired probability that two measurements of identical code
    /// will be considered 'different' due to noise in the measurements. The default value is 0.05,
    /// meaning that approximately 5% of identical benchmarks will register as different due to
    /// noise.
    ///
    /// This presents a trade-off. By setting the significance level closer to 0.0, you can increase
    /// the statistical robustness against noise, but it also weakens Criterion.rs' ability to
    /// detect small but real changes in the performance. By setting the significance level
    /// closer to 1.0, Criterion.rs will be more able to detect small true changes, but will also
    /// report more spurious differences.
    ///
    /// See also the noise threshold setting.
    ///
    /// # Panics
    ///
    /// Panics if the significance level is set to a value outside the `(0, 1)` range
    #[must_use]
    pub fn significance_level(mut self, sl: f64) -> Criterion<M> {
        assert!(sl > 0.0 && sl < 1.0);

        self.config.significance_level = sl;
        self
    }
}

impl<M> Criterion<M>
where
    M: Measurement + 'static,
{
    /// Benchmarks a function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_bindgen_test::{Criterion, wasm_bindgen_bench};
    ///
    /// #[wasm_bindgen_bench]
    /// fn bench(c: &mut Criterion) {
    ///     // Setup (construct data, allocate memory, etc)
    ///     c.bench_function(
    ///         "bench desc",
    ///         |b| b.iter(|| {
    ///             // Code to benchmark goes here
    ///         }),
    ///     );
    /// }
    /// ```
    pub fn bench_function<F>(&mut self, desc: &str, f: F) -> &mut Criterion<M>
    where
        F: FnMut(&mut Bencher<'_, M>),
    {
        const NOOP: RawWaker = {
            const VTABLE: RawWakerVTable = RawWakerVTable::new(
                // Cloning just returns a new no-op raw waker
                |_| NOOP,
                // `wake` does nothing
                |_| {},
                // `wake_by_ref` does nothing
                |_| {},
                // Dropping does nothing as we don't allocate anything
                |_| {},
            );
            RawWaker::new(ptr::null(), &VTABLE)
        };

        // bench_function never be pending
        fn block_on(f: impl Future<Output = ()>) {
            let waker = unsafe { Waker::from_raw(NOOP) };
            let mut ctx = Context::from_waker(&waker);
            match core::pin::pin!(f).poll(&mut ctx) {
                Poll::Ready(_) => (),
                // sync functions not be pending
                Poll::Pending => unreachable!(),
            }
        }

        let id = report::BenchmarkId::new(desc.into());
        block_on(analysis::common(
            &id,
            &mut routine::Function::new(f),
            &self.config,
            self,
        ));

        self
    }

    /// Benchmarks a future.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_bindgen_test::{Criterion, wasm_bindgen_bench};
    ///
    /// #[wasm_bindgen_bench]
    /// async fn bench(c: &mut Criterion) {
    ///     // Setup (construct data, allocate memory, etc)
    ///     c.bench_async_function(
    ///         "bench desc",
    ///         |b| {
    ///             Box::pin(
    ///                 b.iter_future(|| async {
    ///                     // Code to benchmark goes here
    ///                 })
    ///             )
    ///         }
    ///     ).await;
    /// }
    /// ```
    pub async fn bench_async_function<F>(&mut self, desc: &str, f: F) -> &mut Criterion<M>
    where
        for<'b> F: FnMut(&'b mut Bencher<'_, M>) -> Pin<Box<dyn Future<Output = ()> + 'b>>,
    {
        let id = report::BenchmarkId::new(desc.into());
        analysis::common(&id, &mut routine::AsyncFunction::new(f), &self.config, self).await;
        self
    }
}

/// Enum representing different ways of measuring the throughput of benchmarked code.
/// If the throughput setting is configured for a benchmark then the estimated throughput will
/// be reported as well as the time per iteration.
// TODO: Remove serialize/deserialize from the public API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Throughput {
    /// Measure throughput in terms of bytes/second. The value should be the number of bytes
    /// processed by one iteration of the benchmarked code. Typically, this would be the length of
    /// an input string or `&[u8]`.
    Bytes(u64),

    /// Equivalent to Bytes, but the value will be reported in terms of
    /// kilobytes (1000 bytes) per second instead of kibibytes (1024 bytes) per
    /// second, megabytes instead of mibibytes, and gigabytes instead of gibibytes.
    BytesDecimal(u64),

    /// Measure throughput in terms of elements/second. The value should be the number of elements
    /// processed by one iteration of the benchmarked code. Typically, this would be the size of a
    /// collection, but could also be the number of lines of input text or the number of values to
    /// parse.
    Elements(u64),

    /// Measure throughput in terms of bits/second. The value should be the number of bits
    /// processed by one iteration of the benchmarked code. Typically, this would be the number of
    /// bits transferred by a networking function.
    Bits(u64),
}

/// This enum allows the user to control how Criterion.rs chooses the iteration count when sampling.
/// The default is `Auto`, which will choose a method automatically based on the iteration time during
/// the warm-up phase.
#[derive(Debug, Default, Clone, Copy)]
pub enum SamplingMode {
    /// Criterion.rs should choose a sampling method automatically. This is the default, and is
    /// recommended for most users and most benchmarks.
    #[default]
    Auto,

    /// Scale the iteration count in each sample linearly. This is suitable for most benchmarks,
    /// but it tends to require many iterations which can make it very slow for very long benchmarks.
    Linear,

    /// Keep the iteration count the same for all samples. This is not recommended, as it affects
    /// the statistics that Criterion.rs can compute. However, it requires fewer iterations than
    /// the `Linear` method and therefore is more suitable for very long-running benchmarks where
    /// benchmark execution time is more of a problem and statistical precision is less important.
    Flat,
}

impl SamplingMode {
    pub(crate) fn choose_sampling_mode(
        &self,
        warmup_mean_execution_time: f64,
        sample_count: u64,
        target_time: f64,
    ) -> ActualSamplingMode {
        match self {
            SamplingMode::Linear => ActualSamplingMode::Linear,
            SamplingMode::Flat => ActualSamplingMode::Flat,
            SamplingMode::Auto => {
                // Estimate execution time with linear sampling
                let total_runs = sample_count * (sample_count + 1) / 2;
                let d = ceil(target_time / warmup_mean_execution_time / total_runs as f64) as u64;
                let expected_ns = total_runs as f64 * d as f64 * warmup_mean_execution_time;

                if expected_ns > (2.0 * target_time) {
                    ActualSamplingMode::Flat
                } else {
                    ActualSamplingMode::Linear
                }
            }
        }
    }
}

/// Enum to represent the sampling mode without Auto.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum ActualSamplingMode {
    Linear,
    Flat,
}

impl ActualSamplingMode {
    pub(crate) fn iteration_counts(
        &self,
        warmup_mean_execution_time: f64,
        sample_count: u64,
        target_time: &Duration,
    ) -> Vec<u64> {
        match self {
            ActualSamplingMode::Linear => {
                let n = sample_count;
                let met = warmup_mean_execution_time;
                let m_ns = target_time.as_nanos();
                // Solve: [d + 2*d + 3*d + ... + n*d] * met = m_ns
                let total_runs = n * (n + 1) / 2;
                let d = (ceil(m_ns as f64 / met / total_runs as f64) as u64).max(1);
                let expected_ns = total_runs as f64 * d as f64 * met;

                if d == 1 {
                    let recommended_sample_size =
                        ActualSamplingMode::recommend_linear_sample_size(m_ns as f64, met);
                    let actual_time = Duration::from_nanos(expected_ns as u64);
                    console_error!("\nWarning: Unable to complete {} samples in {:.1?}. You may wish to increase target time to {:.1?}",
                            n, target_time, actual_time);

                    if recommended_sample_size != n {
                        console_error!(
                            ", enable flat sampling, or reduce sample count to {}.",
                            recommended_sample_size
                        );
                    } else {
                        console_error!(" or enable flat sampling.");
                    }
                }

                (1..(n + 1)).map(|a| a * d).collect::<Vec<u64>>()
            }
            ActualSamplingMode::Flat => {
                let n = sample_count;
                let met = warmup_mean_execution_time;
                let m_ns = target_time.as_nanos() as f64;
                let time_per_sample = m_ns / (n as f64);
                // This is pretty simplistic; we could do something smarter to fit into the allotted time.
                let iterations_per_sample = (ceil(time_per_sample / met) as u64).max(1);

                let expected_ns = met * (iterations_per_sample * n) as f64;

                if iterations_per_sample == 1 {
                    let recommended_sample_size =
                        ActualSamplingMode::recommend_flat_sample_size(m_ns, met);
                    let actual_time = Duration::from_nanos(expected_ns as u64);
                    console_error!("\nWarning: Unable to complete {} samples in {:.1?}. You may wish to increase target time to {:.1?}",
                            n, target_time, actual_time);

                    if recommended_sample_size != n {
                        console_error!(", or reduce sample count to {}.", recommended_sample_size);
                    } else {
                        console_error!(".");
                    }
                }

                vec![iterations_per_sample; n as usize]
            }
        }
    }

    fn is_linear(&self) -> bool {
        matches!(self, ActualSamplingMode::Linear)
    }

    fn recommend_linear_sample_size(target_time: f64, met: f64) -> u64 {
        // Some math shows that n(n+1)/2 * d * met = target_time. d = 1, so it can be ignored.
        // This leaves n(n+1) = (2*target_time)/met, or n^2 + n - (2*target_time)/met = 0
        // Which can be solved with the quadratic formula. Since A and B are constant 1,
        // this simplifies to sample_size = (-1 +- sqrt(1 - 4C))/2, where C = (2*target_time)/met.
        // We don't care about the negative solution. Experimentation shows that this actually tends to
        // result in twice the desired execution time (probably because of the ceil used to calculate
        // d) so instead I use c = target_time/met.
        let c = target_time / met;
        let sample_size = (-1.0 + sqrt(4.0 * c)) / 2.0;
        let sample_size = sample_size as u64;

        // Round down to the nearest 10 to give a margin and avoid excessive precision
        let sample_size = (sample_size / 10) * 10;

        // Clamp it to be at least 10, since criterion.rs doesn't allow sample sizes smaller than 10.
        if sample_size < 10 {
            10
        } else {
            sample_size
        }
    }

    fn recommend_flat_sample_size(target_time: f64, met: f64) -> u64 {
        let sample_size = (target_time / met) as u64;

        // Round down to the nearest 10 to give a margin and avoid excessive precision
        let sample_size = (sample_size / 10) * 10;

        // Clamp it to be at least 10, since criterion.rs doesn't allow sample sizes smaller than 10.
        if sample_size < 10 {
            10
        } else {
            sample_size
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SavedSample {
    pub(crate) sampling_mode: ActualSamplingMode,
    pub(crate) iters: Vec<f64>,
    pub(crate) times: Vec<f64>,
}
