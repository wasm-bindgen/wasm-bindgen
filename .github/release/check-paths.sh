#!/usr/bin/env bash
# Fails unless every path on stdin is one a release commit may change.
set -euo pipefail

bad=0
while IFS= read -r path; do
  case "$path" in
    Cargo.toml | crates/*/Cargo.toml | CHANGELOG.md | README.md | \
      crates/shared/src/lib.rs | crates/cli/tests/reference/*) ;;
    *)
      echo "::error::Unexpected change to $path"
      bad=1
      ;;
  esac
done
exit $bad
