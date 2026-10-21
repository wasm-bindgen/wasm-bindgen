//! Spawns a `worker_threads` worker from Node.js CJS (the default mode).
//! Requires `NODE_ARGS=--require=worker_spawner.cjs`.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = spawnWorkerWithLog)]
    async fn spawn_worker_with_log();
}

#[wasm_bindgen_test]
async fn test_spawned_worker_logs() {
    spawn_worker_with_log().await;
}
