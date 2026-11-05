use serde::{Deserialize, Serialize};
use statrs::statistics::{Data, Distribution, Max, Min, OrderStatistics};

const IQR_OUTLIER_FACTOR: f64 = 1.5;
const STDEV_OUTLIER_FACTOR: f64 = 3.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkStats {
    min_ns: f64,
    max_ns: f64,
    mean_ns: f64,
    stdev_ns: f64,

    q1_ns: f64,
    median_ns: f64,
    q3_ns: f64,

    rounds: u64,
    total_time: f64,
    iqr_outlier_rounds: u64,
    stdev_outlier_rounds: u64,
    iter_per_round: u64,
    warmup_iters: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct BenchmarkConfig {
    warmup_time_ns: Option<f64>,
    min_round_time_ns: Option<f64>,
    max_time_ns: Option<f64>,
    max_rounds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalltimeBenchmark {
    #[serde(flatten)]
    metadata: BenchmarkMetadata,

    config: BenchmarkConfig,
    stats: BenchmarkStats,
}

impl WalltimeBenchmark {
    pub fn from_runtime_data(
        name: String,
        uri: String,
        iters_per_round: Vec<u128>,
        times_per_round_ns: Vec<u128>,
        max_time_ns: Option<u128>,
    ) -> Self {
        let total_time = times_per_round_ns.iter().sum::<u128>() as f64 / 1_000_000_000.0;
        let time_per_iteration_per_round_ns: Vec<_> = times_per_round_ns
            .into_iter()
            .zip(&iters_per_round)
            .map(|(time_per_round, iter_per_round)| time_per_round / iter_per_round)
            .map(|t| t as f64)
            .collect::<Vec<f64>>();

        let mut data = Data::new(time_per_iteration_per_round_ns);
        let rounds = data.len() as u64;

        let mean_ns = data.mean().unwrap();

        let stdev_ns = if data.len() < 2 {
            // std_dev() returns f64::NAN if data has less than two entries, so we have to
            // manually handle this case.
            0.0
        } else {
            data.std_dev().unwrap()
        };

        let q1_ns = data.quantile(0.25);
        let median_ns = data.median();
        let q3_ns = data.quantile(0.75);

        let iqr_ns = q3_ns - q1_ns;
        let iqr_outlier_rounds = data
            .iter()
            .filter(|&&t| {
                t < q1_ns - IQR_OUTLIER_FACTOR * iqr_ns || t > q3_ns + IQR_OUTLIER_FACTOR * iqr_ns
            })
            .count() as u64;

        let stdev_outlier_rounds = data
            .iter()
            .filter(|&&t| {
                t < mean_ns - STDEV_OUTLIER_FACTOR * stdev_ns
                    || t > mean_ns + STDEV_OUTLIER_FACTOR * stdev_ns
            })
            .count() as u64;

        let min_ns = data.min();
        let max_ns = data.max();

        // TODO(COD-1056): We currently only support single iteration count per round
        let iter_per_round =
            (iters_per_round.iter().sum::<u128>() / iters_per_round.len() as u128) as u64;
        let warmup_iters = 0; // FIXME: add warmup detection

        let stats = BenchmarkStats {
            min_ns,
            max_ns,
            mean_ns,
            stdev_ns,
            q1_ns,
            median_ns,
            q3_ns,
            rounds,
            total_time,
            iqr_outlier_rounds,
            stdev_outlier_rounds,
            iter_per_round,
            warmup_iters,
        };

        WalltimeBenchmark {
            metadata: BenchmarkMetadata { name, uri },
            config: BenchmarkConfig {
                max_time_ns: max_time_ns.map(|t| t as f64),
                ..Default::default()
            },
            stats,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Instrument {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Creator {
    name: String,
    version: String,
    pid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalltimeResults {
    creator: Creator,
    instrument: Instrument,
    benchmarks: Vec<WalltimeBenchmark>,
}

impl WalltimeResults {
    pub fn new(benchmarks: Vec<WalltimeBenchmark>) -> Self {
        WalltimeResults {
            instrument: Instrument {
                type_: "walltime".to_string(),
            },
            creator: Creator {
                name: "codspeed-rust".to_string(),
                // API version at the time of copying
                version: "4.1.0".into(),
                pid: std::process::id(),
            },
            benchmarks,
        }
    }
}
