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
    SEQ_MODE
    NULLABLE
}

/// The word following a `SEQ_MODE` tag: what JS receives, ORed with flags.
pub const SEQ_MODE_VIEW: u32 = 0;
pub const SEQ_MODE_COPY: u32 = 1;
pub const SEQ_MODE_ARRAY: u32 = 2;
/// A `vec = ..` mode, for owned sequences, rather than `slice = ..`.
pub const SEQ_MODE_VEC: u32 = 1 << 2;
/// Written on the argument rather than inherited, so the CLI validates it.
pub const SEQ_MODE_EXPLICIT: u32 = 1 << 3;
