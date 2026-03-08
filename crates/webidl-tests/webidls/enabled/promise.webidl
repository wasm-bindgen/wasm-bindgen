[Constructor()]
interface TestPromises {
  Promise<DOMString> stringPromise();
  Promise<any> anyPromise();
  Promise<DOMString?> optionalStringPromise();

  [WbgGeneric]
  undefined waitForString(Promise<DOMString> p);

  [WbgGeneric]
  undefined waitForAny(Promise<any> p);
};
