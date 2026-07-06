(module
  (import "env" "memory" (memory 10 1024 shared))
  (func $__wasm_init_tls (export "__wasm_init_tls") (param i32)
    (i32.const 232323)
    drop
  )
  (func $__wbindgen_malloc (export "__wbindgen_malloc") (param i32 i32) (result i32)
    call $preexisting_chunk
  )
  ;; Mimics dlmalloc reading the linker-provided `[__heap_base, __heap_end)`
  ;; range as two inlined constants and computing its length at runtime. The
  ;; transform must shift both bounds up past the reserved page.
  (func $preexisting_chunk (export "preexisting_chunk") (result i32)
    i32.const 655360 ;; &__heap_end (initial memory = 10 pages)
    i32.const 327680 ;; &__heap_base
    i32.sub
  )
  (func $start
    i32.const 101010
    drop
  )
  (func $__wbindgen_free (export "__wbindgen_free") (param i32 i32 i32))
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
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32)))
  (import "env" "memory" (memory (;0;) 11 1024 shared))
  (global (;0;) i32 i32.const 393216)
  (global (;1;) (mut i32) i32.const 0)
  (global (;2;) (mut i32) i32.const 65536)
  (global (;3;) (mut i32) i32.const 0)
  (global (;4;) (mut i32) i32.const 2097152)
  (export "__wbindgen_malloc" (func $__wbindgen_malloc))
  (export "preexisting_chunk" (func $preexisting_chunk))
  (export "__wbindgen_free" (func $__wbindgen_free))
  (export "__heap_base" (global 0))
  (export "__tls_base" (global 1))
  (export "__stack_alloc" (global 3))
  (export "__wbindgen_thread_destroy" (func $__wbindgen_thread_destroy))
  (export "__wbindgen_start" (func 1))
  (func $__wbindgen_thread_destroy (;0;) (type 4) (param i32 i32 i32)
    local.get 0
    if ;; label = @1
      local.get 0
      i32.const 128
      i32.const 4
      call $__wbindgen_free
    else
      global.get 1
      i32.const 128
      i32.const 4
      call $__wbindgen_free
      i32.const -2147483648
      global.set 1
    end
    local.get 1
    if ;; label = @1
      local.get 1
      local.get 2
      i32.const 2097152
      local.get 2
      select
      i32.const 16
      call $__wbindgen_free
    else
      i32.const 393216
      global.set 2
      loop ;; label = @2
        i32.const 327684
        i32.const 0
        i32.const 1
        i32.atomic.rmw.cmpxchg
        if ;; label = @3
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
      global.get 4
      i32.const 16
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
    end
  )
  (func (;1;) (type 2) (param i32)
    (local i32 i32)
    call $start
    i32.const 327680
    i32.const 1
    i32.atomic.rmw.add
    local.tee 2
    if ;; label = @1
      local.get 0
      if ;; label = @2
        local.get 0
        global.set 4
      else
      end
      i32.const 393216
      global.set 2
      loop ;; label = @2
        i32.const 327684
        i32.const 0
        i32.const 1
        i32.atomic.rmw.cmpxchg
        if ;; label = @3
          i32.const 327684
          i32.const 1
          i64.const -1
          memory.atomic.wait32
          drop
          br 1 (;@2;)
        else
        end
      end
      global.get 4
      i32.const 16
      call $__wbindgen_malloc
      local.tee 1
      i32.const 327684
      i32.const 0
      i32.atomic.store
      i32.const 327684
      i32.const 1
      memory.atomic.notify
      drop
      global.set 3
      global.get 3
      global.get 4
      i32.add
      global.set 2
    else
    end
    i32.const 128
    i32.const 4
    call $__wbindgen_malloc
    global.set 1
    global.get 1
    call $__wasm_init_tls
  )
  (func $preexisting_chunk (;2;) (type 1) (result i32)
    i32.const 720896
    i32.const 393216
    i32.sub
  )
  (func $__wasm_init_tls (;3;) (type 2) (param i32)
    i32.const 232323
    drop
  )
  (func $start (;4;) (type 0)
    i32.const 101010
    drop
  )
  (func $__wbindgen_malloc (;5;) (type 3) (param i32 i32) (result i32)
    call $preexisting_chunk
  )
  (func $__wbindgen_free (;6;) (type 4) (param i32 i32 i32))
)
;)
