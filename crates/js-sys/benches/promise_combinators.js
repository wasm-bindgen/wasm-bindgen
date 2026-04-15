// Helper for the promise-combinator benchmark.
//
// Creates a promise that resolves to `value` after yielding once to the
// event loop via `setTimeout(0)`. Each invocation returns a fresh promise,
// so callers can build a batch for `Promise.all` / `join_all`.
export function makeTimeoutPromise(value) {
  return new Promise((resolve) => setTimeout(() => resolve(value), 0));
}
