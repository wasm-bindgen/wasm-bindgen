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

// Value async iterable - only has values()
[Constructor()]
interface TestSingleAsyncIterable {
  async iterable<DOMString>;
};

// Pair async iterable
[Constructor()]
interface TestDoubleAsyncIterable {
  async iterable<DOMString, long>;
};
