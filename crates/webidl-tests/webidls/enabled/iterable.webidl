// Single-typed iterable (value iterator) - for interfaces with indexed properties
[Constructor()]
interface TestSingleIterable {
  getter DOMString (unsigned long index);
  readonly attribute unsigned long length;
  iterable<DOMString>;
};

// Double-typed iterable (pair iterator) - for key-value iteration
[Constructor()]
interface TestDoubleIterable {
  iterable<DOMString, long>;
};

// Value async iterable with multiple optional arguments.
// Only has values(), no entries() or keys().
[Constructor()]
interface TestSingleAsyncIterable {
  async iterable<DOMString>(optional TestAsyncIterableOptions options = {}, optional unsigned long skip = 0);
};

// Declared after its use, to make sure that async iterable arguments
// can refer to definitions that appear later.
dictionary TestAsyncIterableOptions {
  boolean reverse = false;
};

// Pair async iterable
[Constructor()]
interface TestDoubleAsyncIterable {
  async iterable<DOMString, long>;
};
