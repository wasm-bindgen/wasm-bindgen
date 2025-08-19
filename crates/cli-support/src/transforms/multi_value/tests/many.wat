;; @xform export "foo" (i32 f32 f64 i64)

(module
  (global (mut i32) (i32.const 0))
  (memory 1)

  (func $foo (export "foo") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32 f32 f64 i64)))
  (type (;1;) (func (param i32)))
  (func $"foo multivalue shim" (;0;) (type 0) (result i32 f32 f64 i64)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $foo
    local.get 0
    i32.load
    local.get 0
    f32.load offset=4
    local.get 0
    f64.load offset=8
    local.get 0
    i64.load offset=16
    local.get 0
    i32.const 32
    i32.add
    global.set 0
  )
  (func $foo (;1;) (type 1) (param i32))
  (memory (;0;) 1)
  (global (;0;) (mut i32) i32.const 0)
  (export "foo" (func $"foo multivalue shim"))
  (@custom "target_features" (after code) "\01+\0amultivalue")
)
;)
