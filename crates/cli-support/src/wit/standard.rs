use crate::descriptor::{SeqValue, VectorKind};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashSet};
use walrus::{ExportId, FunctionId, ImportId, RefType, TypedCustomSectionId};

#[derive(Default, Debug)]
pub struct NonstandardWitSection {
    /// A list of adapter functions, keyed by their id.
    ///
    /// This map is iterated over in multiple places, so we use an ordered map
    /// to ensure that the order of iteration is deterministic. This map affects
    /// all parts of the generated code, so it's important to get this right.
    pub adapters: BTreeMap<AdapterId, Adapter>,

    /// A list of pairs for adapter functions that implement core Wasm imports.
    pub implements: Vec<(ImportId, FunctionId, AdapterId)>,

    /// A list of adapter functions and the names they're exported under.
    pub exports: Vec<(ExportId, AdapterId)>,
}

pub type NonstandardWitSectionId = TypedCustomSectionId<NonstandardWitSection>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct AdapterId(pub usize);

#[derive(Debug, Clone)]
pub struct Adapter {
    pub id: AdapterId,
    pub params: Vec<AdapterType>,
    pub results: Vec<AdapterType>,
    pub inner_results: Vec<AdapterType>,
    pub kind: AdapterKind,
}

#[derive(Debug, Clone)]
pub enum AdapterKind {
    Local {
        instructions: Vec<InstructionData>,
    },
    Import {
        name: String,
        kind: AdapterJsImportKind,
    },
}

#[derive(Debug, Clone)]
pub struct InstructionData {
    pub instr: Instruction,
    pub stack_change: StackChange,
}

/// Calls `f` on every instruction in `instrs`, including those nested in an
/// `EncodedSliceLoad` / `EncodedSliceStore` element coder, which a pass that
/// rewrites or roots instructions must reach too. Passes over the adapter's
/// stack shape must not use this: the nested instructions run on the loop's
/// own stack, so to them the enclosing instruction is a single one.
pub fn visit_instructions(instrs: &[InstructionData], f: &mut impl FnMut(&InstructionData)) {
    for instr in instrs {
        f(instr);
        if let Instruction::EncodedSliceLoad { elem, .. }
        | Instruction::EncodedSliceStore { elem, .. } = &instr.instr
        {
            visit_instructions(elem, f);
        }
    }
}

/// [`visit_instructions`], mutably.
pub fn visit_instructions_mut(
    instrs: &mut [InstructionData],
    f: &mut impl FnMut(&mut InstructionData),
) {
    for instr in instrs {
        f(instr);
        if let Instruction::EncodedSliceLoad { elem, .. }
        | Instruction::EncodedSliceStore { elem, .. } = &mut instr.instr
        {
            visit_instructions_mut(elem, f);
        }
    }
}

#[derive(Debug, Clone)]
pub enum StackChange {
    Modified { pushed: usize, popped: usize },
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AdapterJsImportKind {
    /// The first argument is an `externref` which is the `this` of the function
    /// call
    Method,
    /// The value imported should be invoked as `new`
    Constructor,
    /// A bland function import
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterType {
    S8,
    S16,
    S32,
    S64,
    S128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    String,
    Externref,
    Bool,
    I32,
    I64,
    Vector(VectorKind),
    /// A plain array of the element type.
    Array(Box<AdapterType>),
    /// A `&mut [T]` argument. Unlike `Vector`, the contents are copied back
    /// into the buffer of the passed typed array after the call, so it can't
    /// be a plain array.
    MutableSlice(VectorKind),
    Option(Box<AdapterType>),
    Struct(String),
    Enum(String),
    StringEnum(String),
    DynamicUnion(String),
    NamedExternref(String),
    Function,
    NonNull,
}

/// Describes how a closure's lifetime is managed.
#[derive(Debug, Clone)]
pub enum ClosureDtor {
    /// Persistent/owned closure with a destructor function that will be called
    /// when the closure is dropped on the JS side.
    OwnClosure,
    /// Borrowed closure called from JS (e.g., `forEach` callback). The closure
    /// is invalidated after the JS call returns by calling `_wbg_cb_unref`.
    Immediate,
    /// Borrowed closure (`ScopedClosure`) passed to a Rust async import. The closure's lifetime
    /// is tied to the async call and cleanup is handled on the Rust side.
    Borrowed,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// Call the deallocation function.
    DeferFree {
        free: walrus::FunctionId,
        align: usize,
    },
    /// A call to one of our own defined adapters, similar to the standard
    /// call-adapter instruction
    CallAdapter(AdapterId),
    /// Call an exported function in the core module
    CallExport(walrus::ExportId),

    /// Gets an argument by its index.
    ArgGet(u32),

    /// An instruction to store `ty` at the `offset` index in the return pointer
    StoreRetptr {
        ty: AdapterType,
        offset: usize,
        mem: walrus::MemoryId,
    },
    /// An instruction to load `ty` at the `offset` index from the return pointer
    LoadRetptr {
        ty: AdapterType,
        offset: usize,
        mem: walrus::MemoryId,
    },
    /// An instruction which pushes the return pointer onto the stack, reserving
    /// `size` bytes of space.
    Retptr {
        size: u32,
    },

    /// Pops a 32/16/8-bit integer (`u8`, `s16`, etc.) and pushes a Wasm `i32`.
    Int32ToWasm,
    /// Pops a Wasm `i32` and pushes a 32-bit integer.
    WasmToInt32 {
        /// Whether the integer represents an unsigned 32-bit value.
        unsigned_32: bool,
    },

    /// Pops a 64-bit integer and pushes a Wasm `i64`.
    Int64ToWasm,
    /// Pops a Wasm `i64` and pushes a 64-bit integer.
    WasmToInt64 {
        unsigned: bool,
    },

    /// Pops a 128-bit integer and pushes 2 Wasm 64-bit ints.
    Int128ToWasm,
    /// Pops 2 Wasm 64-bit ints and pushes a 128-bit integer.
    WasmToInt128 {
        signed: bool,
    },

    OptionInt128ToWasm,
    OptionWasmToInt128 {
        signed: bool,
    },

    /// Pops a Wasm `i32` and pushes the enum variant as a string
    WasmToStringEnum {
        name: String,
    },

    OptionWasmToStringEnum {
        name: String,
    },

    /// pops a string and pushes the enum variant as an `i32`
    StringEnumToWasm {
        name: String,
        invalid: u32,
    },

    OptionStringEnumToWasm {
        name: String,
        invalid: u32,
        hole: u32,
    },

    /// Pops a `bool` from the stack and pushes an `i32` equivalent
    I32FromBool,
    /// Pops a `string` from the stack and pushes the first character as `i32`
    I32FromStringFirstChar,
    /// Pops an `externref` from the stack, allocates space in the externref table,
    /// returns the index it was stored at.
    I32FromExternrefOwned,
    /// Pops an `externref` from the stack, pushes it onto the externref Wasm table
    /// stack, and returns the index it was stored at.
    I32FromExternrefBorrow,
    /// Pops an `externref` from the stack, assumes it's a Rust class given, and
    /// deallocates the JS object and returns the i32 Rust pointer.
    I32FromExternrefRustOwned {
        class: String,
    },
    /// Pops an `externref` from the stack, assumes it's a Rust class given, and
    /// passes the pointer to Rust which will be borrowed for the duration of a
    /// call
    I32FromExternrefRustBorrow {
        class: String,
    },
    /// Pops an `externref` from the stack, pushes 0 if it's "none" or the
    /// consumed pointer value if it's "some".
    I32FromOptionRust {
        class: String,
    },
    /// Pops an `externref` from the stack, pushes either 0 if it's "none" or and
    /// index into the owned Wasm table it was stored at if it's "some"
    I32FromOptionExternref {
        /// Set to `Some` by the externref pass of where to put it in the wasm
        /// module, otherwise it's shoved into the JS shim.
        table_and_alloc: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    /// Pops an `externref` from the stack, pushes either a sentinel value if it's
    /// "none" or the integer value of it if it's "some"
    I32FromOptionU32Sentinel,
    /// Pops an `externref` from the stack, pushes 0 for "none", 1 for
    /// "some(false)', and 2 for "some(true)"
    I32FromOptionBool,
    /// Pops an `externref` from the stack, pushes a sentinel for "none" or the
    /// value if it's "some"
    I32FromOptionChar,
    /// Pops a numeric enum's number and pushes it unchanged as an `f64`.
    F64FromEnum,
    /// Pops an `externref` from the stack, pushes `hole` for "none" or the
    /// value if it's "some"
    F64FromOptionEnum {
        hole: u32,
    },
    /// Pops an `externref` from the stack, pushes either a sentinel value if it's
    /// "none" or the integer value of it if it's "some"
    F64FromOptionSentinelInt {
        signed: bool,
    },
    /// Pops an optional JS number, preserving its numeric value while reserving
    /// `Number.MAX_SAFE_INTEGER` as the ABI sentinel for pointer-sized options.
    F64FromOptionSentinelNumber,
    /// Pops an `externref` from the stack, pushes either a sentinel value if it's
    /// "none" or the f32 value of it if it's "some"
    F64FromOptionSentinelF32,
    /// Pops any externref from the stack and then pushes two values. First is a
    /// 0/1 if it's none/some and second is `ty` value if it was there or 0 if
    /// it wasn't there.
    FromOptionNative {
        ty: walrus::ValType,
    },

    /// Pops a vector value of `kind` from the stack, allocates memory with
    /// `malloc`, and then copies all the data into `mem`. Pushes the pointer
    /// and length as i32.
    ///
    /// These allocating instructions also carry `free`, which releases the
    /// allocation when a later argument throws before Rust takes it.
    VectorToMemory {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        free: walrus::FunctionId,
        mem: walrus::MemoryId,
    },
    MutableSliceToMemory {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        free: walrus::FunctionId,
        mem: walrus::MemoryId,
    },

    /// Pops a string, pushes pointer/length or all zeros
    OptionString {
        malloc: walrus::FunctionId,
        free: walrus::FunctionId,
        mem: walrus::MemoryId,
        realloc: Option<walrus::FunctionId>,
    },
    /// Pops a string, pushes pointer/length
    StringToMemory {
        malloc: walrus::FunctionId,
        free: walrus::FunctionId,
        mem: walrus::MemoryId,
        realloc: Option<walrus::FunctionId>,
    },
    /// Pops a pointer + length, pushes a string
    MemoryToString(walrus::MemoryId),

    /// Pops an externref, pushes pointer/length or all zeros
    OptionVector {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        free: walrus::FunctionId,
        mem: walrus::MemoryId,
    },

    /// Pops a nullable externref; if it is non-zero, throws it.
    UnwrapResult {
        /// Similar to `I32FromOptionExternref`,
        /// Set to `Some` by the externref pass, and we then take from the externref table. If
        /// None, we use takeObject.
        table_and_drop: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    UnwrapResultString {
        table_and_drop: Option<(walrus::TableId, walrus::FunctionId)>,
    },

    /// pops a `i32`, pushes `bool`
    BoolFromI32,
    /// pops `i32`, loads externref at that slot, dealloates externref, pushes `externref`
    ExternrefLoadOwned {
        /// This is needed solely for `Result`, since it can contain externrefs,
        /// but has to pass them through a retptr.
        table_and_drop: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    /// pops `i32`, pushes string from that `char`
    StringFromChar,
    /// pops `i32`, pushes an externref for the wrapped rust class
    RustFromI32 {
        class: String,
    },
    OptionRustFromI32 {
        class: String,
    },
    /// pops ptr/length i32, loads string from cache
    CachedStringLoad {
        owned: bool,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
        /// If we're in reference-types mode, the externref table ID to get the cached string from.
        table: Option<walrus::TableId>,
    },
    /// pops ptr/length, pushes a vector, frees the original data. Freed data
    /// cannot be left as a view, so `SeqValue::View` renders as `Copy`.
    VectorLoad {
        kind: VectorKind,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
        mode: SeqValue,
    },
    /// pops ptr/length, pushes a vector, frees the original data
    OptionVectorLoad {
        kind: VectorKind,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
        mode: SeqValue,
    },
    /// pops ptr/length of a buffer holding `length` element ABIs, each laid
    /// out as `words`, pushes the array `elem` decodes them into (or
    /// `undefined` for a null `optional` buffer), frees the buffer
    EncodedSliceLoad {
        words: Vec<AdapterType>,
        elem: Vec<InstructionData>,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
        optional: bool,
    },
    /// pops an array (or, `optional`, `undefined`), encodes each element with
    /// `elem` into the `words` of its ABI in a buffer allocated with `malloc`
    /// (freed with `free` if an element throws), pushes the buffer's
    /// ptr/length (ptr 0 for `undefined`)
    EncodedSliceStore {
        words: Vec<AdapterType>,
        elem: Vec<InstructionData>,
        malloc: walrus::FunctionId,
        free: walrus::FunctionId,
        mem: walrus::MemoryId,
        optional: bool,
    },
    /// pops i32, loads externref from externref table
    TableGet,
    /// pops two i32 data pointers, pushes an externref closure
    Closure {
        adapter: AdapterId,
        nargs: usize,
        mutable: bool,
        dtor: ClosureDtor,
    },
    /// pops two i32 data pointers, pushes a vector view (or a copy of it, per
    /// `mode`); the borrowed data is not freed
    View {
        kind: VectorKind,
        mem: walrus::MemoryId,
        mode: SeqValue,
    },
    /// pops two i32 data pointers, pushes an optional vector view (or a copy
    /// of it, per `mode`); the borrowed data is not freed
    OptionView {
        kind: VectorKind,
        mem: walrus::MemoryId,
        mode: SeqValue,
    },
    /// pops f64, pushes it viewed as an optional value with a known sentinel
    OptionF64Sentinel,
    /// pops i32, pushes it viewed as an optional value with a known sentinel
    OptionU32Sentinel,
    /// pops an i32, then `ty`, then pushes externref
    ToOptionNative {
        ty: walrus::ValType,
        signed: bool,
    },
    OptionBoolFromI32,
    OptionCharFromI32,
    OptionEnumFromF64 {
        hole: u32,
    },
    I32FromOptionNonNull,
    OptionNonNullFromI32,
    I32FromNonNull,
}

impl AdapterType {
    pub fn from_wasm(wasm: walrus::ValType) -> Option<AdapterType> {
        Some(match wasm {
            walrus::ValType::I32 => AdapterType::I32,
            walrus::ValType::I64 => AdapterType::I64,
            walrus::ValType::F32 => AdapterType::F32,
            walrus::ValType::F64 => AdapterType::F64,
            walrus::ValType::Ref(RefType::EXTERNREF) => AdapterType::Externref,
            walrus::ValType::Ref(_) | walrus::ValType::V128 => return None,
        })
    }

    pub fn to_wasm(&self) -> Option<walrus::ValType> {
        Some(match self {
            AdapterType::I32 => walrus::ValType::I32,
            AdapterType::I64 => walrus::ValType::I64,
            AdapterType::F32 => walrus::ValType::F32,
            AdapterType::F64 => walrus::ValType::F64,
            AdapterType::Enum(_) => walrus::ValType::F64,
            AdapterType::Externref | AdapterType::NamedExternref(_) => {
                walrus::ValType::Ref(RefType::EXTERNREF)
            }
            _ => return None,
        })
    }

    pub fn option(self) -> AdapterType {
        AdapterType::Option(Box::new(self))
    }
}

impl NonstandardWitSection {
    pub fn append(
        &mut self,
        params: Vec<AdapterType>,
        results: Vec<AdapterType>,
        inner_results: Vec<AdapterType>,
        kind: AdapterKind,
    ) -> AdapterId {
        let id = AdapterId(self.adapters.len());
        self.adapters.insert(
            id,
            Adapter {
                id,
                params,
                results,
                inner_results,
                kind,
            },
        );
        id
    }

    /// Removes any dead entries in `adapters` that are no longer necessary
    /// and/or no longer referenced.
    ///
    /// Returns `true` if any adapters were deleted, or `false` if the adapters
    /// did not change.
    pub fn gc(&mut self) -> bool {
        // Populate the live set with the exports, implements directives, and
        // anything transitively referenced by those adapters.
        let mut live = HashSet::new();
        for (_, id) in self.exports.iter() {
            self.add_live(*id, &mut live);
        }
        for (_, _, id) in self.implements.iter() {
            self.add_live(*id, &mut live);
        }

        // And now that we have the live set we can filter out our list of
        // adapter definitions.
        let before = self.adapters.len();
        self.adapters.retain(|id, _| live.contains(id));
        before != self.adapters.len()
    }

    fn add_live(&self, id: AdapterId, live: &mut HashSet<AdapterId>) {
        if !live.insert(id) {
            return;
        }
        let instructions = match &self.adapters[&id].kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => return,
        };
        for instr in instructions {
            match instr.instr {
                Instruction::Closure { adapter, .. } | Instruction::CallAdapter(adapter) => {
                    self.add_live(adapter, live);
                }
                _ => {}
            }
        }
    }
}

impl walrus::CustomSection for NonstandardWitSection {
    fn name(&self) -> &str {
        "nonstandard wit section"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<'_, [u8]> {
        panic!("shouldn't emit custom sections just yet");
    }

    fn add_gc_roots(&self, roots: &mut walrus::passes::Roots) {
        use Instruction::*;

        for adapter in self.adapters.values() {
            let instrs = match &adapter.kind {
                AdapterKind::Local { instructions } => instructions,
                AdapterKind::Import { .. } => continue,
            };
            visit_instructions(instrs, &mut |instr| match instr.instr {
                DeferFree { free: f, .. } => {
                    roots.push_func(f);
                }
                StoreRetptr { mem, .. }
                | LoadRetptr { mem, .. }
                | View { mem, .. }
                | OptionView { mem, .. }
                | MemoryToString(mem) => {
                    roots.push_memory(mem);
                }
                VectorToMemory {
                    malloc, free, mem, ..
                }
                | OptionVector {
                    malloc, free, mem, ..
                }
                | MutableSliceToMemory {
                    malloc, free, mem, ..
                } => {
                    roots.push_memory(mem);
                    roots.push_func(malloc);
                    roots.push_func(free);
                }
                EncodedSliceStore {
                    malloc, free, mem, ..
                } => {
                    roots.push_memory(mem);
                    roots.push_func(malloc);
                    roots.push_func(free);
                }
                VectorLoad { free, mem, .. }
                | OptionVectorLoad { free, mem, .. }
                | EncodedSliceLoad { free, mem, .. }
                | CachedStringLoad { free, mem, .. } => {
                    roots.push_memory(mem);
                    roots.push_func(free);
                }
                OptionString {
                    mem,
                    malloc,
                    free,
                    realloc,
                }
                | StringToMemory {
                    mem,
                    malloc,
                    free,
                    realloc,
                } => {
                    roots.push_memory(mem);
                    roots.push_func(malloc);
                    roots.push_func(free);
                    if let Some(id) = realloc {
                        roots.push_func(id);
                    }
                }
                I32FromOptionExternref {
                    table_and_alloc: Some((table, alloc)),
                } => {
                    roots.push_table(table);
                    roots.push_func(alloc);
                }
                UnwrapResult { table_and_drop } | UnwrapResultString { table_and_drop } => {
                    if let Some((table, drop)) = table_and_drop {
                        roots.push_table(table);
                        roots.push_func(drop);
                    }
                }
                _ => {}
            });
        }
    }
}
