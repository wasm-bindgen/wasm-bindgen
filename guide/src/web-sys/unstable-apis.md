# Unstable APIs

It's common for browsers to implement parts of a web API while the specification
for that API is still being written. The API may require frequent changes as the
specification continues to be developed, so the WebIDL is relatively unstable.

This causes some challenges for `web-sys` because it means `web-sys` would have
to make breaking API changes whenever the WebIDL changes. It also means that
previously published `web-sys` versions would be invalid, because the browser
API may have been changed to match the updated WebIDL.

To avoid frequent breaking changes for unstable APIs, `web-sys` hides all
unstable APIs through an attribute that looks like:

```rust
#[cfg(web_sys_unstable_apis)]
pub struct Foo;
```

By hiding unstable APIs through an attribute, it's necessary for crates to
explicitly opt-in to these reduced stability guarantees in order to use these
APIs. Specifically, these APIs do not follow semver and may break whenever the
WebIDL changes.

Crates can opt-in to unstable APIs at compile-time by passing the `cfg` flag
`web_sys_unstable_apis`.

Typically the `RUSTFLAGS` environment variable is used
to do this. For example:

```bash
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo run
```

Alternatively, you can create a [cargo config file](https://doc.rust-lang.org/cargo/reference/config.html)
to set its [rustflags](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags):

Within `./.cargo/config.toml`:
```toml
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```

## String arguments

Unstable APIs bind string arguments generically: each string argument accepts
any of `&str`, `String`, `js_sys::JsString` or `&js_sys::JsString` through the
[`JsStringLike`] bound, crossing the boundary at that shape's native wire
format. Passing a cached `JsString` handle therefore avoids re-encoding the
string on every call:

```rust
ctx.set_fill_style(&cached_js_string);
```

[`JsStringLike`]: https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/experimental_generic_mono.html#string-bounds
