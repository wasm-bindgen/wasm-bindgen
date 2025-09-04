;; @xform export "foo" (externref_owned)

(module
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_set_null"
    (func $set-null (param i32)))
  (func $foo (export "foo") (param i32)
    local.get 0
    call $set-null)
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param externref)))
  (func $"foo externref shim" (;0;) (type 2) (param externref)
    (local i32)
    call $alloc
    local.tee 1
    local.get 0
    table.set 0
    local.get 1
    call $foo
  )
  (func $foo (;1;) (type 1) (param i32)
    local.get 0
    ref.null extern
    table.set 0
  )
  (func $alloc (;2;) (type 0) (result i32)
    i32.const 0
  )
  (table (;0;) 128 externref)
  (export "foo" (func $"foo externref shim"))
  (@custom "target_features" (after code) "\01+\0freference-types")
)
;)
