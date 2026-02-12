//! Transformation pass for generating Wasm catch wrappers.
//!
//! This module generates Wasm wrapper functions that catch JavaScript exceptions
//! using the WebAssembly exception handling proposal with `WebAssembly.JSTag`.
//!
//! When a function is marked with `#[wasm_bindgen(catch)]`, instead of wrapping
//! the import call in a JavaScript `handleError` function, we generate a Wasm
//! wrapper that uses `try_table` (modern EH) or `try`/`catch` (legacy EH) to
//! catch JS exceptions directly in Wasm.
//!
//! The wrapper:
//! 1. Calls the original import inside a try block
//! 2. Catches exceptions using the imported JSTag
//! 3. Stores caught exceptions in the externref table
//! 4. Calls `__wbindgen_exn_store` to signal an exception occurred
//! 5. Returns default values (0, null ref, etc.)

use crate::wit::{NonstandardWitSection, WasmBindgenAux};
use anyhow::Error;
use std::collections::HashMap;
use walrus::ir::*;
use walrus::{FunctionBuilder, FunctionId, LocalId, Module, RefType, TableId, TagId, ValType};

use super::ExceptionHandlingVersion;

/// Intrinsics and IDs needed to generate catch wrappers.
#[derive(Clone, Copy)]
struct CatchContext {
    original_func: FunctionId,
    js_tag: TagId,
    externref_table: TableId,
    heap_alloc: FunctionId,
    exn_store: FunctionId,
    idx_local: LocalId,
    exn_local: LocalId,
}

/// Run the catch handler transformation.
///
/// This finds all imports marked with `catch` and generates Wasm wrapper
/// functions that catch JavaScript exceptions using the appropriate EH mechanism.
///
/// Sets `aux.js_tag` when wrappers are generated.
pub fn run(
    module: &mut Module,
    aux: &mut WasmBindgenAux,
    wit: &NonstandardWitSection,
    eh_version: ExceptionHandlingVersion,
) -> Result<(), Error> {
    if aux.imports_with_catch.is_empty() {
        return Ok(());
    }

    // Get required intrinsics
    let externref_table = aux
        .externref_table
        .ok_or_else(|| anyhow::anyhow!("externref table required for catch wrappers"))?;

    let heap_alloc = aux
        .externref_alloc
        .ok_or_else(|| anyhow::anyhow!("externref alloc required for catch wrappers"))?;

    let exn_store = aux
        .exn_store
        .ok_or_else(|| anyhow::anyhow!("__wbindgen_exn_store required for catch wrappers"))?;

    // Import the JSTag
    let js_tag = import_js_tag(module)?;

    let mut wrappers = HashMap::new();

    // Generate wrappers for each import with catch
    for (_import_id, func_id, adapter_id) in wit.implements.iter() {
        if !aux.imports_with_catch.contains(adapter_id) {
            continue;
        }

        let wrapper_id = generate_catch_wrapper(
            module,
            *func_id,
            js_tag,
            externref_table,
            heap_alloc,
            exn_store,
            eh_version,
        );

        wrappers.insert(*func_id, wrapper_id);
    }

    // Rewrite all calls to the original imports to use the wrappers instead
    rewrite_calls(module, &wrappers)?;

    log::debug!("Catch handler created {} wrappers", wrappers.len());
    aux.js_tag = Some(js_tag);

    Ok(())
}

/// Import the `WebAssembly.JSTag` as a Wasm tag.
fn import_js_tag(module: &mut Module) -> Result<TagId, Error> {
    // JSTag has a single externref parameter (the caught exception)
    let tag_ty = module.types.add(&[ValType::Ref(RefType::EXTERNREF)], &[]);

    // Use the module's helper to add an imported tag
    let (tag_id, _import_id) =
        module.add_import_tag(crate::PLACEHOLDER_MODULE, "__wbindgen_jstag", tag_ty);

    Ok(tag_id)
}

/// Generate a catch wrapper function for the given import.
fn generate_catch_wrapper(
    module: &mut Module,
    original_func: FunctionId,
    js_tag: TagId,
    externref_table: TableId,
    heap_alloc: FunctionId,
    exn_store: FunctionId,
    eh_version: ExceptionHandlingVersion,
) -> FunctionId {
    // Get the original function's type
    let orig_ty = module.funcs.get(original_func).ty();
    let ty = module.types.get(orig_ty);
    let params = ty.params().to_vec();
    let results = ty.results().to_vec();

    // Create the wrapper function with the same signature
    let mut builder = FunctionBuilder::new(&mut module.types, &params, &results);

    // Create locals for parameters
    let param_locals: Vec<LocalId> = params.iter().map(|ty| module.locals.add(*ty)).collect();

    // Create context with intrinsics and scratch locals
    let ctx = CatchContext {
        original_func,
        js_tag,
        externref_table,
        heap_alloc,
        exn_store,
        idx_local: module.locals.add(ValType::I32),
        exn_local: module.locals.add(ValType::Ref(RefType::EXTERNREF)),
    };

    match eh_version {
        ExceptionHandlingVersion::Modern => {
            generate_modern_eh_wrapper(&mut builder, &param_locals, &results, ctx);
        }
        ExceptionHandlingVersion::Legacy => {
            generate_legacy_eh_wrapper(&mut builder, module, &param_locals, &results, ctx);
        }
        ExceptionHandlingVersion::None => {
            unreachable!("generate_catch_wrapper called with ExceptionHandlingVersion::None");
        }
    }

    let wrapper_id = builder.finish(param_locals, &mut module.funcs);

    // Give the wrapper a descriptive name
    let orig_name = module
        .funcs
        .get(original_func)
        .name
        .as_deref()
        .unwrap_or("<unknown>");
    module.funcs.get_mut(wrapper_id).name = Some(format!("{orig_name} catch wrapper"));

    wrapper_id
}

/// Generate wrapper using modern EH (try_table).
///
/// Structure:
/// ```wat
/// (block $catch (result externref)
///   (try_table (catch $jstag $catch)
///     local.get <params>...
///     call $original
///     return
///   )
///   unreachable
/// )
/// ;; Exception path: externref is on stack from catching
/// local.set $exn_local
/// call $heap_alloc
/// local.tee $idx_local
/// local.get $exn_local
/// table.set $externref_table
/// local.get $idx_local
/// call $exn_store
/// <push default return values>
/// ```
fn generate_modern_eh_wrapper(
    builder: &mut FunctionBuilder,
    param_locals: &[LocalId],
    results: &[ValType],
    ctx: CatchContext,
) {
    // Block type for externref result (the caught exception)
    let externref_block_ty: InstrSeqType = ValType::Ref(RefType::EXTERNREF).into();

    // Create the try body sequence
    let try_body_id = builder.dangling_instr_seq(None).id();

    // Create the catch block (receives externref)
    let catch_block_id = builder.dangling_instr_seq(externref_block_ty).id();

    // Build the try body: call original and return on success
    {
        let mut try_body = builder.instr_seq(try_body_id);
        for local in param_locals {
            try_body.local_get(*local);
        }
        try_body.call(ctx.original_func);
        try_body.instr(Return {});
    }

    // Build the catch block: contains try_table + unreachable
    {
        let mut catch_block = builder.instr_seq(catch_block_id);
        catch_block.instr(TryTable {
            seq: try_body_id,
            catches: vec![TryTableCatch::Catch {
                tag: ctx.js_tag,
                label: catch_block_id,
            }],
        });
        catch_block.unreachable();
    }

    // Build function body: catch block + exception handling
    {
        let mut body = builder.func_body();

        // The catch block - on exception, branches here with externref on stack
        body.instr(Block {
            seq: catch_block_id,
        });

        // Exception handling code (externref is on stack from catch)
        emit_catch_handler(&mut body, ctx, results);
    }
}

/// Generate wrapper using legacy EH (try/catch).
///
/// Structure:
/// ```wat
/// (try (result <results>)
///   local.get <params>...
///   call $original
/// catch $jstag
///   ;; externref is on stack from catch
///   local.set $exn_local
///   call $heap_alloc
///   local.tee $idx_local
///   local.get $exn_local
///   table.set $externref_table
///   local.get $idx_local
///   call $exn_store
///   <push default return values>
/// end)
/// ```
fn generate_legacy_eh_wrapper(
    builder: &mut FunctionBuilder,
    module: &mut Module,
    param_locals: &[LocalId],
    results: &[ValType],
    ctx: CatchContext,
) {
    // Block type for results
    let result_block_ty: InstrSeqType = match results.len() {
        0 => None.into(),
        1 => results[0].into(),
        _ => {
            let ty_id = module.types.add(&[], results);
            ty_id.into()
        }
    };

    // Catch handler receives externref and produces results
    // This always needs a multi-value type since it has params
    let catch_params = vec![ValType::Ref(RefType::EXTERNREF)];
    let catch_block_ty: InstrSeqType = module.types.add(&catch_params, results).into();

    // Create the try body
    let try_body_id = builder.dangling_instr_seq(result_block_ty).id();
    {
        let mut try_body = builder.instr_seq(try_body_id);
        for local in param_locals {
            try_body.local_get(*local);
        }
        try_body.call(ctx.original_func);
    }

    // Create the catch handler
    let catch_handler_id = builder.dangling_instr_seq(catch_block_ty).id();
    {
        let mut catch_handler = builder.instr_seq(catch_handler_id);

        // Stack has externref from caught exception
        emit_catch_handler(&mut catch_handler, ctx, results);
    }

    // Add try instruction to function body
    {
        let mut body = builder.func_body();
        body.instr(Try {
            seq: try_body_id,
            catches: vec![LegacyCatch::Catch {
                tag: ctx.js_tag,
                handler: catch_handler_id,
            }],
        });
    }
}

/// Emit the exception handling code that stores the caught exception and returns defaults.
///
/// Expects the caught externref to be on the stack. Emits code to:
/// 1. Store the externref in the externref table
/// 2. Call __wbindgen_exn_store with the table index
/// 3. Push default return values
fn emit_catch_handler(
    builder: &mut walrus::InstrSeqBuilder,
    ctx: CatchContext,
    results: &[ValType],
) {
    builder.local_set(ctx.exn_local);
    builder.call(ctx.heap_alloc);
    builder.local_tee(ctx.idx_local);
    builder.local_get(ctx.exn_local);
    builder.table_set(ctx.externref_table);
    builder.local_get(ctx.idx_local);
    builder.call(ctx.exn_store);
    push_default_values(builder, results);
}

/// Push default values for the given result types onto the stack.
fn push_default_values(builder: &mut walrus::InstrSeqBuilder, results: &[ValType]) {
    for ty in results {
        match ty {
            ValType::I32 => {
                builder.i32_const(0);
            }
            ValType::I64 => {
                builder.i64_const(0);
            }
            ValType::F32 => {
                builder.f32_const(0.0);
            }
            ValType::F64 => {
                builder.f64_const(0.0);
            }
            ValType::V128 => {
                panic!("v128 return type in catch wrapper not implemented");
            }
            ValType::Ref(ref_ty) => {
                builder.ref_null(*ref_ty);
            }
        }
    }
}

/// Rewrite all calls to original import functions to use the wrappers instead.
fn rewrite_calls(
    module: &mut Module,
    wrappers: &HashMap<FunctionId, FunctionId>,
) -> Result<(), Error> {
    if wrappers.is_empty() {
        return Ok(());
    }

    // Get the set of wrapper function IDs to avoid rewriting calls within wrappers
    let wrapper_ids: std::collections::HashSet<_> = wrappers.values().copied().collect();

    // Visit all local functions and rewrite calls
    for (func_id, func) in module.funcs.iter_local_mut() {
        // Don't rewrite calls within the wrappers themselves
        if wrapper_ids.contains(&func_id) {
            continue;
        }

        let entry = func.entry_block();
        dfs_pre_order_mut(&mut CallRewriter { wrappers }, func, entry);
    }

    Ok(())
}

struct CallRewriter<'a> {
    wrappers: &'a HashMap<FunctionId, FunctionId>,
}

impl VisitorMut for CallRewriter<'_> {
    fn start_instr_seq_mut(&mut self, seq: &mut InstrSeq) {
        for (instr, _) in seq.instrs.iter_mut() {
            let func = match instr {
                Instr::Call(Call { func }) => func,
                Instr::ReturnCall(ReturnCall { func }) => func,
                _ => continue,
            };
            if let Some(wrapper) = self.wrappers.get(func) {
                *func = *wrapper;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use walrus::ModuleConfig;

    fn parse_wat(wat: &str) -> walrus::Module {
        let wasm = wat::parse_str(wat).unwrap();
        ModuleConfig::new()
            .generate_producers_section(false)
            .parse(&wasm)
            .unwrap()
    }

    #[test]
    fn test_import_js_tag() {
        let wat = r#"
            (module
                (func $foo)
                (export "foo" (func $foo))
            )
        "#;
        let mut module = parse_wat(wat);

        // Should have no tags initially
        assert_eq!(module.tags.iter().count(), 0);

        // Import the JS tag
        let tag_id = import_js_tag(&mut module).unwrap();

        // Should now have one tag
        assert_eq!(module.tags.iter().count(), 1);

        // The tag should be an import
        let tag = module.tags.get(tag_id);
        assert!(matches!(tag.kind, walrus::TagKind::Import(_)));

        // Check the import exists
        let import = module.imports.iter().find(|i| i.name == "__wbindgen_jstag");
        assert!(import.is_some());
        let import = import.unwrap();
        assert_eq!(import.module, crate::PLACEHOLDER_MODULE);
    }

    #[test]
    fn test_generate_catch_wrapper_modern() {
        let wat = r#"
            (module
                ;; A simple imported function
                (import "env" "my_import" (func $my_import (param i32) (result i32)))

                ;; Externref table for storing caught exceptions
                (table $externrefs 128 externref)

                ;; Heap alloc function (returns index)
                (func $heap_alloc (result i32)
                    i32.const 42
                )

                ;; Exception store function
                (func $exn_store (param i32))

                (export "my_import" (func $my_import))
                (export "__externref_table" (table $externrefs))
                (export "heap_alloc" (func $heap_alloc))
                (export "exn_store" (func $exn_store))
            )
        "#;
        let mut module = parse_wat(wat);

        // Get the import function
        let import_func = module
            .exports
            .iter()
            .find(|e| e.name == "my_import")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        // Get the table
        let table = module
            .exports
            .iter()
            .find(|e| e.name == "__externref_table")
            .and_then(|e| match e.item {
                walrus::ExportItem::Table(t) => Some(t),
                _ => None,
            })
            .unwrap();

        // Get the helper functions
        let heap_alloc = module
            .exports
            .iter()
            .find(|e| e.name == "heap_alloc")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let exn_store = module
            .exports
            .iter()
            .find(|e| e.name == "exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        // Import JSTag
        let js_tag = import_js_tag(&mut module).unwrap();

        // Count functions before
        let func_count_before = module.funcs.iter().count();

        // Generate the catch wrapper
        let wrapper_id = generate_catch_wrapper(
            &mut module,
            import_func,
            js_tag,
            table,
            heap_alloc,
            exn_store,
            super::super::ExceptionHandlingVersion::Modern,
        );

        // Should have created a new function
        assert_eq!(module.funcs.iter().count(), func_count_before + 1);

        // The wrapper should have a name
        let wrapper = module.funcs.get(wrapper_id);
        assert!(wrapper.name.as_ref().unwrap().contains("catch wrapper"));
    }

    #[test]
    fn test_generate_catch_wrapper_legacy() {
        let wat = r#"
            (module
                ;; A simple imported function that returns nothing
                (import "env" "my_void_import" (func $my_void_import))

                ;; Externref table
                (table $externrefs 128 externref)

                ;; Heap alloc function
                (func $heap_alloc (result i32)
                    i32.const 42
                )

                ;; Exception store function
                (func $exn_store (param i32))

                (export "my_void_import" (func $my_void_import))
                (export "__externref_table" (table $externrefs))
                (export "heap_alloc" (func $heap_alloc))
                (export "exn_store" (func $exn_store))
            )
        "#;
        let mut module = parse_wat(wat);

        // Get the import function
        let import_func = module
            .exports
            .iter()
            .find(|e| e.name == "my_void_import")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        // Get the table
        let table = module
            .exports
            .iter()
            .find(|e| e.name == "__externref_table")
            .and_then(|e| match e.item {
                walrus::ExportItem::Table(t) => Some(t),
                _ => None,
            })
            .unwrap();

        // Get the helper functions
        let heap_alloc = module
            .exports
            .iter()
            .find(|e| e.name == "heap_alloc")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let exn_store = module
            .exports
            .iter()
            .find(|e| e.name == "exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        // Import JSTag
        let js_tag = import_js_tag(&mut module).unwrap();

        // Count functions before
        let func_count_before = module.funcs.iter().count();

        // Generate the catch wrapper using legacy EH
        let wrapper_id = generate_catch_wrapper(
            &mut module,
            import_func,
            js_tag,
            table,
            heap_alloc,
            exn_store,
            super::super::ExceptionHandlingVersion::Legacy,
        );

        // Should have created a new function
        assert_eq!(module.funcs.iter().count(), func_count_before + 1);

        // The wrapper should have a name
        let wrapper = module.funcs.get(wrapper_id);
        assert!(wrapper.name.as_ref().unwrap().contains("catch wrapper"));
    }

    #[test]
    fn test_run_with_imports_with_catch() {
        use crate::wit::{AdapterId, NonstandardWitSection, WasmBindgenAux};
        use std::collections::HashSet;

        // Create a module with an import, EH instructions, and required intrinsics
        let wat = r#"
            (module
                ;; Import that we want to wrap with catch
                (import "env" "might_throw" (func $might_throw (result i32)))

                ;; Externref table
                (table $externrefs 128 externref)

                ;; Heap alloc function
                (func $heap_alloc (result i32)
                    i32.const 42
                )

                ;; Exception store function
                (func $exn_store (param i32))

                ;; A function that uses legacy EH (so we detect EH is available)
                (func $uses_eh
                    try
                        i32.const 1
                        drop
                    catch_all
                    end
                )

                ;; A function that calls the import
                (func $caller (result i32)
                    call $might_throw
                )

                (export "__externref_table_alloc" (func $heap_alloc))
                (export "__wbindgen_exn_store" (func $exn_store))
                (export "caller" (func $caller))
            )
        "#;
        let mut module = parse_wat(wat);

        // Find the import
        let import_id = module
            .imports
            .iter()
            .find(|i| i.name == "might_throw")
            .map(|i| i.id())
            .unwrap();

        let import_func_id = match module.imports.get(import_id).kind {
            walrus::ImportKind::Function(f) => f,
            _ => panic!("expected function import"),
        };

        // Get the table
        let table = module.tables.iter().next().unwrap().id();

        // Get heap_alloc and exn_store
        let heap_alloc = module
            .exports
            .iter()
            .find(|e| e.name == "__externref_table_alloc")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let exn_store = module
            .exports
            .iter()
            .find(|e| e.name == "__wbindgen_exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        // Create aux with imports_with_catch
        let adapter_id = AdapterId(0);
        let mut imports_with_catch = HashSet::new();
        imports_with_catch.insert(adapter_id);

        let mut aux = WasmBindgenAux {
            imports_with_catch,
            externref_table: Some(table),
            externref_alloc: Some(heap_alloc),
            exn_store: Some(exn_store),
            ..Default::default()
        };

        // Create wit section with implements mapping
        let mut wit = NonstandardWitSection::default();
        wit.implements.push((import_id, import_func_id, adapter_id));

        // Verify EH is detected
        let eh_version = super::super::detect_exception_handling_version(&module);
        assert_eq!(eh_version, super::super::ExceptionHandlingVersion::Legacy);

        // Run the transform
        run(&mut module, &mut aux, &wit, eh_version).unwrap();

        // JSTag should be set in aux
        assert!(aux.js_tag.is_some());

        // JSTag should be imported
        let jstag_import = module.imports.iter().find(|i| i.name == "__wbindgen_jstag");
        assert!(jstag_import.is_some());
    }

    #[test]
    fn test_run_skips_when_no_imports_with_catch() {
        use crate::wit::{NonstandardWitSection, WasmBindgenAux};

        let wat = r#"
            (module
                (func $foo
                    try
                        i32.const 1
                        drop
                    catch_all
                    end
                )
                (export "foo" (func $foo))
            )
        "#;
        let mut module = parse_wat(wat);

        // Create aux with empty imports_with_catch
        let mut aux = WasmBindgenAux::default();
        let wit = NonstandardWitSection::default();

        let eh_version = super::super::detect_exception_handling_version(&module);
        assert_eq!(eh_version, super::super::ExceptionHandlingVersion::Legacy);

        // Run should be a no-op since no imports need catching
        run(&mut module, &mut aux, &wit, eh_version).unwrap();
        assert!(aux.js_tag.is_none());
    }

    #[test]
    fn test_wrapper_contains_try_instruction_legacy() {
        use walrus::ir::Visitor;

        let wat = r#"
            (module
                (import "env" "my_import" (func $my_import (param i32) (result i32)))
                (table $externrefs 128 externref)
                (func $heap_alloc (result i32) i32.const 42)
                (func $exn_store (param i32))
                (export "my_import" (func $my_import))
                (export "__externref_table" (table $externrefs))
                (export "heap_alloc" (func $heap_alloc))
                (export "exn_store" (func $exn_store))
            )
        "#;
        let mut module = parse_wat(wat);

        let import_func = module
            .exports
            .iter()
            .find(|e| e.name == "my_import")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let table = module
            .exports
            .iter()
            .find(|e| e.name == "__externref_table")
            .and_then(|e| match e.item {
                walrus::ExportItem::Table(t) => Some(t),
                _ => None,
            })
            .unwrap();

        let heap_alloc = module
            .exports
            .iter()
            .find(|e| e.name == "heap_alloc")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let exn_store = module
            .exports
            .iter()
            .find(|e| e.name == "exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let js_tag = import_js_tag(&mut module).unwrap();

        let wrapper_id = generate_catch_wrapper(
            &mut module,
            import_func,
            js_tag,
            table,
            heap_alloc,
            exn_store,
            super::super::ExceptionHandlingVersion::Legacy,
        );

        // Verify the wrapper contains a Try instruction
        struct TryFinder {
            found_try: bool,
        }
        impl<'a> Visitor<'a> for TryFinder {
            fn visit_try(&mut self, _: &Try) {
                self.found_try = true;
            }
        }

        let wrapper = module.funcs.get(wrapper_id);
        if let walrus::FunctionKind::Local(local) = &wrapper.kind {
            let mut finder = TryFinder { found_try: false };
            walrus::ir::dfs_in_order(&mut finder, local, local.entry_block());
            assert!(finder.found_try, "wrapper should contain a Try instruction");
        } else {
            panic!("wrapper should be a local function");
        }
    }

    #[test]
    fn test_wrapper_contains_try_table_instruction_modern() {
        use walrus::ir::Visitor;

        let wat = r#"
            (module
                (import "env" "my_import" (func $my_import (param i32) (result i32)))
                (table $externrefs 128 externref)
                (func $heap_alloc (result i32) i32.const 42)
                (func $exn_store (param i32))
                (export "my_import" (func $my_import))
                (export "__externref_table" (table $externrefs))
                (export "heap_alloc" (func $heap_alloc))
                (export "exn_store" (func $exn_store))
            )
        "#;
        let mut module = parse_wat(wat);

        let import_func = module
            .exports
            .iter()
            .find(|e| e.name == "my_import")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let table = module
            .exports
            .iter()
            .find(|e| e.name == "__externref_table")
            .and_then(|e| match e.item {
                walrus::ExportItem::Table(t) => Some(t),
                _ => None,
            })
            .unwrap();

        let heap_alloc = module
            .exports
            .iter()
            .find(|e| e.name == "heap_alloc")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let exn_store = module
            .exports
            .iter()
            .find(|e| e.name == "exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let js_tag = import_js_tag(&mut module).unwrap();

        let wrapper_id = generate_catch_wrapper(
            &mut module,
            import_func,
            js_tag,
            table,
            heap_alloc,
            exn_store,
            super::super::ExceptionHandlingVersion::Modern,
        );

        // Verify the wrapper contains a TryTable instruction
        struct TryTableFinder {
            found_try_table: bool,
        }
        impl<'a> Visitor<'a> for TryTableFinder {
            fn visit_try_table(&mut self, _: &TryTable) {
                self.found_try_table = true;
            }
        }

        let wrapper = module.funcs.get(wrapper_id);
        if let walrus::FunctionKind::Local(local) = &wrapper.kind {
            let mut finder = TryTableFinder {
                found_try_table: false,
            };
            walrus::ir::dfs_in_order(&mut finder, local, local.entry_block());
            assert!(
                finder.found_try_table,
                "wrapper should contain a TryTable instruction"
            );
        } else {
            panic!("wrapper should be a local function");
        }
    }

    #[test]
    fn test_rewrite_calls() {
        let wat = r#"
            (module
                (func $original (result i32)
                    i32.const 1
                )
                (func $wrapper (result i32)
                    i32.const 2
                )
                (func $caller (result i32)
                    call $original
                )
                (export "original" (func $original))
                (export "wrapper" (func $wrapper))
                (export "caller" (func $caller))
            )
        "#;
        let mut module = parse_wat(wat);

        let original = module
            .exports
            .iter()
            .find(|e| e.name == "original")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let wrapper = module
            .exports
            .iter()
            .find(|e| e.name == "wrapper")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        let caller = module
            .exports
            .iter()
            .find(|e| e.name == "caller")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            })
            .unwrap();

        // Create wrapper map
        let mut wrappers = HashMap::new();
        wrappers.insert(original, wrapper);

        // Rewrite calls
        rewrite_calls(&mut module, &wrappers).unwrap();

        // Check that the caller now calls wrapper instead of original
        let caller_func = module.funcs.get(caller);
        if let walrus::FunctionKind::Local(local) = &caller_func.kind {
            let mut found_call = false;
            for (instr, _) in local.block(local.entry_block()).instrs.iter() {
                if let Instr::Call(Call { func }) = instr {
                    assert_eq!(*func, wrapper, "call should be rewritten to wrapper");
                    found_call = true;
                }
            }
            assert!(found_call, "should have found a call instruction");
        } else {
            panic!("expected local function");
        }
    }
}
