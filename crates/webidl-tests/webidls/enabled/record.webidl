[Constructor()]
interface TestRecord {
  // Record<DOMString, long> should become Object<Number> in non-compat mode
  record<DOMString, long> getNumberRecord();

  // Record<DOMString, DOMString> should become Object<JsString> in non-compat mode
  record<DOMString, DOMString> getStringRecord();

  // Set a record
  undefined setRecord(record<DOMString, long> data);
};
