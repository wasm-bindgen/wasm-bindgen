# wasm-bindgen

Rust/Wasm library for high-level JS interop. Workspace with proc macros, CLI tooling, `js-sys`, `web-sys`, and `wasm-bindgen-futures`.

## Project Structure

- `src/` - Core library
- `crates/` - Sub-crates: `cli`, `macro`, `js-sys`, `web-sys`, `futures`, `test`, `webidl`, etc.
- `tests/` - Integration tests (wasm, headless browser, no-std, worker)
- `guide/` - mdBook documentation

## Build & Test

Uses [`just`](https://github.com/casey/just) as a task runner.

```sh
just clippy          # Lint (all warnings are errors)
just test            # Run all tests

# Individual suites
just test-wasm-bindgen               # End-to-end Node.js tests
just test-wasm-bindgen-futures       # Futures tests
just test-wasm-bindgen-unwind        # Unwind tests
just test-wasm-bindgen-unwind-eh     # Unwind EH tests
just test-cli                        # CLI tests
just test-macro                      # Macro tests
just test-macro-support              # Macro support tests
just test-ui                         # Macro compile-fail tests
just test-js-sys                     # js-sys tests
just test-js-sys-next                # js-sys tests (next)
just test-web-sys                    # web-sys tests
just test-web-sys-next               # web-sys tests (next)
just test-webidl                     # WebIDL tests
just test-webidl-tests               # WebIDL integration tests
just test-webidl-tests-next          # WebIDL integration tests (next)
just test-webidl-tests-unstable      # WebIDL integration tests (unstable)

# Overwrite reference fixtures after intentional changes
just test-cli-overwrite
just test-ui-overwrite
```

Wasm tests target `wasm32-unknown-unknown`; the in-tree test runner is wired up automatically via `.cargo/config.toml`.

Set `WASM_BINDGEN_KEEP_TEST_BUILD=1` to retain the temp build folder on test failure.

## Code Style

- Format: `cargo fmt --all` (enforced in CI)
- Lint: `cargo clippy --all-features --workspace -- -D warnings`
- Use inlined format args: `format!("{x}")` not `format!("{}", x)`
- TOML: formatted with `taplo fmt` (see `taplo.toml`)

Add an entry to `CHANGELOG.md` for any user-facing change.

## PR Process

- PRs must come with sufficient tests
- After posting the PR, add a CHANGELOG.md entry, this is a PR requirement

## Release PR

1. Bump all crate versions:
   ```sh
   rustc publish.rs -o publish && ./publish bump
   ```
2. Update `CHANGELOG.md` — add a new version heading under `## Unreleased`:
   ```
   ## [0.2.X](https://github.com/rustwasm/wasm-bindgen/compare/0.2.<X-1>...0.2.X)
   ```
3. Run the schema test to get the new hash:
   ```sh
   cargo test -p wasm-bindgen-shared
   ```
   If it fails, copy the left hash from the assertion failure and update `APPROVED_SCHEMA_FILE_HASH` in `crates/shared/src/schema_hash_approval.rs`. Re-run to confirm it passes.
4. If the schema was changed (either in previous step, OR via schema-change label on PRs, OR hash change added since last release) bump the `SCHEMA_VERSION` constant in `crates/shared/src/lib.rs` to `"0.2.X"`.
5. Regenerate CLI reference tests:
   ```sh
   just test-cli-overwrite
   ```
6. Commit all changes:
   ```sh
   git add -A && git commit -m "Release 0.2.X"
   ```
7. Post the `Release 0.2.X` PR with the CHANGELOG for the version as the PR description.
