// Typedef that expands to a Promise of a union type, mirroring
// ClipboardItemData = Promise<(DOMString or Blob)>.
typedef Promise<(DOMString or TestBlob)> TestClipboardItemData;

[Constructor()]
interface TestBlob {};

[Constructor()]
interface TestPromises {
  Promise<DOMString> stringPromise();
  Promise<any> anyPromise();
  Promise<DOMString?> optionalStringPromise();

  [WbgGeneric]
  undefined waitForString(Promise<DOMString> p);

  [WbgGeneric]
  undefined waitForAny(Promise<any> p);

  [WbgGeneric]
  undefined maybeWaitForString(optional Promise<DOMString> p);

  [WbgGeneric]
  attribute Promise<DOMString> promiseValue;

  // Tests that Promise<union> typedef expansion does not produce duplicate
  // names: the two canonical Promise branches must be disambiguated from
  // each other (e.g. foo_with_str_promise vs foo_with_test_blob_promise).
  [WbgGeneric]
  undefined acceptClipboardItem(TestClipboardItemData data);
};
