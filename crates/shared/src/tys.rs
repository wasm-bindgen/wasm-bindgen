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

/// Structural tags for the reference-based `Schema` tree (see
/// `wasm_bindgen::describe::Schema`). These label the *shape* of a `Schema`
/// node and are shared by the producer (runtime composition) and the consumer
/// (`wasm-bindgen-cli-support`, which walks the tree out of a cast record's
/// data segment). They are a separate namespace from the schema *opcodes*
/// above: tags live in the `Schema::tag` field, never in the flat opcode
/// stream that `Descriptor::decode` consumes.
///
/// Flattening is uniform regardless of tag — a node contributes its `words`
/// run followed by the flattened streams of its `children`, in order — so the
/// tags are primarily documentation/validation aids:
///
/// * `SCHEMA_NODE_LEAF` — `words` only, no children (e.g. `i32`, `JsValue`).
/// * `SCHEMA_NODE_WRAP` — leading `words` plus one or more children (e.g.
///   `Option<T>`, `&T`, the closure trait-object header + arg/ret schemas).
/// * `SCHEMA_NODE_CAT` — no `words`, children only (pure concatenation).
pub const SCHEMA_NODE_LEAF: u32 = 0;
pub const SCHEMA_NODE_WRAP: u32 = 1;
pub const SCHEMA_NODE_CAT: u32 = 2;
