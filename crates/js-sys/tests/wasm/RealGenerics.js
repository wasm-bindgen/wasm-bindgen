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

export function callWith(f, a, b) {
  return f(a, b);
}

export function show(v) {
  return `${typeof v} ${v}`;
}

export function throwingIterator() {
  return {
    next() {
      throw 'boom';
    },
  };
}

export function later() {
  return Promise.resolve();
}

export function optionals() {
  return ['a', null, undefined, 'b'];
}

export function optionalNumbers() {
  return [1, null, 2.5];
}

export function bufferKind(b) {
  if (b === undefined || b === null) return 'none';
  const bytes = b instanceof ArrayBuffer ? new Uint8Array(b) : new Uint8Array(b.buffer, b.byteOffset, b.byteLength);
  return `${b.constructor.name}:${Array.from(bytes)}`;
}

export function callThunk(f) {
  return f();
}
