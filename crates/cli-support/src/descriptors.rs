//! Recovery of wasm-bindgen descriptors.
//!
//! Each `#[wasm_bindgen]` shim has an accompanying descriptor that
//! encodes its type signature for the cli. The cli reads those
//! descriptors here and converts them into [`Descriptor`] values,
//! which the rest of the pipeline consumes when synthesising JS shims.
//!
//! # Transport
//!
//! The reference-based `Schema` tree (`wasm_bindgen::describe::Schema`)
//! is the **sole canonical descriptor ABI**. There is no custom section
//! and no flat opcode stream.
//!
//! The `#[wasm_bindgen]` macro and the `wbg_cast` runtime emit, per
//! shim/cast, a `#[repr(C)] DescriptorRecord` static into the data
//! segment plus a function that calls the unified marker import
//! [`wasm_bindgen_shared::DESCRIPTOR_MARKER_NAME`] exactly once with the
//! record's address:
//!
//! * **Regular shims / imported statics** emit a small *carrier*
//!   function whose only body is the marker call. The carrier is
//!   exported (which keeps the record alive and forces the archive
//!   member to be pulled); [`WasmBindgenDescriptorsSection::ingest`]
//!   reads its record by the embedded shim name and then strips the
//!   carrier export so the walrus GC drops both the carrier and the
//!   descriptor bytes.
//! * **Casts** call the marker from their `__wbg_cast_trampoline*`,
//!   which is replaced wholesale by a synthesised JS-adapter import
//!   later in the pipeline.
//!
//! Discovery is uniform: [`ingest`](WasmBindgenDescriptorsSection::ingest)
//! scans every function body for calls to the marker, reads each
//! `DescriptorRecord` out of the data segment, walks its `Schema`
//! tree(s) structurally into [`Descriptor`]s, and dispatches on the
//! record's `kind`. No wasm interpretation is involved.

use crate::descriptor::{Descriptor, Function};

use anyhow::{bail, Context, Error};
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::tys;
use wasm_bindgen_shared::tys::SchemaTag;
use wasm_bindgen_shared::{
    DESCRIPTOR_FORMAT_VERSION, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_GENERIC_IMPORT,
    DESCRIPTOR_KIND_REGULAR, DESCRIPTOR_KIND_STATIC, DESCRIPTOR_MARKER_NAME,
};

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub cast_imports: HashMap<Descriptor, Vec<FunctionId>>,
    /// Generic `#[wasm_bindgen]` imports, keyed by shim name. Each entry is
    /// the list of per-monomorphisation `(concrete Function descriptor,
    /// courier function id)` recovered from the call sites: the descriptor is
    /// the template spliced with that monomorphisation's fills, and the
    /// function id is the `breaks_if_inlined_generic_import_*` courier whose
    /// call site is rewritten to the synthesised per-`T` JS adapter import.
    pub generic_imports: HashMap<String, Vec<(Descriptor, FunctionId)>>,
}

pub type WasmBindgenDescriptorsSectionId = TypedCustomSectionId<WasmBindgenDescriptorsSection>;

/// Recover every descriptor from the module by scanning for calls to the
/// descriptor marker import, reading each [`DescriptorRecord`] out of the
/// data segment, and decoding its `Schema` tree(s).
///
/// Afterwards the marker-carrier exports are stripped so the later GC
/// pass drops the carriers and their (now unreferenced) descriptor data.
pub fn execute(module: &mut Module) -> Result<WasmBindgenDescriptorsSectionId, Error> {
    let mut section = WasmBindgenDescriptorsSection::default();
    section
        .ingest(module)
        .context("failed to recover wasm-bindgen descriptors")?;

    // Strip `__wbg_invoke_*` exports. The macro exports each closure invoke
    // wrapper only to give it a clean, stable symbol name; the wrapper is
    // discovered structurally (via the descriptor's relocated
    // `Schema::invoke` function-table index), not by export name. Removing
    // the export lets the later GC pass drop any wrapper whose only
    // remaining liveness root was this export; wrappers that JS will call
    // stay live through the function table.
    strip_closure_invoke_exports(module);

    Ok(module.customs.add(section))
}

/// Remove every `__wbg_invoke_*` export from the module. Their job (giving
/// the macro-emitted closure wrappers a clean exported name) is done by
/// the time this runs.
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

/// One descriptor recovered from a `DescriptorRecord` in the data
/// segment, paired with the function whose body hosted the marker call.
struct FoundRecord {
    /// The function that called the marker. For a cast this is the
    /// `__wbg_cast_trampoline*`; for a regular/static descriptor it is
    /// the exported carrier function.
    host: FunctionId,
    kind: u32,
    /// Shim name (empty for casts).
    name: String,
    descriptor: Descriptor,
    /// `Some(to)` for casts (the `To` type), else `None`.
    cast_to: Option<Descriptor>,
}

impl WasmBindgenDescriptorsSection {
    fn ingest(&mut self, module: &mut Module) -> Result<(), Error> {
        // Locate the unified descriptor marker import. If it isn't
        // present, this module emitted no descriptors.
        let marker_id = module.imports.iter().find_map(|import| {
            if import.module == "__wbindgen_placeholder__" && import.name == DESCRIPTOR_MARKER_NAME {
                if let walrus::ImportKind::Function(id) = import.kind {
                    return Some(id);
                }
            }
            None
        });
        let marker_id = match marker_id {
            Some(id) => id,
            None => return Ok(()),
        };

        // Snapshot the data segments so we can read each `DescriptorRecord`
        // and its `Schema` trees by absolute address.
        let data_view = DataSegmentView::new(module);

        // Find every function that calls the marker and the
        // record-pointer immediate fed to each call.
        let local_funcs: Vec<FunctionId> =
            module.funcs.iter_local().map(|(id, _)| id).collect();
        let mut found: Vec<FoundRecord> = Vec::new();
        for func_id in local_funcs {
            let local = match &module.funcs.get(func_id).kind {
                walrus::FunctionKind::Local(l) => l,
                _ => continue,
            };
            let mut scanner = MarkerCallScanner::new(marker_id);
            scanner.walk(local, local.entry_block());
            for record_ptr in scanner.found_calls {
                if let Some(rec) = data_view.read_descriptor(record_ptr as u32, func_id)? {
                    found.push(rec);
                }
            }
        }

        // Dispatch each recovered record and remember which carrier
        // exports to strip (the regular/static hosts; cast trampolines
        // are replaced downstream and must stay).
        let mut carriers_to_strip: Vec<FunctionId> = Vec::new();
        for rec in found {
            match rec.kind {
                DESCRIPTOR_KIND_REGULAR | DESCRIPTOR_KIND_STATIC => {
                    self.descriptors.insert(rec.name, rec.descriptor);
                    carriers_to_strip.push(rec.host);
                }
                DESCRIPTOR_KIND_GENERIC_IMPORT => {
                    // A per-monomorphisation generic import. `rec.descriptor`
                    // is the concrete `FUNCTION` descriptor (template spliced
                    // with the call site's fills); `rec.host` is the courier
                    // whose call site is rewritten to the synthesised per-`T`
                    // JS adapter. Do NOT strip the host: it is replaced
                    // downstream (like a cast trampoline) and must stay.
                    self.generic_imports
                        .entry(rec.name)
                        .or_default()
                        .push((rec.descriptor, rec.host));
                }
                DESCRIPTOR_KIND_CAST => {
                    let to = rec
                        .cast_to
                        .expect("cast record always carries a `to` schema");
                    // The cast itself is a single-argument function:
                    // `fn(From) -> To`. `inner_ret` mirrors `ret`, as the
                    // rest of the pipeline expects.
                    let cast = Descriptor::Function(Box::new(Function {
                        arguments: vec![rec.descriptor],
                        shim_idx: 0,
                        ret: to.clone(),
                        inner_ret: Some(to),
                    }));
                    self.cast_imports.entry(cast).or_default().push(rec.host);
                }
                other => bail!("descriptor record has unknown kind {other}"),
            }
        }

        // Strip the marker-carrier exports. Their job (keeping the record
        // alive and pulling the archive member) is done; removing the
        // export lets the later GC pass drop the carrier function, its
        // marker call, and the now-unreferenced descriptor bytes.
        if !carriers_to_strip.is_empty() {
            let strip: std::collections::HashSet<FunctionId> =
                carriers_to_strip.into_iter().collect();
            let export_ids: Vec<_> = module
                .exports
                .iter()
                .filter(|e| match e.item {
                    walrus::ExportItem::Function(id) => strip.contains(&id),
                    _ => false,
                })
                .map(|e| e.id())
                .collect();
            for id in export_ids {
                module.exports.delete(id);
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
/// addresses so descriptor/`Schema` pointers can still be resolved.
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

/// Byte offsets of the `#[repr(C)] Schema` fields for a given target
/// pointer width (see [`DataSegmentView::schema_field_offsets`]).
struct SchemaOffsets {
    words: u32,
    words_len: u32,
    children: u32,
    children_len: u32,
    invoke: u32,
}

/// Snapshot of the module's active data segments. Lets us resolve a
/// linear-memory address to the bytes that wasm-ld wrote into the data
/// section at link time. Used to read each `DescriptorRecord` and walk
/// its `Schema` trees by absolute address.
struct DataSegmentView {
    segments: Vec<(u32, Vec<u8>)>, // (start address, bytes)
    /// Target pointer width in bytes (4 on wasm32, 8 on wasm64). The
    /// `#[repr(C)]` `Schema` / `DescriptorRecord` layouts use
    /// pointer-sized fields, so parsing them out of the data segment is
    /// pointer-width dependent.
    ptr_size: u32,
}

impl DataSegmentView {
    fn new(module: &Module) -> Self {
        use walrus::DataKind;

        let ptr_size = if module.memories.iter().any(|m| m.memory64) {
            8
        } else {
            4
        };

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
        // active so the scanner can resolve descriptor/schema pointers.
        if !passive_bytes.is_empty() {
            for (_func_id, local) in module.funcs.iter_local() {
                scan_memory_init(local, &passive_bytes, &mut segments);
            }
        }

        DataSegmentView { segments, ptr_size }
    }

    /// Read a pointer-sized field (`ptr_size` bytes LE) at `addr`,
    /// returning its low 32 bits. Data-segment addresses, run lengths,
    /// and function-table indices all fit in `u32` in practice, so the
    /// rest of the pipeline keeps using `u32` even on wasm64.
    fn read_ptr(&self, addr: u32) -> Result<u32, Error> {
        let bytes = self.read_bytes(addr, self.ptr_size as usize)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read a `u32` (always 4 bytes LE, independent of pointer width) at
    /// `addr`.
    fn read_u32(&self, addr: u32) -> Result<u32, Error> {
        let bytes = self.read_bytes(addr, 4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read a `#[repr(C)] DescriptorRecord` at `addr` and decode the
    /// schema tree(s) it references. Returns `None` (after logging) for a
    /// record whose `version` this CLI does not recognise.
    ///
    /// `DescriptorRecord` layout (pointer-width dependent):
    /// `version: u32`, `kind: u32`, then four/five pointer-aligned
    /// fields `name: *const u8`, `name_len: usize`, `root: *const Schema`,
    /// `to_root: *const Schema`. On wasm64 the two leading `u32`s exactly
    /// fill the 8 bytes before the first 8-byte-aligned pointer.
    fn read_descriptor(
        &self,
        addr: u32,
        host: FunctionId,
    ) -> Result<Option<FoundRecord>, Error> {
        let p = self.ptr_size;
        let version = self.read_u32(addr)?;
        if version != DESCRIPTOR_FORMAT_VERSION {
            log::info!(
                "ignoring DescriptorRecord with unrecognised version {version} \
                 (this CLI understands version {DESCRIPTOR_FORMAT_VERSION}); the \
                 binary was likely produced by a newer wasm-bindgen"
            );
            return Ok(None);
        }
        let kind = self.read_u32(addr + 4)?;
        let name_ptr = self.read_ptr(addr + 8)?;
        let name_len = self.read_ptr(addr + 8 + p)?;
        let root = self.read_ptr(addr + 8 + 2 * p)?;
        let to_root = self.read_ptr(addr + 8 + 3 * p)?;

        let name = if name_len == 0 {
            String::new()
        } else {
            let bytes = self.read_bytes(name_ptr, name_len as usize)?;
            String::from_utf8(bytes).context("descriptor shim name is not UTF-8")?
        };

        // For a generic import, read the per-monomorphisation fills run
        // (one `&Schema` per distinct type parameter, indexed by parameter
        // index) and resolve each `TypeParam(idx)` hole in the template with
        // `fills[idx]` while decoding.
        let fills: Vec<u32> = if kind == DESCRIPTOR_KIND_GENERIC_IMPORT {
            let fills_ptr = self.read_ptr(addr + 8 + 4 * p)?;
            let fills_len = self.read_ptr(addr + 8 + 5 * p)?;
            let mut v = Vec::with_capacity(fills_len as usize);
            for i in 0..fills_len {
                let slot = fills_ptr
                    .checked_add(i.checked_mul(p).ok_or_else(|| {
                        anyhow::anyhow!("generic-import fills run length overflows")
                    })?)
                    .ok_or_else(|| anyhow::anyhow!("generic-import fills pointer overflows"))?;
                v.push(self.read_ptr(slot)?);
            }
            v
        } else {
            Vec::new()
        };

        let descriptor = self.decode_node(root, false, 0, &fills)?;
        let cast_to = if kind == DESCRIPTOR_KIND_CAST {
            Some(self.decode_node(to_root, false, 0, &fills)?)
        } else {
            None
        };

        Ok(Some(FoundRecord {
            host,
            kind,
            name,
            descriptor,
            cast_to,
        }))
    }

    /// `#[repr(C)] Schema` field byte offsets for the target pointer
    /// width. Layout: `tag: u32`, then pointer-aligned `words: *const
    /// u32`, `words_len: usize`, `children: *const *const Schema`,
    /// `children_len: usize`, `invoke: *const ()`. On wasm64 the `u32`
    /// tag is followed by 4 bytes of padding before the first
    /// 8-byte-aligned pointer.
    fn schema_field_offsets(&self) -> SchemaOffsets {
        let p = self.ptr_size;
        let words = p; // align_up(4, ptr_size): 4 on wasm32, 8 on wasm64
        SchemaOffsets {
            words,
            words_len: words + p,
            children: words + 2 * p,
            children_len: words + 3 * p,
            invoke: words + 4 * p,
        }
    }

    /// Read one `Schema` node out of the data segment.
    fn read_node(&self, addr: u32) -> Result<SchemaNode, Error> {
        let off = self.schema_field_offsets();
        let tag = self.read_u32(addr)?;
        let words_ptr = self.read_ptr(addr + off.words)?;
        let words_len = self.read_ptr(addr + off.words_len)?;
        let children_ptr = self.read_ptr(addr + off.children)?;
        let children_len = self.read_ptr(addr + off.children_len)?;
        let invoke = self.read_ptr(addr + off.invoke)?;

        let words = self.read_u32_slice(words_ptr, words_len)?;
        let mut children = Vec::with_capacity(children_len as usize);
        for i in 0..children_len {
            let slot = children_ptr
                .checked_add(
                    i.checked_mul(self.ptr_size)
                        .ok_or_else(|| anyhow::anyhow!("schema children run length overflows"))?,
                )
                .ok_or_else(|| anyhow::anyhow!("schema children pointer overflows"))?;
            children.push(self.read_ptr(slot)?);
        }
        Ok(SchemaNode {
            tag,
            words,
            children,
            invoke,
        })
    }

    /// Decode the `Schema` node rooted at `addr` into a [`Descriptor`].
    ///
    /// Scalars (opcode, lengths, names, holes, `nargs`) come from the
    /// node's `words`; sub-descriptors come from its `children`. A
    /// closure-bearing `FUNCTION` node carries its invoke shim's
    /// function-table index in `invoke`. `clamped` propagates through
    /// wrapper nodes exactly as it did in the previous flat decoder
    /// (only `CLAMPED` sets it, and it resets at function boundaries).
    fn decode_node(
        &self,
        addr: u32,
        clamped: bool,
        depth: u32,
        fills: &[u32],
    ) -> Result<Descriptor, Error> {
        if depth > 256 {
            bail!("schema tree nesting exceeds 256 while decoding descriptor");
        }
        let node = self.read_node(addr)?;
        // A generic-import template hole: splice in the concrete fill for this
        // parameter index (the hole's single word) and decode that instead.
        if node.tag == SchemaTag::TypeParam as u32 {
            let idx = *node
                .words
                .first()
                .ok_or_else(|| anyhow::anyhow!("TypeParam schema node has no parameter index"))?;
            let fill = *fills.get(idx as usize).ok_or_else(|| {
                anyhow::anyhow!(
                    "generic import references type parameter {idx} but only {} fill(s) supplied",
                    fills.len()
                )
            })?;
            return self.decode_node(fill, clamped, depth + 1, fills);
        }
        let mut cur = NodeCursor::new(&node);
        let opcode = cur.word()?;
        let descriptor = match opcode {
            tys::I8 => Descriptor::I8,
            tys::I16 => Descriptor::I16,
            tys::I32 => Descriptor::I32,
            tys::I64 => Descriptor::I64,
            tys::I64_AS_F64 => Descriptor::I64AsF64,
            tys::I128 => Descriptor::I128,
            tys::U8 if clamped => Descriptor::ClampedU8,
            tys::U8 => Descriptor::U8,
            tys::U16 => Descriptor::U16,
            tys::U32 => Descriptor::U32,
            tys::U64 => Descriptor::U64,
            tys::U64_AS_F64 => Descriptor::U64AsF64,
            tys::U128 => Descriptor::U128,
            tys::F32 => Descriptor::F32,
            tys::F64 => Descriptor::F64,
            tys::BOOLEAN => Descriptor::Boolean,
            tys::CHAR => Descriptor::Char,
            tys::UNIT => Descriptor::Unit,
            tys::NONNULL => Descriptor::NonNull,
            tys::RAW_POINTER => Descriptor::RawPointer,
            tys::CACHED_STRING => Descriptor::CachedString,
            tys::STRING => Descriptor::String,
            tys::EXTERNREF => Descriptor::Externref,
            tys::FUNCTION => {
                Descriptor::Function(Box::new(self.decode_function(&node, &mut cur, depth, fills)?))
            }
            tys::CLOSURE => {
                let owned = boolean(cur.word()?)?;
                let mutable = boolean(cur.word()?)?;
                // The single child is the closure's `FUNCTION` node
                // (which carries the invoke index in its `invoke` field).
                let func = match self.decode_child(&mut cur, false, depth, fills)? {
                    Descriptor::Function(f) => *f,
                    _ => bail!("closure body is not a FUNCTION node"),
                };
                Descriptor::Closure(Box::new(crate::descriptor::Closure {
                    owned,
                    mutable,
                    function: func,
                }))
            }
            tys::REF => Descriptor::Ref(Box::new(self.decode_child(&mut cur, clamped, depth, fills)?)),
            tys::REFMUT => {
                Descriptor::RefMut(Box::new(self.decode_child(&mut cur, clamped, depth, fills)?))
            }
            tys::LONGREF => {
                // Most things become normal `Ref`s, but long refs to
                // externrefs become owned.
                let contents = self.decode_child(&mut cur, clamped, depth, fills)?;
                match contents {
                    Descriptor::Externref | Descriptor::NamedExternref(_) => contents,
                    _ => Descriptor::Ref(Box::new(contents)),
                }
            }
            tys::SLICE => Descriptor::Slice(Box::new(self.decode_child(&mut cur, clamped, depth, fills)?)),
            tys::VECTOR => {
                Descriptor::Vector(Box::new(self.decode_child(&mut cur, clamped, depth, fills)?))
            }
            tys::OPTIONAL => {
                Descriptor::Option(Box::new(self.decode_child(&mut cur, clamped, depth, fills)?))
            }
            tys::RESULT => {
                Descriptor::Result(Box::new(self.decode_child(&mut cur, clamped, depth, fills)?))
            }
            tys::CLAMPED => self.decode_child(&mut cur, true, depth, fills)?,
            tys::ENUM => {
                let name = cur.string()?;
                let hole = cur.word()?;
                Descriptor::Enum { name, hole }
            }
            tys::STRING_ENUM => {
                let name = cur.string()?;
                let variant_count = cur.word()?;
                Descriptor::StringEnum {
                    name,
                    invalid: variant_count,
                    hole: variant_count + 1,
                }
            }
            tys::DYNAMIC_UNION => {
                let name = cur.string()?;
                let type_count = cur.word()?;
                let mut variant_types = Vec::with_capacity(type_count as usize);
                for _ in 0..type_count {
                    variant_types.push(self.decode_child(&mut cur, false, depth, fills)?);
                }
                Descriptor::DynamicUnion {
                    name,
                    variant_types,
                }
            }
            tys::RUST_STRUCT => Descriptor::RustStruct(cur.string()?),
            tys::NAMED_EXTERNREF => Descriptor::NamedExternref(cur.string()?),
            other => bail!("unknown descriptor opcode: {other}"),
        };
        cur.finish()?;
        Ok(descriptor)
    }

    /// Decode a `FUNCTION` node. `node`/`cur` are the in-progress node
    /// (the `FUNCTION` opcode has already been consumed from `cur`).
    ///
    /// Words: `[FUNCTION, shim_idx_placeholder, nargs]`. The real
    /// `shim_idx` is the node's relocated `invoke` index when non-zero
    /// (closures); otherwise the placeholder (regular functions, which
    /// the rest of the pipeline matches by name). Children: `nargs`
    /// argument schemas, then `ret`, then `inner_ret`.
    fn decode_function(
        &self,
        node: &SchemaNode,
        cur: &mut NodeCursor,
        depth: u32,
        fills: &[u32],
    ) -> Result<Function, Error> {
        let placeholder = cur.word()?;
        let shim_idx = if node.invoke != 0 {
            node.invoke
        } else {
            placeholder
        };
        let nargs = cur.word()?;
        let mut arguments = Vec::with_capacity(nargs as usize);
        for _ in 0..nargs {
            arguments.push(self.decode_child(cur, false, depth, fills)?);
        }
        let ret = self.decode_child(cur, false, depth, fills)?;
        let inner_ret = Some(self.decode_child(cur, false, depth, fills)?);
        Ok(Function {
            arguments,
            shim_idx,
            ret,
            inner_ret,
        })
    }

    /// Decode the next child of the current node.
    fn decode_child(
        &self,
        cur: &mut NodeCursor,
        clamped: bool,
        depth: u32,
        fills: &[u32],
    ) -> Result<Descriptor, Error> {
        let addr = cur.child()?;
        self.decode_node(addr, clamped, depth + 1, fills)
    }

    /// Read `len` `u32` words (`4 * len` bytes) starting at linear-
    /// memory address `addr`.
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
        // An empty run's base pointer is a dangling-but-aligned address
        // (the producer takes it via `<[_]>::as_ptr` on an empty slice),
        // so it is not inside any data segment. Short-circuit before the
        // segment scan so a zero-length read never fails on that pointer.
        if count == 0 {
            return Ok(Vec::new());
        }
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
            "descriptor pointer {addr:#x}..{:#x} is not inside any active data segment",
            addr as u64 + count as u64
        );
    }
}

/// One `Schema` node read out of the data segment.
struct SchemaNode {
    /// Structural tag (`SchemaTag::Leaf` / `Wrap` / `TypeParam`).
    tag: u32,
    words: Vec<u32>,
    /// Child node addresses (already resolved from the children run).
    children: Vec<u32>,
    /// Relocated closure invoke function-table index, or `0`.
    invoke: u32,
}

/// Cursor over a single node's `words` (scalars) and `children`
/// (sub-descriptor addresses), decoded in lock-step by the grammar.
struct NodeCursor<'a> {
    words: &'a [u32],
    wpos: usize,
    children: &'a [u32],
    cpos: usize,
}

impl<'a> NodeCursor<'a> {
    fn new(node: &'a SchemaNode) -> Self {
        NodeCursor {
            words: &node.words,
            wpos: 0,
            children: &node.children,
            cpos: 0,
        }
    }

    fn word(&mut self) -> Result<u32, Error> {
        let w = self
            .words
            .get(self.wpos)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("unexpected end of schema node words"))?;
        self.wpos += 1;
        Ok(w)
    }

    /// A `get_string`-style payload: a length word followed by that many
    /// `char` codepoint words.
    fn string(&mut self) -> Result<String, Error> {
        let len = self.word()?;
        (0..len)
            .map(|_| {
                let cp = self.word()?;
                char::from_u32(cp)
                    .ok_or_else(|| anyhow::anyhow!("invalid char codepoint {cp:#x} in descriptor"))
            })
            .collect()
    }

    fn child(&mut self) -> Result<u32, Error> {
        let c = self
            .children
            .get(self.cpos)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("schema node has fewer children than its opcode needs"))?;
        self.cpos += 1;
        Ok(c)
    }

    /// Assert the whole node was consumed — no leftover scalar words or
    /// undecoded children. Catches producer/consumer schema drift.
    fn finish(&self) -> Result<(), Error> {
        if self.wpos != self.words.len() {
            bail!(
                "schema node has {} leftover scalar word(s) after decoding",
                self.words.len() - self.wpos
            );
        }
        if self.cpos != self.children.len() {
            bail!(
                "schema node has {} undecoded child/children after decoding",
                self.children.len() - self.cpos
            );
        }
        Ok(())
    }
}

fn boolean(word: u32) -> Result<bool, Error> {
    match word {
        0 => Ok(false),
        1 => Ok(true),
        other => bail!("expected bool value (0/1) in schema, got {other}"),
    }
}

/// Narrow scanner that finds calls to the descriptor marker import and
/// recovers the single record-pointer immediate fed into each. Handles
/// the trivial optimised shape (one `i32.const`/`i64.const`) plus the
/// debug shape where rustc shuttles the value through a local, and
/// recurses into nested instruction sequences so a non-flat (debug)
/// carrier/trampoline body is still handled.
///
/// Not a wasm interpreter: it tracks only `i32.const`/`i64.const` values
/// written into locals and the operand stack the next `call` consumes.
struct MarkerCallScanner {
    target: walrus::FunctionId,
    operand_stack: Vec<Option<i32>>,
    locals: std::collections::BTreeMap<walrus::LocalId, i32>,
    found_calls: Vec<i32>,
}

impl MarkerCallScanner {
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
                Instr::Store(_) => {
                    self.operand_stack.pop(); // value
                    self.operand_stack.pop(); // addr
                }
                Instr::Load(_) => {
                    self.operand_stack.pop();
                    self.operand_stack.push(None);
                }
                Instr::GlobalGet(_) => {
                    self.operand_stack.push(None);
                }
                Instr::GlobalSet(_) => {
                    self.operand_stack.pop();
                }
                Instr::Binop(_) => {
                    self.operand_stack.pop();
                    self.operand_stack.pop();
                    self.operand_stack.push(None);
                }
                Instr::Unop(_) => {
                    self.operand_stack.pop();
                    self.operand_stack.push(None);
                }
                // Recurse into nested instruction sequences so a
                // debug/unoptimised carrier or trampoline body that
                // wraps the marker call in a block/loop/if is still
                // scanned. State is reset around the descent because the
                // operand stack at the nested boundary is not tracked.
                Instr::Block(b) => {
                    self.operand_stack.clear();
                    self.locals.clear();
                    self.walk(func, b.seq);
                    self.operand_stack.clear();
                    self.locals.clear();
                }
                Instr::Loop(l) => {
                    self.operand_stack.clear();
                    self.locals.clear();
                    self.walk(func, l.seq);
                    self.operand_stack.clear();
                    self.locals.clear();
                }
                Instr::IfElse(ifelse) => {
                    self.operand_stack.clear();
                    self.locals.clear();
                    self.walk(func, ifelse.consequent);
                    self.operand_stack.clear();
                    self.locals.clear();
                    self.walk(func, ifelse.alternative);
                    self.operand_stack.clear();
                    self.locals.clear();
                }
                Instr::Call(Call { func: callee })
                | Instr::ReturnCall(ReturnCall { func: callee }) => {
                    if *callee == self.target {
                        // The call consumes the top stack value: the
                        // record pointer.
                        let record_ptr = self.operand_stack.last().copied().flatten();
                        if let Some(ptr) = record_ptr {
                            self.found_calls.push(ptr);
                        }
                        self.operand_stack.pop();
                    } else {
                        // Unknown call: reset, since we don't track its
                        // arity. Carrier/trampoline bodies don't have
                        // intervening calls between the const setup and
                        // the marker.
                        self.operand_stack.clear();
                        self.locals.clear();
                    }
                }
                _ => {
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
