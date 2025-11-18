#!/bin/bash

cargo test -p wasm-bindgen-cli --target x86_64-unknown-linux-gnu reference::runtest::test_
