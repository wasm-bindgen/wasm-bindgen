# JSPI + OPFS

[View full source code][code]

[code]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/jspi-opfs

This example demonstrates [JSPI (JS Promise Integration)][jspi] by calling the
browser's fully Promise-based [Origin Private File System (OPFS)][opfs] API
from plain (non-`async`) Rust functions.

[jspi]: ../reference/jspi.md
[opfs]: https://developer.mozilla.org/en-US/docs/Web/API/File_System_API/Origin_private_file_system

**Requirements:** Chrome 117+, Firefox 150+, or Safari 18.4+; secure context
(HTTPS or `localhost`).

## `Cargo.toml`

```toml
{{#include ../../../examples/jspi-opfs/Cargo.toml}}
```

## `src/lib.rs`

```rust
{{#include ../../../examples/jspi-opfs/src/lib.rs}}
```
