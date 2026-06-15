use std::char;
use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use wasm_bindgen_shared::identifier::is_valid_ident;
use wasm_bindgen_shared::tys::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    },
    StringEnum {
        name: String,
        invalid: u32,
        hole: u32,
    },
    DynamicUnion {
        name: String,
        variant_types: Vec<Descriptor>,
    },
    RustStruct(String),
    Char,
    Option(Box<Descriptor>),
    Result(Box<Descriptor>),
    Unit,
    NonNull,
    RawPointer,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub arguments: Vec<Descriptor>,
    pub shim_idx: u32,
    pub ret: Descriptor,
    pub inner_ret: Option<Descriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// Decode a schema `u32` stream into a [`Descriptor`].
    ///
    /// Used for streams that contain no unresolved `SYMBOL_REF` slots (e.g.
    /// the cast path, where the closure invoke index has already been patched
    /// in). Panics on malformed input, matching the historical contract.
    pub fn decode(data: &[u32]) -> Descriptor {
        Descriptor::decode_with_symbols(data, None)
            .unwrap_or_else(|e| panic!("malformed descriptor stream: {e}"))
    }

    /// Decode a schema `u32` stream, resolving any `SYMBOL_REF` slot against
    /// `symbols` (a `name -> value` map, typically a function-table index).
    ///
    /// This is the single owner of the descriptor grammar. The
    /// `__wasm_bindgen_descriptors` section path calls this so that symbol
    /// resolution and decoding share one traversal — there is no separate
    /// grammar walk to drift out of sync. `SYMBOL_REF` (`0xFF`) is only
    /// honored where the grammar expects it (a `FUNCTION`'s `shim_idx`
    /// slot); any `0xFF` that occurs as literal data (a name codepoint, an
    /// enum hole/variant count, ...) is consumed as data, never as a symbol
    /// reference.
    ///
    /// Returns an error (rather than panicking) on the conditions a section
    /// produced by a mismatched toolchain can realistically hit: an
    /// unresolved symbol, a truncated stream, a bad opcode, or trailing data.
    pub fn decode_with_symbols(
        data: &[u32],
        symbols: Option<&HashMap<String, u32>>,
    ) -> Result<Descriptor> {
        let mut d = Decoder {
            data,
            pos: 0,
            symbols,
        };
        let descriptor = d.descriptor(false)?;
        if d.pos != data.len() {
            bail!("remaining data: {} word(s)", data.len() - d.pos);
        }
        Ok(descriptor)
    }

    pub fn unwrap_function(self) -> Function {
        match self {
            Descriptor::Function(f) => *f,
            _ => panic!("not a function"),
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
        match *inner {
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
            Descriptor::NamedExternref(ref name) => Some(VectorKind::NamedExternref(name.clone())),
            _ => None,
        }
    }
}

/// Cursor over a schema `u32` stream that owns the descriptor grammar.
/// Both the symbol-resolving section path and the symbol-free cast path go
/// through this, so the grammar exists in exactly one place.
struct Decoder<'a> {
    data: &'a [u32],
    pos: usize,
    /// `name -> value` table for resolving `SYMBOL_REF` slots, or `None` when
    /// the caller guarantees the stream contains no unresolved references.
    symbols: Option<&'a HashMap<String, u32>>,
}

impl Decoder<'_> {
    /// Consume and return the next word.
    fn next(&mut self) -> Result<u32> {
        let word = self
            .data
            .get(self.pos)
            .copied()
            .ok_or_else(|| anyhow!("unexpected end of descriptor stream at word {}", self.pos))?;
        self.pos += 1;
        Ok(word)
    }

    /// Peek at the next word without consuming it.
    fn peek(&self) -> Result<u32> {
        self.data
            .get(self.pos)
            .copied()
            .ok_or_else(|| anyhow!("unexpected end of descriptor stream at word {}", self.pos))
    }

    /// Decode a `get_string`-style payload: a length word followed by that
    /// many `char` codepoint words.
    fn string(&mut self) -> Result<String> {
        let len = self.next()?;
        (0..len)
            .map(|_| {
                let cp = self.next()?;
                char::from_u32(cp)
                    .ok_or_else(|| anyhow!("invalid char codepoint {cp:#x} in descriptor"))
            })
            .collect()
    }

    fn boolean(&mut self) -> Result<bool> {
        match self.next()? {
            0 => Ok(false),
            1 => Ok(true),
            other => bail!("expected bool value, got {other}"),
        }
    }

    /// Decode the `shim_idx` slot of a `FUNCTION`. This is the *only* place a
    /// `SYMBOL_REF` may appear; a plain literal (e.g. `0`) is returned as-is.
    fn shim_idx(&mut self) -> Result<u32> {
        if self.peek()? != SYMBOL_REF {
            return self.next();
        }
        // `SYMBOL_REF, name_len, <name packed LE into ceil(len/4) zero-padded
        // words>` (see `wasm_bindgen_shared::tys::SYMBOL_REF`).
        self.next()?; // SYMBOL_REF
        let name_len = self.next()? as usize;
        if name_len == 0 {
            bail!("SYMBOL_REF entry has empty name");
        }
        let word_count = name_len.div_ceil(4);
        let mut name_bytes = Vec::with_capacity(word_count * 4);
        for _ in 0..word_count {
            name_bytes.extend_from_slice(&self.next()?.to_le_bytes());
        }
        if name_bytes[name_len..].iter().any(|&b| b != 0) {
            bail!("SYMBOL_REF padding bytes are not zero");
        }
        name_bytes.truncate(name_len);
        let name = std::str::from_utf8(&name_bytes)
            .map_err(|e| anyhow!("SYMBOL_REF name is not UTF-8: {e}"))?;
        let symbols = self.symbols.ok_or_else(|| {
            anyhow!("descriptor contains a SYMBOL_REF ({name:?}) but no symbol table was provided")
        })?;
        symbols
            .get(name)
            .copied()
            .ok_or_else(|| anyhow!("SYMBOL_REF target {name:?} was not found in the wasm module"))
    }

    fn function(&mut self) -> Result<Function> {
        let shim_idx = self.shim_idx()?;
        let nargs = self.next()?;
        let arguments = (0..nargs)
            .map(|_| self.descriptor(false))
            .collect::<Result<Vec<_>>>()?;
        let ret = self.descriptor(false)?;
        let inner_ret = Some(self.descriptor(false)?);
        Ok(Function {
            arguments,
            shim_idx,
            ret,
            inner_ret,
        })
    }

    fn closure(&mut self) -> Result<Closure> {
        let owned = self.boolean()?;
        let mutable = self.boolean()?;
        let inner = self.next()?;
        if inner != FUNCTION {
            bail!("expected FUNCTION opcode in closure body, got {inner}");
        }
        Ok(Closure {
            owned,
            mutable,
            function: self.function()?,
        })
    }

    fn descriptor(&mut self, clamped: bool) -> Result<Descriptor> {
        Ok(match self.next()? {
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
            FUNCTION => Descriptor::Function(Box::new(self.function()?)),
            CLOSURE => Descriptor::Closure(Box::new(self.closure()?)),
            REF => Descriptor::Ref(Box::new(self.descriptor(clamped)?)),
            REFMUT => Descriptor::RefMut(Box::new(self.descriptor(clamped)?)),
            LONGREF => {
                // This descriptor basically just serves as a macro, where most things
                // become normal `Ref`s, but long refs to externrefs become owned.
                let contents = self.descriptor(clamped)?;
                match contents {
                    Descriptor::Externref | Descriptor::NamedExternref(_) => contents,
                    _ => Descriptor::Ref(Box::new(contents)),
                }
            }
            SLICE => Descriptor::Slice(Box::new(self.descriptor(clamped)?)),
            VECTOR => Descriptor::Vector(Box::new(self.descriptor(clamped)?)),
            OPTIONAL => Descriptor::Option(Box::new(self.descriptor(clamped)?)),
            RESULT => Descriptor::Result(Box::new(self.descriptor(clamped)?)),
            CACHED_STRING => Descriptor::CachedString,
            STRING => Descriptor::String,
            EXTERNREF => Descriptor::Externref,
            ENUM => {
                let name = self.string()?;
                let hole = self.next()?;
                Descriptor::Enum { name, hole }
            }
            STRING_ENUM => {
                let name = self.string()?;
                let variant_count = self.next()?;
                let invalid = variant_count;
                let hole = variant_count + 1;
                Descriptor::StringEnum {
                    name,
                    invalid,
                    hole,
                }
            }
            DYNAMIC_UNION => {
                let name = self.string()?;
                let type_count = self.next()?;
                let mut variant_types = Vec::new();
                for _ in 0..type_count {
                    variant_types.push(self.descriptor(clamped)?);
                }
                Descriptor::DynamicUnion {
                    name,
                    variant_types,
                }
            }
            RUST_STRUCT => Descriptor::RustStruct(self.string()?),
            NAMED_EXTERNREF => Descriptor::NamedExternref(self.string()?),
            CHAR => Descriptor::Char,
            UNIT => Descriptor::Unit,
            CLAMPED => self.descriptor(true)?,
            NONNULL => Descriptor::NonNull,
            RAW_POINTER => Descriptor::RawPointer,
            other => bail!("unknown descriptor: {other}"),
        })
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

#[cfg(test)]
mod decode_tests {
    use super::*;

    /// Build the inline `SYMBOL_REF` payload exactly as the macro emits it:
    /// `SYMBOL_REF, name_len, <name packed LE into ceil(len/4) zero-padded
    /// words>`.
    fn sym_words(name: &str) -> Vec<u32> {
        let b = name.as_bytes();
        let mut v = vec![SYMBOL_REF, b.len() as u32];
        for chunk in b.chunks(4) {
            let mut buf = [0u8; 4];
            buf[..chunk.len()].copy_from_slice(chunk);
            v.push(u32::from_le_bytes(buf));
        }
        v
    }

    fn syms(pairs: &[(&str, u32)]) -> HashMap<String, u32> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn resolves_symbol_ref_in_shim_idx_slot() {
        // FUNCTION whose shim_idx is a SYMBOL_REF "shim", 0 args, ret/inner I32.
        let mut words = vec![FUNCTION];
        words.extend(sym_words("shim"));
        words.extend([0, I32, I32]);
        let map = syms(&[("shim", 17)]);
        let func = Descriptor::decode_with_symbols(&words, Some(&map))
            .unwrap()
            .unwrap_function();
        assert_eq!(func.shim_idx, 17);
        assert!(func.arguments.is_empty());
        assert_eq!(func.ret, Descriptor::I32);
        assert_eq!(func.inner_ret, Some(Descriptor::I32));
    }

    #[test]
    fn resolves_symbol_ref_with_padding() {
        // 5-byte name needs zero padding across two packed words.
        let mut words = vec![FUNCTION];
        words.extend(sym_words("abcde"));
        words.extend([0, I32, I32]);
        let map = syms(&[("abcde", 99)]);
        let func = Descriptor::decode_with_symbols(&words, Some(&map))
            .unwrap()
            .unwrap_function();
        assert_eq!(func.shim_idx, 99);
    }

    #[test]
    fn unresolved_symbol_is_an_error() {
        let mut words = vec![FUNCTION];
        words.extend(sym_words("shim"));
        words.extend([0, I32, I32]);
        let err = Descriptor::decode_with_symbols(&words, Some(&HashMap::new())).unwrap_err();
        assert!(err.to_string().contains("shim"), "got: {err}");
    }

    #[test]
    fn symbol_ref_without_symbol_table_is_an_error() {
        let mut words = vec![FUNCTION];
        words.extend(sym_words("shim"));
        words.extend([0, I32, I32]);
        let err = Descriptor::decode_with_symbols(&words, None).unwrap_err();
        assert!(err.to_string().contains("no symbol table"), "got: {err}");
    }

    // --- Regression tests for the SYMBOL_REF (`0xFF`) vs literal-data
    // --- collision (PLAN.md issue 1). SYMBOL_REF must only be honored at a
    // --- FUNCTION's `shim_idx` slot; a `0xFF` occurring as literal data
    // --- (a name codepoint, an enum hole/variant count, ...) is data.

    #[test]
    fn enum_name_codepoint_0xff_is_not_a_symbol_ref() {
        // ENUM named "ÿ" (U+00FF) with hole 0. The codepoint word equals
        // SYMBOL_REF (0xFF) but is decoded as data.
        assert_eq!(SYMBOL_REF, 0xFF);
        let words = [ENUM, 1, 0xFF, 0];
        assert_eq!(
            Descriptor::decode_with_symbols(&words, None).unwrap(),
            Descriptor::Enum {
                name: "ÿ".to_string(),
                hole: 0,
            }
        );
    }

    #[test]
    fn enum_hole_255_is_not_a_symbol_ref() {
        let words = [ENUM, 1, 'A' as u32, 0xFF];
        assert_eq!(
            Descriptor::decode_with_symbols(&words, None).unwrap(),
            Descriptor::Enum {
                name: "A".to_string(),
                hole: 255,
            }
        );
    }

    #[test]
    fn string_enum_variant_count_255_is_not_a_symbol_ref() {
        // Trailing word == 0xFF; a naive scanner would also run off the end.
        let words = [STRING_ENUM, 1, 'A' as u32, 0xFF];
        assert_eq!(
            Descriptor::decode_with_symbols(&words, None).unwrap(),
            Descriptor::StringEnum {
                name: "A".to_string(),
                invalid: 255,
                hole: 256,
            }
        );
    }

    #[test]
    fn symbol_ref_resolved_alongside_colliding_data() {
        // Torture case: a real SYMBOL_REF shim_idx AND an argument that is an
        // ENUM named "ÿ" (codepoint 0xFF) with hole 255 (also 0xFF). Only the
        // shim_idx is resolved; the data 0xFFs survive.
        let mut words = vec![FUNCTION];
        words.extend(sym_words("cb"));
        words.push(1); // nargs
        words.extend([ENUM, 1, 0xFF, 0xFF]); // arg: enum "ÿ", hole 255
        words.extend([I32, I32]); // ret, inner_ret
        let map = syms(&[("cb", 7)]);
        let func = Descriptor::decode_with_symbols(&words, Some(&map))
            .unwrap()
            .unwrap_function();
        assert_eq!(func.shim_idx, 7);
        assert_eq!(
            func.arguments,
            vec![Descriptor::Enum {
                name: "ÿ".to_string(),
                hole: 255,
            }]
        );
        assert_eq!(func.ret, Descriptor::I32);
        assert_eq!(func.inner_ret, Some(Descriptor::I32));
    }

    #[test]
    fn trailing_words_after_complete_descriptor_are_rejected() {
        let words = [I32, I32];
        let err = Descriptor::decode_with_symbols(&words, None).unwrap_err();
        assert!(err.to_string().contains("remaining data"), "got: {err}");
    }

    #[test]
    fn unknown_opcode_is_rejected() {
        let err = Descriptor::decode_with_symbols(&[9999], None).unwrap_err();
        assert!(err.to_string().contains("unknown descriptor"), "got: {err}");
    }

    #[test]
    fn truncated_stream_is_rejected() {
        // FUNCTION header that promises an inner_ret it never provides.
        let words = [FUNCTION, 0, 0, I32];
        let err = Descriptor::decode_with_symbols(&words, None).unwrap_err();
        assert!(err.to_string().contains("unexpected end"), "got: {err}");
    }
}
