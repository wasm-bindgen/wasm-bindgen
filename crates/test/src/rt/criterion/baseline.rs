//! Record previous benchmark data

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{estimate::Estimates, SavedSample};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use wasm_bindgen::__rt::LazyCell;

#[cfg_attr(target_feature = "atomics", thread_local)]
static BASELINE: LazyCell<RefCell<BTreeMap<String, BenchmarkBaseline>>> =
    LazyCell::new(|| RefCell::new(BTreeMap::new()));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct BenchmarkBaseline {
    pub(crate) file: Option<String>,
    pub(crate) module_path: Option<String>,
    pub(crate) iters: Vec<f64>,
    pub(crate) times: Vec<f64>,
    pub(crate) sample: SavedSample,
    pub(crate) estimates: Estimates,
}

/// Write the corresponding benchmark ID and corresponding data into the table.
pub(crate) fn write(id: &str, baseline: BenchmarkBaseline) {
    BASELINE.borrow_mut().insert(id.into(), baseline);
}

/// Read the data corresponding to the benchmark ID from the table.
pub(crate) fn read(id: &str) -> Option<BenchmarkBaseline> {
    BASELINE.borrow().get(id).cloned()
}

/// Used to write previous benchmark data before the benchmark, for later comparison.
#[wasm_bindgen]
pub fn __wbgbench_import(baseline: Vec<u8>) {
    match serde_json::from_slice(&baseline) {
        Ok(prev) => {
            *BASELINE.borrow_mut() = prev;
        }
        Err(e) => {
            console_log!("Failed to import previous benchmark {e:?}");
        }
    }
}

/// Used to read benchmark data, and then the runner stores it on the local disk.
#[wasm_bindgen]
pub fn __wbgbench_dump() -> Option<Vec<u8>> {
    let baseline = BASELINE.borrow();
    if baseline.is_empty() {
        return None;
    }
    serde_json::to_vec(&*baseline).ok()
}
