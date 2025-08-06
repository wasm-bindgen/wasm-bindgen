# Without a Bundler

[View full source code][code]

[code]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/without-a-bundler

This example shows how the `--target web` flag can be used load code in a
browser directly. For this deployment strategy bundlers like Webpack are not
required. For more information on deployment see the [dedicated
documentation][deployment].

First, you'll need to add `web-sys` to your Cargo.toml.
```toml
[dependencies.web-sys]
version = "0.3.4"
features = [
  'Document',
  'Element',
  'HtmlElement',
  'Node',
  'Window',
]
```

Then, let's take a look at the code and see how when we're using `--target web`
we're not actually losing any functionality!

```rust
{{#include ../../../examples/without-a-bundler/src/lib.rs}}
```

Otherwise the rest of the deployment magic happens in `index.html`:

```html
{{#include ../../../examples/without-a-bundler/index.html}}
```

> **Note**: You cannot directly open `index.html` in your web browser due to [CORS][cors]
> limitations. Instead, you can set up a quick development environment using
> Python's built-in HTTP server:
> ```sh
> wasm-pack build --target web
> python3 -m http.server 8080
> ```
> If you don't have Python installed, you can also use [miniserve][miniserve] which
> is installable via Cargo:
> ```sh
> cargo install miniserve
> miniserve . --index "index.html" -p 8080
> ```

And that's it! Be sure to read up on the [deployment options][deployment] to see
what it means to deploy without a bundler.

[cors]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
[miniserve]: https://crates.io/crates/miniserve
[deployment]: ../reference/deployment.html

## Using the older `--target no-modules`

[View full source code][code-no-modules]

[code-no-modules]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/without-a-bundler-no-modules

The older version of using `wasm-bindgen` without a bundler is to use the
`--target no-modules` flag to the `wasm-bindgen` CLI.

While similar to the newer `--target web`, the `--target no-modules` flag has a
few caveats:

* It does not support [local JS snippets][snippets]
* It does not generate an ES module
* It does not support `--split-linked-modules` outside of a document, e.g.
  inside a worker

With that in mind the main difference is how the wasm/JS code is loaded, and
here's an example of loading the output of `wasm-pack` for the same module as
above.

```html
{{#include ../../../examples/without-a-bundler-no-modules/index.html}}
```

[snippets]: ../reference/js-snippets.html
