use std::borrow::Cow;

use crate::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use proc_macro2::{Ident, Span};
use weedle::attribute::{ExtendedAttribute, ExtendedAttributeList};
use weedle::common::Identifier;
use weedle::term;
use weedle::types::*;

use crate::first_pass::FirstPassRecord;
use crate::util::{array, camel_case_ident, option_ty, shared_ref, snake_case_ident, TypePosition};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub(crate) enum IdlType<'a> {
    Boolean,
    Byte,
    Octet,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    UnrestrictedFloat,
    Double,
    UnrestrictedDouble,
    DomString,
    ByteString,
    UsvString,
    Object,
    Symbol,
    Error,

    ArrayBuffer,
    DataView {
        allow_shared: bool,
    },
    Int8Array {
        allow_shared: bool,
        immutable: bool,
    },
    Uint8Array {
        allow_shared: bool,
        immutable: bool,
    },
    Uint8ClampedArray {
        allow_shared: bool,
        immutable: bool,
    },
    Int16Array {
        allow_shared: bool,
        immutable: bool,
    },
    Uint16Array {
        allow_shared: bool,
        immutable: bool,
    },
    Int32Array {
        allow_shared: bool,
        immutable: bool,
    },
    Uint32Array {
        allow_shared: bool,
        immutable: bool,
    },
    Float32Array {
        allow_shared: bool,
        immutable: bool,
    },
    Float64Array {
        allow_shared: bool,
        immutable: bool,
    },
    ArrayBufferView {
        allow_shared: bool,
        immutable: bool,
    },
    BufferSource {
        allow_shared: bool,
        immutable: bool,
    },

    Nullable(Box<IdlType<'a>>),
    FrozenArray(Box<IdlType<'a>>),
    ObservableArray(Box<IdlType<'a>>),
    Sequence(Box<IdlType<'a>>),
    Promise(Box<IdlType<'a>>),
    Record(Box<IdlType<'a>>, Box<IdlType<'a>>),
    Union(Vec<IdlType<'a>>),

    Any,
    Undefined,

    UnknownIdentifier(&'a str),

    Identifier {
        name: &'a str,
        ty: IdentifierType<'a>,
    },
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub(crate) enum IdentifierType<'a> {
    Callback,
    Iterator,
    AsyncIterator,
    Interface(&'a str),
    Dictionary(&'a str),
    Enum(&'a str),
    CallbackInterface {
        name: &'a str,
        single_function: bool,
    },
    // DOMTimeStamp
    UnsignedLongLong,
    // AllowSharedBufferSource
    AllowSharedBufferSource {
        immutable: bool,
    },
    Int8Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Uint8Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Uint8ClampedSlice {
        allow_shared: bool,
        immutable: bool,
    },
    Int16Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Uint16Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Int32Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Uint32Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Float32Slice {
        allow_shared: bool,
        immutable: bool,
    },
    Float64Slice {
        allow_shared: bool,
        immutable: bool,
    },
}

pub(crate) trait ToIdlType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a>;
}

impl<'a> ToIdlType<'a> for UnionType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        let mut idl_types = Vec::with_capacity(self.body.list.len());
        for t in &self.body.list {
            idl_types.push(t.to_idl_type(record));
        }
        IdlType::Union(idl_types)
    }
}

impl<'a> ToIdlType<'a> for Type<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            Type::Single(t) => t.to_idl_type(record),
            Type::Union(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for SingleType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            SingleType::Any(t) => t.to_idl_type(record),
            SingleType::NonAny(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for NonAnyType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            NonAnyType::Promise(t) => t.to_idl_type(record),
            NonAnyType::Integer(t) => t.to_idl_type(record),
            NonAnyType::FloatingPoint(t) => t.to_idl_type(record),
            NonAnyType::Boolean(t) => t.to_idl_type(record),
            NonAnyType::Byte(t) => t.to_idl_type(record),
            NonAnyType::Octet(t) => t.to_idl_type(record),
            NonAnyType::ByteString(t) => t.to_idl_type(record),
            NonAnyType::DOMString(t) => t.to_idl_type(record),
            NonAnyType::USVString(t) => t.to_idl_type(record),
            NonAnyType::Sequence(t) => t.to_idl_type(record),
            NonAnyType::Object(t) => t.to_idl_type(record),
            NonAnyType::Symbol(t) => t.to_idl_type(record),
            NonAnyType::Error(t) => t.to_idl_type(record),
            NonAnyType::ArrayBuffer(t) => t.to_idl_type(record),
            NonAnyType::DataView(t) => t.to_idl_type(record),
            NonAnyType::Int8Array(t) => t.to_idl_type(record),
            NonAnyType::Int16Array(t) => t.to_idl_type(record),
            NonAnyType::Int32Array(t) => t.to_idl_type(record),
            NonAnyType::Uint8Array(t) => t.to_idl_type(record),
            NonAnyType::Uint16Array(t) => t.to_idl_type(record),
            NonAnyType::Uint32Array(t) => t.to_idl_type(record),
            NonAnyType::Uint8ClampedArray(t) => t.to_idl_type(record),
            NonAnyType::Float32Array(t) => t.to_idl_type(record),
            NonAnyType::Float64Array(t) => t.to_idl_type(record),
            NonAnyType::FrozenArrayType(t) => t.to_idl_type(record),
            NonAnyType::ObservableArrayType(t) => t.to_idl_type(record),
            NonAnyType::ArrayBufferView(t) => t.to_idl_type(record),
            NonAnyType::BufferSource(t) => t.to_idl_type(record),
            NonAnyType::RecordType(t) => t.to_idl_type(record),
            NonAnyType::Identifier(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for SequenceType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        IdlType::Sequence(Box::new(self.generics.body.to_idl_type(record)))
    }
}

impl<'a> ToIdlType<'a> for FrozenArrayType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        IdlType::FrozenArray(Box::new(self.generics.body.to_idl_type(record)))
    }
}

impl<'a> ToIdlType<'a> for ObservableArrayType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        IdlType::ObservableArray(Box::new(self.generics.body.to_idl_type(record)))
    }
}

impl<'a, T: ToIdlType<'a>> ToIdlType<'a> for MayBeNull<T> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        let inner_idl_type = self.type_.to_idl_type(record);
        if self.q_mark.is_some() {
            IdlType::Nullable(Box::new(inner_idl_type))
        } else {
            inner_idl_type
        }
    }
}

impl<'a> ToIdlType<'a> for PromiseType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        IdlType::Promise(Box::new(self.generics.body.to_idl_type(record)))
    }
}

impl<'a> ToIdlType<'a> for IntegerType {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            IntegerType::LongLong(t) => t.to_idl_type(record),
            IntegerType::Long(t) => t.to_idl_type(record),
            IntegerType::Short(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for LongLongType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
        if self.unsigned.is_some() {
            IdlType::UnsignedLongLong
        } else {
            IdlType::LongLong
        }
    }
}

impl<'a> ToIdlType<'a> for LongType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
        if self.unsigned.is_some() {
            IdlType::UnsignedLong
        } else {
            IdlType::Long
        }
    }
}

impl<'a> ToIdlType<'a> for ShortType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
        if self.unsigned.is_some() {
            IdlType::UnsignedShort
        } else {
            IdlType::Short
        }
    }
}

impl<'a> ToIdlType<'a> for FloatingPointType {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            FloatingPointType::Float(t) => t.to_idl_type(record),
            FloatingPointType::Double(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for FloatType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
        if self.unrestricted.is_some() {
            IdlType::UnrestrictedFloat
        } else {
            IdlType::Float
        }
    }
}

impl<'a> ToIdlType<'a> for DoubleType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
        if self.unrestricted.is_some() {
            IdlType::UnrestrictedDouble
        } else {
            IdlType::Double
        }
    }
}

impl<'a> ToIdlType<'a> for RecordType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        IdlType::Record(
            Box::new(self.generics.body.0.to_idl_type(record)),
            Box::new(self.generics.body.2.to_idl_type(record)),
        )
    }
}

impl<'a> ToIdlType<'a> for RecordKeyType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            RecordKeyType::Byte(t) => t.to_idl_type(record),
            RecordKeyType::DOM(t) => t.to_idl_type(record),
            RecordKeyType::USV(t) => t.to_idl_type(record),
            RecordKeyType::NonAny(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for UnionMemberType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            UnionMemberType::Single(t) => t.to_idl_type(record),
            UnionMemberType::Union(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for ConstType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            ConstType::Integer(t) => t.to_idl_type(record),
            ConstType::FloatingPoint(t) => t.to_idl_type(record),
            ConstType::Boolean(t) => t.to_idl_type(record),
            ConstType::Byte(t) => t.to_idl_type(record),
            ConstType::Octet(t) => t.to_idl_type(record),
            ConstType::Identifier(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for ReturnType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        match self {
            ReturnType::Undefined(t) => t.to_idl_type(record),
            ReturnType::Type(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for AttributedType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        self.type_.to_idl_type(record)
    }
}

impl<'a> ToIdlType<'a> for AttributedNonAnyType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        self.type_.to_idl_type(record)
    }
}

impl<'a> ToIdlType<'a> for Identifier<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> IdlType<'a> {
        let ty = if self.0 == "DOMTimeStamp" {
            // https://heycam.github.io/webidl/#DOMTimeStamp
            IdentifierType::UnsignedLongLong
        } else if self.0 == "AllowSharedBufferSource" {
            IdentifierType::AllowSharedBufferSource { immutable: false }
        } else if let Some(idl_type) = record.typedefs.get(&self.0) {
            return idl_type.to_idl_type(record);
        } else if record.interfaces.contains_key(self.0) {
            IdentifierType::Interface(self.0)
        } else if record.dictionaries.contains_key(self.0) {
            IdentifierType::Dictionary(self.0)
        } else if record.enums.contains_key(self.0) {
            IdentifierType::Enum(self.0)
        } else if record.callbacks.contains(self.0) {
            IdentifierType::Callback
        } else if record.iterators.contains(self.0) {
            IdentifierType::Iterator
        } else if record.async_iterators.contains(self.0) {
            IdentifierType::AsyncIterator
        } else if let Some(data) = record.callback_interfaces.get(self.0) {
            IdentifierType::CallbackInterface {
                name: self.0,
                single_function: data.single_function,
            }
        } else if self.0 == "WindowProxy" {
            // See this for more info:
            //
            // https://html.spec.whatwg.org/multipage/window-object.html#windowproxy
            //
            // namely this seems to be "legalese" for "this is a `Window`", so
            // let's translate it as such.
            IdentifierType::Interface("Window")
        } else {
            log::warn!("Unrecognized type: {}", self.0);
            return IdlType::UnknownIdentifier(self.0);
        };

        IdlType::id(self.0, ty)
    }
}

impl<'a> ToIdlType<'a> for term::DataView {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
        IdlType::DataView {
            allow_shared: false,
        }
    }
}

macro_rules! terms_to_idl_type {
    ($($t:tt => $r:tt)*) => ($(
        impl<'a> ToIdlType<'a> for term::$t {
            fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
                IdlType::$r
            }
        }
    )*);
}

// We default to arrays being mutable, but in certain cases where we're certain that
// slices won't get mutated on the JS side (such as the WebGL APIs) we might, later in the flow,
// instead use the immutable version.
macro_rules! terms_to_idl_type_maybe_immutable {
    ($($t:tt => $r:tt)*) => ($(
        impl<'a> ToIdlType<'a> for term::$t {
            fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> IdlType<'a> {
                IdlType::$r { allow_shared: false, immutable: false }
            }
        }
    )*);
}

terms_to_idl_type! {
    Symbol => Symbol
    ByteString => ByteString
    DOMString => DomString
    USVString => UsvString
    Any => Any
    Boolean => Boolean
    Byte => Byte
    Double => Double
    Float => Float
    Long => Long
    Object => Object
    Octet => Octet
    Short => Short
    Undefined => Undefined
    ArrayBuffer => ArrayBuffer
    Error => Error
}

terms_to_idl_type_maybe_immutable! {
    ArrayBufferView => ArrayBufferView
    BufferSource => BufferSource
    Float32Array => Float32Array
    Float64Array => Float64Array
    Int16Array => Int16Array
    Int32Array => Int32Array
    Int8Array => Int8Array
    Uint16Array => Uint16Array
    Uint32Array => Uint32Array
    Uint8Array => Uint8Array
    Uint8ClampedArray => Uint8ClampedArray
}

#[derive(Debug, Clone)]
pub enum TypeError {
    CannotConvert,
}

impl<'a> IdlType<'a> {
    fn id(name: &'a str, ty: IdentifierType<'a>) -> Self {
        IdlType::Identifier { name, ty }
    }

    /// Generates a snake case type name.
    pub(crate) fn push_snake_case_name(&self, dst: &mut String) {
        match self {
            IdlType::Boolean => dst.push_str("bool"),
            IdlType::Byte => dst.push_str("i8"),
            IdlType::Octet => dst.push_str("u8"),
            IdlType::Short => dst.push_str("i16"),
            IdlType::UnsignedShort => dst.push_str("u16"),
            IdlType::Long => dst.push_str("i32"),
            IdlType::UnsignedLong => dst.push_str("u32"),
            IdlType::LongLong => dst.push_str("i64"),
            IdlType::UnsignedLongLong => dst.push_str("u64"),
            IdlType::Float | IdlType::UnrestrictedFloat => dst.push_str("f32"),
            IdlType::Double | IdlType::UnrestrictedDouble => dst.push_str("f64"),
            IdlType::DomString | IdlType::ByteString | IdlType::UsvString => dst.push_str("str"),
            IdlType::Object => dst.push_str("object"),
            IdlType::Symbol => dst.push_str("symbol"),
            IdlType::Error => dst.push_str("error"),

            IdlType::ArrayBuffer => dst.push_str("array_buffer"),
            IdlType::DataView { .. } => dst.push_str("data_view"),
            IdlType::Int8Array { .. } => dst.push_str("i8_array"),
            IdlType::Uint8Array { .. } => dst.push_str("u8_array"),
            IdlType::Uint8ClampedArray { .. } => dst.push_str("u8_clamped_array"),
            IdlType::Int16Array { .. } => dst.push_str("i16_array"),
            IdlType::Uint16Array { .. } => dst.push_str("u16_array"),
            IdlType::Int32Array { .. } => dst.push_str("i32_array"),
            IdlType::Uint32Array { .. } => dst.push_str("u32_array"),
            IdlType::Float32Array { .. } => dst.push_str("f32_array"),
            IdlType::Float64Array { .. } => dst.push_str("f64_array"),
            IdlType::ArrayBufferView { .. } => dst.push_str("array_buffer_view"),
            IdlType::BufferSource { .. } => dst.push_str("buffer_source"),

            IdlType::UnknownIdentifier(name) => dst.push_str(&snake_case_ident(name)),

            IdlType::Nullable(idl_type) => {
                dst.push_str("opt_");
                idl_type.push_snake_case_name(dst);
            }
            IdlType::FrozenArray(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_frozen_array");
            }
            IdlType::Sequence(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_sequence");
            }
            IdlType::ObservableArray(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_observable_array");
            }
            IdlType::Promise(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_promise");
            }
            IdlType::Record(idl_type_from, idl_type_to) => {
                dst.push_str("record_from_");
                idl_type_from.push_snake_case_name(dst);
                dst.push_str("_to_");
                idl_type_to.push_snake_case_name(dst);
            }
            IdlType::Union(idl_types) => {
                dst.push_str("union_of_");
                let mut first = true;
                for idl_type in idl_types {
                    if first {
                        first = false;
                    } else {
                        dst.push_str("_and_");
                    }
                    idl_type.push_snake_case_name(dst);
                }
            }

            IdlType::Any => dst.push_str("any"),
            IdlType::Undefined => dst.push_str("undefined"),

            IdlType::Identifier { ty, .. } => match ty {
                IdentifierType::Callback => dst.push_str("callback"),
                IdentifierType::Iterator => dst.push_str("iterator"),
                IdentifierType::AsyncIterator => dst.push_str("async_iterator"),
                IdentifierType::Interface(name) => dst.push_str(&snake_case_ident(name)),
                IdentifierType::Dictionary(name) => dst.push_str(&snake_case_ident(name)),
                IdentifierType::Enum(name) => dst.push_str(&snake_case_ident(name)),
                IdentifierType::CallbackInterface { name, .. } => {
                    dst.push_str(&snake_case_ident(name))
                }
                IdentifierType::UnsignedLongLong => {
                    IdlType::UnsignedLongLong.push_snake_case_name(dst)
                }
                IdentifierType::AllowSharedBufferSource { immutable } => IdlType::BufferSource {
                    allow_shared: true,
                    immutable: *immutable,
                }
                .push_snake_case_name(dst),
                IdentifierType::Int8Slice { .. } => dst.push_str("i8_slice"),
                IdentifierType::Uint8Slice { .. } => dst.push_str("u8_slice"),
                IdentifierType::Uint8ClampedSlice { .. } => dst.push_str("u8_clamped_slice"),
                IdentifierType::Int16Slice { .. } => dst.push_str("i16_slice"),
                IdentifierType::Uint16Slice { .. } => dst.push_str("u16_slice"),
                IdentifierType::Int32Slice { .. } => dst.push_str("i32_slice"),
                IdentifierType::Uint32Slice { .. } => dst.push_str("u32_slice"),
                IdentifierType::Float32Slice { .. } => dst.push_str("f32_slice"),
                IdentifierType::Float64Slice { .. } => dst.push_str("f64_slice"),
            },
        }
    }

    /// Converts to syn type if possible.
    pub(crate) fn to_syn_type(
        &self,
        pos: TypePosition,
        legacy: bool,
    ) -> Result<Option<syn::Type>, TypeError> {
        let externref = |ty| {
            Some(match pos {
                TypePosition::Argument => shared_ref(ty, false),
                TypePosition::Return => ty,
            })
        };
        let js_sys = |name: &str| {
            let path = vec![rust_ident("js_sys"), rust_ident(name)];
            let ty = leading_colon_path_ty(path);
            externref(ty)
        };
        let js_value = {
            let path = vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")];
            externref(leading_colon_path_ty(path))
        };
        let string = {
            let path = vec![
                rust_ident("alloc"),
                rust_ident("string"),
                rust_ident("String"),
            ];
            externref(leading_colon_path_ty(path))
        };
        match self {
            IdlType::Boolean => Ok(Some(ident_ty(raw_ident("bool")))),
            IdlType::Byte => Ok(Some(ident_ty(raw_ident("i8")))),
            IdlType::Octet => Ok(Some(ident_ty(raw_ident("u8")))),
            IdlType::Short => Ok(Some(ident_ty(raw_ident("i16")))),
            IdlType::UnsignedShort => Ok(Some(ident_ty(raw_ident("u16")))),
            IdlType::Long => Ok(Some(ident_ty(raw_ident("i32")))),
            IdlType::UnsignedLong => Ok(Some(ident_ty(raw_ident("u32")))),

            // Technically these are 64-bit numbers, but we're binding web
            // APIs that don't actually have return the corresponding 64-bit
            // type, `BigInt`. Instead the web basically uses floats for these
            // values. We already expand these types in argument position to
            // i32/f64 (convenience for i32, losslessness for f64). If we get
            // here then we're looking at an un-flattened long type such as
            // dictionary fields or return types. In order to generate bindings
            // for these functions we just use `f64` here, which should match
            // exactly what the JS web currently uses anyway.
            //
            // Perhaps one day we'll bind to u64/i64 here, but we need `BigInt`
            // to see more usage!
            IdlType::LongLong | IdlType::UnsignedLongLong => Ok(Some(ident_ty(raw_ident("f64")))),

            IdlType::Float => Ok(Some(ident_ty(raw_ident("f32")))),
            IdlType::UnrestrictedFloat => Ok(Some(ident_ty(raw_ident("f32")))),
            IdlType::Double => Ok(Some(ident_ty(raw_ident("f64")))),
            IdlType::UnrestrictedDouble => Ok(Some(ident_ty(raw_ident("f64")))),
            IdlType::DomString | IdlType::ByteString | IdlType::UsvString => match pos {
                TypePosition::Argument => Ok(Some(shared_ref(ident_ty(raw_ident("str")), false))),
                TypePosition::Return => Ok(string),
            },
            IdlType::Object => Ok(js_sys("Object")),
            IdlType::Symbol => Err(TypeError::CannotConvert),
            IdlType::Error => Err(TypeError::CannotConvert),

            IdlType::ArrayBuffer => Ok(js_sys("ArrayBuffer")),
            IdlType::DataView { .. } => Ok(js_sys("DataView")),
            IdlType::Int8Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("i8", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Int8Array")),
            },
            IdlType::Uint8Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("u8", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Uint8Array")),
            },
            IdlType::Uint8ClampedArray { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => {
                    Ok(Some(clamped(array("u8", pos, *immutable))))
                }
                (false, TypePosition::Argument) => Ok(js_sys("Uint8ClampedArray")),
            },
            IdlType::Int16Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("i16", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Int16Array")),
            },
            IdlType::Uint16Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("u16", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Uint16Array")),
            },
            IdlType::Int32Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("i32", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Int32Array")),
            },
            IdlType::Uint32Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("u32", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Uint32Array")),
            },
            IdlType::Float32Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("f32", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Float32Array")),
            },
            IdlType::Float64Array { immutable, .. } => match (legacy, pos) {
                (true, _) | (_, TypePosition::Return) => Ok(Some(array("f64", pos, *immutable))),
                (false, TypePosition::Argument) => Ok(js_sys("Float64Array")),
            },

            IdlType::ArrayBufferView { .. } | IdlType::BufferSource { .. } => Ok(js_sys("Object")),

            IdlType::Nullable(idl_type) => {
                let inner = idl_type.to_syn_type(pos, legacy)?;

                match inner {
                    Some(inner) => {
                        // TODO: this is a bit of a hack, but `Option<JsValue>` isn't
                        // supported right now. As a result if we see `JsValue` for our
                        // inner type, leave that as the same when we create a nullable
                        // version of that. That way `any?` just becomes `JsValue` and
                        // it's up to users to dispatch and/or create instances
                        // appropriately.
                        if let syn::Type::Path(path) = &inner {
                            if path.qself.is_none()
                                && path
                                    .path
                                    .segments
                                    .last()
                                    .map(|p| p.ident == "JsValue")
                                    .unwrap_or(false)
                            {
                                return Ok(Some(inner.clone()));
                            }
                        }

                        Ok(Some(option_ty(inner)))
                    }
                    None => Ok(None),
                }
            }
            // webidl sequences must always be returned as javascript `Array`s. They may accept
            // anything implementing the @@iterable interface.
            // The same implementation is fine for `FrozenArray`
            IdlType::FrozenArray(_idl_type)
            | IdlType::Sequence(_idl_type)
            | IdlType::ObservableArray(_idl_type) => match pos {
                TypePosition::Argument => Ok(js_value),
                TypePosition::Return => Ok(js_sys("Array")),
            },
            IdlType::Promise(_idl_type) => Ok(js_sys("Promise")),
            IdlType::Record(_idl_type_from, _idl_type_to) => Ok(js_sys("Object")),
            IdlType::Union(idl_types) => {
                // Note that most union types have already been expanded to
                // their components via `flatten`. Unions in a return position
                // or dictionary fields, however, haven't been flattened, which
                // means we may need to convert them to a `syn` type.
                //
                // Currently this does a bit of a "poor man's" tree traversal by
                // saying that if all union members are interfaces we can assume
                // they've all got `Object` as a superclass, so we can take an
                // object here. If any are not an interface though we
                // pessimisitcally translate the union into a `JsValue`,
                // absolutely anything. It's up to the application to figure out
                // what to do with that.
                //
                // TODO: we should probably do a better job here translating
                // unions to a single type. Two possible strategies could be:
                //
                // 1. Use strategy of finding the nearest common subclass
                //    (finding the best type that is suitable for all values of
                //    this union) instead of always assuming `Object`.
                // 2. Generate enum with payload in Rust for each union type.
                //    Such an enum, however, might have a relatively high
                //    overhead in creating it from a JS value, but would be
                //    cheap to convert from a variant back to a JS value.
                if idl_types.iter().all(|idl_type| {
                    matches!(
                        idl_type,
                        IdlType::Identifier {
                            ty: IdentifierType::Interface(..),
                            ..
                        }
                    )
                }) {
                    IdlType::Object.to_syn_type(pos, legacy)
                } else {
                    IdlType::Any.to_syn_type(pos, legacy)
                }
            }

            IdlType::Any => Ok(js_value),
            IdlType::Undefined => Ok(None),
            IdlType::Identifier { ty, .. } => ty.to_syn_type(pos, legacy),
            IdlType::UnknownIdentifier(_) => Err(TypeError::CannotConvert),
        }
    }

    /// Flattens unions recursively.
    ///
    /// Works similarly to [flattened union member types],
    /// but also flattens unions inside generics of other types.
    ///
    /// [flattened union member types]: https://heycam.github.io/webidl/#dfn-flattened-union-member-types
    pub(crate) fn flatten(&self, attrs: Option<&ExtendedAttributeList<'_>>) -> Vec<Self> {
        match self {
            IdlType::Nullable(idl_type) => idl_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(IdlType::Nullable)
                .collect(),
            IdlType::FrozenArray(idl_type) => idl_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(IdlType::FrozenArray)
                .collect(),
            IdlType::Sequence(idl_type) => idl_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(IdlType::Sequence)
                .collect(),
            IdlType::Promise(idl_type) => idl_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(IdlType::Promise)
                .collect(),
            IdlType::Record(idl_type_from, idl_type_to) => {
                let mut idl_types = Vec::new();
                for idl_type_from in idl_type_from.flatten(attrs) {
                    for idl_type_to in idl_type_to.flatten(attrs) {
                        idl_types.push(IdlType::Record(
                            Box::new(idl_type_from.clone()),
                            Box::new(idl_type_to.clone()),
                        ));
                    }
                }
                idl_types
            }
            IdlType::Union(idl_types) => idl_types
                .iter()
                .flat_map(|idl_type| idl_type.flatten(attrs))
                .collect(),
            IdlType::ArrayBufferView {
                allow_shared,
                immutable,
            } => {
                let view = IdlType::ArrayBufferView {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                };

                if let Some(attrs) = attrs {
                    for attr in &attrs.body.list {
                        if let ExtendedAttribute::NoArgs(attr) = attr {
                            if attr.0 .0 == "RustNotWasmMemory" {
                                return vec![view];
                            }
                        }
                    }
                }

                vec![
                    view,
                    IdlType::Identifier {
                        name: "Uint8Array",
                        ty: IdentifierType::Uint8Slice {
                            allow_shared: *allow_shared,
                            immutable: *immutable,
                        },
                    },
                    IdlType::Uint8Array {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                ]
            }
            IdlType::BufferSource {
                allow_shared,
                immutable,
            } => vec![
                IdlType::BufferSource {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
                IdlType::Identifier {
                    name: "Uint8Array",
                    ty: IdentifierType::Uint8Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Uint8Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::LongLong => vec![IdlType::Long, IdlType::Double],
            IdlType::UnsignedLongLong => vec![IdlType::UnsignedLong, IdlType::Double],
            IdlType::Int8Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Int8Array",
                    ty: IdentifierType::Int8Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Int8Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Uint8Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Uint8Array",
                    ty: IdentifierType::Uint8Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Uint8Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Uint8ClampedArray {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Uint8ClampedArray",
                    ty: IdentifierType::Uint8ClampedSlice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Uint8ClampedArray {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Int16Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Int16Array",
                    ty: IdentifierType::Int16Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Int16Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Uint16Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Uint16Array",
                    ty: IdentifierType::Uint16Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Uint16Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Int32Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Int32Array",
                    ty: IdentifierType::Int32Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Int32Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Uint32Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Uint32Array",
                    ty: IdentifierType::Uint32Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Uint32Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Float32Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Float32Array",
                    ty: IdentifierType::Float32Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Float32Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            IdlType::Float64Array {
                allow_shared,
                immutable,
            } => vec![
                IdlType::Identifier {
                    name: "Float64Array",
                    ty: IdentifierType::Float64Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                IdlType::Float64Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            idl_type @ IdlType::Identifier {
                name: identifier,
                ty,
            } => {
                match ty {
                    IdentifierType::CallbackInterface {
                        name,
                        single_function: true,
                    } => {
                        // According to the webidl spec [1] single-function callback
                        // interfaces can also be replaced in arguments with simply a
                        // single callable function, which we map to a `Callback`.
                        //
                        // [1]: https://heycam.github.io/webidl/#es-user-objects
                        vec![
                            IdlType::id(identifier, IdentifierType::Callback),
                            IdlType::id(
                                identifier,
                                IdentifierType::CallbackInterface {
                                    name,
                                    single_function: false,
                                },
                            ),
                        ]
                    }
                    IdentifierType::UnsignedLongLong => IdlType::UnsignedLongLong.flatten(attrs),
                    IdentifierType::AllowSharedBufferSource { immutable } => {
                        IdlType::BufferSource {
                            allow_shared: true,
                            immutable: *immutable,
                        }
                        .flatten(attrs)
                    }
                    _ => vec![idl_type.clone()],
                }
            }
            idl_type => vec![idl_type.clone()],
        }
    }

    pub(crate) fn orig(&self) -> Cow<'_, Self> {
        if let Self::Identifier { name, .. } = self {
            Cow::Owned(Self::UnknownIdentifier(name))
        } else {
            Cow::Borrowed(self)
        }
    }
}

impl IdentifierType<'_> {
    /// Converts to syn type if possible.
    pub(crate) fn to_syn_type(
        &self,
        pos: TypePosition,
        legacy: bool,
    ) -> Result<Option<syn::Type>, TypeError> {
        let externref = |ty| {
            Some(match pos {
                TypePosition::Argument => shared_ref(ty, false),
                TypePosition::Return => ty,
            })
        };
        let js_sys = |name: &str| {
            let path = vec![rust_ident("js_sys"), rust_ident(name)];
            let ty = leading_colon_path_ty(path);
            externref(ty)
        };
        match self {
            IdentifierType::Callback => Ok(js_sys("Function")),
            IdentifierType::Iterator => Ok(js_sys("Iterator")),
            IdentifierType::AsyncIterator => Ok(js_sys("AsyncIterator")),
            IdentifierType::Interface(name)
            | IdentifierType::Dictionary(name)
            | IdentifierType::CallbackInterface { name, .. } => {
                let ty = ident_ty(rust_ident(camel_case_ident(name).as_str()));
                Ok(externref(ty))
            }
            IdentifierType::Enum(name) => {
                Ok(Some(ident_ty(rust_ident(camel_case_ident(name).as_str()))))
            }
            IdentifierType::UnsignedLongLong => IdlType::UnsignedLongLong.to_syn_type(pos, legacy),
            IdentifierType::AllowSharedBufferSource { immutable } => IdlType::BufferSource {
                allow_shared: true,
                immutable: *immutable,
            }
            .to_syn_type(pos, legacy),
            IdentifierType::Int8Slice { immutable, .. } => Ok(Some(array("i8", pos, *immutable))),
            IdentifierType::Uint8Slice { immutable, .. } => Ok(Some(array("u8", pos, *immutable))),
            IdentifierType::Uint8ClampedSlice { immutable, .. } => {
                Ok(Some(clamped(array("u8", pos, *immutable))))
            }
            IdentifierType::Int16Slice { immutable, .. } => Ok(Some(array("i16", pos, *immutable))),
            IdentifierType::Uint16Slice { immutable, .. } => {
                Ok(Some(array("u16", pos, *immutable)))
            }
            IdentifierType::Int32Slice { immutable, .. } => Ok(Some(array("i32", pos, *immutable))),
            IdentifierType::Uint32Slice { immutable, .. } => {
                Ok(Some(array("u32", pos, *immutable)))
            }
            IdentifierType::Float32Slice { immutable, .. } => {
                Ok(Some(array("f32", pos, *immutable)))
            }
            IdentifierType::Float64Slice { immutable, .. } => {
                Ok(Some(array("f64", pos, *immutable)))
            }
        }
    }
}

#[test]
fn idl_type_flatten_test() {
    use self::IdentifierType::*;
    use self::IdlType::*;

    assert_eq!(
        Union(vec![
            IdlType::id("Node", Interface("Node")),
            Union(vec![
                Sequence(Box::new(Long),),
                IdlType::id("Event", Interface("Event"))
            ]),
            Nullable(Box::new(Union(vec![
                IdlType::id("XMLHttpRequest", Interface("XMLHttpRequest")),
                DomString,
            ])),),
            Sequence(Box::new(Union(vec![
                Sequence(Box::new(Double),),
                IdlType::id("NodeList", Interface("NodeList")),
            ])),),
        ])
        .flatten(None),
        vec![
            IdlType::id("Node", Interface("Node")),
            Sequence(Box::new(Long)),
            IdlType::id("Event", Interface("Event")),
            Nullable(Box::new(IdlType::id(
                "XMLHttpRequest",
                Interface("XMLHttpRequest")
            ))),
            Nullable(Box::new(DomString)),
            Sequence(Box::new(Sequence(Box::new(Double)))),
            Sequence(Box::new(IdlType::id("NodeList", Interface("NodeList")))),
        ],
    );
}

/// From `T` create `::wasm_bindgen::Clamped<T>`
fn clamped(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: vec![syn::GenericArgument::Type(t)].into_iter().collect(),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Clamped");
    let seg = syn::PathSegment { ident, arguments };
    syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: Some(Default::default()),
            segments: vec![Ident::new("wasm_bindgen", Span::call_site()).into(), seg]
                .into_iter()
                .collect(),
        },
    }
    .into()
}
