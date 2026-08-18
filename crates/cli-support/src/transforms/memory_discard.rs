//! Replaces the `env.__wbindgen_memory_discard` import with a local trampoline whose
//! body is the `memory.discard` instruction from the memory-control proposal.
//!
//! Experimental and subject to change, gated behind the
//! `--experimental-memory-discard` CLI flag since the emitted instruction
//! requires an engine supporting the memory-control proposal.
//!
//! LLVM has no way to emit `memory.discard` directly, so allocators that
//! release physical pages back to the host on wasm32-unknown-unknown (such as
//! jemalloc's purging path) declare this import instead:
//!
//! ```c
//! __attribute__((import_module("env"), import_name("__wbindgen_memory_discard")))
//! extern void __wbindgen_memory_discard(void *addr, size_t len);
//! ```
//!
//! The import is deleted and its function id becomes a local function, so
//! nothing survives to instantiation and page discard remains a pure wasm
//! operation. `memory.discard` traps on non-page-aligned ranges and has
//! zeroing semantics, matching `madvise(MADV_DONTNEED)` on an overcommitting
//! Linux.

use crate::wasm_conventions;
use anyhow::{bail, Result};
use walrus::ir::MemoryDiscard;
use walrus::{ImportKind, Module, ValType};

pub const IMPORT_MODULE: &str = "env";
pub const IMPORT_NAME: &str = "__wbindgen_memory_discard";

pub fn run(module: &mut Module, enabled: bool) -> Result<()> {
    let fid = module.imports.iter().find_map(|impt| match impt.kind {
        ImportKind::Function(id) if impt.module == IMPORT_MODULE && impt.name == IMPORT_NAME => {
            Some(id)
        }
        _ => None,
    });
    let Some(fid) = fid else {
        return Ok(());
    };
    if !enabled {
        bail!(
            "module imports `{IMPORT_MODULE}.{IMPORT_NAME}`, which requires the experimental \
             `--experimental-memory-discard` flag; the emitted `memory.discard` instruction \
             needs an engine supporting the wasm memory-control proposal"
        );
    }

    let memory = wasm_conventions::get_memory(module)?;
    // Address and length follow the memory's index type.
    let ptr = if module.memories.get(memory).memory64 {
        ValType::I64
    } else {
        ValType::I32
    };
    let ty = module.types.get(module.funcs.get(fid).ty());
    if ty.params() != [ptr; 2] || !ty.results().is_empty() {
        bail!(
            "`{IMPORT_MODULE}.{IMPORT_NAME}` import must have type [{ptr} {ptr}] -> [], found \
             [{}] -> [{}]",
            ty.params()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
            ty.results()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        );
    }

    module.replace_imported_func(fid, |(body, args)| {
        body.local_get(args[0])
            .local_get(args[1])
            .instr(MemoryDiscard { memory });
    })?;
    module.funcs.get_mut(fid).name = Some(IMPORT_NAME.to_string());
    log::debug!("Replaced `{IMPORT_MODULE}.{IMPORT_NAME}` with a `memory.discard` trampoline");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms::parse_wat;
    use walrus::ir::{dfs_in_order, Instr, Visitor};
    use walrus::FunctionKind;

    fn validate(module: &mut walrus::Module) {
        let features =
            wasmparser::WasmFeatures::default() | wasmparser::WasmFeatures::MEMORY_CONTROL;
        wasmparser::Validator::new_with_features(features)
            .validate_all(&module.emit_wasm())
            .expect("transformed module should validate");
    }

    fn count_discards(module: &walrus::Module) -> usize {
        struct Scan(usize);
        impl<'instr> Visitor<'instr> for Scan {
            fn visit_instr(&mut self, instr: &Instr, _: &walrus::InstrLocId) {
                if matches!(instr, Instr::MemoryDiscard(_)) {
                    self.0 += 1;
                }
            }
        }
        let mut scan = Scan(0);
        for func in module.funcs.iter() {
            if let FunctionKind::Local(local) = &func.kind {
                dfs_in_order(&mut scan, local, local.entry_block());
            }
        }
        scan.0
    }

    #[test]
    fn replaces_import_with_discard_trampoline() {
        let mut module = parse_wat(
            r#"
            (module
                (import "env" "__wbindgen_memory_discard" (func $discard (param i32 i32)))
                (memory 1)
                (export "memory" (memory 0))
                (func $purge (param i32 i32)
                    local.get 0
                    local.get 1
                    call $discard
                )
                (export "purge" (func $purge))
            )
        "#,
        );

        run(&mut module, true).unwrap();

        assert_eq!(module.imports.iter().count(), 0);
        assert_eq!(count_discards(&module), 1);
        validate(&mut module);
    }

    #[test]
    fn memory64_uses_i64_params() {
        let mut module = parse_wat(
            r#"
            (module
                (import "env" "__wbindgen_memory_discard" (func $discard (param i64 i64)))
                (memory i64 1)
                (export "memory" (memory 0))
                (func $purge (param i64 i64)
                    local.get 0
                    local.get 1
                    call $discard
                )
                (export "purge" (func $purge))
            )
        "#,
        );

        run(&mut module, true).unwrap();

        assert_eq!(module.imports.iter().count(), 0);
        assert_eq!(count_discards(&module), 1);
        validate(&mut module);
    }

    #[test]
    fn no_import_is_a_no_op() {
        for enabled in [true, false] {
            let mut module = parse_wat(
                r#"
                (module
                    (memory 1)
                    (export "memory" (memory 0))
                )
            "#,
            );

            run(&mut module, enabled).unwrap();

            assert_eq!(count_discards(&module), 0);
        }
    }

    #[test]
    fn import_without_flag_errors() {
        let mut module = parse_wat(
            r#"
            (module
                (import "env" "__wbindgen_memory_discard" (func (param i32 i32)))
                (memory 1)
            )
        "#,
        );

        let err = run(&mut module, false).unwrap_err().to_string();
        assert!(err.contains("--experimental-memory-discard"), "{err}");
    }

    #[test]
    fn wrong_signature_errors() {
        let mut module = parse_wat(
            r#"
            (module
                (import "env" "__wbindgen_memory_discard" (func (param i32 i32) (result i32)))
                (memory 1)
                (export "memory" (memory 0))
            )
        "#,
        );

        let err = run(&mut module, true).unwrap_err().to_string();
        assert!(err.contains("must have type [i32 i32] -> []"), "{err}");
    }

    #[test]
    fn memory64_rejects_i32_signature() {
        let mut module = parse_wat(
            r#"
            (module
                (import "env" "__wbindgen_memory_discard" (func (param i32 i32)))
                (memory i64 1)
                (export "memory" (memory 0))
            )
        "#,
        );

        let err = run(&mut module, true).unwrap_err().to_string();
        assert!(err.contains("must have type [i64 i64] -> []"), "{err}");
    }

    #[test]
    fn errors_without_a_memory() {
        // The import is present (and the flag is enabled), but the module
        // declares no memory at all, so `wasm_conventions::get_memory` should
        // fail and that failure should propagate out of `run`.
        let mut module = parse_wat(
            r#"
            (module
                (import "env" "__wbindgen_memory_discard" (func (param i32 i32)))
            )
        "#,
        );

        let err = run(&mut module, true).unwrap_err().to_string();
        assert!(err.contains("does not have a memory"), "{err}");
    }

    #[test]
    fn errors_with_multiple_memories() {
        // Build the module via the walrus API directly rather than `.wat`,
        // since the text format's multi-memory syntax requires enabling a
        // wasm proposal `wat::parse_str` doesn't have turned on by default.
        let mut config = walrus::ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = walrus::Module::with_config(config);

        module.memories.add_local(false, false, 1, None, None);
        module.memories.add_local(false, false, 1, None, None);

        let ty = module.types.add(&[ValType::I32, ValType::I32], &[]);
        module.add_import_func(IMPORT_MODULE, IMPORT_NAME, ty);

        let err = run(&mut module, true).unwrap_err().to_string();
        assert!(
            err.contains("expected a single memory, found multiple"),
            "{err}"
        );
    }
}
