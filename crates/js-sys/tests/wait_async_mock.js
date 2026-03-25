// Mock for Atomics.waitAsync that resolves the returned promise synchronously
// via a notify, without actually waiting. This is used to reliably reproduce
// the race condition where the waitAsync promise callback runs before the task
// state has been transitioned back to AWAKE.

export function installNotifyOnlyWaitAsyncMock() {
    const original = Atomics.waitAsync;
    Atomics.waitAsync = function(typedArray, index, value, _timeout) {
        // Call the real waitAsync first so the wait is registered
        const result = original.call(this, typedArray, index, value, 0);
        // Then immediately notify to trigger the promise resolution as fast as
        // possible, before the caller has a chance to set up the SLEEPING state.
        Atomics.notify(typedArray, index, 1);
        return result;
    };
    return original;
}

export function restoreWaitAsyncMock(original) {
    Atomics.waitAsync = original;
}
