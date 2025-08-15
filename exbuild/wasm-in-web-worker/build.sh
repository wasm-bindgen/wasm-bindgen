#!/bin/sh

set -ex

# This example requires to *not* create ES modules, therefore we pass the flag
# `--target no-modules`
wasm-pack build --target no-modules
