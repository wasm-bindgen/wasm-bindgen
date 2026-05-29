macro_rules! tys {
    ($($a:ident)*) => (tys! { @ ($($a)*) 0 });
    (@ () $v:expr) => {};
    (@ ($a:ident $($b:ident)*) $v:expr) => {
        pub const $a: u32 = $v;
        tys!(@ ($($b)*) $v+1);
    }
}

tys! {
    I8
    U8
    I16
    U16
    I32
    U32
    I64
    U64
    I64_AS_F64
    U64_AS_F64
    I128
    U128
    F32
    F64
    BOOLEAN
    FUNCTION
    CLOSURE
    CACHED_STRING
    STRING
    REF
    REFMUT
    LONGREF
    SLICE
    VECTOR
    EXTERNREF
    NAMED_EXTERNREF
    ENUM
    STRING_ENUM
    DYNAMIC_UNION
    RUST_STRUCT
    CHAR
    OPTIONAL
    RESULT
    UNIT
    CLAMPED
    NONNULL
    RAW_POINTER
}

/// Special opcode used in the `__wasm_bindgen_descriptors` custom section to
/// indicate that the next `u32` in the schema stream is not a literal but a
/// reference to a symbol whose linker-assigned value (typically a function
/// table index) must be resolved by `wasm-bindgen-cli-support` after linking.
///
/// Stream encoding when this opcode appears:
///
/// ```text
/// SYMBOL_REF (u32)
/// name_len   (u32)        // length in bytes of the UTF-8 symbol name
/// name_bytes ([u8; n], padded to 4-byte alignment with zeros)
/// ```
///
/// On decode, `cli-support` looks up `name_bytes` in the wasm module's
/// exports (and name section as a fallback) to find the function it refers
/// to, resolves it to a function-table index, and substitutes that index
/// into the stream as the next `u32`. The remainder of the stream is then
/// interpreted unchanged by `Descriptor::decode`.
///
/// Chosen as `0xFF` to leave plenty of headroom above the existing dense
/// opcode range (currently `0..=36`) for future low-numbered additions.
pub const SYMBOL_REF: u32 = 0xFF;

/// Special opcode used in a `#[wasm_bindgen(generic)]` import's signature
/// *template* to mark a generic-parameter hole. The next `u32` in the
/// stream is the zero-based index of the type parameter occupying this
/// position.
///
/// Stream encoding:
///
/// ```text
/// TYPE_PARAM (u32)
/// param_idx  (u32)        // zero-based generic type-parameter index
/// ```
///
/// At link time `wasm-bindgen-cli-support` recovers, per monomorphisation,
/// the concrete `WasmDescribe` schema ("fill") of each type parameter from
/// the call-site courier, and splices `fill[param_idx]` in place of the
/// `TYPE_PARAM param_idx` pair, producing the concrete function descriptor.
/// A parameter may appear in multiple positions (multiple args and/or the
/// return); every occurrence with the same index is filled from the same
/// fill, so the per-monomorphisation courier carries each type's schema
/// only once.
pub const TYPE_PARAM: u32 = 0xFE;
