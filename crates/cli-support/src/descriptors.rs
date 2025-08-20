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

use crate::descriptor::{Closure, Descriptor};
use crate::interpreter::Interpreter;
use anyhow::Error;
use std::borrow::Cow;
use std::collections::HashMap;
use walrus::{CustomSection, FunctionId, FunctionKind, Module, TypedCustomSectionId};
use walrus::{ImportId, ImportedFunction};

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub closure_imports: HashMap<ImportId, Closure>,
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
    section.execute_closures(module, &mut interpreter)?;

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
            if let Some(d) = interpreter.interpret_descriptor(id, module) {
                let name = &export.name[prefix.len()..];
                let descriptor = Descriptor::decode(d);
                self.descriptors.insert(name.to_string(), descriptor);
            }
            to_remove.push(export.id());
        }

        for id in to_remove {
            module.exports.delete(id);
        }
        Ok(())
    }

    fn execute_closures(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        use walrus::ir::*;

        // If our describe closure intrinsic isn't present or wasn't linked
        // then there's no closures, so nothing to do!
        let wbindgen_describe_closure = match interpreter.describe_closure_id() {
            Some(i) => i,
            None => return Ok(()),
        };

        // Find all functions which call `wbindgen_describe_closure`. These are
        // specially codegen'd so we know the rough structure of them. For each
        // one we delegate to the interpreter to figure out the actual result.
        let mut replace_with_imports = Vec::new();
        for (func_id, local) in module.funcs.iter_local() {
            let mut find = FindDescribeClosure {
                wbindgen_describe_closure,
                found: false,
            };
            dfs_in_order(&mut find, local, local.entry_block());
            if find.found {
                replace_with_imports.push(func_id);
            }
        }
        for func_id in replace_with_imports {
            let descriptor = interpreter.interpret_descriptor(func_id, module).unwrap();
            let descriptor = Descriptor::decode(descriptor);
            let import_name = format!("__wbindgen_closure_wrapper{}", func_id.index());
            let import_id = module
                .imports
                .add("__wbindgen_placeholder__", &import_name, func_id);
            self.closure_imports
                .insert(import_id, descriptor.unwrap_closure());
            let func = module.funcs.get_mut(func_id);
            func.kind = FunctionKind::Import(ImportedFunction {
                import: import_id,
                ty: func.ty(),
            });
        }

        return Ok(());

        struct FindDescribeClosure {
            wbindgen_describe_closure: FunctionId,
            found: bool,
        }

        impl Visitor<'_> for FindDescribeClosure {
            fn visit_call(&mut self, call: &Call) {
                if call.func == self.wbindgen_describe_closure {
                    self.found = true;
                }
            }

            fn visit_return_call(&mut self, instr: &walrus::ir::ReturnCall) {
                if instr.func == self.wbindgen_describe_closure {
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
