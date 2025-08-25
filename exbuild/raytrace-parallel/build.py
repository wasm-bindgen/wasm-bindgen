#!/usr/bin/env python3
import os
import subprocess

root_dir = os.path.dirname(__file__)

# A couple of steps are necessary to get this build working which makes it slightly
# nonstandard compared to most other builds.
#
# * First, the Rust standard library needs to be recompiled with atomics
#   enabled. to do that we use Cargo's unstable `-Zbuild-std` feature.
#
# * Next we need to compile everything with the `atomics` and `bulk-memory`
#   features enabled, ensuring that LLVM will generate atomic instructions,
#   shared memory, passive segments, etc.

os.environ.update(
    {"RUSTFLAGS": "-C target-feature=+atomics"}
)

subprocess.run(
    [
        "cargo",
        "build",
        "--target",
        "wasm32-unknown-unknown",
        "--release",
        "-Zbuild-std=std,panic_abort",
    ],
    cwd=root_dir,
).check_returncode()

# Note the usage of `--target no-modules` here which is required for passing
# the memory import to each Wasm module.
subprocess.run(
    [
        "cargo",
        "run",
        "-p",
        "wasm-bindgen-cli",
        "--",
        os.path.join(
            root_dir,
            "..",
            "..",
            "target",
            "wasm32-unknown-unknown",
            "release",
            "raytrace_parallel.wasm",
        ),
        "--out-dir",
        os.path.join(root_dir, "pkg"),
        "--target",
        "no-modules",
    ],
    cwd=root_dir,
).check_returncode()
