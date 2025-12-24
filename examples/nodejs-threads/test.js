/**
 * Integration test for Node.js CJS with worker_threads support.
 *
 * Tests:
 * 1. Main thread auto-initialization (backwards compatibility)
 * 2. Worker thread initialization with initSync({ module, memory })
 * 3. Shared atomic counter between threads
 * 4. Memory growth detection with SharedArrayBuffer (byteLength fix)
 */

const { Worker, isMainThread, parentPort, workerData } = require('worker_threads');
const assert = require('assert');
const path = require('path');

if (isMainThread) {
    // Main thread tests
    async function runTests() {
        console.log('=== Node.js worker_threads Integration Test ===\n');

        // Test 1: Main thread auto-initialization
        console.log('Test 1: Main thread auto-initialization');
        const wasm = require('./pkg/nodejs_threads.js');

        assert.strictEqual(typeof wasm.add, 'function', 'add function should be exported');
        assert.strictEqual(wasm.add(2, 3), 5, 'add(2, 3) should return 5');
        console.log('  ✓ Main thread initialized and functions work\n');

        // Test 2: Verify initSync and __wbg_get_imports are exported
        console.log('Test 2: initSync and __wbg_get_imports exports');
        assert.strictEqual(typeof wasm.initSync, 'function', 'initSync should be exported');
        assert.strictEqual(typeof wasm.__wbg_get_imports, 'function', '__wbg_get_imports should be exported');
        assert.ok(wasm.__wbindgen_wasm_module instanceof WebAssembly.Module, '__wbindgen_wasm_module should be a Module');
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
            const worker = new Worker(__filename, {
                workerData: {
                    wasmModule: wasm.__wbindgen_wasm_module,
                    memory: wasm.memory
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

        // Test 7: Memory growth and cached view invalidation
        console.log('Test 7: Memory growth detection (byteLength fix)');
        const initialSize = wasm.memory.buffer.byteLength;
        console.log(`  Initial memory size: ${initialSize} bytes`);

        // Allocate a large amount to trigger memory growth
        // This should work correctly with the byteLength fix
        const largeSize = 1000000; // 1 million u32s = 4MB
        const sum = wasm.allocate_and_sum(largeSize);
        const expectedSum = (largeSize - 1) * largeSize / 2; // Sum of 0..n-1
        assert.strictEqual(sum, expectedSum, `allocate_and_sum should return ${expectedSum}`);

        const finalSize = wasm.memory.buffer.byteLength;
        console.log(`  Final memory size: ${finalSize} bytes`);

        if (finalSize > initialSize) {
            console.log('  ✓ Memory grew and cached views were correctly invalidated\n');
        } else {
            console.log('  ✓ Memory did not need to grow (already large enough)\n');
        }

        console.log('=== All tests passed! ===');
    }

    runTests().catch(err => {
        console.error('Test failed:', err);
        process.exit(1);
    });

} else {
    // Worker thread
    const wasm = require('./pkg/nodejs_threads.js');

    try {
        // Initialize with shared module and memory
        wasm.initSync({
            module: workerData.wasmModule,
            memory: workerData.memory
        });

        // Verify functions work
        const addResult = wasm.add(4, 6);

        // Increment the shared counter
        wasm.increment();

        parentPort.postMessage({
            success: true,
            addResult: addResult
        });
    } catch (err) {
        parentPort.postMessage({
            success: false,
            error: err.message
        });
    }
}
