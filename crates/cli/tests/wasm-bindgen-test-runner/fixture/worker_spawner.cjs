// Loaded with `NODE_ARGS=--require=...` by the Node.js CJS fixture runs.
const { Worker } = require('worker_threads');

globalThis.spawnWorkerWithLog = function() {
    return new Promise((resolve, reject) => {
        const worker = new Worker(
            `
            console.debug("NODE_WORKER_DEBUG_MARKER_CJS_5K2N9");
            console.log("NODE_WORKER_LOG_MARKER_CJS_5K2N9");
            console.info("NODE_WORKER_INFO_MARKER_CJS_5K2N9");
            console.warn("NODE_WORKER_WARN_MARKER_CJS_5K2N9");
            console.error("NODE_WORKER_ERROR_MARKER_CJS_5K2N9");
            `,
            { eval: true }
        );
        worker.on('exit', () => resolve());
        worker.on('error', reject);
    });
};

globalThis.spawnWorkerWithLogThenFail = function() {
    return new Promise((resolve, reject) => {
        const worker = new Worker(
            `
            console.log("NODE_WORKER_FAILURE_MARKER_7X9K3");
            throw new Error("Intentional node worker failure");
            `,
            { eval: true }
        );
        worker.on('exit', (code) => {
            if (code !== 0) {
                reject(new Error('Worker exited with code ' + code));
            } else {
                resolve();
            }
        });
        worker.on('error', reject);
    });
};
