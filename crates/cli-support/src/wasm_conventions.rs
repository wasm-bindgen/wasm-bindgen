//! A tiny crate of utilities for working with implicit Wasm codegen conventions
//! (often established by LLVM and lld).
//!
//! Examples conventions include:
//!
//! * The stack pointer
//! * The canonical linear memory that contains the stack

use std::io::Cursor;

use anyhow::{anyhow, bail, Context, Result};
use walrus::{
    ir::Value, ConstExpr, ElementId, ElementItems, FunctionBuilder, FunctionId, FunctionKind,
    GlobalId, GlobalKind, MemoryId, Module, RawCustomSection, ValType,
};
use wasmparser::{BinaryReader, WasmFeatures};

/// Get a Wasm module's canonical linear memory.
pub fn get_memory(module: &Module) -> Result<MemoryId> {
    let mut memories = module.memories.iter().map(|m| m.id());
    let memory = memories.next();
    if memories.next().is_some() {
        bail!(
            "expected a single memory, found multiple; multiple memories \
             currently not supported"
        );
    }
    memory.ok_or_else(|| {
        anyhow!(
            "module does not have a memory; must have a memory \
             to transform return pointers into Wasm multi-value"
        )
    })
}

/// Get the `__stack_pointer`.
pub fn get_stack_pointer(module: &Module) -> Option<GlobalId> {
    if let Some(g) = module
        .globals
        .iter()
        .find(|g| matches!(g.name.as_deref(), Some("__stack_pointer")))
    {
        return Some(g.id());
    }

    let candidates = module
        .globals
        .iter()
        .filter(|g| g.ty == ValType::I32)
        .filter(|g| g.mutable)
        // The stack pointer is guaranteed to not be initialized to 0, and it's
        // guaranteed to have an i32 initializer, so find globals which are
        // locally defined, are an i32, and have a nonzero initializer
        .filter(|g| match g.kind {
            GlobalKind::Local(ConstExpr::Value(Value::I32(n))) => n != 0,
            _ => false,
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        0 => None,
        1 => Some(candidates[0].id()),
        2 => {
            log::warn!("Unable to accurately determine the location of `__stack_pointer`");
            Some(candidates[0].id())
        }
        _ => None,
    }
}

/// Get the `__tls_base`.
pub fn get_tls_base(module: &Module) -> Option<GlobalId> {
    let candidates = module
        .exports
        .iter()
        .filter(|ex| ex.name == "__tls_base")
        .filter_map(|ex| match ex.item {
            walrus::ExportItem::Global(id) => Some(id),
            _ => None,
        })
        .filter(|id| {
            let global = module.globals.get(*id);

            global.ty == ValType::I32
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        1 => Some(candidates[0]),
        _ => None,
    }
}

pub struct FunctionTableEntry {
    pub element: ElementId,
    pub idx: usize,
    pub func: Option<FunctionId>,
}

/// Looks up a function table entry by index in the main function table.
pub fn get_function_table_entry(module: &Module, idx: u32) -> Result<FunctionTableEntry> {
    let table = module
        .tables
        .main_function_table()?
        .ok_or_else(|| anyhow!("no function table found in module"))?;
    let table = module.tables.get(table);
    for &segment in table.elem_segments.iter() {
        let segment = module.elements.get(segment);
        let offset = match &segment.kind {
            walrus::ElementKind::Active {
                offset: ConstExpr::Value(Value::I32(n)),
                ..
            } => *n as u32,
            _ => continue,
        };
        let idx = (idx - offset) as usize;

        let slot = match &segment.items {
            ElementItems::Functions(items) => items.get(idx).map(Some),
            ElementItems::Expressions(_, items) => items.get(idx).map(|item| {
                if let ConstExpr::RefFunc(target) = item {
                    Some(target)
                } else {
                    None
                }
            }),
        };

        match slot {
            Some(slot) => {
                return Ok(FunctionTableEntry {
                    element: segment.id(),
                    idx,
                    func: slot.cloned(),
                })
            }
            None => continue,
        }
    }
    bail!("failed to find `{}` in function table", idx);
}

pub fn get_start(module: &mut Module) -> Result<FunctionId, Option<FunctionId>> {
    match module.start {
        Some(start) => match module.funcs.get_mut(start).kind {
            FunctionKind::Import(_) => Err(Some(start)),
            FunctionKind::Local(_) => Ok(start),
            FunctionKind::Uninitialized(_) => unimplemented!(),
        },
        None => Err(None),
    }
}

pub fn get_or_insert_start_builder(module: &mut Module) -> &mut FunctionBuilder {
    let prev_start = get_start(module);

    let id = match prev_start {
        Ok(id) => id,
        Err(prev_start) => {
            let mut builder = FunctionBuilder::new(&mut module.types, &[], &[]);

            if let Some(prev_start) = prev_start {
                builder.func_body().call(prev_start);
            }

            let id = builder.finish(Vec::new(), &mut module.funcs);
            module.start = Some(id);
            id
        }
    };

    module
        .funcs
        .get_mut(id)
        .kind
        .unwrap_local_mut()
        .builder_mut()
}

pub fn target_feature(module: &Module, feature: &str) -> Result<bool> {
    // Taken from <https://github.com/bytecodealliance/wasm-tools/blob/f1898f46bb9d96f0f09682415cb6ccfd6a4dca79/crates/wasmparser/src/limits.rs#L27>.
    anyhow::ensure!(feature.len() <= 100_000, "feature name too long");

    // Try to find an existing section.
    let section = module
        .customs
        .iter()
        .find(|(_, custom)| custom.name() == "target_features");

    if let Some((_, section)) = section {
        let section: &RawCustomSection = section
            .as_any()
            .downcast_ref()
            .context("failed to read section")?;
        let mut reader = BinaryReader::new(&section.data, 0, WasmFeatures::default());
        // The first integer contains the target feature count.
        let count = reader.read_var_u32()?;

        // Try to find if the target feature is already present.
        for _ in 0..count {
            // First byte is the prefix.
            let prefix = reader.read_u8()?;
            // Read the feature.
            let length = reader.read_var_u32()?;
            let this_feature = reader.read_bytes(length as usize)?;

            // If we found the target feature, we are done here.
            if this_feature == feature.as_bytes() {
                // Make sure we set any existing prefix to "enabled".
                if prefix == b'-' {
                    return Ok(false);
                }

                return Ok(true);
            }
        }

        Ok(false)
    } else {
        Ok(false)
    }
}

pub fn insert_target_feature(module: &mut Module, new_feature: &str) -> Result<()> {
    // Taken from <https://github.com/bytecodealliance/wasm-tools/blob/f1898f46bb9d96f0f09682415cb6ccfd6a4dca79/crates/wasmparser/src/limits.rs#L27>.
    anyhow::ensure!(new_feature.len() <= 100_000, "feature name too long");

    // Try to find an existing section.
    let section = module
        .customs
        .iter_mut()
        .find(|(_, custom)| custom.name() == "target_features");

    // If one exists, check if the target feature is already present.
    let section = if let Some((_, section)) = section {
        let section: &mut RawCustomSection = section
            .as_any_mut()
            .downcast_mut()
            .context("failed to read section")?;
        let mut reader = BinaryReader::new(&section.data, 0, WasmFeatures::default());
        // The first integer contains the target feature count.
        let count = reader.read_var_u32()?;

        // Try to find if the target feature is already present.
        for _ in 0..count {
            // First byte is the prefix.
            let prefix_index = reader.current_position();
            let prefix = reader.read_u8()?;
            // Read the feature.
            let length = reader.read_var_u32()?;
            let feature = reader.read_bytes(length as usize)?;

            // If we found the target feature, we are done here.
            if feature == new_feature.as_bytes() {
                // Make sure we set any existing prefix to "enabled".
                if prefix == b'-' {
                    section.data[prefix_index] = b'+';
                }

                return Ok(());
            }
        }

        section
    } else {
        let mut data = Vec::new();
        leb128::write::unsigned(&mut data, 0).unwrap();
        let id = module.customs.add(RawCustomSection {
            name: String::from("target_features"),
            data,
        });
        module.customs.get_mut(id).unwrap()
    };

    // If we couldn't find the target feature, insert it.

    // The first byte contains an integer describing the target feature count, which we increase by one.
    let mut data = Cursor::new(&section.data);
    let count = leb128::read::unsigned(&mut data).unwrap();
    let mut new_count = Vec::new();
    leb128::write::unsigned(&mut new_count, count + 1).unwrap();
    section.data.splice(0..data.position() as usize, new_count);
    // Then we insert the "enabled" prefix at the end.
    section.data.push(b'+');
    // The next byte contains the length of the target feature string.
    leb128::write::unsigned(&mut section.data, new_feature.len() as u64).unwrap();
    // Lastly the target feature string is inserted.
    section.data.extend(new_feature.as_bytes());

    Ok(())
}
