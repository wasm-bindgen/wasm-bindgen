;; @xform export "a" (other externref_borrowed other externref_owned other)

(module
  (func $a (export "a") (param f32 i32 i64 i32 i32))
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param f32 i32 i64 i32 i32)))
  (type (;2;) (func (param f32 externref i64 externref i32)))
  (table $__wbindgen_externrefs (;0;) 1024 externref)
  (global (;0;) (mut i32) i32.const 1024)
  (export "a" (func $"a externref shim"))
  (func $"a externref shim" (;0;) (type 2) (param f32 externref i64 externref i32)
    (local i32 i32)
    global.get 0
    i32.const 1
    i32.sub
    local.tee 5
    global.set 0
    local.get 0
    local.get 5
    local.get 1
    table.set $__wbindgen_externrefs
    local.get 5
    local.get 2
    call $alloc
    local.tee 6
    local.get 3
    table.set $__wbindgen_externrefs
    local.get 6
    local.get 4
    call $a
    local.get 5
    ref.null extern
    table.set $__wbindgen_externrefs
    local.get 5
    i32.const 1
    i32.add
    global.set 0
  )
  (func $alloc (;1;) (type 0) (result i32)
    i32.const 0
  )
  (func $a (;2;) (type 1) (param f32 i32 i64 i32 i32))
  (@custom "target_features" (after code) "\01+\0freference-types")
)
;)
