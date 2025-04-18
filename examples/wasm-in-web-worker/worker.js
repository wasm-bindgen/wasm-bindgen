// Use ES module import syntax to import functionality from the module
// that we have compiled.
//
// The worker has its own scope and no direct access to functions/objects of the
// global scope. 
import init, { NumberEval } from './pkg/wasm_in_web_worker.js';


console.log('Initializing worker')


async function init_wasm_in_worker() {
    // Load the Wasm file.
    await init();

    // Create a new object of the `NumberEval` struct.
    var num_eval = NumberEval.new();

    // Set callback to handle messages passed to the worker.
    self.onmessage = async event => {
        // By using methods of a struct as reaction to messages passed to the
        // worker, we can preserve our state between messages.
        var worker_result = num_eval.is_even(event.data);

        // Send response back to be handled by callback in main thread.
        self.postMessage(worker_result);
    };
};

init_wasm_in_worker();
