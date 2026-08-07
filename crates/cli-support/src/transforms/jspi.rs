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
//!   and the buffer is freed.
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
use std::collections::HashMap;
use walrus::ir::{self, BinaryOp, UnaryOp, Value};
use walrus::{
    ConstExpr, ExportItem, FunctionBuilder, FunctionId, GlobalId, InstrSeqBuilder, MemoryId,
    Module, ValType,
};

pub fn run(
    module: &mut Module,
    aux: &mut WasmBindgenAux,
    wit: &NonstandardWitSection,
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

    // The wasm-level import shims of `#[wasm_bindgen(suspending)]` imports.
    let suspending_imports = wit
        .implements
        .iter()
        .filter(|(_, _, adapter)| aux.imports_with_suspending.contains(adapter))
        .map(|(_, func, _)| *func)
        .collect::<Vec<_>>();

    if jspi_exports.is_empty() && suspending_imports.is_empty() {
        return Ok(());
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

    for export_id in jspi_exports {
        let inner = match module.exports.get(export_id).item {
            ExportItem::Function(f) => f,
            _ => bail!("jspi export is not a function"),
        };
        let wrapper = wrap_export(module, inner, ctx);
        module.exports.get_mut(export_id).item = wrapper.into();
    }

    let mut wrappers = HashMap::new();
    for import in suspending_imports {
        if wrappers.contains_key(&import) {
            continue;
        }
        let wrapper = wrap_import(module, import, ctx);
        wrappers.insert(import, wrapper);
    }
    rewrite_calls(module, &wrappers);

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
///     (local $prev ptr) (local $prev_suspended i32)
///     global.get $__jspi_stack_base
///     local.set $prev
///     global.get $__stack_pointer
///     global.set $__jspi_stack_base
///     global.get $__jspi_suspended
///     local.set $prev_suspended
///     i32.const 0
///     global.set $__jspi_suspended
///     local.get <params>...
///     call $inner
///     ;; if this fiber suspended, its entry-time callers unwound while it
///     ;; was pending (resume only ever happens on an empty stack), so the
///     ;; correct exit SP is the empty-stack top, not the entry offset —
///     ;; leaving the entry offset would permanently leak the region above it
///     global.get $__jspi_suspended
///     if
///         global.get $__jspi_stack_top
///         global.set $__stack_pointer
///     end
///     local.get $prev_suspended
///     global.set $__jspi_suspended
///     local.get $prev
///     global.set $__jspi_stack_base)
/// ```
///
/// This runs inside the fiber that `WebAssembly.promising` creates, so the
/// base is published exactly when the fiber starts executing and restored
/// exactly when it completes, with no window for interleaved microtasks.
fn wrap_export(module: &mut Module, inner: FunctionId, ctx: JspiContext) -> FunctionId {
    let ty = module.types.get(module.funcs.get(inner).ty());
    let params = ty.params().to_vec();
    let results = ty.results().to_vec();

    let mut builder = FunctionBuilder::new(&mut module.types, &params, &results);
    let param_locals: Vec<_> = params.iter().map(|ty| module.locals.add(*ty)).collect();
    let prev = module.locals.add(ctx.ptr_ty);
    let prev_suspended = module.locals.add(ValType::I32);

    let mut body = builder.func_body();
    body.global_get(ctx.base).local_set(prev);
    body.global_get(ctx.sp).global_set(ctx.base);
    body.global_get(ctx.suspended).local_set(prev_suspended);
    body.i32_const(0).global_set(ctx.suspended);
    for local in &param_locals {
        body.local_get(*local);
    }
    body.call(inner);
    body.global_get(ctx.suspended).if_else(
        None,
        |then| {
            then.global_get(ctx.top).global_set(ctx.sp);
        },
        |_| {},
    );
    body.local_get(prev_suspended).global_set(ctx.suspended);
    body.local_get(prev).global_set(ctx.base);

    let wrapper = builder.finish(param_locals, &mut module.funcs);
    let name = module.funcs.get(inner).name.clone();
    module.funcs.get_mut(wrapper).name = name.map(|n| format!("{n} jspi wrapper"));
    wrapper
}

/// Wrap a `#[wasm_bindgen(suspending)]` import with the evacuate-on-suspend
/// sequence:
///
/// ```wat
/// (func $wrapper (param ...) (result ...)
///     (local $base ptr) (local $len ptr) (local $buf ptr)
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
///     ;; suspend
///     local.get <params>...
///     call $import
///     ;; resume: restore the region at its original address before any
///     ;; other wasm instruction can observe the shadow stack
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
/// `$base`, `$len` and `$buf` are locals, which JSPI preserves across the
/// suspension — no state survives in globals or JS.
fn wrap_import(module: &mut Module, import: FunctionId, ctx: JspiContext) -> FunctionId {
    let ty = module.types.get(module.funcs.get(import).ty());
    let params = ty.params().to_vec();
    let results = ty.results().to_vec();

    let mut builder = FunctionBuilder::new(&mut module.types, &params, &results);
    let param_locals: Vec<_> = params.iter().map(|ty| module.locals.add(*ty)).collect();
    let result_locals: Vec<_> = results.iter().map(|ty| module.locals.add(*ty)).collect();
    let base = module.locals.add(ctx.ptr_ty);
    let len = module.locals.add(ctx.ptr_ty);
    let buf = module.locals.add(ctx.ptr_ty);

    let (sub, eqz) = match ctx.ptr_ty {
        ValType::I64 => (BinaryOp::I64Sub, UnaryOp::I64Eqz),
        _ => (BinaryOp::I32Sub, UnaryOp::I32Eqz),
    };
    let align_const = |body: &mut InstrSeqBuilder| {
        match ctx.ptr_ty {
            ValType::I64 => body.i64_const(16),
            _ => body.i32_const(16),
        };
    };
    let memory_copy = ir::MemoryCopy {
        src: ctx.memory,
        dst: ctx.memory,
    };

    let mut body = builder.func_body();

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
    align_const(&mut body);
    body.call(ctx.malloc).local_set(buf);
    body.local_get(buf).global_get(ctx.sp).local_get(len);
    body.instr(memory_copy.clone());
    body.local_get(base).global_set(ctx.sp);

    // The suspension point.
    for local in &param_locals {
        body.local_get(*local);
    }
    body.call(import);
    for local in result_locals.iter().rev() {
        body.local_set(*local);
    }

    // Restore, first thing on resume.
    body.local_get(base).local_get(len).binop(sub);
    body.local_get(buf).local_get(len);
    body.instr(memory_copy);
    body.local_get(base).local_get(len).binop(sub);
    body.global_set(ctx.sp);
    body.local_get(base).global_set(ctx.base);
    // Mark the fiber as having suspended so its exit resets the SP to the
    // empty-stack top (see `wrap_export`).
    body.i32_const(1).global_set(ctx.suspended);
    body.local_get(buf).local_get(len);
    align_const(&mut body);
    body.call(ctx.free);

    for local in &result_locals {
        body.local_get(*local);
    }

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
