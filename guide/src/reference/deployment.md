# Deploying Rust and WebAssembly

At this point in time deploying Rust and WebAssembly to the web or other
locations unfortunately isn't a trivial task to do. This page hopes to serve
as documentation for the various known options, and as always PRs are welcome
to update this if it's out of date!

The methods of deployment and integration here are primarily tied to the
`--target` flag.

| Value           | Summary                                                    |
|-----------------|------------------------------------------------------------|
| [`bundler`]     | Suitable for loading in bundlers like Webpack              |
| [`web`]         | Directly loadable in a web browser                         |
| [`nodejs`]      | Loadable via `require` as a Node.js CommonJS module        |
| [`deno`]        | Loadable using imports from Deno modules                   |
| [`no-modules`]  | Like `web`, but older and doesn't use ES modules           |
| [`experimental-nodejs-module`]  | Loadable via `import` as a Node.js ESM module. |
| [`module`] | Uses the new source phase imports syntax to obtain the compiled WebAssembly module |

[`bundler`]: #bundlers
[`web`]: #without-a-bundler
[`no-modules`]: #without-a-bundler
[`nodejs`]: #nodejs
[`deno`]: #deno
[`experimental-nodejs-module`]: #nodejs-module
[`module`]: #source-phase-imports

## Bundlers

**`--target bundler`**

The default output of `wasm-bindgen`, or the `bundler` target, assumes a model
where the Wasm module itself is natively an ES module. This model, however, is not
natively implemented in any JS implementation at this time. As a result, to
consume the default output of `wasm-bindgen` you will need a bundler of some
form.

> **Note**: the choice of this default output was done to reflect the trends of
> the JS ecosystem. While tools other than bundlers don't support Wasm files as
> native ES modules today they're all very much likely to in the future!

Currently the only known bundler known to be fully compatible with
`wasm-bindgen` is [webpack]. Most [examples] use webpack, and you can check out
the [hello world example online] to see the details of webpack configuration
necessary.

[webpack]: https://webpack.js.org/
[examples]: ../examples/index.html
[hello world example online]: ../examples/hello-world.html

## Without a Bundler

**`--target web` or `--target no-modules`**

If you're not using a bundler but you're still running code in a web browser,
`wasm-bindgen` still supports this! For this use case you'll want to use the
`--target web` flag. You can check out a [full example][nomex] in the
documentation, but the highlights of this output are:

* When compiling you'll pass `--target web` to `wasm-bindgen`
* The output can natively be included on a web page, and doesn't require any
  further postprocessing. The output is included as an ES module.
* The `--target web` mode is not able to use NPM dependencies.
* You'll want to review the [browser requirements] for `wasm-bindgen` because
  no polyfills will be available.

[nomex]: ../examples/without-a-bundler.html
[rfc1]: https://github.com/rustwasm/rfcs/pull/6
[rfc2]: https://github.com/rustwasm/rfcs/pull/8
[browser requirements]: browser-support.html

The CLI also supports an output mode called `--target no-modules` which is
similar to the `web` target in that it requires manual initialization of the
wasm and is intended to be included in web pages without any further
postprocessing. See the [without a bundler example][nomex] for some more
information about `--target no-modules`.

## Node.js

**`--target nodejs`**

If you're deploying WebAssembly into Node.js (perhaps as an alternative to a
native module), then you'll want to pass the `--target nodejs` flag to `wasm-bindgen`.

Like the "without a bundler" strategy, this method of deployment does not
require any further postprocessing. The generated JS shims can be `require`'d
just like any other Node module (even the `*_bg` Wasm file can be `require`'d
as it has a JS shim generated as well).

Note that this method requires a version of Node.js with WebAssembly support,
which is currently Node 8 and above.

## Node.js Module

**`--target experimental-nodejs-module`**

If you're deploying WebAssembly into Node.js as a JavaScript module,
then you'll want to pass the `--target experimental-nodejs-module` flag to `wasm-bindgen`.

Like the "node" strategy, this method of deployment does not
require any further postprocessing. The generated JS shims can be `import`ed
just like any other Node module.

Note that this method requires a version of Node.js with WebAssembly and module support,
which is currently Node 12 and above.

**Currently experimental. Target is expected to be changed before stabilization.**

## Deno

**`--target deno`**

To deploy WebAssembly to Deno, use the `--target deno` flag.
To then import your module inside deno, use

```ts
// @deno-types="./out/crate_name.d.ts"
import { yourFunction } from "./out/crate_name.js";
```

## Module

**`--target module`**

The `--target module` option generates a new cross-toolchain ES module target, that by default uses the new source phase imports syntax to obtain uninstantiated WebAssembly modules.

[Source phase imports](https://github.com/tc39/proposal-source-phase-imports) allow importing the compiled WebAssembly module (rather than its instance) using the Wasm ESM Integration.

_Note that source phase imports are currently only supported in ESBuild and Node.js 24._

For example:

```js
import source wasmModule from "./module.wasm";

// Example illustrative usage:
const instance = new WebAssembly.Instance(wasmModule, imports);
export function foo () {
  instance.doFoo();
}
```

This replaces the need for `compileStreaming` and platform-specific Wasm-loading paths which previously required separate target implementations.

## NPM

If you'd like to deploy compiled WebAssembly to NPM, then the tool for the job
is [`wasm-pack`]. More information on this coming soon!

[`wasm-pack`]: https://rustwasm.github.io/docs/wasm-pack/
