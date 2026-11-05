# Without a Bundler

[View documentation for this example online][dox]

[dox]: https://wasm-bindgen.github.io/wasm-bindgen/examples/without-a-bundler.html

You can build the example locally with:

```
$ wasm-pack build --target web
```

Then serve this directory in your favourite webserver and navigate to `host:port`
to open the index.html in your browser:

```
# static server from https://crates.io/crates/https
http

# or use python
python2 -m SimpleHTTPServer
python3 -m http.server
```
