//! Wasm instrumentation for JSPI (JS Promise Integration) support.
//!
//! JSPI preserves wasm locals and the value stack across a suspension, but
//! *not* globals — and LLVM's shadow stack lives in linear memory behind the
//! `__stack_pointer` global. If another fiber (or plain synchronous wasm) runs
//! while a fiber is suspended, it would clobber the suspended fiber's shadow
//! stack frames.
//!
//! This pass makes suspension safe with an evacuate-on-suspend scheme, done
//! entirely inside the wasm module so the save/restore is atomic with fiber
//! execution (JS-side `finally` ordering cannot guarantee this — two fibers
//! settling in the same microtask checkpoint would race):
//!
//! - Each `#[wasm_bindgen(jspi)]` export is wrapped in a function that records
//!   the shadow-stack watermark at fiber entry in the `__jspi_stack_base`
//!   global (saving/restoring the previous value in a local for nesting).
//!
//! - Each `#[wasm_bindgen(suspending)]` import is wrapped in a function that,
//!   before the call, copies the fiber's live shadow-stack region
//!   `[SP, base)` out to a `__wbindgen_malloc`'d buffer and resets SP to the
//!   base — leaving the shadow stack clean for whatever runs during the
//!   suspension. Immediately after the call returns (the first instructions
//!   executed on resume), the region is copied back to its original address,
//!   SP and the base global are restored from locals (which JSPI preserved),
//!   and the buffer is freed. The call sits in a `try_table` whose
//!   `catch_all_ref` arm performs the same restore before rethrowing, so an
//!   exception delivered at the resume point (e.g. a rejected promise)
//!   unwinds over a restored shadow stack.
//!
//! - Suspending imports also marked `catch` (including the
//!   `__wbindgen_jspi_suspend` intrinsic backing
//!   `js_sys::futures::jspi::block_on_promise`) additionally catch
//!   `WebAssembly.JSTag` exceptions — JSPI throws a rejected promise's reason
//!   into wasm at the resume point — and convert them to data: the reason
//!   becomes the return value and the `__wbindgen_jspi_rejected` flag in
//!   linear memory is set. Fulfillment stores 0. Both stores are in-fiber at
//!   resume, so reading the flag after the call is race-free, and the macro
//!   marshals it into the `Result` return.
//!
//! The suspending-import entries in `implements` are repointed at the
//! wrappers so that the later catch-wrapper pass wraps *outside* them:
//! rejections are consumed innermost as data, while everything else
//! (SuspendError misuse, rethrown exceptions) still reaches the abort/catch
//! machinery — over a restored stack.
//!
//! Because every suspension drains the shadow stack to its entry watermark,
//! the same address range is time-multiplexed between fibers: interior
//! pointers into a fiber's stack are valid whenever that fiber (or its
//! callees) can actually execute. Fibers run on the full main shadow stack,
//! so no fixed per-fiber stack size or overflow guard is needed, and memory
//! cost is proportional to the live stack depth at each suspension.
//!
//! A fiber's exit SP depends on whether it suspended: a fiber may be entered
//! with live shadow frames above it (sync export → JS → promising call),
//! but any suspension means those frames unwound while it was pending —
//! resume only ever happens on an empty stack — so a suspended fiber's
//! completion resets SP to the empty-stack top (`__jspi_stack_top`,
//! snapshotted in the start function) rather than the entry offset, which
//! would permanently leak the region above it. A fiber that never suspends
//! completes synchronously under its still-live callers and keeps the entry
//! offset.

use crate::wit::{AdapterKind, Instruction, NonstandardWitSection, WasmBindgenAux};
use anyhow::{anyhow, bail, Error};

/// The raw wasm export name of the `jspi::spawn_local` poll trampoline in
/// `js_sys::futures::jspi`.
pub const TASK_POLL_EXPORT: &str = "__wbg_jspi_task_poll";
use std::collections::HashMap;
use walrus::ir::{self, BinaryOp, MemArg, UnaryOp, Value};
use walrus::{
    ConstExpr, ExportItem, FunctionBuilder, FunctionId, GlobalId, InstrSeqBuilder, MemoryId,
    Module, RefType, TagId, ValType,
};

pub fn run(
    module: &mut Module,
    aux: &mut WasmBindgenAux,
    wit: &mut NonstandardWitSection,
    externref: bool,
) -> Result<(), Error> {
    // The wasm-level export ids of `#[wasm_bindgen(jspi)]` exports.
    let mut jspi_exports = Vec::new();
    for (id, export) in crate::sorted_iter(&aux.export_map) {
        if !export.jspi {
            continue;
        }
        if let Some(adapter) = wit.adapters.get(id) {
            let AdapterKind::Local { instructions } = &adapter.kind else {
                continue;
            };
            let export_id = instructions
                .iter()
                .find_map(|i| match i.instr {
                    Instruction::CallExport(e) => Some(e),
                    _ => None,
                })
                .ok_or_else(|| {
                    anyhow!("jspi export adapter never calls the underlying function")
                })?;
            jspi_exports.push(export_id);
        }
    }

    // The spawn-machinery signal is attribute-driven: `spawn_local` polls
    // can only ever be promising-entered if the ambient JSPI context can be
    // set, and the base case for the context is a `#[wasm_bindgen(jspi)]`
    // export (promising-entered polls then inherit it transitively). So the
    // poll trampoline — a raw js-sys export recognized by name — is kept and
    // given the jspi-export treatment exactly when the module has jspi
    // exports and links the spawn intrinsics; otherwise it is deleted and
    // the spawn intrinsics are stubbed, so a module that never uses the
    // attributes carries no JSPI machinery and keeps running on engines
    // without exnref/JSPI.
    //
    // Stubbing rather than GC: the wake path (waker vtable → `wake` →
    // `__wbindgen_jspi_spawn_poll`) stays reachable through the vtable's
    // element-segment entries whenever the function table is otherwise
    // live. The stubs can never be called (waking requires a poll to have
    // run, and the ambient context is constant-false without jspi exports);
    // replacing the imports with unreachable local functions removes them
    // from the module, and `gc_module_and_adapters` then prunes their
    // adapters so no JS shims are emitted either.
    let spawn_imports = module
        .imports
        .iter()
        .filter(|i| i.name.contains("__wbindgen_jspi_spawn_"))
        .filter_map(|i| match i.kind {
            walrus::ImportKind::Function(f) => Some(f),
            _ => None,
        })
        .collect::<Vec<_>>();
    let task_poll_export = module
        .exports
        .iter()
        .find(|e| e.name == TASK_POLL_EXPORT)
        .map(|e| e.id());
    if let Some(export_id) = task_poll_export {
        if !jspi_exports.is_empty() && !spawn_imports.is_empty() {
            jspi_exports.push(export_id);
        } else {
            module.exports.delete(export_id);
            for func in spawn_imports {
                module.replace_imported_func(func, |(body, _)| {
                    body.unreachable();
                })?;
            }
        }
    }

    // The wasm-level import shims of `#[wasm_bindgen(suspending)]` imports.
    // Imports also marked `catch` get the rejection-to-data protocol: the
    // JSTag exception thrown at the resume point for a rejected promise is
    // caught in-wasm and reported through the `__wbindgen_jspi_rejected`
    // flag, with the rejection reason as the returned value.
    let suspending_imports = wit
        .implements
        .iter()
        .filter(|(_, _, adapter)| aux.imports_with_suspending.contains(adapter))
        .map(|(_, func, adapter)| (*func, aux.imports_with_catch.contains(adapter)))
        .collect::<Vec<_>>();

    // The ambient-context probe backing context-aware `spawn_local`. It is
    // always rewritten in-wasm — constant `false` when no JSPI
    // instrumentation is present, `__jspi_stack_base != 0` otherwise — so no
    // JS shim is ever emitted for it.
    let in_context_import = module
        .imports
        .iter()
        .find(|i| i.name.contains("__wbindgen_jspi_in_context"))
        .and_then(|i| match i.kind {
            walrus::ImportKind::Function(f) => Some(f),
            _ => None,
        });

    if jspi_exports.is_empty() && suspending_imports.is_empty() {
        if let Some(func) = in_context_import {
            module.replace_imported_func(func, |(body, _)| {
                body.i32_const(0);
            })?;
        }
        return Ok(());
    }

    if !externref {
        bail!(
            "JSPI support requires reference types: the resume value of a \
             suspending import bypasses the JS shim's return conversion, so \
             it must travel as an externref (enabled by default since \
             Rust 1.82)"
        );
    }

    // The single stack-top snapshot and the fiber globals are per-instance,
    // not per-thread, and JSPI itself is a single-threaded proposal.
    if module.memories.iter().any(|m| m.shared) {
        bail!("JSPI is not supported with threads/atomics");
    }

    let sp = aux.stack_pointer.ok_or_else(|| {
        anyhow!(
            "could not locate the `__stack_pointer` global in the Wasm module; \
             JSPI requires it so that suspended fibers' shadow stacks can be \
             saved and restored — ensure the linker retains the symbol name"
        )
    })?;
    let memory = crate::wasm_conventions::get_memory(module)?;
    let malloc = aux
        .jspi_malloc
        .ok_or_else(|| anyhow!("JSPI support requires `__wbindgen_malloc` to be present"))?;
    let free = aux
        .jspi_free
        .ok_or_else(|| anyhow!("JSPI support requires `__wbindgen_free` to be present"))?;
    let ptr_ty = module.globals.get(sp).ty;

    // Watermark of the currently-executing fiber's shadow stack, or 0 when
    // no JSPI fiber is on the stack.
    let base = module
        .globals
        .add_local(ptr_ty, true, false, const_zero(ptr_ty));
    module.globals.get_mut(base).name = Some("__jspi_stack_base".to_string());

    // Whether the currently-executing fiber has suspended at least once.
    // Saved/zeroed at fiber entry and set on every resume; consulted at fiber
    // exit to decide the final SP (see `wrap_export`).
    let suspended = module
        .globals
        .add_local(ValType::I32, true, false, const_zero(ValType::I32));
    module.globals.get_mut(suspended).name = Some("__jspi_suspended".to_string());

    // The empty-stack SP, snapshotted in the start function (which by
    // definition runs on an empty shadow stack). Used instead of reading the
    // SP global's initializer so imported/non-const stack pointers work too.
    let top = module
        .globals
        .add_local(ptr_ty, true, false, const_zero(ptr_ty));
    module.globals.get_mut(top).name = Some("__jspi_stack_top".to_string());
    crate::wasm_conventions::get_or_insert_start_builder(module)
        .func_body()
        .global_get(sp)
        .global_set(top);

    let ctx = JspiContext {
        sp,
        base,
        suspended,
        top,
        memory,
        malloc,
        free,
        ptr_ty,
    };

    // The ambient-context probe: a JSPI context is on the stack iff the
    // innermost fiber base is nonzero.
    if let Some(func) = in_context_import {
        module.replace_imported_func(func, |(body, _)| {
            body.global_get(base);
            match ptr_ty {
                ValType::I64 => body.unop(UnaryOp::I64Eqz),
                _ => body.unop(UnaryOp::I32Eqz),
            };
            body.unop(UnaryOp::I32Eqz);
        })?;
    }

    for export_id in jspi_exports {
        let inner = match module.exports.get(export_id).item {
            ExportItem::Function(f) => f,
            _ => bail!("jspi export is not a function"),
        };
        let wrapper = wrap_export(module, inner, ctx);
        module.exports.get_mut(export_id).item = wrapper.into();
    }

    if suspending_imports.is_empty() {
        return Ok(());
    }

    let restore = make_restore_helper(module, ctx);

    // The rejection protocol for `catch`-marked suspending imports, if any.
    let has_catch = suspending_imports.iter().any(|(_, c)| *c);
    let rejection = if has_catch {
        let addr = aux.jspi_rejected.ok_or_else(|| {
            anyhow!(
                "could not locate the `__wbindgen_jspi_rejected` flag static; \
                 it is defined by the wasm-bindgen runtime"
            )
        })?;
        // The JS glue wires `WebAssembly.JSTag` up to any present tag import
        // (see `generate_jstag_import`); `aux.js_tag` is deliberately left
        // unset — it means "catch imports use wasm catch wrappers", which is
        // the catch-wrapper transform's decision, not ours.
        let js_tag = crate::transforms::catch_handler::get_or_import_js_tag(module);
        Some(Rejection { js_tag, addr })
    } else {
        None
    };

    let mut wrappers = HashMap::new();
    for (import, catch) in suspending_imports {
        if wrappers.contains_key(&import) {
            continue;
        }
        let rejection = if catch {
            // The rejection arm stores the caught JSTag payload (externref)
            // into the result local, so the return ABI must be a single
            // externref (guaranteed by the macro, which marshals suspending
            // returns as `JsValue`).
            let ty = module.types.get(module.funcs.get(import).ty());
            if ty.results() != [ValType::Ref(RefType::EXTERNREF)] {
                bail!(
                    "unexpected ABI for a `catch` suspending import: expected \
                     an externref return, found {:?}",
                    ty.results()
                );
            }
            rejection
        } else {
            None
        };
        let wrapper = wrap_suspending(module, import, ctx, restore, rejection);
        wrappers.insert(import, wrapper);
    }
    rewrite_calls(module, &wrappers);

    // Repoint the `implements` entries at the wrappers so the catch-wrapper
    // pass wraps outside them (see module docs).
    for (_, func, adapter) in wit.implements.iter_mut() {
        if aux.imports_with_suspending.contains(adapter) {
            if let Some(wrapper) = wrappers.get(func) {
                *func = *wrapper;
            }
        }
    }

    Ok(())
}

#[derive(Clone, Copy)]
struct JspiContext {
    sp: GlobalId,
    base: GlobalId,
    suspended: GlobalId,
    top: GlobalId,
    memory: MemoryId,
    malloc: FunctionId,
    free: FunctionId,
    ptr_ty: ValType,
}

/// The rejection-to-data protocol for the suspend intrinsic.
#[derive(Clone, Copy)]
struct Rejection {
    js_tag: TagId,
    /// Linear-memory address of the `__wbindgen_jspi_rejected` u32 flag.
    addr: u64,
}

fn const_zero(ty: ValType) -> ConstExpr {
    match ty {
        ValType::I64 => ConstExpr::Value(Value::I64(0)),
        _ => ConstExpr::Value(Value::I32(0)),
    }
}

/// Wrap a `#[wasm_bindgen(jspi)]` export:
///
/// ```wat
/// (func $wrapper (param ...) (result ...)
///     (local $prev ptr) (local $base ptr) (local $prev_suspended i32)
///     global.get $__jspi_stack_base
///     local.set $prev
///     global.get $__stack_pointer
///     local.tee $base
///     global.set $__jspi_stack_base
///     ;; push the fiber node: `prev` is stored at [base - 16], inside the
///     ;; fiber's own shadow-stack region so it is evacuated and restored
///     ;; with it, and reachable by address from the suspend wrapper
///     local.get $base
///     i32.const 16
///     i32.sub
///     global.set $__stack_pointer
///     local.get $base
///     local.get $prev
///     i32.store offset=-16 (conceptually; emitted as base-16 + store)
///     global.get $__jspi_suspended
///     local.set $prev_suspended
///     i32.const 0
///     global.set $__jspi_suspended
///     block $done (result ...)
///         block $catch_all (result exnref)
///             try_table (result ...) (catch_all_ref $catch_all)
///                 local.get <params>...
///                 call $inner
///             end
///             ;; if this fiber suspended, its entry-time callers unwound
///             ;; while it was pending (resume only ever happens on an empty
///             ;; stack), so the correct exit state is the empty-stack SP and
///             ;; no active fiber — keeping the entry offset would
///             ;; permanently leak the region above it, and `prev` refers to
///             ;; a context that no longer exists
///             global.get $__jspi_suspended
///             if
///                 global.get $__jspi_stack_top
///                 global.set $__stack_pointer
///                 i32.const 0 (ptr)
///                 global.set $__jspi_stack_base
///             else
///                 local.get $base
///                 global.set $__stack_pointer
///                 local.get $prev
///                 global.set $__jspi_stack_base
///             end
///             local.get $prev_suspended
///             global.set $__jspi_suspended
///             br $done
///         end
///         ;; exceptional completion (uncaught rejection, panic unwind, ...):
///         ;; run the same exit-state logic so the fiber globals aren't left
///         ;; stale, then keep unwinding — the exception becomes a rejection
///         ;; of the promising call's promise
///         local.set $exn
///         <same exit-state logic>
///         local.get $exn
///         throw_ref
///     end)
/// ```
///
/// This runs inside the fiber that `WebAssembly.promising` creates, so the
/// base is published exactly when the fiber starts executing and restored
/// exactly when it completes or suspends, with no window for interleaved
/// microtasks. The node makes `prev` reachable from the suspend wrapper:
/// suspension restores `__jspi_stack_base := prev` because the fiber's
/// segment leaves the stack, keeping the global equal to the innermost
/// fiber segment actually on the stack at all times (concurrent fibers
/// interleave arbitrarily, so a global save/restore chain alone would go
/// stale).
fn wrap_export(module: &mut Module, inner: FunctionId, ctx: JspiContext) -> FunctionId {
    let ty = module.types.get(module.funcs.get(inner).ty());
    let params = ty.params().to_vec();
    let results = ty.results().to_vec();

    let results_ty: ir::InstrSeqType = match results.len() {
        0 => ir::InstrSeqType::Simple(None),
        1 => ir::InstrSeqType::Simple(Some(results[0])),
        _ => module.types.add(&[], &results).into(),
    };
    let exnref_ty: ir::InstrSeqType = ValType::Ref(RefType::EXNREF).into();

    let mut builder = FunctionBuilder::new(&mut module.types, &params, &results);
    let param_locals: Vec<_> = params.iter().map(|ty| module.locals.add(*ty)).collect();
    let prev = module.locals.add(ctx.ptr_ty);
    let base = module.locals.add(ctx.ptr_ty);
    let prev_suspended = module.locals.add(ValType::I32);
    let exn = module.locals.add(ValType::Ref(RefType::EXNREF));

    let sub = ptr_sub(ctx.ptr_ty);

    // Fiber exit: if this fiber suspended, its entry-time context is gone
    // (resume only ever happens on an empty stack), so reset to the
    // empty-stack SP with no active fiber; otherwise pop the node and
    // restore the enclosing fiber context.
    let emit_exit = |seq: &mut InstrSeqBuilder| {
        seq.global_get(ctx.suspended).if_else(
            None,
            |then| {
                then.global_get(ctx.top).global_set(ctx.sp);
                match ctx.ptr_ty {
                    ValType::I64 => then.i64_const(0),
                    _ => then.i32_const(0),
                };
                then.global_set(ctx.base);
            },
            |else_| {
                else_.local_get(base).global_set(ctx.sp);
                else_.local_get(prev).global_set(ctx.base);
            },
        );
        seq.local_get(prev_suspended).global_set(ctx.suspended);
    };

    let try_seq = builder.dangling_instr_seq(results_ty).id();
    let catch_seq = builder.dangling_instr_seq(exnref_ty).id();
    let done_seq = builder.dangling_instr_seq(results_ty).id();

    {
        let mut seq = builder.instr_seq(try_seq);
        for local in &param_locals {
            seq.local_get(*local);
        }
        seq.call(inner);
    }

    {
        let mut seq = builder.instr_seq(catch_seq);
        seq.instr(ir::TryTable {
            seq: try_seq,
            catches: vec![ir::TryTableCatch::CatchAllRef { label: catch_seq }],
        });
        // Normal completion: results flow out of the try_table.
        emit_exit(&mut seq);
        seq.br(done_seq);
    }

    // Exceptional completion (an uncaught rejection, a panic under unwind,
    // ...): run the same exit-state logic so the fiber globals aren't left
    // stale, then keep unwinding — the exception becomes a rejection of the
    // promising call's promise.
    {
        let mut seq = builder.instr_seq(done_seq);
        seq.instr(ir::Block { seq: catch_seq });
        seq.local_set(exn);
        emit_exit(&mut seq);
        seq.local_get(exn);
        seq.instr(ir::ThrowRef {});
    }

    let mut body = builder.func_body();
    body.global_get(ctx.base).local_set(prev);
    body.global_get(ctx.sp).local_tee(base).global_set(ctx.base);
    // Push the fiber node holding `prev` at [base - 16].
    body.local_get(base);
    push_align(&mut body, ctx.ptr_ty);
    body.binop(sub).global_set(ctx.sp);
    body.local_get(base);
    push_align(&mut body, ctx.ptr_ty);
    body.binop(sub).local_get(prev);
    store_ptr(&mut body, ctx);
    body.global_get(ctx.suspended).local_set(prev_suspended);
    body.i32_const(0).global_set(ctx.suspended);
    body.instr(ir::Block { seq: done_seq });

    let wrapper = builder.finish(param_locals, &mut module.funcs);
    let name = module.funcs.get(inner).name.clone();
    module.funcs.get_mut(wrapper).name = name.map(|n| format!("{n} jspi wrapper"));
    wrapper
}

/// Store a pointer-sized value: expects `addr, value` on the stack.
fn store_ptr(body: &mut InstrSeqBuilder, ctx: JspiContext) {
    let (kind, align) = match ctx.ptr_ty {
        ValType::I64 => (ir::StoreKind::I64 { atomic: false }, 8),
        _ => (ir::StoreKind::I32 { atomic: false }, 4),
    };
    body.store(ctx.memory, kind, MemArg { align, offset: 0 });
}

/// Load a pointer-sized value: expects `addr` on the stack.
fn load_ptr(body: &mut InstrSeqBuilder, ctx: JspiContext) {
    let (kind, align) = match ctx.ptr_ty {
        ValType::I64 => (ir::LoadKind::I64 { atomic: false }, 8),
        _ => (ir::LoadKind::I32 { atomic: false }, 4),
    };
    body.load(ctx.memory, kind, MemArg { align, offset: 0 });
}

/// Build the shared post-resume restore helper:
///
/// ```wat
/// (func $__jspi_restore (param $base ptr) (param $len ptr) (param $buf ptr)
///     ;; copy the saved shadow-stack region back to its original address
///     local.get $base
///     local.get $len
///     i32.sub
///     local.get $buf
///     local.get $len
///     memory.copy
///     local.get $base
///     local.get $len
///     i32.sub
///     global.set $__stack_pointer
///     local.get $base
///     global.set $__jspi_stack_base
///     i32.const 1
///     global.set $__jspi_suspended
///     local.get $buf
///     local.get $len
///     i32.const 16
///     call $__wbindgen_free)
/// ```
///
/// The helper only touches locals, globals and bulk memory — it has no
/// shadow-stack frame of its own, so it is safe to run before the caller's
/// frames have been restored. Setting `__jspi_suspended` here is
/// unconditionally correct: JSPI performs promise resolution on the
/// Suspending function's return value, so even a non-Promise return suspends
/// for at least one microtask tick — reaching the restore always means the
/// fiber suspended and its entry-time callers unwound.
fn make_restore_helper(module: &mut Module, ctx: JspiContext) -> FunctionId {
    let params = [ctx.ptr_ty; 3];
    let mut builder = FunctionBuilder::new(&mut module.types, &params, &[]);
    let base = module.locals.add(ctx.ptr_ty);
    let len = module.locals.add(ctx.ptr_ty);
    let buf = module.locals.add(ctx.ptr_ty);

    let sub = ptr_sub(ctx.ptr_ty);
    let mut body = builder.func_body();
    body.local_get(base).local_get(len).binop(sub);
    body.local_get(buf).local_get(len);
    body.instr(ir::MemoryCopy {
        src: ctx.memory,
        dst: ctx.memory,
    });
    body.local_get(base).local_get(len).binop(sub);
    body.global_set(ctx.sp);
    body.local_get(base).global_set(ctx.base);
    body.i32_const(1).global_set(ctx.suspended);
    body.local_get(buf).local_get(len);
    push_align(&mut body, ctx.ptr_ty);
    body.call(ctx.free);

    let helper = builder.finish(vec![base, len, buf], &mut module.funcs);
    module.funcs.get_mut(helper).name = Some("__jspi_restore".to_string());
    helper
}

fn ptr_sub(ty: ValType) -> BinaryOp {
    match ty {
        ValType::I64 => BinaryOp::I64Sub,
        _ => BinaryOp::I32Sub,
    }
}

fn push_align(body: &mut InstrSeqBuilder, ty: ValType) {
    match ty {
        ValType::I64 => body.i64_const(16),
        _ => body.i32_const(16),
    };
}

/// Wrap a `#[wasm_bindgen(suspending)]` import with the evacuate-on-suspend
/// sequence:
///
/// ```wat
/// (func $wrapper (param ...) (result ...)
///     (local $base ptr) (local $len ptr) (local $buf ptr) (local $exn exnref)
///     ;; not inside a fiber: plain call (the engine reports misuse
///     ;; with SuspendError if the import actually tries to suspend)
///     global.get $__jspi_stack_base
///     local.tee $base
///     i32.eqz
///     if
///         local.get <params>...
///         call $import
///         return
///     end
///     ;; evacuate [SP, base) to a malloc'd buffer
///     local.get $base
///     global.get $__stack_pointer
///     i32.sub
///     local.set $len
///     local.get $len
///     i32.const 16
///     call $__wbindgen_malloc
///     local.set $buf
///     local.get $buf
///     global.get $__stack_pointer
///     local.get $len
///     memory.copy
///     local.get $base
///     global.set $__stack_pointer
///     ;; the fiber's segment is leaving the stack: the innermost fiber on
///     ;; the stack during the suspension is this fiber's `prev`, read from
///     ;; the node the export wrapper pushed at [base - 16]
///     (global.set $__jspi_stack_base (load [base - 16]))
///     block $done (result ...)
///         block $catch_all (result exnref)
///             ;; only for `catch` suspending imports:
///             block $rejected (result externref)
///                 try_table (result ...) (catch $__wbindgen_jstag $rejected)
///                                        (catch_all_ref $catch_all)
///                     local.get <params>...
///                     call $import        ;; the suspension point
///                 end
///                 ;; fulfilled: restore, record no rejection
///                 local.set <results>...
///                 local.get $base $len $buf
///                 call $__jspi_restore
///                 (i32.store rejected_addr (i32.const 0))
///                 local.get <results>...
///                 br $done
///             end
///             ;; rejected (`catch` only): the reason is the result
///             local.set <result>
///             local.get $base $len $buf
///             call $__jspi_restore
///             (i32.store rejected_addr (i32.const 1))
///             local.get <result>
///             br $done
///         end
///         ;; any other exception: restore, then keep unwinding
///         local.set $exn
///         local.get $base $len $buf
///         call $__jspi_restore
///         local.get $exn
///         throw_ref
///     end)
/// ```
///
/// `$base`, `$len` and `$buf` are locals, which JSPI preserves across the
/// suspension — no state survives in globals or JS.
fn wrap_suspending(
    module: &mut Module,
    import: FunctionId,
    ctx: JspiContext,
    restore: FunctionId,
    rejection: Option<Rejection>,
) -> FunctionId {
    let ty = module.types.get(module.funcs.get(import).ty());
    let params = ty.params().to_vec();
    let results = ty.results().to_vec();

    let results_ty: ir::InstrSeqType = match results.len() {
        0 => ir::InstrSeqType::Simple(None),
        1 => ir::InstrSeqType::Simple(Some(results[0])),
        _ => module.types.add(&[], &results).into(),
    };
    let exnref_ty: ir::InstrSeqType = ValType::Ref(RefType::EXNREF).into();
    let externref_ty: ir::InstrSeqType = ValType::Ref(RefType::EXTERNREF).into();

    let mut builder = FunctionBuilder::new(&mut module.types, &params, &results);
    let param_locals: Vec<_> = params.iter().map(|ty| module.locals.add(*ty)).collect();
    let result_locals: Vec<_> = results.iter().map(|ty| module.locals.add(*ty)).collect();
    let base = module.locals.add(ctx.ptr_ty);
    let len = module.locals.add(ctx.ptr_ty);
    let buf = module.locals.add(ctx.ptr_ty);
    let exn = module.locals.add(ValType::Ref(RefType::EXNREF));

    let eqz = match ctx.ptr_ty {
        ValType::I64 => UnaryOp::I64Eqz,
        _ => UnaryOp::I32Eqz,
    };
    let sub = ptr_sub(ctx.ptr_ty);

    let call_restore = |seq: &mut InstrSeqBuilder| {
        seq.local_get(base).local_get(len).local_get(buf);
        seq.call(restore);
    };
    let stash_results = |seq: &mut InstrSeqBuilder| {
        for local in result_locals.iter().rev() {
            seq.local_set(*local);
        }
    };
    let unstash_results = |seq: &mut InstrSeqBuilder| {
        for local in &result_locals {
            seq.local_get(*local);
        }
    };
    let store_rejected = |seq: &mut InstrSeqBuilder, rejection: Rejection, value: i32| {
        match ctx.ptr_ty {
            ValType::I64 => seq.i64_const(rejection.addr as i64),
            _ => seq.i32_const(rejection.addr as i32),
        };
        seq.i32_const(value);
        seq.store(
            ctx.memory,
            ir::StoreKind::I32 { atomic: false },
            MemArg {
                align: 4,
                offset: 0,
            },
        );
    };

    // Pre-allocate the block sequences so labels can reference each other.
    let try_seq = builder.dangling_instr_seq(results_ty).id();
    let rejected_seq = rejection.map(|_| builder.dangling_instr_seq(externref_ty).id());
    let catch_all_seq = builder.dangling_instr_seq(exnref_ty).id();
    let done_seq = builder.dangling_instr_seq(results_ty).id();

    // The suspension point.
    {
        let mut seq = builder.instr_seq(try_seq);
        for local in &param_locals {
            seq.local_get(*local);
        }
        seq.call(import);
    }

    // Innermost block: the try_table and the fulfilled path.
    let innermost = rejected_seq.unwrap_or(catch_all_seq);
    {
        let mut catches = Vec::new();
        if let (Some(rejection), Some(rejected_seq)) = (rejection, rejected_seq) {
            catches.push(ir::TryTableCatch::Catch {
                tag: rejection.js_tag,
                label: rejected_seq,
            });
        }
        catches.push(ir::TryTableCatch::CatchAllRef {
            label: catch_all_seq,
        });
        let mut seq = builder.instr_seq(innermost);
        seq.instr(ir::TryTable {
            seq: try_seq,
            catches,
        });
        // Fulfilled: results flow out of the try_table.
        stash_results(&mut seq);
        call_restore(&mut seq);
        if let Some(rejection) = rejection {
            store_rejected(&mut seq, rejection, 0);
        }
        unstash_results(&mut seq);
        seq.br(done_seq);
    }

    // Rejection path (`catch` only): the caught reason is the result.
    if let (Some(rejection), Some(rejected_seq)) = (rejection, rejected_seq) {
        let mut seq = builder.instr_seq(catch_all_seq);
        seq.instr(ir::Block { seq: rejected_seq });
        stash_results(&mut seq);
        call_restore(&mut seq);
        store_rejected(&mut seq, rejection, 1);
        unstash_results(&mut seq);
        seq.br(done_seq);
    }

    // Catch-all path: restore the shadow stack, then keep unwinding.
    {
        let mut seq = builder.instr_seq(done_seq);
        seq.instr(ir::Block { seq: catch_all_seq });
        seq.local_set(exn);
        call_restore(&mut seq);
        seq.local_get(exn);
        seq.instr(ir::ThrowRef {});
    }

    let mut body = builder.func_body();

    // Not inside a fiber: plain call.
    body.global_get(ctx.base).local_tee(base).unop(eqz).if_else(
        None,
        |then| {
            for local in &param_locals {
                then.local_get(*local);
            }
            then.call(import);
            then.instr(ir::Return {});
        },
        |_| {},
    );

    // Evacuate the live shadow-stack region.
    body.local_get(base).global_get(ctx.sp).binop(sub);
    body.local_set(len);
    body.local_get(len);
    push_align(&mut body, ctx.ptr_ty);
    body.call(ctx.malloc).local_set(buf);
    body.local_get(buf).global_get(ctx.sp).local_get(len);
    body.instr(ir::MemoryCopy {
        src: ctx.memory,
        dst: ctx.memory,
    });
    body.local_get(base).global_set(ctx.sp);
    // The fiber's segment is leaving the stack: the innermost fiber on the
    // stack during the suspension is this fiber's `prev`, read from the node
    // the export wrapper pushed at [base - 16] (part of the evacuated
    // region, so it is restored with the fiber on resume).
    body.local_get(base);
    push_align(&mut body, ctx.ptr_ty);
    body.binop(sub);
    load_ptr(&mut body, ctx);
    body.global_set(ctx.base);

    body.instr(ir::Block { seq: done_seq });

    let wrapper = builder.finish(param_locals, &mut module.funcs);
    let name = module.funcs.get(import).name.clone();
    module.funcs.get_mut(wrapper).name = Some(match name {
        Some(n) => format!("{n} suspending wrapper"),
        None => "suspending wrapper".to_string(),
    });
    wrapper
}

/// Rewrite all calls to suspending imports to go through the wrappers.
fn rewrite_calls(module: &mut Module, wrappers: &HashMap<FunctionId, FunctionId>) {
    let wrapper_ids: std::collections::HashSet<_> = wrappers.values().copied().collect();
    for (func_id, func) in module.funcs.iter_local_mut() {
        if wrapper_ids.contains(&func_id) {
            continue;
        }
        let entry = func.entry_block();
        ir::dfs_pre_order_mut(&mut CallRewriter { wrappers }, func, entry);
    }
}

struct CallRewriter<'a> {
    wrappers: &'a HashMap<FunctionId, FunctionId>,
}

impl ir::VisitorMut for CallRewriter<'_> {
    fn start_instr_seq_mut(&mut self, seq: &mut ir::InstrSeq) {
        for (instr, _) in seq.instrs.iter_mut() {
            let func = match instr {
                ir::Instr::Call(ir::Call { func }) => func,
                ir::Instr::ReturnCall(ir::ReturnCall { func }) => func,
                _ => continue,
            };
            if let Some(wrapper) = self.wrappers.get(func) {
                *func = *wrapper;
            }
        }
    }
}
