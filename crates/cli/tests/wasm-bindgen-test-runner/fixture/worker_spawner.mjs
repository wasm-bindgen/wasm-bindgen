// Loaded with `NODE_ARGS=--import=...` by the Node.js ESM fixture run.
import { Worker } from 'worker_threads';

globalThis.spawnWorkerWithLog = function() {
    return new Promise((resolve, reject) => {
        const worker = new Worker(
            `
            console.debug("NODE_WORKER_DEBUG_MARKER_ESM_8T4R2");
            console.log("NODE_WORKER_LOG_MARKER_ESM_8T4R2");
            console.info("NODE_WORKER_INFO_MARKER_ESM_8T4R2");
            console.warn("NODE_WORKER_WARN_MARKER_ESM_8T4R2");
            console.error("NODE_WORKER_ERROR_MARKER_ESM_8T4R2");
            `,
            { eval: true }
        );
        worker.on('exit', () => resolve());
        worker.on('error', reject);
    });
};
