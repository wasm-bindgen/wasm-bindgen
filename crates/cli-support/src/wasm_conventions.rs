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
    ir::Value, ConstExpr, ConstOp, ElementItems, FunctionBuilder, FunctionId, FunctionKind,
    GlobalId, GlobalKind, MemoryId, Module, RawCustomSection, ValType,
};
use wasmparser::BinaryReader;

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
        .filter(|g| g.ty == ValType::I32 || g.ty == ValType::I64)
        .filter(|g| g.mutable)
        // The stack pointer is guaranteed to not be initialized to 0, and it's
        // guaranteed to have an i32/i64 initializer, so find globals which are
        // locally defined and have a nonzero initializer
        .filter(|g| match g.kind {
            GlobalKind::Local(ConstExpr::Value(Value::I32(n))) => n != 0,
            GlobalKind::Local(ConstExpr::Value(Value::I64(n))) => n != 0,
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

            global.ty == ValType::I32 || global.ty == ValType::I64
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        1 => Some(candidates[0]),
        _ => None,
    }
}

/// Attempts to statically evaluate a `ConstExpr` to a scalar `Value`.
///
/// Handles `Value` (immediate), `Global` (resolved via the module's globals),
/// and `Extended` (a small stack-machine subset: numeric constants, GlobalGet,
/// and integer add/sub/mul). Returns `None` for anything that cannot be fully
/// reduced at compile time (imported globals, unsupported opcodes, etc.).
///
/// This is needed because rustc 1.94+ / lld emits element segment offsets as
/// `global.get $__table_base` (`ConstExpr::Global`) or even
/// `global.get $__table_base; i32.const K; i32.add` (`ConstExpr::Extended`)
/// for large WASM modules, rather than a plain `i32.const N`.
fn evaluate_const_expr(expr: &ConstExpr, module: &Module) -> Option<Value> {
    match expr {
        ConstExpr::Value(v) => Some(*v),
        ConstExpr::Global(g) => {
            match &module.globals.get(*g).kind {
                GlobalKind::Local(inner) => evaluate_const_expr(inner, module),
                // Imported globals have no known static value.
                _ => None,
            }
        }
        ConstExpr::Extended(ops) => {
            let mut stack: Vec<Value> = Vec::new();
            for op in ops {
                match op {
                    ConstOp::I32Const(n) => stack.push(Value::I32(*n)),
                    ConstOp::I64Const(n) => stack.push(Value::I64(*n)),
                    ConstOp::F32Const(n) => stack.push(Value::F32(*n)),
                    ConstOp::F64Const(n) => stack.push(Value::F64(*n)),
                    ConstOp::GlobalGet(g) => {
                        let v = match &module.globals.get(*g).kind {
                            GlobalKind::Local(inner) => evaluate_const_expr(inner, module)?,
                            _ => return None,
                        };
                        stack.push(v);
                    }
                    ConstOp::I32Add => {
                        let (Value::I32(b), Value::I32(a)) = (stack.pop()?, stack.pop()?) else {
                            return None;
                        };
                        stack.push(Value::I32(a.wrapping_add(b)));
                    }
                    ConstOp::I32Sub => {
                        let (Value::I32(b), Value::I32(a)) = (stack.pop()?, stack.pop()?) else {
                            return None;
                        };
                        stack.push(Value::I32(a.wrapping_sub(b)));
                    }
                    ConstOp::I32Mul => {
                        let (Value::I32(b), Value::I32(a)) = (stack.pop()?, stack.pop()?) else {
                            return None;
                        };
                        stack.push(Value::I32(a.wrapping_mul(b)));
                    }
                    ConstOp::I64Add => {
                        let (Value::I64(b), Value::I64(a)) = (stack.pop()?, stack.pop()?) else {
                            return None;
                        };
                        stack.push(Value::I64(a.wrapping_add(b)));
                    }
                    ConstOp::I64Sub => {
                        let (Value::I64(b), Value::I64(a)) = (stack.pop()?, stack.pop()?) else {
                            return None;
                        };
                        stack.push(Value::I64(a.wrapping_sub(b)));
                    }
                    ConstOp::I64Mul => {
                        let (Value::I64(b), Value::I64(a)) = (stack.pop()?, stack.pop()?) else {
                            return None;
                        };
                        stack.push(Value::I64(a.wrapping_mul(b)));
                    }
                    _ => return None,
                }
            }
            if stack.len() == 1 {
                stack.pop()
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Looks up a function table entry by index in the main function table.
pub fn get_function_table_entry(module: &Module, idx: u32) -> Result<FunctionId> {
    let table = module
        .tables
        .main_function_table()?
        .ok_or_else(|| anyhow!("no function table found in module"))?;
    let table = module.tables.get(table);
    for &segment in table.elem_segments.iter() {
        let segment = module.elements.get(segment);
        let offset = match &segment.kind {
            walrus::ElementKind::Active { offset, .. } => match evaluate_const_expr(offset, module)
            {
                Some(Value::I32(n)) => n as u32,
                Some(Value::I64(n)) => match u32::try_from(n) {
                    Ok(n) => n,
                    // Cannot represent this segment's offset as a table index; skip it.
                    Err(_) => continue,
                },
                // Cannot statically evaluate this segment's offset; skip it.
                _ => continue,
            },
            _ => continue,
        };

        // Guard: if idx < offset this segment does not contain idx.
        let local_idx = match idx.checked_sub(offset) {
            Some(i) => i as usize,
            None => continue,
        };

        let slot = match &segment.items {
            ElementItems::Functions(items) => items.get(local_idx).map(Some),
            ElementItems::Expressions(_, items) => items.get(local_idx).map(|item| {
                if let ConstExpr::RefFunc(target) = item {
                    Some(target)
                } else {
                    None
                }
            }),
        };

        match slot {
            Some(slot) => {
                return slot.copied().context("function table entry wasn't filled");
            }
            None => continue,
        }
    }
    bail!("failed to find `{idx}` in function table");
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
        let mut reader = BinaryReader::new(&section.data, 0);
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
        let mut reader = BinaryReader::new(&section.data, 0);
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

#[cfg(test)]
mod tests {
    use super::*;
    use walrus::{
        ConstOp, ElementItems, ElementKind, FunctionBuilder, Module, ModuleConfig, RefType,
    };

    /// Build a minimal module containing a function table, add an active
    /// element segment with the given offset expression, and return the module
    /// together with the FunctionId that was placed into the segment.
    fn make_module_with_segment(offset: ConstExpr) -> (Module, FunctionId) {
        let mut config = ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = Module::with_config(config);

        let func_id =
            FunctionBuilder::new(&mut module.types, &[], &[]).finish(vec![], &mut module.funcs);
        module.exports.add("f", func_id);

        let table_id = module.tables.add_local(false, 64, None, RefType::FUNCREF);

        let elem_id = module.elements.add(
            ElementKind::Active {
                table: table_id,
                offset,
            },
            ElementItems::Functions(vec![func_id]),
        );
        module
            .tables
            .get_mut(table_id)
            .elem_segments
            .insert(elem_id);

        (module, func_id)
    }

    // -----------------------------------------------------------------------
    // evaluate_const_expr
    // -----------------------------------------------------------------------

    #[test]
    fn evaluate_immediate_i32() {
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        let expr = ConstExpr::Value(Value::I32(42));
        assert!(matches!(
            evaluate_const_expr(&expr, &module),
            Some(Value::I32(42))
        ));
    }

    #[test]
    fn evaluate_global_offset() {
        let mut config = ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = Module::with_config(config);

        // Immutable local global initialised to 5.
        let g =
            module
                .globals
                .add_local(ValType::I32, false, false, ConstExpr::Value(Value::I32(5)));

        let expr = ConstExpr::Global(g);
        assert!(matches!(
            evaluate_const_expr(&expr, &module),
            Some(Value::I32(5))
        ));
    }

    #[test]
    fn evaluate_extended_global_plus_const() {
        // Models the lld pattern: global.get $__table_base; i32.const K; i32.add
        let mut config = ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = Module::with_config(config);

        let g =
            module
                .globals
                .add_local(ValType::I32, false, false, ConstExpr::Value(Value::I32(1)));

        let expr = ConstExpr::Extended(vec![
            ConstOp::GlobalGet(g),
            ConstOp::I32Const(7),
            ConstOp::I32Add,
        ]);
        assert!(matches!(
            evaluate_const_expr(&expr, &module),
            Some(Value::I32(8))
        ));
    }

    #[test]
    fn evaluate_extended_returns_none_for_unknown_op() {
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        // RefNull is not a numeric op — should return None.
        let expr = ConstExpr::Extended(vec![ConstOp::RefNull(walrus::RefType::FUNCREF)]);
        assert!(evaluate_const_expr(&expr, &module).is_none());
    }

    // -----------------------------------------------------------------------
    // get_function_table_entry
    // -----------------------------------------------------------------------

    #[test]
    fn lookup_with_immediate_i32_offset() {
        // Baseline: plain i32.const offset — must still work after the refactor.
        let (module, func_id) = make_module_with_segment(ConstExpr::Value(Value::I32(1)));
        let result = get_function_table_entry(&module, 1);
        assert_eq!(result.unwrap(), func_id);
    }

    #[test]
    fn lookup_with_global_offset() {
        // rustc 1.94+ / lld emits global.get $__table_base as the segment offset
        // for large WASM modules.  This was the primary trigger for issue #5076.
        let mut config = ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = Module::with_config(config);

        // Immutable global holding the table base value of 1.
        let g =
            module
                .globals
                .add_local(ValType::I32, false, false, ConstExpr::Value(Value::I32(1)));
        module.exports.add("__table_base", g);

        let func_id =
            FunctionBuilder::new(&mut module.types, &[], &[]).finish(vec![], &mut module.funcs);
        module.exports.add("f", func_id);

        let table_id = module.tables.add_local(false, 4, None, RefType::FUNCREF);
        let elem_id = module.elements.add(
            ElementKind::Active {
                table: table_id,
                offset: ConstExpr::Global(g),
            },
            ElementItems::Functions(vec![func_id]),
        );
        module
            .tables
            .get_mut(table_id)
            .elem_segments
            .insert(elem_id);

        // Table index 1 is where the segment starts (global value = 1).
        let result = get_function_table_entry(&module, 1);
        assert_eq!(result.unwrap(), func_id);
    }

    #[test]
    fn lookup_with_extended_offset() {
        // lld with multiple object files: global.get $base + i32.const 4.
        let mut config = ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = Module::with_config(config);

        let g =
            module
                .globals
                .add_local(ValType::I32, false, false, ConstExpr::Value(Value::I32(1)));
        module.exports.add("__table_base", g);

        let func_id =
            FunctionBuilder::new(&mut module.types, &[], &[]).finish(vec![], &mut module.funcs);
        module.exports.add("f", func_id);

        let table_id = module.tables.add_local(false, 16, None, RefType::FUNCREF);
        let elem_id = module.elements.add(
            ElementKind::Active {
                table: table_id,
                // offset = 1 + 4 = 5
                offset: ConstExpr::Extended(vec![
                    ConstOp::GlobalGet(g),
                    ConstOp::I32Const(4),
                    ConstOp::I32Add,
                ]),
            },
            ElementItems::Functions(vec![func_id]),
        );
        module
            .tables
            .get_mut(table_id)
            .elem_segments
            .insert(elem_id);

        let result = get_function_table_entry(&module, 5);
        assert_eq!(result.unwrap(), func_id);
    }

    #[test]
    fn lookup_fails_gracefully_when_index_not_in_any_segment() {
        let (module, _) = make_module_with_segment(ConstExpr::Value(Value::I32(1)));
        // Index 99 is beyond the single-entry segment at offset 1.
        assert!(get_function_table_entry(&module, 99).is_err());
    }

    #[test]
    fn lookup_multi_segment_no_underflow() {
        // Two segments: A at offset 0 (func_a), B at offset 128 (func_b).
        // Looking up index 128 must find func_b without underflowing when
        // subtracting segment A's offset from 128 would give 128, which is
        // out-of-bounds for segment A (length 1) — that's fine.
        // The old bug was that idx < offset could wrap in u32 arithmetic.
        let mut config = ModuleConfig::new();
        config.generate_producers_section(false);
        let mut module = Module::with_config(config);

        let func_a =
            FunctionBuilder::new(&mut module.types, &[], &[]).finish(vec![], &mut module.funcs);
        module.exports.add("func_a", func_a);

        let func_b =
            FunctionBuilder::new(&mut module.types, &[], &[]).finish(vec![], &mut module.funcs);
        module.exports.add("func_b", func_b);

        let table_id = module.tables.add_local(false, 256, None, RefType::FUNCREF);

        let seg_a = module.elements.add(
            ElementKind::Active {
                table: table_id,
                offset: ConstExpr::Value(Value::I32(0)),
            },
            ElementItems::Functions(vec![func_a]),
        );
        module.tables.get_mut(table_id).elem_segments.insert(seg_a);

        let seg_b = module.elements.add(
            ElementKind::Active {
                table: table_id,
                offset: ConstExpr::Value(Value::I32(128)),
            },
            ElementItems::Functions(vec![func_b]),
        );
        module.tables.get_mut(table_id).elem_segments.insert(seg_b);

        assert_eq!(get_function_table_entry(&module, 0).unwrap(), func_a);
        assert_eq!(get_function_table_entry(&module, 128).unwrap(), func_b);
    }
}
