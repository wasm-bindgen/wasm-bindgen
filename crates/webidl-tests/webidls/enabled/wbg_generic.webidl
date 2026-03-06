// Test the [WbgGeneric] attribute which opts stable APIs into typed generics.

[WbgGeneric]
dictionary GenericDict {
  sequence<long> items;
  required DOMString name;
};

// A non-generic dictionary for comparison
dictionary NonGenericDict {
  sequence<long> items;
  required DOMString name;
};
