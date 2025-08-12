use wasm_bindgen::prelude::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32,
}

#[automatically_derived]
impl wasm_bindgen::__rt::marker::SupportsConstructor for Counter {}
#[automatically_derived]
impl wasm_bindgen::__rt::marker::SupportsInstanceProperty for Counter {}
#[automatically_derived]
impl wasm_bindgen::__rt::marker::SupportsStaticProperty for Counter {}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribe for Counter {
    fn describe() {
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(7u32);
        inform(67u32);
        inform(111u32);
        inform(117u32);
        inform(110u32);
        inform(116u32);
        inform(101u32);
        inform(114u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::IntoWasmAbi for Counter {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::WasmRefCell;
        Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::FromWasmAbi for Counter {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::core::result::Result::{Err, Ok};
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<Counter>;
        assert_not_null(ptr);
        let rc = Rc::from_raw(ptr);
        match Rc::try_unwrap(rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => wasm_bindgen::throw_str(
                "attempted to take ownership of Rust value while it was borrowed",
            ),
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::core::convert::From<Counter> for wasm_bindgen::JsValue {
    fn from(value: Counter) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        extern "C" {
            fn __wbg_counter_new(ptr: u32) -> u32;
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_counter_new(ptr),
            )
        }
    }
}
#[automatically_derived]
const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_counter_free(ptr: u32, allow_delayed: u32) {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        if allow_delayed != 0 {
            let ptr = ptr as *mut wasm_bindgen::__rt::WasmRefCell<Counter>;
            wasm_bindgen::__rt::assert_not_null(ptr);
            drop(Rc::from_raw(ptr));
        } else {
            let _ = <Counter as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr);
        }
    }
};
#[automatically_derived]
impl<const LONG_LIVED: bool> wasm_bindgen::convert::ArgFromWasmAbi<LONG_LIVED> for &Counter {
    type Anchor = wasm_bindgen::__rt::RcRef<Counter>;
    type SameButOver<'a> = &'a Counter;
    fn arg_from_anchor(anchor: &mut Self::Anchor) -> Self::SameButOver<'_> {
        anchor
    }
}
#[automatically_derived]
impl<const LONG_LIVED: bool> wasm_bindgen::convert::ArgFromWasmAbi<LONG_LIVED> for &mut Counter {
    type Anchor = wasm_bindgen::__rt::RcRefMut<Counter>;
    type SameButOver<'a> = &'a mut Counter;
    fn arg_from_anchor(anchor: &mut Self::Anchor) -> Self::SameButOver<'_> {
        anchor
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionIntoWasmAbi for Counter {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionFromWasmAbi for Counter {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::TryFromJsValue for Counter {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        extern "C" {
            fn __wbg_counter_unwrap(ptr: u32) -> u32;
        }
        let ptr = unsafe { __wbg_counter_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::core::result::Result::Err(value)
        } else {
            #[allow(clippy::mem_forget)]
            wasm_bindgen::__rt::core::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::core::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribeVector for Counter {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(7u32);
        inform(67u32);
        inform(111u32);
        inform(117u32);
        inform(110u32);
        inform(116u32);
        inform(101u32);
        inform(114u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorIntoWasmAbi for Counter {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(vector: wasm_bindgen::__rt::alloc::boxed::Box<[Counter]>) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorFromWasmAbi for Counter {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(js: Self::Abi) -> wasm_bindgen::__rt::alloc::boxed::Box<[Counter]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::VectorIntoJsValue for Counter {
    fn vector_into_jsvalue(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[Counter]>,
    ) -> wasm_bindgen::JsValue {
        {
            #[automatically_derived]
            const _: () = {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C" fn __wbindgen_describe___wbg_cast_18eb52d383fdead5() {
                    use wasm_bindgen::describe::*;
                    ::wasm_bindgen::__rt::link_mem_intrinsics();
                    inform(FUNCTION);
                    inform(0);
                    inform(1u32);
                    <wasm_bindgen::__rt::alloc::boxed::Box<[Counter]> as WasmDescribe>::describe();
                    <wasm_bindgen::JsValue as WasmDescribe>::describe();
                    <wasm_bindgen::JsValue as WasmDescribe>::describe();
                }
            };
            #[allow(nonstandard_style)]
            #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
            /// Foobar.
            fn __wbindgen_cast(
                value: wasm_bindgen::__rt::alloc::boxed::Box<[Counter]>,
            ) -> wasm_bindgen::JsValue {
                #[link(wasm_import_module = "__wbindgen_placeholder__")]
                extern "C" {
                    fn __wbg_cast_18eb52d383fdead5(
                        value_1: <<wasm_bindgen::__rt::alloc::boxed::Box<
                            [Counter],
                        > as ::wasm_bindgen::convert::IntoWasmAbi>::Abi as ::wasm_bindgen::convert::WasmAbi>::Prim1,
                        value_2: <<wasm_bindgen::__rt::alloc::boxed::Box<
                            [Counter],
                        > as ::wasm_bindgen::convert::IntoWasmAbi>::Abi as ::wasm_bindgen::convert::WasmAbi>::Prim2,
                        value_3: <<wasm_bindgen::__rt::alloc::boxed::Box<
                            [Counter],
                        > as ::wasm_bindgen::convert::IntoWasmAbi>::Abi as ::wasm_bindgen::convert::WasmAbi>::Prim3,
                        value_4: <<wasm_bindgen::__rt::alloc::boxed::Box<
                            [Counter],
                        > as ::wasm_bindgen::convert::IntoWasmAbi>::Abi as ::wasm_bindgen::convert::WasmAbi>::Prim4,
                    ) -> ::wasm_bindgen::convert::WasmRet<
                        <wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi>::Abi,
                    >;
                }
                unsafe {
                    let _ret = {
                        let value = <wasm_bindgen::__rt::alloc::boxed::Box<
                            [Counter],
                        > as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                        let (value_1, value_2, value_3, value_4) = <<wasm_bindgen::__rt::alloc::boxed::Box<
                            [Counter],
                        > as ::wasm_bindgen::convert::IntoWasmAbi>::Abi as ::wasm_bindgen::convert::WasmAbi>::split(
                            value,
                        );
                        __wbg_cast_18eb52d383fdead5(value_1, value_2, value_3, value_4)
                    };
                    <wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        _ret.join(),
                    )
                }
            }
            #[automatically_derived]
            const _: () = {
                use wasm_bindgen::__rt::{flat_byte_slices, flat_len};
                static _INCLUDED_FILES: &[&str] = &[];
                const _ENCODED_BYTES: &[u8] = {
                    const _CHUNK_SLICES: [&[u8]; 1usize] = [
                        b"\0\0\x01\0\0\0\x1b__wbg_cast_18eb52d383fdead5\0\0\0\0\x01\x01\x05value\0\0\0\n/* cast */\x01\x01\0\0\0\0\0\0\0\x15char-12d476aa1d7cc62e\0\0",
                    ];
                    #[allow(long_running_const_eval)]
                    const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
                    #[allow(long_running_const_eval)]
                    const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
                    const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
                    const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
                    #[allow(long_running_const_eval)]
                    const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] =
                        flat_byte_slices([&_LEN_BYTES, &_CHUNKS]);
                    &_ENCODED_BYTES
                };
                const _PREFIX_JSON_BYTES: &[u8] =
                    b"<\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100 (206d0254f)\"}";
                const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
                const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
                const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
                #[link_section = "__wasm_bindgen_unstable"]
                #[allow(long_running_const_eval)]
                static _GENERATED: [u8; _LEN] =
                    flat_byte_slices([_PREFIX_JSON_BYTES, _ENCODED_BYTES]);
            };
            __wbindgen_cast(vector)
        }
    }
}
#[automatically_derived]
const _: () = {
    use wasm_bindgen::__rt::{flat_byte_slices, flat_len};
    static _INCLUDED_FILES: &[&str] = &[];
    const _ENCODED_BYTES: &[u8] = {
        const _CHUNK_SLICES: [&[u8]; 1usize] =
            [b"\0\0\0\x01\x07Counter\0\0\0\x01\0\0\0\x15char-12d476aa1d7cc62e\0\0"];
        #[allow(long_running_const_eval)]
        const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
        #[allow(long_running_const_eval)]
        const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
        const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
        const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
        #[allow(long_running_const_eval)]
        const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([&_LEN_BYTES, &_CHUNKS]);
        &_ENCODED_BYTES
    };
    const _PREFIX_JSON_BYTES: &[u8] =
        b"<\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100 (206d0254f)\"}";
    const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
    const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
    const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
    #[link_section = "__wasm_bindgen_unstable"]
    #[allow(long_running_const_eval)]
    static _GENERATED: [u8; _LEN] = flat_byte_slices([_PREFIX_JSON_BYTES, _ENCODED_BYTES]);
};
impl Counter {
    pub fn update_key(&mut self, key: char) {
        #[automatically_derived]
        const _: () = {
            #[export_name = "counter_update_key"]
            pub unsafe extern "C" fn __wasm_bindgen_generated_Counter_update_key(
                me: u32,
                arg0_1: <<<char as wasm_bindgen::convert::ArgFromWasmAbi<
                    false,
                >>::Anchor as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg0_2: <<<char as wasm_bindgen::convert::ArgFromWasmAbi<
                    false,
                >>::Anchor as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg0_3: <<<char as wasm_bindgen::convert::ArgFromWasmAbi<
                    false,
                >>::Anchor as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg0_4: <<<char as wasm_bindgen::convert::ArgFromWasmAbi<
                    false,
                >>::Anchor as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<<() as wasm_bindgen::convert::ReturnWasmAbi>::Abi>
            {
                const _: () = {};
                let _ret = {
                    let mut me_anchor = unsafe { wasm_bindgen::convert::FromWasmAbi::from_abi(me) };
                    let me =
                        <Counter as wasm_bindgen::convert::ArgFromWasmAbi<false>>::arg_from_anchor(
                            &mut me_anchor,
                        );
                    let mut arg0_anchor = unsafe {
                        <<char as wasm_bindgen::convert::ArgFromWasmAbi<
                            false,
                        >>::Anchor as wasm_bindgen::convert::FromWasmAbi>::from_abi_prims(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        )
                    };
                    let arg0 =
                        <char as wasm_bindgen::convert::ArgFromWasmAbi<false>>::arg_from_anchor(
                            &mut arg0_anchor,
                        );
                    let _ret = me.update_key(arg0);
                    _ret
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        #[automatically_derived]
        const _: () = {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe_counter_update_key() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(1u32);
                <char as WasmDescribe>::describe();
                <() as WasmDescribe>::describe();
                <() as WasmDescribe>::describe();
            }
        };
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::__rt::{flat_byte_slices, flat_len};
            static _INCLUDED_FILES: &[&str] = &[];
            const _ENCODED_BYTES: &[u8] = {
                const _CHUNK_SLICES: [&[u8]; 1usize] = [
                    b"\x01\x01\x07Counter\0\0\x01\x03key\0\0\0\nupdate_key\x01\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\x15char-12d476aa1d7cc62e\0\0",
                ];
                #[allow(long_running_const_eval)]
                const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
                #[allow(long_running_const_eval)]
                const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
                const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
                const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
                #[allow(long_running_const_eval)]
                const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] =
                    flat_byte_slices([&_LEN_BYTES, &_CHUNKS]);
                &_ENCODED_BYTES
            };
            const _PREFIX_JSON_BYTES: &[u8] =
                b"<\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100 (206d0254f)\"}";
            const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
            const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
            const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
            #[link_section = "__wasm_bindgen_unstable"]
            #[allow(long_running_const_eval)]
            static _GENERATED: [u8; _LEN] = flat_byte_slices([_PREFIX_JSON_BYTES, _ENCODED_BYTES]);
        };
        self.key = key;
    }
}
#[automatically_derived]
const _: () = {
    use wasm_bindgen::__rt::{flat_byte_slices, flat_len};
    static _INCLUDED_FILES: &[&str] = &[];
    const _ENCODED_BYTES: &[u8] = {
        const _CHUNK_SLICES: [&[u8]; 1usize] = [b"\0\0\0\0\0\0\0\x15char-12d476aa1d7cc62e\0\0"];
        #[allow(long_running_const_eval)]
        const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
        #[allow(long_running_const_eval)]
        const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
        const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
        const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
        #[allow(long_running_const_eval)]
        const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([&_LEN_BYTES, &_CHUNKS]);
        &_ENCODED_BYTES
    };
    const _PREFIX_JSON_BYTES: &[u8] =
        b"<\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100 (206d0254f)\"}";
    const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
    const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
    const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
    #[link_section = "__wasm_bindgen_unstable"]
    #[allow(long_running_const_eval)]
    static _GENERATED: [u8; _LEN] = flat_byte_slices([_PREFIX_JSON_BYTES, _ENCODED_BYTES]);
};
