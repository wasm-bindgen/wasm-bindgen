#![allow(clippy::all)]

use serde::{Deserialize, Serialize};
use wasm_bindgen::__rt::*;
use wasm_bindgen::convert::*;
use wasm_bindgen::describe::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Serialize, Deserialize)]
pub struct SomeType {
    pub prop: String,
}

#[derive(Serialize, Deserialize)]
pub struct SomeGenericType<T> {
    pub field: T,
}

#[derive(Serialize, Deserialize)]
pub struct OtherGenericType<T, E> {
    pub field1: T,
    pub field2: E,
}

// some error type
pub struct Error;

#[wasm_bindgen]
pub struct TestStruct;
#[wasm_bindgen]
impl TestStruct {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TestStruct
    }
    #[wasm_bindgen]
    pub async fn method1(
        _arg1: SomeGenericType<u8>,
        _arg2: OtherGenericType<bool, String>,
        _arg3: &OtherGenericType<u64, SomeType>,
    ) -> Result<OtherGenericType<Vec<SomeGenericType<Option<u8>>>, Option<Vec<String>>>, Error>
    {
        Ok(OtherGenericType {
            field1: vec![SomeGenericType { field: Some(0) }],
            field2: Some(vec![String::new()]),
        })
    }
    #[wasm_bindgen(getter = "someProperty")]
    pub fn method2(
        &self,
    ) -> Result<OtherGenericType<Option<Vec<SomeGenericType<Option<Vec<u8>>>>>, ()>, Error> {
        Ok(OtherGenericType {
            field1: Some(vec![SomeGenericType {
                field: Some(vec![0]),
            }]),
            field2: (),
        })
    }
}

#[wasm_bindgen(js_name = "someFn")]
pub async fn some_fn(
    _arg1: &OtherGenericType<SomeGenericType<Vec<u16>>, Option<Vec<String>>>,
    _arg2: Vec<SomeGenericType<Vec<SomeType>>>,
) -> Result<OtherGenericType<Option<Vec<SomeGenericType<Option<u8>>>>, Option<Vec<String>>>, Error>
{
    Ok(OtherGenericType {
        field1: Some(vec![SomeGenericType { field: Some(0) }]),
        field2: Some(vec![String::new()]),
    })
}

#[wasm_bindgen(js_name = "someOtherFn")]
pub fn some_other_fn(
    _arg1: SomeGenericType<Option<SomeType>>,
) -> Option<Vec<OtherGenericType<SomeType, SomeGenericType<Option<SomeType>>>>> {
    Some(vec![OtherGenericType {
        field1: SomeType {
            prop: String::new(),
        },
        field2: SomeGenericType {
            field: Some(SomeType {
                prop: String::new(),
            }),
        },
    }])
}

#[wasm_bindgen(js_name = "anotherFn")]
pub async fn another_fn(
    _arg1: OtherGenericType<SomeType, Vec<u32>>,
) -> OtherGenericType<SomeType, SomeGenericType<Option<SomeType>>> {
    OtherGenericType {
        field1: SomeType {
            prop: String::new(),
        },
        field2: SomeGenericType {
            field: Some(SomeType {
                prop: String::new(),
            }),
        },
    }
}

// ---
// section below is just implementing wasm bindgen traits and ttypescript def for test structs

// a helper macro to impl wasm traits for a given type
macro_rules! impl_wasm_traits {
    ($type_name:ident $(< $($generics:ident),+ >)?) => {
        impl$(<$($generics),+>)? IntoWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            fn into_abi(self) -> Self::Abi {
                serde_wasm_bindgen::to_value(&self)
                    .map(<JsValue as JsCast>::unchecked_from_js)
                    .unwrap_throw()
                    .into_abi()
            }
        }
        impl$(<$($generics),+>)? OptionIntoWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            fn none() -> Self::Abi {
                0
            }
        }
        impl$(<$($generics),+>)? FromWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            unsafe fn from_abi(js: Self::Abi) -> Self {
                serde_wasm_bindgen::from_value(JsValue::from_abi(js).into()).unwrap_throw()
            }
        }
        impl$(<$($generics),+>)? OptionFromWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            fn is_none(js: &Self::Abi) -> bool {
                *js == 0
            }
        }
        impl$(<$($generics),+>)? RefFromWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = Box<Self>;
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                Box::new(<Self as FromWasmAbi>::from_abi(js))
            }
        }
        impl$(<$($generics),+>)? LongRefFromWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = Box<Self>;
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                Box::new(<Self as FromWasmAbi>::from_abi(js))
            }
        }
        impl$(<$($generics),+>)? VectorIntoWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Abi = <Box<[JsValue]> as IntoWasmAbi>::Abi;
            fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
                js_value_vector_into_abi(vector)
            }
        }
        impl$(<$($generics),+>)? VectorFromWasmAbi for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Abi = <Box<[JsValue]> as FromWasmAbi>::Abi;
            unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
                js_value_vector_from_abi(js)
            }
        }
        impl$(<$($generics),+>)? WasmDescribeVector for $type_name$(<$($generics),+>)? {
            fn describe_vector() {
                inform(VECTOR);
                <Self as WasmDescribe>::describe();
            }
        }
        impl$(<$($generics),+>)? From<$type_name$(<$($generics),+>)?> for JsValue
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            fn from(value: $type_name$(<$($generics),+>)?) -> Self {
                serde_wasm_bindgen::to_value(&value).unwrap_throw()
            }
        }
        impl$(<$($generics),+>)? TryFromJsValue for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            type Error = serde_wasm_bindgen::Error;
            fn try_from_js_value(value: JsValue) -> Result<Self, Self::Error> {
                serde_wasm_bindgen::from_value(value)
            }
        }
        impl$(<$($generics),+>)? VectorIntoJsValue for $type_name$(<$($generics),+>)?
        $(where $($generics: serde::Serialize + for<'de> serde::Deserialize<'de>, )+ )?
        {
            fn vector_into_jsvalue(vector: Box<[Self]>) -> JsValue {
                js_value_vector_into_jsvalue(vector)
            }
        }
    };
}

// impl wasm traits for test types
impl_wasm_traits!(SomeType);
impl WasmDescribe for SomeType {
    fn describe() {
        inform(NAMED_EXTERNREF);
        inform(8u32);
        inform(83u32);
        inform(111u32);
        inform(109u32);
        inform(101u32);
        inform(84u32);
        inform(121u32);
        inform(112u32);
        inform(101u32);
    }
}
#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_CONTENT: &str = "export interface SomeType {
    prop: string;
}";

impl_wasm_traits!(SomeGenericType<T>);
impl<T> WasmDescribe for SomeGenericType<T> {
    fn describe() {
        inform(NAMED_EXTERNREF);
        inform(15u32);
        inform(83u32);
        inform(111u32);
        inform(109u32);
        inform(101u32);
        inform(71u32);
        inform(101u32);
        inform(110u32);
        inform(101u32);
        inform(114u32);
        inform(105u32);
        inform(99u32);
        inform(84u32);
        inform(121u32);
        inform(112u32);
        inform(101u32);
    }
}
#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_CONTENT: &str = "export interface SomeGenericType<T> {
    field: T;
}";

impl_wasm_traits!(OtherGenericType<T, E>);
impl<T, E> WasmDescribe for OtherGenericType<T, E> {
    fn describe() {
        inform(NAMED_EXTERNREF);
        inform(16u32);
        inform(79u32);
        inform(116u32);
        inform(104u32);
        inform(101u32);
        inform(114u32);
        inform(71u32);
        inform(101u32);
        inform(110u32);
        inform(101u32);
        inform(114u32);
        inform(105u32);
        inform(99u32);
        inform(84u32);
        inform(121u32);
        inform(112u32);
        inform(101u32);
    }
}
#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_CONTENT: &str = "export interface OtherGenericType<T, E> {
    field1: T;
    field2: E;
}";

impl From<Error> for JsValue {
    fn from(_value: Error) -> Self {
        JsValue::from(JsError::new("some error msg"))
    }
}
