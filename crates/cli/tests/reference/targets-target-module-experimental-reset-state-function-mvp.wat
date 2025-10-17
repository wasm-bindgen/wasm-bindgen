(module $targets_reftest_mvp.wasm
  (type (;0;) (func (result f64)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (import "./reference_test_bg.js" "__wbg_random_c82d91f28994c195" (func (;0;) (type 0)))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "add_that_might_fail" (func $add_that_might_fail))
  (func $add_that_might_fail (;1;) (type 1) (param i32 i32) (result i32))
)
