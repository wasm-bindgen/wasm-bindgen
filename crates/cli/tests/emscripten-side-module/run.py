#!/usr/bin/env python3
"""Build and execute wasm-bindgen callbacks from an Emscripten side module."""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def executable(value):
    resolved = shutil.which(value)
    if resolved is None:
        raise argparse.ArgumentTypeError(f"executable not found: {value}")
    return str(Path(resolved).resolve())


def run(args, directory):
    fixture = Path(__file__).resolve().parent
    repo = fixture.parents[3]
    side = directory / "side"
    (side / "src").mkdir(parents=True)
    shutil.copyfile(fixture / "lib.rs", side / "src/lib.rs")
    (side / "Cargo.toml").write_text(
        '[package]\nname="side-bindings"\nversion="0.0.0"\nedition="2021"\n'
        '[workspace]\n[lib]\ncrate-type=["cdylib", "rlib"]\n[dependencies]\n'
        f'wasm-bindgen={{path={json.dumps(str(repo))}}}\n'
        f'js-sys={{path={json.dumps(str(repo / "crates/js-sys"))}}}\n'
    )
    (directory / "bin").mkdir()
    suffix = ".exe" if os.name == "nt" else ""
    shutil.copy2(args.cli, directory / "bin" / ("wasm-bindgen" + suffix))
    (directory / "temps").mkdir()
    env = os.environ.copy()
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    env.update(
        RUSTC_WRAPPER="",
        CARGO_TARGET_DIR=str(directory / "target"),
        CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER=args.emcc,
        RUSTFLAGS="-Clink-arg=-sWASM_BINDGEN=auto -Clink-arg=-sENVIRONMENT=node",
        EMCC_DEBUG_SAVE="1",
        EMCC_TEMP_DIR=str(directory / "temps"),
    )
    env["PATH"] = os.pathsep.join([str(directory / "bin"), str(Path(args.node).parent), env["PATH"]])
    if args.rust_toolchain:
        env["RUSTUP_TOOLCHAIN"] = args.rust_toolchain

    def command(*argv, cwd=directory):
        print("Running:", " ".join(map(str, argv)), flush=True)
        subprocess.run(list(map(str, argv)), cwd=cwd, env=env, check=True)

    profile = "release" if args.release else "debug"
    build_args = ["--release"] if args.release else []
    command("cargo", "build", "--target", "wasm32-unknown-emscripten", *build_args, cwd=side)
    side_wasm = directory / f"target/wasm32-unknown-emscripten/{profile}/side_bindings.wasm"
    libraries = list((directory / "temps").glob("emscripten_temp_*/bindgen_out/library_bindgen.js"))
    if len(libraries) != 1:
        raise RuntimeError(f"expected one generated binding library, found {len(libraries)}")
    # Keep the generated JS library so the main module can satisfy the side
    # module's JS imports. The generated bindings are not edited by the test.
    library = libraries[0]
    padding = directory / "padding.wasm"
    optimization = "-O2" if args.release else "-O0"
    command(args.emcc, fixture / "padding.c", optimization, "-sSIDE_MODULE=2", "-o", padding)
    for name, inputs in [("plain", [side_wasm]), ("padded", [padding, side_wasm])]:
        host = directory / f"{name}.mjs"
        command(
            args.emcc, fixture / "main.c", *inputs, "--js-library", library,
            optimization, "-fwasm-exceptions", "-sMAIN_MODULE=2", "-sENVIRONMENT=node",
            "-sMODULARIZE=1", "-sEXPORT_ES6=1", "-sALLOW_MEMORY_GROWTH=1",
            "-sSTACK_SIZE=1048576", "-o", host,
        )
        command(args.node, fixture / "assertions.mjs", host, side_wasm, padding)
    print("EMSCRIPTEN-SIDE-MODULE-BINDINGS-OK", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", required=True, type=executable, help="wasm-bindgen CLI built from this checkout")
    parser.add_argument("--emcc", default="emcc", type=executable)
    parser.add_argument("--node", default="node", type=executable)
    parser.add_argument("--release", action="store_true", help="test optimized Rust and JavaScript output")
    parser.add_argument("--rust-toolchain", help="optional rustup toolchain with the Emscripten target installed")
    parser.add_argument("--work-dir", type=Path, help="retain generated files in a new directory under this path")
    args = parser.parse_args()
    if args.work_dir:
        args.work_dir.mkdir(parents=True, exist_ok=True)
        directory = Path(tempfile.mkdtemp(prefix="side-module-", dir=args.work_dir.resolve()))
        print("Artifacts:", directory, flush=True)
        run(args, directory)
    else:
        with tempfile.TemporaryDirectory(prefix="side-module-") as directory:
            run(args, Path(directory))


if __name__ == "__main__":
    main()
