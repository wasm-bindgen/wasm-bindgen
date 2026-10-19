# Contributing

See the ["Contributing" section of the `wasm-bindgen`
guide](https://wasm-bindgen.github.io/wasm-bindgen/contributing/index.html).

## Justfile

This project includes a [`justfile`](https://github.com/casey/just) as a convenient store of commonly used commands. The justfile is entirely optional - all tasks can still be run using their underlying commands directly. It simply provides shortcuts for script invocations you would otherwise need to remember or look up.

Available commands:

- `just clippy` - Run clippy linting (accepts optional args)
- `just test` - Run all tests

Run individual tests (all accept test names as args):

- `just test-wasm-bindgen` - Run end to end Node.js tests (accepts optional args for test names)
- `just test-wasm-bindgen-futures` - Run end to end Node.js futures tests (accepts optional args for test names)
- `just test-cli` - Run CLI tests
- `just test-macro` - Run macro tests
- `just test-macro-support` - Run macro support tests
- `just test-ui` - Run UI tests for macros
- `just test-js-sys` - Run the JS Sys tests
- `just test-web-sys` - Run the Web Sys tests

To inspect failed generated tests for `just test-wasm-bindgen`, set `WASM_BINDGEN_KEEP_TEST_BUILD=1` to retain the temporary folder for test output.

Update fixtures:

- `just test-cli-overwrite` - Run CLI tests overwriting reference tests (must be run with the latest `stable` Rust toolchain, matching CI, or you will get spurious unrelated diffs)
- `just test-ui-overwrite` - Overwrite UI test outputs

## Browser Examples

The JSPI browser examples (`jspi`, `jspi-opfs`, `jspi-fetch-streams`) require a
browser with JSPI support. Chrome has JSPI enabled by default since v137, and CI
runs all three examples automatically via the Playwright test suite
(`channel: 'chrome'`, which tracks current stable). They cannot be run via
`cargo test`.

To build and test locally:

```sh
# Build the local CLI first so wasm-pack picks it up instead of a cached version.
cargo build -p wasm-bindgen-cli

# Build a specific example (substitute jspi-fetch-streams as needed).
cd examples/jspi-opfs
PATH="$(git rev-parse --show-toplevel)/target/debug:$PATH" npm run build
cd ..

# Run the Playwright tests for one or all JSPI examples.
PREBUILT_EXAMPLES=1 pnpm exec playwright test -g "jspi"
```

## Release Process

Releases are automated, with maintainers reviewing each step:

1. Run the [Release PR](https://github.com/wasm-bindgen/wasm-bindgen/actions/workflows/release-pr.yml)
   workflow on `main`. It bumps the patch version with `./publish bump`, has
   Claude finalize the changelog, set the schema version if the schema
   changed, and update the README MSRV table if the MSRV changed. It then
   regenerates the CLI reference tests and opens a `Release x.y.z` PR from
   `wasm-bindgen-bot` whose description is the release's changelog section.
   Claude's notes for reviewers are posted as a comment.
2. Review the PR, pushing fixes to it as needed. If more PRs land on `main`
   first, run the workflow again, which regenerates the branch from `main`
   and discards changes pushed to it.
3. Squash-merge the PR. Once CI passes on the merge commit, the
   [Release tag](https://github.com/wasm-bindgen/wasm-bindgen/actions/workflows/release-tag.yml)
   workflow tags it. If CI fails, re-running the failed jobs tags it once they
   pass.
4. CI on the tag builds the binaries into a draft GitHub release with the
   changelog section as its notes, then waits for a member of
   `wasm-bindgen-publish` to approve the `publish` environment.
5. Approval publishes the crates to crates.io, then publishes the GitHub
   release and marks it as the latest.

To release by hand, run `./publish bump`, regenerate the reference tests
(`just test-cli-overwrite`), set `SCHEMA_VERSION` in
`crates/shared/src/lib.rs` to the new version if the schema changed since the
last release (see `crates/shared/src/schema_hash_approval.rs`), verify
`cargo test -p wasm-bindgen-shared` passes, finalize the changelog, and merge
the change. A `wasm-bindgen-publish` member can then push the tag once CI
passes, if the Release tag workflow didn't.
