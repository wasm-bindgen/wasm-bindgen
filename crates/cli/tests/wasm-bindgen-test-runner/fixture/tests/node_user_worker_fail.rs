//! Spawns a `worker_threads` worker that logs and then throws, failing the
//! test. Requires `NODE_ARGS=--require=worker_spawner.cjs`.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = spawnWorkerWithLogThenFail)]
    async fn spawn_worker_with_log_then_fail();
}

#[wasm_bindgen_test]
async fn test_spawned_worker_logs_then_fails() {
    spawn_worker_with_log_then_fail().await;
}
