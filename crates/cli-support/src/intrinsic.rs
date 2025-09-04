//! Definition of all wasm-bindgen intrinsics.
//!
//! This contains a definition of all intrinsics used by `src/lib.rs` in the
//! wasm-bindgen crate. Each intrinsic listed here is part of an `enum
//! Intrinsic` and is generated through a macro to reduce repetition.
//!
//! Intrinsics in this module currently largely contain their expected symbol
//! name as well as the signature of the function that it expects.

use crate::descriptor::{self, Descriptor, Function};

macro_rules! intrinsics {
    (pub enum Intrinsic {
        $(
            #[symbol = $sym:tt]
            #[signature = fn($($arg:expr),*) -> $ret:expr]
            $name:ident,
        )*
    }) => {
        /// All wasm-bindgen intrinsics that could be depended on by a wasm
        /// module.
        #[derive(Debug)]
        pub enum Intrinsic {
            $($name,)*
        }

        impl Intrinsic {
            /// Returns the corresponding intrinsic for a symbol name, if one
            /// matches.
            pub fn from_symbol(symbol: &str) -> Option<Intrinsic> {
                match symbol {
                    $($sym => Some(Intrinsic::$name),)*
                    _ => None,
                }
            }

            /// Returns the expected signature of this intrinsic, used for
            /// generating a JS shim.
            pub fn signature(&self) -> Function {
                use crate::descriptor::Descriptor::*;
                match self {
                    $(
                        Intrinsic::$name => {
                            descriptor::Function {
                                shim_idx: 0,
                                arguments: vec![$($arg),*],
                                ret: $ret,
                                inner_ret: None
                            }
                        }
                    )*
                }
            }
        }
    };
}

fn ref_externref() -> Descriptor {
    Descriptor::Ref(Box::new(Descriptor::Externref))
}

fn ref_string() -> Descriptor {
    Descriptor::Ref(Box::new(Descriptor::String))
}

fn opt_string() -> Descriptor {
    Descriptor::Option(Box::new(Descriptor::String))
}

fn opt_f64() -> Descriptor {
    Descriptor::Option(Box::new(Descriptor::F64))
}

fn opt_i64() -> Descriptor {
    Descriptor::Option(Box::new(Descriptor::I64))
}

fn slice(contents: Descriptor) -> Descriptor {
    Descriptor::Ref(Box::new(Descriptor::Slice(Box::new(contents))))
}

intrinsics! {
    pub enum Intrinsic {
        #[symbol = "__wbindgen_jsval_eq"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        JsvalEq,
        #[symbol = "__wbindgen_jsval_loose_eq"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        JsvalLooseEq,
        #[symbol = "__wbindgen_is_function"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsFunction,
        #[symbol = "__wbindgen_is_undefined"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsUndefined,
        #[symbol = "__wbindgen_is_null"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsNull,
        #[symbol = "__wbindgen_is_object"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsObject,
        #[symbol = "__wbindgen_is_symbol"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsSymbol,
        #[symbol = "__wbindgen_is_string"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsString,
        #[symbol = "__wbindgen_is_bigint"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsBigInt,
        #[symbol = "__wbindgen_typeof"]
        #[signature = fn(ref_externref()) -> Externref]
        Typeof,
        #[symbol = "__wbindgen_in"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        In,
        #[symbol = "__wbindgen_is_falsy"]
        #[signature = fn(ref_externref()) -> Boolean]
        IsFalsy,
        #[symbol = "__wbindgen_try_into_number"]
        #[signature = fn(ref_externref()) -> Externref]
        TryIntoNumber,
        #[symbol = "__wbindgen_neg"]
        #[signature = fn(ref_externref()) -> Externref]
        Neg,
        #[symbol = "__wbindgen_bit_and"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        BitAnd,
        #[symbol = "__wbindgen_bit_or"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        BitOr,
        #[symbol = "__wbindgen_bit_xor"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        BitXor,
        #[symbol = "__wbindgen_bit_not"]
        #[signature = fn(ref_externref()) -> Externref]
        BitNot,
        #[symbol = "__wbindgen_shl"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Shl,
        #[symbol = "__wbindgen_shr"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Shr,
        #[symbol = "__wbindgen_unsigned_shr"]
        #[signature = fn(ref_externref(), ref_externref()) -> U32]
        UnsignedShr,
        #[symbol = "__wbindgen_add"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Add,
        #[symbol = "__wbindgen_sub"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Sub,
        #[symbol = "__wbindgen_div"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Div,
        #[symbol = "__wbindgen_checked_div"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        CheckedDiv,
        #[symbol = "__wbindgen_mul"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Mul,
        #[symbol = "__wbindgen_rem"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Rem,
        #[symbol = "__wbindgen_pow"]
        #[signature = fn(ref_externref(), ref_externref()) -> Externref]
        Pow,
        #[symbol = "__wbindgen_lt"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        LT,
        #[symbol = "__wbindgen_le"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        LE,
        #[symbol = "__wbindgen_ge"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        GE,
        #[symbol = "__wbindgen_gt"]
        #[signature = fn(ref_externref(), ref_externref()) -> Boolean]
        GT,
        #[symbol = "__wbindgen_object_clone_ref"]
        #[signature = fn(ref_externref()) -> Externref]
        ObjectCloneRef,
        #[symbol = "__wbindgen_object_drop_ref"]
        #[signature = fn(Externref) -> Unit]
        ObjectDropRef,
        #[symbol = "__wbindgen_cb_drop"]
        #[signature = fn(Externref) -> Boolean]
        CallbackDrop,
        #[symbol = "__wbindgen_bigint_get_as_i64"]
        #[signature = fn(ref_externref()) -> opt_i64()]
        BigIntGetAsI64,
        #[symbol = "__wbindgen_number_get"]
        #[signature = fn(ref_externref()) -> opt_f64()]
        NumberGet,
        #[symbol = "__wbindgen_string_get"]
        #[signature = fn(ref_externref()) -> opt_string()]
        StringGet,
        #[symbol = "__wbindgen_boolean_get"]
        #[signature = fn(ref_externref()) -> I32]
        BooleanGet,
        #[symbol = "__wbindgen_throw"]
        #[signature = fn(ref_string()) -> Unit]
        Throw,
        #[symbol = "__wbindgen_rethrow"]
        #[signature = fn(Externref) -> Unit]
        Rethrow,
        #[symbol = "__wbindgen_memory"]
        #[signature = fn() -> Externref]
        Memory,
        #[symbol = "__wbindgen_exports"]
        #[signature = fn() -> Externref]
        Exports,
        #[symbol = "__wbindgen_module"]
        #[signature = fn() -> Externref]
        Module,
        #[symbol = "__wbindgen_function_table"]
        #[signature = fn() -> Externref]
        FunctionTable,
        #[symbol = "__wbindgen_debug_string"]
        #[signature = fn(ref_externref()) -> String]
        DebugString,
        #[symbol = "__wbindgen_copy_to_typed_array"]
        #[signature = fn(slice(U8), ref_externref()) -> Unit]
        CopyToTypedArray,
        #[symbol = "__wbindgen_externref_heap_live_count"]
        #[signature = fn() -> I32]
        ExternrefHeapLiveCount,
        #[symbol = "__wbindgen_init_externref_table"]
        #[signature = fn() -> Unit]
        InitExternrefTable,
    }
}
