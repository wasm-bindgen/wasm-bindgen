//! Collects walltime and converts it into codspeed results,
//! based on `codspeed-rust` 4.1.0.

mod codspeed;

use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::codspeed::{WalltimeBenchmark, WalltimeResults};

#[derive(Deserialize)]
pub struct CriterionBenchmark {
    file: String,
    module_path: String,
    iters: Vec<f64>,
    times: Vec<f64>,
}

fn create_uri_and_name(file: &str, module: &str, desc: &str) -> (String, String) {
    let uri = format!("{file}::{module}::{desc}");
    (uri, desc.into())
}

fn collect_walltime_results(desc: &str, benchmark: &CriterionBenchmark) -> WalltimeBenchmark {
    let CriterionBenchmark {
        file,
        module_path,
        iters,
        times,
    } = benchmark;
    let (uri, bench_name) = create_uri_and_name(file, module_path, desc);

    let iters_per_round = iters.iter().map(|t| *t as u128).collect();
    let times_per_round_ns = times.iter().map(|t| *t as u128).collect();

    WalltimeBenchmark::from_runtime_data(bench_name, uri, iters_per_round, times_per_round_ns, None)
}

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .context("Failed to get path of wasm-bindgen benchmark result")?;
    let result = std::fs::read(path).unwrap();
    let benchmark: HashMap<String, CriterionBenchmark> = serde_json::from_slice(&result).unwrap();
    let results = benchmark
        .into_iter()
        .map(|(desc, benchmark)| collect_walltime_results(&desc, &benchmark))
        .collect::<Vec<_>>();
    let results = WalltimeResults::new(results);

    let results_folder = std::env::var("CODSPEED_PROFILE_FOLDER")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new("target/codspeed/profiles").to_path_buf())
        .join("results");

    std::fs::create_dir_all(&results_folder).context("Failed to create results folder")?;

    // The pid here does not represent the test run's process ID, as profiling is currently skipped.
    let results_path = results_folder.join(format!("{}.json", std::process::id()));
    let mut results_file =
        std::fs::File::create(&results_path).context("Failed to create results file")?;
    serde_json::to_writer_pretty(&results_file, &results)?;
    results_file
        .flush()
        .context("Failed to flush results file")?;

    Ok(())
}
