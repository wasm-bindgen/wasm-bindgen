use super::stats::bivariate::regression::Slope;
use super::stats::bivariate::Data;
use super::stats::univariate::Sample;
use super::stats::{Distribution, Tails};

use super::benchmark::BenchmarkConfig;
use super::estimate::{
    build_estimates, ConfidenceInterval, Distributions, Estimate, Estimates, PointEstimates,
};
use super::measurement::Measurement;
use super::report::{BenchmarkId, Report};
use super::routine::Routine;
use super::{baseline, compare, Criterion, SavedSample};

use alloc::vec::Vec;

// Common analysis procedure
pub(crate) async fn common<M: Measurement>(
    id: &BenchmarkId,
    routine: &mut dyn Routine<M>,
    config: &BenchmarkConfig,
    criterion: &Criterion<M>,
) {
    criterion.report.benchmark_start(id);

    let (sampling_mode, iters, times);
    let sample = routine
        .sample(&criterion.measurement, id, config, criterion)
        .await;
    sampling_mode = sample.0;
    iters = sample.1;
    times = sample.2;

    criterion.report.analysis(id);

    if times.contains(&0.0) {
        return;
    }

    let avg_times = iters
        .iter()
        .zip(times.iter())
        .map(|(&iters, &elapsed)| elapsed / iters)
        .collect::<Vec<f64>>();
    let avg_times = Sample::new(&avg_times);
    let labeled_sample = super::stats::univariate::outliers::tukey::classify(avg_times);

    let data = Data::new(&iters, &times);
    let (mut distributions, mut estimates) = estimates(avg_times, config);
    if sampling_mode.is_linear() {
        let (distribution, slope) = regression(&data, config);

        estimates.slope = Some(slope);
        distributions.slope = Some(distribution);
    }

    let comparison = compare::common(id, avg_times, config).map(
        |(t_value, t_distribution, relative_estimates, ..)| {
            let p_value = t_distribution.p_value(t_value, &Tails::Two);
            super::report::ComparisonData {
                p_value,
                relative_estimates,
                significance_threshold: config.significance_level,
                noise_threshold: config.noise_threshold,
            }
        },
    );

    let measurement_data = super::report::MeasurementData {
        avg_times: labeled_sample,
        absolute_estimates: estimates.clone(),
        comparison,
    };

    criterion
        .report
        .measurement_complete(id, &measurement_data, criterion.measurement.formatter());

    baseline::write(
        id.desc(),
        baseline::BenchmarkBaseline {
            file: criterion.location.as_ref().map(|l| l.file.clone()),
            module_path: criterion.location.as_ref().map(|l| l.module_path.clone()),
            iters: data.x().as_ref().to_vec(),
            times: data.y().as_ref().to_vec(),
            sample: SavedSample {
                sampling_mode,
                iters: data.x().as_ref().to_vec(),
                times: data.y().as_ref().to_vec(),
            },
            estimates,
        },
    );
}

// Performs a simple linear regression on the sample
fn regression(
    data: &Data<'_, f64, f64>,
    config: &BenchmarkConfig,
) -> (Distribution<f64>, Estimate) {
    let cl = config.confidence_level;

    let distribution = data.bootstrap(config.nresamples, |d| (Slope::fit(&d).0,)).0;

    let point = Slope::fit(data);
    let (lb, ub) = distribution.confidence_interval(config.confidence_level);
    let se = distribution.std_dev(None);

    (
        distribution,
        Estimate {
            confidence_interval: ConfidenceInterval {
                confidence_level: cl,
                lower_bound: lb,
                upper_bound: ub,
            },
            point_estimate: point.0,
            standard_error: se,
        },
    )
}

// Estimates the statistics of the population from the sample
fn estimates(avg_times: &Sample<f64>, config: &BenchmarkConfig) -> (Distributions, Estimates) {
    fn stats(sample: &Sample<f64>) -> (f64, f64, f64, f64) {
        let mean = sample.mean();
        let std_dev = sample.std_dev(Some(mean));
        let median = sample.percentiles().median();
        let mad = sample.median_abs_dev(Some(median));

        (mean, std_dev, median, mad)
    }

    let cl = config.confidence_level;
    let nresamples = config.nresamples;

    let (mean, std_dev, median, mad) = stats(avg_times);
    let points = PointEstimates {
        mean,
        median,
        std_dev,
        median_abs_dev: mad,
    };

    let (dist_mean, dist_stddev, dist_median, dist_mad) = avg_times.bootstrap(nresamples, stats);

    let distributions = Distributions {
        mean: dist_mean,
        slope: None,
        median: dist_median,
        median_abs_dev: dist_mad,
        std_dev: dist_stddev,
    };

    let estimates = build_estimates(&distributions, &points, cl);

    (distributions, estimates)
}
