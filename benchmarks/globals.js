export function jsthunk() {}
export function add(a, b) { return a + b; }
export function use_baz(baz) { 
  if (baz !== Baz['variant-2']) {
    throw new Error("Passed wrong variant");
  }
 }
export class Foo {
  bar() {}
}

export const Baz = {
  'variant-1': 'variant-1',
  'variant-2': 'variant-2',
  'variant-3': 'variant-3',
}
