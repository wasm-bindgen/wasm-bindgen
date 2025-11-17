//! Definition of all wasm-bindgen intrinsics.
//!
//! This contains a definition of all intrinsics used by `src/lib.rs` in the
//! wasm-bindgen crate. Each intrinsic listed here is part of an `enum
//! Intrinsic` and is generated through a macro to reduce repetition.
//!
//! Intrinsics in this module currently largely contain their expected symbol
//! name as well as the signature of the function that it expects.

macro_rules! intrinsics {
    (pub enum Intrinsic {
        $(
            $name:ident = $sym:literal,
        )*
    }) => {
        /// All wasm-bindgen intrinsics that could be depended on by a wasm
        /// module.
        #[derive(Debug)]
        pub enum Intrinsic {
            $($name,)*
        }

        impl std::str::FromStr for Intrinsic {
            type Err = anyhow::Error;

            /// Returns the corresponding intrinsic for a symbol name, if one
            /// matches.
            fn from_str(symbol: &str) -> anyhow::Result<Intrinsic> {
                Ok(match symbol {
                    $($sym => Intrinsic::$name,)*
                    _ => anyhow::bail!("unknown intrinsic `{symbol}`"),
                })
            }
        }
    };
}

intrinsics! {
    pub enum Intrinsic {
        JsvalEq = "__wbindgen_jsval_eq",
        JsvalLooseEq = "__wbindgen_jsval_loose_eq",
        IsFunction = "__wbindgen_is_function",
        IsUndefined = "__wbindgen_is_undefined",
        IsNull = "__wbindgen_is_null",
        ObjectIsNullOrUndefined = "__wbindgen_object_is_null_or_undefined",
        ObjectIsUndefined = "__wbindgen_object_is_undefined",
        IsObject = "__wbindgen_is_object",
        IsSymbol = "__wbindgen_is_symbol",
        IsString = "__wbindgen_is_string",
        IsBigInt = "__wbindgen_is_bigint",
        Typeof = "__wbindgen_typeof",
        In = "__wbindgen_in",
        IsFalsy = "__wbindgen_is_falsy",
        TryIntoNumber = "__wbindgen_try_into_number",
        Neg = "__wbindgen_neg",
        BitAnd = "__wbindgen_bit_and",
        BitOr = "__wbindgen_bit_or",
        BitXor = "__wbindgen_bit_xor",
        BitNot = "__wbindgen_bit_not",
        Shl = "__wbindgen_shl",
        Shr = "__wbindgen_shr",
        UnsignedShr = "__wbindgen_unsigned_shr",
        Add = "__wbindgen_add",
        Sub = "__wbindgen_sub",
        Div = "__wbindgen_div",
        CheckedDiv = "__wbindgen_checked_div",
        Mul = "__wbindgen_mul",
        Rem = "__wbindgen_rem",
        Pow = "__wbindgen_pow",
        LT = "__wbindgen_lt",
        LE = "__wbindgen_le",
        GE = "__wbindgen_ge",
        GT = "__wbindgen_gt",
        ObjectCloneRef = "__wbindgen_object_clone_ref",
        ObjectDropRef = "__wbindgen_object_drop_ref",
        BigIntGetAsI64 = "__wbindgen_bigint_get_as_i64",
        NumberGet = "__wbindgen_number_get",
        StringGet = "__wbindgen_string_get",
        BooleanGet = "__wbindgen_boolean_get",
        Throw = "__wbindgen_throw",
        Rethrow = "__wbindgen_rethrow",
        Memory = "__wbindgen_memory",
        Exports = "__wbindgen_exports",
        Module = "__wbindgen_module",
        FunctionTable = "__wbindgen_function_table",
        DebugString = "__wbindgen_debug_string",
        CopyToTypedArray = "__wbindgen_copy_to_typed_array",
        ExternrefHeapLiveCount = "__wbindgen_externref_heap_live_count",
        InitExternrefTable = "__wbindgen_init_externref_table",
        PanicError = "__wbindgen_panic_error",
    }
}
