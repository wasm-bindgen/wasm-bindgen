;; @xform export "foo" (externref_borrowed)

(module
  (func $foo (export "foo") (param i32))
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param externref)))
  (func $"foo externref shim" (;0;) (type 1) (param externref)
    (local i32)
    global.get 0
    i32.const 1
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    table.set 0
    local.get 1
    call $foo
    local.get 1
    ref.null extern
    table.set 0
    local.get 1
    i32.const 1
    i32.add
    global.set 0
  )
  (func $foo (;1;) (type 0) (param i32))
  (table (;0;) 128 externref)
  (global (;0;) (mut i32) i32.const 128)
  (export "foo" (func $"foo externref shim"))
  (@custom "target_features" (after code) "\01+\0freference-types")
)
;)
