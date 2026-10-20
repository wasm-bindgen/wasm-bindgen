You are preparing the wasm-bindgen {{version}} release, the release after
{{prev}}. Your working directory is the repository at `main` with
`./publish bump` already applied. You can read any file, but you can only edit
`CHANGELOG.md`, `README.md` and `crates/shared/src/lib.rs`. Maintainers review
everything you change in the release PR.

## 1. Changelog

The `## Unreleased` section of `CHANGELOG.md` collects the entries since
{{prev}}. Turn it into the {{version}} section:

- Insert `## [{{version}}](https://github.com/wasm-bindgen/wasm-bindgen/compare/{{prev}}...{{version}})`
  so that the current entries move under it, and leave an empty
  `## Unreleased` section above it with empty `### Added`, `### Changed`,
  `### Fixed` and `### Removed` subsections, as in previous releases.
- Drop empty subsections from the {{version}} section, keeping the order
  Added, Changed, Fixed, Removed.
- Fix clear mistakes only: typos, broken links, duplicates, and entries under
  the wrong subsection. Don't otherwise reword entries.
- Compare the entries against the commits below. When a commit's user-facing
  change has no entry and its subject makes the change clear, add an entry in
  the style of the surrounding ones, with the PR link on its own line.
  Otherwise list the commit in `notes`. Internal changes such as CI, tests and
  refactors don't need entries.

## 2. Schema version

Read the comment at the top of `crates/shared/src/schema_hash_approval.rs`,
then use the schema file changes below to decide whether the encoded schema
changed since {{prev}}. If it did, `SCHEMA_VERSION` in
`crates/shared/src/lib.rs` must be `"{{version}}"`, so set it if it isn't. If
it didn't, leave the file alone. Change nothing else in `lib.rs`.

## 3. MSRV

If the `rust-version` changes below raise the library MSRV (the root
`Cargo.toml`) or the CLI MSRV (`crates/cli/Cargo.toml`), the MSRV History
table in `README.md` needs a {{version}} row dated {{date}} at the top. If the
row already exists, correct its versions and date. Otherwise leave
`README.md` alone.

## Output

Set `schema_changed` to your schema decision. In `notes`, write short
Markdown for the maintainers reviewing the release PR: why you decided the
schema did or didn't change, commits that may be missing entries, and
anything else you were unsure about. Leave out what you changed, since the
diff shows that.
