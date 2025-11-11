//! Record previous benchmark data

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{estimate::Estimates, SavedSample};
use core::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static PREV: RefCell<HashMap<String, PrevBenchmark>> = RefCell::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrevBenchmark {
    pub sample: SavedSample,
    pub estimates: Estimates,
}

/// Write the corresponding benchmark ID and corresponding data into the table.
pub fn write(id: &str, prev: PrevBenchmark) {
    PREV.with(|p| p.borrow_mut().insert(id.into(), prev));
}

/// Read the data corresponding to the benchmark ID from the table.
pub fn read(id: &str) -> Option<PrevBenchmark> {
    PREV.with(|p| p.borrow().get(id).cloned())
}

/// Used to write previous benchmark data before the benchmark, for later comparison.
#[wasm_bindgen]
pub fn __wbgbench_import(prev: Vec<u8>) {
    match serde_json::from_slice(&prev) {
        Ok(prev) => {
            PREV.with(|c| std::mem::replace(&mut *c.borrow_mut(), prev));
        }
        Err(e) => {
            console_log!("Failed to import previous benchmark {e:?}");
        }
    }
}

/// Used to read benchmark data, and then the runner stores it on the local disk.
#[wasm_bindgen]
pub fn __wbgbench_dump() -> Option<Vec<u8>> {
    PREV.with(|c| serde_json::to_vec(&*c.borrow())).ok()
}
