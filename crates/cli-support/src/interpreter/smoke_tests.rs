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

            (global (mut i32) (i32.const 0))

            (func $foo
                (local i32)
                global.get 0
                local.set 0
                local.get 0
                call $__wbindgen_describe
                local.get 0
                global.set 0
            )

            (export "foo" (func $foo))
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

            (global (mut i32) (i32.const 0))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get 0
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set 0

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
                global.set 0
            )

            (export "foo" (func $foo))
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

            (global i32 (i32.const 0))
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
            (global (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get 0
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set 0

                try
                    i32.const 1
                    call $__wbindgen_describe
                catch_all
                end

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set 0
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
            (global (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get 0
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set 0

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
                global.set 0
            )

            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[1]);
}

#[test]
fn blocks() {
    let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global (mut i32) (i32.const 0))
            (memory 1)

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get 0
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set 0

                (block
                    i32.const 0
                    call $__wbindgen_describe
                )

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set 0
            )
            (export "foo" (func $foo))
        )
    "#;
    interpret(wat, "foo", &[0]);
}
