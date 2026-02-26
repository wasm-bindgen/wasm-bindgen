(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param externref)))
  (type (;4;) (func (param externref) (result i32)))
  (import "env" "my_import" (func $my_import (;0;) (type 2)))
  (import "__wbindgen_placeholder__" "__wbindgen_jstag" (tag (;0;) (type 3) (param externref)))
  (table $externrefs (;0;) 128 externref)
  (memory (;0;) 1)
  (global $__instance_terminated (;0;) i32 i32.const 1048576)
  (export "__instance_terminated" (global $__instance_terminated))
  (export "my_import" (func $my_import))
  (export "__externref_table" (table $externrefs))
  (export "__externref_table_alloc" (func $__externref_table_alloc))
  (export "exn_store" (func $exn_store))
  (func $"my_import catch wrapper" (;1;) (type 2) (param i32) (result i32)
    (local i32 externref)
    try (result i32) ;; label = @1
      local.get 0
      call $my_import
      i32.const 1048576
      i32.load
      if ;; label = @2
        unreachable
      else
      end
    catch 0
      i32.const 1048576
      i32.load
      if ;; label = @2
        unreachable
      else
      end
      local.set 2
      call $__externref_table_alloc
      local.tee 1
      local.get 2
      table.set $externrefs
      local.get 1
      call $exn_store
      i32.const 0
    catch_all
      i32.const 1048576
      i32.load
      if ;; label = @2
        unreachable
      else
      end
      rethrow 0 (;@1;)
    end
  )
  (func $__externref_table_alloc (;2;) (type 0) (result i32)
    i32.const 42
  )
  (func $exn_store (;3;) (type 1) (param i32))
)