;; @xform export "i32" (i32)
;; @xform export "i64" (i64)
;; @xform export "f32" (f32)
;; @xform export "f64" (f64)

(module
  (global (mut i32) (i32.const 0))
  (memory 1)

  (func $i32 (export "i32") (param i32))
  (func $i64 (export "i64") (param i32))
  (func $f32 (export "f32") (param i32))
  (func $f64 (export "f64") (param i32))
)

(; CHECK-ALL:
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (result i64)))
  (type (;2;) (func (result f32)))
  (type (;3;) (func (result f64)))
  (type (;4;) (func (param i32)))
  (func $"i32 multivalue shim" (;0;) (type 0) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $i32
    local.get 0
    i32.load
    local.get 0
    i32.const 16
    i32.add
    global.set 0
  )
  (func $"i64 multivalue shim" (;1;) (type 1) (result i64)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $i64
    local.get 0
    i64.load
    local.get 0
    i32.const 16
    i32.add
    global.set 0
  )
  (func $"f32 multivalue shim" (;2;) (type 2) (result f32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $f32
    local.get 0
    f32.load
    local.get 0
    i32.const 16
    i32.add
    global.set 0
  )
  (func $"f64 multivalue shim" (;3;) (type 3) (result f64)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $f64
    local.get 0
    f64.load
    local.get 0
    i32.const 16
    i32.add
    global.set 0
  )
  (func $i32 (;4;) (type 4) (param i32))
  (func $i64 (;5;) (type 4) (param i32))
  (func $f32 (;6;) (type 4) (param i32))
  (func $f64 (;7;) (type 4) (param i32))
  (memory (;0;) 1)
  (global (;0;) (mut i32) i32.const 0)
  (export "i32" (func $"i32 multivalue shim"))
  (export "i64" (func $"i64 multivalue shim"))
  (export "f32" (func $"f32 multivalue shim"))
  (export "f64" (func $"f64 multivalue shim"))
  (@custom "target_features" (after code) "\01+\0amultivalue")
)
;)
