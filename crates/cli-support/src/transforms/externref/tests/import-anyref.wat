;; @xform import "" "a" (externref_borrowed)

(module
  (import "" "a" (func $a (param i32)))
  (func (export "foo")
    i32.const 0
    call $a)
  (func $alloc (export "__externref_table_alloc") (result i32)
    i32.const 0)
  (func $dealloc (export "__externref_table_dealloc") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param externref)))
  (import "" "a" (func $a (;0;) (type 2)))
  (func $"a externref shim" (;1;) (type 1) (param i32)
    local.get 0
    table.get 0
    call $a
  )
  (func (;2;) (type 0)
    i32.const 0
    call $"a externref shim"
  )
  (table (;0;) 128 externref)
  (export "foo" (func 2))
  (@custom "target_features" (after code) "\01+\0freference-types")
)
;)
