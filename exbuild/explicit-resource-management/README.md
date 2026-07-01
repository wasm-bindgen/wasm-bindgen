# Using Explicit Resource Management (via Deno)

You can build the example with

```sh
$ ./build.sh
```

and test it with

```sh
$ deno run --allow-read test.ts
```

The `--allow-read` flag is needed because the Wasm file is read during runtime.
This will be fixed when https://github.com/denoland/deno/issues/2552 is resolved.
