#!/bin/sh

export WASM_BINDGEN_BENCH_RESULT=$(pwd)/target/wbg_bench.json
export WASM_BINGEN_CODSPEED_NAPI=$(pwd)/index.node
export NODE_ARGS="--perf-basic-prof-only-functions"
export CODSPEED_LOG="debug"

cargo bench --target wasm32-unknown-unknown -p js-sys

cargo install --git https://github.com/Spxg/wcodspeed.git
wcodspeed $WASM_BINDGEN_BENCH_RESULT

