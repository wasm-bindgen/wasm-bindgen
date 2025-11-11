use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;
use codspeed::{
    instrument_hooks::InstrumentHooks,
    walltime_results::{WalltimeBenchmark, WalltimeResults},
};
use napi_derive::napi;

fn aggregate_raw_walltime_data_impl(workspace_root: &Path) -> anyhow::Result<()> {
    let results = WalltimeResults::collect_walltime_results(workspace_root)
        .with_context(|| {
            format!(
                "Failed to collect walltime results. This may be due to version incompatibility. \
                Ensure that your compat layer (codspeed-criterion-compat, codspeed-bencher-compat, or codspeed-divan-compat) \
                has the same major version as cargo-codspeed (currently v{}).",
                env!("CARGO_PKG_VERSION")
            )
        })?;

    if results.benchmarks().is_empty() {
        eprintln!("No walltime benchmarks found");
        return Ok(());
    }

    for bench in results.benchmarks() {
        if bench.is_invalid() {
            eprintln!(
                "Warning: Benchmark {} was possibly optimized away",
                bench.name()
            );
        }
    }

    let results_folder = std::env::var("CODSPEED_PROFILE_FOLDER")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("target/codspeed/profiles"))
        .join("results");
    std::fs::create_dir_all(&results_folder).context("Failed to create results folder")?;

    let results_path = results_folder.join(format!("{}.json", std::process::id()));
    let mut results_file =
        std::fs::File::create(&results_path).context("Failed to create results file")?;
    serde_json::to_writer_pretty(&results_file, &results)?;
    results_file
        .flush()
        .context("Failed to flush results file")?;
    Ok(())
}

#[napi]
pub fn aggregate_raw_walltime_data(workspace_root: String) -> bool {
    aggregate_raw_walltime_data_impl(Path::new(&workspace_root)).is_ok()
}

#[napi]
pub fn collect_raw_walltime_results(
    scope: String,
    name: String,
    uri: String,
    iters_per_round: Vec<String>,
    times_per_round_ns: Vec<String>,
    max_time_ns: Option<String>,
) {
    WalltimeBenchmark::collect_raw_walltime_results(
        &scope,
        name,
        uri,
        iters_per_round
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        times_per_round_ns
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        max_time_ns.map(|s| s.parse().unwrap()),
    )
}

#[napi]
pub fn init_codspeed() {
    InstrumentHooks::instance();
}

#[napi]
pub fn current_timestamp() -> String {
    InstrumentHooks::current_timestamp().to_string()
}

#[napi]
pub fn add_benchmark_timestamps(start: String, end: String) {
    InstrumentHooks::instance()
        .add_benchmark_timestamps(start.parse().unwrap(), end.parse().unwrap());
}

#[napi]
pub fn set_executed_benchmark(uri: String) -> bool {
    InstrumentHooks::instance()
        .set_executed_benchmark(&uri)
        .is_ok()
}

#[napi]
pub fn start_benchmark() -> bool {
    InstrumentHooks::instance().start_benchmark().is_ok()
}

#[napi]
pub fn stop_benchmark() -> bool {
    InstrumentHooks::instance().stop_benchmark().is_ok()
}
