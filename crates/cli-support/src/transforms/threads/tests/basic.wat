(module
  (import "env" "memory" (memory 6 1024 shared))
  (func $__wasm_init_tls (export "__wasm_init_tls") (param i32)
    (i32.const 232323)
    drop
  )
  (func $__wbindgen_malloc (export "__wbindgen_malloc") (param i32) (result i32)
    i32.const 999999
  )
  (func $start
    i32.const 101010
    drop
  )
  (func $__wbindgen_free (export "__wbindgen_free") (param i32 i32))
  (global (export "__heap_base") i32 (i32.const 327680))
  (global (export "__tls_size") i32 (i32.const 128))
  (global (export "__tls_align") i32 (i32.const 4))
  (global (export "__tls_base") (mut i32) (i32.const 0))
  ;; stack pointer
  (global (mut i32) (i32.const 65536))
  (start $start)
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32)))
  (import "env" "memory" (memory (;0;) 7 16384 shared))
  (func (;0;) (type 0)
    (local i32)
    call $start
    i32.const 327680
    i32.const 1
    i32.atomic.rmw.add
    if  ;; label = @1
      i32.const 393216
      global.set 2
      loop  ;; label = @2
        i32.const 327684
        i32.const 0
        i32.const 1
        i32.atomic.rmw.cmpxchg
        if  ;; label = @3
          i32.const 327684
          i32.const 1
          i64.const -1
          memory.atomic.wait32
          drop
          br 1 (;@2;)
        else
        end
      end
      i32.const 1048576
      call $__wbindgen_malloc
      local.tee 0
      i32.const 327684
      i32.const 0
      i32.atomic.store
      i32.const 327684
      i32.const 1
      memory.atomic.notify
      drop
      global.set 3
      global.get 3
      i32.const 1048576
      i32.add
      global.set 2
    else
    end
    i32.const 128
    i32.const 4
    drop
    call $__wbindgen_malloc
    global.set 1
    global.get 1
    call $__wasm_init_tls)
  (func $__wbindgen_thread_destroy (type 3) (param i32 i32)
    (local i32 i32)
    local.get 0
    if  ;; label = @1
      local.get 0
      i32.const 128
      call $__wbindgen_free
    else
      global.get 1
      i32.const 128
      call $__wbindgen_free
      i32.const -2147483648
      global.set 1
    end
    local.get 1
    if  ;; label = @1
      local.get 1
      i32.const 1048576
      call $__wbindgen_free
    else
      i32.const 393216
      global.set 2
      loop  ;; label = @2
        i32.const 327684
        i32.const 0
        i32.const 1
        i32.atomic.rmw.cmpxchg
        if  ;; label = @3
          i32.const 327684
          i32.const 1
          i64.const -1
          memory.atomic.wait32
          drop
          br 1 (;@2;)
        else
        end
      end
      global.get 3
      i32.const 1048576
      call $__wbindgen_free
      i32.const 327684
      i32.const 0
      i32.atomic.store
      i32.const 327684
      i32.const 1
      memory.atomic.notify
      drop
      i32.const 0
      global.set 3
    end)
  (func $__wasm_init_tls (type 1) (param i32)
    i32.const 232323
    drop)
  (func $start (type 0)
    i32.const 101010
    drop)
  (func $__wbindgen_malloc (type 2) (param i32) (result i32)
    i32.const 999999)
  (func $__wbindgen_free (type 3) (param i32 i32))
  (global (;0;) i32 i32.const 393216)
  (global (;1;) (mut i32) i32.const 0)
  (global (;2;) (mut i32) i32.const 65536)
  (global (;3;) (mut i32) i32.const 0)
  (export "__wbindgen_malloc" (func $__wbindgen_malloc))
  (export "__wbindgen_free" (func $__wbindgen_free))
  (export "__heap_base" (global 0))
  (export "__tls_base" (global 1))
  (export "__stack_alloc" (global 3))
  (export "__wbindgen_thread_destroy" (func $__wbindgen_thread_destroy))
  (start 0))
;)
