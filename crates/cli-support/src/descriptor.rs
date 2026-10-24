use std::char;

use wasm_bindgen_shared::identifier::is_valid_ident;
use wasm_bindgen_shared::tys::*;

/// Which kind of per-monomorphisation binding a discovered descriptor function
/// asks for.
///
/// On the wire these are distinguished only by the length of the leading key
/// string: zero-length means [`Cast`](Self::Cast). Keeping that distinction in
/// the type rather than as an empty-`String` sentinel means the two cases can't
/// be confused, and in particular that the "a cast takes exactly one argument"
/// invariant in `bind_generic_imports` is a property of the `Cast` variant
/// rather than of an untyped string being empty.
///
/// The derived `Ord` orders `Cast` before every `Shim`, which is only used as a
/// deterministic tie-break; see `bind_generic_imports`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GenericImportKey {
    /// A [`wbg_cast`](wasm_bindgen::__rt::wbg_cast) identity adapter. Needs no
    /// AST metadata: it is bound as JS that returns its single argument
    /// unchanged.
    Cast,
    /// A `#[wasm_bindgen(experimental_generic_mono)]` import. The string is the shim key
    /// identifying which generic-import AST entry supplies the JS binding
    /// metadata (name, kind, namespace, catch, ...).
    Shim(String),
}

// `PartialOrd`/`Ord` exist so that a decoded descriptor can be used as a total
// tie-breaker when sorting (see `bind_generic_imports`). The order itself is
// arbitrary — it follows variant declaration order — and is not meaningful.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Descriptor {
    I8,
    U8,
    ClampedU8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I64AsF64,
    U64AsF64,
    I128,
    U128,
    F32,
    F64,
    Boolean,
    Function(Box<Function>),
    Closure(Box<Closure>),
    Ref(Box<Descriptor>),
    RefMut(Box<Descriptor>),
    Slice(Box<Descriptor>),
    Vector(Box<Descriptor>),
    CachedString,
    String,
    Externref,
    NamedExternref(String),
    Enum {
        name: String,
        hole: u32,
        unique_crate_identifier: String,
        module_hash: u32,
    },
    StringEnum {
        name: String,
        invalid: u32,
        hole: u32,
        unique_crate_identifier: String,
        module_hash: u32,
    },
    DynamicUnion {
        name: String,
        variant_types: Vec<Descriptor>,
        unique_crate_identifier: String,
        module_hash: u32,
    },
    RustStruct {
        name: String,
        unique_crate_identifier: String,
    },
    Char,
    /// `Option<T>`: `undefined` and `null` both read as `None`, and `None`
    /// is written as `undefined`.
    Option(Box<Descriptor>),
    /// `Nullable<T>`: `null` crosses as its own empty value. Inside an
    /// `Option`, it is the `T | null | undefined` union.
    Nullable(Box<Descriptor>),
    Result(Box<Descriptor>),
    Unit,
    NonNull,
    RawPointer,
    /// An imported function argument under a `slice = ..` or `vec = ..` mode
    /// that applies to it, or that was written on it, for the binding to
    /// validate (see [`Descriptor::seq_mode_fit`]).
    SeqMode(SeqMode, Box<Descriptor>),
}

/// How a sequence's cells hold its elements, which the element's descriptor
/// decides.
#[derive(Debug, PartialEq, Eq)]
pub enum SequenceCell<'a> {
    /// The cell is the element itself: a number, or a JS handle's index.
    InPlace(VectorKind),
    /// The cell is the element's scalar ABI, converted by its own glue.
    Encoded(&'a Descriptor),
}

/// How a typed sequence reaches JS: as a view of wasm memory, a fresh plain
/// `Array`, or a fresh typed array.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SeqValue {
    #[default]
    View,
    Array,
    Copy,
}

/// An imported function argument's `slice = ..` or `vec = ..` mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SeqMode {
    /// `vec = ..`, for owned sequences, rather than `slice = ..`.
    pub vec: bool,
    pub value: SeqValue,
    /// Written on the argument rather than inherited.
    pub explicit: bool,
}

/// As written: `slice = copy`.
impl std::fmt::Debug for SeqMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let key = if self.vec { "vec" } else { "slice" };
        let value = match self.value {
            SeqValue::View => "view",
            SeqValue::Array => "array",
            SeqValue::Copy => "copy",
        };
        write!(f, "{key} = {value}")
    }
}

/// Kinds of exported items that share the one JS/TypeScript name space.
/// Structs and numeric enums are JS values, string enums and dynamic unions
/// are TypeScript aliases, but a name can only be declared once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemKind {
    Value,
    StringEnum,
    DynamicUnion,
}

/// Where a Rust type named in a descriptor is defined.
#[derive(Clone, Copy)]
pub struct ItemOrigin<'a> {
    pub kind: ItemKind,
    pub unique_crate_identifier: &'a str,
    /// `module_path_hash` of the defining module; `None` for structs, whose
    /// wasm shims are only unique per crate.
    pub module_hash: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Function {
    pub arguments: Vec<Descriptor>,
    pub shim_idx: u32,
    pub ret: Descriptor,
    pub inner_ret: Option<Descriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Closure {
    pub owned: bool,
    pub function: Function,
    pub mutable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VectorKind {
    I8,
    U8,
    ClampedU8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    Externref,
    NamedExternref(String),
}

impl Descriptor {
    pub fn decode(mut data: &[u32]) -> Descriptor {
        let descriptor = Descriptor::_decode(&mut data, false);
        assert!(data.is_empty(), "remaining data {data:?}");
        descriptor
    }

    /// Decode a per-monomorphisation generic-import descriptor stream.
    ///
    /// The stream is a length-prefixed `shim` key string (identifying which
    /// generic-import AST entry supplies the JS binding metadata) followed by
    /// the concrete `FUNCTION` signature for this monomorphisation. A
    /// zero-length key on the wire denotes a [`GenericImportKey::Cast`]; the
    /// wire format is unchanged by that being an enum here.
    pub fn decode_generic_import(mut data: &[u32]) -> (GenericImportKey, Descriptor) {
        let key = get_string(&mut data);
        let descriptor = Descriptor::_decode(&mut data, false);
        assert!(data.is_empty(), "remaining data {data:?}");
        let key = if key.is_empty() {
            GenericImportKey::Cast
        } else {
            GenericImportKey::Shim(key)
        };
        (key, descriptor)
    }

    fn _decode(data: &mut &[u32], clamped: bool) -> Descriptor {
        match get(data) {
            I8 => Descriptor::I8,
            I16 => Descriptor::I16,
            I32 => Descriptor::I32,
            I64 => Descriptor::I64,
            I64_AS_F64 => Descriptor::I64AsF64,
            I128 => Descriptor::I128,
            U8 if clamped => Descriptor::ClampedU8,
            U8 => Descriptor::U8,
            U16 => Descriptor::U16,
            U32 => Descriptor::U32,
            U64 => Descriptor::U64,
            U64_AS_F64 => Descriptor::U64AsF64,
            U128 => Descriptor::U128,
            F32 => Descriptor::F32,
            F64 => Descriptor::F64,
            BOOLEAN => Descriptor::Boolean,
            FUNCTION => Descriptor::Function(Box::new(Function::decode(data))),
            CLOSURE => Descriptor::Closure(Box::new(Closure::decode(data))),
            REF => Descriptor::Ref(Box::new(Descriptor::_decode(data, clamped))),
            REFMUT => Descriptor::RefMut(Box::new(Descriptor::_decode(data, clamped))),
            LONGREF => {
                // This descriptor basically just serves as a macro, where most things
                // become normal `Ref`s, but long refs to externrefs become owned.
                let contents = Descriptor::_decode(data, clamped);
                match contents {
                    Descriptor::Externref | Descriptor::NamedExternref(_) => contents,
                    _ => Descriptor::Ref(Box::new(contents)),
                }
            }
            SLICE => Descriptor::Slice(Box::new(Descriptor::_decode(data, clamped))),
            VECTOR => Descriptor::Vector(Box::new(Descriptor::_decode(data, clamped))),
            OPTIONAL => Descriptor::Option(Box::new(Descriptor::_decode(data, clamped))),
            NULLABLE => Descriptor::Nullable(Box::new(Descriptor::_decode(data, clamped))),
            RESULT => Descriptor::Result(Box::new(Descriptor::_decode(data, clamped))),
            CACHED_STRING => Descriptor::CachedString,
            STRING => Descriptor::String,
            EXTERNREF => Descriptor::Externref,
            ENUM => {
                let name = get_string(data);
                let hole = get(data);
                let unique_crate_identifier = get_string(data);
                let module_hash = get(data);
                Descriptor::Enum {
                    name,
                    hole,
                    unique_crate_identifier,
                    module_hash,
                }
            }
            STRING_ENUM => {
                let name = get_string(data);
                let variant_count = get(data);
                let invalid = variant_count;
                let hole = variant_count + 1;
                let unique_crate_identifier = get_string(data);
                let module_hash = get(data);
                Descriptor::StringEnum {
                    name,
                    invalid,
                    hole,
                    unique_crate_identifier,
                    module_hash,
                }
            }
            DYNAMIC_UNION => {
                let name = get_string(data);
                let unique_crate_identifier = get_string(data);
                let module_hash = get(data);
                let type_count = get(data);
                let mut variant_types = Vec::new();
                for _ in 0..type_count {
                    variant_types.push(Descriptor::_decode(data, clamped));
                }
                Descriptor::DynamicUnion {
                    name,
                    variant_types,
                    unique_crate_identifier,
                    module_hash,
                }
            }
            RUST_STRUCT => {
                let name = get_string(data);
                let unique_crate_identifier = get_string(data);
                Descriptor::RustStruct {
                    name,
                    unique_crate_identifier,
                }
            }
            NAMED_EXTERNREF => {
                let name = get_string(data);
                Descriptor::NamedExternref(name)
            }
            CHAR => Descriptor::Char,
            UNIT => Descriptor::Unit,
            CLAMPED => Descriptor::_decode(data, true),
            NONNULL => Descriptor::NonNull,
            RAW_POINTER => Descriptor::RawPointer,
            // The macro marks every argument in an inherited mode's scope, so
            // a mode with nothing to do drops here, and the argument keeps its
            // default handling. One that can't be honoured is kept for the
            // binding to reject.
            SEQ_MODE => {
                let word = get(data);
                let mode = SeqMode {
                    vec: word & SEQ_MODE_VEC != 0,
                    value: match word & !(SEQ_MODE_VEC | SEQ_MODE_EXPLICIT) {
                        SEQ_MODE_VIEW => SeqValue::View,
                        SEQ_MODE_COPY => SeqValue::Copy,
                        SEQ_MODE_ARRAY => SeqValue::Array,
                        other => panic!("unknown slice mode: {other}"),
                    },
                    explicit: word & SEQ_MODE_EXPLICIT != 0,
                };
                let contents = Descriptor::_decode(data, clamped);
                if contents.seq_mode_fit(mode) == Ok(false) {
                    contents
                } else {
                    Descriptor::SeqMode(mode, Box::new(contents))
                }
            }
            other => panic!("unknown descriptor: {other}"),
        }
    }

    /// Visit every Rust type and externref type name embedded in this
    /// descriptor tree, so callers can rewrite names to their final JS
    /// identities (see `Context::resolve_descriptor` in `wit`). Rust types
    /// include where they are defined; named externrefs do not.
    pub fn visit_named_types_mut<E>(
        &mut self,
        f: &mut impl FnMut(&mut String, Option<ItemOrigin<'_>>) -> Result<(), E>,
    ) -> Result<(), E> {
        match self {
            Descriptor::Function(function) => function.visit_named_types_mut(f),
            Descriptor::Closure(closure) => closure.function.visit_named_types_mut(f),
            Descriptor::Ref(d)
            | Descriptor::RefMut(d)
            | Descriptor::Slice(d)
            | Descriptor::Vector(d)
            | Descriptor::Result(d) => d.visit_named_types_mut(f),
            Descriptor::Option(inner) | Descriptor::Nullable(inner) => {
                inner.visit_named_types_mut(f)
            }
            // An externref has no defining crate, so a JS type whose name
            // matches an ambiguous all-`private` struct/enum name still hits
            // the ambiguity check even though it never refers to the Rust
            // type.
            Descriptor::NamedExternref(name) => f(name, None),
            Descriptor::RustStruct {
                name,
                unique_crate_identifier,
            } => f(
                name,
                Some(ItemOrigin {
                    kind: ItemKind::Value,
                    unique_crate_identifier,
                    module_hash: None,
                }),
            ),
            Descriptor::Enum {
                name,
                unique_crate_identifier,
                module_hash,
                ..
            } => f(
                name,
                Some(ItemOrigin {
                    kind: ItemKind::Value,
                    unique_crate_identifier,
                    module_hash: Some(*module_hash),
                }),
            ),
            Descriptor::StringEnum {
                name,
                unique_crate_identifier,
                module_hash,
                ..
            } => f(
                name,
                Some(ItemOrigin {
                    kind: ItemKind::StringEnum,
                    unique_crate_identifier,
                    module_hash: Some(*module_hash),
                }),
            ),
            Descriptor::DynamicUnion {
                name,
                variant_types,
                unique_crate_identifier,
                module_hash,
            } => {
                f(
                    name,
                    Some(ItemOrigin {
                        kind: ItemKind::DynamicUnion,
                        unique_crate_identifier,
                        module_hash: Some(*module_hash),
                    }),
                )?;
                for d in variant_types {
                    d.visit_named_types_mut(f)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn unwrap_function(self) -> Function {
        match self {
            Descriptor::Function(f) => *f,
            _ => panic!("not a function"),
        }
    }

    /// How `mode` fits this import argument: whether it acts, which it does
    /// only on a sequence of its key's shape with a typed-array form, or why it
    /// can't be honoured. Elsewhere an inherited mode has nothing to do, as
    /// does `array` on a sequence that is a fresh `Array` anyway, but a mode
    /// written on the argument is an error, and so is any mode but `view` on
    /// a `&mut` slice, which would discard JS's writes.
    pub fn seq_mode_fit(&self, mode: SeqMode) -> Result<bool, &'static str> {
        let mut d = self;
        while let Descriptor::SeqMode(_, inner) = d {
            d = inner;
        }
        let d = match d {
            Descriptor::Option(inner) | Descriptor::Nullable(inner) => inner,
            d => d,
        };
        let d = match d {
            Descriptor::Nullable(inner) => inner,
            d => d,
        };
        let (borrowed, mutable) = match d {
            Descriptor::Ref(s) => (matches!(**s, Descriptor::Slice(_)), false),
            Descriptor::RefMut(s) => (matches!(**s, Descriptor::Slice(_)), true),
            _ => (false, false),
        };
        let typed = d.vector_kind().is_some_and(|kind| {
            !matches!(
                kind,
                VectorKind::String | VectorKind::Externref | VectorKind::NamedExternref(_)
            )
        });
        let owned = matches!(d, Descriptor::Vector(_));
        let misfit = if mode.vec && borrowed {
            "`vec` applies to owned vectors; for a borrowed `&[T]` use `slice = view | copy | array`"
        } else if mode.vec && !owned {
            "`vec` applies to owned vector arguments (`Vec<T>`, `Box<[T]>`)"
        } else if !mode.vec && owned {
            "`slice` applies to borrowed slices; for an owned `Vec<T>` use `vec = copy | array`"
        } else if !mode.vec && !borrowed {
            "`slice` applies to borrowed slice arguments (`&[T]`)"
        } else if mutable && mode.value != SeqValue::View {
            return Err("JS would receive a copy, whose writes never reach the `&mut [T]`; only `slice = view` applies");
        } else if !typed && mode.value != SeqValue::Array {
            "it has no typed-array form, so it always reaches JS as a fresh `Array`"
        } else {
            return Ok(typed);
        };
        if mode.explicit {
            Err(misfit)
        } else {
            Ok(false)
        }
    }

    pub fn vector_kind(&self) -> Option<VectorKind> {
        let inner = match *self {
            Descriptor::String | Descriptor::CachedString => return Some(VectorKind::String),
            Descriptor::Vector(ref d) => &**d,
            Descriptor::Slice(ref d) => &**d,
            Descriptor::Ref(ref d) => match **d {
                Descriptor::Slice(ref d) => &**d,
                Descriptor::String | Descriptor::CachedString => return Some(VectorKind::String),
                _ => return None,
            },
            Descriptor::RefMut(ref d) => match **d {
                Descriptor::Slice(ref d) => &**d,
                _ => return None,
            },
            _ => return None,
        };
        inner.in_place_kind()
    }

    /// The cell of a sequence whose element is `self`: the element itself for
    /// a number or a JS handle, and its scalar ABI for anything else, which
    /// the binding rejects if it has no glue for it.
    pub fn sequence_cell(&self) -> SequenceCell<'_> {
        match self.in_place_kind() {
            Some(kind) => SequenceCell::InPlace(kind),
            None => SequenceCell::Encoded(self),
        }
    }

    /// The cell of an owned vector whose element is `self`. An optional JS
    /// handle, `Option<T>` or `Nullable<T>` of a string or imported type, is
    /// the one element that differs from a borrowed slice's: owned, the
    /// elements are converted to handles in Rust and cross in place, typed
    /// as the optional element; borrowed, they are encoded.
    pub fn vector_cell(&self, incoming: bool) -> SequenceCell<'_> {
        let (inner, empty) = match self {
            Descriptor::Option(inner) => match &**inner {
                Descriptor::Nullable(inner) => (&**inner, " | null | undefined"),
                // `Option<T>` reads `null` as `None` but writes `None` as
                // `undefined`.
                inner if incoming => (inner, " | null | undefined"),
                inner => (inner, " | undefined"),
            },
            Descriptor::Nullable(inner) => (&**inner, " | null"),
            _ => return self.sequence_cell(),
        };
        match inner {
            Descriptor::Externref => SequenceCell::InPlace(VectorKind::Externref),
            Descriptor::NamedExternref(name) => {
                SequenceCell::InPlace(VectorKind::NamedExternref(format!("{name}{empty}")))
            }
            _ => self.sequence_cell(),
        }
    }

    fn in_place_kind(&self) -> Option<VectorKind> {
        match *self {
            Descriptor::I8 => Some(VectorKind::I8),
            Descriptor::I16 => Some(VectorKind::I16),
            Descriptor::I32 => Some(VectorKind::I32),
            Descriptor::I64 | Descriptor::I64AsF64 => Some(VectorKind::I64),
            Descriptor::U8 => Some(VectorKind::U8),
            Descriptor::ClampedU8 => Some(VectorKind::ClampedU8),
            Descriptor::U16 => Some(VectorKind::U16),
            Descriptor::U32 => Some(VectorKind::U32),
            Descriptor::U64 | Descriptor::U64AsF64 => Some(VectorKind::U64),
            Descriptor::F32 => Some(VectorKind::F32),
            Descriptor::F64 => Some(VectorKind::F64),
            Descriptor::Externref => Some(VectorKind::Externref),
            Descriptor::NamedExternref(ref name) | Descriptor::RustStruct { ref name, .. } => {
                Some(VectorKind::NamedExternref(name.clone()))
            }
            _ => None,
        }
    }
}

fn get(a: &mut &[u32]) -> u32 {
    let ret = a[0];
    *a = &a[1..];
    ret
}

fn get_string(data: &mut &[u32]) -> String {
    (0..get(data))
        .map(|_| char::from_u32(get(data)).unwrap())
        .collect()
}

impl Closure {
    fn decode(data: &mut &[u32]) -> Closure {
        let [owned, mutable] = std::array::from_fn(|_| match get(data) {
            0 => false,
            1 => true,
            other => panic!("expected bool value, got {other}"),
        });
        assert_eq!(get(data), FUNCTION);
        Closure {
            owned,
            mutable,
            function: Function::decode(data),
        }
    }
}

#[test]
fn vector_kind_accepts_memory64_scalar_descriptors() {
    assert_eq!(
        Descriptor::Vector(Box::new(Descriptor::U64AsF64)).vector_kind(),
        Some(VectorKind::U64)
    );
    assert_eq!(
        Descriptor::Ref(Box::new(Descriptor::Slice(Box::new(Descriptor::I64AsF64)))).vector_kind(),
        Some(VectorKind::I64)
    );
}

#[test]
fn sequence_cells() {
    assert_eq!(
        Descriptor::U8.sequence_cell(),
        SequenceCell::InPlace(VectorKind::U8)
    );
    assert_eq!(
        Descriptor::Char.sequence_cell(),
        SequenceCell::Encoded(&Descriptor::Char)
    );
    // A numeric enum's cell is its number, not a JS handle.
    let level = Descriptor::Enum {
        name: "Level".to_string(),
        hole: 0,
        unique_crate_identifier: String::new(),
        module_hash: 0,
    };
    assert_eq!(level.sequence_cell(), SequenceCell::Encoded(&level));
    assert_eq!(Descriptor::decode(&[VECTOR, BOOLEAN]).vector_kind(), None);
    assert_eq!(
        Descriptor::decode(&[REF, SLICE, BOOLEAN]).vector_kind(),
        None
    );
}

#[test]
fn seq_modes_apply_by_shape() {
    let decode = |word, rest: &[u32]| Descriptor::decode(&[&[SEQ_MODE, word], rest].concat());
    let fit = |word, rest: &[u32]| match decode(word, rest) {
        Descriptor::SeqMode(mode, d) => d.seq_mode_fit(mode),
        _ => Ok(false),
    };
    let (copy, array, explicit) = (SEQ_MODE_COPY, SEQ_MODE_ARRAY, SEQ_MODE_EXPLICIT);
    let vec_array = SEQ_MODE_VEC | SEQ_MODE_ARRAY;
    // A mode acts only on its key's shape with a typed element.
    assert_eq!(fit(copy, &[REF, SLICE, U8]), Ok(true));
    assert_eq!(fit(array, &[OPTIONAL, REF, SLICE, F64]), Ok(true));
    assert_eq!(fit(array, &[CLAMPED, REF, SLICE, U8]), Ok(true));
    assert_eq!(fit(vec_array, &[OPTIONAL, VECTOR, U8]), Ok(true));
    assert_eq!(fit(explicit, &[REFMUT, SLICE, U8]), Ok(true));
    // Inherited, it has nothing to do elsewhere, and is dropped.
    assert_eq!(fit(array, &[VECTOR, U8]), Ok(false));
    assert_eq!(fit(vec_array, &[REF, SLICE, U8]), Ok(false));
    assert_eq!(fit(copy, &[REF, SLICE, EXTERNREF]), Ok(false));
    assert_eq!(fit(copy, &[REF, SLICE, BOOLEAN]), Ok(false));
    assert_eq!(fit(copy, &[REF, STRING]), Ok(false));
    assert_eq!(fit(array, &[U32]), Ok(false));
    // A sequence of anything else already arrives as a fresh `Array`.
    assert_eq!(fit(explicit | array, &[REF, SLICE, REF, STRING]), Ok(false));
    // Written on the argument, any other misfit is an error, and on a `&mut`
    // slice so is an inherited mode.
    assert!(fit(explicit | SEQ_MODE_VEC | copy, &[REF, SLICE, U8]).is_err());
    assert!(fit(explicit | array, &[VECTOR, U8]).is_err());
    assert!(fit(explicit, &[REF, SLICE, REF, STRING]).is_err());
    assert!(fit(copy, &[REFMUT, SLICE, U8]).is_err());
    assert!(fit(copy, &[OPTIONAL, REFMUT, SLICE, U8]).is_err());
}

impl Function {
    /// See [`Descriptor::visit_named_types_mut`].
    pub fn visit_named_types_mut<E>(
        &mut self,
        f: &mut impl FnMut(&mut String, Option<ItemOrigin<'_>>) -> Result<(), E>,
    ) -> Result<(), E> {
        for d in &mut self.arguments {
            d.visit_named_types_mut(f)?;
        }
        self.ret.visit_named_types_mut(f)?;
        if let Some(d) = &mut self.inner_ret {
            d.visit_named_types_mut(f)?;
        }
        Ok(())
    }

    fn decode(data: &mut &[u32]) -> Function {
        let shim_idx = get(data);
        let arguments = (0..get(data))
            .map(|_| Descriptor::_decode(data, false))
            .collect::<Vec<_>>();
        Function {
            arguments,
            shim_idx,
            ret: Descriptor::_decode(data, false),
            inner_ret: Some(Descriptor::_decode(data, false)),
        }
    }
}

impl VectorKind {
    pub fn js_ty(&self) -> String {
        match *self {
            VectorKind::String => "string".to_string(),
            VectorKind::I8 => "Int8Array".to_string(),
            VectorKind::U8 => "Uint8Array".to_string(),
            VectorKind::ClampedU8 => "Uint8ClampedArray".to_string(),
            VectorKind::I16 => "Int16Array".to_string(),
            VectorKind::U16 => "Uint16Array".to_string(),
            VectorKind::I32 => "Int32Array".to_string(),
            VectorKind::U32 => "Uint32Array".to_string(),
            VectorKind::I64 => "BigInt64Array".to_string(),
            VectorKind::U64 => "BigUint64Array".to_string(),
            VectorKind::F32 => "Float32Array".to_string(),
            VectorKind::F64 => "Float64Array".to_string(),
            VectorKind::Externref => "any[]".to_string(),
            VectorKind::NamedExternref(ref name) => {
                if is_valid_ident(name.as_str()) {
                    format!("{name}[]")
                } else {
                    format!("({name})[]")
                }
            }
        }
    }

    /// The element type of a plain JS array that can be passed instead of the
    /// typed array from `js_ty` when the vector is copied into Wasm memory.
    /// The copy uses `TypedArray.prototype.set`, which converts each element
    /// the same way a typed array store does.
    pub fn plain_array_element_ty(&self) -> Option<&'static str> {
        match *self {
            VectorKind::I8
            | VectorKind::U8
            | VectorKind::I16
            | VectorKind::U16
            | VectorKind::I32
            | VectorKind::U32
            | VectorKind::F32
            | VectorKind::F64 => Some("number"),
            VectorKind::I64 | VectorKind::U64 => Some("bigint"),
            // Copied through a `Uint8Array` view, so numbers in a plain array
            // would wrap around instead of being clamped.
            VectorKind::ClampedU8 => None,
            // Not typed arrays.
            VectorKind::String | VectorKind::Externref | VectorKind::NamedExternref(_) => None,
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            VectorKind::String => 1,
            VectorKind::I8 => 1,
            VectorKind::U8 => 1,
            VectorKind::ClampedU8 => 1,
            VectorKind::I16 => 2,
            VectorKind::U16 => 2,
            VectorKind::I32 => 4,
            VectorKind::U32 => 4,
            VectorKind::I64 => 8,
            VectorKind::U64 => 8,
            VectorKind::F32 => 4,
            VectorKind::F64 => 8,
            VectorKind::Externref => 4,
            VectorKind::NamedExternref(_) => 4,
        }
    }
}
