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
