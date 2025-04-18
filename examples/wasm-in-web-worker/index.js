// Use ES module import syntax to import functionality from the module
// that we have compiled.
import init, { startup } from './pkg/wasm_in_web_worker.js';

async function run_wasm() {
    await init();

    console.log('index.js loaded');

    // Run main Wasm entry point
    // This will create a worker from within our Rust code compiled to Wasm
    startup();
}

run_wasm();
