#!/bin/sh

export WASM_BINDGEN_BENCH_RESULT=$(pwd)/target/wbg_bench.json

cargo bench --target wasm32-unknown-unknown -p js-sys

cargo install --git https://github.com/Spxg/wcodspeed.git
wcodspeed $WASM_BINDGEN_BENCH_RESULT

