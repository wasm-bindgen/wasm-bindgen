//! `ts-gen`: Generate `wasm-bindgen` Rust bindings from TypeScript `.d.ts` files.
//!
//! This crate provides both a library API and a CLI tool for converting
//! TypeScript declaration files into Rust `#[wasm_bindgen]` extern blocks.
//!
//! # Pipeline
//!
//! ```text
//! .d.ts files
//!   → oxc_parser (AST)
//!   → First Pass phase 1: collect all type names into TypeRegistry
//!   → First Pass phase 2: populate full IR (resolve references, merge var+interface patterns)
//!   → Assembly: group by ModuleContext, resolve inheritance chains, classify types
//!   → Code Generation: IR → syn::File → prettyplease → .rs files
//! ```

pub mod codegen;
pub mod context;
pub mod external_map;
pub mod ir;
pub mod parse;
pub mod util;

use std::path::Path;

use anyhow::Result;

use crate::context::GlobalContext;

/// Parse `.d.ts` files and return the module + global context.
///
/// Diagnostics are collected on `GlobalContext.diagnostics` — callers should
/// inspect or display them as needed (e.g., `ctx.diagnostics.emit()`).
///
/// External type mappings can be configured on the returned `GlobalContext`
/// before passing it to `codegen::generate`.
pub fn parse(
    paths: &[impl AsRef<Path>],
    lib_name: Option<&str>,
) -> Result<(ir::Module, GlobalContext)> {
    parse::parse_dts_files(paths, lib_name)
}

/// Parse a single `.d.ts` source string and return the module + global context.
///
/// Diagnostics are collected on `GlobalContext.diagnostics` — callers should
/// inspect or display them as needed (e.g., `ctx.diagnostics.emit()`).
pub fn parse_source(source: &str, lib_name: Option<&str>) -> Result<(ir::Module, GlobalContext)> {
    parse::parse_single_source(source, lib_name)
}
