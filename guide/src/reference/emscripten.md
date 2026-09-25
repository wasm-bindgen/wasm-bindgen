# Emscripten Target

`wasm-bindgen` supports the `wasm32-unknown-emscripten` target in addition to
`wasm32-unknown-unknown`. Emscripten links a libc, an in-memory file system,
POSIX-style APIs and its own JavaScript runtime around the Wasm, so more of
`std` works out of the box (`std::fs`, `std::time`, `std::env`, ...), Rust can
be linked against C/C++ sources, a Tokio runtime can drive exported async
functions (see [Tokio](#tokio)), and JSPI goes through Emscripten's own fiber
runtime (see [JSPI](#jspi)). Otherwise `wasm32-unknown-unknown` stays the
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
Emscripten 6.0.10 plus those changes; with `emcc` 6.0.10 (emsdk, Homebrew,
...) already on `PATH`, use its checkout as the frontend over that toolchain:

```sh
git clone --depth 1 --branch 6.0.10-cf.emscripten https://github.com/guybedford/emscripten
(cd emscripten && ./bootstrap.py)
printf "LLVM_ROOT = '%s'\nBINARYEN_ROOT = '%s'\n" "$(em-config LLVM_ROOT)" "$(em-config BINARYEN_ROOT)" > emscripten/.emscripten
export PATH=$PWD/emscripten:$PATH
```

Timers, `tokio::spawn` and the sync primitives work on stock Emscripten
6.0.10 too, where the missing listener intrinsics make linking with Tokio's
`net` feature require `-Clink-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0`.

[tokio#8484]: https://github.com/tokio-rs/tokio/pull/8484
[mio#1969]: https://github.com/tokio-rs/mio/pull/1969
[emscripten#27547]: https://github.com/emscripten-core/emscripten/pull/27547
[emscripten#27742]: https://github.com/emscripten-core/emscripten/pull/27742
[guybedford/emscripten]: https://github.com/guybedford/emscripten/tree/6.0.10-cf.emscripten

## JSPI

> **Experimental.** This depends on Emscripten's JSPI lifecycle hooks, which
> have not shipped in an Emscripten release, and is subject to change.

### Getting started

1. Emscripten 6.0.10 with the hooks: the `6.0.10-cf.emscripten` tag of
   [guybedford/emscripten] (the release plus [emscripten#27698] and
   [emscripten#27699]) as the frontend over your existing Emscripten install,
   and the [`version_132_jspi_hooks_1`][binaryen-release] Binaryen, whose
   `wasm-opt` has the `--jspi-hooks` pass. With `emcc` 6.0.10 (emsdk, Homebrew,
   ...) already on `PATH`:

   ```sh
   git clone --depth 1 --branch 6.0.10-cf.emscripten https://github.com/guybedford/emscripten
   (cd emscripten && ./bootstrap.py)
   curl -L https://github.com/guybedford/binaryen/releases/download/version_132_jspi_hooks_1/binaryen-version_132_jspi_hooks_1-x86_64-linux.tar.gz | tar xz
   printf "LLVM_ROOT = '%s'\nBINARYEN_ROOT = '%s'\n" "$(em-config LLVM_ROOT)" "$PWD/binaryen-version_132_jspi_hooks_1" > emscripten/.emscripten
   export PATH=$PWD/emscripten:$PATH
   ```

   (Pick the [binaryen asset][binaryen-release] for your platform.)

2. Build with the cfg and link with `-sJSPI` plus the hooks: `-sJSPI_HOOKS`,
   or `-sREENTRANT_JSPI`, which also gives every promising activation its
   own shadow stack so any number of them may be suspended at once:

   ```toml
   # .cargo/config.toml
   [target.wasm32-unknown-emscripten]
   rustflags = [
     "-Cpanic=abort",
     "-Cllvm-args=-enable-emscripten-cxx-exceptions=0",
     "-Crelocation-model=static",
     "--cfg=wasm_bindgen_unstable_jspi",
     "-Clink-arg=-sWASM_BINDGEN",
     "-Clink-arg=-sJSPI",
     "-Clink-arg=-sREENTRANT_JSPI",
   ]
   ```

3. Use the [JSPI attributes](jspi.md) as on any other target:

   ```rust
   #[wasm_bindgen]
   extern "C" {
       #[wasm_bindgen(suspending)]
       fn fetch_text(url: &str) -> String;
   }

   #[wasm_bindgen(jspi)]
   pub fn load(url: &str) -> usize {
       fetch_text(url).len() // parks the activation until the promise settles
   }
   ```

4. Run under a JSPI-capable engine (Node 25+, or Node 24 with
   `--experimental-wasm-jspi`). `jspi` exports return a `Promise`.

### How it works

On this target the fibers belong to Emscripten's JSPI runtime, so instead of
its own [shadow stack management](jspi.md#shadow-stack-management)
wasm-bindgen wraps each `#[wasm_bindgen(jspi)]` export and
`#[wasm_bindgen(suspending)]` import with the runtime's `__jspi_enter` /
`__jspi_exit` and `__jspi_suspend` / `__jspi_resume` hook exports (the same
instrumentation binaryen's `--jspi-hooks` pass applies to Emscripten's own
`JSPI_EXPORTS` and `JSPI_IMPORTS`), and tracks the ambient JSPI context
through a hook registered with `<emscripten/jspi.h>`. `jspi_block_on_promise`
and the JSPI context inheritance of `spawn_local` work as on the other
targets, and Emscripten's own promising exports and suspending imports
(`main`, `emscripten_sleep`, ...) share the same fiber system.

Without the cfg, JSPI on Emscripten keeps wasm-bindgen's shadow stack
management: nothing Emscripten-specific is referenced, so stock Emscripten
links. The runtime built with the cfg marks the module, and only then does
the CLI require the hooks (failing with a pointer at `-sJSPI_HOOKS` when they
are missing).

### With Tokio

`jspi` combines with [`experimental_tokio`](#tokio) into a *parked* runtime:
the export is a promising activation whose body runs the future to completion
with `block_on` on a current-thread Tokio runtime, and every wait of that
runtime (timers, I/O readiness, an idle scheduler) is a JSPI suspension of the
activation. To JS it is a sync `jspi` export: a `Promise` of the value.
`jspi_block_on_promise` and suspending imports work anywhere inside, in the
root or in spawned tasks. Invocations interleave at every wait: Tokio's
fiber-owned runtime context (`--cfg tokio_unstable_jspi_hooks`, in the tagged
Tokio) lets a sibling invocation enter while another is parked, whether the
park is Tokio's own or a suspension issued from task code.

```rust
#[wasm_bindgen(jspi, experimental_tokio)]
pub async fn handle(req: Request) -> Response {
    let config = jspi_block_on_promise(&load_config())?; // parks the activation
    tokio::time::sleep(Duration::from_millis(10)).await;  // parks it too
    // ...
}
```

By default all such exports share the thread's ambient runtime; with
`experimental_tokio = "isolated"` each invocation owns a fresh runtime,
dropped once the root settles (tasks still in flight are dropped, the reactor
closed). The shared runtime has one scheduler core: while an activation is
suspended from inside a task (a suspending import or `jspi_block_on_promise`
in task code, rather than a Tokio wait), it holds that core, and a sibling
invocation's timers and I/O only advance once it resumes. Isolated runtimes
park independently.

The complete configuration for the combination is the union of the Tokio and
JSPI ones above: the JSPI toolchain setup (the tag carries the Tokio changes
too), and Node 25 or newer (Node 24 with `--experimental-wasm-jspi`) to run.
Crates:

```toml
# Cargo.toml
[patch.crates-io]
mio = { git = "https://github.com/guybedford/mio", tag = "1.2.3-cf.emscripten" }
tokio = { git = "https://github.com/guybedford/tokio", tag = "1.53.1-cf.emscripten" }
```

```toml
# .cargo/config.toml
[target.wasm32-unknown-emscripten]
rustflags = [
  "-Cpanic=abort",
  "-Cllvm-args=-enable-emscripten-cxx-exceptions=0",
  "-Crelocation-model=static",
  "--cfg=wasm_bindgen_unstable_tokio",
  "--cfg=wasm_bindgen_unstable_jspi",
  "--cfg=tokio_unstable",
  "--cfg=tokio_unstable_jspi_hooks",
  "-Clink-arg=-sWASM_BINDGEN",
  "-Clink-arg=-sJSPI",
  "-Clink-arg=-sREENTRANT_JSPI",
]
```

`-sREENTRANT_JSPI` (which implies `-sJSPI_HOOKS`) is required: every parked
activation keeps its own shadow stack.

[emscripten#27698]: https://github.com/emscripten-core/emscripten/pull/27698
[emscripten#27699]: https://github.com/emscripten-core/emscripten/pull/27699
[binaryen-release]: https://github.com/guybedford/binaryen/releases/tag/version_132_jspi_hooks_1

## Limitations

- `-Cpanic=unwind` is not yet supported
  ([#5165](https://github.com/wasm-bindgen/wasm-bindgen/issues/5165)).
- `wasm-bindgen-test` supports the target via
  `wasm_bindgen_test_configure!(run_in_emscripten)`, but the harness
  currently only verifies the generated bindings; test bodies are compiled,
  not executed.
