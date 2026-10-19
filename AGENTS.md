# wasm-bindgen

Rust/Wasm library for high-level JS interop. Workspace with proc macros, CLI
tooling, `js-sys`, `web-sys`, and `wasm-bindgen-futures`.

## Architecture

The proc macro (`crates/macro`, `crates/macro-support`) encodes each item into
a custom section, and the CLI (`crates/cli-support`) decodes it and emits the
JS glue. `src/` holds the runtime types and the ABI traits that cross the
boundary. Changes often span all three, so the macro, the encoding in
`crates/shared` and the CLI side must agree.

## Project Structure

- `src/` - Core library
- `crates/` - Sub-crates: `cli`, `cli-support`, `macro`, `macro-support`,
  `shared`, `js-sys`, `web-sys`, `futures`, `test`, `webidl`, etc.
- `tests/` - Integration tests (wasm, headless browser, no-std, worker)
- `guide/` - mdBook documentation

## Build & Test

Uses [`just`](https://github.com/casey/just) as a task runner.

```sh
just clippy          # Lint (all warnings are errors)
just test            # Run all tests

# Individual suites
just test-wasm-bindgen                   # End-to-end Node.js tests
just test-wasm-bindgen-futures           # Futures tests
just test-wasm-bindgen-unwind            # Unwind tests
just test-wasm-bindgen-unwind-legacy-eh  # Unwind tests (legacy EH)
just test-wasm64                         # wasm64 tests
just test-cli                            # CLI tests
just test-macro                          # Macro tests
just test-macro-support                  # Macro support tests
just test-ui                             # Macro compile-fail tests
just test-js-sys                         # js-sys tests
just test-js-sys-next                    # js-sys tests (next)
just test-web-sys                        # web-sys tests
just test-web-sys-next                   # web-sys tests (next)
just test-webidl                         # WebIDL tests
just test-webidl-tests                   # WebIDL integration tests
just test-webidl-tests-next              # WebIDL integration tests (next)
just test-webidl-tests-unstable          # WebIDL integration tests (unstable)

# Overwrite reference fixtures after intentional changes
just test-cli-overwrite
just test-ui-overwrite
```

Wasm tests target `wasm32-unknown-unknown`; the in-tree test runner is wired
up automatically via `.cargo/config.toml`.

Set `WASM_BINDGEN_KEEP_TEST_BUILD=1` to retain the temp build folder on test
failure.

## Conventions

- User-facing changes need a `CHANGELOG.md` entry under `Unreleased`, linking
  the PR.
- PRs must come with sufficient tests: end-to-end tests in `tests/wasm` (Rust
  with a JS counterpart) for behaviour, UI tests in `crates/macro/ui-tests`
  for new compile errors.
- Changes to generated output update the reference tests in
  `crates/cli/tests/reference` (`.js`, `.d.ts`, `.wat`) via
  `just test-cli-overwrite`, which must run on the latest stable toolchain;
  unexplained churn in those files is a smell.
- Changes to `crates/shared/src/lib.rs` or `crates/shared/src/tys.rs` alter
  the schema; see `crates/shared/src/schema_hash_approval.rs` for when
  `SCHEMA_VERSION` must be bumped.
- `crates/web-sys/src/features` is generated from the WebIDL in
  `crates/web-sys/webidls` (`just generate-web-sys`); edit the WebIDL, not the
  output.
- The MSRV is `rust-version` in the root `Cargo.toml`; new syntax or std APIs
  above it break the MSRV build.

## Code Style

- Format: `cargo fmt --all` (enforced in CI)
- Lint: `cargo clippy --all-features --workspace -- -D warnings`
- Use inlined format args: `format!("{x}")` not `format!("{}", x)`
- TOML: formatted with `taplo fmt` (see `taplo.toml`)

## Release PR

1. Bump all crate versions:
   ```sh
   rustc publish.rs -o publish && ./publish bump
   ```
2. Update `CHANGELOG.md`: add a new version heading under `## Unreleased`:
   ```
   ## [0.2.X](https://github.com/wasm-bindgen/wasm-bindgen/compare/0.2.<X-1>...0.2.X)
   ```
3. Run the schema test to get the new hash:
   ```sh
   cargo test -p wasm-bindgen-shared
   ```
   If it fails, copy the left hash from the assertion failure and update
   `APPROVED_SCHEMA_FILE_HASH` in `crates/shared/src/schema_hash_approval.rs`.
   Re-run to confirm it passes.
4. If the schema changed (in the previous step, via the `schema-change` label
   on PRs, or via a hash change since the last release), bump the
   `SCHEMA_VERSION` constant in `crates/shared/src/lib.rs` to `"0.2.X"`.
5. Regenerate CLI reference tests:
   ```sh
   just test-cli-overwrite
   ```
6. Commit all changes:
   ```sh
   git add -A && git commit -m "Release 0.2.X"
   ```
7. Post the `Release 0.2.X` PR with the CHANGELOG for the version as the PR
   description.
