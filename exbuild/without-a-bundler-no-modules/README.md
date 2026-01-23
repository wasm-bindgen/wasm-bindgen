# Without a Bundler Using `--target no-modules`

[View documentation for this example online][dox]

[dox]: https://wasm-bindgen.github.io/wasm-bindgen/examples/without-a-bundler.html#using-the-older---target-no-modules

You can build the example locally with:

```
$ wasm-pack build --target no-modules
```

and then opening `index.html` in a browser should run the example!

Note that this example is in contrast to the [without a bundler][wab] example
which performs a similar purpose except it uses `--target no-modules` instead of
`--target web`. The main difference here is how the shim JS and module are
loaded, where this example uses old-school `script` tags while `--target web`
uses ES modules.

[wab]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/without-a-bundler
