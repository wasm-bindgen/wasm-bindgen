// String argument positions under typed generics: with [WbgGeneric] (or in
// next_unstable mode) every top-level string argument becomes a generic
// parameter bounded by JsStringLike, so any string shape crosses at its
// native wire format. Returns stay concrete String.

[Constructor(), WbgGeneric]
interface GenericStrings {
  attribute DOMString title;
  attribute DOMString? nickname;
  DOMString echo(DOMString value);
  DOMString join(DOMString a, USVString b, unsigned long n);
  DOMString? maybe(DOMString? value);
  [Throws] ByteString tryEcho(DOMString value);
  static DOMString echoStatic(DOMString value);
  DOMString describeDict(GenericStringDict dict);
};

[WbgGeneric]
dictionary GenericStringDict {
  required DOMString label;
  DOMString note;
};

// The same surface without [WbgGeneric]: legacy &str/String signatures in
// stable modes, generic in next_unstable mode.
[Constructor()]
interface PlainStrings {
  attribute DOMString title;
  DOMString echo(DOMString value);
};

namespace stringNs {
  readonly attribute DOMString tag;
  DOMString concat(DOMString a, DOMString b);
};
