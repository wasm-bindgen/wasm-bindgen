use super::Interpreter;
use walrus::ModuleConfig;

fn interpret(wat: &str, name: &str, result: &[u32]) {
    let wasm = wat::parse_str(wat).unwrap();
    let module = ModuleConfig::new()
        .generate_producers_section(false)
        .parse(&wasm)
        .unwrap();
    let mut i = Interpreter::new(&module).unwrap();
    let id = module
        .exports
        .iter()
        .filter(|e| e.name == name)
        .find_map(|e| match e.item {
            walrus::ExportItem::Function(f) => Some(f),
            _ => None,
        })
        .unwrap();
    assert_eq!(i.interpret_descriptor(id, &module), result);
}

#[test]
fn smoke() {
    let wat = r#"
        (module
            (export "foo" (func $foo))

            (func $foo)
        )
    "#;
    interpret(wat, "foo", &[]);

    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                i32.const 1
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[1]);
}

#[test]
fn imported_function_got_preserves_the_exported_table_index() {
    interpret(
        r#"(module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (param i32)))
            (import "env" "__table_base" (global $base i32))
            (import "GOT.func" "closure_adapter" (global $adapter_index (mut i32)))
            (table 2 funcref)
            (elem (global.get $base) $unrelated $adapter)
            (func $unrelated (result i32) i32.const 42)
            (func $adapter (param i32 i32))
            (export "closure_adapter" (func $adapter))
            (func (export "describe_adapter")
                global.get $adapter_index
                call $describe)
        )"#,
        "describe_adapter",
        &[1],
    );
}

#[test]
fn function_got_can_refer_to_an_export_without_a_static_table_slot() {
    let wasm = wat::parse_str(
        r#"(module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (param i32)))
            (import "GOT.func" "closure_adapter" (global $adapter_index (mut i32)))
            (table 0 funcref)
            (func $adapter (export "closure_adapter") (param i32 i32))
            (func (export "describe_adapter")
                global.get $adapter_index
                call $describe)
        )"#,
    )
    .unwrap();
    let module = ModuleConfig::new().parse(&wasm).unwrap();
    let exported = |name: &str| {
        module
            .exports
            .iter()
            .find_map(|export| {
                if export.name != name {
                    return None;
                }
                match export.item {
                    walrus::ExportItem::Function(func) => Some(func),
                    _ => None,
                }
            })
            .unwrap()
    };
    let mut interpreter = Interpreter::new(&module).unwrap();
    let index = interpreter.interpret_descriptor(exported("describe_adapter"), &module)[0];
    assert_eq!(
        interpreter.into_function_table()[&index],
        exported("closure_adapter")
    );
}

#[test]
fn function_got_indices_do_not_overlap_expression_elements_or_table_holes() {
    let wasm = wat::parse_str(
        r#"(module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (param i32)))
            (import "env" "__table_base" (global $base i32))
            (import "GOT.func" "existing" (global $existing (mut i32)))
            (import "GOT.func" "new" (global $new (mut i32)))
            (table 16 funcref)
            (func $existing (export "existing"))
            (func (export "new"))
            (elem (i32.add (global.get $base) (i32.const 4)) funcref
                (ref.null func) (ref.func $existing))
            (func (export "describe_indices")
                global.get $existing
                call $describe
                global.get $new
                call $describe)
        )"#,
    )
    .unwrap();
    let module = ModuleConfig::new().parse(&wasm).unwrap();
    let describe = module.exports.get_func("describe_indices").unwrap();
    let new = module.exports.get_func("new").unwrap();
    let mut interpreter = Interpreter::new(&module).unwrap();
    let descriptor = interpreter.interpret_descriptor(describe, &module).to_vec();
    assert_eq!(descriptor[0], 5);
    assert!(descriptor[1] >= 16);
    assert_eq!(interpreter.into_function_table()[&descriptor[1]], new);
}

#[test]
#[should_panic(expected = "cannot resolve GOT.func::missing during descriptor interpretation")]
fn an_unresolved_function_got_is_not_treated_as_table_entry_zero() {
    interpret(
        r#"(module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (param i32)))
            (import "GOT.func" "missing" (global $missing (mut i32)))
            (func (export "describe_missing")
                global.get $missing
                call $describe)
        )"#,
        "describe_missing",
        &[],
    );
}

#[test]
fn locals() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                (local i32)
                i32.const 2
                local.set 0
                local.get 0
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[2]);
}

#[test]
fn globals() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer (mut i32) (i32.const 32768))

            (func $foo
                (local i32)
                global.get $__stack_pointer
                local.set 0
                local.get 0
                call $__wbindgen_describe
                local.get 0
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[32768]);
}

#[test]
fn arithmetic() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                i32.const 1
                i32.const 2
                i32.add
                call $__wbindgen_describe
                i32.const 2
                i32.const 1
                i32.sub
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[3, 1]);
}

#[test]
fn return_early() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                i32.const 1
                i32.const 2
                call $__wbindgen_describe
                return
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[2]);
}

#[test]
fn loads_and_stores() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            ;; 1 page = 65536 bytes; SP starts at the top
            (global $__stack_pointer (mut i32) (i32.const 65536))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                ;; store 1 at fp+0
                local.get 0
                i32.const 1
                i32.store offset=0

                ;; store 2 at fp+4
                local.get 0
                i32.const 2
                i32.store offset=4

                ;; store 3 at fp+8
                local.get 0
                i32.const 3
                i32.store offset=8

                ;; store8
                local.get 0
                i32.const 3
                i32.store8 offset=7

                ;; load8
                local.get 0
                i32.load8_u offset=7
                drop

                ;; load fp+0 and call
                local.get 0
                i32.load offset=0
                call $__wbindgen_describe

                ;; load fp+4 and call
                local.get 0
                i32.load offset=4
                call $__wbindgen_describe

                ;; load fp+8 and call
                local.get 0
                i32.load offset=8
                call $__wbindgen_describe

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[1, 50331650, 3]);
}

// Regression test: `-Cinstrument-coverage` injects i64 profiler-counter
// load/store/add into the `__wbindgen_describe_*` helper functions. The
// descriptor interpreter must tolerate width-8 (i64) loads (it already handled
// width-8 stores) so coverage-instrumented modules can still be processed.
// Without the i64-load handling this panics with "Unhandled load width 8".
#[test]
fn i64_loads_and_stores() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer (mut i32) (i32.const 65536))
            (memory 1)

            (func $foo
                (local i32)

                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                ;; i64.store 42 at fp+0 (width 8)
                local.get 0
                i64.const 42
                i64.store offset=0

                ;; i64.load fp+0 (width 8), wrap to i32, and describe it
                local.get 0
                i64.load offset=0
                i32.wrap_i64
                call $__wbindgen_describe

                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[42]);
}

// Regression test: a width-8 (i64) access that is only 4-byte aligned (not
// 8-byte aligned). `-Cinstrument-coverage` injects i64 profiler-counter
// load/store/add into the `__wbindgen_describe_*` helpers, and those counters
// can land on a 4-aligned (not 8-aligned) slot. The interpreter models linear
// memory as i32 words, so 4-byte alignment is sufficient; a natural-alignment
// check wrongly panicked with "Condition failed: `address % width == 0` (4 vs 0)".
#[test]
fn i64_loads_and_stores_unaligned() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer (mut i32) (i32.const 65536))
            (memory 1)

            (func $foo
                (local i32)

                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                ;; i64.store at fp+4: the address is only 4-aligned
                local.get 0
                i64.const 42
                i64.store offset=4

                local.get 0
                i64.load offset=4
                i32.wrap_i64
                call $__wbindgen_describe

                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[42]);
}

#[test]
fn calling_functions() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer i32 (i32.const 0))
            (memory 1)

            (func $foo
                call $bar
            )

            (func $bar
                i32.const 0
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[0]);
}

#[test]
fn try_block() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))
            (global $__stack_pointer (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                try
                    i32.const 1
                    call $__wbindgen_describe
                catch_all
                end

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[1]);
}

#[test]
fn try_table_block() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))
            (global $__stack_pointer (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                (block $catch
                    (try_table (catch_all $catch)
                        i32.const 1
                        call $__wbindgen_describe
                    )
                )

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[1]);
}

#[test]
fn br_out_of_try_table() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))
            (global $__stack_pointer (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                (block $outer
                    (try_table (catch_all $outer)
                        i32.const 42
                        call $__wbindgen_describe
                        br $outer
                    )
                )

                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[42]);
}

#[test]
fn br_if_taken() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                (block $skip
                    i32.const 1
                    br_if $skip
                    i32.const 99
                    call $__wbindgen_describe
                )
                i32.const 7
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[7]);
}

#[test]
fn br_if_not_taken() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                (block $skip
                    i32.const 0
                    br_if $skip
                    i32.const 5
                    call $__wbindgen_describe
                )
                i32.const 10
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[5, 10]);
}

#[test]
fn calling_functions_with_args() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer i32 (i32.const 0))
            (memory 1)

            (func $foo
                i32.const 1
                i32.const 2
                call $bar
            )

            (func $bar (param i32) (param i32)
                local.get 0
                call $__wbindgen_describe
                local.get 1
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[1, 2]);
}

#[test]
#[should_panic]
fn calling_function_with_args_out_of_order() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer i32 (i32.const 0))
            (memory 1)

            (func $foo
                i32.const 1
                i32.const 2
                call $bar
            )

            (func $bar (param i32) (param i32)
                local.get 0
                call $__wbindgen_describe
                local.get 1
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[2, 1]);
}

#[test]
fn blocks() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer (mut i32) (i32.const 0))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                (block
                    i32.const 0
                    call $__wbindgen_describe
                )

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )
            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[0]);
}

// Test for issue #5080: interpreter should distinguish between __stack_pointer
// and other globals like GOT.func.internal.*
#[test]
fn multiple_globals_with_stack_pointer() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer (mut i32) (i32.const 32768))
            (global $other1 i32 (i32.const 42))
            (global $other2 (mut i32) (i32.const 100))

            (func $foo
                ;; Read other global - should get 42, not stack pointer
                global.get $other1
                call $__wbindgen_describe

                ;; Read stack pointer - should get 32768
                global.get $__stack_pointer
                call $__wbindgen_describe

                ;; Modify other global
                i32.const 200
                global.set $other2

                ;; Read modified other global
                global.get $other2
                call $__wbindgen_describe

                ;; Modify stack pointer
                global.get $__stack_pointer
                i32.const 16
                i32.sub
                global.set $__stack_pointer

                ;; Read stack pointer again - should get 32752
                global.get $__stack_pointer
                call $__wbindgen_describe

                ;; Restore stack pointer
                global.get $__stack_pointer
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[42, 32768, 200, 32752]);
}

// Test for issue #5093: __stack_pointer exists as a named global but is NOT
// exported. The interpreter must still distinguish it from other globals.
#[test]
fn multiple_globals_with_named_stack_pointer_not_exported() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            ;; 1 page = 65536 bytes; SP starts at the top
            (global $__stack_pointer (mut i32) (i32.const 65536))
            (global $got_entry (mut i32) (i32.const 7))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer
                global.get $__stack_pointer
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $__stack_pointer

                ;; store a value via the stack
                local.get 0
                i32.const 5
                i32.store offset=0

                ;; load it back and describe
                local.get 0
                i32.load offset=0
                call $__wbindgen_describe

                ;; describe the GOT-like global (should be 7)
                global.get $got_entry
                call $__wbindgen_describe

                ;; restore the stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[5, 7]);
}

// Emscripten rewrites calls that may unwind/longjmp into indirect calls through
// the function table, wrapped in imported `invoke_*(fnptr, ..args)` helpers. The
// interpreter must redirect such a call to the real table-indexed target and
// forward the trailing arguments.
#[test]
fn invoke_redirect() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $describe (param i32)))
            (import "env" "invoke_vi" (func $invoke_vi (param i32 i32)))

            (table 1 funcref)
            (elem (i32.const 0) $target)

            (func $target (param i32)
                local.get 0
                call $describe
            )

            (func $foo
                i32.const 0   ;; table index of $target
                i32.const 5   ;; forwarded argument
                call $invoke_vi
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[5]);
}

#[test]
fn if_else() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $describe (param i32)))

            (func $foo
                i32.const 1
                (if
                    (then
                        i32.const 11
                        call $describe)
                    (else
                        i32.const 22
                        call $describe)))

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[11]);
}

#[test]
fn loop_back_edge() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $describe (param i32)))

            (func $foo
                (local $i i32)
                (loop $l
                    local.get $i
                    call $describe
                    local.get $i
                    i32.const 1
                    i32.add
                    local.tee $i
                    i32.const 2
                    i32.lt_s
                    br_if $l))

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[0, 1]);
}

#[test]
fn br_table_block() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $describe (param i32)))

            (func $foo
                (block $out
                    (block $zero
                        (block $one
                            i32.const 1
                            br_table $zero $one $out)
                        i32.const 100
                        call $describe
                        br $out)
                    i32.const 200
                    call $describe))

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[100]);
}

#[test]
fn wasm64_stack_pointer_global() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $__stack_pointer (mut i64) (i64.const 65536))

            (func $foo
                global.get $__stack_pointer
                i64.const 16
                i64.sub
                global.set $__stack_pointer

                global.get $__stack_pointer
                i32.wrap_i64
                call $__wbindgen_describe

                global.get $__stack_pointer
                i64.const 16
                i64.add
                global.set $__stack_pointer
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[65520]);
}

#[test]
fn indirect_calls_forward_arguments_and_results() {
    for call in ["call_indirect", "return_call_indirect"] {
        interpret(
            &format!(
                r#"(module
                (type $add (func (param i32 i32) (result i32)))
                (import "__wbindgen_placeholder__" "__wbindgen_describe"
                    (func $describe (param i32)))
                (table 4 funcref)
                (elem (i32.const 2) $add)
                (func $add (type $add)
                    local.get 0 local.get 1 i32.add)
                (func $dispatch (result i32)
                    i32.const 19 i32.const 23 i32.const 2
                    {call} (type $add))
                (func (export "run") call $dispatch call $describe)
            )"#
            ),
            "run",
            &[42],
        );
    }
}

#[test]
fn descriptor_imports_can_be_called_indirectly() {
    for call in ["call_indirect", "return_call_indirect"] {
        interpret(
            &format!(
                r#"(module
                (type $describe (func (param i32)))
                (import "__wbindgen_placeholder__" "__wbindgen_describe"
                    (func $describe (type $describe)))
                (table 1 funcref)
                (elem (i32.const 0) $describe)
                (func (export "run")
                    i32.const 42 i32.const 0 {call} (type $describe))
            )"#
            ),
            "run",
            &[42],
        );
    }
}

#[test]
fn imported_function_addresses_use_the_same_indirect_call_table() {
    interpret(
        r#"(module
            (type $callback (func (param i32)))
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (type $callback)))
            (import "GOT.func" "callback" (global $callback (mut i32)))
            (import "env" "invoke_vi" (func $invoke (param i32 i32)))
            (table 4 funcref)
            (func (export "callback") (type $callback) local.get 0 call $describe)
            (func (export "run")
                i32.const 42 global.get $callback call_indirect (type $callback)
                global.get $callback i32.const 43 call $invoke)
        )"#,
        "run",
        &[42, 43],
    );
}

#[test]
fn later_element_segments_replace_earlier_functions() {
    interpret(
        r#"(module
            (type $callback (func))
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (param i32)))
            (table 2 funcref)
            (func $first i32.const 1 call $describe)
            (func $second i32.const 2 call $describe)
            (elem (i32.const 1) $first)
            (elem (i32.const 1) $second)
            (func (export "run") i32.const 1 call_indirect (type $callback))
        )"#,
        "run",
        &[2],
    );
}

#[test]
#[should_panic(expected = "function table entry 1 is unavailable during descriptor interpretation")]
fn a_null_element_overwrites_an_earlier_function() {
    interpret(
        r#"(module
            (type $callback (func))
            (table 2 funcref)
            (func $callback)
            (elem (i32.const 1) $callback)
            (elem (i32.const 1) funcref (ref.null func))
            (func (export "run") i32.const 1 call_indirect (type $callback))
        )"#,
        "run",
        &[],
    );
}

#[test]
#[should_panic(expected = "function table entry 9 is unavailable during descriptor interpretation")]
fn an_indirect_call_to_an_unavailable_function_reports_its_index() {
    interpret(
        r#"(module
            (type $callback (func))
            (table 2 funcref)
            (func (export "run") i32.const 9 call_indirect (type $callback))
        )"#,
        "run",
        &[],
    );
}

#[test]
#[should_panic(expected = "indirect call type mismatch at function table index 0")]
fn indirect_calls_check_the_function_type() {
    interpret(
        r#"(module
            (type $callback (func))
            (table 1 funcref)
            (func $wrong (result i32) i32.const 42)
            (elem (i32.const 0) $wrong)
            (func (export "run") i32.const 0 call_indirect (type $callback))
        )"#,
        "run",
        &[],
    );
}

#[test]
fn unrelated_tables_do_not_overwrite_function_entries() {
    interpret(
        r#"(module
            (type $callback (func))
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $describe (param i32)))
            (table $functions 1 funcref)
            (table $objects 1 externref)
            (func $callback i32.const 42 call $describe)
            (elem (table $functions) (i32.const 0) func $callback)
            (elem (table $objects) (i32.const 0) externref (ref.null extern))
            (func (export "run") i32.const 0 call_indirect (type $callback))
        )"#,
        "run",
        &[42],
    );
}
