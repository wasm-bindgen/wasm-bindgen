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

use crate::descriptor::Descriptor;
use crate::interpreter::Interpreter;
use anyhow::Error;
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, FunctionId, Module, TypedCustomSectionId};

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub cast_imports: HashMap<Descriptor, Vec<FunctionId>>,
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
    section.execute_casts(module, &mut interpreter)?;

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

    fn execute_casts(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        use walrus::ir::*;

        // If our describe cast intrinsic isn't present or wasn't linked
        // then there're no casts, so nothing to do!
        let wbindgen_describe_cast = match interpreter.describe_cast_id() {
            Some(i) => i,
            None => return Ok(()),
        };

        // Find all functions which call `wbindgen_describe_cast`. These are
        // specially codegen'd so we know the rough structure of them. For each
        // one we delegate to the interpreter to figure out the source and
        // target type descriptors.
        let mut replace_with_imports = Vec::new();
        for (func_id, local) in module.funcs.iter_local() {
            let mut find = FindDescribeCast {
                wbindgen_describe_cast,
                found: false,
            };
            dfs_in_order(&mut find, local, local.entry_block());
            if find.found {
                replace_with_imports.push(func_id);
            }
        }
        for func_id in replace_with_imports {
            let descriptor = interpreter.interpret_descriptor(func_id, module);
            let descriptor = Descriptor::decode(descriptor);
            self.cast_imports
                .entry(descriptor)
                .or_default()
                .push(func_id);
        }

        return Ok(());

        struct FindDescribeCast {
            wbindgen_describe_cast: FunctionId,
            found: bool,
        }

        impl Visitor<'_> for FindDescribeCast {
            fn visit_call(&mut self, call: &Call) {
                if call.func == self.wbindgen_describe_cast {
                    self.found = true;
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
