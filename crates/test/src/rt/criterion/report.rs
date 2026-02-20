use super::estimate::ChangeEstimates;
use super::estimate::Estimate;
use super::estimate::Estimates;
use super::format;
use super::measurement::ValueFormatter;
use super::stats::univariate::outliers::tukey::LabeledSample;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt;
use nu_ansi_term::{Color, Style};
use serde::{Deserialize, Serialize};

pub struct ComparisonData {
    pub p_value: f64,
    pub relative_estimates: ChangeEstimates,
    pub significance_threshold: f64,
    pub noise_threshold: f64,
}

pub struct MeasurementData<'a> {
    pub avg_times: LabeledSample<'a, f64>,
    pub absolute_estimates: Estimates,
    pub comparison: Option<ComparisonData>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkId {
    desc: String,
}

impl BenchmarkId {
    pub fn new(desc: String) -> BenchmarkId {
        BenchmarkId { desc }
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }
}
impl fmt::Display for BenchmarkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.desc())
    }
}
impl fmt::Debug for BenchmarkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BenchmarkId {{ desc: \"{}\" }}", self.desc,)
    }
}

pub trait Report {
    fn benchmark_start(&self, _id: &BenchmarkId) {}
    fn warmup(&self, _id: &BenchmarkId, _warmup_ns: f64) {}
    fn analysis(&self, _id: &BenchmarkId) {}
    fn measurement_start(
        &self,
        _id: &BenchmarkId,
        _sample_count: u64,
        _estimate_ns: f64,
        _iter_count: u64,
    ) {
    }
    fn measurement_complete(
        &self,
        _id: &BenchmarkId,
        _measurements: &MeasurementData,
        _formatter: &dyn ValueFormatter,
    ) {
    }
}

pub(crate) struct WasmReport;

impl WasmReport {
    fn print(&self, s: String) {
        console_log!("{}", s);
    }

    fn with_color(&self, color: Color, s: &str) -> String {
        color.paint(s).to_string()
    }

    fn green(&self, s: &str) -> String {
        self.with_color(Color::Green, s)
    }

    fn yellow(&self, s: &str) -> String {
        self.with_color(Color::Yellow, s)
    }

    fn red(&self, s: &str) -> String {
        self.with_color(Color::Red, s)
    }

    fn bold(&self, s: String) -> String {
        Style::new().bold().paint(s).to_string()
    }

    fn faint(&self, s: String) -> String {
        Style::new().dimmed().paint(s).to_string()
    }

    pub fn outliers(&self, sample: &LabeledSample<'_, f64>) {
        let (los, lom, _, him, his) = sample.count();
        let noutliers = los + lom + him + his;
        let sample_size = sample.len();

        if noutliers == 0 {
            return;
        }

        let percent = |n: usize| 100. * n as f64 / sample_size as f64;

        console_log!(
            "{}",
            self.yellow(&format!(
                "Found {noutliers} outliers among {sample_size} measurements ({:.2}%)",
                percent(noutliers)
            ))
        );

        let print = |n, label| {
            if n != 0 {
                console_log!("  {} ({:.2}%) {}", n, percent(n), label);
            }
        };

        print(los, "low severe");
        print(lom, "low mild");
        print(him, "high mild");
        print(his, "high severe");
    }
}

impl Report for WasmReport {
    fn warmup(&self, _id: &BenchmarkId, warmup_ns: f64) {
        self.print(format!("Warming up for {}", format::time(warmup_ns)));
    }

    fn measurement_start(
        &self,
        _id: &BenchmarkId,
        sample_count: u64,
        estimate_ns: f64,
        iter_count: u64,
    ) {
        let iter_string = format::iter_count(iter_count);

        self.print(format!(
            "Collecting {sample_count} samples in estimated {} ({iter_string})",
            format::time(estimate_ns)
        ));
    }

    fn measurement_complete(
        &self,
        id: &BenchmarkId,
        meas: &MeasurementData,
        formatter: &dyn ValueFormatter,
    ) {
        let typical_estimate = &meas.absolute_estimates.typical();

        let mut id = id.desc().to_string();

        if id.len() > 23 {
            console_log!("{}", self.green(&id));
            id.clear();
        }
        let id_len = id.len();

        console_log!(
            "{}{}time:   [{} {} {}]",
            self.green(&id),
            " ".repeat(24 - id_len),
            self.faint(formatter.format_value(typical_estimate.confidence_interval.lower_bound)),
            self.bold(formatter.format_value(typical_estimate.point_estimate)),
            self.faint(formatter.format_value(typical_estimate.confidence_interval.upper_bound))
        );

        if let Some(ref comp) = meas.comparison {
            let different_mean = comp.p_value < comp.significance_threshold;
            let mean_est = &comp.relative_estimates.mean;
            let point_estimate = mean_est.point_estimate;
            let mut point_estimate_str = format::change(point_estimate, true);
            // The change in throughput is related to the change in timing. Reducing the timing by
            // 50% increases the throughput by 100%.
            let explanation_str: String;

            if !different_mean {
                explanation_str = "No change in performance detected.".to_string();
            } else {
                let comparison = compare_to_threshold(mean_est, comp.noise_threshold);
                match comparison {
                    ComparisonResult::Improved => {
                        point_estimate_str = self.green(&self.bold(point_estimate_str));
                        explanation_str = format!("Performance has {}.", self.green("improved"));
                    }
                    ComparisonResult::Regressed => {
                        point_estimate_str = self.red(&self.bold(point_estimate_str));
                        explanation_str = format!("Performance has {}.", self.red("regressed"));
                    }
                    ComparisonResult::NonSignificant => {
                        explanation_str = "Change within noise threshold.".to_string();
                    }
                }
            }

            console_log!(
                "{}change: [{} {} {}] (p = {:.2} {} {:.2})",
                " ".repeat(24),
                self.faint(format::change(
                    mean_est.confidence_interval.lower_bound,
                    true
                )),
                point_estimate_str,
                self.faint(format::change(
                    mean_est.confidence_interval.upper_bound,
                    true
                )),
                comp.p_value,
                if different_mean { "<" } else { ">" },
                comp.significance_threshold
            );

            console_log!("{}{}", " ".repeat(24), explanation_str);
        }

        self.outliers(&meas.avg_times);
    }
}

enum ComparisonResult {
    Improved,
    Regressed,
    NonSignificant,
}

fn compare_to_threshold(estimate: &Estimate, noise: f64) -> ComparisonResult {
    let ci = &estimate.confidence_interval;
    let lb = ci.lower_bound;
    let ub = ci.upper_bound;

    if lb < -noise && ub < -noise {
        ComparisonResult::Improved
    } else if lb > noise && ub > noise {
        ComparisonResult::Regressed
    } else {
        ComparisonResult::NonSignificant
    }
}
