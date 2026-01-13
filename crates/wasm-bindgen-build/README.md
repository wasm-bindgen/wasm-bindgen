# wasm-bindgen-build

`wasm-bindgen-build` is a CLI tool designed to streamline the process of building Rust WebAssembly crates. It serves as a specialized, configurable alternative to `wasm-pack build`, offering tighter integration with `Cargo.toml` and sensible defaults that keep your project clean.

## Features

- **Clean Output Defaults**: Unlike standard tools that output to a `pkg` directory in your project root, `wasm-bindgen-build` defaults to `target/wasm-bindgen/<profile>/` (e.g., `target/wasm-bindgen/debug/`), mirroring standard Cargo build artifacts.
- **Manifest Configuration**: Persist your build settings directly in `Cargo.toml` under `[package.metadata.wasm-bindgen]`.
- **Flexible Overrides**: CLI arguments always take precedence over configuration files.
- **Full Control**: Boolean flags include inverse options (e.g., `--typescript` and `--no-typescript`), allowing you to easily toggle settings defined in your manifest from the command line.

## Installation

```bash
cargo install wasm-bindgen-cli
```

## Usage

While you can simply run the command in your crate's directory:

```bash
wasm-bindgen-build
```

it is preferred to always run the command via cargo, ensuring the version of wasm-bindgen always matches the version in your crate:

```bash
cargo wasm-bindgen build
```


### Command Line Arguments

`wasm-bindgen-build` supports a wide range of arguments to customize the build process.

**General:**
- `--target <bundler|nodejs|web|no-modules|deno>`: Sets the target environment (default: `bundler`).
- `--out-dir <path>`: Sets the output directory.
- `--out-name <name>`: Sets the output file names (defaults to package name).
- `--scope <scope>`: Sets the npm scope for the package.

**Build Profiles:**
- `--dev`: Create a development build (debug info enabled, optimizations disabled).
- `--release`: Create a release build (optimizations enabled, debug info disabled).
- `--profiling`: Create a profiling build.
- `--profile <name>`: Use a custom profile.

**Feature Toggles (with inverses):**
- `--no-typescript` / `--typescript`: Disable/Enable generation of `.d.ts` files.
- `--weak-refs` / `--no-weak-refs`: Enable/Disable usage of the JS weak references proposal.
- `--reference-types` / `--no-reference-types`: Enable/Disable usage of WebAssembly reference types.
- `--no-pack` / `--pack`: Disable/Enable generation of `package.json`.
- `--no-opt` / `--opt`: Disable/Enable `wasm-opt` optimization.
- `--wasm-opt-version <version>`: Set the `wasm-opt` version (e.g., `125`, defaults to `125`).

### Configuration via Cargo.toml

You can define default build options in your `Cargo.toml`. This is useful for sharing build configurations with your team or CI.

```toml
[package.metadata.wasm-bindgen]
target = "web"
out-dir = "static/pkg"
no-typescript = true
weak-refs = true
```

### Precedence Rules

The configuration is resolved in the following order (highest priority first):
1.  **CLI Arguments**: Flags passed directly to the command (e.g., `wasm-bindgen-build --target nodejs`).
2.  **Cargo.toml Metadata**: Values defined in `[package.metadata.wasm-bindgen]`.
3.  **Built-in Defaults**: 
    - `target`: `bundler`
    - `out-dir`: `target/wasm-bindgen/<profile>`

### Examples

**1. Standard Dev Build**
```bash
wasm-bindgen-build --dev
# Outputs to: target/wasm-bindgen/debug/
```

**2. Production Build for Web**
```bash
wasm-bindgen-build --release --target web
# Outputs to: target/wasm-bindgen/release/
```

**3. Overriding Manifest Configuration**
If your `Cargo.toml` has:
```toml
[package.metadata.wasm-bindgen]
no-typescript = true
```

You can force TypeScript generation for a specific build:
```bash
wasm-bindgen-build --typescript
```
