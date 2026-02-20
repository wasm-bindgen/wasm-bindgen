use crate::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use proc_macro2::{Ident, Span};
use syn::parse_quote;
use weedle::attribute::{ExtendedAttribute, ExtendedAttributeList};
use weedle::common::Identifier;
use weedle::term;
use weedle::types::*;

use crate::first_pass::FirstPassRecord;
use crate::util::{
    array, camel_case_ident, generic_ty, js_option_ty, option_ty, shared_ref, slice_ty,
    snake_case_ident, Direction, TypePosition,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub(crate) enum WbgType<'a> {
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
    BigInt,
    Date,

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

    JsOption(Box<WbgType<'a>>),
    FrozenArray(Box<WbgType<'a>>),
    ObservableArray(Box<WbgType<'a>>),
    Sequence(Box<WbgType<'a>>),
    Promise(Box<WbgType<'a>>),
    Record(Box<WbgType<'a>>, Box<WbgType<'a>>),
    Iterator(Box<WbgType<'a>>),
    AsyncIterator(Box<WbgType<'a>>),
    Callback {
        params: Vec<WbgType<'a>>,
        return_type: Option<Box<WbgType<'a>>>,
    },
    ArrayTuple(Box<WbgType<'a>>, Box<WbgType<'a>>),
    Union(Vec<WbgType<'a>>),

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
    #[allow(dead_code)]
    Callback,
    #[allow(dead_code)]
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

pub(crate) trait ToWbgType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a>;
}

impl<'a> ToWbgType<'a> for UnionType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        let mut wbg_types = Vec::with_capacity(self.body.list.len());
        for t in &self.body.list {
            wbg_types.push(t.to_wbg_type(record));
        }
        WbgType::Union(wbg_types)
    }
}

impl<'a> ToWbgType<'a> for Type<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            Type::Single(t) => t.to_wbg_type(record),
            Type::Union(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for SingleType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            SingleType::Any(t) => t.to_wbg_type(record),
            SingleType::NonAny(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for NonAnyType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            NonAnyType::Promise(t) => t.to_wbg_type(record),
            NonAnyType::Integer(t) => t.to_wbg_type(record),
            NonAnyType::FloatingPoint(t) => t.to_wbg_type(record),
            NonAnyType::Boolean(t) => t.to_wbg_type(record),
            NonAnyType::Byte(t) => t.to_wbg_type(record),
            NonAnyType::Octet(t) => t.to_wbg_type(record),
            NonAnyType::ByteString(t) => t.to_wbg_type(record),
            NonAnyType::DOMString(t) => t.to_wbg_type(record),
            NonAnyType::USVString(t) => t.to_wbg_type(record),
            NonAnyType::Sequence(t) => t.to_wbg_type(record),
            NonAnyType::Object(t) => t.to_wbg_type(record),
            NonAnyType::Symbol(t) => t.to_wbg_type(record),
            NonAnyType::Error(t) => t.to_wbg_type(record),
            NonAnyType::ArrayBuffer(t) => t.to_wbg_type(record),
            NonAnyType::DataView(t) => t.to_wbg_type(record),
            NonAnyType::Int8Array(t) => t.to_wbg_type(record),
            NonAnyType::Int16Array(t) => t.to_wbg_type(record),
            NonAnyType::Int32Array(t) => t.to_wbg_type(record),
            NonAnyType::Uint8Array(t) => t.to_wbg_type(record),
            NonAnyType::Uint16Array(t) => t.to_wbg_type(record),
            NonAnyType::Uint32Array(t) => t.to_wbg_type(record),
            NonAnyType::Uint8ClampedArray(t) => t.to_wbg_type(record),
            NonAnyType::Float32Array(t) => t.to_wbg_type(record),
            NonAnyType::Float64Array(t) => t.to_wbg_type(record),
            NonAnyType::FrozenArrayType(t) => t.to_wbg_type(record),
            NonAnyType::ObservableArrayType(t) => t.to_wbg_type(record),
            NonAnyType::ArrayBufferView(t) => t.to_wbg_type(record),
            NonAnyType::BufferSource(t) => t.to_wbg_type(record),
            NonAnyType::RecordType(t) => t.to_wbg_type(record),
            NonAnyType::Identifier(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for SequenceType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        WbgType::Sequence(Box::new(self.generics.body.to_wbg_type(record)))
    }
}

impl<'a> ToWbgType<'a> for FrozenArrayType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        WbgType::FrozenArray(Box::new(self.generics.body.to_wbg_type(record)))
    }
}

impl<'a> ToWbgType<'a> for ObservableArrayType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        WbgType::ObservableArray(Box::new(self.generics.body.to_wbg_type(record)))
    }
}

impl<'a, T: ToWbgType<'a>> ToWbgType<'a> for MayBeNull<T> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        let inner_wbg_type = self.type_.to_wbg_type(record);
        if self.q_mark.is_some() {
            WbgType::JsOption(Box::new(inner_wbg_type))
        } else {
            inner_wbg_type
        }
    }
}

impl<'a> ToWbgType<'a> for PromiseType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        WbgType::Promise(Box::new(self.generics.body.to_wbg_type(record)))
    }
}

impl<'a> ToWbgType<'a> for IntegerType {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            IntegerType::LongLong(t) => t.to_wbg_type(record),
            IntegerType::Long(t) => t.to_wbg_type(record),
            IntegerType::Short(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for LongLongType {
    fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
        if self.unsigned.is_some() {
            WbgType::UnsignedLongLong
        } else {
            WbgType::LongLong
        }
    }
}

impl<'a> ToWbgType<'a> for LongType {
    fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
        if self.unsigned.is_some() {
            WbgType::UnsignedLong
        } else {
            WbgType::Long
        }
    }
}

impl<'a> ToWbgType<'a> for ShortType {
    fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
        if self.unsigned.is_some() {
            WbgType::UnsignedShort
        } else {
            WbgType::Short
        }
    }
}

impl<'a> ToWbgType<'a> for FloatingPointType {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            FloatingPointType::Float(t) => t.to_wbg_type(record),
            FloatingPointType::Double(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for FloatType {
    fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
        if self.unrestricted.is_some() {
            WbgType::UnrestrictedFloat
        } else {
            WbgType::Float
        }
    }
}

impl<'a> ToWbgType<'a> for DoubleType {
    fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
        if self.unrestricted.is_some() {
            WbgType::UnrestrictedDouble
        } else {
            WbgType::Double
        }
    }
}

impl<'a> ToWbgType<'a> for RecordType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        WbgType::Record(
            Box::new(self.generics.body.0.to_wbg_type(record)),
            Box::new(self.generics.body.2.to_wbg_type(record)),
        )
    }
}

impl<'a> ToWbgType<'a> for RecordKeyType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            RecordKeyType::Byte(t) => t.to_wbg_type(record),
            RecordKeyType::DOM(t) => t.to_wbg_type(record),
            RecordKeyType::USV(t) => t.to_wbg_type(record),
            RecordKeyType::NonAny(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for UnionMemberType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            UnionMemberType::Single(t) => t.to_wbg_type(record),
            UnionMemberType::Union(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for ConstType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            ConstType::Integer(t) => t.to_wbg_type(record),
            ConstType::FloatingPoint(t) => t.to_wbg_type(record),
            ConstType::Boolean(t) => t.to_wbg_type(record),
            ConstType::Byte(t) => t.to_wbg_type(record),
            ConstType::Octet(t) => t.to_wbg_type(record),
            ConstType::Identifier(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for ReturnType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        match self {
            ReturnType::Undefined(t) => t.to_wbg_type(record),
            ReturnType::Type(t) => t.to_wbg_type(record),
        }
    }
}

impl<'a> ToWbgType<'a> for AttributedType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        self.type_.to_wbg_type(record)
    }
}

impl<'a> ToWbgType<'a> for AttributedNonAnyType<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        self.type_.to_wbg_type(record)
    }
}

impl<'a> ToWbgType<'a> for Identifier<'a> {
    fn to_wbg_type(&self, record: &FirstPassRecord<'a>) -> WbgType<'a> {
        let ty = if self.0 == "DOMTimeStamp" {
            // https://heycam.github.io/webidl/#DOMTimeStamp
            IdentifierType::UnsignedLongLong
        } else if self.0 == "AllowSharedBufferSource" {
            IdentifierType::AllowSharedBufferSource { immutable: false }
        } else if let Some(wbg_type) = record.typedefs.get(&self.0) {
            return wbg_type.to_wbg_type(record);
        } else if record.interfaces.contains_key(self.0) {
            IdentifierType::Interface(self.0)
        } else if record.dictionaries.contains_key(self.0) {
            IdentifierType::Dictionary(self.0)
        } else if record.enums.contains_key(self.0) {
            IdentifierType::Enum(self.0)
        } else if let Some(callback_data) = record.callbacks.get(self.0) {
            if record.options.next_unstable.get() {
                // In next_unstable mode, use typed callback
                return WbgType::Callback {
                    params: callback_data.params.clone(),
                    return_type: callback_data.return_type.clone().map(Box::new),
                };
            } else {
                // Legacy: just use Callback identifier type
                IdentifierType::Callback
            }
        } else if record.iterators.contains(self.0) {
            // Always use interface - generator handles compat/non-compat
            IdentifierType::Interface(self.0)
        } else if record.async_iterators.contains(self.0) {
            IdentifierType::AsyncIterator
        } else if let Some(data) = record.callback_interfaces.get(self.0) {
            if record.options.next_unstable.get() && data.single_function {
                // In next_unstable mode, single-function callback interfaces become typed callbacks
                return WbgType::Callback {
                    params: data.params.clone(),
                    return_type: data.return_type.clone().map(Box::new),
                };
            } else {
                IdentifierType::CallbackInterface {
                    name: self.0,
                    single_function: data.single_function,
                }
            }
        } else if self.0 == "WindowProxy" {
            // See this for more info:
            //
            // https://html.spec.whatwg.org/multipage/window-object.html#windowproxy
            //
            // namely this seems to be "legalese" for "this is a `Window`", so
            // let's translate it as such.
            IdentifierType::Interface("Window")
        } else if self.0 == "UUID" || self.0 == "URI" {
            // UUID and URI are DOMStrings in practice
            return WbgType::DomString;
        } else if self.0 == "bigint" {
            // WebIDL bigint maps to js_sys::BigInt
            return WbgType::BigInt;
        } else if self.0 == "Date" {
            // WebIDL Date maps to js_sys::Date
            return WbgType::Date;
        } else {
            log::warn!("Unrecognized type: {}", self.0);
            return WbgType::UnknownIdentifier(self.0);
        };

        WbgType::id(self.0, ty)
    }
}

impl<'a> ToWbgType<'a> for term::DataView {
    fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
        WbgType::DataView {
            allow_shared: false,
        }
    }
}

macro_rules! terms_to_wbg_type {
    ($($t:tt => $r:tt)*) => ($(
        impl<'a> ToWbgType<'a> for term::$t {
            fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
                WbgType::$r
            }
        }
    )*);
}

// We default to arrays being mutable, but in certain cases where we're certain that
// slices won't get mutated on the JS side (such as the WebGL APIs) we might, later in the flow,
// instead use the immutable version.
macro_rules! terms_to_wbg_type_maybe_immutable {
    ($($t:tt => $r:tt)*) => ($(
        impl<'a> ToWbgType<'a> for term::$t {
            fn to_wbg_type(&self, _record: &FirstPassRecord<'a>) -> WbgType<'a> {
                WbgType::$r { allow_shared: false, immutable: false }
            }
        }
    )*);
}

terms_to_wbg_type! {
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

terms_to_wbg_type_maybe_immutable! {
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

impl<'a> WbgType<'a> {
    fn id(name: &'a str, ty: IdentifierType<'a>) -> Self {
        WbgType::Identifier { name, ty }
    }

    /// Generates a snake case type name.
    pub(crate) fn push_snake_case_name(&self, dst: &mut String) {
        match self {
            WbgType::Boolean => dst.push_str("bool"),
            WbgType::Byte => dst.push_str("i8"),
            WbgType::Octet => dst.push_str("u8"),
            WbgType::Short => dst.push_str("i16"),
            WbgType::UnsignedShort => dst.push_str("u16"),
            WbgType::Long => dst.push_str("i32"),
            WbgType::UnsignedLong => dst.push_str("u32"),
            WbgType::LongLong => dst.push_str("i64"),
            WbgType::UnsignedLongLong => dst.push_str("u64"),
            WbgType::Float | WbgType::UnrestrictedFloat => dst.push_str("f32"),
            WbgType::Double | WbgType::UnrestrictedDouble => dst.push_str("f64"),
            WbgType::DomString | WbgType::ByteString | WbgType::UsvString => dst.push_str("str"),
            WbgType::Object => dst.push_str("object"),
            WbgType::Symbol => dst.push_str("symbol"),
            WbgType::Error => dst.push_str("error"),
            WbgType::BigInt => dst.push_str("big_int"),
            WbgType::Date => dst.push_str("date"),

            WbgType::ArrayBuffer => dst.push_str("array_buffer"),
            WbgType::DataView { .. } => dst.push_str("data_view"),
            WbgType::Int8Array { .. } => dst.push_str("i8_array"),
            WbgType::Uint8Array { .. } => dst.push_str("u8_array"),
            WbgType::Uint8ClampedArray { .. } => dst.push_str("u8_clamped_array"),
            WbgType::Int16Array { .. } => dst.push_str("i16_array"),
            WbgType::Uint16Array { .. } => dst.push_str("u16_array"),
            WbgType::Int32Array { .. } => dst.push_str("i32_array"),
            WbgType::Uint32Array { .. } => dst.push_str("u32_array"),
            WbgType::Float32Array { .. } => dst.push_str("f32_array"),
            WbgType::Float64Array { .. } => dst.push_str("f64_array"),
            WbgType::ArrayBufferView { .. } => dst.push_str("array_buffer_view"),
            WbgType::BufferSource { .. } => dst.push_str("buffer_source"),

            WbgType::UnknownIdentifier(name) => dst.push_str(&snake_case_ident(name)),

            WbgType::JsOption(wbg_type) => {
                dst.push_str("opt_");
                wbg_type.push_snake_case_name(dst);
            }
            WbgType::FrozenArray(wbg_type) => {
                wbg_type.push_snake_case_name(dst);
                dst.push_str("_frozen_array");
            }
            WbgType::Sequence(wbg_type) => {
                wbg_type.push_snake_case_name(dst);
                dst.push_str("_sequence");
            }
            WbgType::ObservableArray(wbg_type) => {
                wbg_type.push_snake_case_name(dst);
                dst.push_str("_observable_array");
            }
            WbgType::Promise(wbg_type) => {
                wbg_type.push_snake_case_name(dst);
                dst.push_str("_promise");
            }
            WbgType::Iterator(wbg_type) => {
                wbg_type.push_snake_case_name(dst);
                dst.push_str("_iterator");
            }
            WbgType::AsyncIterator(wbg_type) => {
                wbg_type.push_snake_case_name(dst);
                dst.push_str("_async_iterator");
            }
            WbgType::Callback { .. } => {
                // Always use simple "callback" for naming - signature info is only for type generation
                dst.push_str("callback");
            }
            WbgType::ArrayTuple(wbg_type_a, wbg_type_b) => {
                dst.push_str("array_tuple_");
                wbg_type_a.push_snake_case_name(dst);
                dst.push('_');
                wbg_type_b.push_snake_case_name(dst);
            }
            WbgType::Record(wbg_type_from, wbg_type_to) => {
                dst.push_str("record_from_");
                wbg_type_from.push_snake_case_name(dst);
                dst.push_str("_to_");
                wbg_type_to.push_snake_case_name(dst);
            }
            WbgType::Union(wbg_types) => {
                dst.push_str("union_of_");
                let mut first = true;
                for wbg_type in wbg_types {
                    if first {
                        first = false;
                    } else {
                        dst.push_str("_and_");
                    }
                    wbg_type.push_snake_case_name(dst);
                }
            }

            WbgType::Any => dst.push_str("any"),
            WbgType::Undefined => dst.push_str("undefined"),

            WbgType::Identifier { ty, .. } => match ty {
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
                    WbgType::UnsignedLongLong.push_snake_case_name(dst)
                }
                IdentifierType::AllowSharedBufferSource { immutable } => WbgType::BufferSource {
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

    /// Returns true if this type is a primitive that can be used directly in a Rust slice.
    /// These types have efficient `IntoWasmAbi` implementations for `&[T]`.
    pub(crate) fn is_slice_primitive(&self) -> bool {
        matches!(
            self,
            WbgType::Byte
                | WbgType::Octet
                | WbgType::Short
                | WbgType::UnsignedShort
                | WbgType::Long
                | WbgType::UnsignedLong
                | WbgType::Float
                | WbgType::UnrestrictedFloat
                | WbgType::Double
                | WbgType::UnrestrictedDouble
        )
    }

    /// Converts to syn type if possible.
    pub(crate) fn to_syn_type(
        &self,
        pos: TypePosition,
        legacy: bool,
        no_generics: bool,
    ) -> Result<Option<syn::Type>, TypeError> {
        // Inner position with no_generics doesn't make sense - inner types need generics
        assert!(
            !(pos.inner && no_generics),
            "Inner TypePosition must never be used when no_generics=true"
        );

        // Handle inner types (inside generics or callbacks): primitives → JS types
        // These must use JS-compatible types since they're erased at the ABI level
        if pos.inner {
            let js_sys = |name: &str| {
                let path = vec![rust_ident("js_sys"), rust_ident(name)];
                Some(leading_colon_path_ty(path))
            };

            return match self {
                // All numeric types become js_sys::Number for inner positions
                WbgType::Byte
                | WbgType::Octet
                | WbgType::Short
                | WbgType::UnsignedShort
                | WbgType::Long
                | WbgType::UnsignedLong
                | WbgType::LongLong
                | WbgType::UnsignedLongLong
                | WbgType::Float
                | WbgType::UnrestrictedFloat
                | WbgType::Double
                | WbgType::UnrestrictedDouble => Ok(js_sys("Number")),

                // Boolean becomes js_sys::Boolean
                WbgType::Boolean => Ok(js_sys("Boolean")),

                // Strings become js_sys::JsString
                WbgType::DomString | WbgType::ByteString | WbgType::UsvString => {
                    Ok(js_sys("JsString"))
                }

                // JsOption must use JsOption<T> for inner positions (not Rust Option<T>)
                WbgType::JsOption(wbg_type) => {
                    let inner = wbg_type.to_syn_type(pos, legacy, no_generics)?;
                    match inner {
                        Some(inner) => {
                            // JsOption<JsValue> isn't well-supported, just use JsValue
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
                            Ok(Some(js_option_ty(inner)))
                        }
                        None => Ok(None),
                    }
                }

                // For everything else, delegate to normal conversion but keep inner=true
                _ => self.to_syn_type_inner(pos, legacy, no_generics),
            };
        }

        self.to_syn_type_inner(pos, legacy, no_generics)
    }

    /// Inner implementation of to_syn_type (handles non-inner-specific logic)
    fn to_syn_type_inner(
        &self,
        pos: TypePosition,
        legacy: bool,
        no_generics: bool,
    ) -> Result<Option<syn::Type>, TypeError> {
        let externref = |ty| {
            Some(if pos.is_argument() && !pos.inner {
                shared_ref(ty, false)
            } else {
                ty
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
        match self {
            WbgType::Boolean => Ok(Some(ident_ty(raw_ident("bool")))),
            WbgType::Byte => Ok(Some(ident_ty(raw_ident("i8")))),
            WbgType::Octet => Ok(Some(ident_ty(raw_ident("u8")))),
            WbgType::Short => Ok(Some(ident_ty(raw_ident("i16")))),
            WbgType::UnsignedShort => Ok(Some(ident_ty(raw_ident("u16")))),
            WbgType::Long => Ok(Some(ident_ty(raw_ident("i32")))),
            WbgType::UnsignedLong => Ok(Some(ident_ty(raw_ident("u32")))),

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
            WbgType::LongLong | WbgType::UnsignedLongLong => Ok(Some(ident_ty(raw_ident("f64")))),

            WbgType::Float => Ok(Some(ident_ty(raw_ident("f32")))),
            WbgType::UnrestrictedFloat => Ok(Some(ident_ty(raw_ident("f32")))),
            WbgType::Double => Ok(Some(ident_ty(raw_ident("f64")))),
            WbgType::UnrestrictedDouble => Ok(Some(ident_ty(raw_ident("f64")))),
            WbgType::DomString | WbgType::ByteString | WbgType::UsvString => {
                if pos.is_argument() {
                    Ok(Some(shared_ref(ident_ty(raw_ident("str")), false)))
                } else {
                    // Return position: use String
                    let path = vec![
                        rust_ident("alloc"),
                        rust_ident("string"),
                        rust_ident("String"),
                    ];
                    Ok(Some(leading_colon_path_ty(path)))
                }
            }
            WbgType::Object => Ok(js_sys("Object")),
            WbgType::Symbol => Ok(js_sys("Symbol")),
            WbgType::Error => Ok(js_sys("Error")),
            WbgType::BigInt => Ok(js_sys("BigInt")),
            WbgType::Date => Ok(js_sys("Date")),

            WbgType::ArrayBuffer => Ok(js_sys("ArrayBuffer")),
            WbgType::DataView { .. } => Ok(js_sys("DataView")),
            WbgType::Int8Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("i8", pos, *immutable)))
                } else {
                    Ok(js_sys("Int8Array"))
                }
            }
            WbgType::Uint8Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("u8", pos, *immutable)))
                } else {
                    Ok(js_sys("Uint8Array"))
                }
            }
            WbgType::Uint8ClampedArray { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(clamped(array("u8", pos, *immutable))))
                } else {
                    Ok(js_sys("Uint8ClampedArray"))
                }
            }
            WbgType::Int16Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("i16", pos, *immutable)))
                } else {
                    Ok(js_sys("Int16Array"))
                }
            }
            WbgType::Uint16Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("u16", pos, *immutable)))
                } else {
                    Ok(js_sys("Uint16Array"))
                }
            }
            WbgType::Int32Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("i32", pos, *immutable)))
                } else {
                    Ok(js_sys("Int32Array"))
                }
            }
            WbgType::Uint32Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("u32", pos, *immutable)))
                } else {
                    Ok(js_sys("Uint32Array"))
                }
            }
            WbgType::Float32Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("f32", pos, *immutable)))
                } else {
                    Ok(js_sys("Float32Array"))
                }
            }
            WbgType::Float64Array { immutable, .. } => {
                if legacy || pos.is_return() {
                    Ok(Some(array("f64", pos, *immutable)))
                } else {
                    Ok(js_sys("Float64Array"))
                }
            }

            WbgType::ArrayBufferView { .. } | WbgType::BufferSource { .. } => Ok(js_sys("Object")),

            WbgType::JsOption(wbg_type) => {
                // Top-level position: use Rust Option<T>
                // (inner positions are handled in to_syn_type before calling to_syn_type_inner)
                let inner = wbg_type.to_syn_type(pos, legacy, no_generics)?;

                match inner {
                    Some(inner) => {
                        // `Option<JsValue>` isn't well-supported.
                        // If the inner type is JsValue, just return JsValue directly.
                        // That way `any?` just becomes `JsValue` and it's up to users
                        // to dispatch and/or create instances appropriately.
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
            WbgType::FrozenArray(_wbg_type)
            | WbgType::ObservableArray(_wbg_type)
            | WbgType::Sequence(_wbg_type) => {
                if pos.is_argument() && !pos.inner && no_generics {
                    // Legacy mode: no generics, fall back to &JsValue for arguments
                    Ok(js_value)
                } else if pos.is_argument() && !pos.inner {
                    // Generics mode: use &[T] for arguments
                    let inner = _wbg_type
                        .to_syn_type(pos.to_inner(), legacy, no_generics)?
                        .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                    Ok(Some(shared_ref(slice_ty(inner), false)))
                } else if no_generics {
                    // Backwards compat mode: no generics
                    Ok(js_sys("Array"))
                } else {
                    // New mode: with generics
                    let path = vec![rust_ident("js_sys"), rust_ident("Array")];
                    let base = leading_colon_path_ty(path);
                    let inner = _wbg_type
                        .to_syn_type(pos.to_inner(), legacy, no_generics)?
                        .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                    let generic_type = generic_ty(base, inner);
                    Ok(externref(generic_type))
                }
            }
            WbgType::Promise(_wbg_type) => {
                if no_generics {
                    // Backwards compat mode: no generics
                    Ok(js_sys("Promise"))
                } else {
                    // New mode: with generics
                    let path = vec![rust_ident("js_sys"), rust_ident("Promise")];
                    let base = leading_colon_path_ty(path);
                    let inner = _wbg_type
                        .to_syn_type(pos.to_inner(), legacy, no_generics)?
                        .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                    let generic_type = generic_ty(base, inner);
                    Ok(externref(generic_type))
                }
            }
            WbgType::Record(_wbg_type_from, _wbg_type_to) => {
                if no_generics {
                    // Backwards compat mode: no generics
                    Ok(js_sys("Object"))
                } else {
                    // New mode: with generics
                    // Map Record<DOMString, T> to Object<T>
                    let path = vec![rust_ident("js_sys"), rust_ident("Object")];
                    let base = leading_colon_path_ty(path);
                    let inner = _wbg_type_to
                        .to_syn_type(pos.to_inner(), legacy, no_generics)?
                        .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                    let generic_type = generic_ty(base, inner);
                    Ok(externref(generic_type))
                }
            }
            WbgType::Iterator(_wbg_type) => {
                if no_generics {
                    // Backwards compat mode: no generics
                    Ok(js_sys("Iterator"))
                } else {
                    // New mode: with generics
                    let path = vec![rust_ident("js_sys"), rust_ident("Iterator")];
                    let base = leading_colon_path_ty(path);
                    let inner = _wbg_type
                        .to_syn_type(pos.to_inner(), legacy, no_generics)?
                        .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                    let generic_type = generic_ty(base, inner);
                    Ok(externref(generic_type))
                }
            }
            WbgType::AsyncIterator(_wbg_type) => {
                if no_generics {
                    // Backwards compat mode: no generics
                    Ok(js_sys("AsyncIterator"))
                } else {
                    // New mode: with generics
                    let path = vec![rust_ident("js_sys"), rust_ident("AsyncIterator")];
                    let base = leading_colon_path_ty(path);
                    let inner = _wbg_type
                        .to_syn_type(pos.to_inner(), legacy, no_generics)?
                        .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                    let generic_type = generic_ty(base, inner);
                    Ok(externref(generic_type))
                }
            }
            WbgType::Callback {
                params,
                return_type,
            } => {
                if no_generics {
                    // Backwards compat mode: no generics, just plain Function
                    Ok(js_sys("Function"))
                } else {
                    // New mode: Function<fn(A1, A2, ...) -> R>
                    // Callback params and returns are always inner (they're erased to Function at ABI level)
                    let inner_arg = TypePosition {
                        direction: Direction::Argument,
                        inner: true,
                    };
                    let inner_ret = TypePosition {
                        direction: Direction::Return,
                        inner: true,
                    };

                    // Convert up to 9 parameters
                    let mut param_types = Vec::new();
                    for param in params.iter().take(9) {
                        let ty = param
                            .to_syn_type(inner_arg, legacy, false)?
                            .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                        param_types.push(ty);
                    }

                    // Return type defaults to Undefined for void functions
                    let ret_ty: syn::Type = match return_type {
                        Some(rt) => rt
                            .to_syn_type(inner_ret, legacy, false)?
                            .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue)),
                        None => parse_quote!(::js_sys::Undefined),
                    };

                    // Build Function<fn(A1, A2, ...) -> R>
                    let ty: syn::Type = if param_types.is_empty() {
                        parse_quote! {
                            ::js_sys::Function<fn() -> #ret_ty>
                        }
                    } else {
                        parse_quote! {
                            ::js_sys::Function<fn(#(#param_types),*) -> #ret_ty>
                        }
                    };

                    Ok(externref(ty))
                }
            }
            WbgType::ArrayTuple(_wbg_type_a, _wbg_type_b) => {
                // ArrayTuple<(K, V)> represents a JS array [k, v] pair
                let path = vec![rust_ident("js_sys"), rust_ident("ArrayTuple")];
                let base = leading_colon_path_ty(path);
                let inner_a = _wbg_type_a
                    .to_syn_type(pos.to_inner(), legacy, no_generics)?
                    .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                let inner_b = _wbg_type_b
                    .to_syn_type(pos.to_inner(), legacy, no_generics)?
                    .unwrap_or_else(|| parse_quote!(::wasm_bindgen::JsValue));
                // ArrayTuple takes a single tuple type parameter: ArrayTuple<(K, V)>
                let tuple_ty: syn::Type = parse_quote!((#inner_a, #inner_b));
                let generic_type = generic_ty(base, tuple_ty);
                Ok(externref(generic_type))
            }
            WbgType::Union(wbg_types) => {
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
                if wbg_types.iter().all(|wbg_type| {
                    matches!(
                        wbg_type,
                        WbgType::Identifier {
                            ty: IdentifierType::Interface(..),
                            ..
                        }
                    )
                }) {
                    WbgType::Object.to_syn_type(pos, legacy, no_generics)
                } else {
                    WbgType::Any.to_syn_type(pos, legacy, no_generics)
                }
            }

            WbgType::Any => Ok(js_value),
            WbgType::Undefined => {
                // Undefined is only used in inner positions (callback returns, inside generics)
                // For regular method returns, void/undefined means no return type (None)
                if pos.inner {
                    Ok(Some(parse_quote!(::js_sys::Undefined)))
                } else {
                    Ok(None)
                }
            }
            WbgType::Identifier { ty, .. } => ty.to_syn_type(pos, legacy, no_generics),
            WbgType::UnknownIdentifier(name) => {
                log::warn!("Cannot convert unknown identifier: {name}");
                Err(TypeError::CannotConvert)
            }
        }
    }

    /// Flattens unions recursively.
    ///
    /// Works similarly to [flattened union member types],
    /// but also flattens unions inside generics of other types.
    ///
    /// When `next_unstable` is true, only the generic variant is returned for types
    /// that have multiple representations (e.g., callback interfaces only return
    /// the typed callback, not the dict type).
    ///
    /// [flattened union member types]: https://heycam.github.io/webidl/#dfn-flattened-union-member-types
    pub(crate) fn flatten(&self, attrs: Option<&ExtendedAttributeList<'_>>) -> Vec<Self> {
        match self {
            WbgType::JsOption(wbg_type) => wbg_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(WbgType::JsOption)
                .collect(),
            WbgType::FrozenArray(wbg_type) => wbg_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(WbgType::FrozenArray)
                .collect(),
            WbgType::Sequence(wbg_type) => wbg_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(WbgType::Sequence)
                .collect(),
            WbgType::Promise(wbg_type) => wbg_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(WbgType::Promise)
                .collect(),
            WbgType::Iterator(wbg_type) => wbg_type
                .flatten(attrs)
                .into_iter()
                .map(Box::new)
                .map(WbgType::Iterator)
                .collect(),
            WbgType::ArrayTuple(wbg_type_a, wbg_type_b) => {
                let mut wbg_types = Vec::new();
                for wbg_type_a in wbg_type_a.flatten(attrs) {
                    for wbg_type_b in wbg_type_b.flatten(attrs) {
                        wbg_types.push(WbgType::ArrayTuple(
                            Box::new(wbg_type_a.clone()),
                            Box::new(wbg_type_b.clone()),
                        ));
                    }
                }
                wbg_types
            }
            WbgType::Record(wbg_type_from, wbg_type_to) => {
                let mut wbg_types = Vec::new();
                for wbg_type_from in wbg_type_from.flatten(attrs) {
                    for wbg_type_to in wbg_type_to.flatten(attrs) {
                        wbg_types.push(WbgType::Record(
                            Box::new(wbg_type_from.clone()),
                            Box::new(wbg_type_to.clone()),
                        ));
                    }
                }
                wbg_types
            }
            WbgType::Union(wbg_types) => wbg_types
                .iter()
                .flat_map(|wbg_type| wbg_type.flatten(attrs))
                .collect(),
            WbgType::ArrayBufferView {
                allow_shared,
                immutable,
            } => {
                let view = WbgType::ArrayBufferView {
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
                    WbgType::Identifier {
                        name: "Uint8Array",
                        ty: IdentifierType::Uint8Slice {
                            allow_shared: *allow_shared,
                            immutable: *immutable,
                        },
                    },
                    WbgType::Uint8Array {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                ]
            }
            WbgType::BufferSource {
                allow_shared,
                immutable,
            } => vec![
                WbgType::BufferSource {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
                WbgType::Identifier {
                    name: "Uint8Array",
                    ty: IdentifierType::Uint8Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Uint8Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::LongLong => vec![WbgType::Long, WbgType::Double],
            WbgType::UnsignedLongLong => vec![WbgType::UnsignedLong, WbgType::Double],
            WbgType::Int8Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Int8Array",
                    ty: IdentifierType::Int8Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Int8Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Uint8Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Uint8Array",
                    ty: IdentifierType::Uint8Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Uint8Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Uint8ClampedArray {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Uint8ClampedArray",
                    ty: IdentifierType::Uint8ClampedSlice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Uint8ClampedArray {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Int16Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Int16Array",
                    ty: IdentifierType::Int16Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Int16Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Uint16Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Uint16Array",
                    ty: IdentifierType::Uint16Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Uint16Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Int32Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Int32Array",
                    ty: IdentifierType::Int32Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Int32Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Uint32Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Uint32Array",
                    ty: IdentifierType::Uint32Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Uint32Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Float32Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Float32Array",
                    ty: IdentifierType::Float32Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Float32Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            WbgType::Float64Array {
                allow_shared,
                immutable,
            } => vec![
                WbgType::Identifier {
                    name: "Float64Array",
                    ty: IdentifierType::Float64Slice {
                        allow_shared: *allow_shared,
                        immutable: *immutable,
                    },
                },
                WbgType::Float64Array {
                    allow_shared: *allow_shared,
                    immutable: *immutable,
                },
            ],
            wbg_type @ WbgType::Identifier {
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
                            WbgType::Identifier {
                                name: identifier,
                                ty: IdentifierType::Callback,
                            },
                            WbgType::Identifier {
                                name: identifier,
                                ty: IdentifierType::CallbackInterface {
                                    name,
                                    single_function: false,
                                },
                            },
                        ]
                    }
                    IdentifierType::UnsignedLongLong => WbgType::UnsignedLongLong.flatten(attrs),
                    IdentifierType::AllowSharedBufferSource { immutable } => {
                        WbgType::BufferSource {
                            allow_shared: true,
                            immutable: *immutable,
                        }
                        .flatten(attrs)
                    }
                    _ => vec![wbg_type.clone()],
                }
            }
            wbg_type => vec![wbg_type.clone()],
        }
    }
}

impl IdentifierType<'_> {
    /// Converts to syn type if possible.
    pub(crate) fn to_syn_type(
        &self,
        pos: TypePosition,
        legacy: bool,
        no_generics: bool,
    ) -> Result<Option<syn::Type>, TypeError> {
        let externref = |ty| {
            Some(if pos.is_argument() && !pos.inner {
                shared_ref(ty, false)
            } else {
                ty
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
            IdentifierType::CallbackInterface {
                name,
                single_function,
            } => {
                // In next_unstable mode (!no_generics), non-single-function callback
                // interfaces cannot be represented, so return None to skip the method
                if !no_generics && !single_function {
                    return Ok(None);
                }
                let ty = ident_ty(rust_ident(camel_case_ident(name).as_str()));
                Ok(externref(ty))
            }
            IdentifierType::Interface(name) | IdentifierType::Dictionary(name) => {
                let ty = ident_ty(rust_ident(camel_case_ident(name).as_str()));
                Ok(externref(ty))
            }
            IdentifierType::Enum(name) => {
                if pos.inner {
                    // String enums are repr(u32) so they can't be used directly
                    // in inner/slice positions. Use JsString instead, which is
                    // repr(transparent) over JsValue.
                    Ok(js_sys("JsString"))
                } else {
                    Ok(Some(ident_ty(rust_ident(camel_case_ident(name).as_str()))))
                }
            }
            IdentifierType::UnsignedLongLong => {
                WbgType::UnsignedLongLong.to_syn_type(pos, legacy, no_generics)
            }
            IdentifierType::AllowSharedBufferSource { immutable } => WbgType::BufferSource {
                allow_shared: true,
                immutable: *immutable,
            }
            .to_syn_type(pos, legacy, no_generics),
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
fn wbg_type_flatten_test() {
    use self::IdentifierType::*;
    use self::WbgType::*;

    assert_eq!(
        Union(vec![
            WbgType::id("Node", Interface("Node")),
            Union(vec![
                Sequence(Box::new(Long),),
                WbgType::id("Event", Interface("Event"))
            ]),
            JsOption(Box::new(Union(vec![
                WbgType::id("XMLHttpRequest", Interface("XMLHttpRequest")),
                DomString,
            ])),),
            Sequence(Box::new(Union(vec![
                Sequence(Box::new(Double),),
                WbgType::id("NodeList", Interface("NodeList")),
            ])),),
        ])
        .flatten(None),
        vec![
            WbgType::id("Node", Interface("Node")),
            Sequence(Box::new(Long)),
            WbgType::id("Event", Interface("Event")),
            JsOption(Box::new(WbgType::id(
                "XMLHttpRequest",
                Interface("XMLHttpRequest")
            ))),
            JsOption(Box::new(DomString)),
            Sequence(Box::new(Sequence(Box::new(Double)))),
            Sequence(Box::new(WbgType::id("NodeList", Interface("NodeList")))),
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
