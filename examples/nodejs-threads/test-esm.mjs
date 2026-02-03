/**
 * Integration test for Node.js ESM with worker_threads support.
 *
 * Tests:
 * 1. Main thread auto-initialization (backwards compatibility)
 * 2. Worker thread initialization with initSync({ module, memory })
 * 3. Shared atomic counter between threads
 * 4. __wbg_get_imports export availability
 * 5. Memory growth visibility from worker threads
 */

import { Worker, isMainThread, parentPort, workerData } from 'worker_threads';
import assert from 'assert';
import { fileURLToPath } from 'url';

// Path to built WASM module (in dist for CI, pkg for local dev)
const WASM_PATH = '../dist/nodejs-threads-esm/nodejs_threads.js';

if (isMainThread) {
    // Main thread tests
    async function runTests() {
        console.log('=== Node.js ESM worker_threads Integration Test ===\n');

        // Test 1: Main thread auto-initialization
        console.log('Test 1: Main thread auto-initialization');
        const wasm = await import(WASM_PATH);

        assert.strictEqual(typeof wasm.add, 'function', 'add function should be exported');
        assert.strictEqual(wasm.add(2, 3), 5, 'add(2, 3) should return 5');
        console.log('  ✓ Main thread initialized and functions work\n');

        // Test 2: Verify initSync and __wbg_get_imports are exported
        console.log('Test 2: initSync and __wbg_get_imports exports');
        assert.strictEqual(typeof wasm.initSync, 'function', 'initSync should be exported');
        assert.strictEqual(typeof wasm.__wbg_get_imports, 'function', '__wbg_get_imports should be exported');
        assert.ok(wasm.__wbg_wasm_module instanceof WebAssembly.Module, '__wbindgen_wasm_module should be a Module');
        console.log('  ✓ initSync and __wbg_get_imports are exported\n');

        // Test 3: Atomic counter starts at 0
        console.log('Test 3: Atomic counter initial state');
        assert.strictEqual(wasm.get_counter(), 0, 'Counter should start at 0');
        console.log('  ✓ Counter starts at 0\n');

        // Test 4: Increment counter from main thread
        console.log('Test 4: Increment from main thread');
        const prev = wasm.increment();
        assert.strictEqual(prev, 0, 'First increment should return 0');
        assert.strictEqual(wasm.get_counter(), 1, 'Counter should be 1 after increment');
        console.log('  ✓ Counter incremented to 1\n');

        // Test 5: Worker thread initialization and shared memory
        console.log('Test 5: Worker thread with shared memory');

        const workerResult = await new Promise((resolve, reject) => {
            const worker = new Worker(fileURLToPath(import.meta.url), {
                workerData: {
                    wasmModule: wasm.__wbg_wasm_module,
                    memory: wasm.__wbg_memory,
                    testType: 'basic'
                }
            });

            worker.on('message', resolve);
            worker.on('error', reject);
            worker.on('exit', (code) => {
                if (code !== 0) reject(new Error(`Worker exited with code ${code}`));
            });
        });

        assert.strictEqual(workerResult.success, true, 'Worker should succeed');
        assert.strictEqual(workerResult.addResult, 10, 'Worker add(4, 6) should be 10');
        console.log('  ✓ Worker thread initialized and ran successfully\n');

        // Test 6: Verify shared counter was modified by worker
        console.log('Test 6: Shared counter across threads');
        const finalCount = wasm.get_counter();
        assert.strictEqual(finalCount, 2, 'Counter should be 2 (main + worker each incremented once)');
        console.log(`  ✓ Counter is ${finalCount} (modified by both threads)\n`);

        // Test 7: Memory growth visibility from worker threads
        // This test verifies:
        // 1. Worker can trigger memory growth via allocations
        // 2. Main thread sees the memory growth (shared memory)
        console.log('Test 7: Memory growth visibility from worker');
        const sizeBeforeWorker = wasm.__wbg_memory.buffer.byteLength;
        console.log(`  Size before worker: ${sizeBeforeWorker} bytes`);

        const growthResult = await new Promise((resolve, reject) => {
            const worker = new Worker(fileURLToPath(import.meta.url), {
                workerData: {
                    wasmModule: wasm.__wbg_wasm_module,
                    memory: wasm.__wbg_memory,
                    testType: 'memory_growth'
                }
            });

            worker.on('message', resolve);
            worker.on('error', reject);
            worker.on('exit', (code) => {
                if (code !== 0) reject(new Error(`Worker exited with code ${code}`));
            });
        });

        assert.strictEqual(growthResult.success, true, 'Worker memory growth test should succeed');

        // Verify memory growth from worker is visible to main thread
        const sizeAfterWorker = wasm.__wbg_memory.buffer.byteLength;
        assert.strictEqual(sizeAfterWorker, growthResult.finalSize,
            'Main thread should see same memory size as worker');
        assert.ok(sizeAfterWorker >= sizeBeforeWorker,
            'Memory should have grown or stayed same (never shrink)');

        console.log(`  Worker reports final size: ${growthResult.finalSize} bytes`);
        console.log(`  Main thread now sees: ${sizeAfterWorker} bytes`);
        console.log('  ✓ Memory growth is visible across threads\n');

        console.log('=== All ESM tests passed! ===');
    }

    runTests().catch(err => {
        console.error('Test failed:', err);
        process.exit(1);
    });

} else {
    // Worker thread
    const wasm = await import(WASM_PATH);

    try {
        // Initialize with shared module and memory
        wasm.initSync({
            module: workerData.wasmModule,
            memory: workerData.memory
        });

        if (workerData.testType === 'basic') {
            // Verify functions work
            const addResult = wasm.add(4, 6);

            // Increment the shared counter
            wasm.increment();

            parentPort.postMessage({
                success: true,
                addResult: addResult
            });
        } else if (workerData.testType === 'memory_growth') {
            // Trigger memory growth from worker
            wasm.allocate_and_sum(50000); // Allocate enough to potentially grow

            // Check final size
            const finalSize = wasm.__wbg_memory.buffer.byteLength;

            parentPort.postMessage({
                success: true,
                finalSize: finalSize
            });
        }
    } catch (err) {
        parentPort.postMessage({
            success: false,
            error: err.message
        });
    }
}
