//! Management of wasm-bindgen descriptor functions.
//!
//! The purpose of this module is to basically execute a pass on a raw wasm
//! module that just came out of the compiler. The pass will execute all
//! relevant descriptor functions contained in the module which wasm-bindgen
//! uses to convey type information here, to the CLI.
//!
//! All descriptor functions are removed after this pass runs and in their stead
//! a new custom section, defined in this module, is inserted into the
//! `walrus::Module` which contains all the results of all the descriptor
//! functions.

use crate::descriptor::{Descriptor, GenericImportKey};
use crate::interpreter::Interpreter;
use anyhow::{bail, Error};
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, FunctionId, Module, TypedCustomSectionId};

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    /// Per-monomorphisation imports discovered via the
    /// `__wbindgen_describe_generic_import` marker. Keyed by the
    /// `(key, signature)` pair so that two distinct generic imports sharing an
    /// identical concrete signature don't collapse into a single manufactured
    /// binding. See [`GenericImportKey`] for the two kinds of key.
    pub generic_imports: HashMap<(GenericImportKey, Descriptor), Vec<FunctionId>>,
}

pub type WasmBindgenDescriptorsSectionId = TypedCustomSectionId<WasmBindgenDescriptorsSection>;

/// Execute all `__wbindgen_describe_*` functions in a module, inserting a
/// custom section which represents the executed value of each descriptor.
///
/// Afterwards this will delete all descriptor functions from the module.
pub fn execute(module: &mut Module) -> Result<WasmBindgenDescriptorsSectionId, Error> {
    let mut section = WasmBindgenDescriptorsSection::default();
    let mut interpreter = Interpreter::new(module)?;

    section.execute_exports(module, &mut interpreter)?;
    section.execute_generic_imports(module, &mut interpreter)?;

    Ok(module.customs.add(section))
}

impl WasmBindgenDescriptorsSection {
    fn execute_exports(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        let mut to_remove = Vec::new();

        if let Some(id) = interpreter.skip_interpret() {
            to_remove.push(id);
        }

        for export in module.exports.iter() {
            let prefix = "__wbindgen_describe_";
            if !export.name.starts_with(prefix) {
                continue;
            }
            let id = match export.item {
                walrus::ExportItem::Function(id) => id,
                _ => panic!("{} export not a function", export.name),
            };
            // Interpret descriptor with 0 args (export descriptors shouldn't take any).
            let d = interpreter.interpret_descriptor(id, module);
            let name = &export.name[prefix.len()..];
            let descriptor = Descriptor::decode(d);
            self.descriptors.insert(name.to_string(), descriptor);
            to_remove.push(export.id());
        }

        for id in to_remove {
            module.exports.delete(id);
        }
        Ok(())
    }

    /// Discover per-monomorphisation imports (generic imports and `wbg_cast`
    /// identity adapters).
    ///
    /// It finds every function that calls the
    /// `__wbindgen_describe_generic_import` marker, interprets each to recover
    /// its `(key, concrete signature)`, and groups the originating function ids
    /// by that key. See [`GenericImportKey`] for what the key distinguishes.
    fn execute_generic_imports(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        use walrus::ir::*;

        let wbindgen_describe_generic_import = match interpreter.describe_generic_import_id() {
            Some(i) => i,
            None => return Ok(()),
        };

        let mut generic_funcs = Vec::new();
        for (func_id, local) in module.funcs.iter_local() {
            let mut find = FindDescribeGenericImport {
                wbindgen_describe_generic_import,
                calls: 0,
            };
            dfs_in_order(&mut find, local, local.entry_block());
            if find.calls > 0 {
                generic_funcs.push((func_id, find.calls));
            }
        }
        for (func_id, calls) in generic_funcs {
            // `interpret_descriptor` stops at the first marker call, so a second
            // one in the same function would be silently dropped and every call
            // site bound to it mis-bound. That should be impossible — the macro
            // emits one `#[inline(never)]` shim per monomorphisation — but
            // function merging or outlining in LLVM could in principle fuse two
            // of them, so refuse rather than miscompile.
            if calls > 1 {
                bail!(
                    "function {} contains {calls} calls to \
                     `__wbindgen_describe_generic_import`, but exactly one was expected. \
                     Each monomorphisation must live in its own `#[inline(never)]` shim; \
                     if two were merged into one Wasm function only the first would be \
                     bound. This is a wasm-bindgen bug, please report it.",
                    describe_func(module, func_id),
                );
            }

            let descriptor = interpreter.interpret_descriptor(func_id, module);
            let (key, descriptor) = Descriptor::decode_generic_import(descriptor);
            self.generic_imports
                .entry((key, descriptor))
                .or_default()
                .push(func_id);
        }

        return Ok(());

        /// Best-effort human-readable identification of a function for error
        /// messages: the name if the module has one, else the raw index.
        fn describe_func(module: &Module, id: FunctionId) -> String {
            match &module.funcs.get(id).name {
                Some(name) => format!("`{name}`"),
                None => format!("#{:?}", id.index()),
            }
        }

        struct FindDescribeGenericImport {
            wbindgen_describe_generic_import: FunctionId,
            calls: usize,
        }

        impl Visitor<'_> for FindDescribeGenericImport {
            fn visit_call(&mut self, call: &Call) {
                if call.func == self.wbindgen_describe_generic_import {
                    self.calls += 1;
                }
            }

            // A tail call to the marker is just as much a direct call. Both the
            // interpreter (`Instr::Call(..) | Instr::ReturnCall(..)`) and the
            // call-site rewriter already treat the two alike; without this
            // override discovery alone would miss `return_call` and the
            // monomorphisation would never be bound.
            fn visit_return_call(&mut self, call: &ReturnCall) {
                if call.func == self.wbindgen_describe_generic_import {
                    self.calls += 1;
                }
            }
        }
    }
}

impl CustomSection for WasmBindgenDescriptorsSection {
    fn name(&self) -> &str {
        "wasm-bindgen descriptors"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<'_, [u8]> {
        panic!("shouldn't emit custom sections just yet");
    }
}
