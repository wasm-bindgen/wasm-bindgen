[Constructor()]
interface TestPromises {
  Promise<DOMString> stringPromise();
  Promise<any> anyPromise();
  Promise<DOMString?> optionalStringPromise();
};

// Test that an interface extending a JS built-in type (Promise)
// generates the correct extends = ::js_sys::Promise attribute.
[Constructor()]
interface PromiseSubclass : Promise {
  DOMString label();
};

// Test that the resolution type is correctly inferred from the onfulfilled
// callback's first parameter (DOMString), not from then()'s return type.
callback TextThenCallback = DOMString (DOMString value);

[Constructor()]
interface TypedTextPromise : Promise {
  Promise<DOMString> then(TextThenCallback onfulfilled);
};

// Test that a child interface inherits the resolution type from
// its parent's then() method.
[Constructor()]
interface ChildTextPromise : TypedTextPromise {
  DOMString childLabel();
};
