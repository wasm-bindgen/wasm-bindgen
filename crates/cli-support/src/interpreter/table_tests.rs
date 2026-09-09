use super::{eval_const, Interpreter};
use walrus::{ir::Value, ConstExpr, FunctionId, ValType};
use walrus::{ConstOp, ElementItems, ElementKind, FunctionBuilder, Module, ModuleConfig, RefType};

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

#[test]
fn evaluate_immediate_i32() {
    let config = ModuleConfig::new();
    let module = Module::with_config(config);
    let expr = ConstExpr::Value(Value::I32(42));
    assert!(matches!(
        eval_const(&expr, &Interpreter::new(&module).unwrap().globals),
        Some(42)
    ));
}

#[test]
fn evaluate_global_offset() {
    let mut config = ModuleConfig::new();
    config.generate_producers_section(false);
    let mut module = Module::with_config(config);

    // Immutable local global initialised to 5.
    let g = module
        .globals
        .add_local(ValType::I32, false, false, ConstExpr::Value(Value::I32(5)));

    let expr = ConstExpr::Global(g);
    assert!(matches!(
        eval_const(&expr, &Interpreter::new(&module).unwrap().globals),
        Some(5)
    ));
}

#[test]
fn evaluate_extended_global_plus_const() {
    // Models the lld pattern: global.get $__table_base; i32.const K; i32.add
    let mut config = ModuleConfig::new();
    config.generate_producers_section(false);
    let mut module = Module::with_config(config);

    let g = module
        .globals
        .add_local(ValType::I32, false, false, ConstExpr::Value(Value::I32(1)));

    let expr = ConstExpr::Extended(vec![
        ConstOp::GlobalGet(g),
        ConstOp::I32Const(7),
        ConstOp::I32Add,
    ]);
    assert!(matches!(
        eval_const(&expr, &Interpreter::new(&module).unwrap().globals),
        Some(8)
    ));
}

#[test]
fn evaluate_extended_returns_none_for_unknown_op() {
    let config = ModuleConfig::new();
    let module = Module::with_config(config);
    // RefNull is not a numeric op — should return None.
    let expr = ConstExpr::Extended(vec![ConstOp::RefNull(walrus::RefType::FUNCREF)]);
    assert!(eval_const(&expr, &Interpreter::new(&module).unwrap().globals).is_none());
}

#[test]
fn lookup_with_immediate_i32_offset() {
    let (module, func_id) = make_module_with_segment(ConstExpr::Value(Value::I32(1)));
    let result = table_entry(&module, 1);
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
    let g = module
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
    let result = table_entry(&module, 1);
    assert_eq!(result.unwrap(), func_id);
}

#[test]
fn lookup_with_extended_offset() {
    // lld with multiple object files: global.get $base + i32.const 4.
    let mut config = ModuleConfig::new();
    config.generate_producers_section(false);
    let mut module = Module::with_config(config);

    let g = module
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

    let result = table_entry(&module, 5);
    assert_eq!(result.unwrap(), func_id);
}

#[test]
fn lookup_fails_gracefully_when_index_not_in_any_segment() {
    let (module, _) = make_module_with_segment(ConstExpr::Value(Value::I32(1)));
    // Index 99 is beyond the single-entry segment at offset 1.
    assert!(table_entry(&module, 99).is_err());
}

#[test]
fn lookup_multi_segment_no_underflow() {
    // Entries in separate segments must resolve without index arithmetic
    // wrapping at the earlier segment.
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

    assert_eq!(table_entry(&module, 0).unwrap(), func_a);
    assert_eq!(table_entry(&module, 128).unwrap(), func_b);
}

fn table_entry(module: &Module, index: u32) -> anyhow::Result<FunctionId> {
    Interpreter::new(module)?.table_entry(index)
}
