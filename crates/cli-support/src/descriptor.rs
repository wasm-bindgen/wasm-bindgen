use std::char;

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
    RustStruct(String),
    Char,
    Option(Box<Descriptor>),
    Result(Box<Descriptor>),
    Unit,
    NonNull,
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
    pub dtor_idx: u32,
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

    fn _decode(data: &mut &[u32], clamped: bool) -> Descriptor {
        match get(data) {
            I8 => Descriptor::I8,
            I16 => Descriptor::I16,
            I32 => Descriptor::I32,
            I64 => Descriptor::I64,
            I128 => Descriptor::I128,
            U8 if clamped => Descriptor::ClampedU8,
            U8 => Descriptor::U8,
            U16 => Descriptor::U16,
            U32 => Descriptor::U32,
            U64 => Descriptor::U64,
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
            RESULT => Descriptor::Result(Box::new(Descriptor::_decode(data, clamped))),
            CACHED_STRING => Descriptor::CachedString,
            STRING => Descriptor::String,
            EXTERNREF => Descriptor::Externref,
            ENUM => {
                let name = get_string(data);
                let hole = get(data);
                Descriptor::Enum { name, hole }
            }
            STRING_ENUM => {
                let name = get_string(data);
                let variant_count = get(data);
                let invalid = variant_count;
                let hole = variant_count + 1;
                Descriptor::StringEnum {
                    name,
                    invalid,
                    hole,
                }
            }
            RUST_STRUCT => {
                let name = get_string(data);
                Descriptor::RustStruct(name)
            }
            NAMED_EXTERNREF => {
                let name = get_string(data);
                Descriptor::NamedExternref(name)
            }
            CHAR => Descriptor::Char,
            UNIT => Descriptor::Unit,
            CLAMPED => Descriptor::_decode(data, true),
            NONNULL => Descriptor::NonNull,
            other => panic!("unknown descriptor: {other}"),
        }
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
            Descriptor::I64 => Some(VectorKind::I64),
            Descriptor::U8 => Some(VectorKind::U8),
            Descriptor::ClampedU8 => Some(VectorKind::ClampedU8),
            Descriptor::U16 => Some(VectorKind::U16),
            Descriptor::U32 => Some(VectorKind::U32),
            Descriptor::U64 => Some(VectorKind::U64),
            Descriptor::F32 => Some(VectorKind::F32),
            Descriptor::F64 => Some(VectorKind::F64),
            Descriptor::Externref => Some(VectorKind::Externref),
            Descriptor::NamedExternref(ref name) => Some(VectorKind::NamedExternref(name.clone())),
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
        let dtor_idx = get(data);
        let mutable = match get(data) {
            0 => false,
            1 => true,
            other => panic!("expected bool value, got {other}"),
        };
        assert_eq!(get(data), FUNCTION);
        Closure {
            dtor_idx,
            mutable,
            function: Function::decode(data),
        }
    }
}

impl Function {
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
