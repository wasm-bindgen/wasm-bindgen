//! Spawns a `worker_threads` worker from Node.js ESM.
//! Requires `NODE_ARGS=--import=worker_spawner.mjs`.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = spawnWorkerWithLog)]
    async fn spawn_worker_with_log();
}

#[wasm_bindgen_test]
async fn test_spawned_worker_logs() {
    spawn_worker_with_log().await;
}
