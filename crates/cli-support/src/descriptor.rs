use wasm_bindgen_shared::identifier::is_valid_ident;

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

