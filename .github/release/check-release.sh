#!/usr/bin/env bash
# Fails unless the staged changes against $1 are ones a release commit may
# make: version bumps, the changelog and README, regenerated reference tests,
# and the schema version with its approved hash.
set -euo pipefail

base=$1
bad=0
fail() {
  echo "::error::$1"
  bad=1
}

# Fails if the file's diff changes any line besides those matching $2.
only_lines() {
  if git diff --cached -U0 "$base" -- "$1" | grep -E '^[-+]' | grep -vE '^(\+\+\+|---) ' | grep -vE "^[-+]$2\$"; then
    fail "$1 has changes besides $3"
  fi
}

while IFS= read -r path; do
  case "$path" in
    Cargo.toml | crates/*/Cargo.toml | CHANGELOG.md | README.md | crates/cli/tests/reference/*) ;;
    crates/shared/src/lib.rs)
      only_lines "$path" 'pub const SCHEMA_VERSION: &str = "[0-9.]+";' SCHEMA_VERSION ;;
    crates/shared/src/schema_hash_approval.rs)
      only_lines "$path" 'const APPROVED_SCHEMA_FILE_HASH: &str = "[0-9]+";' APPROVED_SCHEMA_FILE_HASH ;;
    *) fail "Unexpected change to $path" ;;
  esac
done < <(git diff --cached --name-only --no-renames "$base")
exit $bad
