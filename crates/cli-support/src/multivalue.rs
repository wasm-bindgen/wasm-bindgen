use crate::transforms::multi_value as multi_value_xform;
use crate::wasm_conventions;
use crate::wit::{Adapter, NonstandardWitSection};
use crate::wit::{AdapterKind, Instruction, WasmBindgenAux};
use anyhow::{anyhow, Error};
use walrus::Module;

pub fn run(module: &mut Module) -> Result<(), Error> {
    let mut adapters = module
        .customs
        .delete_typed::<NonstandardWitSection>()
        .unwrap();
    let mut to_xform = Vec::new();
    let mut exports = Vec::new();

    for adapter in adapters.adapters.values_mut() {
        extract_xform(module, adapter, &mut to_xform, &mut exports);
    }
    if to_xform.is_empty() {
        // Early exit to avoid failing if we don't have a memory or stack
        // pointer because this is a minimal module that doesn't use linear
        // memory.
        module.customs.add(*adapters);
        return Ok(());
    }

    let stack_pointer = module
        .customs
        .get_typed::<WasmBindgenAux>()
        .expect("aux section should be present")
        .stack_pointer
        .ok_or_else(|| anyhow!("failed to find stack pointer in Wasm module"))?;
    let memory = wasm_conventions::get_memory(module)?;
    let wrappers = multi_value_xform::run(module, memory, stack_pointer, &to_xform)?;

    for (export, id) in exports.into_iter().zip(wrappers) {
        module.exports.get_mut(export).item = id.into();
    }

    module.customs.add(*adapters);

    Ok(())
}

fn extract_xform(
    module: &Module,
    adapter: &mut Adapter,
    to_xform: &mut Vec<(walrus::FunctionId, usize, Vec<walrus::ValType>)>,
    exports: &mut Vec<walrus::ExportId>,
) {
    let instructions = match &mut adapter.kind {
        AdapterKind::Local { instructions } => instructions,
        AdapterKind::Import { .. } => return,
    };

    // If the first instruction is a `Retptr`, then this must be an exported
    // adapter which calls a wasm-defined function. Something we'd like to
    // adapt to multi-value!
    if let Some(Instruction::Retptr { .. }) = instructions.first().map(|e| &e.instr) {
        instructions.remove(0);
        let mut types = Vec::new();
        instructions.retain(|instruction| match &instruction.instr {
            Instruction::LoadRetptr { ty, .. } => {
                types.push(ty.to_wasm().unwrap());
                false
            }
            _ => true,
        });
        let export = instructions
            .iter_mut()
            .find_map(|i| match i.instr {
                Instruction::CallExport(e) => Some(e),
                _ => None,
            })
            .expect("adapter never calls the underlying function");

        // LLVM currently always uses the first parameter for the return
        // pointer. We hard code that here, since we have no better option.
        let id = match module.exports.get(export).item {
            walrus::ExportItem::Function(f) => f,
            _ => panic!("found call to non-function export"),
        };
        to_xform.push((id, 0, types));
        exports.push(export);
    }

    // If the last instruction is a `StoreRetptr`, then this must be an adapter
    // which calls an imported function.
    //
    // FIXME(#1872) handle this
    // if let Some(Instruction::StoreRetptr { .. }) = instructions.last() {}
}
