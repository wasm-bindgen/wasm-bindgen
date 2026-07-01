# Using Node.js

You can build the example with

```sh
$ npm run build:nodejs
```

and test it with

```sh
$ npm run test:nodejs
```

# Using Deno

You can build the example with

```sh
$ npm run build:deno
```

and test it with

```sh
$ npm run test:deno
```

`test:deno` uses `deno run --allow-read` flag because the Wasm file is read during runtime.
This will be fixed when https://github.com/denoland/deno/issues/2552 is resolved.
