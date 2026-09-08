# Type Translations in `web-sys`

Most of the types specified in [WebIDL (the interface definition language for
all Web APIs)][webidl] have relatively straightforward translations into
`web-sys`, but it's worth calling out a few in particular:

* `BufferSource` and `ArrayBufferView` - these two types show up in a number of
  APIs that generally deal with a buffer of bytes. We bind them in `web-sys`
  with two different types, `js_sys::Object` and `&mut [u8]`. Using
  `js_sys::Object` allows passing in arbitrary JS values which represent a view
  of bytes (like any typed array object), and `&mut [u8]` allows using a raw
  slice in Rust. Unfortunately we must pessimistically assume that JS will
  modify all slices as we don't currently have information of whether they're
  modified or not.

* Callbacks are all represented as `js_sys::Function`. This means that all
  callbacks going through `web-sys` are a raw JS value. You can work with this
  by either juggling actual `js_sys::Function` instances or you can create a
  `Closure<dyn FnMut(...)>`, extract the underlying `JsValue` with `as_ref`, and
  then use `JsCast::unchecked_ref` to convert it to a `js_sys::Function`.

* String returns (`DOMString` and friends) are bound as an owned Rust
  `String`, which copies the string data into wasm memory. Every
  string-returning function also has a `_js_string` variant returning a
  `js_sys::JsString` handle to the same value without copying it out — useful
  when the string is only passed back to other JS APIs, or is large.

[webidl]: https://heycam.github.io/webidl/
