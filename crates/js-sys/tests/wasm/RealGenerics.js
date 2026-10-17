// `Kind:[values]`, with each value's `typeof`, so a test sees exactly what JS
// received.
export function describe(v) {
  const kind = Array.isArray(v) ? 'Array' : v.constructor.name;
  return `${kind}:${JSON.stringify(Array.from(v, (x) => `${typeof x} ${x}`))}`;
}

export async function* asyncStrings() {
  yield 'a';
  yield 'b';
}

export function strings() {
  return ['x', 'y'];
}
