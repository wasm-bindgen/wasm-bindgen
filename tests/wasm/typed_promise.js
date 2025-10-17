export function createTypedNumberPromise(value) {
  return Promise.resolve(value * 2);
}

export function createTypedStringPromise(value) {
  return Promise.resolve(`JS: ${value}`);
}

export function processTypedPromise(promise) {
  return promise.then((value) => value + 200);
}

export function chainTypedPromises(promise1, promise2) {
  return promise1.then((value1) => promise2.then((value2) => value1 + value2));
}

export async function checkPromise(p) {
  const val = await p;
  if (val !== 42) {
    throw new Error('Promise did not resolve to 42');
  }
}
