[Constructor()]
interface TestPromises {
  Promise<DOMString> stringPromise();
  Promise<any> anyPromise();
  Promise<DOMString?> optionalStringPromise();
};
