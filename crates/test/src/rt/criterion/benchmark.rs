use super::SamplingMode;
use core::time::Duration;

// TODO: Move the benchmark config stuff to a separate module for easier use.

/// Struct containing all of the configuration options for a benchmark.
pub struct BenchmarkConfig {
    pub confidence_level: f64,
    pub measurement_time: Duration,
    pub noise_threshold: f64,
    pub nresamples: usize,
    pub sample_size: usize,
    pub significance_level: f64,
    pub warm_up_time: Duration,
    pub sampling_mode: SamplingMode,
}
