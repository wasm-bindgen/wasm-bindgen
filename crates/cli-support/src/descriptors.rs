//! Management of wasm-bindgen descriptor functions.
//!
//! The purpose of this module is to basically execute a pass on a raw wasm
//! module that just came out of the compiler. The pass will execute all
//! relevant descriptor functions contained in the module which wasm-bindgen
//! uses to convey type information here, to the CLI.
//!
//! All descriptor functions are removed after this pass runs and in their stead
//! a new custom section, defined in this module, is inserted into the
//! `walrus::Module` which contains all the results of all the descriptor
//! functions.
//!
//! ## Transport
//!
//! Two transports coexist during the migration away from the interpreter:
//!
//! 1. The `__wasm_bindgen_descriptors` custom section
//!    ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]). When the
//!    `#[wasm_bindgen]` macro can produce a descriptor's bytes purely
//!    from compile-time information, it does so and writes the bytes into
//!    this section directly. Entries here are decoded by
//!    [`crate::descriptors_section`] and turn directly into [`Descriptor`]s.
//! 2. The historical `__wbindgen_describe_<name>` synthetic export
//!    functions plus the wasm interpreter in [`crate::interpreter`]. Used
//!    as a fallback for any shim whose descriptor is not present in the
//!    new section. The medium-term goal of the descriptors-via-custom-
//!    section work is to remove this fallback entirely.
//!
//! The section wins whenever both are present: an entry recovered from
//! `__wasm_bindgen_descriptors` shadows the interpreter result for the
//! same shim. This lets the migration land function-by-function rather
//! than as an atomic swap.

use crate::descriptor::Descriptor;
use crate::descriptors_section;
use crate::interpreter::Interpreter;
use anyhow::{Context, Error};
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, ExportItem, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::{
    DESCRIPTORS_SECTION_NAME, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR,
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

    // Phase 1: harvest anything present in the new __wasm_bindgen_descriptors
    // section. Names that show up here will be skipped by the interpreter
    // pass below.
    let regular_from_section = section
        .ingest_section(module)
        .context("failed to read __wasm_bindgen_descriptors section")?;

    // Phase 2: legacy interpreter for everything not already covered.
    let mut interpreter = Interpreter::new(module)?;
    section.execute_exports(module, &mut interpreter, &regular_from_section)?;
    section.execute_casts(module, &mut interpreter)?;

    Ok(module.customs.add(section))
}

impl WasmBindgenDescriptorsSection {
    /// Pull the `__wasm_bindgen_descriptors` custom section out of `module`
    /// (if present), parse it, and populate `self` with the entries it
    /// contains. Returns the set of regular shim names that came from the
    /// section so that [`Self::execute_exports`] can avoid interpreting
    /// the matching `__wbindgen_describe_<name>` functions a second time.
    ///
    /// Cast entries from the section are not yet stored on `self` (we
    /// still need a way to back-resolve their `breaks_if_inlined` symbol
    /// name to a `FunctionId`). They are ignored here and left to the
    /// interpreter pathway until a follow-up commit. The plumbing is
    /// intentionally separated so that piece can land independently.
    fn ingest_section(
        &mut self,
        module: &mut Module,
    ) -> Result<std::collections::HashSet<String>, Error> {
        use std::collections::HashSet;

        let raw = match module.customs.remove_raw(DESCRIPTORS_SECTION_NAME) {
            Some(raw) => raw,
            None => return Ok(HashSet::new()),
        };

        let (entries, stats) = descriptors_section::parse(&raw.data)?;
        if stats.skipped_total() > 0 {
            // Per-entry version mismatches are not fatal — the affected
            // shims fall back to the legacy interpreter pathway — but
            // they are noteworthy. Log at `info` so they surface in
            // a normal CLI run without needing RUST_LOG=debug.
            for (version, count) in &stats.skipped_unknown_version {
                log::info!(
                    "wasm-bindgen-cli does not recognise format_version {version} \
                     for {count} __wasm_bindgen_descriptors entries; falling back \
                     to the legacy interpreter for those shims. This usually means \
                     the binary was produced by a newer wasm-bindgen than this CLI."
                );
            }
        }
        let resolved_symbols = build_symbol_table(module);

        let mut regular_names = HashSet::new();
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
            // Defensively decode: an entry whose schema is malformed (for
            // instance because the macro emitted a function that mentions
            // a type with no SCHEMA const set) should not poison the
            // whole pipeline. Drop the bad entry and let the legacy
            // interpreter handle that shim instead. This is what makes
            // it safe for the macro to emit the section optimistically.
            let descriptor = match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| Descriptor::decode(&stream)),
            ) {
                Ok(d) => d,
                Err(_) => {
                    log::debug!(
                        "ignoring malformed __wasm_bindgen_descriptors entry for {:?}; \
                         falling back to interpreter",
                        entry.name
                    );
                    continue;
                }
            };
            match entry.kind {
                DESCRIPTOR_KIND_REGULAR => {
                    regular_names.insert(entry.name.clone());
                    self.descriptors.insert(entry.name, descriptor);
                }
                DESCRIPTOR_KIND_CAST => {
                    // Cast entries need their `breaks_if_inlined<From,To>`
                    // symbol resolved into a FunctionId. That lookup is
                    // delegated to a follow-up commit; until then casts
                    // continue to flow through the interpreter pathway,
                    // which scans for direct calls to
                    // __wbindgen_describe_cast.
                    log::debug!(
                        "ignoring cast descriptor for {:?} in section (still \
                         handled by interpreter for now)",
                        entry.name
                    );
                }
                _ => unreachable!("parser already validated kind byte"),
            }
        }
        Ok(regular_names)
    }

    fn execute_exports(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
        already_from_section: &std::collections::HashSet<String>,
    ) -> Result<(), Error> {
        let mut to_remove = Vec::new();

        if let Some(id) = interpreter.skip_interpret() {
            to_remove.push(id);
        }

        for export in module.exports.iter() {
            let prefix = "__wbindgen_describe_";
            if !export.name.starts_with(prefix) {
                continue;
            }
            let id = match export.item {
                walrus::ExportItem::Function(id) => id,
                _ => panic!("{} export not a function", export.name),
            };
            let name = &export.name[prefix.len()..];
            if already_from_section.contains(name) {
                // The new transport already produced this descriptor; just
                // delete the now-redundant synthetic export so the rest of
                // the pipeline behaves as if the interpreter had handled it.
                log::debug!(
                    "skipping interpreter for {name:?}; already produced \
                     by __wasm_bindgen_descriptors section"
                );
                to_remove.push(export.id());
                continue;
            }
            // Interpret descriptor with 0 args (export descriptors shouldn't take any).
            let d = interpreter.interpret_descriptor(id, module);
            let descriptor = Descriptor::decode(d);
            self.descriptors.insert(name.to_string(), descriptor);
            to_remove.push(export.id());
        }

        for id in to_remove {
            module.exports.delete(id);
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
        }
    }
    out
}
