;; A memory64 module as rustc and wasm-bindgen emit it for
;; `wasm64-unknown-unknown` with atomics: an `i64` memory, `i64` pointer
;; globals, and `f64` words for `__wbindgen_malloc` and `__wbindgen_free`.
(module
  (import "env" "memory" (memory i64 6 1024 shared))
  (func $__wasm_init_tls (export "__wasm_init_tls") (param i64)
    (i32.const 232323)
    drop
  )
  (func $__wbindgen_malloc (export "__wbindgen_malloc") (param f64 f64) (result f64)
    f64.const 999999
  )
  (func $start
    i32.const 101010
    drop
  )
  (func $__wbindgen_free (export "__wbindgen_free") (param f64 f64 f64))
  (global (export "__tls_size") i64 (i64.const 128))
  (global (export "__tls_align") i64 (i64.const 8))
  (global (export "__tls_base") (mut i64) (i64.const 0))
  ;; stack pointer
  (global (mut i64) (i64.const 65536))
  (start $start)
)

(; CHECK-ALL:
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i64)))
  (type (;3;) (func (param f64 f64) (result f64)))
  (type (;4;) (func (param f64 f64 f64)))
  (import "env" "memory" (memory (;0;) i64 7 1024 shared))
  (global (;0;) (mut i64) i64.const 0)
  (global (;1;) (mut i64) i64.const 65536)
  (global (;2;) (mut i64) i64.const 0)
  (global (;3;) (mut i64) i64.const 2097152)
  (export "__wbindgen_malloc" (func $__wbindgen_malloc))
  (export "__wbindgen_free" (func $__wbindgen_free))
  (export "__tls_base" (global 0))
  (export "__stack_alloc" (global 2))
  (export "__wbindgen_thread_destroy" (func $__wbindgen_thread_destroy))
  (export "__wbindgen_start" (func 1))
  (func $__wbindgen_thread_destroy (;0;) (type 4) (param f64 f64 f64)
    local.get 0
    i64.trunc_sat_f64_u
    i64.const 0
    i64.ne
    if ;; label = @1
      local.get 0
      f64.const 0x1p+7 (;=128;)
      f64.const 0x1p+3 (;=8;)
      call $__wbindgen_free
    else
      global.get 0
      f64.convert_i64_u
      f64.const 0x1p+7 (;=128;)
      f64.const 0x1p+3 (;=8;)
      call $__wbindgen_free
      i64.const -9223372036854775808
      global.set 0
    end
    local.get 1
    i64.trunc_sat_f64_u
    i64.const 0
    i64.ne
    if ;; label = @1
      local.get 1
      local.get 2
      f64.const 0x1p+21 (;=2097152;)
      local.get 2
      i64.trunc_sat_f64_u
      i64.const 0
      i64.ne
      select
      f64.const 0x1p+4 (;=16;)
      call $__wbindgen_free
    else
      i64.const 458752
      global.set 1
      loop ;; label = @2
        i64.const 393220
        i32.const 0
        i32.const 1
        i32.atomic.rmw.cmpxchg
        if ;; label = @3
          i64.const 393220
          i32.const 1
          i64.const -1
          memory.atomic.wait32
          drop
          br 1 (;@2;)
        else
        end
      end
      global.get 2
      f64.convert_i64_u
      global.get 3
      f64.convert_i64_u
      f64.const 0x1p+4 (;=16;)
      call $__wbindgen_free
      i64.const 393220
      i32.const 0
      i32.atomic.store
      i64.const 393220
      i32.const 1
      memory.atomic.notify
      drop
      i64.const 0
      global.set 2
    end
  )
  (func (;1;) (type 1) (param i32)
    (local i32 i64)
    call $start
    i64.const 393216
    i32.const 1
    i32.atomic.rmw.add
    local.tee 1
    if ;; label = @1
      local.get 0
      if ;; label = @2
        local.get 0
        i64.extend_i32_u
        global.set 3
      else
      end
      i64.const 458752
      global.set 1
      loop ;; label = @2
        i64.const 393220
        i32.const 0
        i32.const 1
        i32.atomic.rmw.cmpxchg
        if ;; label = @3
          i64.const 393220
          i32.const 1
          i64.const -1
          memory.atomic.wait32
          drop
          br 1 (;@2;)
        else
        end
      end
      global.get 3
      f64.convert_i64_u
      f64.const 0x1p+4 (;=16;)
      call $__wbindgen_malloc
      i64.trunc_sat_f64_u
      local.tee 2
      i64.const 393220
      i32.const 0
      i32.atomic.store
      i64.const 393220
      i32.const 1
      memory.atomic.notify
      drop
      global.set 2
      global.get 2
      global.get 3
      i64.add
      global.set 1
    else
    end
    f64.const 0x1p+7 (;=128;)
    f64.const 0x1p+3 (;=8;)
    call $__wbindgen_malloc
    i64.trunc_sat_f64_u
    global.set 0
    global.get 0
    call $__wasm_init_tls
  )
  (func $__wasm_init_tls (;2;) (type 2) (param i64)
    i32.const 232323
    drop
  )
  (func $start (;3;) (type 0)
    i32.const 101010
    drop
  )
  (func $__wbindgen_malloc (;4;) (type 3) (param f64 f64) (result f64)
    f64.const 0x1.e847ep+19 (;=999999;)
  )
  (func $__wbindgen_free (;5;) (type 4) (param f64 f64 f64))
)
;)
