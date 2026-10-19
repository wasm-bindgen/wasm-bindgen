wasm-bindgen generates JS and TypeScript bindings for Rust compiled to Wasm.
The proc macro (`crates/macro`, `crates/macro-support`) encodes each item into
a custom section, and the CLI (`crates/cli-support`) decodes it and emits the
JS glue. `src/` holds the runtime types and the ABI traits that cross the
boundary. Changes often span all three, so check that the macro, the encoding
in `crates/shared` and the CLI side agree.

- User-visible changes need a `CHANGELOG.md` entry under `Unreleased`, linking
  the PR.
- Changes to generated output should update the reference tests in
  `crates/cli/tests/reference` (`.js`, `.d.ts`, `.wat`); unexplained churn in
  those files is worth flagging.
- Behaviour changes should be covered by end-to-end tests in `tests/wasm`
  (Rust with a JS counterpart), or UI tests in `crates/macro/ui-tests` for new
  compile errors.
- Changes to `crates/shared/src/lib.rs` or `crates/shared/src/tys.rs` alter
  the schema; see `crates/shared/src/schema_hash_approval.rs` for when
  `SCHEMA_VERSION` must be bumped.
- `crates/web-sys/src/features` is generated from the WebIDL in
  `crates/web-sys/webidls`; edits should be to the WebIDL, not the output.
- The MSRV is in the root `Cargo.toml`; new syntax or std APIs above it break
  the MSRV build.
- `.github/workflows/bot.yml` deliberately uses the `wasm-bindgen-bot`
  actions at `@main`; that repository is maintained by the same team.
