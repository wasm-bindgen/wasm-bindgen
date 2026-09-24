# Emscripten Target

`wasm-bindgen` supports the `wasm32-unknown-emscripten` target in addition to
`wasm32-unknown-unknown`. Emscripten links a libc, an in-memory file system,
POSIX-style APIs and its own JavaScript runtime around the Wasm, so more of
`std` works out of the box (`std::fs`, `std::time`, `std::env`, ...), Rust can
be linked against C/C++ sources, and a Tokio runtime can drive exported async
functions (see [Tokio](#tokio)). Otherwise `wasm32-unknown-unknown` stays the
right default: it has the smallest runtime and the fastest cold start.

> **Experimental.** Emscripten support is newer than the rest of `wasm-bindgen`
> and is still being smoothed out; flags and output shape may still change.

## Quick start

Install the target, [Emscripten](https://emscripten.org/docs/getting_started/downloads.html)
**6.0.10 or newer** with `emcc` on `PATH`, and the `wasm-bindgen` CLI at
**exactly** the `wasm-bindgen` crate version in your `Cargo.lock`:

```sh
rustup target add wasm32-unknown-emscripten
cargo install wasm-bindgen-cli --version <version>
```

Emscripten packages are **binary crates**: a plain `src/main.rs` with no
`[lib] crate-type = ["cdylib"]`; `main()` runs on module init and may be
empty. Write the API as `#[wasm_bindgen]` exports:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Greeter {
    greeting: String,
}

#[wasm_bindgen]
impl Greeter {
    #[wasm_bindgen(constructor)]
    pub fn new(greeting: String) -> Greeter {
        Greeter { greeting }
    }

    pub fn greet(&self, name: &str) -> String {
        format!("{}, {}!", self.greeting, name)
    }
}

fn main() {}
```

Configure the target and the Emscripten link settings in `.cargo/config.toml`:

```toml
[build]
target = "wasm32-unknown-emscripten"

[target.wasm32-unknown-emscripten]
rustflags = [
  "-Cpanic=abort",
  "-Cllvm-args=-enable-emscripten-cxx-exceptions=0",
  "-Crelocation-model=static",
  "-Clink-arg=-sWASM_BINDGEN",
  "-Clink-arg=-Wno-experimental",
  "-Clink-arg=-sMODULARIZE",
  "-Clink-arg=-sEXPORT_ES6",
]
```

Then `cargo build` produces `<name>.js` and `<name>.wasm` in
`target/wasm32-unknown-emscripten/debug/`, ready to import:

```js
import Module from "./target/wasm32-unknown-emscripten/debug/my_crate.js";

const mod = await Module();
console.log(new mod.Greeter("Hello").greet("world"));
```

## How it works

Instead of running the `wasm-bindgen` CLI yourself after linking, Emscripten
runs it for you. rustc drives `emcc` as the linker for the Emscripten target,
and when `-sWASM_BINDGEN` is passed, `emcc` detects the marker section the
`wasm-bindgen` crate embeds for Emscripten builds, runs the `wasm-bindgen`
CLI (found by name on `PATH`) over the linked Wasm as a post-link step, and
integrates the generated bindings (`library_bindgen.js`) into its own
JavaScript output. The clean `#[wasm_bindgen]` API — free functions, classes,
enums, namespaces — is then surfaced by Emscripten's module wrapper, while the
raw Wasm exports and `wasm-bindgen`'s internal glue are kept off the public
surface. There is no separate `wasm-bindgen` or `wasm-opt` step: `emcc` runs
its own optimization pipeline at link time, driven by the rustc opt-level.

`-sWASM_BINDGEN` is a no-op for inputs without the marker section, so it can
be passed unconditionally.

The crate must be a `bin` because rustc links Emscripten `bin` targets through
`emcc` as self-contained main modules; a `cdylib` is instead linked as an
Emscripten *side module* (`-sSIDE_MODULE=2`), a relocatable object for
Emscripten's dynamic linking rather than a usable package.

## Build configuration

The codegen flags in the configuration above:

- `-Cpanic=abort` — `panic=unwind` is not yet supported across the
  `wasm-bindgen` boundary on Emscripten
  ([#5165](https://github.com/wasm-bindgen/wasm-bindgen/issues/5165)).
- `-Cllvm-args=-enable-emscripten-cxx-exceptions=0` — avoids pulling in a
  C++ exception runtime that `panic=abort` never uses.
- `-Crelocation-model=static` — PIC is not needed for a statically linked
  main module.

`-Wno-experimental` silences `emcc`'s warning that `-sWASM_BINDGEN` is an
experimental setting. Any other `emcc` setting is passed the same way, e.g.
`"-Clink-arg=-sSTACK_SIZE=8MB"` or `"-Clink-arg=-sALLOW_MEMORY_GROWTH"`.

### Output modes

Emscripten's usual output settings control how the API is exposed:

| Settings | Consumption |
|---|---|
| `-sMODULARIZE -sEXPORT_ES6` | A factory: `import Module from './name.js'; const mod = await Module(); new mod.Greeter(..)` |
| `-sMODULARIZE=instance -sEXPORT_ES6` | Named ESM exports plus an `init` default export: `import init, { Greeter } from './name.js'; await init();` |
| `-sWASM_ESM_INTEGRATION` | As above, with the Wasm itself imported as an ES module |

With the named-export modes, adding `-sAUTO_INIT` makes the module
self-initialize on import so no `init()` call is needed.

### JavaScript imports and snippets

When the crate imports JavaScript modules (`#[wasm_bindgen(module = "...")]`
or [JS snippets](js-snippets.md)), the imports are emitted to a sidecar
`library_bindgen.extern-pre.js` that `emcc` prepends to its output, and the
`snippets/` directory is copied next to the output. Cargo only copies the
primary `.js` and `.wasm` out of `deps/`, so consume from — or copy from —
`target/wasm32-unknown-emscripten/<profile>/deps/` in that case.

### Linking as a static library

The other direction also works: build a `staticlib` with `cargo`, then let
`emcc` drive the link together with C/C++ sources:

```sh
emcc main.c target/wasm32-unknown-emscripten/release/libmylib.a -sWASM_BINDGEN -o out.js
```

`EMSCRIPTEN_KEEPALIVE` native exports and the `#[wasm_bindgen]` API compose
in the same module.

## Tokio

> **Experimental.** This depends on Tokio's Emscripten event-loop support,
> which has not yet shipped in a Tokio release, and is subject to change.

An exported `async fn` marked `#[wasm_bindgen(experimental_tokio)]` is driven
as a root on a Tokio event-loop runtime instead of the `wasm-bindgen-futures`
executor, with its outcome bridged to the returned `Promise`. `tokio::spawn`,
timers and Tokio I/O then work inside the export without JSPI. All such
exports share the thread's ambient runtime; `experimental_tokio = "isolated"`
gives each invocation its own runtime, torn down once the root future
settles, for multiplexed hosts where one invocation's I/O must not cross
into another's context.

```rust
#[wasm_bindgen(experimental_tokio)]
pub async fn fetch(req: Request) -> Response {
    // tokio::net, tokio::time, tokio::spawn are all usable here
}
```

The support is unstable and gated behind `--cfg wasm_bindgen_unstable_tokio`.
The cfg makes `wasm-bindgen-futures` depend on Tokio and expose the runtime
glue; without it nothing Tokio-related is compiled or linked, and the
attribute is a compile error (as it is on any other target).

Until the Tokio side lands upstream ([tokio#8484], with [mio#1969] for the
reactor), both crates come from a tagged patchset, together with Tokio's own
`--cfg tokio_unstable`:

```toml
# Cargo.toml
[patch.crates-io]
mio = { git = "https://github.com/guybedford/mio", tag = "1.2.3-cf.emscripten" }
tokio = { git = "https://github.com/guybedford/tokio", tag = "1.53.1-cf.emscripten" }
```

```toml
# .cargo/config.toml
[target.wasm32-unknown-emscripten]
rustflags = ["--cfg=wasm_bindgen_unstable_tokio", "--cfg=tokio_unstable", ...]
```

The reactor also needs Emscripten's epoll readiness listeners
([emscripten#27547]) and async DNS ([emscripten#27742]), not yet in a
release. The `6.0.10-cf.emscripten` tag of [guybedford/emscripten] is
Emscripten 6.0.10 plus those two changes; with emsdk 6.0.10 installed and
activated, use its checkout in place of emsdk's `upstream/emscripten` (replace
that directory, or point `EM_CONFIG` at a config reusing emsdk's toolchain):

```sh
git clone --branch 6.0.10-cf.emscripten https://github.com/guybedford/emscripten
cat > emscripten/.emscripten <<EOF
LLVM_ROOT = '$EMSDK/upstream/bin'
BINARYEN_ROOT = '$EMSDK/upstream'
NODE_JS = '$(command -v node)'
EOF
export EM_CONFIG=$PWD/emscripten/.emscripten PATH=$PWD/emscripten:$PATH
```

Timers, `tokio::spawn` and the sync primitives work on stock Emscripten
6.0.10 too, where the missing listener intrinsics make linking with Tokio's
`net` feature require `-Clink-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0`.

[tokio#8484]: https://github.com/tokio-rs/tokio/pull/8484
[mio#1969]: https://github.com/tokio-rs/mio/pull/1969
[emscripten#27547]: https://github.com/emscripten-core/emscripten/pull/27547
[emscripten#27742]: https://github.com/emscripten-core/emscripten/pull/27742
[guybedford/emscripten]: https://github.com/guybedford/emscripten/tree/6.0.10-cf.emscripten

## Limitations

- `-Cpanic=unwind` is not yet supported
  ([#5165](https://github.com/wasm-bindgen/wasm-bindgen/issues/5165)).
- `wasm-bindgen-test` supports the target via
  `wasm_bindgen_test_configure!(run_in_emscripten)`, but the harness
  currently only verifies the generated bindings; test bodies are compiled,
  not executed.
