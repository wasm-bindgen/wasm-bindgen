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

            (global $sp (mut i32) (i32.const 0))

            (func $foo
                (local i32)
                global.get $sp
                local.set 0
                local.get 0
                call $__wbindgen_describe
                local.get 0
                global.set $sp
            )

            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
        )
    "#;
    // __wbindgen_describe is called with a global - in Frame.eval we assume all access to globals is the stack pointer
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

            (global $sp (mut i32) (i32.const 0))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $sp
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $sp

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
                global.set $sp
            )

            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
        )
    "#;
    interpret(wat, "foo", &[1, 50331650, 3]);
}

#[test]
fn calling_functions() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $sp i32 (i32.const 0))
            (memory 1)

            (func $foo
                call $bar
            )

            (func $bar
                i32.const 0
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
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
            (global $sp (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $sp
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $sp

                try
                    i32.const 1
                    call $__wbindgen_describe
                catch_all
                end

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $sp
            )

            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
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
            (global $sp (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $sp
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $sp

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
                global.set $sp
            )

            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
        )
    "#;
    interpret(wat, "foo", &[1]);
}

#[test]
fn calling_functions_with_args() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global $sp i32 (i32.const 0))
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
            (export "__stack_pointer" (global $sp))
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

            (global $sp i32 (i32.const 0))
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
            (export "__stack_pointer" (global $sp))
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

            (global $sp (mut i32) (i32.const 0))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get $sp
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set $sp

                (block
                    i32.const 0
                    call $__wbindgen_describe
                )

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set $sp
            )
            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
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

            ;; __stack_pointer should be handled specially
            (global $sp (mut i32) (i32.const 0))
            ;; Other globals should be stored separately
            (global $other1 i32 (i32.const 42))
            (global $other2 (mut i32) (i32.const 100))

            (func $foo
                ;; Read other global - should get 42, not stack pointer
                global.get $other1
                call $__wbindgen_describe

                ;; Read stack pointer - should get 32768
                global.get $sp
                call $__wbindgen_describe

                ;; Modify other global
                i32.const 200
                global.set $other2

                ;; Read modified other global
                global.get $other2
                call $__wbindgen_describe

                ;; Modify stack pointer
                global.get $sp
                i32.const 16
                i32.sub
                global.set $sp

                ;; Read stack pointer again - should get 32752
                global.get $sp
                call $__wbindgen_describe

                ;; Restore stack pointer
                global.get $sp
                i32.const 16
                i32.add
                global.set $sp
            )

            (export "foo" (func $foo))
            (export "__stack_pointer" (global $sp))
        )
    "#;
    interpret(wat, "foo", &[42, 32768, 200, 32752]);
}
