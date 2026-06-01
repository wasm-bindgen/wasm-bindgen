//! Management of wasm-bindgen descriptor functions.
//!
//! Each `#[wasm_bindgen]` shim has an accompanying descriptor that
//! encodes its type signature for the cli. The cli reads those
//! descriptors here and converts them into [`Descriptor`] values,
//! which the rest of the pipeline consumes when synthesising JS shims.
//!
//! # Transport
//!
//! The **primary** transport is the `__wasm_bindgen_descriptors`
//! custom section ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]).
//! The `#[wasm_bindgen]` macro builds each descriptor's bytes purely
//! from compile-time information and writes them into this section as
//! `#[link_section]` statics. Entries are decoded by
//! [`crate::descriptors_section`] and turn directly into [`Descriptor`]s.
//!
//! [`assert_no_legacy_describe_exports`] enforces that the macro has
//! actually populated the section for every shim it emits: any
//! leftover `__wbindgen_describe_<name>` export (the historical
//! "describe-by-execution" mechanism) is a hard error, not a silent
//! fallback to the interpreter.
//!
//! # The one remaining interpreter use
//!
//! Closure-cast descriptors —
//! `wbg_cast::<OwnedClosure<T, UW>, JsValue>` and similar — still go
//! through [`crate::interpreter`] from [`Self::execute_casts`]. The
//! cast's descriptor payload includes the closure's full schema
//! (`nargs`, per-arg schemas, ret schema) — a variable-length tail
//! that depends on `T` post-monomorphisation. Building that as a
//! `#[link_section]` static needs generic-length const arrays
//! (`[u32; <T as Trait>::N]`), which require
//! [`generic_const_exprs`] (nightly-only, no stable timeline).
//!
//! When `generic_const_exprs` stabilises, closure-cast descriptors
//! become ordinary `DESCRIPTOR_KIND_CAST` section entries and the
//! [`crate::interpreter`] directory deletes entirely. The migration
//! is one localised change to `breaks_if_inlined` in `src/rt/mod.rs`
//! plus the matching `Descriptor::decode` invocation here.
//!
//! [`generic_const_exprs`]: https://github.com/rust-lang/rust/issues/76560

use crate::descriptor::Descriptor;
use crate::descriptors_section;

use anyhow::{Context, Error};
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, ExportItem, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::{
    DESCRIPTORS_SECTION_NAME, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR, DESCRIPTOR_KIND_STATIC,
};

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub cast_imports: HashMap<Descriptor, Vec<FunctionId>>,
    /// Per-monomorphisation generic imports recovered from
    /// `__wbindgen_describe_generic_import` marker calls, keyed by the
    /// import's shim name. Each entry pairs the courier function with the
    /// concrete (spliced) descriptor for its monomorphisation. The shim
    /// is the key into the normal AST custom section, from which the cli
    /// recovers the import's metadata (js_name, module, namespace, …).
    pub generic_imports: HashMap<String, Vec<(FunctionId, Descriptor)>>,
}

pub type WasmBindgenDescriptorsSectionId = TypedCustomSectionId<WasmBindgenDescriptorsSection>;

/// Execute all `__wbindgen_describe_*` functions in a module, inserting a
/// custom section which represents the executed value of each descriptor.
///
/// Afterwards this will delete all descriptor functions from the module.
pub fn execute(module: &mut Module) -> Result<WasmBindgenDescriptorsSectionId, Error> {
    let mut section = WasmBindgenDescriptorsSection::default();

    // Phase 1: ingest the `__wasm_bindgen_descriptors` custom section.
    // This is the primary descriptor transport; every shim except
    // closure casts has its descriptor here.
    section
        .ingest_section(module)
        .context("failed to read __wasm_bindgen_descriptors section")?;

    // Phase 2: cross-check that the macro did not emit any legacy
    // `__wbindgen_describe_<name>` exports. The cli no longer reads
    // them; leftovers indicate a stale macro paired with this cli.
    assert_no_legacy_describe_exports(module)?;

    // Phase 3: recover closure-cast descriptors. Each `wbg_cast_closure`
    // monomorphisation emits a `breaks_if_inlined` function whose
    // body calls `__wbindgen_describe_cast` with five `i32.const`
    // immediates (schema pointers + invoke address); the scanner
    // here reads those immediates and composes the descriptor. No
    // wasm interpretation involved.
    section.execute_casts(module)?;

    // Phase 3b: recover per-monomorphisation generic imports. Each
    // `wbg_generic_import_*` monomorphisation emits a courier whose
    // body calls `__wbindgen_describe_generic_import` with four
    // `i32.const` immediates (JS import name ptr/len + arg schema
    // ptr/len); the scanner reads those and composes a function
    // descriptor bound to the named JS import.
    section.execute_generic_imports(module)?;

    // Phase 4: strip __wbg_invoke_* exports.
    //
    // The macro emits these to give per-closure-monomorphisation
    // forwarding wrappers a stable name we can look up. Now that
    // every SYMBOL_REF has been resolved to a function-table slot
    // we no longer need the export. Deleting it lets the walrus GC
    // pass run later (see `gc_module_and_adapters`) drop wrappers
    // whose only remaining liveness root was this export; wrappers
    // that JS will call retain liveness through the function table.
    strip_closure_invoke_exports(module);

    Ok(module.customs.add(section))
}

/// Remove every `__wbg_invoke_*` export from the module. Their job
/// (giving the macro-emitted closure wrappers a stable name for
/// `function_table_slot_of` to find) is done by the time this runs.
/// The actual removal of unreferenced wrapper *functions* happens
/// later via the existing walrus GC pass.
fn strip_closure_invoke_exports(module: &mut Module) {
    const PREFIX: &str = "__wbg_invoke_";
    let to_remove: Vec<_> = module
        .exports
        .iter()
        .filter(|e| e.name.starts_with(PREFIX))
        .map(|e| e.id())
        .collect();
    for id in to_remove {
        module.exports.delete(id);
    }
}

/// Assert that every shim's descriptor came through the
/// `__wasm_bindgen_descriptors` section: the macro must not be
/// emitting any legacy `__wbindgen_describe_<name>` exports. The
/// interpreter is no longer wired to read them, so leftovers indicate
/// a macro-version mismatch (a binary produced by an older macro
/// paired with this newer cli).
///
/// Returns an error rather than silently invoking the interpreter so
/// any regression in macro coverage surfaces immediately.
fn assert_no_legacy_describe_exports(module: &Module) -> Result<(), Error> {
    use anyhow::bail;
    const PREFIX: &str = "__wbindgen_describe_";
    // Allow `__wbindgen_describe` itself (the inform-stream marker
    // import; not an export). We're only looking at exports with
    // a `_<name>` suffix.
    let leftovers: Vec<&str> = module
        .exports
        .iter()
        .filter_map(|e| {
            let name = e.name.as_str();
            if !name.starts_with(PREFIX) {
                return None;
            }
            // Exact name `__wbindgen_describe_cast` happens only if a
            // module re-exports the marker import for some reason —
            // not produced by the macro, but guard anyway.
            if name == "__wbindgen_describe_cast" {
                return None;
            }
            Some(name)
        })
        .collect();
    if !leftovers.is_empty() {
        bail!(
            "wasm-bindgen-cli no longer reads legacy `__wbindgen_describe_<name>` \
             exports; every shim's descriptor must come from the \
             `__wasm_bindgen_descriptors` custom section. The following exports \
             were emitted by an older `#[wasm_bindgen]` macro and would have \
             been read by the legacy wasm interpreter: {leftovers:?}"
        );
    }
    Ok(())
}

impl WasmBindgenDescriptorsSection {
    /// Pull the `__wasm_bindgen_descriptors` custom section out of
    /// `module` (if present), parse it, and populate `self` with the
    /// REGULAR and STATIC entries it contains.
    ///
    /// CAST entries are ignored here: their `breaks_if_inlined<From,
    /// To>` symbol needs to be back-resolved into a `FunctionId`,
    /// which the closure-cast interpreter pathway in
    /// [`Self::execute_casts`] does by walking calls to
    /// `__wbindgen_describe_cast`. Section-side cast entries remain
    /// for a future commit that lands the variable-length per-
    /// monomorphisation tail (blocked on `generic_const_exprs`; see
    /// module-level docs).
    fn ingest_section(&mut self, module: &mut Module) -> Result<(), Error> {
        let raw = match module.customs.remove_raw(DESCRIPTORS_SECTION_NAME) {
            Some(raw) => raw,
            None => return Ok(()),
        };

        let (entries, stats) = descriptors_section::parse(&raw.data)?;
        if stats.skipped_total() > 0 {
            for (version, count) in &stats.skipped_unknown_version {
                log::info!(
                    "wasm-bindgen-cli does not recognise format_version {version} \
                     for {count} __wasm_bindgen_descriptors entries; these are \
                     ignored. This usually means the binary was produced by a \
                     newer wasm-bindgen than this CLI."
                );
            }
        }
        let resolved_symbols = build_symbol_table(module)?;

        for entry in entries {
            let stream =
                descriptors_section::resolve_symbols(&entry.schema_bytes, &resolved_symbols)
                    .with_context(|| {
                        format!(
                            "failed to resolve symbol references in descriptor for {:?}",
                            entry.name
                        )
                    })?;
            let descriptor = Descriptor::decode(&stream);
            match entry.kind {
                DESCRIPTOR_KIND_REGULAR | DESCRIPTOR_KIND_STATIC => {
                    // STATIC entries decode the same way (`Descriptor::decode`
                    // accepts either a FUNCTION-wrapped or a bare type
                    // schema); the difference is purely how the macro
                    // emits it. ImportStatic consumes the resulting
                    // `Descriptor` as the static's type directly.
                    self.descriptors.insert(entry.name, descriptor);
                }
                DESCRIPTOR_KIND_CAST => {
                    log::debug!(
                        "ignoring cast descriptor for {:?} in section \
                         (still handled by interpreter; see module docs)",
                        entry.name
                    );
                }
                _ => unreachable!("parser already validated kind byte"),
            }
        }
        Ok(())
    }

    fn execute_casts(&mut self, module: &mut Module) -> Result<(), Error> {
        // Locate the `__wbindgen_describe_cast` import. If it isn't
        // present nothing in this module performs a `wbg_cast`.
        let describe_cast_id = module.imports.iter().find_map(|import| {
            if import.module == "__wbindgen_placeholder__"
                && import.name == "__wbindgen_describe_cast"
            {
                if let walrus::ImportKind::Function(id) = import.kind {
                    return Some(id);
                }
            }
            None
        });
        let describe_cast_id = match describe_cast_id {
            Some(id) => id,
            None => return Ok(()),
        };

        // Snapshot the data segments so we can read schema bytes from
        // the static `SCHEMA_BUF` storage by absolute address.
        let data_view = DataSegmentView::new(module);

        // For each function containing a call to
        // `__wbindgen_describe_cast`, scan its body for the five
        // immediates feeding that call:
        //   from_ptr, from_len, to_ptr, to_len, invoke_addr
        // All five are `i32.const` values in optimised builds, possibly
        // round-tripped through `local.set`/`local.get` in debug.
        let mut local_funcs = Vec::new();
        for (func_id, _local) in module.funcs.iter_local() {
            local_funcs.push(func_id);
        }
        for func_id in local_funcs {
            let local = match &module.funcs.get(func_id).kind {
                walrus::FunctionKind::Local(l) => l,
                _ => continue,
            };
            // Walk the entry block linearly tracking i32.const ->
            // local correspondences. When we hit the marker call,
            // resolve the most recent five values on the operand
            // stack.
            let entry = local.entry_block();
            let mut scanner = CastCallScanner::new(describe_cast_id, 5);
            scanner.walk(local, entry);
            for args in scanner.found_calls {
                let from_ptr = args[0] as u32;
                let from_len = args[1] as u32;
                let to_ptr = args[2] as u32;
                let to_len = args[3] as u32;
                let invoke_addr = args[4] as u32;
                let from_schema = data_view.read_u32_slice(from_ptr, from_len)?;
                let to_schema = data_view.read_u32_slice(to_ptr, to_len)?;
                let descriptor = compose_cast_descriptor(&from_schema, &to_schema, invoke_addr);
                let descriptor = Descriptor::decode(&descriptor);
                self.cast_imports
                    .entry(descriptor)
                    .or_default()
                    .push(func_id);
            }
        }

        Ok(())
    }

    fn execute_generic_imports(&mut self, module: &mut Module) -> Result<(), Error> {
        // Locate the `__wbindgen_describe_generic_import` marker import.
        let describe_id = module.imports.iter().find_map(|import| {
            if import.module == "__wbindgen_placeholder__"
                && import.name == "__wbindgen_describe_generic_import"
            {
                if let walrus::ImportKind::Function(id) = import.kind {
                    return Some(id);
                }
            }
            None
        });
        let describe_id = match describe_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let data_view = DataSegmentView::new(module);

        let mut local_funcs = Vec::new();
        for (func_id, _local) in module.funcs.iter_local() {
            local_funcs.push(func_id);
        }
        for func_id in local_funcs {
            let local = match &module.funcs.get(func_id).kind {
                walrus::FunctionKind::Local(l) => l,
                _ => continue,
            };
            let entry = local.entry_block();
            // Seven immediates: shim_ptr, shim_len, template_ptr,
            // template_len, fills_ptr, fills_len (the fills blob is a
            // concatenation of the per-hole schemas), and closure_invoke_addr
            // (the single closure hole's invoke-shim table index, or 0).
            let mut scanner = CastCallScanner::new(describe_id, 7);
            scanner.walk(local, entry);
            for args in scanner.found_calls {
                let shim_ptr = args[0] as u32;
                let shim_len = args[1] as u32;
                let template_ptr = args[2] as u32;
                let template_len = args[3] as u32;
                let fills_ptr = args[4] as u32;
                let fills_len = args[5] as u32;
                let closure_invoke_addr = args[6] as u32;
                let shim_bytes = data_view.read_bytes(shim_ptr, shim_len as usize)?;
                let shim = String::from_utf8(shim_bytes)
                    .context("generic import shim name was not valid UTF-8")?;
                let template = data_view.read_u32_slice(template_ptr, template_len)?;
                let fills = data_view.read_u32_slice(fills_ptr, fills_len)?;
                let fills = Descriptor::decode_sequence(&fills);
                let mut descriptor = Descriptor::decode(&template).substitute(&fills);
                if closure_invoke_addr != 0 {
                    descriptor.patch_closure_shim(closure_invoke_addr);
                }
                self.generic_imports
                    .entry(shim)
                    .or_default()
                    .push((func_id, descriptor));
            }
        }

        Ok(())
    }
}

/// Walk a function body looking for `memory.init data_id` patterns
/// with a constant destination address. For `+bulk-memory` builds
/// (atomics, wasm-shared, etc.) wasm-ld emits passive data segments
/// and a `__wasm_init_memory` ctor function that copies each one into
/// linear memory at startup; this scanner extracts those destination
/// addresses so closure-cast `SCHEMA_BUF` pointers can still be
/// resolved.
fn scan_memory_init(
    func: &walrus::LocalFunction,
    passive: &HashMap<walrus::DataId, Vec<u8>>,
    out: &mut Vec<(u32, Vec<u8>)>,
) {
    let mut stack: Vec<Option<i32>> = Vec::new();
    scan_memory_init_seq(func, func.entry_block(), passive, out, &mut stack);
}

fn scan_memory_init_seq(
    func: &walrus::LocalFunction,
    seq: walrus::ir::InstrSeqId,
    passive: &HashMap<walrus::DataId, Vec<u8>>,
    out: &mut Vec<(u32, Vec<u8>)>,
    stack: &mut Vec<Option<i32>>,
) {
    use walrus::ir::*;
    for (instr, _) in func.block(seq).iter() {
        match instr {
            Instr::Const(c) => stack.push(match c.value {
                Value::I32(n) => Some(n),
                Value::I64(n) => Some(n as i32),
                _ => None,
            }),
            Instr::MemoryInit(m) => {
                // Stack: [dest, src, len], top is len.
                let _len = stack.pop().unwrap_or(None);
                let _src = stack.pop().unwrap_or(None);
                let dest = stack.pop().unwrap_or(None);
                if let (Some(dest), Some(bytes)) = (dest, passive.get(&m.data)) {
                    out.push((dest as u32, bytes.clone()));
                }
            }
            // Recurse into nested blocks: `__wasm_init_memory` wraps
            // its `memory.init` calls in a few levels of `block` for
            // the once-per-thread cmpxchg guard.
            Instr::Block(b) => scan_memory_init_seq(func, b.seq, passive, out, stack),
            Instr::Loop(l) => scan_memory_init_seq(func, l.seq, passive, out, stack),
            Instr::IfElse(ifelse) => {
                let saved = stack.clone();
                scan_memory_init_seq(func, ifelse.consequent, passive, out, stack);
                *stack = saved.clone();
                scan_memory_init_seq(func, ifelse.alternative, passive, out, stack);
                *stack = saved;
            }
            Instr::Drop(_) => {
                stack.pop();
            }
            Instr::LocalGet(_) | Instr::GlobalGet(_) => stack.push(None),
            Instr::LocalSet(_) | Instr::GlobalSet(_) => {
                stack.pop();
            }
            Instr::LocalTee(_) => { /* stack unchanged */ }
            Instr::Br(_) | Instr::BrIf(_) | Instr::BrTable(_) | Instr::Return(_) => {
                stack.clear();
            }
            _ => {
                stack.clear();
            }
        }
    }
}

/// Snapshot of the module's active data segments. Lets us resolve a
/// linear-memory address to the bytes that wasm-ld wrote into the
/// data section at link time. Used by the closure-cast scanner to
/// read each cast's static `SCHEMA_BUF` content via the pointer the
/// runtime passed to `__wbindgen_describe_cast`.
struct DataSegmentView {
    segments: Vec<(u32, Vec<u8>)>, // (start address, bytes)
}

impl DataSegmentView {
    fn new(module: &Module) -> Self {
        use walrus::DataKind;

        let mut segments: Vec<(u32, Vec<u8>)> = Vec::new();
        let mut passive_bytes: HashMap<walrus::DataId, Vec<u8>> = HashMap::new();

        for segment in module.data.iter() {
            match &segment.kind {
                DataKind::Active { offset, .. } => {
                    let offset_val =
                        match crate::wasm_conventions::evaluate_const_expr(offset, module) {
                            Some(walrus::ir::Value::I32(n)) => n as u32,
                            Some(walrus::ir::Value::I64(n)) => n as u32,
                            _ => continue,
                        };
                    segments.push((offset_val, segment.value.clone()));
                }
                DataKind::Passive => {
                    passive_bytes.insert(segment.id(), segment.value.clone());
                }
            }
        }

        // For Passive segments (produced by `+bulk-memory` builds,
        // including atomics), look for `memory.init` instructions in
        // `__wasm_init_memory` to learn where wasm-ld will copy each
        // segment at module-init time. Record those as effectively
        // active so the scanner can resolve schema pointers.
        if !passive_bytes.is_empty() {
            for (_func_id, local) in module.funcs.iter_local() {
                scan_memory_init(local, &passive_bytes, &mut segments);
            }
        }

        DataSegmentView { segments }
    }

    /// Read `len` `u32` words (`4 * len` bytes) starting at linear-
    /// memory address `addr`. Returns an error if the range straddles
    /// segment boundaries or is unmapped.
    fn read_u32_slice(&self, addr: u32, len: u32) -> Result<Vec<u32>, Error> {
        let byte_count = (len as usize)
            .checked_mul(4)
            .ok_or_else(|| anyhow::anyhow!("schema length overflows"))?;
        let bytes = self.read_bytes(addr, byte_count)?;
        let mut out = Vec::with_capacity(len as usize);
        for chunk in bytes.chunks_exact(4) {
            out.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
        Ok(out)
    }

    fn read_bytes(&self, addr: u32, count: usize) -> Result<Vec<u8>, Error> {
        for (start, bytes) in &self.segments {
            let end = start
                .checked_add(bytes.len() as u32)
                .ok_or_else(|| anyhow::anyhow!("data segment address overflow"))?;
            if addr >= *start && addr.saturating_add(count as u32) <= end {
                let offset = (addr - start) as usize;
                return Ok(bytes[offset..offset + count].to_vec());
            }
        }
        anyhow::bail!(
            "schema pointer {addr:#x}..{:#x} is not inside any active data segment",
            addr as u64 + count as u64
        );
    }
}

/// Compose a complete cast descriptor stream from the From/To schema
/// halves and the closure-invoke address. The cli's
/// `Descriptor::decode` consumes:
///
///   [FUNCTION, shim_idx=0, nargs=1, <From schema>, <To schema>, <To schema>]
///
/// For closure casts, the From schema is the OwnedClosure /
/// BorrowedClosure layout, which contains a `0` placeholder at the
/// shim_idx slot (inside the inner FUNCTION descriptor). We overwrite
/// that slot with the real `invoke_addr` here. Non-closure casts
/// (`invoke_addr == 0`) need no patching.
fn compose_cast_descriptor(from: &[u32], to: &[u32], invoke_addr: u32) -> Vec<u32> {
    let mut from = from.to_vec();
    if invoke_addr != 0 {
        // Find FUNCTION inside the From schema and patch the next
        // word (shim_idx placeholder) with the invoke address.
        // Closure schema layout:
        //   [CLOSURE, owned, IS_MUT, FUNCTION, 0, nargs, ...args, ret, ret]
        // We look for the first FUNCTION opcode (the inner function
        // descriptor) and confirm the next word is the placeholder 0
        // before patching.
        let function_op = wasm_bindgen_shared::tys::FUNCTION;
        if let Some(idx) = from.iter().position(|w| *w == function_op) {
            if idx + 1 < from.len() && from[idx + 1] == 0 {
                from[idx + 1] = invoke_addr;
            }
        }
    }
    let mut out = Vec::with_capacity(3 + from.len() + 2 * to.len());
    out.push(wasm_bindgen_shared::tys::FUNCTION);
    out.push(0); // shim_idx for the cast itself, unused
    out.push(1); // nargs
    out.extend_from_slice(&from);
    out.extend_from_slice(to);
    out.extend_from_slice(to);
    out
}

/// Narrow scanner that finds calls to `__wbindgen_describe_cast` and
/// recovers the 5 `i32.const` immediates fed into each. Handles the
/// trivial optimised shape (5 consecutive `i32.const`s) plus the
/// debug shape where rustc shuttles values through locals.
///
/// Not a wasm interpreter: we track only `i32.const` values written
/// into locals (`local.set` / `local.tee`) and an ordered list of
/// "pending immediates" the next `call` will consume. Branches,
/// loops, memory loads, globals, arithmetic are all unsupported and
/// simply invalidate the pending list — which is fine because they
/// don't appear in a `breaks_if_inlined` body.
struct CastCallScanner {
    target: walrus::FunctionId,
    /// Number of `i32.const` immediates the marker call consumes.
    arity: usize,
    /// Pending list: i32.const values pushed onto the operand stack
    /// since the most recent stack reset. The top of the stack is at
    /// the end. `None` means "value at this stack slot is not a known
    /// constant" (we still track depth so we can find the right slots
    /// at the call).
    operand_stack: Vec<Option<i32>>,
    /// Locals -> last i32.const written via `local.set`/`local.tee`.
    locals: std::collections::BTreeMap<walrus::LocalId, i32>,
    /// All marker calls we've found in this function (each with its
    /// `arity` resolved immediates).
    found_calls: Vec<Vec<i32>>,
}

impl CastCallScanner {
    fn new(target: walrus::FunctionId, arity: usize) -> Self {
        Self {
            target,
            arity,
            operand_stack: Vec::new(),
            locals: std::collections::BTreeMap::new(),
            found_calls: Vec::new(),
        }
    }

    fn walk(&mut self, func: &walrus::LocalFunction, seq: walrus::ir::InstrSeqId) {
        use walrus::ir::*;
        for (instr, _) in func.block(seq).iter() {
            match instr {
                Instr::Const(c) => {
                    let v = match c.value {
                        Value::I32(n) => Some(n),
                        Value::I64(n) => Some(n as i32),
                        _ => None,
                    };
                    self.operand_stack.push(v);
                }
                Instr::LocalGet(e) => {
                    self.operand_stack.push(self.locals.get(&e.local).copied());
                }
                Instr::LocalSet(e) => {
                    let v = self.operand_stack.pop().unwrap_or(None);
                    if let Some(n) = v {
                        self.locals.insert(e.local, n);
                    } else {
                        self.locals.remove(&e.local);
                    }
                }
                Instr::LocalTee(e) => {
                    let v = self.operand_stack.last().copied().unwrap_or(None);
                    if let Some(n) = v {
                        self.locals.insert(e.local, n);
                    } else {
                        self.locals.remove(&e.local);
                    }
                }
                Instr::Drop(_) => {
                    self.operand_stack.pop();
                }
                // Store: consumes two operands (addr + value). We
                // don't track its effect on linear memory; the
                // body's stack-pointer dance writes intermediate
                // values to the stack frame without affecting the
                // immediates that ultimately flow into the cast
                // marker call.
                Instr::Store(_) => {
                    self.operand_stack.pop(); // value
                    self.operand_stack.pop(); // addr
                }
                // Load: consumes one (addr) and pushes one
                // (untracked).
                Instr::Load(_) => {
                    self.operand_stack.pop();
                    self.operand_stack.push(None);
                }
                // Global get/set: track stack effect but not value.
                Instr::GlobalGet(_) => {
                    self.operand_stack.push(None);
                }
                Instr::GlobalSet(_) => {
                    self.operand_stack.pop();
                }
                // Arithmetic operations on the stack pointer / local
                // ABI plumbing. Treat as opaque: consume operands,
                // produce untracked value.
                Instr::Binop(_) => {
                    self.operand_stack.pop();
                    self.operand_stack.pop();
                    self.operand_stack.push(None);
                }
                Instr::Unop(_) => {
                    self.operand_stack.pop();
                    self.operand_stack.push(None);
                }
                Instr::Call(Call { func: callee })
                | Instr::ReturnCall(ReturnCall { func: callee }) => {
                    if *callee == self.target {
                        // The call consumes the top `arity` stack values.
                        let n = self.operand_stack.len();
                        if n >= self.arity {
                            let args = self.operand_stack[n - self.arity..n]
                                .iter()
                                .map(|slot| slot.unwrap_or(0))
                                .collect();
                            self.found_calls.push(args);
                        }
                        for _ in 0..self.arity {
                            self.operand_stack.pop();
                        }
                    } else {
                        // Unknown call: consume the right number of
                        // args and push the right number of results
                        // so the stack accounting stays right. We
                        // don't know exact arity at this level; the
                        // safest move is to skip past it by resetting
                        // when we can't reason about it. Cast bodies
                        // don't have intervening calls between the
                        // const setup and the marker.
                        self.operand_stack.clear();
                        self.locals.clear();
                    }
                }
                _ => {
                    // Branches, loops, etc. — none of these appear
                    // in `breaks_if_inlined` bodies. Reset to be
                    // safe.
                    self.operand_stack.clear();
                    self.locals.clear();
                }
            }
        }
    }
}

impl CustomSection for WasmBindgenDescriptorsSection {
    fn name(&self) -> &str {
        "wasm-bindgen descriptors"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<'_, [u8]> {
        panic!("shouldn't emit custom sections just yet");
    }
}

/// Build a `name -> u32` lookup for [`descriptors_section::resolve_symbols`].
///
/// For each exported function present in the main function table, the
/// map records the function-table slot index — the value the legacy
/// interpreter would have observed via `invoke as *const () as u32`.
/// This is what makes the closure SYMBOL_REF path work: the macro
/// emits a non-generic wrapper around `invoke` whose `#[export_name]`
/// is a stable content hash of the closure signature, and the cli
/// surfaces its slot here.
///
/// The macro-emitted `#[used] static FOO: fn-ptr = wrapper;` keepalive
/// causes wasm-ld to place the wrapper in an element segment on
/// `wasm32`, but on `wasm64` that keepalive does not trigger the
/// address-taken treatment (rustc/wasm-ld limitation). For exported
/// functions named `__wbg_invoke_*` that are missing from any element
/// segment, this function appends them to the main function table by
/// adding a fresh active element segment at the current table tail
/// (growing the table by one) so the SYMBOL_REF resolver can address
/// them. This keeps the runtime-side macro simple and works uniformly
/// across wasm32 and wasm64.
fn build_symbol_table(module: &mut Module) -> Result<HashMap<String, u32>, Error> {
    use walrus::{ConstExpr, ElementItems, ElementKind};

    let mut out = HashMap::new();

    // Snapshot exports first; we may mutate the module below.
    let exports: Vec<(String, walrus::FunctionId)> = module
        .exports
        .iter()
        .filter_map(|e| match e.item {
            ExportItem::Function(id) => Some((e.name.clone(), id)),
            _ => None,
        })
        .collect();

    let main_table_id = module.tables.main_function_table().ok().flatten();

    for (name, func_id) in exports {
        if let Ok(slot) = crate::wasm_conventions::function_table_slot_of(module, func_id) {
            out.insert(name, slot);
            continue;
        }
        // Fallback A: some test-runner / transformation layers wrap an
        // exported function (`__wbg_invoke_X`) with a thin shim
        // (`__wbg_invoke_X.command_export`). The export now points at
        // the shim, but the original function still sits in the
        // function table under its original symbol name.
        if let Some(slot) = lookup_table_slot_by_name(module, &name) {
            out.insert(name, slot);
            continue;
        }
        // Fallback B: the macro-emitted keepalive didn't make wasm-ld
        // place the wrapper in the function table (notably on wasm64).
        // Append it to the table ourselves. Limit this to closure
        // invoke wrappers to avoid touching unrelated exports.
        if !name.starts_with("__wbg_invoke_") {
            continue;
        }
        let table_id = match main_table_id {
            Some(id) => id,
            None => continue,
        };
        let (slot, table64) = {
            let table = module.tables.get_mut(table_id);
            let slot = u32::try_from(table.initial)
                .map_err(|_| anyhow::anyhow!("function table initial size does not fit in u32"))?;
            // Grow the table by one to make room for the new slot.
            table.initial = table.initial.saturating_add(1);
            if let Some(max) = table.maximum.as_mut() {
                if *max < table.initial {
                    *max = table.initial;
                }
            }
            (slot, table.table64)
        };
        // Active element segment offset uses the table's index type:
        // i64 for table64 (wasm64), i32 otherwise.
        let offset_val = if table64 {
            walrus::ir::Value::I64(slot as i64)
        } else {
            walrus::ir::Value::I32(slot as i32)
        };
        let elem_id = module.elements.add(
            ElementKind::Active {
                table: table_id,
                offset: ConstExpr::Value(offset_val),
            },
            ElementItems::Functions(vec![func_id]),
        );
        module
            .tables
            .get_mut(table_id)
            .elem_segments
            .insert(elem_id);
        out.insert(name, slot);
    }
    Ok(out)
}

/// Walk the main function table's element segments and find any
/// function whose own `name` matches `wanted_name`. Returns the
/// absolute slot index in that case. Used as a fallback when an
/// export points to a wrapping shim function instead of the table-
/// registered original.
fn lookup_table_slot_by_name(module: &Module, wanted_name: &str) -> Option<u32> {
    use walrus::{ConstExpr, ElementItems, ElementKind};

    let table_id = module.tables.main_function_table().ok().flatten()?;
    let table = module.tables.get(table_id);
    for &segment_id in &table.elem_segments {
        let segment = module.elements.get(segment_id);
        let base = match &segment.kind {
            ElementKind::Active {
                offset: ConstExpr::Value(walrus::ir::Value::I32(n)),
                ..
            } => *n as u32,
            _ => continue,
        };
        let funcs: Vec<walrus::FunctionId> = match &segment.items {
            ElementItems::Functions(items) => items.clone(),
            ElementItems::Expressions(_, items) => items
                .iter()
                .filter_map(|e| match e {
                    ConstExpr::RefFunc(f) => Some(*f),
                    _ => None,
                })
                .collect(),
        };
        for (i, fid) in funcs.iter().enumerate() {
            if module.funcs.get(*fid).name.as_deref() == Some(wanted_name) {
                return Some(base + i as u32);
            }
        }
    }
    None
}
