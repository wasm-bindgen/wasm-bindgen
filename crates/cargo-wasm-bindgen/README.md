# cargo-wasm-bindgen

`cargo-wasm-bindgen` is a dynamic CLI wrapper and version manager for `wasm-bindgen` ecosystem tools and other Cargo-installable binaries. It ensures that the tools you run match the versions defined in your project's dependency graph.

## Features

*   **Version Sync**: Automatically detects and uses the correct version of a tool based on your project's dependencies (using `cargo metadata`).
*   **Arbitrary Packages**: Supports running any tool installable via `cargo install`.
*   **Shortcuts**: easy-to-use aliases for common commands (e.g., `cargo wasm-bindgen build`).
*   **Caching**: Manages isolated installations of tool versions in a dedicated cache directory.

## Installation

```bash
cargo install cargo-wasm-bindgen
```

## Usage

### Shortcuts (Default Mode)

The primary way to use `cargo-wasm-bindgen` is through shortcuts.

```bash
cargo wasm-bindgen <shortcut> [args...]
```

**Default Shortcuts:**

*   `build` -> Runs `wasm-bindgen-build` (from `wasm-bindgen-cli`)
*   `cli` -> Runs `wasm-bindgen` (from `wasm-bindgen-cli`)
*   `test-runner` -> Runs `wasm-bindgen-test-runner` (from `wasm-bindgen-cli`)
*   `wasm2es6js` -> Runs `wasm2es6js` (from `wasm-bindgen-cli`)

**Example:**

```bash
# Runs `wasm-bindgen` matching the version of the `wasm-bindgen` crate in your project
cargo wasm-bindgen cli --out-dir pkg ./target/wasm32-unknown-unknown/debug/app.wasm
```

### Management Commands

*   **`--list`**: List all packages and versions currently installed in the cache.
    ```bash
    cargo wasm-bindgen --list
    ```

*   **`--list-shortcuts`**: List all configured shortcuts and metadata mappings.
    ```bash
    cargo wasm-bindgen --list-shortcuts
    ```

*   **`--create-shortcut`**: Create a new global shortcut.
    ```bash
    # Usage: --create-shortcut <shortcut-name> <install-package> <binary-name>
    cargo wasm-bindgen --create-shortcut my-tool my-crate-name my-binary
    ```

*   **`--delete-shortcut`**: Delete an existing shortcut.
    ```bash
    cargo wasm-bindgen --delete-shortcut my-tool
    ```

### Advanced Usage (`--run`)

You can run arbitrary packages without creating a shortcut using the `--run` command.

**Syntax:**

1.  **Explicit Version:**
    ```bash
    # --run <install-package> --version <version> <binary-name>
    cargo wasm-bindgen --run wasm-bindgen-cli --version 0.2.92 wasm-bindgen
    ```

2.  **Metadata Resolution:**
    ```bash
    # --run <install-package> <version-lookup-package> <binary-name>
    # Looks up the version of `wasm-bindgen` in your project dependencies, 
    # then installs that version of `wasm-bindgen-cli` and runs `wasm-bindgen`.
    cargo wasm-bindgen --run wasm-bindgen-cli wasm-bindgen wasm-bindgen
    ```

3.  **Self-Lookup:**
    ```bash
    # --run <install-package> <binary-name>
    # Looks up the version of `some-tool` in your project dependencies.
    cargo wasm-bindgen --run some-tool some-tool-bin
    ```

## Configuration

Configuration is loaded in the following order (first found wins):

1.  **File defined by `CARGO_WASM_BINDGEN_CONFIG_TOML`** environment variable.
2.  **Global config file**: `$CARGO_WASM_BINDGEN_CACHE_DIR/config.toml` (or `~/.cargo/wasm-bindgen/config.toml`).
3.  **Built-in Defaults**.

### Config File Structure (`config.toml`)

```toml
[shortcuts]
build = { install_package = "wasm-bindgen-cli", binary_name = "wasm-bindgen-build" }
cli = { install_package = "wasm-bindgen-cli", binary_name = "wasm-bindgen" }

[metadata_mappings]
# Maps an install package to the crate name used for version lookup in cargo metadata.
# Required when the CLI tool version should track a library version (e.g. wasm-bindgen).
wasm-bindgen-cli = "wasm-bindgen"
```

## Environment Variables

*   `CARGO_WASM_BINDGEN_CACHE_DIR`: Directory where tools are installed. Defaults to `~/.cargo/wasm-bindgen`.
*   `CARGO_WASM_BINDGEN_CONFIG_TOML`: Path to a custom configuration file.