use crate::ast;
use crate::encode;
use crate::encode::EncodeChunk;
use crate::generics::{self, generic_to_concrete};
use crate::hash::ShortHash;
use crate::Diagnostic;
use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;
use quote::quote_spanned;
use quote::{quote, ToTokens};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::{Attribute, Meta, MetaList};
use wasm_bindgen_shared as shared;

/// A trait for converting AST structs into Tokens and adding them to a TokenStream,
/// or providing a diagnostic if conversion fails.
pub trait TryToTokens {
    /// Attempt to convert a `Self` into tokens and add it to the `TokenStream`
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic>;

    /// Attempt to convert a `Self` into a new `TokenStream`
    fn try_to_token_stream(&self) -> Result<TokenStream, Diagnostic> {
        let mut tokens = TokenStream::new();
        self.try_to_tokens(&mut tokens)?;
        Ok(tokens)
    }
}

impl TryToTokens for ast::Program {
    // Generate wrappers for all the items that we've found
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        let mut errors = Vec::new();
        for export in self.exports.iter() {
            if let Err(e) = export.try_to_tokens(tokens) {
                errors.push(e);
            }
        }
        for s in self.structs.iter() {
            s.to_tokens(tokens);
        }
        let mut types = HashMap::new();
        for i in self.imports.iter() {
            if let ast::ImportKind::Type(t) = &i.kind {
                types.insert(t.rust_name.to_string(), t.rust_name.clone());
            }
        }
        for i in self.imports.iter() {
            DescribeImport {
                kind: &i.kind,
                wasm_bindgen: &self.wasm_bindgen,
            }
            .try_to_tokens(tokens)?;

            // If there is a js namespace, check that name isn't a type. If it is,
            // this import might be a method on that type.
            if let Some(nss) = &i.js_namespace {
                // When the namespace is `A.B`, the type name should be `B`.
                if let Some(ns) = nss.last().and_then(|t| types.get(t)) {
                    if i.kind.fits_on_impl() {
                        let kind = match i.kind.try_to_token_stream() {
                            Ok(kind) => kind,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        };
                        (quote! {
                            #[automatically_derived]
                            impl #ns { #kind }
                        })
                        .to_tokens(tokens);
                        continue;
                    }
                }
            }

            if let Err(e) = i.kind.try_to_tokens(tokens) {
                errors.push(e);
            }
        }
        for e in self.enums.iter() {
            e.to_tokens(tokens);
        }

        Diagnostic::from_vec(errors)?;

        // Generate a static which will eventually be what lives in a custom section
        // of the Wasm executable. For now it's just a plain old static, but we'll
        // eventually have it actually in its own section.

        // See comments in `crates/cli-support/src/lib.rs` about what this
        // `schema_version` is.
        let prefix_json = format!(
            r#"{{"schema_version":"{}","version":"{}"}}"#,
            shared::SCHEMA_VERSION,
            shared::version()
        );

        let wasm_bindgen = &self.wasm_bindgen;

        let encoded = encode::encode(self)?;

        let encoded_chunks: Vec<_> = encoded
            .custom_section
            .iter()
            .map(|chunk| match chunk {
                EncodeChunk::EncodedBuf(buf) => {
                    let buf = syn::LitByteStr::new(buf.as_slice(), Span::call_site());
                    quote!(#buf)
                }
                EncodeChunk::StrExpr(expr) => {
                    // encode expr as str
                    quote!({
                        use #wasm_bindgen::__rt::{encode_u32_to_fixed_len_bytes};
                        const _STR_EXPR: &str = #expr;
                        const _STR_EXPR_BYTES: &[u8] = _STR_EXPR.as_bytes();
                        const _STR_EXPR_BYTES_LEN: usize = _STR_EXPR_BYTES.len() + 5;
                        const _ENCODED_BYTES: [u8; _STR_EXPR_BYTES_LEN] = flat_byte_slices([
                            &encode_u32_to_fixed_len_bytes(_STR_EXPR_BYTES.len() as u32),
                            _STR_EXPR_BYTES,
                        ]);
                        &_ENCODED_BYTES
                    })
                }
            })
            .collect();

        let chunk_len = encoded_chunks.len();

        // concatenate all encoded chunks and write the length in front of the chunk;
        let encode_bytes = quote!({
            const _CHUNK_SLICES: [&[u8]; #chunk_len] = [
                #(#encoded_chunks,)*
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);

            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([&_LEN_BYTES, &_CHUNKS]);
            &_ENCODED_BYTES
        });

        // We already consumed the contents of included files when generating
        // the custom section, but we want to make sure that updates to the
        // generated files will cause this macro to rerun incrementally. To do
        // that we use `include_str!` to force rustc to think it has a
        // dependency on these files. That way when the file changes Cargo will
        // automatically rerun rustc which will rerun this macro. Other than
        // this we don't actually need the results of the `include_str!`, so
        // it's just shoved into an anonymous static.
        let file_dependencies = encoded.included_files.iter().map(|file| {
            let file = file.to_str().unwrap();
            quote! { include_str!(#file) }
        });

        let len = prefix_json.len() as u32;
        let prefix_json_bytes = [&len.to_le_bytes()[..], prefix_json.as_bytes()].concat();
        let prefix_json_bytes = syn::LitByteStr::new(&prefix_json_bytes, Span::call_site());

        (quote! {
            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            #[automatically_derived]
            const _: () = {
                use #wasm_bindgen::__rt::{flat_len, flat_byte_slices};

                static _INCLUDED_FILES: &[&str] = &[#(#file_dependencies),*];

                const _ENCODED_BYTES: &[u8] = #encode_bytes;
                const _PREFIX_JSON_BYTES: &[u8] = #prefix_json_bytes;
                const _ENCODED_BYTES_LEN: usize  = _ENCODED_BYTES.len();
                const _PREFIX_JSON_BYTES_LEN: usize =  _PREFIX_JSON_BYTES.len();
                const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;

                #[link_section = "__wasm_bindgen_unstable"]
                #[allow(long_running_const_eval)]
                static _GENERATED: [u8; _LEN] = flat_byte_slices([_PREFIX_JSON_BYTES, _ENCODED_BYTES]);
            };
        })
        .to_tokens(tokens);

        Ok(())
    }
}

impl TryToTokens for ast::LinkToModule {
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        let mut program_tokens = TokenStream::new();
        self.0.try_to_tokens(&mut program_tokens)?;
        let link_function_name = self.0.link_function_name(0);
        let name = Ident::new(&link_function_name, Span::call_site());
        let wasm_bindgen = &self.0.wasm_bindgen;
        let abi_ret = quote! { #wasm_bindgen::convert::WasmRet<<#wasm_bindgen::__rt::alloc::string::String as #wasm_bindgen::convert::FromWasmAbi>::Abi> };
        let extern_fn = extern_fn(&name, &[], &[], &[], abi_ret);
        (quote! {
            {
                #program_tokens
                #extern_fn

                static __VAL: #wasm_bindgen::__rt::LazyLock<#wasm_bindgen::__rt::alloc::string::String> =
                    #wasm_bindgen::__rt::LazyLock::new(|| unsafe {
                        <#wasm_bindgen::__rt::alloc::string::String as #wasm_bindgen::convert::FromWasmAbi>::from_abi(#name().join())
                    });

                #wasm_bindgen::__rt::alloc::string::String::clone(&__VAL)
            }
        })
        .to_tokens(tokens);
        Ok(())
    }
}

impl ToTokens for ast::Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.rust_name;
        let name_str = self.qualified_name.to_string();
        let name_len = name_str.len() as u32;
        let name_chars: Vec<u32> = name_str.chars().map(|c| c as u32).collect();
        let new_fn = Ident::new(&shared::new_function(&name_str), Span::call_site());
        let free_fn = Ident::new(&shared::free_function(&name_str), Span::call_site());
        let unwrap_fn = Ident::new(&shared::unwrap_function(&name_str), Span::call_site());
        let wasm_bindgen = &self.wasm_bindgen;
        let class_abi = quote! {
            #wasm_bindgen::__rt::WasmPtr<#wasm_bindgen::__rt::WasmRefCell<#name>>
        };
        (quote! {
            #[automatically_derived]
            impl #wasm_bindgen::__rt::marker::SupportsConstructor for #name {}
            #[automatically_derived]
            impl #wasm_bindgen::__rt::marker::SupportsInstanceProperty for #name {}
            #[automatically_derived]
            impl #wasm_bindgen::__rt::marker::SupportsStaticProperty for #name {}

            #[automatically_derived]
            impl #wasm_bindgen::describe::WasmDescribe for #name {
                // Rust-side struct schema: RUST_STRUCT, name_len,
                // ...name chars. Lockstep with describe().
                const SCHEMA: &'static [u32] = &[
                    #wasm_bindgen::describe::RUST_STRUCT,
                    #name_len,
                    #(#name_chars,)*
                ];
                fn describe() {
                    use #wasm_bindgen::describe::*;
                    inform(RUST_STRUCT);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::IntoWasmAbi for #name {
                type Abi = #class_abi;

                fn into_abi(self) -> Self::Abi {
                    use #wasm_bindgen::__rt::alloc::rc::Rc;
                    use #wasm_bindgen::__rt::{WasmPtr, WasmRefCell};
                    WasmPtr::from_ptr(Rc::into_raw(Rc::new(WasmRefCell::new(self))) as *mut WasmRefCell<#name>)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::FromWasmAbi for #name {
                type Abi = #class_abi;

                unsafe fn from_abi(js: Self::Abi) -> Self {
                    use #wasm_bindgen::__rt::alloc::rc::Rc;
                    use #wasm_bindgen::__rt::core::result::Result::{Ok, Err};
                    use #wasm_bindgen::__rt::{assert_not_null, WasmRefCell};

                    let ptr = js.into_ptr();
                    assert_not_null(ptr);
                    let rc = Rc::from_raw(ptr);
                    match Rc::try_unwrap(rc) {
                        Ok(cell) => cell.into_inner(),
                        Err(_) => #wasm_bindgen::throw_str(
                            "attempted to take ownership of Rust value while it was borrowed"
                        ),
                    }
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::__rt::core::convert::From<#name> for
                #wasm_bindgen::JsValue
            {
                fn from(value: #name) -> Self {
                    let ptr = #wasm_bindgen::convert::IntoWasmAbi::into_abi(value);

                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
                    extern "C" {
                        fn #new_fn(ptr: #class_abi) -> u32;
                    }

                    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
                    unsafe fn #new_fn(_: #class_abi) -> u32 {
                        panic!("cannot convert to JsValue outside of the Wasm target")
                    }

                    unsafe {
                        <#wasm_bindgen::JsValue as #wasm_bindgen::convert::FromWasmAbi>
                            ::from_abi(#new_fn(ptr))
                    }
                }
            }



            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            #[automatically_derived]
            const _: () = {
                #wasm_bindgen::__wbindgen_coverage! {
                #[no_mangle]
                #[doc(hidden)]
                // `allow_delayed` is whether it's ok to not actually free the `ptr` immediately
                // if it's still borrowed.
                pub unsafe extern "C-unwind" fn #free_fn(ptr: #class_abi, allow_delayed: u32) {
                    use #wasm_bindgen::__rt::alloc::rc::Rc;

                    if allow_delayed != 0 {
                        // Just drop the implicit `Rc` owned by JS, and then if the value is still
                        // referenced it'll be kept alive by its other `Rc`s.
                        let ptr = ptr.into_ptr();
                        #wasm_bindgen::__rt::assert_not_null(ptr);
                        drop(Rc::from_raw(ptr));
                    } else {
                        // Claim ownership of the value, which will panic if it's borrowed.
                        let _ = <#name as #wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr);
                    }
                }
                }
            };

            #[automatically_derived]
            impl #wasm_bindgen::convert::RefFromWasmAbi for #name {
                type Abi = #class_abi;
                type Anchor = #wasm_bindgen::__rt::RcRef<#name>;

                unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    use #wasm_bindgen::__rt::alloc::rc::Rc;

                    let js = js.into_ptr();
                    #wasm_bindgen::__rt::assert_not_null(js);

                    Rc::increment_strong_count(js);
                    let rc = Rc::from_raw(js);
                    #wasm_bindgen::__rt::RcRef::new(rc)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::RefMutFromWasmAbi for #name {
                type Abi = #class_abi;
                type Anchor = #wasm_bindgen::__rt::RcRefMut<#name>;

                unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
                    use #wasm_bindgen::__rt::alloc::rc::Rc;

                    let js = js.into_ptr();
                    #wasm_bindgen::__rt::assert_not_null(js);

                    Rc::increment_strong_count(js);
                    let rc = Rc::from_raw(js);
                    #wasm_bindgen::__rt::RcRefMut::new(rc)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::LongRefFromWasmAbi for #name {
                type Abi = #class_abi;
                type Anchor = #wasm_bindgen::__rt::RcRef<#name>;

                unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                    <Self as #wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionIntoWasmAbi for #name {
                #[inline]
                fn none() -> Self::Abi { <#class_abi>::null() }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionFromWasmAbi for #name {
                #[inline]
                fn is_none(abi: &Self::Abi) -> bool { abi.is_null() }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::TryFromJsValue for #name {
                fn try_from_js_value(value: #wasm_bindgen::JsValue) -> #wasm_bindgen::__rt::core::result::Result<Self, #wasm_bindgen::JsValue> {
                    Self::try_from_js_value_ref(&value).ok_or(value)
                }
                fn try_from_js_value_ref(value: &#wasm_bindgen::JsValue) -> #wasm_bindgen::__rt::core::option::Option<Self> {
                    let idx = #wasm_bindgen::convert::IntoWasmAbi::into_abi(value);

                    #[link(wasm_import_module = "__wbindgen_placeholder__")]
                    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
                    extern "C" {
                        fn #unwrap_fn(ptr: u32) -> #class_abi;
                    }

                    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
                    unsafe fn #unwrap_fn(_: u32) -> #class_abi {
                        panic!("cannot convert from JsValue outside of the Wasm target")
                    }

                    let ptr = unsafe { #unwrap_fn(idx) };
                    if ptr.is_null() {
                        #wasm_bindgen::__rt::core::option::Option::None
                    } else {
                        unsafe {
                            #wasm_bindgen::__rt::core::option::Option::Some(
                                <Self as #wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr)
                            )
                        }
                    }
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::describe::WasmDescribeVector for #name {
                // `Vec<UserStruct>` schema: VECTOR, NAMED_EXTERNREF,
                // name_len, ...name chars. The struct'\''s own
                // WasmDescribe::SCHEMA is RUST_STRUCT (a single value
                // descriptor); this overrides the blanket impl so the
                // section transport produces the right shape for
                // Vec<UserStruct> args.
                const VECTOR_SCHEMA: &'static [u32] = &[
                    #wasm_bindgen::describe::VECTOR,
                    #wasm_bindgen::describe::NAMED_EXTERNREF,
                    #name_len,
                    #(#name_chars,)*
                ];
                fn describe_vector() {
                    use #wasm_bindgen::describe::*;
                    inform(VECTOR);
                    inform(NAMED_EXTERNREF);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::VectorIntoWasmAbi for #name {
                type Abi = <
                    #wasm_bindgen::__rt::alloc::boxed::Box<[#wasm_bindgen::JsValue]>
                    as #wasm_bindgen::convert::IntoWasmAbi
                >::Abi;

                fn vector_into_abi(
                    vector: #wasm_bindgen::__rt::alloc::boxed::Box<[#name]>
                ) -> Self::Abi {
                    #wasm_bindgen::convert::js_value_vector_into_abi(vector)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::VectorFromWasmAbi for #name {
                type Abi = <
                    #wasm_bindgen::__rt::alloc::boxed::Box<[#wasm_bindgen::JsValue]>
                    as #wasm_bindgen::convert::FromWasmAbi
                >::Abi;

                unsafe fn vector_from_abi(
                    js: Self::Abi
                ) -> #wasm_bindgen::__rt::alloc::boxed::Box<[#name]> {
                    #wasm_bindgen::convert::js_value_vector_from_abi(js)
                }
            }

            // VectorIntoJsValue dispatch: lets `From<Box<[#name]>> for JsValue`
            // pick up the right per-type conversion (array_new + push loop).
            // This replaces the generic `impl<T: VectorIntoWasmAbi> From<Box<[T]>> for JsValue`
            // that used to route through wbg_cast.
            #[automatically_derived]
            impl #wasm_bindgen::__rt::VectorIntoJsValue for #name {
                fn vector_into_jsvalue(
                    vector: #wasm_bindgen::__rt::alloc::boxed::Box<[#name]>,
                ) -> #wasm_bindgen::JsValue {
                    #wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
                }
            }
        })
        .to_tokens(tokens);

        // If this struct `extends` another exported Rust struct, emit:
        //
        //   - `AsRef<Parent<ParentType>>` projecting to the wrapper field
        //     so generic code can accept any direct child where it expects
        //     a borrowed `Parent<ParentType>`. This impl is direct-parent
        //     only: `AsRef` returns `&Parent<P>` borrowed from `&self`,
        //     and ancestors at depth ≥ 2 live inside an `Rc<WasmRefCell>`
        //     reachable only via a transient `borrow()` guard whose
        //     lifetime would not satisfy the `AsRef` contract.
        //   - The upcast wasm export used by the JS side to produce a
        //     separately-refcounted parent pointer when a child instance is
        //     constructed (or when wasm returns a child back to JS). The
        //     upcast clones the `Rc<WasmRefCell<Parent>>` held by the
        //     child's `Parent<ParentType>` field.
        //
        // The JS-side of the extends relationship (class Child extends
        // Parent, instanceof, prototype-chain dispatch) is wired up by
        // cli-support using this export and the matching `extends` schema
        // entry.
        if let Some(parent_path) = &self.extends {
            let parent_field = self.fields.iter().find(|f| f.is_parent);
            if let Some(parent_field) = parent_field {
                let field_name = &parent_field.rust_name;
                let field_ty = &parent_field.ty;
                // The upcast shim symbol must encode the parent's JS-side
                // identity (`extends_js_class` / `extends_js_namespace`),
                // not its Rust path, so that cli-support (which keys
                // `exported_classes` by qualified_name) and the macro
                // agree on the wasm symbol name. Defaults to the last
                // segment of the `extends` path (matching the no-rename
                // case).
                let parent_bare_name = self
                    .extends_js_class
                    .clone()
                    .or_else(|| parent_path.segments.last().map(|s| s.ident.to_string()))
                    .unwrap_or_default();
                let parent_qualified =
                    shared::qualified_name(self.extends_js_namespace.as_deref(), &parent_bare_name);
                let upcast_fn = Ident::new(
                    &shared::upcast_function(&name_str, &parent_qualified),
                    Span::call_site(),
                );
                (quote! {
                    #[automatically_derived]
                    impl #wasm_bindgen::__rt::core::convert::AsRef<#field_ty> for #name {
                        #[inline]
                        fn as_ref(&self) -> &#field_ty {
                            &self.#field_name
                        }
                    }

                    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
                    #[automatically_derived]
                    const _: () = {
                        #[no_mangle]
                        #[doc(hidden)]
                        pub unsafe extern "C-unwind" fn #upcast_fn(ptr: u32) -> u32 {
                            use #wasm_bindgen::__rt::alloc::rc::Rc;
                            use #wasm_bindgen::__rt::{assert_not_null, WasmRefCell};

                            let ptr = ptr as *mut WasmRefCell<#name>;
                            assert_not_null(ptr);
                            let cell = &*ptr;
                            let rc_clone = cell.borrow().#field_name.__wbg_clone_rc();
                            Rc::into_raw(rc_clone) as u32
                        }
                    };
                })
                .to_tokens(tokens);
            }
        }

        for field in self.fields.iter() {
            field.to_tokens(tokens);
        }
    }
}

impl ToTokens for ast::StructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Parent fields exist solely to back the `extends` relationship
        // (used by `AsRef`/`Deref` codegen above). They are not exposed to
        // JS as a getter/setter.
        if self.is_parent {
            return;
        }

        let rust_name = &self.rust_name;
        let struct_name = &self.struct_name;
        let ty = &self.ty;
        let getter = &self.getter;
        let setter = &self.setter;

        let maybe_assert_copy = if self.getter_with_clone.is_some() {
            quote! {}
        } else {
            quote! { assert_copy::<#ty>() }
        };
        let maybe_assert_copy = respan(maybe_assert_copy, ty);

        // Split this out so that it isn't affected by `quote_spanned!`.
        //
        // If we don't do this, it might end up being unable to reference `js`
        // properly because it doesn't have the same span.
        //
        // See https://github.com/wasm-bindgen/wasm-bindgen/pull/3725.
        let js_token = quote! { js };
        let mut val = quote_spanned!(self.rust_name.span()=> (*#js_token).borrow().#rust_name);
        if let Some(span) = self.getter_with_clone {
            val = quote_spanned!(span=> <#ty as Clone>::clone(&#val) );
        }

        let wasm_bindgen = &self.wasm_bindgen;
        let struct_abi = quote! {
            #wasm_bindgen::__rt::WasmPtr<#wasm_bindgen::__rt::WasmRefCell<#struct_name>>
        };

        (quote! {
            #[automatically_derived]
            const _: () = {
                #wasm_bindgen::__wbindgen_coverage! {
                #[cfg_attr(all(target_family = "wasm", not(target_os = "wasi")), no_mangle)]
                #[doc(hidden)]
                pub unsafe extern "C-unwind" fn #getter(js: #struct_abi)
                    -> #wasm_bindgen::convert::WasmRet<<#ty as #wasm_bindgen::convert::IntoWasmAbi>::Abi>
                {
                    use #wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                    use #wasm_bindgen::convert::IntoWasmAbi;

                    fn assert_copy<T: Copy>(){}
                    #maybe_assert_copy;

                    let js = js.into_ptr();
                    assert_not_null(js);
                    let val = #val;
                    <#ty as IntoWasmAbi>::into_abi(val).into()
                }
                }
            };
        })
        .to_tokens(tokens);

        // Getter's descriptor is the bare field type schema (no
        // FUNCTION header). Cli's struct field processor reads it
        // from the section keyed by the getter shim name. Same shape
        // as ImportStatic.
        emit_static_descriptor_entry_static(
            &self.wasm_bindgen,
            &getter.to_string(),
            schema_parts_for_type(&self.wasm_bindgen, ty),
            tokens,
        );

        if self.readonly {
            return;
        }

        let abi = quote! { <#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi };
        let (args, names) = splat(wasm_bindgen, &Ident::new("val", rust_name.span()), &abi);

        (quote! {
            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            #[automatically_derived]
            const _: () = {
                #wasm_bindgen::__wbindgen_coverage! {
                #[no_mangle]
                #[doc(hidden)]
                pub unsafe extern "C-unwind" fn #setter(
                    js: #struct_abi,
                    #(#args,)*
                ) {
                    use #wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                    use #wasm_bindgen::convert::FromWasmAbi;

                    let js = js.into_ptr();
                    assert_not_null(js);
                    let val = <#abi as #wasm_bindgen::convert::WasmAbi>::join(#(#names),*);
                    let val = <#ty as FromWasmAbi>::from_abi(val);
                    (*js).borrow_mut().#rust_name = val;
                }
                }
            };
        })
        .to_tokens(tokens);
    }
}

impl TryToTokens for ast::Export {
    fn try_to_tokens(self: &ast::Export, into: &mut TokenStream) -> Result<(), Diagnostic> {
        let generated_name = self.rust_symbol();
        let export_name = self.export_name();
        let mut args = vec![];
        let mut arg_conversions = vec![];
        let mut converted_arguments = vec![];
        let ret = Ident::new("_ret", Span::call_site());

        let name = &self.rust_name;
        let wasm_bindgen = &self.wasm_bindgen;

        let offset = if self.method_self.is_some() {
            if matches!(self.method_self, Some(ast::MethodSelf::ByValue)) {
                let class = self.rust_class.as_ref().unwrap();
                args.push(quote! { me: <#class as #wasm_bindgen::convert::FromWasmAbi>::Abi });
            } else {
                let class = self.rust_class.as_ref().unwrap();
                let abi = match self.method_self {
                    Some(ast::MethodSelf::RefMutable) => {
                        quote! { <#class as #wasm_bindgen::convert::RefMutFromWasmAbi>::Abi }
                    }
                    Some(ast::MethodSelf::RefShared) => {
                        if self.function.r#async {
                            quote! { <#class as #wasm_bindgen::convert::LongRefFromWasmAbi>::Abi }
                        } else {
                            quote! { <#class as #wasm_bindgen::convert::RefFromWasmAbi>::Abi }
                        }
                    }
                    _ => unreachable!(),
                };
                args.push(quote! { me: #abi });
            }
            1
        } else {
            0
        };
        let wasm_bindgen_futures = &self.wasm_bindgen_futures;
        let js_sys = &self.js_sys;
        let futures = if ast::use_js_sys_futures() {
            quote! { #js_sys::futures }
        } else {
            quote! { #wasm_bindgen_futures }
        };
        let receiver = match self.method_self {
            Some(ast::MethodSelf::ByValue) => {
                let class = self.rust_class.as_ref().unwrap();
                arg_conversions.push(quote! {
                    // Owned `self` is consumed inside the catch-unwind closure;
                    // assert it's `UnwindSafe` so a panic mid-method doesn't
                    // surface a half-modified observable value to the caller.
                    #wasm_bindgen::__rt::ensure_unwind_safe::<#class>();
                    let me = unsafe {
                        <#class as #wasm_bindgen::convert::FromWasmAbi>::from_abi(me)
                    };
                });
                quote! { me.#name }
            }
            Some(ast::MethodSelf::RefMutable) => {
                let class = self.rust_class.as_ref().unwrap();
                arg_conversions.push(quote! {
                    // `&mut self` requires `Self: RefUnwindSafe` (logical
                    // unwind-safety): if the method panics partway through
                    // mutation, the caller may observe the struct again, so
                    // any interior mutability whose invariants could be
                    // broken must be opt-in via `AssertUnwindSafe` or a
                    // manual `impl RefUnwindSafe`. Stdlib's `&mut T:
                    // !UnwindSafe` blanket would otherwise reject every
                    // `&mut self` method, so we use a separate type-level
                    // assertion rather than relying on closure capture
                    // inference.
                    #wasm_bindgen::__rt::ensure_ref_unwind_safe::<#class>();
                    let mut me = unsafe {
                        <#class as #wasm_bindgen::convert::RefMutFromWasmAbi>
                            ::ref_mut_from_abi(me)
                    };
                    let me = &mut *me;
                });
                quote! { me.#name }
            }
            Some(ast::MethodSelf::RefShared) => {
                let class = self.rust_class.as_ref().unwrap();
                let (trait_, func, borrow) = if self.function.r#async {
                    (
                        quote!(LongRefFromWasmAbi),
                        quote!(long_ref_from_abi),
                        quote!(
                            <<#class as #wasm_bindgen::convert::LongRefFromWasmAbi>
                                ::Anchor as #wasm_bindgen::__rt::core::borrow::Borrow<#class>>
                                ::borrow(&me)
                        ),
                    )
                } else {
                    (quote!(RefFromWasmAbi), quote!(ref_from_abi), quote!(&*me))
                };
                arg_conversions.push(quote! {
                    // `&self` requires `Self: RefUnwindSafe` for the same
                    // reason as `&mut self` — a panic mid-method can leave
                    // interior-mutable state in a torn condition observable
                    // by subsequent calls.
                    #wasm_bindgen::__rt::ensure_ref_unwind_safe::<#class>();
                    let me = unsafe {
                        <#class as #wasm_bindgen::convert::#trait_>::#func(me)
                    };
                    let me = #borrow;
                });
                quote! { me.#name }
            }
            None => match &self.rust_class {
                Some(class) => quote! { #class::#name },
                None => quote! { #name },
            },
        };

        let mut argtys = Vec::new();
        for (i, arg) in self.function.arguments.iter().enumerate() {
            argtys.push(&*arg.pat_type.ty);
            let i = i + offset;
            let ident = Ident::new(&format!("arg{i}"), Span::call_site());
            fn unwrap_nested_types(ty: &syn::Type) -> &syn::Type {
                match &ty {
                    syn::Type::Group(syn::TypeGroup { ref elem, .. }) => unwrap_nested_types(elem),
                    syn::Type::Paren(syn::TypeParen { ref elem, .. }) => unwrap_nested_types(elem),
                    _ => ty,
                }
            }
            let ty = unwrap_nested_types(&arg.pat_type.ty);

            match &ty {
                syn::Type::Reference(syn::TypeReference {
                    mutability: Some(_),
                    elem,
                    ..
                }) => {
                    let abi = quote! { <#elem as #wasm_bindgen::convert::RefMutFromWasmAbi>::Abi };
                    let (prim_args, prim_names) = splat(wasm_bindgen, &ident, &abi);
                    args.extend(prim_args);
                    arg_conversions.push(quote! {
                        // `&mut T` arg: same logical-unwind-safety check as
                        // `&mut self` — `T` must be `RefUnwindSafe` so any
                        // panic mid-call cannot leave torn interior state.
                        #wasm_bindgen::__rt::ensure_ref_unwind_safe::<#elem>();
                        let mut #ident = unsafe {
                            <#elem as #wasm_bindgen::convert::RefMutFromWasmAbi>
                                ::ref_mut_from_abi(
                                    <#abi as #wasm_bindgen::convert::WasmAbi>::join(#(#prim_names),*)
                                )
                        };
                        let #ident = &mut *#ident;
                    });
                }
                syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                    if self.function.r#async {
                        let abi =
                            quote! { <#elem as #wasm_bindgen::convert::LongRefFromWasmAbi>::Abi };
                        let (prim_args, prim_names) = splat(wasm_bindgen, &ident, &abi);
                        args.extend(prim_args);
                        arg_conversions.push(quote! {
                            // `&T` arg in async export: enforce
                            // `T: RefUnwindSafe` for the same reason.
                            #wasm_bindgen::__rt::ensure_ref_unwind_safe::<#elem>();
                            let #ident = unsafe {
                                <#elem as #wasm_bindgen::convert::LongRefFromWasmAbi>
                                    ::long_ref_from_abi(
                                        <#abi as #wasm_bindgen::convert::WasmAbi>::join(#(#prim_names),*)
                                    )
                            };
                            let #ident = <<#elem as #wasm_bindgen::convert::LongRefFromWasmAbi>
                                ::Anchor as core::borrow::Borrow<#elem>>
                                ::borrow(&#ident);
                        });
                    } else {
                        let abi = quote! { <#elem as #wasm_bindgen::convert::RefFromWasmAbi>::Abi };
                        let (prim_args, prim_names) = splat(wasm_bindgen, &ident, &abi);
                        args.extend(prim_args);
                        arg_conversions.push(quote! {
                            // `&T` arg: enforce `T: RefUnwindSafe`.
                            #wasm_bindgen::__rt::ensure_ref_unwind_safe::<#elem>();
                            let #ident = unsafe {
                                <#elem as #wasm_bindgen::convert::RefFromWasmAbi>
                                    ::ref_from_abi(
                                        <#abi as #wasm_bindgen::convert::WasmAbi>::join(#(#prim_names),*)
                                    )
                            };
                            let #ident = &*#ident;
                        });
                    }
                }
                _ => {
                    let abi = quote! { <#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi };
                    let (prim_args, prim_names) = splat(wasm_bindgen, &ident, &abi);
                    args.extend(prim_args);
                    arg_conversions.push(quote! {
                        // Owned arg: consumed locally inside the catch-unwind
                        // closure, so `UnwindSafe` (not `RefUnwindSafe`) is
                        // the relevant property.
                        #wasm_bindgen::__rt::ensure_unwind_safe::<#ty>();
                        let #ident = unsafe {
                            <#ty as #wasm_bindgen::convert::FromWasmAbi>
                                ::from_abi(
                                    <#abi as #wasm_bindgen::convert::WasmAbi>::join(#(#prim_names),*)
                                )
                        };
                    });
                }
            }
            converted_arguments.push(quote! { #ident });
        }
        let syn_unit = syn::Type::Tuple(syn::TypeTuple {
            elems: Default::default(),
            paren_token: Default::default(),
        });
        let syn_ret = self
            .function
            .ret
            .as_ref()
            .map(|ret| &ret.r#type)
            .unwrap_or(&syn_unit);
        if let syn::Type::Reference(_) = syn_ret {
            bail_span!(syn_ret, "cannot return a borrowed ref with #[wasm_bindgen]",)
        }

        // For an `async` function we always run it through `future_to_promise`
        // since we're returning a promise to JS, and this will implicitly
        // require that the function returns a `Future<Output = Result<...>>`
        let (ret_ty, inner_ret_ty, ret_expr) = if self.function.r#async {
            if self.start.is_start() {
                (
                    quote! { () },
                    quote! { () },
                    quote! {
                        <#syn_ret as #wasm_bindgen::__rt::Start>::start(#ret.await)
                    },
                )
            } else {
                (
                    quote! { #wasm_bindgen::JsValue },
                    quote! { #syn_ret },
                    quote! {
                        <#syn_ret as #wasm_bindgen::__rt::IntoJsResult>::into_js_result(#ret.await)
                    },
                )
            }
        } else if self.start.is_start() {
            (
                quote! { () },
                quote! { () },
                quote! { <#syn_ret as #wasm_bindgen::__rt::Start>::start(#ret) },
            )
        } else {
            (quote! { #syn_ret }, quote! { #syn_ret }, quote! { #ret })
        };

        let mut call = quote! {
            {
                #(#arg_conversions)*
                let #ret = #receiver(#(#converted_arguments),*);
                #ret_expr
            }
        };

        if self.function.r#async {
            if self.start.is_start() {
                call = quote! {
                    #futures::spawn_local(async move {
                        #call
                    })
                }
            } else {
                call = quote! {
                    #futures::future_to_promise(async move {
                        #call
                    }).into()
                }
            }
        } else {
            call = quote! {
                #wasm_bindgen::__rt::maybe_catch_unwind(|| {
                    #call
                })
            };
        }

        let projection = quote! { <#ret_ty as #wasm_bindgen::convert::ReturnWasmAbi> };
        let convert_ret = quote! { #projection::return_abi(#ret).into() };
        let nargs = self.function.arguments.len() as u32;
        let attrs = self
            .function
            .rust_attrs
            .iter()
            .map(|attr| match &attr.meta {
                Meta::List(list @ MetaList { path, .. }) if path.is_ident("expect") => {
                    let list = MetaList {
                        path: parse_quote!(allow),
                        ..list.clone()
                    };
                    Attribute {
                        meta: Meta::List(list),
                        ..*attr
                    }
                }
                _ => attr.clone(),
            })
            .collect::<Vec<_>>();

        let mut checks = Vec::new();
        if self.start.is_start() {
            checks.push(quote! { const _ASSERT: fn() = || -> #projection::Abi { loop {} }; });
        };

        if let Some(class) = self.rust_class.as_ref() {
            // little helper function to make sure the check points to the
            // location of the function causing the assert to fail
            let mut add_check = |token_stream| {
                checks.push(respan(token_stream, &self.rust_name));
            };

            match &self.method_kind {
                ast::MethodKind::Constructor => {
                    add_check(quote! {
                        let _: #wasm_bindgen::__rt::marker::CheckSupportsConstructor<#class>;
                    });

                    if self.function.r#async {
                        (quote_spanned! {
                            self.function.name_span =>
                            const _: () = {
                                #[deprecated(note = "async constructors produce invalid TS code and support will be removed in the future")]
                                const fn constructor() {}
                                constructor();
                            };
                        })
                        .to_tokens(into);
                    }
                }
                ast::MethodKind::Operation(operation) => match operation.kind {
                    ast::OperationKind::Getter(_) | ast::OperationKind::Setter(_) => {
                        if operation.is_static {
                            add_check(quote! {
                                let _: #wasm_bindgen::__rt::marker::CheckSupportsStaticProperty<#class>;
                            });
                        } else {
                            add_check(quote! {
                                let _: #wasm_bindgen::__rt::marker::CheckSupportsInstanceProperty<#class>;
                            });
                        }
                    }
                    _ => {}
                },
            }
        }

        (quote! {
            #[automatically_derived]
            const _: () = {
                #wasm_bindgen::__wbindgen_coverage! {
                #(#attrs)*
                #[cfg_attr(
                    all(target_family = "wasm", not(target_os = "wasi")),
                    export_name = #export_name,
                )]
                pub unsafe extern "C-unwind" fn #generated_name(#(#args),*) -> #wasm_bindgen::convert::WasmRet<#projection::Abi> {
                    const _: () = {
                        #(#checks)*
                    };

                    let #ret = #call;
                    #convert_ret
                }
                }
            };
        })
        .to_tokens(into);

        // Section transport: emit the descriptor bytes for this
        // export directly into `__wasm_bindgen_descriptors`. The
        // legacy synthetic `__wbindgen_describe_<export_name>`
        // function (executed by the cli's wasm interpreter) is no
        // longer emitted — the section is the sole transport.
        //
        // The set of schema parts is a 3-word header, one entry per
        // argument (with LONGREF inserted for async-shared-ref args,
        // matching the legacy describe-stream shape), then ret_ty and
        // inner_ret_ty.
        let arg_parts = build_arg_parts(&self.wasm_bindgen, &argtys, self.function.r#async);
        let ret_parts = schema_parts_for_type_tokens(&self.wasm_bindgen, &ret_ty);
        let inner_ret_parts = schema_parts_for_type_tokens(&self.wasm_bindgen, &inner_ret_ty);
        emit_static_descriptor_entry(
            &self.wasm_bindgen,
            &export_name,
            &arg_parts,
            ret_parts,
            inner_ret_parts,
            nargs,
            &attrs,
            into,
        );

        Ok(())
    }
}

/// Emit a `static` linked into the `__wasm_bindgen_descriptors` custom
/// section whose bytes encode the same descriptor stream the legacy
/// `__wbindgen_describe_<name>` function would have produced.
///
/// The layout of the static matches the section format documented next
/// to [`wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME`].
///
/// The composition relies on every involved `<Ty as WasmDescribe>::SCHEMA`
/// being a non-empty slice. If any type's `SCHEMA` is still the trait
/// default (`&[]`), the resulting bytes are malformed; `cli-support`
/// detects this on decode and falls back to the interpreter for that
/// shim, so emitting the static is always safe. The macro will expand
/// the set of recognised wrapper types in follow-up commits, eventually
/// covering everything the interpreter handles today.
/// Build per-argument schema-parts streams shared between the export
/// and import codegen paths. Async-shared-ref args get LONGREF instead
/// of REF; everything else flows through `schema_parts_for_type`.
fn build_arg_parts(
    wasm_bindgen: &syn::Path,
    argtys: &[&syn::Type],
    is_async: bool,
) -> Vec<TokenStream> {
    argtys
        .iter()
        .map(|ty| match ty {
            syn::Type::Reference(reference) if is_async && reference.mutability.is_none() => {
                let inner = &reference.elem;
                let inner_parts = schema_parts_for_type(wasm_bindgen, inner);
                quote! {
                    &[#wasm_bindgen::describe::LONGREF],
                    #inner_parts
                }
            }
            _ => schema_parts_for_type(wasm_bindgen, ty),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn emit_static_descriptor_entry(
    wasm_bindgen: &syn::Path,
    shim_name: &str,
    arg_parts: &[TokenStream],
    ret_parts: TokenStream,
    inner_ret_parts: TokenStream,
    nargs: u32,
    // Same #[cfg(...)] / #[doc(...)] attrs that wrap the legacy
    // __wbindgen_describe_<name> function. They must be replicated
    // on the new section static so the two transports turn on and
    // off in lockstep; otherwise a cfg-gated extern function (e.g.
    // js-sys's #[cfg(js_sys_unstable_apis)] items) would have its
    // descriptor emitted unconditionally and reference types whose
    // declarations are gated out.
    attrs: &[syn::Attribute],
    into: &mut TokenStream,
) {
    // The static's identifier is derived from the shim name so each
    // descriptor entry has a unique symbol within its crate. The name
    // is also written into the entry's body verbatim.
    let static_ident = Ident::new(
        &format!("__WBG_DESCRIPTOR_{}", mangle_export_name_for_ident(shim_name)),
        Span::call_site(),
    );
    let shim_name_bytes = syn::LitByteStr::new(shim_name.as_bytes(), Span::call_site());
    let shim_name_len = shim_name.len();

    // Drain any closure-wrapper emissions queued by recursive
    // `schema_parts_for_type` calls during `arg_parts` / `ret_parts`
    // building. These appear next to the descriptor entry static so
    // the wrappers and the section entries live or die together at
    // compile-time cfg evaluation.
    let pending_closure_wrappers = take_pending_closure_wrappers();

    (quote! {
        #[cfg(target_family = "wasm")]
        #(#attrs)*
        #[automatically_derived]
        const _: () = {
            const __PARTS: &[&[u32]] = &[
                &[
                    #wasm_bindgen::describe::FUNCTION,
                    0u32,
                    #nargs,
                ],
                #(#arg_parts)*
                #ret_parts
                #inner_ret_parts
            ];
            const __WORDS: usize =
                #wasm_bindgen::describe::schema::word_total(__PARTS);
            const __SCHEMA: [u32; __WORDS] =
                #wasm_bindgen::describe::schema::concat_words::<__WORDS>(__PARTS);
            const __ENTRY_LEN: usize =
                #wasm_bindgen::describe::schema::entry_byte_len(#shim_name_len, __WORDS);
            // The `link_section` attribute places this byte array in
            // the shared __wasm_bindgen_descriptors custom section
            // that wasm-bindgen-cli parses.
            //
            // No `#[used]` here: the legacy __wasm_bindgen_unstable
            // section emission (see line ~181 of this file) is the
            // template. With `#[used]`, LLVM also materialises the
            // bytes in linear memory (the data section), which on
            // wasm doubles the storage cost for every descriptor. The
            // `pub` visibility plus the static name being unique
            // keeps the symbol live through linking without `#[used]`.
            #[link_section = "__wasm_bindgen_descriptors"]
            #[doc(hidden)]
            pub static #static_ident: [u8; __ENTRY_LEN] =
                #wasm_bindgen::describe::schema::pack_entry::<__ENTRY_LEN>(
                    #shim_name_bytes,
                    #wasm_bindgen::__rt::DESCRIPTOR_KIND_REGULAR,
                    &__SCHEMA,
                );
        };

        // Per-monomorphisation closure-invoke wrappers required by
        // any SYMBOL_REFs in the schema above. Each wrapper is its
        // own top-level item; they all share their `#[cfg]` posture
        // with the descriptor entry by being emitted into the same
        // module scope from the same expansion.
        #(#pending_closure_wrappers)*
    })
    .to_tokens(into);
}

/// Emit a `DESCRIPTOR_KIND_STATIC` entry. Unlike the regular variant,
/// the schema is just the static's type schema (no FUNCTION header).
/// Used by `ToTokens for ast::ImportStatic` so `#[wasm_bindgen]`
/// `static`s flow through the section transport instead of relying on
/// the legacy interpreter.
fn emit_static_descriptor_entry_static(
    wasm_bindgen: &syn::Path,
    shim_name: &str,
    schema_parts: TokenStream,
    into: &mut TokenStream,
) {
    let static_ident = Ident::new(
        &format!("__WBG_DESCRIPTOR_{}", mangle_export_name_for_ident(shim_name)),
        Span::call_site(),
    );
    let shim_name_bytes = syn::LitByteStr::new(shim_name.as_bytes(), Span::call_site());
    let shim_name_len = shim_name.len();

    (quote! {
        #[cfg(target_family = "wasm")]
        #[automatically_derived]
        const _: () = {
            const __PARTS: &[&[u32]] = &[
                #schema_parts
            ];
            const __WORDS: usize =
                #wasm_bindgen::describe::schema::word_total(__PARTS);
            const __SCHEMA: [u32; __WORDS] =
                #wasm_bindgen::describe::schema::concat_words::<__WORDS>(__PARTS);
            const __ENTRY_LEN: usize =
                #wasm_bindgen::describe::schema::entry_byte_len(#shim_name_len, __WORDS);
            #[link_section = "__wasm_bindgen_descriptors"]
            #[doc(hidden)]
            pub static #static_ident: [u8; __ENTRY_LEN] =
                #wasm_bindgen::describe::schema::pack_entry::<__ENTRY_LEN>(
                    #shim_name_bytes,
                    #wasm_bindgen::__rt::DESCRIPTOR_KIND_STATIC,
                    &__SCHEMA,
                );
        };
    })
    .to_tokens(into);
}

/// Convert a syntactic type into a sequence of `&[u32]` schema-part
/// expressions, ready to be spliced into a `&[ ... ]` parts array.
///
/// The returned `TokenStream` always ends with a trailing comma so the
/// caller can splat several together without worrying about separators.
///
/// Recognised structural shapes (matching the runtime `impl WasmDescribe`
/// blocks in `src/describe.rs`):
///
/// * `&T`         -> `REF, <T schema...>`
/// * `&mut T`     -> `REFMUT, <T schema...>`
/// * `Option<T>`  -> `OPTIONAL, <T schema...>`
/// * `Vec<T>`     -> `VECTOR, <T schema...>`
/// * `Box<[T]>`   -> `VECTOR, <T schema...>`
/// * `[T]`        -> `SLICE, <T schema...>`
/// * `Result<T, E>` -> `RESULT, <T schema...>`  (E is discarded; matches
///   the runtime impl)
/// * `Clamped<T>` -> `CLAMPED, <T schema...>`
///
/// For everything else, falls back to the type's own
/// `<Ty as WasmDescribe>::SCHEMA` slice. Leaf user types (struct/enum
/// impls produced by `#[wasm_bindgen]` itself, plus all the primitives
/// and JsValue) populate that slice, so the fallback yields a non-empty
/// schema for any type the macro has taken over.
///
/// If a function references a type whose `SCHEMA` is still the default
/// empty slice (a wrapper shape the macro hasn't taught itself yet, or
/// an external `impl WasmDescribe` that hasn't been ported), the
/// resulting bytes will fail to decode in cli-support and the legacy
/// interpreter handles that shim instead. This keeps the migration safe
/// at every stage.
fn schema_parts_for_type(wasm_bindgen: &syn::Path, ty: &syn::Type) -> TokenStream {
    // Closure-shaped args (`&dyn Fn(...)`, `&mut dyn FnMut(...)`)
    // need SYMBOL_REF handling because the function-table slot the
    // legacy descriptor would have carried is only knowable post-link.
    // Hand off to schema_parts_for_raw_closure, which also queues a
    // wrapper-emission side-effect.
    if let Some((is_mut, arg_tys, ret_ty)) = detect_closure_arg(ty) {
        return schema_parts_for_raw_closure(wasm_bindgen, is_mut, &arg_tys, &ret_ty);
    }
    let unwrapped = get_ty(ty);
    match unwrapped {
        syn::Type::Reference(r) => {
            let inner = schema_parts_for_type(wasm_bindgen, &r.elem);
            let opcode = if r.mutability.is_some() {
                quote! { #wasm_bindgen::describe::REFMUT }
            } else {
                quote! { #wasm_bindgen::describe::REF }
            };
            quote! { &[#opcode], #inner }
        }
        syn::Type::Slice(s) => {
            let inner = schema_parts_for_type(wasm_bindgen, &s.elem);
            quote! { &[#wasm_bindgen::describe::SLICE], #inner }
        }
        syn::Type::Path(p) if p.qself.is_none() => {
            if let Some(last) = p.path.segments.last() {
                let ident = &last.ident;
                if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                    // Pull out single-parameter type generics.
                    let first_type_arg = args.args.iter().find_map(|a| match a {
                        syn::GenericArgument::Type(t) => Some(t),
                        _ => None,
                    });
                    if let Some(inner_ty) = first_type_arg {
                        if ident == "Option" {
                            let inner = schema_parts_for_type(wasm_bindgen, inner_ty);
                            return quote! {
                                &[#wasm_bindgen::describe::OPTIONAL], #inner
                            };
                        }
                        if ident == "Vec" {
                            // `Vec<T>` schema flows through
                            // `<T as WasmDescribeVector>::VECTOR_SCHEMA`,
                            // the same trait + const path that the
                            // runtime `Box<[T]>::describe -> T::describe_vector`
                            // chain uses. This is critical because the
                            // shape varies per element type:
                            //
                            //   primitives:    [VECTOR, <T's SCHEMA>]
                            //   String:        [VECTOR, NAMED_EXTERNREF, "string"]
                            //   user struct:   [VECTOR, NAMED_EXTERNREF, <name>]
                            //   string enum:   [VECTOR, EXTERNREF]
                            //
                            // The trait const is set per-impl to the
                            // right value; we just defer to it here.
                            return quote! {
                                <#inner_ty as #wasm_bindgen::describe::WasmDescribeVector>::VECTOR_SCHEMA,
                            };
                        }
                        if ident == "Result" {
                            // `impl WasmDescribe for Result<T, E>` only
                            // describes the Ok-arm's type.
                            let inner = schema_parts_for_type(wasm_bindgen, inner_ty);
                            return quote! {
                                &[#wasm_bindgen::describe::RESULT], #inner
                            };
                        }
                        if ident == "Clamped" {
                            let inner = schema_parts_for_type(wasm_bindgen, inner_ty);
                            return quote! {
                                &[#wasm_bindgen::describe::CLAMPED], #inner
                            };
                        }
                        if ident == "Box" {
                            // Recognise `Box<[T]>` specifically; same
                            // shape as `Vec<T>`.
                            if let syn::Type::Slice(s) = get_ty(inner_ty) {
                                let elem = &s.elem;
                                return quote! {
                                    <#elem as #wasm_bindgen::describe::WasmDescribeVector>::VECTOR_SCHEMA,
                                };
                            }
                        }
                    }
                }
            }
            // Plain path with no recognised wrapper: defer to the trait const.
            quote! { <#ty as #wasm_bindgen::describe::WasmDescribe>::SCHEMA, }
        }
        // Anything we don't recognise (tuples, fn pointers, trait objects)
        // is handed off to the type's own SCHEMA. If it's `&[]` the
        // resulting section entry will be malformed and cli-support will
        // fall back to the interpreter for that shim.
        _ => quote! { <#ty as #wasm_bindgen::describe::WasmDescribe>::SCHEMA, },
    }
}

/// Variant of [`schema_parts_for_type`] that accepts a pre-built
/// `TokenStream` for the type. Used by the export-site code where
/// `ret_ty` and `inner_ret_ty` have already been synthesised as
/// `quote!{ ... }` rather than parsed as `syn::Type`. We do a quick
/// re-parse so the same structural recognition applies; if the parse
/// fails, fall back to the trait const, which is always safe.
fn schema_parts_for_type_tokens(
    wasm_bindgen: &syn::Path,
    ty_tokens: &TokenStream,
) -> TokenStream {
    match syn::parse2::<syn::Type>(ty_tokens.clone()) {
        Ok(ty) => schema_parts_for_type(wasm_bindgen, &ty),
        Err(_) => quote! { <#ty_tokens as #wasm_bindgen::describe::WasmDescribe>::SCHEMA, },
    }
}

thread_local! {
    /// Module-scope items emitted as side effects of `schema_parts_for_type`.
    /// Specifically: the closure-invoke wrappers that back closure
    /// SYMBOL_REF entries. Drained at the end of each
    /// `emit_static_descriptor_entry` so the wrappers appear next to
    /// the descriptor they support.
    ///
    /// Per-thread because proc-macros run on whatever thread cargo
    /// dispatches. Per-crate dedup is handled by [`CLOSURE_WRAPPERS_EMITTED`].
    static CLOSURE_WRAPPER_EMISSIONS: RefCell<Vec<TokenStream>> =
        const { RefCell::new(Vec::new()) };

    /// Stable per-crate dedup: a closure signature plus unwind flavour
    /// should only emit a wrapper once even if mentioned in many
    /// `#[wasm_bindgen]` functions in the same crate. Keyed by the
    /// content hash that names the wrapper.
    static CLOSURE_WRAPPERS_EMITTED: RefCell<HashSet<String>> =
        RefCell::default();
}

/// Drain the thread-local closure-wrapper emission queue. Called once
/// at the end of each `emit_static_descriptor_entry` so the wrappers
/// land in the same `TokenStream` as the descriptor entry they support,
/// keeping cargo's incremental-recompile boundaries tight.
fn take_pending_closure_wrappers() -> Vec<TokenStream> {
    CLOSURE_WRAPPER_EMISSIONS.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// Recognise a closure-shaped argument type. Returns parsed pieces
/// usable for both wrapper emission and schema-parts synthesis:
///
/// * `is_mut`: `true` for `&mut dyn FnMut(...)`, `false` for `&dyn Fn(...)`.
/// * `arg_tys`: parsed argument types `(A1, A2, ...)`.
/// * `ret_ty`: parsed return type. `()` if the signature has no `-> R`.
///
/// Only matches the **raw `&dyn Fn` / `&mut dyn FnMut`** form right now.
/// `Closure<T>` and friends are intentionally not detected here — those
/// flow through the legacy interpreter pathway for now and will be
/// migrated in a follow-up commit. The fallback is safe because
/// emitting a malformed section entry causes cli-support to drop it
/// and use the interpreter for that shim.
fn detect_closure_arg(ty: &syn::Type) -> Option<(bool, Vec<syn::Type>, syn::Type)> {
    let unwrapped = get_ty(ty);
    let syn::Type::Reference(syn::TypeReference {
        mutability, elem, ..
    }) = unwrapped
    else {
        return None;
    };
    let inner = get_ty(elem);
    let syn::Type::TraitObject(trait_obj) = inner else {
        return None;
    };
    let is_mut = mutability.is_some();
    for bound in &trait_obj.bounds {
        let syn::TypeParamBound::Trait(tb) = bound else {
            continue;
        };
        let Some(last) = tb.path.segments.last() else {
            continue;
        };
        let name = last.ident.to_string();
        let want_mut = match name.as_str() {
            "Fn" => false,
            "FnMut" => true,
            _ => continue,
        };
        if want_mut != is_mut {
            continue;
        }
        // Pull arg list + return type out of `Fn(A, B) -> R` shape.
        let syn::PathArguments::Parenthesized(paren) = &last.arguments else {
            continue;
        };
        let args: Vec<syn::Type> = paren.inputs.iter().cloned().collect();
        // The runtime only has impls for the specific 1-arg `(&A)`
        // shape on `dyn Fn`/`dyn FnMut`, not for multi-arg shapes
        // mixing ref + owned. If we see anything other than exactly
        // one `&T` arg (and no other args), bail out and let the
        // interpreter handle that closure. Single owned-arg and
        // multi-arg-all-owned cases are handled below.
        let has_ref = args.iter().any(|t| matches!(get_ty(t), syn::Type::Reference(_)));
        if has_ref && args.len() != 1 {
            return None;
        }
        let ret_ty: syn::Type = match &paren.output {
            syn::ReturnType::Default => syn::parse_quote!(()),
            syn::ReturnType::Type(_, t) => (**t).clone(),
        };
        return Some((is_mut, args, ret_ty));
    }
    None
}

/// Emit the schema parts for a `&dyn Fn(...) -> R` or `&mut dyn FnMut(...) -> R`
/// argument: `REF/REFMUT, FUNCTION, SYMBOL_REF, <name_payload>, nargs, ...args..., ret, ret`.
/// Side-effect: queues a wrapper-function emission into
/// `CLOSURE_WRAPPER_EMISSIONS` so the next call to
/// `take_pending_closure_wrappers` returns it.
///
/// Per the runtime's `WasmDescribe for dyn Fn(...)` impl, `UNWIND_SAFE`
/// is always `true` for raw `&dyn Fn` / `&mut dyn FnMut` args — the
/// panic-catching invoke shim is selected. We emit only the matching
/// wrapper here. If a future closure form needs `false`, we'll emit
/// both variants and let post-link cleanup strip whichever wasn't
/// referenced.
fn schema_parts_for_raw_closure(
    wasm_bindgen: &syn::Path,
    is_mut: bool,
    arg_tys: &[syn::Type],
    ret_ty: &syn::Type,
) -> TokenStream {
    const UNWIND_SAFE: bool = true;
    let nargs = arg_tys.len() as u32;

    // Content hash of the closure signature, used to name the wrapper
    // and the SYMBOL_REF target identically. `ShortHash` mixes in
    // CARGO_PKG_NAME / VERSION already, so the same signature in two
    // crates gets distinct names — fine, each crate emits its own
    // wrapper.
    let sig_repr = format!(
        "closure|is_mut={is_mut}|unwind_safe={UNWIND_SAFE}|args=[{}]|ret={}",
        arg_tys
            .iter()
            .map(|t| t.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join(","),
        ret_ty.to_token_stream().to_string(),
    );
    let hash = ShortHash(&sig_repr).to_string();
    let export_name = format!("__wbg_invoke_{hash}");

    // Recurse into each arg type / return type to build their schema.
    let arg_parts: Vec<TokenStream> = arg_tys
        .iter()
        .map(|t| schema_parts_for_type(wasm_bindgen, t))
        .collect();
    let ret_parts = schema_parts_for_type(wasm_bindgen, ret_ty);

    // SYMBOL_REF payload: in the schema stream, we need
    // `[SYMBOL_REF, name_len_u32, name_chars_padded_to_4bytes]` as a
    // sequence of u32 words. Each char of the name is one byte; we
    // pack 4 chars per u32 in LE order. Padding bytes are zero.
    //
    // Doing the packing at proc-macro time keeps the schema's
    // `concat_words` step trivially const-evaluable: we emit a
    // literal `&[u32]` slice for the SYMBOL_REF payload, just like we
    // do for plain opcode runs.
    let name_bytes = export_name.as_bytes();
    let name_len_u32 = name_bytes.len() as u32;
    let mut packed_words: Vec<u32> = Vec::with_capacity(name_bytes.len().div_ceil(4) + 1);
    packed_words.push(name_len_u32);
    let mut i = 0;
    while i < name_bytes.len() {
        let b0 = name_bytes[i];
        let b1 = name_bytes.get(i + 1).copied().unwrap_or(0);
        let b2 = name_bytes.get(i + 2).copied().unwrap_or(0);
        let b3 = name_bytes.get(i + 3).copied().unwrap_or(0);
        packed_words.push(
            u32::from(b0)
                | (u32::from(b1) << 8)
                | (u32::from(b2) << 16)
                | (u32::from(b3) << 24),
        );
        i += 4;
    }

    // Queue the wrapper emission, deduplicating by export name.
    let already_emitted = CLOSURE_WRAPPERS_EMITTED
        .with(|set| !set.borrow_mut().insert(export_name.clone()));
    if !already_emitted {
        let wrapper = emit_closure_wrapper(
            wasm_bindgen,
            &export_name,
            is_mut,
            arg_tys,
            ret_ty,
        );
        CLOSURE_WRAPPER_EMISSIONS.with(|cell| cell.borrow_mut().push(wrapper));
    }

    // Schema parts: REF (or REFMUT) + FUNCTION + SYMBOL_REF + name
    // payload + 3-word header that the legacy `describe_invoke`
    // would have emitted next (shim_idx slot now lives in SYMBOL_REF;
    // we still need the nargs word, plus the arg/ret schemas).
    //
    // Layout matches:
    //   REF/REFMUT,
    //   FUNCTION,
    //   SYMBOL_REF, name_len, <packed name>,
    //   nargs,
    //   <arg schemas...>,
    //   <ret schema>,
    //   <ret schema>          (legacy stream emits ret twice)
    let ref_opcode = if is_mut {
        quote! { #wasm_bindgen::describe::REFMUT }
    } else {
        quote! { #wasm_bindgen::describe::REF }
    };
    quote! {
        &[
            #ref_opcode,
            #wasm_bindgen::describe::FUNCTION,
            #wasm_bindgen::describe::SYMBOL_REF,
            #(#packed_words),*
        ],
        &[#nargs],
        #(#arg_parts)*
        #ret_parts
        #ret_parts
    }
}

/// Emit the closure invoke wrapper at module scope:
///
/// ```rust,ignore
/// #[cfg(target_family = "wasm")]
/// #[export_name = "__wbg_invoke_<hash>"]
/// pub unsafe extern "C-unwind" fn __wbg_invoke_<hash>(
///     a: WasmWord, b: WasmWord,
///     /* per-arg splatted primitives... */
/// ) -> WasmRet<<R as ReturnWasmAbi>::Abi> {
///     /* transmute (a, b) into &dyn Fn(...), reassemble args, call,
///        wrap the result via maybe_catch_unwind (UNWIND_SAFE=true). */
/// }
/// ```
///
/// The wrapper is exported by name so cli-support can find it via
/// `module.exports`, and is placed in the function table because
/// wasm-ld puts functions that have their address taken into element
/// segments. We force that by following the wrapper definition with
/// a `#[used] static` of the wrapper's function-pointer type.
fn emit_closure_wrapper(
    wasm_bindgen: &syn::Path,
    export_name: &str,
    is_mut: bool,
    arg_tys: &[syn::Type],
    ret_ty: &syn::Type,
) -> TokenStream {
    let wrapper_ident = Ident::new(
        &format!("__wbg_invoke_wrap_{}", mangle_export_name_for_ident(export_name)),
        Span::call_site(),
    );
    let static_ident = Ident::new(
        &format!("__wbg_invoke_keepalive_{}", mangle_export_name_for_ident(export_name)),
        Span::call_site(),
    );

    // Per-arg splatted primitives. Mirrors the `splat()` helper used
    // for exports/imports but inlined here because we also need to
    // reassemble the primitives back into the original ABI type in
    // the wrapper body.
    //
    // Each arg is one of two shapes:
    //   - owned `T`:  uses `<T as FromWasmAbi>::Abi`, reassembled via
    //                 `from_abi(join(...))`.
    //   - reference `&T`: uses `<T as RefFromWasmAbi>::Abi`, reassembled
    //                     via `&*ref_from_abi(join(...))`.
    // Matches the runtime closure invoke shim's `closures!` macro
    // dispatch on `$FromWasmAbi` (FromWasmAbi vs RefFromWasmAbi).
    let mut sig_args: Vec<TokenStream> = Vec::new();
    let mut reassemble_args: Vec<TokenStream> = Vec::new();
    let mut call_args: Vec<TokenStream> = Vec::new();
    let mut keepalive_arg_tys: Vec<TokenStream> = Vec::new();
    for (i, ty) in arg_tys.iter().enumerate() {
        let prim1 = format_ident!("arg{i}_1");
        let prim2 = format_ident!("arg{i}_2");
        let prim3 = format_ident!("arg{i}_3");
        let prim4 = format_ident!("arg{i}_4");
        let (abi, reassemble) = if let syn::Type::Reference(reference) = get_ty(ty) {
            let inner = &reference.elem;
            let abi = quote! { <#inner as #wasm_bindgen::convert::RefFromWasmAbi>::Abi };
            let local = format_ident!("arg{i}");
            let reassemble = quote! {
                let #local = <#inner as #wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                    <#abi as #wasm_bindgen::convert::WasmAbi>::join(
                        #prim1, #prim2, #prim3, #prim4,
                    )
                );
                let #local = &*#local;
            };
            call_args.push(quote! { #local });
            (abi, reassemble)
        } else {
            let abi = quote! { <#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi };
            let local = format_ident!("arg{i}");
            let reassemble = quote! {
                let #local = <#ty as #wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    <#abi as #wasm_bindgen::convert::WasmAbi>::join(
                        #prim1, #prim2, #prim3, #prim4,
                    )
                );
            };
            call_args.push(quote! { #local });
            (abi, reassemble)
        };
        sig_args.push(quote! {
            #prim1: <#abi as #wasm_bindgen::convert::WasmAbi>::Prim1
        });
        sig_args.push(quote! {
            #prim2: <#abi as #wasm_bindgen::convert::WasmAbi>::Prim2
        });
        sig_args.push(quote! {
            #prim3: <#abi as #wasm_bindgen::convert::WasmAbi>::Prim3
        });
        sig_args.push(quote! {
            #prim4: <#abi as #wasm_bindgen::convert::WasmAbi>::Prim4
        });
        reassemble_args.push(reassemble);
        keepalive_arg_tys.push(quote! { <#abi as #wasm_bindgen::convert::WasmAbi>::Prim1 });
        keepalive_arg_tys.push(quote! { <#abi as #wasm_bindgen::convert::WasmAbi>::Prim2 });
        keepalive_arg_tys.push(quote! { <#abi as #wasm_bindgen::convert::WasmAbi>::Prim3 });
        keepalive_arg_tys.push(quote! { <#abi as #wasm_bindgen::convert::WasmAbi>::Prim4 });
    }

    let mut_kw = if is_mut { quote!(mut) } else { quote!() };
    let fn_kw = if is_mut { quote!(FnMut) } else { quote!(Fn) };
    let arg_ty_list = arg_tys.iter().map(|t| quote! { #t });

    let abi_ret = quote! { <#ret_ty as #wasm_bindgen::convert::ReturnWasmAbi>::Abi };

    quote! {
        // The wrapper itself: signature matches the legacy `invoke`
        // shim for this closure monomorphisation. Body transmutes
        // (a, b) into the closure trait object and forwards args.
        // panic=unwind is opt-in for the runtime; we always use the
        // catching variant (`maybe_catch_unwind`) since that matches
        // the runtime's `WasmDescribe for dyn Fn(...)` choice.
        #[cfg(target_family = "wasm")]
        #[automatically_derived]
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(clippy::too_many_arguments)]
        #[export_name = #export_name]
        pub unsafe extern "C-unwind" fn #wrapper_ident(
            a: #wasm_bindgen::__rt::WasmWord,
            b: #wasm_bindgen::__rt::WasmWord,
            #(#sig_args),*
        ) -> #wasm_bindgen::convert::WasmRet<#abi_ret> {
            use #wasm_bindgen::convert::{FromWasmAbi, ReturnWasmAbi, WasmAbi};
            if a.is_zero() {
                #wasm_bindgen::throw_str(
                    "closure invoked recursively or after being dropped",
                );
            }
            let f: & #mut_kw dyn #fn_kw(#(#arg_ty_list),*) -> #ret_ty =
                ::core::mem::transmute((a.into_usize(), b.into_usize()));
            #(#reassemble_args)*
            let ret = #wasm_bindgen::__rt::maybe_catch_unwind(
                ::core::panic::AssertUnwindSafe(|| f(#(#call_args),*)),
            );
            <#ret_ty as ReturnWasmAbi>::return_abi(ret).into()
        }

        // Keep-alive: wasm-ld only places a function in the function
        // table when its address is taken in code it can see. The
        // wrapper is exported by name, which keeps it from DCE, but
        // by itself doesn't force a table entry. Taking its address
        // through a `#[used] static` does.
        //
        // Type-erased: we store the wrapper's address as a raw
        // pointer so the static's type doesn't have to mirror the
        // wrapper's (per-monomorphisation) signature. wasm-ld treats
        // the address-of as a function-pointer relocation regardless.
        //
        // After cli-support harvests the wrapper's table slot via
        // `function_table_slot_of`, it strips this export and the
        // walrus GC pass drops the wrapper if the slot is now
        // unreferenced. Common case: the slot is referenced from the
        // exported function table (via a JS-callable adapter
        // generated by cli-support) and the wrapper stays live, just
        // without its descriptor-emission-related export name.
        #[cfg(target_family = "wasm")]
        #[automatically_derived]
        #[doc(hidden)]
        #[used]
        static #static_ident: unsafe extern "C-unwind" fn(
            #wasm_bindgen::__rt::WasmWord,
            #wasm_bindgen::__rt::WasmWord,
            #(#keepalive_arg_tys),*
        ) -> #wasm_bindgen::convert::WasmRet<#abi_ret> = #wrapper_ident;
    }
}

/// Mangle an export name into something usable as a Rust identifier. Export
/// names are arbitrary UTF-8 strings, but identifiers must be alphanumeric +
/// underscores. Non-conforming bytes become `_<hex>`.
fn mangle_export_name_for_ident(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for b in name.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' => out.push(b as char),
            _ => {
                out.push('_');
                out.push_str(&format!("{b:02x}"));
            }
        }
    }
    out
}

impl TryToTokens for ast::ImportKind {
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        match *self {
            ast::ImportKind::Function(ref f) => f.try_to_tokens(tokens)?,
            ast::ImportKind::Static(ref s) => s.to_tokens(tokens),
            ast::ImportKind::String(ref s) => s.to_tokens(tokens),
            ast::ImportKind::Type(ref t) => t.try_to_tokens(tokens)?,
            ast::ImportKind::Enum(ref e) => e.to_tokens(tokens),
            ast::ImportKind::DynamicUnion(ref e) => e.to_tokens(tokens),
        }

        Ok(())
    }
}

impl TryToTokens for ast::ImportType {
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        let vis = &self.vis;
        let rust_name = &self.rust_name;
        let attrs = &self.attrs;
        let doc_comment = match &self.doc_comment {
            None => "",
            Some(comment) => comment,
        };
        let instanceof_shim = Ident::new(&self.instanceof_shim, Span::call_site());

        let wasm_bindgen = &self.wasm_bindgen;
        let internal_obj = match self.extends.first() {
            Some(target) => {
                quote! { #target }
            }
            None => {
                quote! { #wasm_bindgen::JsValue }
            }
        };

        let (description, schema_literal, vector_schema_literal) =
            if let Some(typescript_type) = &self.typescript_type {
                let typescript_type_len = typescript_type.len() as u32;
                let typescript_type_chars: Vec<u32> =
                    typescript_type.chars().map(|c| c as u32).collect();
                (
                    quote! {
                        use #wasm_bindgen::describe::*;
                        inform(NAMED_EXTERNREF);
                        inform(#typescript_type_len);
                        #(inform(#typescript_type_chars);)*
                    },
                    // Schema parts for the section transport: a literal
                    // NAMED_EXTERNREF opcode, the name length, and one u32
                    // per UTF-32 char. Kept in lockstep with the describe()
                    // emission above so the two transports never disagree
                    // on shape.
                    quote! {
                        &[
                            #wasm_bindgen::describe::NAMED_EXTERNREF,
                            #typescript_type_len,
                            #(#typescript_type_chars,)*
                        ]
                    },
                    // VECTOR_SCHEMA prefix is VECTOR + this type's own
                    // schema. Needed so `Vec<Self>` and `Box<[Self]>`
                    // args render correctly via the section transport
                    // without falling back to the runtime describe stream.
                    quote! {
                        &[
                            #wasm_bindgen::describe::VECTOR,
                            #wasm_bindgen::describe::NAMED_EXTERNREF,
                            #typescript_type_len,
                            #(#typescript_type_chars,)*
                        ]
                    },
                )
            } else {
                (
                    quote! {
                        JsValue::describe()
                    },
                    quote! {
                        <#wasm_bindgen::JsValue as #wasm_bindgen::describe::WasmDescribe>::SCHEMA
                    },
                    quote! {
                        &[
                            #wasm_bindgen::describe::VECTOR,
                            #wasm_bindgen::describe::EXTERNREF,
                        ]
                    },
                )
            };

        let is_type_of = self.is_type_of.as_ref().map(|is_type_of| {
            quote! {
                #[inline]
                fn is_type_of(val: &JsValue) -> bool {
                    let is_type_of: fn(&JsValue) -> bool = #is_type_of;
                    is_type_of(val)
                }
            }
        });

        let no_deref = self.no_deref;
        let no_promising = self.no_promising;
        let no_into_js_generic = self.no_into_js_generic;

        let doc = if doc_comment.is_empty() {
            quote! {}
        } else {
            quote! {
                #[doc = #doc_comment]
            }
        };

        let class_generic_params = generics::generic_params(&self.generics);
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let type_params_with_bounds = generics::type_params_with_bounds(&self.generics);
        let impl_generics_with_lifetime_a = if type_params_with_bounds.is_empty() {
            quote! { <'a> }
        } else {
            quote! { <'a, #(#type_params_with_bounds),*> }
        };

        // For struct definitions, we need generics with defaults, so use params directly
        let struct_generics = if self.generics.params.is_empty() {
            quote! {}
        } else {
            let params = &self.generics.params;
            quote! { <#params> }
        };

        let phantom;
        let phantom_init;
        let lifetime_params = generics::lifetime_params(&self.generics);

        // For `From<JsValue>`, only include lifetime params so type params
        // fall back to their defaults and callers don't need turbofish.
        let from_jsvalue_generics = if lifetime_params.is_empty() {
            quote! {}
        } else {
            quote! { <#(#lifetime_params),*> }
        };

        if !class_generic_params.is_empty() || !lifetime_params.is_empty() {
            let generic_param_names: Vec<_> = class_generic_params.iter().map(|p| p.0).collect();
            let lifetime_refs = lifetime_params.iter().map(|lt| quote! { &#lt () });
            phantom = quote! {
                generics: ::core::marker::PhantomData<(#(#generic_param_names,)* #(#lifetime_refs),*)>
            };
            phantom_init = quote! { generics: ::core::marker::PhantomData };
        } else {
            phantom = quote! {};
            phantom_init = quote! {};
        }

        // Identity implementation of `IntoJsGeneric`. Declaring this per-type,
        // rather than via a blanket over `T: JsGeneric`, preserves the option
        // for future wrapper types to pick a non-identity `JsCanon`.
        //
        // The body takes `self` by value and reinterprets the transparent JS
        // handle wrapper into its canonical type. This lets the impl apply
        // uniformly to types that do not implement Rust-level `Clone` (e.g.
        // generic types whose parameters aren't `Clone`, or plain handle
        // wrappers that simply don't derive `Clone`).
        //
        // Types whose Rust wrapper enforces owned-once destruction semantics
        // (currently just `JsClosure`) opt out via the
        // `#[wasm_bindgen(no_into_js_generic)]` attribute — producing a
        // duplicate wrapper over the same handle would violate those semantics.
        //
        // The extra `Self: JsGeneric` predicate propagates any generic
        // type-parameter requirements the `JsGeneric` blanket imposes
        // through `ErasableGeneric<Repr = JsValue>` etc.
        let into_js_generic_impl = if no_into_js_generic {
            quote! {}
        } else {
            let mut clause =
                self.generics
                    .where_clause
                    .clone()
                    .unwrap_or_else(|| syn::WhereClause {
                        where_token: Default::default(),
                        predicates: Default::default(),
                    });
            let self_ty_generics = &ty_generics;
            let self_ty: syn::Type = syn::parse_quote!(#rust_name #self_ty_generics);
            let wasm_bindgen_path: syn::Path = syn::parse_quote!(#wasm_bindgen);
            clause.predicates.push(syn::parse_quote!(
                #self_ty: #wasm_bindgen_path::JsGeneric
            ));
            quote! {
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::IntoJsGeneric
                    for #rust_name #ty_generics
                #clause
                {
                    type JsCanon = #rust_name #ty_generics;
                    #[inline]
                    fn to_js(self) -> #rust_name #ty_generics {
            unsafe { core::mem::transmute_copy(&core::mem::ManuallyDrop::new(self)) }
                    }
                }
            }
        };

        (quote! {
            #(#attrs)*
            #doc
            #[repr(transparent)]
            #vis struct #rust_name #struct_generics #where_clause {
                obj: #internal_obj,
                #phantom
            }

            #[automatically_derived]
            const _: () = {
                use #wasm_bindgen::convert::TryFromJsValue;
                use #wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
                use #wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
                use #wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
                use #wasm_bindgen::describe::WasmDescribe;
                use #wasm_bindgen::{JsValue, JsCast};
                use #wasm_bindgen::__rt::{core, marker::ErasableGeneric};

                #[automatically_derived]
                impl #impl_generics WasmDescribe for #rust_name #ty_generics #where_clause {
                    const SCHEMA: &'static [u32] = #schema_literal;
                    fn describe() {
                        #description
                    }
                }

                // `Vec<Self>` / `Box<[Self]>` schema for the section
                // transport. Mirrors `WasmDescribe::SCHEMA` above with a
                // VECTOR prefix so `Vec<#rust_name>` args don't fall
                // back to the interpreter.
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::describe::WasmDescribeVector
                    for #rust_name #ty_generics #where_clause
                {
                    const VECTOR_SCHEMA: &'static [u32] = #vector_schema_literal;
                    fn describe_vector() {
                        use #wasm_bindgen::describe::*;
                        inform(VECTOR);
                        <Self as WasmDescribe>::describe();
                    }
                }

                #[automatically_derived]
                impl #impl_generics IntoWasmAbi for #rust_name #ty_generics #where_clause {
                    type Abi = <JsValue as IntoWasmAbi>::Abi;

                    #[inline]
                    fn into_abi(self) -> Self::Abi {
                        self.obj.into_abi()
                    }
                }

                #[automatically_derived]
                impl #impl_generics OptionIntoWasmAbi for #rust_name #ty_generics #where_clause {
                    #[inline]
                    fn none() -> Self::Abi {
                        0
                    }
                }

                #[automatically_derived]
                impl #impl_generics_with_lifetime_a OptionIntoWasmAbi for &'a #rust_name #ty_generics #where_clause {
                    #[inline]
                    fn none() -> Self::Abi {
                        0
                    }
                }

                #[automatically_derived]
                impl #impl_generics FromWasmAbi for #rust_name #ty_generics #where_clause {
                    type Abi = <JsValue as FromWasmAbi>::Abi;

                    #[inline]
                    unsafe fn from_abi(js: Self::Abi) -> Self {
                        #rust_name {
                            obj: JsValue::from_abi(js).into(),
                            #phantom_init
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics OptionFromWasmAbi for #rust_name #ty_generics #where_clause {
                    #[inline]
                    fn is_none(abi: &Self::Abi) -> bool { *abi == 0 }
                }

                #[automatically_derived]
                impl #impl_generics_with_lifetime_a IntoWasmAbi for &'a #rust_name #ty_generics #where_clause {
                    type Abi = <&'a JsValue as IntoWasmAbi>::Abi;

                    #[inline]
                    fn into_abi(self) -> Self::Abi {
                        (&self.obj).into_abi()
                    }
                }

                #[automatically_derived]
                impl #impl_generics RefFromWasmAbi for #rust_name #ty_generics #where_clause {
                    type Abi = <JsValue as RefFromWasmAbi>::Abi;
                    type Anchor = core::mem::ManuallyDrop<#rust_name #ty_generics>;

                    #[inline]
                    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                        let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                        core::mem::ManuallyDrop::new(#rust_name {
                            obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                            #phantom_init
                        })
                    }
                }

                #[automatically_derived]
                impl #impl_generics LongRefFromWasmAbi for #rust_name #ty_generics #where_clause {
                    type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
                    type Anchor = #rust_name #ty_generics;

                    #[inline]
                    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                        let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                        #rust_name {
                            obj: tmp.into(),
                            #phantom_init
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics AsRef<JsValue> for #rust_name #ty_generics #where_clause {
                    #[inline]
                    fn as_ref(&self) -> &JsValue { self.obj.as_ref() }
                }

                #[automatically_derived]
                impl #impl_generics AsRef<#rust_name #ty_generics> for #rust_name #ty_generics #where_clause {
                    #[inline]
                    fn as_ref(&self) -> &#rust_name #ty_generics { self }
                }

                #into_js_generic_impl

                // TODO: remove this on the next major version
                // Only include lifetime params here; type params use their
                // defaults so callers don't need turbofish annotations.
                #[automatically_derived]
                impl #from_jsvalue_generics From<JsValue> for #rust_name #from_jsvalue_generics {
                    #[inline]
                    fn from(obj: JsValue) -> Self {
                        #rust_name {
                            obj: obj.into(),
                            #phantom_init
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics From<#rust_name #ty_generics> for JsValue #where_clause {
                    #[inline]
                    fn from(obj: #rust_name #ty_generics) -> JsValue {
                        obj.obj.into()
                    }
                }

                // `VectorIntoJsValue` enables `Vec<#rust_name>` and
                // `Box<[#rust_name]>` to convert to JsValue via the
                // generic-array push loop. Without this impl the
                // generic blanket `impl<T: VectorIntoJsValue> From<Box<[T]>>`
                // wouldn't apply to `Vec<#rust_name>`.
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::__rt::VectorIntoJsValue for #rust_name #ty_generics #where_clause {
                    fn vector_into_jsvalue(
                        vector: #wasm_bindgen::__rt::alloc::boxed::Box<[#rust_name #ty_generics]>,
                    ) -> #wasm_bindgen::JsValue {
                        #wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
                    }
                }

                #[automatically_derived]
                impl #impl_generics JsCast for #rust_name #ty_generics #where_clause {
                    fn instanceof(val: &JsValue) -> bool {
                        #[link(wasm_import_module = "__wbindgen_placeholder__")]
                        #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
                        extern "C" {
                            fn #instanceof_shim(val: u32) -> u32;
                        }
                        #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
                        unsafe fn #instanceof_shim(_: u32) -> u32 {
                            panic!("cannot check instanceof on non-wasm targets");
                        }
                        unsafe {
                            let idx = val.into_abi();
                            #instanceof_shim(idx) != 0
                        }
                    }

                    #is_type_of

                    #[inline]
                    fn unchecked_from_js(val: JsValue) -> Self {
                        #rust_name {
                            obj: val.into(),
                            #phantom_init
                        }
                    }

                    #[inline]
                    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                        // Should be safe because `#rust_name` is a transparent
                        // wrapper around `val`
                        unsafe { &*(val as *const JsValue as *const Self) }
                    }
                }

                unsafe impl #impl_generics ErasableGeneric for #rust_name #ty_generics #where_clause {
                    type Repr = JsValue;
                }
            };
        })
        .to_tokens(tokens);

        if !no_promising {
            (quote! {
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::sys::Promising for #rust_name #ty_generics #where_clause {
                    type Resolution = #rust_name #ty_generics;
                }
            })
            .to_tokens(tokens);
        }

        if !no_deref {
            (quote! {
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::__rt::core::ops::Deref for #rust_name #ty_generics #where_clause {
                    type Target = #internal_obj;

                    #[inline]
                    fn deref(&self) -> &#internal_obj {
                        &self.obj
                    }
                }
            })
            .to_tokens(tokens);
        }

        for superclass in self.extends.iter() {
            (quote! {
                #[automatically_derived]
                impl #impl_generics From<#rust_name #ty_generics> for #superclass #where_clause {
                    #[inline]
                    fn from(obj: #rust_name #ty_generics) -> #superclass {
                        use #wasm_bindgen::JsCast;
                        #superclass::unchecked_from_js(obj.into())
                    }
                }

                #[automatically_derived]
                impl #impl_generics AsRef<#superclass> for #rust_name #ty_generics #where_clause {
                    #[inline]
                    fn as_ref(&self) -> &#superclass {
                        use #wasm_bindgen::JsCast;
                        #superclass::unchecked_from_js_ref(self.as_ref())
                    }
                }
            })
            .to_tokens(tokens);
        }

        // Generate UpcastFrom implementations (unless no_upcast is set)
        if !self.no_upcast {
            // 1. Always generate UpcastFrom<Self> for JsValue
            (quote! {
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                    for #wasm_bindgen::JsValue
                #where_clause
                {
                }
            })
            .to_tokens(tokens);

            // 2. For non-generic types: generate identity upcast (UpcastFrom<Self> for Self, UpcastFrom<Self> for JsOption<Self>)
            // 3. For generic types: generate structural covariance
            let type_params: Vec<_> = self.generics.type_params().collect();
            if type_params.is_empty() {
                // Identity impls for non-generic (or lifetime-only) types.
                // Always use #ty_generics so that lifetime params are included.
                (quote! {
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #rust_name #ty_generics
                    #where_clause
                    {
                    }
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsOption<#rust_name #ty_generics>
                    #where_clause
                    {
                    }
                })
                .to_tokens(tokens);
            } else {
                // Structural covariance impl for generic types
                // Build impl generics: all original params plus a Target param for each
                let mut impl_generics_extended = self.generics.clone();
                let target_param_names: Vec<syn::Ident> = type_params
                    .iter()
                    .enumerate()
                    .map(|(i, tp)| {
                        let target_name = quote::format_ident!("__UpcastTarget{}", i);
                        // Copy bounds from the original type param to the target param
                        // If no bounds, just add the type param without colon
                        if tp.bounds.is_empty() {
                            impl_generics_extended
                                .params
                                .push(syn::parse_quote!(#target_name));
                        } else {
                            let bounds = &tp.bounds;
                            impl_generics_extended
                                .params
                                .push(syn::parse_quote!(#target_name: #bounds));
                        }
                        target_name
                    })
                    .collect();

                // Build where clause: Target: UpcastFrom<T>
                let mut where_clause_extended =
                    self.generics
                        .where_clause
                        .clone()
                        .unwrap_or_else(|| syn::WhereClause {
                            where_token: Default::default(),
                            predicates: Default::default(),
                        });

                for (type_param, target_name) in type_params.iter().zip(&target_param_names) {
                    let param_ident = &type_param.ident;
                    where_clause_extended.predicates.push(syn::parse_quote!(
                        #target_name: #wasm_bindgen::convert::UpcastFrom<#param_ident>
                    ));
                }

                let (impl_generics_split, _, _) = impl_generics_extended.split_for_impl();

                // Build target ty_generics: lifetime params forwarded, type params replaced
                let target_lifetime_params = generics::lifetime_params(&self.generics);
                let target_ty_generics =
                    quote! { <#(#target_lifetime_params,)* #(#target_param_names),*> };

                // Structural covariance - Type<Target0, Target1, ...> can be upcast from Type<T1, T2, ...>
                (quote! {
                    #[automatically_derived]
                    impl #impl_generics_split #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #rust_name #target_ty_generics
                    #where_clause_extended
                    {
                    }
                    #[automatically_derived]
                    impl #impl_generics_split #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsOption<#rust_name #target_ty_generics>
                    #where_clause_extended
                    {
                    }
                })
                .to_tokens(tokens);
            }

            // 4. For each superclass in extends, generate UpcastFrom<Self> for superclass
            for superclass in self.extends.iter() {
                (quote! {
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #superclass
                    #where_clause
                    {
                    }
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsOption<#superclass>
                    #where_clause
                    {
                    }
                })
                .to_tokens(tokens);
            }
        }

        Ok(())
    }
}

// String enums predate dynamic unions and overlap structurally: a string
// enum is equivalent to a dynamic union with only string-literal variants,
// minus a few details. Future cleanup (separate PR) could subsume string
// enums into the dynamic-union codegen. Differences to reconcile first:
//
// * `__Invalid`: string enums silently accept unknown JS strings as a hidden
//   `__Invalid` variant. Dynamic unions throw, or accept an explicit
//   `#[wasm_bindgen(fallback)]` catch-all variant. Migrating means dropping
//   `__Invalid` (telling users to switch to `fallback`).
// * Inherent helpers: `from_str` / `to_str` / `from_js_value` are emitted
//   here as inherent methods. Dynamic unions don't generate equivalents.
//   Either preserve them or document removal as breaking.
// * `TryFromJsValue`: string enums currently lack this impl, so they
//   can't be `dyn_into` targets or dynamic-union variant payloads.
//   Dynamic unions have it. Unification would gain this on the string
//   enum path for free.
// * ABI: string enums use a u32 discriminant; dynamic unions use an
//   externref. Benchmarks (see `benches/enum_roundtrip.rs`) show the
//   round-trip cost is within ~1% on Node, so the perf argument for
//   keeping the discriminant ABI is weak.
impl ToTokens for ast::StringEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = &self.vis;
        let enum_name = &self.name;
        let name_str = &self.export_name;
        let name_len = name_str.len() as u32;
        let name_chars: Vec<u32> = name_str.chars().map(u32::from).collect();
        let variants = &self.variants;
        let variant_count = self.variant_values.len() as u32;
        let variant_values = &self.variant_values;
        let variant_indices = (0..variant_count).collect::<Vec<_>>();
        let invalid = variant_count;
        let hole = variant_count + 1;
        let attrs = &self.rust_attrs;

        let invalid_to_str_msg = format!(
            "Converting an invalid string enum ({enum_name}) back to a string is currently not supported"
        );

        // A vector of EnumName::VariantName tokens for this enum
        let variant_paths: Vec<TokenStream> = self
            .variants
            .iter()
            .map(|v| quote!(#enum_name::#v).into_token_stream())
            .collect();

        // Borrow variant_paths because we need to use it multiple times inside the quote! macro
        let variant_paths_ref = &variant_paths;

        let wasm_bindgen = &self.wasm_bindgen;

        (quote! {
            #(#attrs)*
            #[non_exhaustive]
            #[repr(u32)]
            #vis enum #enum_name {
                #(#variants = #variant_indices,)*
                #[automatically_derived]
                #[doc(hidden)]
                __Invalid
            }

            #[automatically_derived]
            impl #enum_name {
                fn from_str(s: &str) -> Option<#enum_name> {
                    match s {
                        #(#variant_values => Some(#variant_paths_ref),)*
                        _ => None,
                    }
                }

                fn to_str(&self) -> &'static str {
                    match self {
                        #(#variant_paths_ref => #variant_values,)*
                        #enum_name::__Invalid => panic!(#invalid_to_str_msg),
                    }
                }

                #vis fn from_js_value(obj: &#wasm_bindgen::JsValue) -> Option<#enum_name> {
                    obj.as_string().and_then(|obj_str| Self::from_str(obj_str.as_str()))
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::IntoWasmAbi for #enum_name {
                type Abi = u32;

                #[inline]
                fn into_abi(self) -> u32 {
                    self as u32
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::FromWasmAbi for #enum_name {
                type Abi = u32;

                unsafe fn from_abi(val: u32) -> Self {
                    match val {
                        #(#variant_indices => #variant_paths_ref,)*
                        #invalid => #enum_name::__Invalid,
                        _ => unreachable!("The JS binding should only ever produce a valid value or the specific 'invalid' value"),
                    }
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionFromWasmAbi for #enum_name {
                #[inline]
                fn is_none(val: &u32) -> bool { *val == #hole }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionIntoWasmAbi for #enum_name {
                #[inline]
                fn none() -> Self::Abi { #hole }
            }

            #[automatically_derived]
            impl #wasm_bindgen::describe::WasmDescribe for #enum_name {
                // String-enum schema: STRING_ENUM, name_len, ...name chars
                // (one u32 per char), variant_count. Kept in lockstep
                // with the describe() body below.
                const SCHEMA: &'static [u32] = &[
                    #wasm_bindgen::describe::STRING_ENUM,
                    #name_len,
                    #(#name_chars,)*
                    #variant_count,
                ];
                fn describe() {
                    use #wasm_bindgen::describe::*;
                    inform(STRING_ENUM);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                    inform(#variant_count);
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::__rt::core::convert::From<#enum_name> for
                #wasm_bindgen::JsValue
            {
                fn from(val: #enum_name) -> Self {
                    #wasm_bindgen::JsValue::from_str(val.to_str())
                }
            }
        })
        .to_tokens(tokens);
    }
}

impl ToTokens for ast::DynamicUnion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = &self.vis;
        let enum_name = &self.name;
        let wasm_bindgen = &self.wasm_bindgen;
        let attrs = &self.rust_attrs;

        // Separate string-literal variants from tuple (typed payload) variants
        let (known_variants, fallback_variants): (Vec<_>, Vec<_>) = self
            .variants
            .iter()
            .zip(&self.variant_fields)
            .partition(|(_, fields)| fields.is_empty());

        let known_variant_names: Vec<_> = known_variants.iter().map(|(v, _)| v).collect();
        let known_variant_values: Vec<_> = known_variants
            .iter()
            .map(|(v, _)| {
                let idx = self.variants.iter().position(|x| x == *v).unwrap();
                &self.variant_values[idx]
            })
            .collect();

        // Build enum definition with all variants
        let fallback_variant_defs = fallback_variants.iter().map(|(name, fields)| {
            let ty = &fields[0];
            quote! { #name(#ty) }
        });

        let enum_def = quote! {
            #(#known_variant_names,)*
            #(#fallback_variant_defs,)*
        };

        // IntoWasmAbi - convert everything to JsValue
        let known_into_arms: Vec<_> = known_variant_names
            .iter()
            .zip(&known_variant_values)
            .map(|(vname, value)| {
                quote! {
                    #enum_name::#vname => #wasm_bindgen::JsValue::from_str(#value)
                }
            })
            .collect();

        let fallback_into_arms: Vec<_> = fallback_variants
            .iter()
            .map(|(name, _)| {
                quote! {
                    #enum_name::#name(value) => #wasm_bindgen::JsValue::from(value)
                }
            })
            .collect();

        // FromWasmAbi - try to match JsValue to each variant. All string
        // literal variants share a single `as_string` call coalesced into one
        // `match`, so the worst-case dispatch cost is a single string read
        // regardless of how many literal variants exist.
        let known_from_block = if known_variant_names.is_empty() {
            quote! {}
        } else {
            let arms =
                known_variant_names
                    .iter()
                    .zip(&known_variant_values)
                    .map(|(vname, value)| {
                        quote! { #value => return #enum_name::#vname, }
                    });
            quote! {
                if let Some(s) = js_value.as_string() {
                    match s.as_str() {
                        #(#arms)*
                        _ => {}
                    }
                }
            }
        };

        // When `#[wasm_bindgen(fallback)]` is set on the enum and there is
        // at least one tuple variant, the *last* tuple variant becomes an
        // unconditional catch-all: anything that didn't match an earlier
        // variant is unconditionally accepted as that variant's payload via
        // an unchecked cast. This lets unions terminate in a type whose
        // `instanceof` check is meaningless (e.g., interface-only imports).
        let last_fallback_idx = if self.fallback && !fallback_variants.is_empty() {
            Some(fallback_variants.len() - 1)
        } else {
            None
        };

        let fallback_from_arms: Vec<_> = fallback_variants
            .iter()
            .enumerate()
            .map(|(idx, (name, fields))| {
                let ty = &fields[0];
                if Some(idx) == last_fallback_idx {
                    quote! {
                        return #enum_name::#name(
                            <#wasm_bindgen::JsValue as #wasm_bindgen::JsCast>::unchecked_into::<#ty>(js_value)
                        );
                    }
                } else {
                    quote! {
                        if let Ok(value) = <#ty as #wasm_bindgen::convert::TryFromJsValue>::try_from_js_value(js_value.clone()) {
                            return #enum_name::#name(value);
                        }
                    }
                }
            })
            .collect();

        // Same dispatch as `fallback_from_arms` but for `TryFromJsValue`,
        // which returns `Err(value)` on full failure rather than throwing.
        // The same fallback rule applies.
        let fallback_try_from_arms: Vec<_> = fallback_variants
            .iter()
            .enumerate()
            .map(|(idx, (name, fields))| {
                let ty = &fields[0];
                if Some(idx) == last_fallback_idx {
                    quote! {
                        return #wasm_bindgen::__rt::core::result::Result::Ok(
                            #enum_name::#name(
                                <#wasm_bindgen::JsValue as #wasm_bindgen::JsCast>::unchecked_into::<#ty>(value)
                            )
                        );
                    }
                } else {
                    quote! {
                        if let Ok(inner) = <#ty as #wasm_bindgen::convert::TryFromJsValue>::try_from_js_value(value.clone()) {
                            return #wasm_bindgen::__rt::core::result::Result::Ok(#enum_name::#name(inner));
                        }
                    }
                }
            })
            .collect();

        // The dispatch chain ends with a throw / `Err` only when the enum
        // does *not* have a fallback variant. With a fallback, the last
        // tuple-variant arm always `return`s unconditionally, so any
        // trailing expression would be unreachable.
        let from_abi_tail = if last_fallback_idx.is_some() {
            quote! {}
        } else {
            quote! { #wasm_bindgen::throw_str("invalid dynamic union value") }
        };
        let try_from_tail = if last_fallback_idx.is_some() {
            quote! {}
        } else {
            quote! { #wasm_bindgen::__rt::core::result::Result::Err(value) }
        };

        let name_str = &self.js_name;
        let name_len = name_str.len() as u32;
        let name_chars: Vec<u32> = name_str.chars().map(u32::from).collect();

        let mut string_variants = Vec::new();
        let mut type_variants = Vec::new();
        for (idx, fields) in self.variant_fields.iter().enumerate() {
            if fields.is_empty() {
                string_variants.push(&self.variant_values[idx]);
            } else {
                type_variants.push(&fields[0]);
            }
        }
        let type_count = type_variants.len() as u32;
        // Variant schema parts for the section transport. Uses
        // `schema_parts_for_type` so wrapper types (`Option<T>`,
        // `Vec<T>`, `Result<T, E>`, etc.) resolve their schema via the
        // macro's recursive dispatch rather than via the wrapper's
        // empty trait-level `SCHEMA` const.
        let variant_schema_parts: Vec<TokenStream> = type_variants
            .iter()
            .map(|ty| schema_parts_for_type(wasm_bindgen, ty))
            .collect();

        (quote! {
            #(#attrs)*
            #vis enum #enum_name {
                #enum_def
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::IntoWasmAbi for #enum_name {
                type Abi = u32;

                #[inline]
                fn into_abi(self) -> u32 {
                    let js_value: #wasm_bindgen::JsValue = match self {
                        #(#known_into_arms,)*
                        #(#fallback_into_arms,)*
                    };
                    #wasm_bindgen::convert::IntoWasmAbi::into_abi(js_value)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::FromWasmAbi for #enum_name {
                type Abi = u32;

                #[inline]
                unsafe fn from_abi(js: u32) -> Self {
                    let js_value = <#wasm_bindgen::JsValue as #wasm_bindgen::convert::FromWasmAbi>::from_abi(js);
                    #known_from_block
                    #(#fallback_from_arms)*
                    #from_abi_tail
                }
            }

            // Despite the generic implementation, we still encode the type information for TypeScript output
            #[automatically_derived]
            impl #wasm_bindgen::describe::WasmDescribe for #enum_name {
                // Section-transport schema: same structure as the
                // runtime `describe()` stream, concatenated through
                // the `schema::concat_words` const-fn so each variant
                // type's schema parts get spliced in at their concrete
                // lengths. The variant parts come from
                // `schema_parts_for_type` (the same macro helper used
                // for argument/return-type schemas) so wrapper shapes
                // like `Option<T>` / `Vec<T>` resolve to the correct
                // section bytes rather than relying on the wrapper's
                // own `SCHEMA` const (which is empty for `Option<T>`
                // because of the generic-const-expr wall).
                const SCHEMA: &'static [u32] = {
                    const __HEADER: &[u32] = &[
                        #wasm_bindgen::describe::DYNAMIC_UNION,
                        #name_len,
                        #(#name_chars,)*
                        #type_count,
                    ];
                    const __PARTS: &[&[u32]] = &[
                        __HEADER,
                        #(#variant_schema_parts)*
                    ];
                    const __WORDS: usize =
                        #wasm_bindgen::describe::schema::word_total(__PARTS);
                    const __BUILT: [u32; __WORDS] =
                        #wasm_bindgen::describe::schema::concat_words::<__WORDS>(__PARTS);
                    &__BUILT
                };
                fn describe() {
                    use #wasm_bindgen::describe::*;
                    inform(DYNAMIC_UNION);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                    inform(#type_count);
                    #(<#type_variants as WasmDescribe>::describe();)*
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::__rt::core::convert::From<#enum_name> for #wasm_bindgen::JsValue {
                fn from(value: #enum_name) -> Self {
                    match value {
                        #(#known_into_arms,)*
                        #(#fallback_into_arms,)*
                    }
                }
            }

            // Allows this union to appear inside `Option<...>`. Reuses
            // `JsValue`'s `undefined` sentinel since the union ABI is a
            // single externref slot. This is sound only because dynamic
            // unions cannot match `undefined` as a variant.
            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionIntoWasmAbi for #enum_name {
                #[inline]
                fn none() -> u32 {
                    <#wasm_bindgen::JsValue as #wasm_bindgen::convert::OptionIntoWasmAbi>::none()
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionFromWasmAbi for #enum_name {
                #[inline]
                fn is_none(js: &u32) -> bool {
                    <#wasm_bindgen::JsValue as #wasm_bindgen::convert::OptionFromWasmAbi>::is_none(js)
                }
            }

            // Allows this union to appear as a variant payload of another
            // dynamic union (nested unions) and anywhere else the macro
            // dispatches through `TryFromJsValue`.
            #[automatically_derived]
            impl #wasm_bindgen::convert::TryFromJsValue for #enum_name {
                fn try_from_js_value(
                    value: #wasm_bindgen::JsValue,
                ) -> #wasm_bindgen::__rt::core::result::Result<Self, #wasm_bindgen::JsValue> {
                    if let Some(s) = value.as_string() {
                        #(
                            if s == #known_variant_values {
                                return #wasm_bindgen::__rt::core::result::Result::Ok(
                                    #enum_name::#known_variant_names
                                );
                            }
                        )*
                    }
                    #(#fallback_try_from_arms)*
                    #try_from_tail
                }

                fn try_from_js_value_ref(
                    value: &#wasm_bindgen::JsValue,
                ) -> #wasm_bindgen::__rt::core::option::Option<Self> {
                    Self::try_from_js_value(value.clone()).ok()
                }
            }
        })
        .to_tokens(tokens);

        // Per-variant descriptors so cli-support can look them up.
        // Each is the bare variant type schema (no FUNCTION header),
        // same shape as ImportStatic / struct getters.
        for (idx, ty) in type_variants.iter().enumerate() {
            let descriptor_name = Ident::new(
                &shared::dynamic_union_variant(name_str, idx as u32),
                Span::call_site(),
            );
            emit_static_descriptor_entry_static(
                &self.wasm_bindgen,
                &descriptor_name.to_string(),
                schema_parts_for_type(&self.wasm_bindgen, ty),
                tokens,
            );
        }
    }
}

impl TryToTokens for ast::ImportFunction {
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        let mut class = None;
        let mut is_constructor = false;
        let mut is_method = false;
        let mut is_self_returning_static = false;
        if let ast::ImportFunctionKind::Method {
            class: class_name,
            ty,
            kind,
            ..
        } = &self.kind
        {
            class = Some((class_name, get_ty(ty)));
            match kind {
                ast::MethodKind::Constructor => is_constructor = true,
                ast::MethodKind::Operation(ast::Operation {
                    is_static: false, ..
                }) => is_method = true,
                _ => {}
            };
            // For constructors and static methods whose return type matches the
            // class (e.g. `Array::of<T>() -> Array<T>`), override the class type
            // to use the return type so class-level generics get hoisted.
            if self.class_return_path().is_some() {
                class = Some((class_name, get_ty(self.js_ret.as_ref().unwrap())));
                if !is_constructor {
                    is_self_returning_static = true;
                }
            }
        }

        let vis = &self.function.rust_vis;
        let ret = match self.function.ret.as_ref().map(|ret| &ret.r#type) {
            Some(ty) => quote! { -> #ty },
            None => quote!(),
        };

        let mut abi_argument_names = Vec::new();
        let mut abi_arguments = Vec::new();
        let mut arg_conversions = Vec::new();
        let mut arguments = Vec::new();

        let mut fn_class_generics = self.get_fn_generics()?;
        let (fn_lifetime_param_names, fn_generic_param_names) =
            generics::all_param_names(&self.generics);

        let ret_ident = Ident::new("_ret", Span::call_site());
        let wasm_bindgen = &self.wasm_bindgen;
        let wasm_bindgen_futures = &self.wasm_bindgen_futures;
        let js_sys = &self.js_sys;
        let futures = if ast::use_js_sys_futures() {
            quote! { #js_sys::futures }
        } else {
            quote! { #wasm_bindgen_futures }
        };
        let promise = if ast::use_js_sys_futures() {
            quote! { #js_sys::Promise }
        } else {
            quote! { #wasm_bindgen_futures::js_sys::Promise }
        };

        for (i, arg) in self.function.arguments.iter().enumerate() {
            let ty = &*arg.pat_type.ty;
            let name = match &*arg.pat_type.pat {
                syn::Pat::Ident(syn::PatIdent {
                    by_ref: None,
                    ident,
                    subpat: None,
                    ..
                }) => ident.clone(),
                syn::Pat::Wild(_) => syn::Ident::new(&format!("__genarg_{i}"), Span::call_site()),
                _ => bail_span!(
                    arg.pat_type.pat,
                    "unsupported pattern in #[wasm_bindgen] imported function",
                ),
            };

            let var = if i == 0 && is_method {
                quote! { self }
            } else {
                quote! { #name }
            };

            let abi_ty;
            let convert_arg;

            if generics::uses_generic_params(ty, &fn_generic_param_names)
                || generics::uses_lifetime_params(ty, &fn_lifetime_param_names)
            {
                let (inner_ty, ref_mut, ref_lifetime) =
                    if let syn::Type::Reference(syn::TypeReference {
                        elem,
                        mutability: mut_,
                        lifetime,
                        ..
                    }) = ty
                    {
                        ((**elem).clone(), Some(mut_), lifetime.clone())
                    } else {
                        (ty.clone(), None, None)
                    };
                let concrete_ty = generic_to_concrete(
                    inner_ty.clone(),
                    &fn_class_generics.concrete_defaults,
                    &fn_lifetime_param_names,
                )?;
                if i > 0 || !is_method {
                    fn_class_generics.add_fn_bound(if let Some(mut_) = ref_mut {
                        arguments.push(quote! { #name: & #ref_lifetime #mut_ #inner_ty });
                        if mut_.is_some() {
                            parse_quote! { #inner_ty: #wasm_bindgen::__rt::marker::ErasableGenericBorrowMut<#concrete_ty> }
                        } else {
                            parse_quote! { #inner_ty: #wasm_bindgen::__rt::marker::ErasableGenericBorrow<#concrete_ty> }
                        }
                    } else {
                        arguments.push(quote! { #name: #ty });
                        parse_quote! { #inner_ty: #wasm_bindgen::__rt::marker::ErasableGenericOwn<#concrete_ty> }
                    });
                }
                // abi_ty is fully concrete with 'static lifetimes (used for both extern block and transmute)
                abi_ty = if let Some(mut_) = ref_mut {
                    quote! { &'static #mut_ #concrete_ty }
                } else {
                    quote! { #concrete_ty }
                };

                convert_arg = quote! { unsafe { core::mem::transmute_copy(&core::mem::ManuallyDrop::new(#var)) } };
            } else if let Some((is_mut, fn_bounds)) = detect_raw_fn_trait_obj(ty) {
                // Raw `&dyn Fn(...)` or `&mut dyn FnMut(...)` argument.
                //
                // Emit as `&mut (impl FnMut(...) + MaybeUnwindSafe)` / `&(impl Fn(...) + MaybeUnwindSafe)`
                // so that callers must satisfy UnwindSafe when `panic = "unwind"`, while remaining
                // backward-compatible when `panic != "unwind"` (MaybeUnwindSafe is blanket-impl'd).
                // Using `impl Trait` keeps the signature clean — no hidden generic param or where-clause.
                if i > 0 || !is_method {
                    if is_mut {
                        arguments.push(quote! {
                            #name: &mut (impl #fn_bounds + #wasm_bindgen::__rt::marker::MaybeUnwindSafe)
                        });
                    } else {
                        arguments.push(quote! {
                            #name: &(impl #fn_bounds + #wasm_bindgen::__rt::marker::MaybeUnwindSafe)
                        });
                    }
                }

                // The ABI type is still the erased dyn type — same wire format.
                if is_mut {
                    abi_ty = quote! { &mut dyn #fn_bounds };
                } else {
                    abi_ty = quote! { &dyn #fn_bounds };
                }

                // Coerce the concrete impl Trait type to the dyn trait object for into_abi.
                if is_mut {
                    convert_arg = quote! { #var as &mut dyn #fn_bounds };
                } else {
                    convert_arg = quote! { #var as &dyn #fn_bounds };
                }
            } else {
                if i > 0 || !is_method {
                    arguments.push(quote! { #name: #ty });
                }
                abi_ty = quote! { #ty };

                convert_arg = quote! { #var };
            }

            // `slice_to_array`: re-route an `&[T]` (or `Option<&[T]>`)
            // outgoing argument through `<T as VectorRefIntoWasmAbi>`
            // instead of the default `&[T]: IntoWasmAbi`. The user-facing
            // parameter is unchanged; only the ABI / describe path
            // changes. `VectorRefIntoWasmAbi`'s impls cover the two
            // genuine ABI shapes (zero-copy primitive borrow,
            // fresh-`Box<[u32]>` for handle-shaped element types) — no
            // `T: Clone` bound is introduced.
            //
            // Wire format is `WasmSlice` either way; the cli-support
            // side picks the right JS shim based on the element
            // `VectorKind` recovered from the descriptor.
            // `slice_to_array` is set per-fn or per-`extern "C"` block
            // and applies to every `&[T]` / `Option<&[T]>` argument of
            // every fn it covers. Args that aren't slice-shaped (e.g.
            // the `this: &Foo` of a method, or any other non-slice
            // argument of a slice_to_array fn) silently fall through to
            // the default ABI path — there's no per-arg opt-out form
            // in Rust attribute syntax to require, so silent no-op is
            // the only sensible behaviour.
            if arg.slice_to_array && detect_slice_or_option_slice(ty).is_some() {
                let (elem_ty, is_option) = detect_slice_or_option_slice(ty).unwrap();

                let abi = quote! { #wasm_bindgen::convert::WasmSlice };
                let (prim_args, prim_names) = splat(wasm_bindgen, &name, &abi);
                abi_arguments.extend(prim_args);
                abi_argument_names.extend(prim_names.iter().cloned());

                let body = if is_option {
                    quote! {
                        match #var {
                            ::core::option::Option::Some(s) =>
                                <#elem_ty as #wasm_bindgen::convert::VectorRefIntoWasmAbi>
                                    ::slice_into_abi(s),
                            ::core::option::Option::None =>
                                <#elem_ty as #wasm_bindgen::convert::VectorRefIntoWasmAbi>
                                    ::slice_none(),
                        }
                    }
                } else {
                    quote! {
                        <#elem_ty as #wasm_bindgen::convert::VectorRefIntoWasmAbi>
                            ::slice_into_abi(#var)
                    }
                };

                arg_conversions.push(quote! {
                    let #name: #wasm_bindgen::convert::WasmSlice = #body;
                    let (#(#prim_names),*) =
                        <#wasm_bindgen::convert::WasmSlice as #wasm_bindgen::convert::WasmAbi>
                            ::split(#name);
                });
                continue;
            }

            let abi = quote! { <#abi_ty as #wasm_bindgen::convert::IntoWasmAbi>::Abi };
            let (prim_args, prim_names) = splat(wasm_bindgen, &name, &abi);
            abi_arguments.extend(prim_args);
            abi_argument_names.extend(prim_names.iter().cloned());

            arg_conversions.push(quote! {
                let #name = <#abi_ty as #wasm_bindgen::convert::IntoWasmAbi>
                    ::into_abi(#convert_arg);
                let (#(#prim_names),*) = <#abi as #wasm_bindgen::convert::WasmAbi>::split(#name);
            });
        }
        let abi_ret;
        let mut convert_ret;
        match &self.js_ret {
            Some(syn::Type::Reference(_)) => {
                bail_span!(
                    self.js_ret,
                    "cannot return references in #[wasm_bindgen] imports yet"
                );
            }
            Some(ref original_ty) => {
                let maybe_async_wrapped;
                let ty = if self.function.r#async {
                    maybe_async_wrapped = parse_quote!(#promise<#original_ty>);
                    &maybe_async_wrapped
                } else {
                    original_ty
                };
                if generics::uses_generic_params(ty, &fn_generic_param_names)
                    || generics::uses_lifetime_params(ty, &fn_lifetime_param_names)
                {
                    let concrete_ty = generic_to_concrete(
                        ty.clone(),
                        &fn_class_generics.concrete_defaults,
                        &fn_lifetime_param_names,
                    )?;
                    fn_class_generics.add_fn_bound(
                        parse_quote! { #ty: #wasm_bindgen::__rt::marker::ErasableGenericOwn<#concrete_ty> },
                    );
                    convert_ret = quote! { unsafe { core::mem::transmute_copy(&core::mem::ManuallyDrop::new(<#concrete_ty as #wasm_bindgen::convert::FromWasmAbi>::from_abi(#ret_ident.join()))) } };
                    abi_ret = quote! { #wasm_bindgen::convert::WasmRet<<#concrete_ty as #wasm_bindgen::convert::FromWasmAbi>::Abi> };
                } else {
                    convert_ret = quote! { <#ty as #wasm_bindgen::convert::FromWasmAbi>::from_abi(#ret_ident.join()) };
                    abi_ret = quote! { #wasm_bindgen::convert::WasmRet<<#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi> };
                }
                if self.function.r#async {
                    convert_ret = quote! {
                        #futures::JsFuture::from(
                            <#promise<#original_ty> as #wasm_bindgen::convert::FromWasmAbi>
                                ::from_abi(#ret_ident.join())
                        ).await
                    };
                    if self.catch {
                        convert_ret = quote! { Ok(#convert_ret?) };
                    } else {
                        convert_ret = quote! { #convert_ret.expect("uncaught exception") };
                    };
                }
            }
            None => {
                if self.function.r#async {
                    abi_ret = quote! {
                        #wasm_bindgen::convert::WasmRet<<#promise as #wasm_bindgen::convert::FromWasmAbi>::Abi>
                    };
                    let future = quote! {
                        #futures::JsFuture::from(
                            <#promise as #wasm_bindgen::convert::FromWasmAbi>
                                ::from_abi(#ret_ident.join())
                        ).await
                    };
                    convert_ret = if self.catch {
                        quote! { #future?; Ok(()) }
                    } else {
                        quote! { #future.expect("uncaught exception"); }
                    };
                } else {
                    abi_ret = quote! { () };
                    convert_ret = quote! { () };
                }
            }
        }

        let mut exceptional_ret = quote!();
        if self.catch && !self.function.r#async {
            convert_ret = quote! { Ok(#convert_ret) };
            exceptional_ret = quote! {
                #wasm_bindgen::__rt::take_last_exception()?;
            };
        }

        let rust_name = &self.rust_name;
        let import_name = &self.shim;
        let attrs = &self.function.rust_attrs;
        let arguments = &arguments;
        let abi_arguments = &abi_arguments[..];
        let abi_argument_names = &abi_argument_names[..];

        let doc = if self.doc_comment.is_empty() {
            quote! {}
        } else {
            let doc_comment = &self.doc_comment;
            quote! { #[doc = #doc_comment] }
        };

        let me = if is_method {
            quote! { &self, }
        } else {
            quote!()
        };

        // Route any errors pointing to this imported function to the identifier
        // of the function we're imported from so we at least know what function
        // is causing issues.
        //
        // Note that this is where type errors like "doesn't implement
        // FromWasmAbi" or "doesn't implement IntoWasmAbi" currently get routed.
        // I suspect that's because they show up in the signature via trait
        // projections as types of arguments, and all that needs to typecheck
        // before the body can be typechecked. Due to rust-lang/rust#60980 (and
        // probably related issues) we can't really get a precise span.
        //
        // Ideally what we want is to point errors for particular types back to
        // the specific argument/type that generated the error, but it looks
        // like rustc itself doesn't do great in that regard so let's just do
        // the best we can in the meantime.
        let extern_fn = respan(
            extern_fn(
                import_name,
                attrs,
                abi_arguments,
                abi_argument_names,
                abi_ret,
            ),
            &self.rust_name,
        );

        let maybe_unsafe = if self.function.r#unsafe {
            Some(quote! { unsafe })
        } else {
            None
        };
        let maybe_async = if self.function.r#async {
            Some(quote! { async })
        } else {
            None
        };

        let mut class_impl_def = None;
        if let Some((_, class)) = class {
            let mut class = class.clone();
            if let syn::Type::Path(syn::TypePath {
                qself: None,
                ref mut path,
            }) = class
            {
                if let Some(segment) = path.segments.last_mut() {
                    segment.arguments = syn::PathArguments::None;
                }
            }
            let has_class_generics = !fn_class_generics.class_generic_params.is_empty()
                || !fn_class_generics.class_lifetime_params.is_empty()
                || !fn_class_generics.class_bound_lifetime_params.is_empty();
            if (!is_method && !is_constructor && !is_self_returning_static) || !has_class_generics {
                // For static functions not the constructor/self-returning, we impl on generic default
                class_impl_def = Some(quote! { impl #class });
            } else {
                // Type lifetimes: appear on impl AND passed to type
                let class_lifetime_params = &fn_class_generics.class_lifetime_params;
                // Bound-only lifetimes: appear on impl but NOT passed to type
                let class_bound_lifetime_params = &fn_class_generics.class_bound_lifetime_params;
                let class_generic_params = &fn_class_generics.class_generic_params;
                let class_generic_exprs = &fn_class_generics.class_generic_exprs;
                let impl_where_clause = if !fn_class_generics.class_bounds.is_empty() {
                    let class_bounds = fn_class_generics.class_bounds.iter();
                    quote! { where #(#class_bounds),* }
                } else {
                    quote! {}
                };
                class_impl_def = Some(
                    quote! { impl<#(#class_lifetime_params,)* #(#class_bound_lifetime_params,)* #(#class_generic_params),*> #class <#(#class_lifetime_params,)* #(#class_generic_exprs),*> #impl_where_clause },
                );
            }
        };

        // Function-level lifetime params
        let fn_lifetime_params = &fn_class_generics.fn_lifetime_params;
        let has_generics =
            !fn_class_generics.fn_generic_params.is_empty() || !fn_lifetime_params.is_empty();
        let impl_generics = if !has_generics {
            quote! {}
        } else {
            let fn_generic_params = fn_class_generics.fn_generic_params;
            quote! { <#(#fn_lifetime_params,)* #(#fn_generic_params),*> }
        };
        let has_bounds = !fn_class_generics.fn_bounds.is_empty();
        let where_clause = if !has_bounds {
            quote! {}
        } else {
            let fn_bounds = fn_class_generics.fn_bounds;
            quote! { where #(#fn_bounds),* }
        };

        let invocation = quote! {
            // This is due to `#[automatically_derived]` attribute cannot be
            // placed onto bare functions.
            #[allow(nonstandard_style)]
            #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
            #(#attrs)*
            #doc
            #vis #maybe_async #maybe_unsafe fn #rust_name #impl_generics (#me #(#arguments),*) #ret #where_clause {
                #extern_fn

                unsafe {
                    let #ret_ident = {
                        #(#arg_conversions)*
                        #import_name(#(#abi_argument_names),*)
                    };
                    #exceptional_ret
                    #convert_ret
                }
            }
        };

        if let Some(class_impl_def) = class_impl_def {
            quote! {
                #[automatically_derived]
                #class_impl_def {
                    #invocation
                }
            }
            .to_tokens(tokens);
        } else {
            invocation.to_tokens(tokens);
        }

        Ok(())
    }
}

// See comment above in ast::Export for what's going on here.
struct DescribeImport<'a> {
    kind: &'a ast::ImportKind,
    wasm_bindgen: &'a syn::Path,
}

// Extracted impl block info given class generics and function-level method generics
struct FnClassGenerics<'a> {
    // the hoisted class-level param idents used, with identifiers renamed to use function generic identifier names
    class_generic_params: BTreeSet<syn::Ident>,
    // the struct generic expressions on those params
    class_generic_exprs: Vec<&'a syn::Type>,
    // class where bounds including hoisted function bounds
    class_bounds: Vec<Cow<'a, syn::WherePredicate>>,
    // the remaining non-hoisted function-level param idents
    fn_generic_params: Vec<&'a syn::Ident>,
    // function bounds on params which are only specific to the function not hoisted as class bounds
    fn_bounds: Vec<Cow<'a, syn::WherePredicate>>,
    // the union of class-level defaults (for identifier generics) and function defaults
    // this is used to form the concrete type via replacement (using JsValue otherwise)
    concrete_defaults: BTreeMap<&'a syn::Ident, Option<Cow<'a, syn::Type>>>,
    // hoisted class-level lifetime params passed to the type
    class_lifetime_params: Vec<&'a syn::Lifetime>,
    // hoisted class-level lifetime params only used in bounds (not passed to type)
    class_bound_lifetime_params: Vec<syn::Lifetime>,
    // the remaining non-hoisted function-level lifetime params
    fn_lifetime_params: Vec<&'a syn::Lifetime>,
}

impl<'a> FnClassGenerics<'a> {
    /// Adds a new function bound, checking it is not already a bound
    fn add_fn_bound(&mut self, bound: syn::WherePredicate) {
        if !self.fn_bounds.iter().any(|existing| **existing == bound) {
            self.fn_bounds.push(Cow::Owned(bound));
        }
    }
}

impl ast::ImportFunction {
    fn get_fn_generics<'a>(&'a self) -> Result<FnClassGenerics<'a>, Diagnostic> {
        let original_fn_generics = generics::generic_params(&self.generics);
        let mut fn_generic_params: Vec<&syn::Ident> =
            original_fn_generics.iter().map(|p| p.0).collect();
        let concrete_defaults: BTreeMap<_, _> = original_fn_generics
            .into_iter()
            .map(|(i, d)| (i, d.map(Cow::Borrowed)))
            .collect();

        // Extract lifetime parameters
        let all_lifetime_params = generics::lifetime_params(&self.generics);
        let mut fn_lifetime_params: Vec<&syn::Lifetime> = all_lifetime_params.clone();

        let mut where_predicates: Vec<Cow<syn::WherePredicate>> = Vec::new();
        for param in &self.generics.params {
            if let syn::GenericParam::Type(type_param) = param {
                if !type_param.bounds.is_empty() {
                    let ident = &type_param.ident;
                    let bounds = type_param.bounds.clone();
                    let predicate = syn::WherePredicate::Type(syn::PredicateType {
                        lifetimes: None,
                        bounded_ty: syn::parse_quote!(#ident),
                        colon_token: syn::Token![:](proc_macro2::Span::call_site()),
                        bounds,
                    });
                    where_predicates.push(Cow::Owned(predicate));
                }
            }
        }

        let mut class_bounds = Vec::new();
        let mut fn_bounds = generics::generic_bounds(&self.generics);
        let mut class_generic_params = BTreeSet::new();
        let mut class_lifetime_params_set = BTreeSet::new();
        let mut class_bound_lifetime_params_set: BTreeSet<syn::Lifetime> = BTreeSet::new();
        let mut class_generic_exprs = Vec::new();

        let mut class = None;
        if let ast::ImportFunctionKind::Method {
            ty,
            kind:
                ast::MethodKind::Operation(ast::Operation {
                    is_static: false, ..
                }),
            ..
        } = &self.kind
        {
            let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
                unreachable!(); // validated at parse time
            };
            class = Some(path);
        }

        // For constructors and static methods whose return type matches the class
        // (e.g. `Array::of<T>() -> Array<T>`), use the return type path for hoisting
        // since it carries the generic arguments.
        if class.is_none() {
            class = self.class_return_path();
        }

        if let Some(cls_path) = class {
            if let Some(syn::PathSegment {
                arguments: syn::PathArguments::AngleBracketed(gen_args),
                ..
            }) = cls_path.segments.last()
            {
                // Iterate the &self<expr1, expr2, ...> gen args, as the class_generic_exprs Vec
                for gen_arg in gen_args.args.iter() {
                    // Handle lifetime arguments for hoisting
                    if let syn::GenericArgument::Lifetime(lt) = gen_arg {
                        if all_lifetime_params.contains(&lt) {
                            class_lifetime_params_set.insert(lt.clone());
                        }
                        continue;
                    }

                    let syn::GenericArgument::Type(ty) = gen_arg else {
                        bail_span!(gen_arg, "Functions must provide generic arguments");
                    };

                    class_generic_exprs.push(ty);

                    // Visit the generic expression, adding all used function generics to the hoisted class generic params
                    class_generic_params =
                        generics::used_generic_params(ty, &fn_generic_params, class_generic_params);

                    // Also find lifetimes used in class generic expressions
                    let used_lifetimes = generics::used_lifetimes_in_type(ty, &all_lifetime_params);
                    class_lifetime_params_set.extend(used_lifetimes);
                }

                // Transitively hoist generic params and lifetimes that are used in bounds OF already-hoisted params.
                // For example, if F is hoisted and has bound `F: JsFunction<Ret = Ret>`, then Ret
                // must also be hoisted since it appears in a bound on F. Same for lifetimes.
                // We only hoist from bounds where the bounded_ty IS the class param (not just mentions it).
                loop {
                    let remaining_fn_params: Vec<&Ident> = fn_generic_params
                        .iter()
                        .filter(|p| !class_generic_params.contains(*p))
                        .copied()
                        .collect();

                    let remaining_fn_lifetimes: Vec<&syn::Lifetime> = fn_lifetime_params
                        .iter()
                        .filter(|lt| {
                            !class_lifetime_params_set.contains(*lt)
                                && !class_bound_lifetime_params_set.contains(*lt)
                        })
                        .copied()
                        .collect();

                    let mut params_to_add = Vec::new();
                    let mut lifetimes_to_add = BTreeSet::new();

                    for bound in &fn_bounds {
                        // Only process bounds where the bounded type IS a class param
                        // e.g., for `F: JsFunction<Ret = Ret>`, bounded_ty is `F`
                        if let syn::WherePredicate::Type(pred_type) = bound.as_ref() {
                            if let syn::Type::Path(type_path) = &pred_type.bounded_ty {
                                if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
                                    let bounded_ident = &type_path.path.segments[0].ident;
                                    if class_generic_params.contains(bounded_ident) {
                                        // This bound is ON a class param, check for fn params and lifetimes used in the bounds
                                        let mut found_set = BTreeSet::new();
                                        let mut visitor = generics::GenericNameVisitor::new(
                                            &remaining_fn_params,
                                            &mut found_set,
                                        );
                                        for type_bound in &pred_type.bounds {
                                            syn::visit::Visit::visit_type_param_bound(
                                                &mut visitor,
                                                type_bound,
                                            );
                                        }
                                        params_to_add.extend(found_set);

                                        // Also hoist lifetimes from the same bounds
                                        let used = generics::used_lifetimes_in_bounds(
                                            &pred_type.bounds,
                                            &remaining_fn_lifetimes,
                                        );
                                        lifetimes_to_add.extend(used);
                                    }
                                }
                            }
                        }
                    }

                    if params_to_add.is_empty() && lifetimes_to_add.is_empty() {
                        break;
                    }
                    for param in params_to_add {
                        class_generic_params.insert(param);
                    }
                    for lt in lifetimes_to_add {
                        class_bound_lifetime_params_set.insert(lt);
                    }
                }

                let class_generic_params_refs: Vec<&Ident> = class_generic_params.iter().collect();

                // fn generic params are all params not hoisted as class params
                fn_generic_params = fn_generic_params
                    .iter()
                    .copied()
                    .filter(|&p| !class_generic_params.contains(p))
                    .collect();

                // fn lifetime params are all lifetime params not hoisted as class lifetime params
                fn_lifetime_params.retain(|&lt| {
                    !class_lifetime_params_set.contains(lt)
                        && !class_bound_lifetime_params_set.contains(lt)
                });

                // hoist function where bounds on class generic params
                fn_bounds.retain(|bound| {
                    if generics::generics_predicate_uses(bound, &class_generic_params_refs)
                        && !generics::generics_predicate_uses(bound, &fn_generic_params)
                    {
                        class_bounds.push(bound.clone());
                        false
                    } else {
                        true
                    }
                });
            }
        }

        // Convert class_lifetime_params_set to Vec, maintaining order from original params
        let class_lifetime_params: Vec<&syn::Lifetime> = all_lifetime_params
            .iter()
            .copied()
            .filter(|lt| class_lifetime_params_set.contains(*lt))
            .collect();

        // Convert class_bound_lifetime_params_set to Vec, maintaining order from original params
        let class_bound_lifetime_params: Vec<syn::Lifetime> = all_lifetime_params
            .iter()
            .copied()
            .filter(|lt| class_bound_lifetime_params_set.contains(*lt))
            .cloned()
            .collect();

        Ok(FnClassGenerics {
            class_generic_params,
            class_generic_exprs,
            class_bounds,
            fn_generic_params,
            fn_bounds,
            concrete_defaults,
            class_lifetime_params,
            class_bound_lifetime_params,
            fn_lifetime_params,
        })
    }

    /// For constructors and static methods (via `static_method_of`), checks whether
    /// the return type matches the class name. If so, returns the path from `js_ret`
    /// which carries any generic arguments (e.g. `Array<T>`).
    ///
    /// This is used to determine when class-level generic hoisting should apply:
    ///  - Constructors always return their own class, so this always matches.
    ///  - Static methods like `#[wasm_bindgen(static_method_of = Array, js_name = of)]`
    ///    returning `Array<T>` also match, and need the same hoisting treatment.
    ///
    /// For static methods, since we are *inferring* that hoisting should happen (the
    /// user didn't explicitly opt in like with `constructor`), we only match when all
    /// type generic arguments are bare type parameter idents (e.g. `Array<T>`). Cases
    /// like `Array<I::Item>` or `Promise<U::Resolution>` are left as plain static
    /// methods — the associated type is a function-level concern, not a class property.
    fn class_return_path(&self) -> Option<&syn::Path> {
        let ast::ImportFunctionKind::Method {
            class: class_name,
            kind,
            ..
        } = &self.kind
        else {
            return None;
        };

        let is_constructor = matches!(kind, ast::MethodKind::Constructor);
        let is_static = matches!(
            kind,
            ast::MethodKind::Operation(ast::Operation {
                is_static: true,
                ..
            })
        );

        if !is_constructor && !is_static {
            return None;
        }

        let ret_ty = self.js_ret.as_ref()?;
        let syn::Type::Path(syn::TypePath {
            qself: None,
            ref path,
        }) = get_ty(ret_ty)
        else {
            return None;
        };

        let seg = path.segments.last()?;
        if seg.ident != class_name.as_str() {
            return None;
        }

        // Only hoist fn generics onto the class impl header when every fn
        // generic mentioned in the return type's args appears in a
        // *structurally constraining* position (per E0207 / RFC 0447).
        //
        // Non-constraining positions — projections (`<T as Trait>::Assoc`,
        // `T::Item`), fn-ptr slots (`fn(T)` / `Fn(T)` sugar), associated-type
        // binding RHS, etc. — would produce an `impl<T> Ret<...>` whose `T`
        // is not determinable from `Self`, yielding a borrow-check-level
        // compilation error. When we detect such a shape, bail so the
        // parameter stays function-level.
        //
        // This replaces the earlier "static methods must have only bare
        // idents" heuristic, which was both too strict (rejected valid
        // shapes like `Array<Option<T>>`) and too narrow (didn't apply to
        // constructors, leading to E0207 for `Promise<<T as Promising>::Resolution>`).
        if let syn::PathArguments::AngleBracketed(ref gen_args) = seg.arguments {
            let fn_params: Vec<&Ident> = generics::generic_params(&self.generics)
                .iter()
                .map(|p| p.0)
                .collect();
            if !generics::args_are_constraining_for(&gen_args.args, &fn_params) {
                return None;
            }
        }

        Some(path)
    }
}

impl TryToTokens for DescribeImport<'_> {
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        let f = match *self.kind {
            ast::ImportKind::Function(ref f) => f,
            ast::ImportKind::Static(_) => return Ok(()),
            ast::ImportKind::String(_) => return Ok(()),
            ast::ImportKind::Type(_) => return Ok(()),
            ast::ImportKind::Enum(_) => return Ok(()),
            ast::ImportKind::DynamicUnion(_) => return Ok(()),
        };
        let fn_class_generics = f.get_fn_generics()?;
        let fn_lifetime_params = generics::lifetime_params(&f.generics);
        let argtys = f
            .function
            .arguments
            .iter()
            .map(|arg| {
                let ty = generics::generic_to_concrete(
                    (*arg.pat_type.ty).clone(),
                    &fn_class_generics.concrete_defaults,
                    &fn_lifetime_params,
                )?;
                // For `slice_to_array` args, describe through `&Vec<T>` (or
                // `Option<&Vec<T>>`) to match the ABI rewrite in
                // `ImportFunction::try_to_tokens` — the descriptor shape is
                // `Ref(Vector(T))`, which the cli-support side recognises.
                // Non-slice args (e.g. `this: &Foo` of a method) under a
                // fn- or block-level `slice_to_array` silently fall through
                // to their default describe — slice_to_array is a mode that
                // only acts on slice-shaped args.
                if arg.slice_to_array {
                    if let Some((elem_ty, is_option)) = detect_slice_or_option_slice(&ty) {
                        if is_option {
                            return Ok(parse_quote! {
                                ::core::option::Option<&::std::vec::Vec<#elem_ty>>
                            });
                        } else {
                            return Ok(parse_quote! { &::std::vec::Vec<#elem_ty> });
                        }
                    }
                }
                Ok(ty)
            })
            .collect::<Result<Vec<syn::Type>, Diagnostic>>()?;
        let nargs = f.function.arguments.len() as u32;
        let wasm_bindgen = self.wasm_bindgen;
        // Compute the concrete return type that the section-emission
        // helper will pattern-match for schema parts.
        let ret_schema_ty = match &f.js_ret {
            Some(ref t) => {
                let t = generics::generic_to_concrete(
                    t.clone(),
                    &fn_class_generics.concrete_defaults,
                    &fn_lifetime_params,
                )?;
                quote! { #t }
            }
            // async functions always return a JsValue, even if they say to return ()
            None if f.function.r#async => quote! { #wasm_bindgen::JsValue },
            None => quote! { () },
        };

        // Section transport: 3-word header, one entry per argument
        // (no async-LONGREF transform — imports' args aren't held
        // across awaits), then the ret schema twice.
        let argty_refs: Vec<&syn::Type> = argtys.iter().collect();
        let arg_parts = build_arg_parts(self.wasm_bindgen, &argty_refs, false);
        let ret_parts = schema_parts_for_type_tokens(self.wasm_bindgen, &ret_schema_ty);
        let inner_ret_parts = ret_parts.clone();
        emit_static_descriptor_entry(
            self.wasm_bindgen,
            &f.shim.to_string(),
            &arg_parts,
            ret_parts,
            inner_ret_parts,
            nargs,
            &f.function.rust_attrs,
            tokens,
        );

        Ok(())
    }
}

impl ToTokens for ast::Enum {
    fn to_tokens(&self, into: &mut TokenStream) {
        let enum_name = &self.rust_name;
        let name_str = shared::qualified_name(self.js_namespace.as_deref(), &self.js_name);
        let name_len = name_str.len() as u32;
        let name_chars: Vec<u32> = name_str.chars().map(|c| c as u32).collect();
        let hole = &self.hole;
        let underlying = if self.signed {
            quote! { i32 }
        } else {
            quote! { u32 }
        };
        let cast_clauses = self.variants.iter().map(|variant| {
            let variant_name = &variant.rust_name;
            quote! {
                if js == #enum_name::#variant_name as #underlying {
                    #enum_name::#variant_name
                }
            }
        });
        let try_from_cast_clauses = cast_clauses.clone();
        let wasm_bindgen = &self.wasm_bindgen;
        (quote! {
            #[automatically_derived]
            impl #wasm_bindgen::convert::IntoWasmAbi for #enum_name {
                type Abi = #underlying;

                #[inline]
                fn into_abi(self) -> #underlying {
                    self as #underlying
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::FromWasmAbi for #enum_name {
                type Abi = #underlying;

                #[inline]
                unsafe fn from_abi(js: #underlying) -> Self {
                    #(#cast_clauses else)* {
                        #wasm_bindgen::throw_str("invalid enum value passed")
                    }
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionFromWasmAbi for #enum_name {
                #[inline]
                fn is_none(val: &Self::Abi) -> bool { *val == #hole as #underlying }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::OptionIntoWasmAbi for #enum_name {
                #[inline]
                fn none() -> Self::Abi { #hole as #underlying }
            }

            #[automatically_derived]
            impl #wasm_bindgen::describe::WasmDescribe for #enum_name {
                // Regular enum schema: ENUM, name_len, ...name chars,
                // hole. Lockstep with describe() below.
                const SCHEMA: &'static [u32] = &[
                    #wasm_bindgen::describe::ENUM,
                    #name_len,
                    #(#name_chars,)*
                    #hole,
                ];
                fn describe() {
                    use #wasm_bindgen::describe::*;
                    inform(ENUM);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                    inform(#hole);
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::__rt::core::convert::From<#enum_name> for
                #wasm_bindgen::JsValue
            {
                fn from(value: #enum_name) -> Self {
                    #wasm_bindgen::JsValue::from_f64((value as #underlying).into())
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::TryFromJsValue for #enum_name {
                fn try_from_js_value_ref(value: &#wasm_bindgen::JsValue) -> #wasm_bindgen::__rt::core::option::Option<Self> {
                    let js = value.as_f64()? as #underlying;

                    #wasm_bindgen::__rt::core::option::Option::Some(
                        #(#try_from_cast_clauses else)* {
                            return #wasm_bindgen::__rt::core::option::Option::None;
                        }
                    )
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::describe::WasmDescribeVector for #enum_name {
                // `Vec<StringEnum>` crosses the boundary as a JS array
                // of JsValues. Equivalent to `Vec<JsValue>` —
                // VECTOR followed by JsValue's SCHEMA (EXTERNREF).
                const VECTOR_SCHEMA: &'static [u32] = &[
                    #wasm_bindgen::describe::VECTOR,
                    #wasm_bindgen::describe::EXTERNREF,
                ];
                fn describe_vector() {
                    use #wasm_bindgen::describe::*;
                    inform(VECTOR);
                    <#wasm_bindgen::JsValue as #wasm_bindgen::describe::WasmDescribe>::describe();
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::VectorIntoWasmAbi for #enum_name {
                type Abi = <
                    #wasm_bindgen::__rt::alloc::boxed::Box<[#wasm_bindgen::JsValue]>
                    as #wasm_bindgen::convert::IntoWasmAbi
                >::Abi;

                fn vector_into_abi(
                    vector: #wasm_bindgen::__rt::alloc::boxed::Box<[#enum_name]>
                ) -> Self::Abi {
                    #wasm_bindgen::convert::js_value_vector_into_abi(vector)
                }
            }

            #[automatically_derived]
            impl #wasm_bindgen::convert::VectorFromWasmAbi for #enum_name {
                type Abi = <
                    #wasm_bindgen::__rt::alloc::boxed::Box<[#wasm_bindgen::JsValue]>
                    as #wasm_bindgen::convert::FromWasmAbi
                >::Abi;

                unsafe fn vector_from_abi(
                    js: Self::Abi
                ) -> #wasm_bindgen::__rt::alloc::boxed::Box<[#enum_name]> {
                    #wasm_bindgen::convert::js_value_vector_from_abi(js)
                }
            }

            // VectorIntoJsValue: provides per-monomorphisation
            // `From<Box<[#enum_name]>> for JsValue`. The generic
            // `impl<T: VectorIntoWasmAbi> From<Box<[T]>>` used to
            // route through wbg_cast; now each user type carries
            // its own conversion.
            #[automatically_derived]
            impl #wasm_bindgen::__rt::VectorIntoJsValue for #enum_name {
                fn vector_into_jsvalue(
                    vector: #wasm_bindgen::__rt::alloc::boxed::Box<[#enum_name]>,
                ) -> #wasm_bindgen::JsValue {
                    #wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
                }
            }
        })
        .to_tokens(into);
    }
}

impl ToTokens for ast::ImportStatic {
    fn to_tokens(&self, into: &mut TokenStream) {
        let ty = &self.ty;

        if let Some(thread_local) = self.thread_local {
            thread_local_import(
                &self.vis,
                &self.rust_name,
                &self.wasm_bindgen,
                ty,
                ty,
                &self.shim,
                thread_local,
            )
            .to_tokens(into)
        } else {
            let vis = &self.vis;
            let name = &self.rust_name;
            let wasm_bindgen = &self.wasm_bindgen;
            let ty = &self.ty;
            let shim_name = &self.shim;
            let init = static_init(wasm_bindgen, ty, shim_name);

            into.extend(quote! {
                #[automatically_derived]
                #[deprecated = "use with `#[wasm_bindgen(thread_local_v2)]` instead"]
            });
            into.extend(
                quote_spanned! { name.span() => #vis static #name: #wasm_bindgen::JsStatic<#ty> = {
                        fn init() -> #ty {
                            #init
                        }
                        #wasm_bindgen::__rt::std::thread_local!(static _VAL: #ty = init(););
                        #wasm_bindgen::JsStatic {
                            __inner: &_VAL,
                        }
                    };
                },
            );
        }

        // ImportStatic's descriptor is not function-shaped (no
        // FUNCTION header, no nargs, no ret/inner_ret duplication) —
        // just the type's own SCHEMA. Carried as a
        // `DESCRIPTOR_KIND_STATIC` section entry.
        emit_static_descriptor_entry_static(
            &self.wasm_bindgen,
            &self.shim.to_string(),
            schema_parts_for_type(&self.wasm_bindgen, &self.ty),
            into,
        );
    }
}

impl ToTokens for ast::ImportString {
    fn to_tokens(&self, into: &mut TokenStream) {
        let js_sys = &self.js_sys;
        let actual_ty: syn::Type = parse_quote!(#js_sys::JsString);

        thread_local_import(
            &self.vis,
            &self.rust_name,
            &self.wasm_bindgen,
            &actual_ty,
            &self.ty,
            &self.shim,
            self.thread_local,
        )
        .to_tokens(into);
    }
}

fn thread_local_import(
    vis: &syn::Visibility,
    name: &Ident,
    wasm_bindgen: &syn::Path,
    actual_ty: &syn::Type,
    ty: &syn::Type,
    shim_name: &Ident,
    thread_local: ast::ThreadLocal,
) -> TokenStream {
    let init = static_init(wasm_bindgen, ty, shim_name);

    match thread_local {
        ast::ThreadLocal::V1 => quote! {
            #wasm_bindgen::__rt::std::thread_local! {
                #[automatically_derived]
                #[deprecated = "use with `#[wasm_bindgen(thread_local_v2)]` instead"]
                #vis static #name: #actual_ty = {
                    #init
                };
            }
        },
        ast::ThreadLocal::V2 => {
            quote! {
                #vis static #name: #wasm_bindgen::JsThreadLocal<#actual_ty> = {
                    fn init() -> #actual_ty {
                        #init
                    }
                    #wasm_bindgen::__wbindgen_thread_local!(#wasm_bindgen, #actual_ty)
                };
            }
        }
    }
}

fn static_init(wasm_bindgen: &syn::Path, ty: &syn::Type, shim_name: &Ident) -> TokenStream {
    let abi_ret = quote! {
        #wasm_bindgen::convert::WasmRet<<#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi>
    };
    quote! {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
        extern "C" {
            fn #shim_name() -> #abi_ret;
        }

        #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
        unsafe fn #shim_name() -> #abi_ret {
            panic!("cannot access imported statics on non-wasm targets")
        }

        unsafe {
            <#ty as #wasm_bindgen::convert::FromWasmAbi>::from_abi(#shim_name().join())
        }
    }
}
fn extern_fn(
    import_name: &Ident,
    attrs: &[syn::Attribute],
    abi_arguments: &[TokenStream],
    abi_argument_names: &[Ident],
    abi_ret: TokenStream,
) -> TokenStream {
    quote! {
        #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
        #(#attrs)*
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        extern "C" {
            fn #import_name(#(#abi_arguments),*) -> #abi_ret;
        }

        #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
        unsafe fn #import_name(#(#abi_arguments),*) -> #abi_ret {
            #(
                drop(#abi_argument_names);
            )*
            panic!("cannot call wasm-bindgen imported functions on \
                    non-wasm targets");
        }
    }
}

/// Splats an argument with the given name and ABI type into 4 arguments, one
/// for each primitive that the ABI type splits into.
///
/// Returns an `(args, names)` pair, where `args` is the list of arguments to
/// be inserted into the function signature, and `names` is a list of the names
/// of those arguments.
fn splat(
    wasm_bindgen: &syn::Path,
    name: &Ident,
    abi: &TokenStream,
) -> (Vec<TokenStream>, Vec<Ident>) {
    let mut args = Vec::new();
    let mut names = Vec::new();

    for n in 1_u32..=4 {
        let arg_name = format_ident!("{}_{}", name, n);
        let prim_name = format_ident!("Prim{}", n);
        args.push(quote! {
            #arg_name: <#abi as #wasm_bindgen::convert::WasmAbi>::#prim_name
        });
        names.push(arg_name);
    }

    (args, names)
}

/// Converts `span` into a stream of tokens, and attempts to ensure that `input`
/// has all the appropriate span information so errors in it point to `span`.
fn respan(input: TokenStream, span: &dyn ToTokens) -> TokenStream {
    let mut first_span = Span::call_site();
    let mut last_span = Span::call_site();
    let mut spans = TokenStream::new();
    span.to_tokens(&mut spans);

    for (i, token) in spans.into_iter().enumerate() {
        if i == 0 {
            first_span = Span::call_site().located_at(token.span());
        }
        last_span = Span::call_site().located_at(token.span());
    }

    let mut new_tokens = Vec::new();
    for (i, mut token) in input.into_iter().enumerate() {
        if i == 0 {
            token.set_span(first_span);
        } else {
            token.set_span(last_span);
        }
        new_tokens.push(token);
    }
    new_tokens.into_iter().collect()
}

fn get_ty(mut ty: &syn::Type) -> &syn::Type {
    while let syn::Type::Group(g) = ty {
        ty = &g.elem;
    }
    ty
}

/// Detects whether a type is a raw `&dyn Fn(...)` or `&mut dyn FnMut(...)` argument.
///
/// Returns `Some((is_mut, fn_trait_bounds))` where:
/// - `is_mut` is `true` for `&mut dyn FnMut`, `false` for `&dyn Fn`
/// - `fn_trait_bounds` are the `TypeParamBound`s from the `dyn` trait object (e.g. `FnMut(A)->R`)
///
/// This is used by the import function codegen to auto-inject `MaybeUnwindSafe`
/// bounds for closure arguments, ensuring unwind safety when `panic = "unwind"`.
/// Recognise `&[T]` and `Option<&[T]>` argument types. Returns the element
/// type plus a flag indicating whether the outer `Option` was present. Used
/// by the `slice_to_array` codegen to rewrite the ABI path.
fn detect_slice_or_option_slice(ty: &syn::Type) -> Option<(syn::Type, bool)> {
    // Direct `&[T]` (mutability ignored — `&mut [T]` is intentionally
    // accepted too; the ABI layer treats it the same as `&[T]`).
    if let syn::Type::Reference(syn::TypeReference { elem, .. }) = ty {
        if let syn::Type::Slice(syn::TypeSlice { elem: inner, .. }) = &**elem {
            return Some(((**inner).clone(), false));
        }
    }
    // `Option<&[T]>` — match shape `Option<...>` and recurse once.
    if let syn::Type::Path(syn::TypePath { qself: None, path }) = ty {
        if let Some(seg) = path.segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if args.args.len() == 1 {
                        if let syn::GenericArgument::Type(inner) = &args.args[0] {
                            if let Some((elem, false)) = detect_slice_or_option_slice(inner) {
                                return Some((elem, true));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn detect_raw_fn_trait_obj(
    ty: &syn::Type,
) -> Option<(
    bool,
    &syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>,
)> {
    let syn::Type::Reference(syn::TypeReference {
        mutability, elem, ..
    }) = ty
    else {
        return None;
    };
    let inner = get_ty(elem);
    let syn::Type::TraitObject(trait_obj) = inner else {
        return None;
    };
    let is_mut = mutability.is_some();
    // Check that the primary bound is Fn or FnMut (matching mutability)
    for bound in &trait_obj.bounds {
        if let syn::TypeParamBound::Trait(tb) = bound {
            if let Some(last_seg) = tb.path.segments.last() {
                let name = last_seg.ident.to_string();
                if is_mut && name == "FnMut" {
                    return Some((true, &trait_obj.bounds));
                }
                if !is_mut && name == "Fn" {
                    return Some((false, &trait_obj.bounds));
                }
            }
        }
    }
    None
}
