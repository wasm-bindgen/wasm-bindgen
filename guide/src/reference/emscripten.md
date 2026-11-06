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

- `-Crelocation-model=static` — PIC is not needed for a statically linked
  main module.

`-Wno-experimental` silences `emcc`'s warning that `-sWASM_BINDGEN` is an
experimental setting. Any other `emcc` setting is passed the same way, e.g.
`"-Clink-arg=-sSTACK_SIZE=8MB"` or `"-Clink-arg=-sALLOW_MEMORY_GROWTH"`.

### Panics

`panic=unwind` is the default strategy on `wasm32-unknown-emscripten` and the
prebuilt `std` ships with it, so unlike `wasm32-unknown-unknown` no nightly
toolchain or `-Zbuild-std` is needed. rustc links with `-fwasm-exceptions`:
panics are thrown as native Wasm exceptions (as C++ exceptions, through
Emscripten's libunwind) in the legacy `try`/`catch` instruction dialect that
Emscripten emits by default. `wasm-bindgen` detects that dialect and emits its
own exception-handling code in the same form, so what
[Catching panics](catch-unwind.md) describes applies:

- A panic that reaches a `#[wasm_bindgen]` export or closure is caught and
  rethrown to JavaScript as a `PanicError`; the instance stays usable and the
  shadow stack is restored.
- `#[wasm_bindgen(catch)]` imports hand JavaScript exceptions to Rust as
  `Err`, and exceptions from other imports propagate through Rust frames
  (running destructors) unchanged.

Fatal errors, on the other hand, are Emscripten's business rather than
`wasm-bindgen`'s: a trap (`WebAssembly.RuntimeError`: `unreachable`, stack
overflow, out-of-bounds access) escaping an export is passed to Emscripten's
`abort()`, which calls `Module.onAbort`, sets `ABORT` and re-traps so the
error reaches the caller as a `RuntimeError`. None of the machinery in
[Handling aborts](handling-aborts.md) is generated for Emscripten output:
`set_on_abort` callbacks never fire (use `Module.onAbort`), later export
calls are not refused, and `schedule_reinit()` is a no-op since Emscripten
owns instantiation.

Anything else that escapes an export is rethrown to the caller untouched.
Unlike `wasm32-unknown-unknown`, most imports in an Emscripten module aren't
`wasm-bindgen`'s and aren't wrapped by it, so that covers JavaScript errors
thrown by `EM_JS` / JS library / embind imports, C++ exceptions from linked
C++ code, and Emscripten's own control-flow exceptions (`ExitStatus` from
`exit()`, `'unwind'` from `emscripten_set_main_loop(.., true)` or
`emscripten_exit_with_live_runtime`), the way Emscripten's `handleException`
passes them through. Note that these *unwind* through Rust frames, so such
functions must be bound as `extern "C-unwind"` (e.g. `emscripten_force_exit`
or `emscripten_throw_string`); through an `extern "C"` binding rustc's
cannot-unwind guard aborts first. That includes `std::process::exit`: it goes
through `libc::exit`, which std declares `extern "C"` and rustc won't let you
redeclare with another ABI.

All of the above covers Rust entered through `wasm-bindgen`'s glue: exports,
methods, and closures handed to JavaScript. Rust callbacks that Emscripten
itself invokes through the function table — `emscripten_async_call`,
`emscripten_set_main_loop`, `emscripten_set_*_callback` and the like, via its
`callUserCallback` — don't pass through that glue, so a panic inside one is
*not* converted to a `PanicError` and the shadow stack is not restored: bound
as `extern "C-unwind"` the raw `WebAssembly.Exception` escapes to Emscripten's
`handleException`, and bound as `extern "C"` rustc aborts on the unwind.
Catch panics inside such callbacks yourself (`std::panic::catch_unwind`) if
they must not take the runtime down.

This needs an engine with the legacy exception-handling instructions and
`WebAssembly.JSTag` (Node.js 22+ or a current browser). Unlike
`wasm32-unknown-unknown`, `JSTag` is not polyfilled on Emscripten, so Node.js
20 is not supported.

#### Standardized (`exnref`) exception handling

Emscripten selects the standardized exception-handling instructions with
`-sWASM_LEGACY_EXCEPTIONS=0`, which switches its own system libraries. The
Rust side has to match: user code via `-Cllvm-args=-wasm-use-legacy-eh=0`, and
`std` by rebuilding it, since the prebuilt one uses the legacy instructions.
That makes this a nightly + `-Zbuild-std` configuration:

```toml
[target.wasm32-unknown-emscripten]
rustflags = [
  "-Cllvm-args=-wasm-use-legacy-eh=0",
  "-Clink-arg=-sWASM_LEGACY_EXCEPTIONS=0",
  "-Crelocation-model=static",
  "-Clink-arg=-sWASM_BINDGEN",
  "-Clink-arg=-Wno-experimental",
  "-Clink-arg=-sMODULARIZE",
  "-Clink-arg=-sEXPORT_ES6",
]

[unstable]
build-std = ["std", "panic_unwind"]
```

`wasm-bindgen` detects the dialect from the linked module and emits its
wrappers as `try_table`. The output needs an engine with `exnref` support
(Node.js 22.22.3+ / 24.15.0+ or a recent browser), as on
`wasm32-unknown-unknown`.

#### `panic=abort`

To opt out of unwinding, build with `-Cpanic=abort`. Panics then trap, and
the trap goes to Emscripten's `abort()` as described above. Since the
prebuilt `std` still carries exception-handling instructions (and so does any
linked C++), `wasm-bindgen` detects the dialect and emits the same catch
wrappers and shadow-stack wrappers it does for `panic=unwind`; they're
inert for panics in this configuration but do route JavaScript exceptions
from `#[wasm_bindgen(catch)]` imports and `WebAssembly.RuntimeError`s the
same way. Adding `-Cllvm-args=-enable-emscripten-cxx-exceptions=0` avoids
linking the unused C++ exception runtime.

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

Until the Tokio side lands upstream ([tokio#8484]), Tokio comes from a tagged
patchset, together with its own `--cfg tokio_unstable`:

```toml
# Cargo.toml
[patch.crates-io]
tokio = { git = "https://github.com/guybedford/tokio", tag = "1.54.1-cf.emscripten" }
tokio-macros = { git = "https://github.com/guybedford/tokio", tag = "1.54.1-cf.emscripten" }
# With Tokio's `net` feature, until tokio-rs/mio#1969 is in a mio release:
mio = { git = "https://github.com/tokio-rs/mio" }
```

```toml
# .cargo/config.toml
[target.wasm32-unknown-emscripten]
rustflags = ["--cfg=wasm_bindgen_unstable_tokio", "--cfg=tokio_unstable", ...]
```

The event loop is hosted by the JS event loop: it registers its reactor's
readiness and its timer deadlines with the host and runs its tasks from those
callbacks. An export called from JS also runs one batch on the caller's
stack, so the future's first poll is synchronous, as it is for a plain
`async` export.

This needs Emscripten 6.0.12 or later, for its epoll readiness listeners
([emscripten#27547]). Resolving hostnames additionally needs async DNS
([emscripten#27742]), not yet in a release, and a multiplexed host needs the
listener callbacks delivered as a microtask of the task that made the reactor
ready. The `7.0.0-cf.emscripten` tag of [guybedford/emscripten][emscripten-cf]
is Emscripten 7.0.0 plus those two changes; with `emcc` 7.0.0 (emsdk,
Homebrew, ...) already on `PATH`, use its checkout as the frontend over that
toolchain:

```sh
git clone --depth 1 --branch 7.0.0-cf.emscripten https://github.com/guybedford/emscripten
(cd emscripten && ./bootstrap.py)
printf "LLVM_ROOT = '%s'\nBINARYEN_ROOT = '%s'\n" "$(em-config LLVM_ROOT)" "$(em-config BINARYEN_ROOT)" > emscripten/.emscripten
export PATH=$PWD/emscripten:$PATH
```

On an Emscripten older than 6.0.12, timers, `tokio::spawn` and the sync
primitives still work when linked with
`-Clink-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0`, as long as Tokio's `net` feature
is not enabled anywhere in the build: with it the event loop has a reactor to
register, and building it aborts.

[tokio#8484]: https://github.com/tokio-rs/tokio/pull/8484
[emscripten#27547]: https://github.com/emscripten-core/emscripten/pull/27547
[emscripten#27742]: https://github.com/emscripten-core/emscripten/pull/27742
[emscripten-cf]: https://github.com/guybedford/emscripten/tree/7.0.0-cf.emscripten
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

The JSPI configurations here keep `-Cpanic=abort` deliberately: the
combination of [panic unwinding](#panics) with Emscripten's JSPI lifecycle
hooks is not yet exercised by wasm-bindgen's test suite, so it is not
documented as supported. The catch wrappers described under
[`panic=abort`](#panicabort) are still emitted; what is untested is panics
unwinding across a suspended activation. Panics in a `jspi` export trap and
go to Emscripten's `abort()`.

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
root or in spawned tasks.

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
closed).

The runtime stays entered on the thread while its activation is parked, so
only one such invocation can be in flight: a second `jspi, experimental_tokio`
export called before the first settles panics as a nested runtime, in either
mode. An `experimental_tokio` export without `jspi` called meanwhile is
queued, and runs once the parked invocation returns.

The complete configuration for the combination is the union of the Tokio and
JSPI ones above: the JSPI toolchain setup, and Node 25 or newer (Node 24 with
`--experimental-wasm-jspi`) to run. That toolchain is Emscripten 6.0.10, so
it predates the readiness listeners: link with
`-Clink-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0`, and leave Tokio's `net` feature
off, as an `experimental_tokio` export without `jspi` aborts with it.
Crates:

```toml
# Cargo.toml
[patch.crates-io]
tokio = { git = "https://github.com/guybedford/tokio", tag = "1.54.1-cf.emscripten" }
tokio-macros = { git = "https://github.com/guybedford/tokio", tag = "1.54.1-cf.emscripten" }
# With Tokio's `net` feature, until tokio-rs/mio#1969 is in a mio release:
mio = { git = "https://github.com/tokio-rs/mio" }
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
  "-Clink-arg=-sWASM_BINDGEN",
  "-Clink-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0",
  "-Clink-arg=-sJSPI",
  "-Clink-arg=-sREENTRANT_JSPI",
]
```

`-sREENTRANT_JSPI` (which implies `-sJSPI_HOOKS`) is required: every parked
activation keeps its own shadow stack. As in the [JSPI
configuration](#getting-started), `-Cpanic=abort` stays in place.

[emscripten#27698]: https://github.com/emscripten-core/emscripten/pull/27698
[emscripten#27699]: https://github.com/emscripten-core/emscripten/pull/27699
[binaryen-release]: https://github.com/guybedford/binaryen/releases/tag/version_132_jspi_hooks_1

## Limitations

- `wasm-bindgen-test` supports the target via
  `wasm_bindgen_test_configure!(run_in_emscripten)`, but the harness
  currently only verifies the generated bindings; test bodies are compiled,
  not executed.
