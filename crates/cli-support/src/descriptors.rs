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
use crate::interpreter::Interpreter;
use anyhow::{Context, Error};
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, ExportItem, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::{
    DESCRIPTORS_SECTION_NAME, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR,
    DESCRIPTOR_KIND_STATIC,
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

    // Phase 3: invoke the interpreter — scoped to closure-cast
    // descriptor recovery only. See module-level docs for why this
    // single path still requires interpretation.
    let mut interpreter = Interpreter::new(module)?;
    section.execute_casts(module, &mut interpreter)?;
    strip_skip_interpret_export(module);

    // Phase 3: strip __wbg_invoke_* exports.
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

/// Remove the `__wbindgen_skip_interpret_calls` export from the
/// module. This export's only purpose is to give the closure-cast
/// interpreter a place to find wasm-ld-injected ctor calls to skip
/// (see [`crate::interpreter::skip_calls`]). Once the interpreter
/// pass has consumed it, it's dead weight.
fn strip_skip_interpret_export(module: &mut Module) {
    let to_remove: Vec<_> = module
        .exports
        .iter()
        .filter(|e| e.name == "__wbindgen_skip_interpret_calls")
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
            let stream = descriptors_section::resolve_symbols(
                &entry.schema_bytes,
                &resolved_symbols,
            )
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

    fn execute_casts(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        use walrus::ir::*;

        // If our describe cast intrinsic isn't present or wasn't linked
        // then there're no casts, so nothing to do!
        let wbindgen_describe_cast = match interpreter.describe_cast_id() {
            Some(i) => i,
            None => return Ok(()),
        };

        // Find all functions which call `wbindgen_describe_cast`. These are
        // specially codegen'd so we know the rough structure of them. For each
        // one we delegate to the interpreter to figure out the source and
        // target type descriptors.
        let mut replace_with_imports = Vec::new();
        for (func_id, local) in module.funcs.iter_local() {
            let mut find = FindDescribeCast {
                wbindgen_describe_cast,
                found: false,
            };
            dfs_in_order(&mut find, local, local.entry_block());
            if find.found {
                replace_with_imports.push(func_id);
            }
        }
        for func_id in replace_with_imports {
            let descriptor = interpreter.interpret_descriptor(func_id, module);
            let descriptor = Descriptor::decode(descriptor);
            self.cast_imports
                .entry(descriptor)
                .or_default()
                .push(func_id);
        }

        return Ok(());

        struct FindDescribeCast {
            wbindgen_describe_cast: FunctionId,
            found: bool,
        }

        impl Visitor<'_> for FindDescribeCast {
            fn visit_call(&mut self, call: &Call) {
                if call.func == self.wbindgen_describe_cast {
                    self.found = true;
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
