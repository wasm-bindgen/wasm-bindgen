;; @xform export "foo" (externref_owned)

(module
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_grow"
    (func $grow (param i32) (result i32)))
  (func $foo (export "foo") (param i32)
    (local i32)
    i32.const 0
    local.tee 0
    call $grow
    drop)
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param externref)))
  (table $__wbindgen_externrefs (;0;) 1024 externref)
  (export "foo" (func $"foo externref shim"))
  (func $foo (;0;) (type 1) (param i32)
    (local i32)
    i32.const 0
    local.tee 0
    local.set 1
    ref.null extern
    local.get 1
    table.grow $__wbindgen_externrefs
    drop
  )
  (func $"foo externref shim" (;1;) (type 2) (param externref)
    (local i32)
    call $alloc
    local.tee 1
    local.get 0
    table.set $__wbindgen_externrefs
    local.get 1
    call $foo
  )
  (func $alloc (;2;) (type 0) (result i32)
    i32.const 0
  )
  (@custom "target_features" (after code) "\01+\0freference-types")
)
;)
