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

/// Structural tags for the reference-based `Schema` tree (see
/// `wasm_bindgen::describe::Schema`). These label the *shape* of a `Schema`
/// node and are shared by the producer (runtime composition) and the consumer
/// (`wasm-bindgen-cli-support`, which walks the tree out of a descriptor
/// record's data segment). Tags live in the `Schema::tag` field, alongside
/// the schema *opcodes* above (which live in a node's `words` run):
///
/// * `Leaf` — `words` only, no children (e.g. `i32`, `JsValue`).
/// * `Wrap` — leading `words` plus zero or more children (e.g.
///   `Option<T>`, `&T`, the closure trait-object header + arg/ret schemas).
///   An empty `words` run is permitted, so `Wrap` also covers a pure
///   concatenation of children.
///
/// The CLI decodes a node by reading scalar opcodes/operands from `words`
/// and recursing into `children` for sub-descriptors; the tag is a
/// validation/documentation aid.
///
/// `#[repr(u32)]` pins the discriminants so the `Schema::tag` field keeps the
/// exact 4-byte layout the previous bare-`u32` constants produced; the
/// CLI-side `#[repr(C)] Schema` parser is unaffected.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SchemaTag {
    /// `words` only, no children (e.g. `i32`, `JsValue`).
    Leaf = 0,
    /// Leading `words` plus zero or more children (e.g. `Option<T>`, `&T`,
    /// the closure trait-object header + arg/ret schemas). An empty `words`
    /// run is permitted, so this also covers a pure concatenation of
    /// children.
    Wrap = 1,
}
