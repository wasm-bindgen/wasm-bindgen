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
        let resolved_symbols = build_symbol_table(module);

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
        use walrus::ir::*;

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
            let mut scanner = CastCallScanner::new(describe_cast_id);
            scanner.walk(local, entry);
            for args in scanner.found_calls {
                let from_ptr = args[0] as u32;
                let from_len = args[1] as u32;
                let to_ptr = args[2] as u32;
                let to_len = args[3] as u32;
                let invoke_addr = args[4] as u32;
                let from_schema = data_view.read_u32_slice(from_ptr, from_len)?;
                let to_schema = data_view.read_u32_slice(to_ptr, to_len)?;
                let descriptor =
                    compose_cast_descriptor(&from_schema, &to_schema, invoke_addr);
                let descriptor = Descriptor::decode(&descriptor);
                self.cast_imports
                    .entry(descriptor)
                    .or_default()
                    .push(func_id);
            }
        }

        Ok(())
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
        let mut segments = Vec::new();
        for segment in module.data.iter() {
            if let walrus::DataKind::Active { offset, .. } = &segment.kind {
                let offset_val = match offset {
                    walrus::ConstExpr::Value(walrus::ir::Value::I32(n)) => *n as u32,
                    walrus::ConstExpr::Value(walrus::ir::Value::I64(n)) => *n as u32,
                    _ => continue, // PIC / unknown offset, skip
                };
                segments.push((offset_val, segment.value.clone()));
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
            let end = start.checked_add(bytes.len() as u32).ok_or_else(|| {
                anyhow::anyhow!("data segment address overflow")
            })?;
            if addr >= *start && addr.checked_add(count as u32).unwrap_or(u32::MAX) <= end {
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
        // word (shim_idx placeholder) with the invoke address. We
        // look for the first FUNCTION opcode after the initial
        // CLOSURE / owned / IS_MUT triple, which is where every
        // closure cast's inner function descriptor starts.
        use crate::descriptor;
        let function_op = wasm_bindgen_shared::tys::FUNCTION;
        // Closure schema layout: [CLOSURE, owned, IS_MUT, FUNCTION, 0, nargs, ...]
        // Find FUNCTION and confirm the next word is the placeholder 0.
        let _ = descriptor::Descriptor::String; // suppress unused-import warnings if any
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
    /// Pending list: i32.const values pushed onto the operand stack
    /// since the most recent stack reset. The top of the stack is at
    /// the end. `None` means "value at this stack slot is not a known
    /// constant" (we still track depth so we can find the right slots
    /// at the call).
    operand_stack: Vec<Option<i32>>,
    /// Locals -> last i32.const written via `local.set`/`local.tee`.
    locals: std::collections::BTreeMap<walrus::LocalId, i32>,
    /// All cast calls we've found in this function (each with its 5
    /// resolved immediates).
    found_calls: Vec<[i32; 5]>,
}

impl CastCallScanner {
    fn new(target: walrus::FunctionId) -> Self {
        Self {
            target,
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
                Instr::Call(Call { func: callee })
                | Instr::ReturnCall(ReturnCall { func: callee }) => {
                    if *callee == self.target {
                        // The call consumes the top 5 stack values.
                        let n = self.operand_stack.len();
                        if n >= 5 {
                            let mut args = [0i32; 5];
                            for (i, slot) in
                                self.operand_stack[n - 5..n].iter().enumerate()
                            {
                                args[i] = slot.unwrap_or(0);
                            }
                            self.found_calls.push(args);
                        }
                        // Consume the args, push nothing (return is unit).
                        for _ in 0..5 {
                            self.operand_stack.pop();
                        }
                    } else {
                        // Unknown function call: consume nothing tracked.
                        // We can't model its arity precisely, but the
                        // body shape between `breaks_if_inlined` and
                        // `__wbindgen_describe_cast` doesn't usually
                        // have intervening calls beyond optimisation
                        // helpers. Bail out conservatively: reset.
                        self.operand_stack.clear();
                        self.locals.clear();
                    }
                }
                _ => {
                    // Anything we don't handle resets the tracked
                    // state. `breaks_if_inlined` bodies are simple
                    // enough that this path shouldn't fire between
                    // the const setup and the marker call.
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
/// is a stable content hash of the closure signature, wasm-ld places
/// it in the table, and we surface its slot here.
///
/// Functions that are exported but not present in any element segment
/// are omitted from the map. If the macro ever emits a SYMBOL_REF
/// naming such a function, the resolver reports a "not found" error,
/// the affected entry is dropped, and the legacy interpreter pathway
/// handles that shim. Same fallback posture as every other section
/// degradation mode.
fn build_symbol_table(module: &Module) -> HashMap<String, u32> {
    let mut out = HashMap::new();
    for export in module.exports.iter() {
        let func_id = match export.item {
            ExportItem::Function(id) => id,
            _ => continue,
        };
        if let Ok(slot) = crate::wasm_conventions::function_table_slot_of(module, func_id) {
            out.insert(export.name.clone(), slot);
            continue;
        }
        // Fallback: some test-runner / transformation layers wrap an
        // exported function (`__wbg_invoke_X`) with a thin shim
        // (`__wbg_invoke_X.command_export`). The export now points at
        // the shim, but the original function still sits in the
        // function table under its original symbol name. Look it up
        // by name and use that slot if found.
        if let Some(slot) = lookup_table_slot_by_name(module, &export.name) {
            out.insert(export.name.clone(), slot);
        }
    }
    out
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
            ElementKind::Active { offset, .. } => match offset {
                ConstExpr::Value(walrus::ir::Value::I32(n)) => *n as u32,
                _ => continue,
            },
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
