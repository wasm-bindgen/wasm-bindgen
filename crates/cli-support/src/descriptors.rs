//! Recovery of wasm-bindgen descriptors.
//!
//! Each `#[wasm_bindgen]` shim has an accompanying descriptor that
//! encodes its type signature for the cli. The cli reads those
//! descriptors here and converts them into [`Descriptor`] values, which
//! the rest of the pipeline consumes when synthesising JS shims.
//!
//! # Transport
//!
//! There are two producers, both feeding the same reference-based
//! `Schema` tree into [`Descriptor`]s:
//!
//! * **Regular shims, imported statics, struct-field getters, and raw
//!   `&dyn Fn` arguments** come from the `__wasm_bindgen_descriptors`
//!   custom section ([`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`]).
//!   The macro serialises each schema tree there as a set of **nodes**,
//!   each identified by a 128-bit content hash; nodes reference their
//!   children — and each entry references its root — by id. The section
//!   carries no linker relocations. [`ingest_section`] parses it, unions
//!   every node into one `id -> node` map, and decodes each entry's root
//!   structurally.
//!
//! * **Casts** (`wbg_cast::<From, To>`) are emitted by generic runtime
//!   code, which cannot place per-monomorphisation bytes into a named
//!   `#[link_section]`. They keep the data-segment transport: each
//!   `__wbg_cast_trampoline*` calls the `__wbindgen_descriptor_marker`
//!   import with the address of a `#[repr(C)] DescriptorRecord`, and
//!   [`ingest_casts`] reads that record and its `Schema` trees out of the
//!   data segment.
//!
//! No wasm interpretation is involved in either path.
//!
//! [`ingest_section`]: WasmBindgenDescriptorsSection::ingest_section
//! [`ingest_casts`]: WasmBindgenDescriptorsSection::ingest_casts

use crate::descriptor::{Descriptor, Function};

use anyhow::{anyhow, bail, Context, Error};
use std::borrow::Cow;
use std::collections::hash_map::HashMap;
use walrus::{CustomSection, ExportItem, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::tys;
use wasm_bindgen_shared::{
    DESCRIPTORS_SECTION_NAME, DESCRIPTOR_FORMAT_VERSION, DESCRIPTOR_KIND_CAST,
    DESCRIPTOR_KIND_REGULAR, DESCRIPTOR_KIND_STATIC, DESCRIPTOR_MARKER_NAME,
};

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub cast_imports: HashMap<Descriptor, Vec<FunctionId>>,
}

pub type WasmBindgenDescriptorsSectionId = TypedCustomSectionId<WasmBindgenDescriptorsSection>;

/// Recover every descriptor from the module.
///
/// First the `__wasm_bindgen_descriptors` custom section (regular /
/// static / import descriptors), then the data-segment cast records.
/// Afterwards the section, the exported descriptor anchors, and the
/// closure invoke exports are stripped so the later GC pass drops them
/// and their now-unreferenced data.
pub fn execute(module: &mut Module) -> Result<WasmBindgenDescriptorsSectionId, Error> {
    let mut section = WasmBindgenDescriptorsSection::default();
    section
        .ingest_section(module)
        .context("failed to read __wasm_bindgen_descriptors section")?;
    section
        .ingest_casts(module)
        .context("failed to recover wasm-bindgen cast descriptors")?;

    // Strip the exported descriptor anchors. Their job (forcing archive
    // inclusion and keeping the `#[link_section]` bytes live through
    // linking) is done; removing the export lets the later GC pass drop
    // the anchor functions.
    strip_prefixed_exports(module, "__wbindgen_descr_anchor_");

    // Strip `__wbg_invoke_*` exports. The macro exports each closure
    // invoke wrapper so the section's `invoke_name` can be resolved to a
    // function-table slot (done above in `ingest_section`). The export
    // itself is no longer needed; wrappers that JS will call stay live
    // through the function table.
    strip_prefixed_exports(module, "__wbg_invoke_");

    Ok(module.customs.add(section))
}

/// Remove every export whose name starts with `prefix`.
fn strip_prefixed_exports(module: &mut Module, prefix: &str) {
    let to_remove: Vec<_> = module
        .exports
        .iter()
        .filter(|e| e.name.starts_with(prefix))
        .map(|e| e.id())
        .collect();
    for id in to_remove {
        module.exports.delete(id);
    }
}

impl WasmBindgenDescriptorsSection {
    /// Parse the `__wasm_bindgen_descriptors` custom section (if present)
    /// and decode every regular/static/import descriptor it contains.
    fn ingest_section(&mut self, module: &mut Module) -> Result<(), Error> {
        let raw = match module.customs.remove_raw(DESCRIPTORS_SECTION_NAME) {
            Some(raw) => raw,
            None => return Ok(()),
        };

        let parsed = parse_section(&raw.data)?;

        // Resolve every closure invoke shim name referenced by a node to a
        // function-table slot. This may append missing wrappers to the
        // table, so it needs `&mut module` and must run before the invoke
        // exports are stripped.
        let symbols = build_symbol_table(module)?;

        let graph = SectionGraph::new(parsed.nodes, &symbols)?;

        for entry in parsed.entries {
            let descriptor = decode_node(&graph, entry.root_id, false, 0).with_context(|| {
                format!("failed to decode descriptor for {:?}", entry.name)
            })?;
            match entry.kind {
                DESCRIPTOR_KIND_REGULAR | DESCRIPTOR_KIND_STATIC => {
                    self.descriptors.insert(entry.name, descriptor);
                }
                DESCRIPTOR_KIND_CAST => {
                    // Casts are never emitted into the section (see module
                    // docs); ignore if one somehow appears.
                    log::debug!("ignoring cast entry {:?} found in section", entry.name);
                }
                other => bail!("descriptor section entry has unknown kind {other}"),
            }
        }
        Ok(())
    }

    /// Recover cast descriptors from the data segment.
    ///
    /// Each `wbg_cast` monomorphisation emits a `__wbg_cast_trampoline*`
    /// whose body calls `__wbindgen_descriptor_marker` with the address of
    /// a `#[repr(C)] DescriptorRecord`. Scan for those calls, read the
    /// record and its `From`/`To` `Schema` trees, and compose the cast's
    /// `fn(From) -> To` descriptor.
    fn ingest_casts(&mut self, module: &mut Module) -> Result<(), Error> {
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

        let data_view = DataSegmentView::new(module);

        let local_funcs: Vec<FunctionId> = module.funcs.iter_local().map(|(id, _)| id).collect();
        for func_id in local_funcs {
            let local = match &module.funcs.get(func_id).kind {
                walrus::FunctionKind::Local(l) => l,
                _ => continue,
            };
            let mut scanner = MarkerCallScanner::new(marker_id);
            scanner.walk(local, local.entry_block());
            for record_ptr in scanner.found_calls {
                if let Some((kind, name, descriptor, cast_to)) =
                    data_view.read_descriptor(record_ptr as u32)?
                {
                    match kind {
                        DESCRIPTOR_KIND_CAST => {
                            let to = cast_to.expect("cast record always carries a `to` schema");
                            let cast = Descriptor::Function(Box::new(Function {
                                arguments: vec![descriptor],
                                shim_idx: 0,
                                ret: to.clone(),
                                inner_ret: Some(to),
                            }));
                            self.cast_imports.entry(cast).or_default().push(func_id);
                        }
                        // Regular/static descriptors now come from the
                        // section; a data-segment record of another kind
                        // would be a producer/consumer mismatch.
                        DESCRIPTOR_KIND_REGULAR | DESCRIPTOR_KIND_STATIC => {
                            let _ = name;
                            bail!(
                                "unexpected non-cast DescriptorRecord in data segment; \
                                 regular/static descriptors must come from the \
                                 {DESCRIPTORS_SECTION_NAME} section"
                            );
                        }
                        other => bail!("descriptor record has unknown kind {other}"),
                    }
                }
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Generic structural decoder
// ---------------------------------------------------------------------------

/// One schema node, resolved out of whatever transport produced it.
struct DecodedNode<K> {
    words: Vec<u32>,
    /// Child node keys (ids for the section, addresses for the data
    /// segment).
    children: Vec<K>,
    /// Closure invoke function-table index, or `0` for a non-closure
    /// node.
    invoke: u32,
}

/// A source of schema nodes addressed by some opaque key. Implemented by
/// the section graph (`Key = u128` content id) and the data-segment view
/// (`Key = u32` linear-memory address) so the opcode grammar is written
/// exactly once.
trait NodeSource {
    type Key: Copy + std::fmt::Debug;
    fn node(&self, key: Self::Key) -> Result<DecodedNode<Self::Key>, Error>;
}

/// Decode the schema node identified by `key` into a [`Descriptor`].
///
/// Scalars (opcode, lengths, names, holes, `nargs`) come from the node's
/// `words`; sub-descriptors come from its `children`. A closure-bearing
/// `FUNCTION` node carries its invoke shim's function-table index in
/// `invoke`. `clamped` propagates through wrapper nodes (only `CLAMPED`
/// sets it, and it resets at function boundaries).
fn decode_node<S: NodeSource>(
    src: &S,
    key: S::Key,
    clamped: bool,
    depth: u32,
) -> Result<Descriptor, Error> {
    if depth > 256 {
        bail!("schema tree nesting exceeds 256 while decoding descriptor");
    }
    let node = src.node(key)?;
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
        tys::FUNCTION => Descriptor::Function(Box::new(decode_function(src, &node, &mut cur, depth)?)),
        tys::CLOSURE => {
            let owned = boolean(cur.word()?)?;
            let mutable = boolean(cur.word()?)?;
            let func = match decode_child(src, &mut cur, false, depth)? {
                Descriptor::Function(f) => *f,
                _ => bail!("closure body is not a FUNCTION node"),
            };
            Descriptor::Closure(Box::new(crate::descriptor::Closure {
                owned,
                mutable,
                function: func,
            }))
        }
        tys::REF => Descriptor::Ref(Box::new(decode_child(src, &mut cur, clamped, depth)?)),
        tys::REFMUT => Descriptor::RefMut(Box::new(decode_child(src, &mut cur, clamped, depth)?)),
        tys::LONGREF => {
            let contents = decode_child(src, &mut cur, clamped, depth)?;
            match contents {
                Descriptor::Externref | Descriptor::NamedExternref(_) => contents,
                _ => Descriptor::Ref(Box::new(contents)),
            }
        }
        tys::SLICE => Descriptor::Slice(Box::new(decode_child(src, &mut cur, clamped, depth)?)),
        tys::VECTOR => Descriptor::Vector(Box::new(decode_child(src, &mut cur, clamped, depth)?)),
        tys::OPTIONAL => Descriptor::Option(Box::new(decode_child(src, &mut cur, clamped, depth)?)),
        tys::RESULT => Descriptor::Result(Box::new(decode_child(src, &mut cur, clamped, depth)?)),
        tys::CLAMPED => decode_child(src, &mut cur, true, depth)?,
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
                variant_types.push(decode_child(src, &mut cur, false, depth)?);
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

/// Decode a `FUNCTION` node. `node`/`cur` are the in-progress node (the
/// `FUNCTION` opcode has already been consumed from `cur`).
///
/// Words: `[FUNCTION, shim_idx_placeholder, nargs]`. The real `shim_idx`
/// is the node's `invoke` index when non-zero (closures); otherwise the
/// placeholder (regular functions, matched by name downstream). Children:
/// `nargs` argument schemas, then `ret`, then `inner_ret`.
fn decode_function<S: NodeSource>(
    src: &S,
    node: &DecodedNode<S::Key>,
    cur: &mut NodeCursor<S::Key>,
    depth: u32,
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
        arguments.push(decode_child(src, cur, false, depth)?);
    }
    let ret = decode_child(src, cur, false, depth)?;
    let inner_ret = Some(decode_child(src, cur, false, depth)?);
    Ok(Function {
        arguments,
        shim_idx,
        ret,
        inner_ret,
    })
}

fn decode_child<S: NodeSource>(
    src: &S,
    cur: &mut NodeCursor<S::Key>,
    clamped: bool,
    depth: u32,
) -> Result<Descriptor, Error> {
    let key = cur.child()?;
    decode_node(src, key, clamped, depth + 1)
}

/// Cursor over a single node's `words` (scalars) and `children`
/// (sub-descriptor keys), decoded in lock-step by the grammar.
struct NodeCursor<'a, K> {
    words: &'a [u32],
    wpos: usize,
    children: &'a [K],
    cpos: usize,
}

impl<'a, K: Copy> NodeCursor<'a, K> {
    fn new(node: &'a DecodedNode<K>) -> Self {
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
            .ok_or_else(|| anyhow!("unexpected end of schema node words"))?;
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
                    .ok_or_else(|| anyhow!("invalid char codepoint {cp:#x} in descriptor"))
            })
            .collect()
    }

    fn child(&mut self) -> Result<K, Error> {
        let c = self
            .children
            .get(self.cpos)
            .copied()
            .ok_or_else(|| anyhow!("schema node has fewer children than its opcode needs"))?;
        self.cpos += 1;
        Ok(c)
    }

    /// Assert the whole node was consumed. Catches producer/consumer
    /// schema drift.
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

// ---------------------------------------------------------------------------
// Section transport (content-addressed node DAG)
// ---------------------------------------------------------------------------

/// A single node as parsed out of the section (invoke still by name).
struct ParsedNode {
    words: Vec<u32>,
    child_ids: Vec<u128>,
    invoke_name: String,
}

/// A single descriptor entry parsed out of the section.
struct ParsedEntry {
    kind: u32,
    name: String,
    root_id: u128,
}

struct ParsedSection {
    nodes: HashMap<u128, ParsedNode>,
    entries: Vec<ParsedEntry>,
}

/// Parse the raw `__wasm_bindgen_descriptors` bytes into its entries and
/// the union of all node records. Entries whose `format_version` is not
/// recognised are skipped byte-accurately (their `body_len` framing is a
/// stable part of the contract).
fn parse_section(bytes: &[u8]) -> Result<ParsedSection, Error> {
    let mut r = Reader::new(bytes);
    let mut nodes: HashMap<u128, ParsedNode> = HashMap::new();
    let mut entries: Vec<ParsedEntry> = Vec::new();

    while !r.at_end() {
        let version = r.u8()?;
        let body_len = r.u32()? as usize;
        let body = r.take(body_len)?;
        if version as u32 != DESCRIPTOR_FORMAT_VERSION {
            log::info!(
                "ignoring {DESCRIPTORS_SECTION_NAME} entry with unrecognised \
                 version {version} (this CLI understands version \
                 {DESCRIPTOR_FORMAT_VERSION})"
            );
            continue;
        }
        let mut b = Reader::new(body);
        let kind = b.u8()? as u32;
        let name_len = b.u32()? as usize;
        let name = String::from_utf8(b.take(name_len)?.to_vec())
            .context("descriptor shim name is not UTF-8")?;
        let root_id = b.u128()?;
        let node_count = b.u32()?;
        for _ in 0..node_count {
            let id = b.u128()?;
            let n_words = b.u32()?;
            let mut words = Vec::with_capacity(n_words as usize);
            for _ in 0..n_words {
                words.push(b.u32()?);
            }
            let n_children = b.u32()?;
            let mut child_ids = Vec::with_capacity(n_children as usize);
            for _ in 0..n_children {
                child_ids.push(b.u128()?);
            }
            let invoke_name_len = b.u32()? as usize;
            let invoke_name = String::from_utf8(b.take(invoke_name_len)?.to_vec())
                .context("closure invoke name is not UTF-8")?;
            // Duplicate node records (the same type emitted by many shims
            // or crates) collapse: identical id implies identical content.
            nodes.entry(id).or_insert(ParsedNode {
                words,
                child_ids,
                invoke_name,
            });
        }
        entries.push(ParsedEntry {
            kind,
            name,
            root_id,
        });
    }

    Ok(ParsedSection { nodes, entries })
}

/// The section's `id -> node` map with every closure invoke name already
/// resolved to a function-table slot.
struct SectionGraph {
    nodes: HashMap<u128, ResolvedNode>,
}

struct ResolvedNode {
    words: Vec<u32>,
    child_ids: Vec<u128>,
    invoke: u32,
}

impl SectionGraph {
    fn new(
        nodes: HashMap<u128, ParsedNode>,
        symbols: &HashMap<String, u32>,
    ) -> Result<Self, Error> {
        let mut resolved = HashMap::with_capacity(nodes.len());
        for (id, node) in nodes {
            let invoke = if node.invoke_name.is_empty() {
                0
            } else {
                *symbols.get(&node.invoke_name).ok_or_else(|| {
                    anyhow!(
                        "closure invoke shim {:?} referenced by a descriptor is not \
                         present in the function table",
                        node.invoke_name
                    )
                })?
            };
            resolved.insert(
                id,
                ResolvedNode {
                    words: node.words,
                    child_ids: node.child_ids,
                    invoke,
                },
            );
        }
        Ok(SectionGraph { nodes: resolved })
    }
}

impl NodeSource for SectionGraph {
    type Key = u128;

    fn node(&self, key: u128) -> Result<DecodedNode<u128>, Error> {
        let node = self
            .nodes
            .get(&key)
            .ok_or_else(|| anyhow!("descriptor references unknown node id {key:#034x}"))?;
        Ok(DecodedNode {
            words: node.words.clone(),
            children: node.child_ids.clone(),
            invoke: node.invoke,
        })
    }
}

/// Minimal little-endian byte reader for the section parser.
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self
            .pos
            .checked_add(n)
            .ok_or_else(|| anyhow!("descriptor section read overflows"))?;
        if end > self.data.len() {
            bail!("descriptor section truncated (wanted {n} bytes)");
        }
        let out = &self.data[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, Error> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn u128(&mut self) -> Result<u128, Error> {
        let b = self.take(16)?;
        let mut arr = [0u8; 16];
        arr.copy_from_slice(b);
        Ok(u128::from_le_bytes(arr))
    }
}

/// Build a `name -> function-table slot` map for closure invoke shims.
///
/// For each exported function that sits in the main function table, the
/// map records its slot index — the value a closure descriptor needs as
/// its `shim_idx`. `__wbg_invoke_*` wrappers that the linker did not
/// place in the table (notably on wasm64) are appended to it here so the
/// name always resolves.
fn build_symbol_table(module: &mut Module) -> Result<HashMap<String, u32>, Error> {
    use walrus::{ConstExpr, ElementItems, ElementKind};

    let mut out = HashMap::new();

    let exports: Vec<(String, FunctionId)> = module
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
        if let Some(slot) = lookup_table_slot_by_name(module, &name) {
            out.insert(name, slot);
            continue;
        }
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
                .map_err(|_| anyhow!("function table initial size does not fit in u32"))?;
            table.initial = table.initial.saturating_add(1);
            if let Some(max) = table.maximum.as_mut() {
                if *max < table.initial {
                    *max = table.initial;
                }
            }
            (slot, table.table64)
        };
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

/// Walk the main function table's element segments and find any function
/// whose own `name` matches `wanted_name`, returning its absolute slot.
/// Used when an export points at a wrapping shim rather than the
/// table-registered original.
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
        let funcs: Vec<FunctionId> = match &segment.items {
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

// ---------------------------------------------------------------------------
// Data-segment transport (casts)
// ---------------------------------------------------------------------------

/// Walk a function body looking for `memory.init data_id` patterns with a
/// constant destination address. For `+bulk-memory` builds (atomics,
/// wasm-shared, etc.) wasm-ld emits passive data segments and a
/// `__wasm_init_memory` ctor; this scanner extracts those destination
/// addresses so cast record / `Schema` pointers can still be resolved.
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
                let _len = stack.pop().unwrap_or(None);
                let _src = stack.pop().unwrap_or(None);
                let dest = stack.pop().unwrap_or(None);
                if let (Some(dest), Some(bytes)) = (dest, passive.get(&m.data)) {
                    out.push((dest as u32, bytes.clone()));
                }
            }
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
/// pointer width.
struct SchemaOffsets {
    words: u32,
    words_len: u32,
    children: u32,
    children_len: u32,
    invoke: u32,
}

/// Snapshot of the module's active data segments, used to read cast
/// `DescriptorRecord`s and walk their `Schema` trees by absolute address.
struct DataSegmentView {
    segments: Vec<(u32, Vec<u8>)>,
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

        if !passive_bytes.is_empty() {
            for (_func_id, local) in module.funcs.iter_local() {
                scan_memory_init(local, &passive_bytes, &mut segments);
            }
        }

        DataSegmentView { segments, ptr_size }
    }

    fn read_ptr(&self, addr: u32) -> Result<u32, Error> {
        let bytes = self.read_bytes(addr, self.ptr_size as usize)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_u32(&self, addr: u32) -> Result<u32, Error> {
        let bytes = self.read_bytes(addr, 4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read a `#[repr(C)] DescriptorRecord` at `addr` and decode the
    /// schema tree(s) it references. Returns `None` for a record whose
    /// `version` this CLI does not recognise.
    fn read_descriptor(
        &self,
        addr: u32,
    ) -> Result<Option<(u32, String, Descriptor, Option<Descriptor>)>, Error> {
        let p = self.ptr_size;
        let version = self.read_u32(addr)?;
        if version != DESCRIPTOR_FORMAT_VERSION {
            log::info!(
                "ignoring DescriptorRecord with unrecognised version {version} \
                 (this CLI understands version {DESCRIPTOR_FORMAT_VERSION})"
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

        let descriptor = decode_node(self, root, false, 0)?;
        let cast_to = if kind == DESCRIPTOR_KIND_CAST {
            Some(decode_node(self, to_root, false, 0)?)
        } else {
            None
        };

        Ok(Some((kind, name, descriptor, cast_to)))
    }

    fn schema_field_offsets(&self) -> SchemaOffsets {
        let p = self.ptr_size;
        let words = p;
        SchemaOffsets {
            words,
            words_len: words + p,
            children: words + 2 * p,
            children_len: words + 3 * p,
            invoke: words + 4 * p,
        }
    }

    fn read_u32_slice(&self, addr: u32, len: u32) -> Result<Vec<u32>, Error> {
        let byte_count = (len as usize)
            .checked_mul(4)
            .ok_or_else(|| anyhow!("schema length overflows"))?;
        let bytes = self.read_bytes(addr, byte_count)?;
        let mut out = Vec::with_capacity(len as usize);
        for chunk in bytes.chunks_exact(4) {
            out.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
        Ok(out)
    }

    fn read_bytes(&self, addr: u32, count: usize) -> Result<Vec<u8>, Error> {
        if count == 0 {
            return Ok(Vec::new());
        }
        for (start, bytes) in &self.segments {
            let end = start
                .checked_add(bytes.len() as u32)
                .ok_or_else(|| anyhow!("data segment address overflow"))?;
            if addr >= *start && addr.saturating_add(count as u32) <= end {
                let offset = (addr - start) as usize;
                return Ok(bytes[offset..offset + count].to_vec());
            }
        }
        bail!(
            "descriptor pointer {addr:#x}..{:#x} is not inside any active data segment",
            addr as u64 + count as u64
        );
    }
}

impl NodeSource for DataSegmentView {
    type Key = u32;

    fn node(&self, addr: u32) -> Result<DecodedNode<u32>, Error> {
        let off = self.schema_field_offsets();
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
                        .ok_or_else(|| anyhow!("schema children run length overflows"))?,
                )
                .ok_or_else(|| anyhow!("schema children pointer overflows"))?;
            children.push(self.read_ptr(slot)?);
        }
        Ok(DecodedNode {
            words,
            children,
            invoke,
        })
    }
}

/// Narrow scanner that finds calls to the descriptor marker import and
/// recovers the single record-pointer immediate fed into each. Handles
/// the trivial optimised shape (one `i32.const`/`i64.const`) plus the
/// debug shape where rustc shuttles the value through a local, and
/// recurses into nested instruction sequences.
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
                    self.operand_stack.pop();
                    self.operand_stack.pop();
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
                        let record_ptr = self.operand_stack.last().copied().flatten();
                        if let Some(ptr) = record_ptr {
                            self.found_calls.push(ptr);
                        }
                        self.operand_stack.pop();
                    } else {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Append one node record to `body` in the wire format the producer's
    /// `describe::schema` module emits.
    fn push_node(body: &mut Vec<u8>, id: u128, words: &[u32], child_ids: &[u128], invoke: &str) {
        body.extend_from_slice(&id.to_le_bytes());
        body.extend_from_slice(&(words.len() as u32).to_le_bytes());
        for w in words {
            body.extend_from_slice(&w.to_le_bytes());
        }
        body.extend_from_slice(&(child_ids.len() as u32).to_le_bytes());
        for c in child_ids {
            body.extend_from_slice(&c.to_le_bytes());
        }
        body.extend_from_slice(&(invoke.len() as u32).to_le_bytes());
        body.extend_from_slice(invoke.as_bytes());
    }

    /// Frame a body into a full section entry (version + body_len prefix).
    fn frame_entry(kind: u32, name: &str, root_id: u128, nodes: &[u8], node_count: u32) -> Vec<u8> {
        let mut body = Vec::new();
        body.push(kind as u8);
        body.extend_from_slice(&(name.len() as u32).to_le_bytes());
        body.extend_from_slice(name.as_bytes());
        body.extend_from_slice(&root_id.to_le_bytes());
        body.extend_from_slice(&node_count.to_le_bytes());
        body.extend_from_slice(nodes);

        let mut entry = Vec::new();
        entry.push(DESCRIPTOR_FORMAT_VERSION as u8);
        entry.extend_from_slice(&(body.len() as u32).to_le_bytes());
        entry.extend_from_slice(&body);
        entry
    }

    fn decode_entry(section: &[u8]) -> Descriptor {
        let parsed = parse_section(section).expect("parse");
        let graph = SectionGraph::new(parsed.nodes, &HashMap::new()).expect("graph");
        let entry = &parsed.entries[0];
        decode_node(&graph, entry.root_id, false, 0).expect("decode")
    }

    #[test]
    fn round_trips_option_i32() {
        // Two nodes: OPTIONAL(id=100) -> I32(id=200).
        let mut nodes = Vec::new();
        push_node(&mut nodes, 100, &[tys::OPTIONAL], &[200], "");
        push_node(&mut nodes, 200, &[tys::I32], &[], "");
        let section = frame_entry(DESCRIPTOR_KIND_REGULAR, "f", 100, &nodes, 2);

        let d = decode_entry(&section);
        assert_eq!(d, Descriptor::Option(Box::new(Descriptor::I32)));
    }

    #[test]
    fn round_trips_function_with_shared_child() {
        // FUNCTION(id=1) with words [FUNCTION, 0, 1]; one arg + ret +
        // inner_ret all referencing the SAME i32 node (id=2) by id — the
        // whole point of content-addressed cross-references.
        let mut nodes = Vec::new();
        push_node(&mut nodes, 1, &[tys::FUNCTION, 0, 1], &[2, 2, 2], "");
        push_node(&mut nodes, 2, &[tys::I32], &[], "");
        let section = frame_entry(DESCRIPTOR_KIND_REGULAR, "add", 1, &nodes, 2);

        match decode_entry(&section) {
            Descriptor::Function(f) => {
                assert_eq!(f.arguments, vec![Descriptor::I32]);
                assert_eq!(f.ret, Descriptor::I32);
                assert_eq!(f.inner_ret, Some(Descriptor::I32));
                assert_eq!(f.shim_idx, 0);
            }
            other => panic!("expected function, got {other:?}"),
        }
    }

    #[test]
    fn unknown_version_entry_is_skipped_and_later_entries_still_parse() {
        // A leading entry with an unknown version must be skipped
        // byte-accurately via its body_len, leaving the real entry intact.
        let mut nodes = Vec::new();
        push_node(&mut nodes, 200, &[tys::I32], &[], "");
        let good = frame_entry(DESCRIPTOR_KIND_STATIC, "s", 200, &nodes, 1);

        let mut section = Vec::new();
        // Bogus future-version entry: version=255, body_len=3, 3 bytes.
        section.push(255);
        section.extend_from_slice(&3u32.to_le_bytes());
        section.extend_from_slice(&[1, 2, 3]);
        section.extend_from_slice(&good);

        let parsed = parse_section(&section).expect("parse");
        assert_eq!(parsed.entries.len(), 1);
        assert_eq!(parsed.entries[0].name, "s");
    }

    #[test]
    fn missing_node_id_is_an_error() {
        // Root references an id that no node record defines.
        let nodes = Vec::new();
        let section = frame_entry(DESCRIPTOR_KIND_REGULAR, "f", 999, &nodes, 0);
        let parsed = parse_section(&section).expect("parse");
        let graph = SectionGraph::new(parsed.nodes, &HashMap::new()).unwrap();
        assert!(decode_node(&graph, 999, false, 0).is_err());
    }
}
