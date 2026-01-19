use super::stats::univariate::Sample;
use super::stats::univariate::{self, mixed};
use super::stats::Distribution;

use super::benchmark::BenchmarkConfig;
use super::estimate::{
    build_change_estimates, ChangeDistributions, ChangeEstimates, ChangePointEstimates, Estimates,
};
use super::report::BenchmarkId;
use super::SavedSample;

use alloc::vec::Vec;

// Common comparison procedure
#[allow(clippy::type_complexity)]
pub(crate) fn common(
    id: &BenchmarkId,
    avg_times: &Sample<f64>,
    config: &BenchmarkConfig,
) -> Option<(
    f64,
    Distribution<f64>,
    ChangeEstimates,
    ChangeDistributions,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Estimates,
)> {
    let prev = super::baseline::read(id.desc())?;
    let SavedSample { iters, times, .. } = prev.sample;
    let base_estimates: Estimates = prev.estimates;

    let base_avg_times: Vec<f64> = iters
        .iter()
        .zip(times.iter())
        .map(|(iters, elapsed)| elapsed / iters)
        .collect();
    let base_avg_time_sample = Sample::new(&base_avg_times);

    let (t_statistic, t_distribution) = t_test(avg_times, base_avg_time_sample, config);

    let (estimates, relative_distributions) = estimates(avg_times, base_avg_time_sample, config);
    Some((
        t_statistic,
        t_distribution,
        estimates,
        relative_distributions,
        iters,
        times,
        base_avg_times.clone(),
        base_estimates,
    ))
}

// Performs a two sample t-test
fn t_test(
    avg_times: &Sample<f64>,
    base_avg_times: &Sample<f64>,
    config: &BenchmarkConfig,
) -> (f64, Distribution<f64>) {
    let nresamples = config.nresamples;

    let t_statistic = avg_times.t(base_avg_times);
    let t_distribution =
        mixed::bootstrap(avg_times, base_avg_times, nresamples, |a, b| (a.t(b),)).0;

    // HACK: Filter out non-finite numbers, which can happen sometimes when sample size is very small.
    // Downstream code doesn't like non-finite values here.
    let t_distribution = Distribution::from(
        t_distribution
            .iter()
            .filter(|a| a.is_finite())
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );

    (t_statistic, t_distribution)
}

// Estimates the relative change in the statistics of the population
fn estimates(
    avg_times: &Sample<f64>,
    base_avg_times: &Sample<f64>,
    config: &BenchmarkConfig,
) -> (ChangeEstimates, ChangeDistributions) {
    fn stats(a: &Sample<f64>, b: &Sample<f64>) -> (f64, f64) {
        (
            a.mean() / b.mean() - 1.,
            a.percentiles().median() / b.percentiles().median() - 1.,
        )
    }

    let cl = config.confidence_level;
    let nresamples = config.nresamples;

    let (dist_mean, dist_median) =
        univariate::bootstrap(avg_times, base_avg_times, nresamples, stats);

    let distributions = ChangeDistributions {
        mean: dist_mean,
        median: dist_median,
    };

    let (mean, median) = stats(avg_times, base_avg_times);
    let points = ChangePointEstimates { mean, median };

    let estimates = build_change_estimates(&distributions, &points, cl);

    (estimates, distributions)
}
