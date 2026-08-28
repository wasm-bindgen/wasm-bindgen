use crate::ast;
use crate::encode;
use crate::encode::EncodeChunk;
use crate::generics::{self, generic_to_concrete};
use crate::Diagnostic;
use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;
use quote::quote_spanned;
use quote::{quote, ToTokens};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use syn::ext::IdentExt;
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::visit_mut::{self, VisitMut};
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
        let mut types: HashMap<String, Vec<&ast::ImportType>> = HashMap::new();
        for i in self.imports.iter() {
            if let ast::ImportKind::Type(t) = &i.kind {
                types
                    .entry(t.rust_name.unraw().to_string())
                    .or_default()
                    .push(t);
            }
        }
        for i in self.imports.iter() {
            let class_contexts = match &i.kind {
                ast::ImportKind::Function(function) => imported_class_generics(function, &types),
                _ => Vec::new(),
            };
            DescribeImport {
                kind: &i.kind,
                wasm_bindgen: &self.wasm_bindgen,
                class_cfg_attrs: cfg_union_attrs(&class_contexts),
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
                        let function_cfg_attrs = match &i.kind {
                            ast::ImportKind::Function(function) => {
                                crate::cfg_gate_attrs(&function.function.rust_attrs)
                            }
                            _ => Vec::new(),
                        };
                        for candidate in ns {
                            let candidate_cfg_attrs = crate::cfg_gate_attrs(&candidate.attrs);
                            let rust_name = &candidate.rust_name;
                            (quote! {
                                #(#function_cfg_attrs)*
                                #(#candidate_cfg_attrs)*
                                #[automatically_derived]
                                impl #rust_name { #kind }
                            })
                            .to_tokens(tokens);
                        }
                        continue;
                    }
                }
            }

            let result = match &i.kind {
                ast::ImportKind::Function(function) => {
                    let mut result = Ok(());
                    for (class_generics, class_cfg_attrs) in class_contexts {
                        if let Err(error) = function.try_to_tokens_with_class_generics(
                            tokens,
                            class_generics,
                            &class_cfg_attrs,
                        ) {
                            let function_cfg_attrs =
                                crate::cfg_gate_attrs(&function.function.rust_attrs);
                            if class_cfg_attrs.is_empty() && function_cfg_attrs.is_empty() {
                                result = Err(error);
                                break;
                            }
                            quote! {
                                #(#function_cfg_attrs)*
                                #(#class_cfg_attrs)*
                                #error
                            }
                            .to_tokens(tokens);
                        }
                    }
                    result
                }
                _ => i.kind.try_to_tokens(tokens),
            };
            if let Err(e) = result {
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

fn imported_class_generics<'a>(
    function: &ast::ImportFunction,
    types: &HashMap<String, Vec<&'a ast::ImportType>>,
) -> Vec<(Option<&'a syn::Generics>, Vec<syn::Attribute>)> {
    let ast::ImportFunctionKind::Method {
        ty: class_ty, kind, ..
    } = &function.kind
    else {
        return vec![(None, Vec::new())];
    };
    let syn::Type::Path(syn::TypePath {
        attrs: _,
        qself: None,
        path,
    }) = get_ty(class_ty)
    else {
        return vec![(None, Vec::new())];
    };
    // Candidates are matched by the path's final identifier, so a qualified
    // path is only trusted when it can plausibly name this invocation's own
    // module. A constructor is exempt: its class comes from its return type,
    // which must name the imported type for the JS binding to attach at all.
    let is_constructor = matches!(kind, ast::MethodKind::Constructor);
    let is_local_path = path.leading_colon.is_none()
        && (path.segments.len() == 1
            || path
                .segments
                .first()
                .is_some_and(|segment| segment.ident == "self" || segment.ident == "crate"));
    if !is_constructor && !is_local_path {
        return vec![(None, Vec::new())];
    }
    let Some(local_name) = path.segments.last().map(|segment| &segment.ident) else {
        return vec![(None, Vec::new())];
    };
    let Some(candidates) = types.get(&local_name.unraw().to_string()) else {
        return vec![(None, Vec::new())];
    };
    candidates
        .iter()
        .map(|candidate| {
            (
                Some(&candidate.generics),
                crate::cfg_gate_attrs(&candidate.attrs),
            )
        })
        .collect()
}

fn cfg_union_attrs(
    contexts: &[(Option<&syn::Generics>, Vec<syn::Attribute>)],
) -> Vec<syn::Attribute> {
    let mut alternatives = Vec::new();
    for (_, attrs) in contexts {
        if attrs.is_empty() {
            return Vec::new();
        }
        let conditions = attrs.iter().filter_map(|attr| match &attr.meta {
            syn::Meta::List(list) if list.path.is_ident("cfg") => Some(&list.tokens),
            _ => None,
        });
        alternatives.push(quote! { all(#(#conditions),*) });
    }
    if alternatives.is_empty() {
        Vec::new()
    } else {
        vec![syn::parse_quote! { #[cfg(any(#(#alternatives),*))] }]
    }
}

fn same_class_path(left: &syn::Path, right: &syn::Path) -> bool {
    if left.leading_colon.is_some() != right.leading_colon.is_some() {
        return false;
    }
    let left = left
        .segments
        .iter()
        .skip(usize::from(
            left.leading_colon.is_none()
                && left.segments.len() > 1
                && left
                    .segments
                    .first()
                    .is_some_and(|segment| segment.ident == "self"),
        ))
        .collect::<Vec<_>>();
    let right = right
        .segments
        .iter()
        .skip(usize::from(
            right.leading_colon.is_none()
                && right.segments.len() > 1
                && right
                    .segments
                    .first()
                    .is_some_and(|segment| segment.ident == "self"),
        ))
        .collect::<Vec<_>>();
    left.len() == right.len()
        && left
            .iter()
            .zip(&right)
            .enumerate()
            .all(|(index, (left_segment, right_segment))| {
                left_segment.ident == right_segment.ident
                    && (index + 1 == left.len()
                        || left_segment.arguments == right_segment.arguments)
            })
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
        let name_len = name_str.chars().count() as u32;
        let name_chars: Vec<u32> = name_str.chars().map(|c| c as u32).collect();
        let unique_crate_identifier = crate::hash::unique_crate_identifier();
        let unique_crate_identifier_len = unique_crate_identifier.chars().count() as u32;
        let unique_crate_identifier_chars = unique_crate_identifier.chars().map(|c| c as u32);
        // The Rust idents stay canonical so rustc diagnostics read cleanly;
        // only the wasm symbols carry the per-crate hash.
        let new_fn = Ident::new(&shared::new_function(&name_str), Span::call_site());
        let new_fn_symbol = crate::hash::crate_mangled_symbol(&shared::new_function(&name_str));
        let free_fn = Ident::new(&shared::free_function(&name_str), Span::call_site());
        let free_fn_symbol = crate::hash::crate_mangled_symbol(&shared::free_function(&name_str));
        let unwrap_fn = Ident::new(&shared::unwrap_function(&name_str), Span::call_site());
        let unwrap_fn_symbol =
            crate::hash::crate_mangled_symbol(&shared::unwrap_function(&name_str));
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
                fn describe() {
                    use #wasm_bindgen::describe::*;
                    inform(RUST_STRUCT);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                    inform(#unique_crate_identifier_len);
                    #(inform(#unique_crate_identifier_chars);)*
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
                        #[link_name = #new_fn_symbol]
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
                #[export_name = #free_fn_symbol]
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
                        #[link_name = #unwrap_fn_symbol]
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
                let upcast_fn_symbol = crate::hash::crate_mangled_symbol(&shared::upcast_function(
                    &name_str,
                    &parent_qualified,
                ));
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
                        #[export_name = #upcast_fn_symbol]
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
        // The Rust idents stay canonical so rustc diagnostics read cleanly;
        // only the wasm symbols carry the per-crate hash.
        let getter = &self.getter;
        let setter = &self.setter;
        let getter_symbol = crate::hash::crate_mangled_symbol(&getter.to_string());
        let setter_symbol = crate::hash::crate_mangled_symbol(&setter.to_string());

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
                #[cfg_attr(all(target_family = "wasm", not(target_os = "wasi")), export_name = #getter_symbol)]
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

        // The describe function is named after the wasm symbol, not the
        // Rust ident, since cli-support matches descriptors to shims by
        // stripping the `__wbindgen_describe_` prefix.
        let getter_descriptor = Ident::new(&getter_symbol, Span::call_site());
        Descriptor {
            ident: &getter_descriptor,
            inner: quote! {
                <#ty as WasmDescribe>::describe();
            },
            attrs: vec![],
            wasm_bindgen: &self.wasm_bindgen,
        }
        .to_tokens(tokens);

        if self.readonly {
            return;
        }

        let abi = quote! { <#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi };
        let (args, names) = splat(
            wasm_bindgen,
            &Ident::new("val", rust_name.span()),
            &abi,
            Span::call_site(),
        );

        (quote! {
            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            #[automatically_derived]
            const _: () = {
                #wasm_bindgen::__wbindgen_coverage! {
                #[export_name = #setter_symbol]
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
        if self.function.jspi {
            (quote_spanned! {
                self.function.name_span =>
                const _: () = {
                    #[deprecated(note = "JSPI support is experimental and subject to change; \
                        `#[wasm_bindgen(jspi)]` requires a runtime with WebAssembly \
                        JS Promise Integration enabled")]
                    const fn jspi_is_experimental() {}
                    jspi_is_experimental();
                };
            })
            .to_tokens(into);
        }

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
                    let (prim_args, prim_names) =
                        splat(wasm_bindgen, &ident, &abi, Span::call_site());
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
                        let (prim_args, prim_names) =
                            splat(wasm_bindgen, &ident, &abi, Span::call_site());
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
                                ::Anchor as #wasm_bindgen::__rt::core::borrow::Borrow<#elem>>
                                ::borrow(&#ident);
                        });
                    } else {
                        let abi = quote! { <#elem as #wasm_bindgen::convert::RefFromWasmAbi>::Abi };
                        let (prim_args, prim_names) =
                            splat(wasm_bindgen, &ident, &abi, Span::call_site());
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
                    let (prim_args, prim_names) =
                        splat(wasm_bindgen, &ident, &abi, Span::call_site());
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
            attrs: Vec::new(),
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
        let describe_ret = quote! {
            <#ret_ty as WasmDescribe>::describe();
            <#inner_ret_ty as WasmDescribe>::describe();
        };
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

        let describe_args: TokenStream = argtys
            .iter()
            .map(|ty| match ty {
                syn::Type::Reference(reference)
                    if self.function.r#async && reference.mutability.is_none() =>
                {
                    let inner = &reference.elem;
                    quote! {
                        inform(LONGREF);
                        <#inner as WasmDescribe>::describe();
                    }
                }
                _ => quote! { <#ty as WasmDescribe>::describe(); },
            })
            .collect();

        // In addition to generating the shim function above which is what
        // our generated JS will invoke, we *also* generate a "descriptor"
        // shim. This descriptor shim uses the `WasmDescribe` trait to
        // programmatically describe the type signature of the generated
        // shim above. This in turn is then used to inform the
        // `wasm-bindgen` CLI tool exactly what types and such it should be
        // using in JS.
        //
        // Note that this descriptor function is a purely an internal detail
        // of `#[wasm_bindgen]` and isn't intended to be exported to anyone
        // or actually part of the final was binary. Additionally, this is
        // literally executed when the `wasm-bindgen` tool executes.
        //
        // In any case, there's complications in `wasm-bindgen` to handle
        // this, but the tl;dr; is that this is stripped from the final wasm
        // binary along with anything it references.
        let export = Ident::new(&export_name, Span::call_site());
        Descriptor {
            ident: &export,
            inner: quote! {
                inform(FUNCTION);
                inform(0);
                inform(#nargs);
                #describe_args
                #describe_ret
            },
            attrs,
            wasm_bindgen: &self.wasm_bindgen,
        }
        .to_tokens(into);

        Ok(())
    }
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
        let cfg_attrs = crate::cfg_gate_attrs(attrs);
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

        let description = if let Some(typescript_type) = &self.typescript_type {
            // One descriptor word per `char`, so count `char`s, not bytes.
            let typescript_type_len = typescript_type.chars().count() as u32;
            let typescript_type_chars = typescript_type.chars().map(|c| c as u32);
            quote! {
                use #wasm_bindgen::describe::*;
                inform(NAMED_EXTERNREF);
                inform(#typescript_type_len);
                #(inform(#typescript_type_chars);)*
            }
        } else {
            quote! {
                JsValue::describe()
            }
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

        let mut declaration_generics = self.generics.clone();
        generics::move_lifetime_bounds_to_where(&mut declaration_generics);
        let class_generic_params = generics::generic_params(&declaration_generics);
        let (impl_generics, ty_generics, where_clause) = declaration_generics.split_for_impl();

        let lifetime_args = generics::lifetime_args(&declaration_generics);
        // The reference impls below (`&'__wbg_ref #rust_name #ty_generics`)
        // need to declare the type's *own* lifetime params on the impl
        // header, not just a fresh reference lifetime and the type params:
        // `ty_generics` re-emits every one of the type's lifetime arguments,
        // so leaving them undeclared here is an undeclared-lifetime error
        // (E0261) unless the type happens to have no lifetimes of its own.
        //
        // Keep the normalized structured declarations and only prepend a fresh
        // borrow lifetime. Reassembling this header from lifetime names loses
        // bounds such as `'a: 'b`.
        let reference_lifetime = generics::fresh_lifetime(&declaration_generics, "__wbg_ref");
        let mut reference_generics = declaration_generics.clone();
        reference_generics.params.insert(
            0,
            syn::GenericParam::Lifetime(syn::LifetimeParam::new(reference_lifetime.clone())),
        );
        let (reference_impl_generics, _, _) = reference_generics.split_for_impl();

        // For struct definitions, we need generics with defaults, so use params directly
        let struct_generics = if declaration_generics.params.is_empty() {
            quote! {}
        } else {
            let params = &declaration_generics.params;
            quote! { <#params> }
        };

        let phantom;
        let phantom_init;

        // For `From<JsValue>`, only include lifetime params so type params
        // fall back to their defaults and callers don't need turbofish.
        let lifetime_declarations = generics::lifetime_params_with_bounds(&declaration_generics);
        let from_jsvalue_generics = if lifetime_declarations.is_empty() {
            quote! {}
        } else {
            quote! { <#(#lifetime_declarations),*> }
        };
        let from_jsvalue_lifetime_predicates: Vec<_> = declaration_generics
            .where_clause
            .iter()
            .flat_map(|clause| clause.predicates.iter())
            .filter(|predicate| matches!(predicate, syn::WherePredicate::Lifetime(_)))
            .collect();
        let from_jsvalue_where_clause = if from_jsvalue_lifetime_predicates.is_empty() {
            quote! {}
        } else {
            quote! { where #(#from_jsvalue_lifetime_predicates),* }
        };

        if !class_generic_params.is_empty() || !lifetime_args.is_empty() {
            let generic_param_names: Vec<_> = class_generic_params.iter().map(|p| p.0).collect();
            let lifetime_refs = lifetime_args.iter().map(|lt| quote! { &#lt () });
            // Via `#wasm_bindgen::__rt::core`, not a bare `::core`: this is
            // expanded with call-site hygiene into the user's own module, where a
            // user item named `core` shadows the extern-prelude entry, and a
            // 2015-edition consumer would resolve `core::` relative to its crate
            // root and fail outright.
            phantom = quote! {
                generics: #wasm_bindgen::__rt::core::marker::PhantomData<(#(#generic_param_names,)* #(#lifetime_refs),*)>
            };
            phantom_init = quote! { generics: #wasm_bindgen::__rt::core::marker::PhantomData };
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
            let mut clause = declaration_generics
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
                        unsafe {
                            #wasm_bindgen::__rt::core::mem::transmute_copy(
                                &#wasm_bindgen::__rt::core::mem::ManuallyDrop::new(self),
                            )
                        }
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

            #(#cfg_attrs)*
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
                    fn describe() {
                        #description
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
                impl #reference_impl_generics OptionIntoWasmAbi for &#reference_lifetime #rust_name #ty_generics #where_clause {
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
                impl #reference_impl_generics IntoWasmAbi for &#reference_lifetime #rust_name #ty_generics #where_clause {
                    type Abi = <&#reference_lifetime JsValue as IntoWasmAbi>::Abi;

                    #[inline]
                    fn into_abi(self) -> Self::Abi {
                        (&self.obj).into_abi()
                    }
                }

                #[automatically_derived]
                impl #impl_generics RefFromWasmAbi for #rust_name #ty_generics #where_clause {
                    type Abi = <JsValue as RefFromWasmAbi>::Abi;
                    type Anchor = #wasm_bindgen::__rt::core::mem::ManuallyDrop<#rust_name #ty_generics>;

                    #[inline]
                    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                        let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                        #wasm_bindgen::__rt::core::mem::ManuallyDrop::new(#rust_name {
                            obj: #wasm_bindgen::__rt::core::mem::ManuallyDrop::into_inner(tmp).into(),
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
                impl #from_jsvalue_generics From<JsValue> for #rust_name #from_jsvalue_generics #from_jsvalue_where_clause {
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
                #(#cfg_attrs)*
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::sys::Promising for #rust_name #ty_generics #where_clause {
                    type Resolution = #rust_name #ty_generics;
                }
            })
            .to_tokens(tokens);
        }

        if !no_deref {
            (quote! {
                #(#cfg_attrs)*
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
                #(#cfg_attrs)*
                #[automatically_derived]
                impl #impl_generics From<#rust_name #ty_generics> for #superclass #where_clause {
                    #[inline]
                    fn from(obj: #rust_name #ty_generics) -> #superclass {
                        use #wasm_bindgen::JsCast;
                        #superclass::unchecked_from_js(obj.into())
                    }
                }

                #(#cfg_attrs)*
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
            // 1. Always generate UpcastFrom<Self> for JsValue, including its
            // JsOption/JsNullable wrappers (like superclass targets below)
            (quote! {
                #(#cfg_attrs)*
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                    for #wasm_bindgen::JsValue
                #where_clause
                {
                }
                #(#cfg_attrs)*
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                    for #wasm_bindgen::sys::JsOption<#wasm_bindgen::JsValue>
                #where_clause
                {
                }
                #(#cfg_attrs)*
                #[automatically_derived]
                impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                    for #wasm_bindgen::sys::JsNullable<#wasm_bindgen::JsValue>
                #where_clause
                {
                }
            })
            .to_tokens(tokens);

            // 2. For non-generic types: generate identity upcast (UpcastFrom<Self> for Self, UpcastFrom<Self> for JsOption<Self>/JsNullable<Self>)
            // 3. For generic types: generate structural covariance
            let type_params: Vec<_> = declaration_generics.type_params().collect();
            if type_params.is_empty() {
                // Identity impls for non-generic (or lifetime-only) types.
                // Always use #ty_generics so that lifetime params are included.
                (quote! {
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #rust_name #ty_generics
                    #where_clause
                    {
                    }
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsOption<#rust_name #ty_generics>
                    #where_clause
                    {
                    }
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsNullable<#rust_name #ty_generics>
                    #where_clause
                    {
                    }
                })
                .to_tokens(tokens);
            } else {
                // Structural covariance impl for generic types
                // Build impl generics: all original params plus a Target param for each
                let mut impl_generics_extended = declaration_generics.clone();
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
                let mut where_clause_extended = declaration_generics
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
                let target_lifetime_params = generics::lifetime_args(&declaration_generics);
                let target_ty_generics =
                    quote! { <#(#target_lifetime_params,)* #(#target_param_names),*> };

                // Structural covariance - Type<Target0, Target1, ...> can be upcast from Type<T1, T2, ...>
                (quote! {
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics_split #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #rust_name #target_ty_generics
                    #where_clause_extended
                    {
                    }
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics_split #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsOption<#rust_name #target_ty_generics>
                    #where_clause_extended
                    {
                    }
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics_split #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsNullable<#rust_name #target_ty_generics>
                    #where_clause_extended
                    {
                    }
                })
                .to_tokens(tokens);
            }

            // 4. For each superclass in extends, generate UpcastFrom<Self> for superclass
            for superclass in self.extends.iter() {
                (quote! {
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #superclass
                    #where_clause
                    {
                    }
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsOption<#superclass>
                    #where_clause
                    {
                    }
                    #(#cfg_attrs)*
                    #[automatically_derived]
                    impl #impl_generics #wasm_bindgen::convert::UpcastFrom<#rust_name #ty_generics>
                        for #wasm_bindgen::sys::JsNullable<#superclass>
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
        let name_len = name_str.chars().count() as u32;
        let name_chars = name_str.chars().map(u32::from);
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
        let name_len = name_str.chars().count() as u32;
        let name_chars = name_str.chars().map(u32::from);

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

        // Generate descriptor exports for each type variant so cli-support can look them up
        for (idx, ty) in type_variants.iter().enumerate() {
            let descriptor_name = Ident::new(
                &crate::hash::crate_mangled_symbol(&shared::dynamic_union_variant(
                    name_str, idx as u32,
                )),
                Span::call_site(),
            );
            Descriptor {
                ident: &descriptor_name,
                inner: quote! {
                    <#ty as WasmDescribe>::describe();
                },
                attrs: vec![],
                wasm_bindgen: &self.wasm_bindgen,
            }
            .to_tokens(tokens);
        }
    }
}

impl TryToTokens for ast::ImportFunction {
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        self.try_to_tokens_with_class_generics(tokens, None, &[])
    }
}

impl ast::ImportFunction {
    fn try_to_tokens_with_class_generics(
        &self,
        tokens: &mut TokenStream,
        class_generics: Option<&syn::Generics>,
        class_cfg_attrs: &[syn::Attribute],
    ) -> Result<(), Diagnostic> {
        if self.suspending {
            (quote_spanned! {
                self.function.name_span =>
                const _: () = {
                    #[deprecated(note = "JSPI support is experimental and subject to change; \
                        `#[wasm_bindgen(suspending)]` requires a runtime with WebAssembly \
                        JS Promise Integration enabled")]
                    const fn suspending_is_experimental() {}
                    suspending_is_experimental();
                };
            })
            .to_tokens(tokens);
        }

        if self.generic_per_mono {
            return self.try_to_tokens_generic(tokens, class_generics, class_cfg_attrs);
        }
        let mut class = None;
        let mut is_constructor = false;
        let mut is_method = false;
        let mut is_self_returning_static = false;
        let mut explicit_static_class_generics = false;
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
            explicit_static_class_generics = !is_method
                && !is_constructor
                && class_path_arguments(get_ty(ty)).is_some_and(|arguments| !arguments.is_empty());
            // For constructors and static methods whose return type matches the
            // class (e.g. `Array::of<T>() -> Array<T>`), override the class type
            // to use the return type so class-level generics get hoisted.
            if self.class_return_path().is_some()
                && (is_constructor || !explicit_static_class_generics)
            {
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
        let hoist = is_method
            || is_constructor
            || is_self_returning_static
            || explicit_static_class_generics;
        if hoist {
            if let Some((_, class)) = class {
                self.validate_class_shape(class, is_method || explicit_static_class_generics)?;
            }
        }
        if let (Some((_, class)), Some(class_generics)) = (class, class_generics) {
            fn_class_generics.add_class_bounds(class_generics, class, &[])?;
        }
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

                convert_arg = quote! { unsafe { #wasm_bindgen::__rt::core::mem::transmute_copy(&#wasm_bindgen::__rt::core::mem::ManuallyDrop::new(#var)) } };
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

            // `slice_to_array`: hand JS an owned `Array` instead of a
            // typed-array view. See `slice_to_array_rewrite`.
            if arg.slice_to_array {
                if let Some(rewrite) = slice_to_array_rewrite(wasm_bindgen, &name, &var, ty) {
                    abi_arguments.extend(rewrite.abi_args);
                    abi_argument_names.extend(rewrite.prim_names);
                    arg_conversions.push(rewrite.conversion);
                    continue;
                }
            }

            // Span the `IntoWasmAbi` projection at the argument's own type. This
            // is where a "does not implement `IntoWasmAbi`" error is actually
            // raised (the projection has to resolve before anything else in the
            // signature typechecks), and with a `quote!` call-site span every
            // such error in an `extern "C"` block lands on the block's
            // `#[wasm_bindgen]` attribute instead — N identical errors with no
            // indication of which argument is at fault. `&T` arguments make this
            // routine rather than exotic: `&T: IntoWasmAbi` exists only for
            // `&JsValue`, JS handle types and the concrete slice/string
            // references, so any `&SomeStruct` argument fails here.
            let abi_span = arg.pat_type.ty.span();
            let abi = quote_spanned! { abi_span =>
                <#abi_ty as #wasm_bindgen::convert::IntoWasmAbi>::Abi
            };
            let (prim_args, prim_names) = splat(wasm_bindgen, &name, &abi, abi_span);
            abi_arguments.extend(prim_args);
            abi_argument_names.extend(prim_names.iter().cloned());

            arg_conversions.push(quote_spanned! { abi_span =>
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
                } else if self.suspending {
                    // A suspending import's settled value arrives as the raw
                    // externref return of the call (the JS shim hands the
                    // Promise straight to `WebAssembly.Suspending`, so no
                    // shim-side return conversion can ever see the settled
                    // value). The ABI is therefore always a `JsValue`, and
                    // the declared return type is converted in Rust after
                    // the fiber resumes.
                    maybe_async_wrapped = parse_quote!(#wasm_bindgen::JsValue);
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
                    convert_ret = quote! { unsafe { #wasm_bindgen::__rt::core::mem::transmute_copy(&#wasm_bindgen::__rt::core::mem::ManuallyDrop::new(<#concrete_ty as #wasm_bindgen::convert::FromWasmAbi>::from_abi(#ret_ident.join()))) } };
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
                } else if self.suspending {
                    // Convert the settled value to the declared return type
                    // post-resume via a cast adapter (`wbg_cast`), which
                    // routes the conversion through a CLI-generated JS shim
                    // with the standard descriptor-driven ABI semantics —
                    // the same conversion any other import return would get,
                    // just executed after the fiber resumes instead of in
                    // the (pre-settlement) import shim. With `catch`, a
                    // rejection is reported by the in-wasm suspend wrapper
                    // through the `__wbindgen_jspi_rejected` flag, with the
                    // rejection reason as the returned value.
                    let convert_ok = quote! {
                        #wasm_bindgen::__rt::wbg_cast::<#wasm_bindgen::JsValue, #original_ty>(__wbg_settled)
                    };
                    let body = if self.catch {
                        quote! {
                            if #wasm_bindgen::__rt::jspi_rejected() {
                                Err(__wbg_settled)
                            } else {
                                Ok(#convert_ok)
                            }
                        }
                    } else {
                        convert_ok
                    };
                    convert_ret = quote! {
                        {
                            let __wbg_settled =
                                <#wasm_bindgen::JsValue as #wasm_bindgen::convert::FromWasmAbi>
                                    ::from_abi(#ret_ident.join());
                            #body
                        }
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
                } else if self.suspending && self.catch {
                    // Even with no declared return value the error channel
                    // needs the settled value: the rejection reason arrives
                    // as the import's externref return.
                    abi_ret = quote! {
                        #wasm_bindgen::convert::WasmRet<<#wasm_bindgen::JsValue as #wasm_bindgen::convert::FromWasmAbi>::Abi>
                    };
                    convert_ret = quote! {
                        {
                            let __wbg_settled =
                                <#wasm_bindgen::JsValue as #wasm_bindgen::convert::FromWasmAbi>
                                    ::from_abi(#ret_ident.join());
                            if #wasm_bindgen::__rt::jspi_rejected() {
                                Err(__wbg_settled)
                            } else {
                                Ok(())
                            }
                        }
                    };
                } else {
                    abi_ret = quote! { () };
                    convert_ret = quote! { () };
                }
            }
        }

        let mut exceptional_ret = quote!();
        if self.catch && !self.function.r#async && !self.suspending {
            convert_ret = quote! { Ok(#convert_ret) };
            exceptional_ret = quote! {
                #wasm_bindgen::__rt::take_last_exception()?;
            };
        }

        let rust_name = &self.rust_name;
        // The shim's own identifier is synthesised (`__wbg_<name>_<hash>`) and so
        // is created at `Span::call_site()`. Both the declaration of the shim and
        // the call to it normalise its whole signature, and rustc attributes that
        // to the identifier — which means every "argument type doesn't implement
        // `IntoWasmAbi`" failure is reported on the `#[wasm_bindgen]` attribute of
        // the enclosing `extern "C"` block. Re-span it onto the imported
        // function's own name so at least the failing *import* is identified.
        let fn_span = Span::call_site().located_at(self.rust_name.span());
        let import_name = &Ident::new(&self.shim.to_string(), fn_span);
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
        // `respan` only rewrites the top-level tokens of the stream, so
        // everything nested inside the generated function's signature and body
        // keeps `Span::call_site()` — which is the `#[wasm_bindgen]` attribute of
        // the enclosing `extern "C"` block. Rewrite the whole tree instead, so a
        // shim whose signature does not typecheck at least names the import it
        // belongs to. Per-argument precision comes from `arg_conversions` in the
        // wrapper body, which is spanned at each argument's own type.
        let extern_fn = respan_all(
            extern_fn(
                import_name,
                attrs,
                abi_arguments,
                abi_argument_names,
                abi_ret,
            ),
            fn_span,
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
                attrs: _,
                qself: None,
                ref mut path,
            }) = class
            {
                if let Some(segment) = path.segments.last_mut() {
                    segment.arguments = syn::PathArguments::None;
                }
            }
            // Explicit `static_method_of = Class<T>` arguments select the impl
            // just as a receiver or self-returning static does. Only a bare
            // static class path binds the imported type's own defaults.
            class_impl_def = Some(fn_class_generics.class_impl_def(&class, hoist));
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

        // Calling the shim normalises its whole signature at one span, so a bad
        // argument's ABI projection is reported here too — with no way to say
        // *which* argument, since a single call covers all of them. Point it at
        // the function's own name, matching `respan(extern_fn, ..)` above, so at
        // least it names the import rather than the `#[wasm_bindgen]` attribute
        // on the enclosing `extern "C"` block.
        let shim_call = quote_spanned! { fn_span =>
            #import_name(#(#abi_argument_names),*)
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
                        #shim_call
                    };
                    #exceptional_ret
                    #convert_ret
                }
            }
        };

        if let Some(class_impl_def) = class_impl_def {
            let function_cfg_attrs = crate::cfg_gate_attrs(attrs);
            quote! {
                #(#function_cfg_attrs)*
                #(#class_cfg_attrs)*
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

/// Whether `ty` marshals to a JS value that the `variadic` spread (`...arg`)
/// can iterate.
///
/// This is an allow-list rather than a deny-list, and deliberately so. The
/// property being checked is "crosses the ABI as a JS array", which only the
/// sequence shapes have; everything else — a bare `T`, `Option<T>`, `Box<T>`,
/// an associated type `T::Item`, a tuple — either cannot spread at all or can
/// only spread for *some* monomorphisations, which is worse, because the import
/// then compiles and throws a `TypeError` at runtime for the instantiations that
/// happen not to be iterable.
///
/// Top-level references and grouping layers are stripped first: `&[T]` crosses
/// as whatever `[T]` does, and grouping is transparent to a type's meaning.
fn is_spreadable_sequence(ty: &syn::Type) -> bool {
    let mut ty = ty;
    loop {
        ty = match ty {
            syn::Type::Reference(r) => &r.elem,
            syn::Type::Paren(p) => &p.elem,
            syn::Type::Group(g) => &g.elem,
            _ => break,
        };
    }
    match ty {
        // `[T]` and `[T; N]`.
        syn::Type::Slice(_) | syn::Type::Array(_) => true,
        syn::Type::Path(syn::TypePath {
            attrs: _,
            qself: None,
            path,
        }) => {
            let Some(seg) = path.segments.last() else {
                return false;
            };
            if seg.ident == "Vec" {
                return true;
            }
            // `Box<[T]>` marshals as a slice, but `Box<T>` does not.
            if seg.ident == "Box" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return matches!(get_ty(inner), syn::Type::Slice(_));
                    }
                }
            }
            false
        }
        _ => false,
    }
}

impl ast::ImportFunction {
    /// Experimental per-monomorphisation codegen for a generic import.
    ///
    /// Instead of erasing type parameters to `JsValue`, this emits a
    /// monomorphised `#[inline(never)]` shim (modelled on `wbg_cast`'s
    /// `breaks_if_inlined`) that, per concrete instantiation, describes its
    /// exact ABI signature and terminates with the
    /// `__wbindgen_describe_generic_import` marker. The CLI interpreter
    /// discovers each monomorphisation, recovers its `(shim key, signature)`,
    /// and rewrites the call site to a manufactured JS binding.
    ///
    /// Supported: free functions, methods, constructors, static methods,
    /// getters/setters (structural and non-structural), owned arguments
    /// (including generic `T`, `Option<T>`, `Vec<T>`, and concrete
    /// references/slices/strings), a bare shared reference to a generic type
    /// parameter (`&T`, passed by value/handle), unit and non-unit returns
    /// (including a generic `-> T`), `catch`, `variadic`, `async` (including a
    /// generic `-> T`, since the value crossing the ABI is the `Promise` and the
    /// resolved value is converted inside `JsFuture<T>`), and `slice_to_array`
    /// for slices with a concrete element type. Argument-position `impl Trait`
    /// (e.g. `fn f(x: impl Clone)`, including nested, e.g. `Vec<impl Clone>`)
    /// is also supported: it is desugared into a synthesized named type
    /// parameter with the same bounds before any of the above logic runs, so
    /// it is monomorphised exactly like a type parameter the user named
    /// themselves.
    ///
    /// User-written trait bounds, both inline (`fn f<T: Trait>`) and in a `where`
    /// clause (including higher-ranked predicates), are part of the declared
    /// contract and are carried through to the generated wrapper, so callers must
    /// satisfy them. They also reach the monomorphised shim, whose ABI signature
    /// may project associated types off a bounded parameter — including a bound
    /// on a parameter hoisted onto the `impl` header, which the shim cannot
    /// inherit and so has restated on its own `where` clause.
    ///
    /// Generic parameters belonging to the imported *class* rather than just the
    /// function (`this: &Holder<T>`, `-> Holder<T>` from a constructor or
    /// self-returning static method, `this: &'a Holder<'a>`) are supported by
    /// *hoisting* them onto the generated `impl` block's own generic header,
    /// reusing the type-erasure path's `get_fn_generics` analysis. Not every
    /// class argument list can be hoisted; see `validate_class_shape` for the
    /// four shapes that are rejected instead.
    ///
    /// Lifetime parameters on the function (e.g. `fn f<'a, T>(x: &'a T)`) are
    /// supported: they carry no runtime information (lifetimes are erased before
    /// the wasm ABI boundary), so the work is just redeclaring them, with their
    /// bounds, on the monomorphised shim below — a nested item, which inherits
    /// none of the wrapper's generics — and, for a method whose receiver names
    /// one (`this: &'a Foo`), binding the receiver as `&'a self`.
    ///
    /// Not (yet) supported, and rejected with a diagnostic:
    /// - class argument lists `validate_class_shape` cannot faithfully hoist;
    /// - a mutable reference to a generic type parameter (`&mut T`), or a
    ///   reference to one nested inside another type (e.g. `Option<&T>`);
    /// - a bare generic type parameter, or a reference to one (`&T`), as the
    ///   `variadic` argument, since it may monomorphise to a non-iterable
    ///   scalar;
    /// - a type parameter in the error position of a `catch` import, since only
    ///   the `Ok` type is monomorphised;
    /// - `slice_to_array` on a slice whose element type mentions a type
    ///   parameter;
    /// - `reexport` and `assert_no_shim`, neither of which has a well-defined
    ///   meaning when one shim is manufactured per monomorphisation.
    ///
    /// Const generic parameters never reach here: `validate_generics` in the
    /// parser rejects them for *every* wasm-bindgen generic, erased or not.
    ///
    /// The rejected shapes generally keep working on the type-erasure path.
    fn try_to_tokens_generic(
        &self,
        tokens: &mut TokenStream,
        class_generics: Option<&syn::Generics>,
        class_cfg_attrs: &[syn::Attribute],
    ) -> Result<(), Diagnostic> {
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

        // --- Generic-parameter guards (opt-in path, so bailing is safe) ---
        //
        // Argument-position `impl Trait` desugars to an anonymous generic
        // type parameter: a function like `fn f(x: impl Clone)` genuinely has
        // one, it just has no name, so it never shows up in `self.generics`.
        // Per-mono codegen needs a real name for every type parameter it
        // monomorphises over — one shows up in `where` bounds, the shim's own
        // generic parameter list, and the `breaks_if_inlined::<..>`
        // turbofish — so give each occurrence one: rewrite every `impl
        // Trait` argument type into a synthesized named type parameter
        // carrying the same bounds, exactly what a user would otherwise have
        // to write by hand (`fn f<T: Trait>(x: T)`). This has to happen
        // before `type_params` is computed below, since an argument whose
        // *only* type parameter is an `impl Trait` would otherwise look like
        // it has none.
        //
        // `arg_types` replaces `arg.pat_type.ty` as the source of truth for
        // every argument's type for the rest of this function.
        let mut synthesized_params: Vec<syn::GenericParam> = Vec::new();
        let arg_types: Vec<syn::Type> = self
            .function
            .arguments
            .iter()
            .map(|arg| {
                let mut ty = (*arg.pat_type.ty).clone();
                generics::desugar_impl_trait(&mut ty, &mut synthesized_params);
                ty
            })
            .collect();
        let synthesized_type_idents = synthesized_params.iter().map(|p| match p {
            syn::GenericParam::Type(tp) => &tp.ident,
            _ => unreachable!("desugar_impl_trait only synthesizes type parameters"),
        });

        let type_params: Vec<&syn::Ident> = self
            .generics
            .type_params()
            .map(|tp| &tp.ident)
            .chain(synthesized_type_idents)
            .collect();
        if type_params.is_empty() {
            bail_span!(
                self.rust_name,
                "experimental_generic_mono requires at least one type parameter"
            );
        }

        // A type-parameter default has no meaning here — every instantiation
        // gets its own shim, so there is no single one to pick a default for —
        // and unlike the type-erasure path (which gives defaults meaning via
        // `concrete_defaults`) per-mono codegen just drops them.
        //
        // Normally rustc's deny-by-default `invalid_type_param_default` lint
        // would catch this on the user's behalf, but nothing of the original
        // signature survives expansion: the wrapper's parameter list is built
        // from bare idents and the shim's from `type_params_with_bounds`, so
        // neither re-emits the default and the lint never fires. Reproduce
        // rustc's own diagnostic verbatim rather than letting it pass silently.
        for type_param in self.generics.type_params() {
            if type_param.default.is_some() {
                bail_span!(
                    type_param,
                    "defaults for generic parameters are not allowed here"
                );
            }
        }

        // --- Determine the receiver/class shape (mirrors the normal path) ---
        let mut class = None;
        let mut is_method = false;
        let mut is_constructor = false;
        let mut is_self_returning_static = false;
        let mut explicit_static_class_generics = false;
        let class_return_path = self.class_return_path();
        if let ast::ImportFunctionKind::Method { ty, kind, .. } = &self.kind {
            class = Some(get_ty(ty).clone());
            match kind {
                ast::MethodKind::Constructor => is_constructor = true,
                ast::MethodKind::Operation(ast::Operation {
                    is_static: false, ..
                }) => is_method = true,
                _ => {}
            }
            explicit_static_class_generics = !is_method
                && !is_constructor
                && class_path_arguments(get_ty(ty)).is_some_and(|arguments| !arguments.is_empty());
            // Constructors and self-returning static methods impl on the return
            // type's class so the manufactured JS binding attaches correctly.
            if class_return_path.is_some() && (is_constructor || !explicit_static_class_generics) {
                class = Some(get_ty(self.js_ret.as_ref().unwrap()).clone());
                if !is_constructor {
                    is_self_returning_static = true;
                }
            }
        }
        if !is_method && class_return_path.is_none() {
            self.validate_unhoisted_class_return_lifetimes()?;
        }
        // Class-level generics (a type parameter or lifetime of the function
        // appearing in the receiver/return *class* type's own argument list,
        // e.g. `this: &Holder<T>` or `this: &'a Holder<'a>`) are supported by
        // reusing the same hoisting analysis the type-erasure path uses: any
        // of the function's own generics that the class type's argument list
        // mentions are *hoisted* onto the `impl` block's own generic header
        // instead of staying on the wrapper function's parameter list. See
        // `get_fn_generics` and its use at the bottom of this function, where
        // the (possibly parameterised) `impl` block is assembled.
        let mut fn_class_generics = self.get_fn_generics()?;
        if let (Some(class), Some(class_generics)) = (&class, class_generics) {
            let mut class_with_defaults = class.clone();
            if is_constructor && class_return_path.is_none() {
                // `class_return_path` rejected a non-constraining constructor
                // return such as `Holder<T::Item>`. `get_fn_generics` then
                // emits `impl Holder`, which selects the imported type's
                // defaults, rather than an impl for that return instantiation.
                // Propagate declaration bounds for the same defaulted self type
                // so the discarded function generic cannot leak into the impl.
                if let syn::Type::Path(syn::TypePath {
                    attrs: _,
                    qself: None,
                    path,
                }) = &mut class_with_defaults
                {
                    if let Some(segment) = path.segments.last_mut() {
                        segment.arguments = syn::PathArguments::None;
                    }
                }
            }
            let shim_lifetimes = generics::lifetime_args(&self.generics);
            fn_class_generics.add_class_bounds(
                class_generics,
                &class_with_defaults,
                &shim_lifetimes,
            )?;
        }

        // Whether the wrapper's enclosing `impl` block can carry class-level
        // generics at all. A static method that is neither the constructor nor
        // self-returning impls on the class's own parameter defaults instead,
        // so there is nothing to hoist and nothing to validate. Spelled once
        // here and reused when the `impl` is assembled at the bottom.
        let hoist = is_method
            || is_constructor
            || is_self_returning_static
            || explicit_static_class_generics;

        if !hoist {
            if let Some(class) = &class {
                match class_generics {
                    // Without declaration metadata, let rustc validate whether the
                    // bare class path can use its imported type's defaults.
                    None => {}
                    Some(generics)
                        if generics.params.iter().any(|parameter| {
                            !matches!(parameter, syn::GenericParam::Type(parameter) if parameter.default.is_some())
                        }) =>
                    {
                        bail_span!(
                            class,
                            "experimental_generic_mono requires static methods on imported classes with required generic parameters to write those arguments in `static_method_of = ...`"
                        );
                    }
                    Some(_) => {}
                }
            }
        }

        // Not every class argument list *can* be reproduced on the generated
        // `impl` header, and the ones that cannot must be rejected here rather
        // than left to fall through as a rustc error against generated code.
        // See `validate_class_shape`.
        if hoist {
            if let Some(class_ty) = &class {
                self.validate_class_shape(class_ty, is_method || explicit_static_class_generics)?;
            }
        }

        // A `variadic` import splats its final argument (`...arg`) on the JS
        // side, which requires that argument to be iterable at runtime.
        //
        // Only check arguments that actually mention a type parameter: a fully
        // concrete variadic argument is the ordinary import path's business, and
        // is left exactly as permissive as it is there. Once a type parameter is
        // involved, though, iterability stops being a property of the
        // declaration and becomes a property of each instantiation, so require a
        // shape that marshals to a JS array for *every* monomorphisation. A bare
        // `T` can be `u32`; `Option<T>`, `Box<T>` and `T::Item` are no better.
        if self.variadic {
            if let Some(ty) = arg_types.last() {
                if generics::uses_generic_params(ty, &type_params) && !is_spreadable_sequence(ty) {
                    bail_span!(
                        ty,
                        "experimental_generic_mono requires the `variadic` argument to be a sequence when \
                         it mentions a type parameter, because it is spread with `...` on the JS \
                         side and must be iterable for every monomorphisation; use `Vec<T>`, \
                         `[T; N]` or `&[T]`, or the type-erasure generic path instead"
                    );
                }
            }
        }

        // `catch` keeps only the `Ok` type and hard-codes the error to `JsValue`
        // (see `extract_first_ty_param` in the parser), so the `?` the codegen
        // emits would need `T: From<JsValue>`. A type parameter in the error
        // position therefore surfaces as "`?` couldn't convert the error to `T`"
        // pointing at `#[wasm_bindgen]`. Reject it with the error type's own span.
        if self.catch {
            if let Some(err_ty) = self
                .function
                .ret
                .as_ref()
                .and_then(|ret| result_err_ty(&ret.r#type))
            {
                if generics::uses_generic_params(err_ty, &type_params) {
                    bail_span!(
                        err_ty,
                        "the error type of a `catch` import must be `JsValue`, not a type \
                         parameter; only the `Ok` type is monomorphised"
                    );
                }
            }
        }

        // --- Per-argument wrapper signature, ABI splat, describe, and bounds ---
        //
        // Each type param stays concrete via rustc monomorphisation, so args are
        // marshalled with the plain `IntoWasmAbi`/`WasmDescribe` traits (no
        // erasure). We add a `where` bound for exactly the arg/return types that
        // mention a type parameter (bounding concrete types would be a trivial
        // bound, which is an error on stable).
        //
        // The user's own bounds are part of the declared signature's contract, so
        // they are carried through verbatim ahead of the synthesized ones: callers
        // are then held to them by rustc, and any associated type they unlock
        // (e.g. `T::Assoc` in an argument) resolves in both the wrapper and the
        // monomorphised shim.
        //
        // Unlike the shim below (which redeclares every parameter from scratch
        // and so can carry inline bounds directly on its own parameter list),
        // the wrapper's declared generics further down only list the params
        // that stay on the function (`fn_class_generics.fn_generic_params`) —
        // anything hoisted onto the enclosing `impl` block's own header
        // (`class_generic_params`) has no parameter-list slot left on the
        // wrapper to carry an inline bound. So the non-hoisted bounds are
        // collected here as plain `where` predicates instead. `get_fn_generics`
        // already derived this list from the combination of inline and
        // `where`-clause bounds on `self.generics`
        // (`generics::generic_bounds`), so nothing further needs collecting
        // from `self.generics` directly here.
        //
        // The hoisted bounds (`class_bounds`) deliberately do *not* appear
        // here: they are emitted as predicates on the `impl` header itself by
        // `class_impl_def` below. A predicate on the method's `where` clause
        // would be accepted but would not *constrain* an impl-header
        // parameter, leaving anything hoisted transitively out of a bound as
        // an unconstrained impl parameter (E0207).
        let mut where_bounds: Vec<TokenStream> = fn_class_generics
            .fn_bounds
            .iter()
            .map(|b| b.to_token_stream())
            .collect();
        let mut wrapper_args = Vec::new();
        let mut shim_abi_args = Vec::new();
        let mut all_prim_names = Vec::new();
        let mut arg_conversions = Vec::new();
        let mut describe_args = Vec::new();
        for (i, arg) in self.function.arguments.iter().enumerate() {
            // Rewritten to desugar any `impl Trait`; see `arg_types` above.
            let ty = &arg_types[i];
            let name = match &*arg.pat_type.pat {
                syn::Pat::Ident(syn::PatIdent {
                    by_ref: None,
                    ident,
                    subpat: None,
                    ..
                }) => ident.clone(),
                syn::Pat::Wild(_) => Ident::new(&format!("__genarg_{i}"), Span::call_site()),
                _ => bail_span!(
                    arg.pat_type.pat,
                    "unsupported pattern in experimental_generic_mono imported function",
                ),
            };

            if generics::uses_generic_params(ty, &type_params) {
                // A bare, shared reference to a generic type parameter (`&T`)
                // is supported: the referent's schema is emitted via `REF`
                // (`WasmDescribe for &T`), and the value is marshalled by the
                // referent-generic `IntoWasmAbi` impls (`&Handle`, `&JsValue`,
                // `&str`, `&[T]`). Because
                // the shim names `<&T as IntoWasmAbi>::Abi` under a late-bound
                // elided lifetime, the required bound is higher-ranked over
                // the referent rather than `&T: IntoWasmAbi`.
                //
                // Still rejected: `&mut T`, and references nested inside
                // another type (e.g. `Option<&T>`, `(T, &T)`, `[&T; N]`,
                // `&&T`), which need the type-erasure path.
                //
                // Note this cannot fire for an argument the `slice_to_array`
                // rewrite below will take over: such an argument must have a
                // concrete element type (checked before the loop), so it never
                // mentions a type parameter and no bound is recorded for it.
                let top_level_shared_ref = match ty {
                    syn::Type::Reference(r) if r.mutability.is_none() => {
                        !generics::references_generic_param(&r.elem, &type_params)
                    }
                    _ => false,
                };
                if let (true, syn::Type::Reference(r)) = (top_level_shared_ref, ty) {
                    let elem = &r.elem;
                    where_bounds.push(quote! {
                        for<'__wbg> &'__wbg #elem: #wasm_bindgen::convert::IntoWasmAbi
                            + #wasm_bindgen::describe::WasmDescribe
                    });
                } else if generics::references_generic_param(ty, &type_params) {
                    bail_span!(
                        ty,
                        "experimental_generic_mono only supports a bare shared reference to a generic \
                         type parameter (`&T`); mutable references (`&mut T`) and references \
                         nested inside another type (e.g. `Option<&T>`) are not supported — \
                         take the argument by value or use the type-erasure generic path"
                    );
                } else {
                    where_bounds.push(quote! {
                        #ty: #wasm_bindgen::convert::IntoWasmAbi + #wasm_bindgen::describe::WasmDescribe
                    });
                }
            }

            // For methods the first argument is the receiver, referred to as
            // `self` and omitted from the explicit parameter list.
            let var = if i == 0 && is_method {
                quote! { self }
            } else {
                wrapper_args.push(quote! { #name: #ty });
                quote! { #name }
            };

            // `slice_to_array`: hand JS an owned `Array` instead of a
            // typed-array view, exactly as on the normal import path. Before
            // this was wired up the attribute was silently ignored here.
            if arg.slice_to_array {
                if let Some(rewrite) = slice_to_array_rewrite(wasm_bindgen, &name, &var, ty) {
                    shim_abi_args.extend(rewrite.abi_args);
                    all_prim_names.extend(rewrite.prim_names);
                    arg_conversions.push(rewrite.conversion);
                    let describe_ty = rewrite.describe_ty;
                    describe_args.push(quote! {
                        <#describe_ty as #wasm_bindgen::describe::WasmDescribe>::describe();
                    });
                    continue;
                }
            }

            let abi = quote! { <#ty as #wasm_bindgen::convert::IntoWasmAbi>::Abi };
            let (args, names) = splat(wasm_bindgen, &name, &abi, Span::call_site());
            shim_abi_args.extend(args);
            arg_conversions.push(quote! {
                let #name = <#ty as #wasm_bindgen::convert::IntoWasmAbi>::into_abi(#var);
                let (#(#names),*) = <#abi as #wasm_bindgen::convert::WasmAbi>::split(#name);
            });
            all_prim_names.extend(names);
            describe_args.push(quote! {
                <#ty as #wasm_bindgen::describe::WasmDescribe>::describe();
            });
        }

        // --- Return handling (mirrors the normal import path) ---
        let ret_ident = Ident::new("_ret", Span::call_site());
        let shim_ret_ty;
        let shim_ret_expr;
        let describe_ret;
        let mut convert_ret;
        // Paths in here go through `#wasm_bindgen::__rt::core` (the `pub extern
        // crate core` re-export) rather than a bare `core::`, because the shim is
        // expanded with call-site hygiene into the user's module: a user item named
        // `core` in that module shadows the extern-prelude entry and the expansion
        // fails with `cannot find function read in module core::ptr`, pointing at
        // code the user never wrote. A 2015-edition consumer would resolve `core::`
        // relative to its crate root and fail outright.
        //
        // The explicit `unsafe` blocks in `shim_ret_expr` below are belt-and-braces
        // rather than load-bearing: rustc suppresses `unsafe_op_in_unsafe_fn` (like
        // most lints) inside an external macro's expansion, so downstream crates do
        // not see it fire even on edition 2024 with `#![deny(warnings)]`. They are
        // kept because they document which operations are actually unsafe, and
        // because an `unsafe` block inside an `unsafe fn` does not trigger
        // `unused_unsafe` on any edition, so they cost nothing.
        let marker_call = quote! {
            #wasm_bindgen::describe::describe_generic_import(
                breaks_if_inlined::<#(#type_params),*> as _,
                &(#(#all_prim_names,)*) as *const _ as _,
            )
        };
        match &self.js_ret {
            Some(syn::Type::Reference(_)) => {
                bail_span!(
                    self.js_ret,
                    "cannot return references in #[wasm_bindgen] imports yet"
                );
            }
            Some(original_ty) => {
                let maybe_async_wrapped;
                let ty = if self.function.r#async {
                    maybe_async_wrapped = parse_quote!(#promise<#original_ty>);
                    &maybe_async_wrapped
                } else {
                    original_ty
                };
                if self.function.r#async {
                    // The resolved value of the promise crosses the closure seam
                    // inside `JsFuture<T>` via `T::from_abi`, so an `async` import
                    // *can* return a monomorphised `T` — the bound that makes
                    // `JsFuture<T>: From<Promise<T>>` hold is the one below, not
                    // `T: WasmDescribe`.
                    if generics::uses_generic_params(original_ty, &type_params) {
                        where_bounds.push(quote! {
                            #original_ty: #wasm_bindgen::convert::FromWasmAbi + 'static
                        });
                    }
                } else if generics::uses_generic_params(ty, &type_params) {
                    where_bounds.push(quote! {
                        #ty: #wasm_bindgen::convert::FromWasmAbi
                            + #wasm_bindgen::describe::WasmDescribe
                    });
                }
                shim_ret_ty = quote! {
                    #wasm_bindgen::convert::WasmRet<<#ty as #wasm_bindgen::convert::FromWasmAbi>::Abi>
                };
                shim_ret_expr =
                    quote! { unsafe { #wasm_bindgen::__rt::core::ptr::read(#marker_call as _) } };
                describe_ret =
                    import_describe_ret(wasm_bindgen, Some(original_ty), self.function.r#async);
                convert_ret = quote! {
                    <#ty as #wasm_bindgen::convert::FromWasmAbi>::from_abi(#ret_ident.join())
                };
                if self.function.r#async {
                    convert_ret = quote! {
                        #futures::JsFuture::from(
                            <#promise<#original_ty> as #wasm_bindgen::convert::FromWasmAbi>
                                ::from_abi(#ret_ident.join())
                        ).await
                    };
                    convert_ret = if self.catch {
                        quote! { Ok(#convert_ret?) }
                    } else {
                        quote! { #convert_ret.expect("uncaught exception") }
                    };
                }
            }
            None => {
                if self.function.r#async {
                    shim_ret_ty = quote! {
                        #wasm_bindgen::convert::WasmRet<<#promise as #wasm_bindgen::convert::FromWasmAbi>::Abi>
                    };
                    shim_ret_expr = quote! { unsafe { #wasm_bindgen::__rt::core::ptr::read(#marker_call as _) } };
                    describe_ret = import_describe_ret(wasm_bindgen, None, true);
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
                    shim_ret_ty = quote! { () };
                    shim_ret_expr = quote! { let _ = unsafe { #marker_call }; };
                    describe_ret = import_describe_ret(wasm_bindgen, None, false);
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

        // --- Descriptor stream: [key string][FUNCTION signature] ---
        let key = self.shim.to_string();
        // The shim key is built in `parser.rs` from an ASCII-filtered function name
        // plus a hex hash, so it is always ASCII; the `chars().count()` below is
        // therefore equal to `len()`, but it is what the wire format actually
        // wants (one word per `char`) and stays correct if that ever changes.
        debug_assert!(key.is_ascii(), "shim keys are ASCII-filtered in parser.rs");
        let key_len = key.chars().count() as u32;
        let key_chars = key.chars().map(|c| c as u32);
        let nargs = self.function.arguments.len() as u32;

        // --- Assemble ---
        let vis = &self.function.rust_vis;
        let rust_name = &self.rust_name;
        let attrs = &self.function.rust_attrs;
        let ret = match self.function.ret.as_ref().map(|r| &r.r#type) {
            Some(ty) => quote! { -> #ty },
            None => quote!(),
        };
        let doc = if self.doc_comment.is_empty() {
            quote! {}
        } else {
            let doc_comment = &self.doc_comment;
            quote! { #[doc = #doc_comment] }
        };
        // The wrapper's own declared generics are only the *remaining*,
        // non-hoisted parameters: anything the class type's argument list
        // uses (`class_generic_params` / `class_lifetime_params` /
        // `class_bound_lifetime_params`) moves onto the `impl` block's own
        // header instead, assembled below. These are bare idents/lifetimes with
        // no inline bounds attached: every inline bound, on a hoisted parameter
        // or not, was reified into a `where` predicate by
        // `generics::generic_bounds` inside `get_fn_generics` and has already
        // been emitted — a non-hoisted one into `where_bounds` above, a hoisted
        // one onto the `impl` header via `class_impl_def`. Type-param defaults
        // cannot appear at all — they are rejected up front; see the note on the
        // shim's parameter list below.
        //
        // Type parameters synthesized from argument-position `impl Trait` are
        // appended too, with their bounds inline: they exist only in argument
        // positions, never in the class type's argument list, so they are never
        // hoisted and are not in `self.generics` for `get_fn_generics` to see.
        let fn_generic_params = &fn_class_generics.fn_generic_params;
        let fn_lifetime_params = &fn_class_generics.fn_lifetime_params;
        let generic_params = quote! {
            #(#fn_lifetime_params,)* #(#fn_generic_params,)* #(#synthesized_params),*
        };
        // The shim redeclares the wrapper's type *and* lifetime parameters,
        // since it's a nested item and inherits none of the wrapper's generics.
        // Type params need their inline bounds too: its signature names ABI
        // types projected off them (`<T::Assoc as IntoWasmAbi>::Abi`), which
        // only resolve under the bound.
        //
        // Type-param *defaults* are not carried here, and cannot reach this
        // point: they are rejected up front with rustc's own "defaults for
        // generic parameters are not allowed here", since neither this list nor
        // the wrapper's re-emits them and so rustc's deny-by-default
        // `invalid_type_param_default` lint would never fire.
        //
        // Lifetime params carry their inline bounds (`<'a: 'b>`) across too, and
        // here that is load-bearing in both directions: the shim needs them so a
        // `where` predicate that relates two lifetimes stays provable, and the
        // wrapper needs them so its own declaration is not strictly weaker than
        // the shim it calls. The wrapper has no parameter-list slot for a
        // hoisted parameter's inline bound, so it carries lifetime bounds as
        // synthesized `where` predicates instead — see `generics::generic_bounds`,
        // which reifies inline lifetime bounds for exactly this reason.
        let shim_generic_params: Vec<TokenStream> =
            generics::lifetime_params_with_bounds(&self.generics)
                .into_iter()
                .chain(generics::type_params_with_bounds(&self.generics))
                .chain(synthesized_params.iter().map(|p| quote! { #p }))
                .collect();
        let where_clause = if where_bounds.is_empty() {
            quote! {}
        } else {
            quote! { where #(#where_bounds),* }
        };
        // The shim needs the hoisted bounds too, even though the wrapper must
        // not have them (see the note where `where_bounds` is built: a
        // predicate on the wrapper's own `where` clause does not constrain an
        // impl-header parameter, so they live on the `impl` header instead).
        // The shim is a *nested* item, so it inherits nothing from that header,
        // yet its ABI signature may project associated types off a hoisted,
        // bounded parameter — `<T::Item as IntoWasmAbi>::Abi` for a
        // `fn f<T>(this: &Boxed<T>, v: T::Item) where T: IntoIterator`, which
        // is `E0220` without the bound back in scope.
        //
        // Inline bounds on the same parameter also arrive via the shim's own
        // parameter list (`type_params_with_bounds`), so a hoisted inline bound
        // is stated twice here. That is redundant, not an error.
        let shim_where_clause =
            if where_bounds.is_empty() && fn_class_generics.class_bounds.is_empty() {
                quote! {}
            } else {
                let class_bounds = fn_class_generics.class_bounds.iter();
                quote! { where #(#where_bounds,)* #(#class_bounds),* }
            };
        // Bind the receiver at whatever lifetime the declaration named it with
        // (`this: &'a Foo` -> `&'a self`). The argument loop above marshals the
        // receiver as `<&'a Foo as IntoWasmAbi>::into_abi(self)`, so a plain
        // `&self` would give `self` an anonymous, caller-chosen lifetime that
        // rustc cannot prove outlives `'a` — "lifetime may not live long
        // enough", reported on the `#[wasm_bindgen]` attribute. An elided `'_`
        // is dropped rather than forwarded: it is not a name `&'_ self` can be
        // bound at, and it means the same thing as the elided `&self`.
        let me = if is_method {
            let recv_lifetime = self
                .function
                .arguments
                .first()
                .and_then(|arg| match get_ty(&arg.pat_type.ty) {
                    syn::Type::Reference(r) => r.lifetime.as_ref(),
                    _ => None,
                })
                .filter(|lt| lt.ident != "_");
            quote! { & #recv_lifetime self, }
        } else {
            quote!()
        };
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

        // The monomorphised shim's signature names ABI projections and, for `&T`
        // arguments, a higher-ranked `IntoWasmAbi` bound. When one of those does
        // not hold, rustc reports it at the offending token's span — which is
        // `Span::call_site()` for everything `quote!` emits, i.e. the
        // `#[wasm_bindgen]` attribute on the enclosing `extern "C"` block. The
        // ordinary import path avoids that by respanning its `extern_fn` onto the
        // function's own name; do the same here so an HRTB or `WasmAbi::PrimN`
        // failure at least says which import it belongs to.
        let fn_span = Span::call_site().located_at(self.rust_name.span());
        let shim = respan_all(
            quote! {
                // Route through `__wbindgen_coverage!` rather than writing
                // `#[cfg_attr(wasm_bindgen_unstable_test_coverage, ..)]` here:
                // that cfg is only declared inside the `wasm-bindgen` crate, so
                // naming it in generated code warns (`unexpected_cfgs`) in every
                // downstream crate, and the bare `#[coverage(off)]` it expands to
                // needs the `allow_internal_unstable` that the macro carries.
                #wasm_bindgen::__wbindgen_coverage! {
                #[inline(never)]
                unsafe extern "C" fn breaks_if_inlined<#(#shim_generic_params),*>(
                    #(#shim_abi_args),*
                ) -> #shim_ret_ty
                #shim_where_clause
                {
                    use #wasm_bindgen::describe::*;
                    // Leading length-prefixed `shim` key identifying the AST
                    // entry that supplies this import's JS binding metadata.
                    inform(#key_len);
                    #(inform(#key_chars);)*
                    // Concrete FUNCTION signature for this monomorphisation.
                    inform(FUNCTION);
                    inform(0);
                    inform(#nargs);
                    #(#describe_args)*
                    #describe_ret
                    #describe_ret
                    #shim_ret_expr
                }
                }
            },
            fn_span,
        );

        let invocation = quote! {
            #[allow(nonstandard_style)]
            #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
            #(#attrs)*
            #doc
            #vis #maybe_async #maybe_unsafe fn #rust_name <#generic_params> (#me #(#wrapper_args),*) #ret #where_clause {
                #shim

                unsafe {
                    let #ret_ident = {
                        #(#arg_conversions)*
                        breaks_if_inlined::<#(#type_params),*>(#(#all_prim_names),*)
                    };
                    #exceptional_ret
                    #convert_ret
                }
            }
        };

        if let Some(class) = class {
            // Strip any generic arguments from the class type's last path
            // segment so the bare class name is left to parameterise below
            // (or to `impl` directly, if there is nothing to hoist).
            let mut class = class;
            if let syn::Type::Path(syn::TypePath {
                attrs: _,
                qself: None,
                path,
            }) = &mut class
            {
                if let Some(segment) = path.segments.last_mut() {
                    segment.arguments = syn::PathArguments::None;
                }
            }
            // `hoist` is the same gate the type-erasure path applies, computed
            // once above where the class shape is determined and validated.
            let class_impl_def = fn_class_generics.class_impl_def(&class, hoist);
            let function_cfg_attrs = crate::cfg_gate_attrs(attrs);
            quote! {
                #(#function_cfg_attrs)*
                #(#class_cfg_attrs)*
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
    class_cfg_attrs: Vec<syn::Attribute>,
}

// Extracted impl block info given class generics and function-level method generics
struct FnClassGenerics<'a> {
    // the hoisted class-level param idents used, with identifiers renamed to use function generic identifier names
    class_generic_params: BTreeSet<syn::Ident>,
    // the struct generic expressions on those params
    class_generic_exprs: Vec<&'a syn::Type>,
    // class where bounds including hoisted function bounds
    class_bounds: Vec<syn::WherePredicate>,
    // the remaining non-hoisted function-level param idents
    fn_generic_params: Vec<&'a syn::Ident>,
    // function bounds on params which are only specific to the function not hoisted as class bounds
    fn_bounds: Vec<Cow<'a, syn::WherePredicate>>,
    // the union of class-level defaults (for identifier generics) and function defaults
    // this is used to form the concrete type via replacement (using JsValue otherwise)
    concrete_defaults: BTreeMap<&'a syn::Ident, Option<Cow<'a, syn::Type>>>,
    // hoisted class-level lifetime params, deduplicated, for the impl header
    class_lifetime_params: Vec<&'a syn::Lifetime>,
    // the class type's lifetime arguments as written, in position, including
    // repeats and lifetimes the function does not declare; passed to the type
    class_lifetime_args: Vec<syn::Lifetime>,
    // hoisted class-level lifetime params only used in bounds (not passed to type)
    class_bound_lifetime_params: Vec<syn::Lifetime>,
    // the remaining non-hoisted function-level lifetime params
    fn_lifetime_params: Vec<&'a syn::Lifetime>,
}

fn class_path_arguments(class: &syn::Type) -> Option<Vec<syn::GenericArgument>> {
    let syn::Type::Path(syn::TypePath {
        attrs: _,
        qself: None,
        path,
    }) = class
    else {
        return None;
    };
    let segment = path.segments.last()?;
    match &segment.arguments {
        syn::PathArguments::None => Some(Vec::new()),
        syn::PathArguments::AngleBracketed(arguments) => {
            Some(arguments.args.iter().cloned().collect())
        }
        syn::PathArguments::Parenthesized(_) => None,
    }
}

struct ClassGenericSubstituter<'a> {
    type_replacements: &'a BTreeMap<Ident, syn::Type>,
    lifetime_replacements: &'a BTreeMap<Ident, syn::Lifetime>,
    unsupported_projection: bool,
}

impl VisitMut for ClassGenericSubstituter<'_> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        let syn::Type::Path(type_path) = ty else {
            return visit_mut::visit_type_mut(self, ty);
        };
        if let Some(qself) = &mut type_path.qself {
            self.visit_type_mut(&mut qself.ty);
        }
        if type_path.qself.is_none()
            && type_path.path.leading_colon.is_none()
            && !type_path.path.segments.is_empty()
        {
            let first = &type_path.path.segments[0].ident;
            if let Some(replacement) = self.type_replacements.get(first) {
                if type_path.path.segments.len() == 1 {
                    if ty == replacement {
                        return;
                    }
                    *ty = replacement.clone();
                    return;
                }
                let syn::Type::Path(replacement_path) = replacement else {
                    // `T::Assoc` cannot be rewritten as `(U, u8)::Assoc`.
                    self.unsupported_projection = true;
                    return;
                };
                if replacement_path.qself.is_some() || replacement_path.path.leading_colon.is_some()
                {
                    // Appending `::Assoc` to `<U as Trait>::Item` requires a
                    // new qualified-self type; splicing its path would discard
                    // the qualification and change the bound's meaning.
                    self.unsupported_projection = true;
                    return;
                }
                let remaining = type_path
                    .path
                    .segments
                    .iter()
                    .skip(1)
                    .cloned()
                    .collect::<Vec<_>>();
                type_path.path.leading_colon = replacement_path.path.leading_colon;
                type_path.path.segments = replacement_path.path.segments.clone();
                type_path.path.segments.extend(remaining);

                // Replacements are simultaneous. Only the original suffix may
                // contain identifiers that still need replacement.
                for segment in type_path
                    .path
                    .segments
                    .iter_mut()
                    .skip(replacement_path.path.segments.len())
                {
                    self.visit_path_arguments_mut(&mut segment.arguments);
                }
                return;
            }
        }
        // `qself` was visited above. Visiting the whole type again would apply
        // substitutions twice to `<T as Trait>::Assoc` expressions.
        for segment in &mut type_path.path.segments {
            self.visit_path_arguments_mut(&mut segment.arguments);
        }
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if let Some(replacement) = self.lifetime_replacements.get(&lifetime.ident) {
            *lifetime = replacement.clone();
        }
    }
}

fn class_generic_substituter<'a>(
    type_replacements: &'a BTreeMap<Ident, syn::Type>,
    lifetime_replacements: &'a BTreeMap<Ident, syn::Lifetime>,
) -> ClassGenericSubstituter<'a> {
    ClassGenericSubstituter {
        type_replacements,
        lifetime_replacements,
        unsupported_projection: false,
    }
}

fn substitute_class_type(
    ty: &mut syn::Type,
    type_replacements: &BTreeMap<Ident, syn::Type>,
    lifetime_replacements: &BTreeMap<Ident, syn::Lifetime>,
) -> bool {
    let mut substituter = class_generic_substituter(type_replacements, lifetime_replacements);
    substituter.visit_type_mut(ty);
    !substituter.unsupported_projection
}

fn substitute_class_predicate(
    predicate: &mut syn::WherePredicate,
    type_replacements: &BTreeMap<Ident, syn::Type>,
    lifetime_replacements: &BTreeMap<Ident, syn::Lifetime>,
) -> bool {
    let mut substituter = class_generic_substituter(type_replacements, lifetime_replacements);
    substituter.visit_where_predicate_mut(predicate);
    !substituter.unsupported_projection
}

fn class_argument_has_elided_lifetime(ty: &syn::Type) -> bool {
    struct Visitor {
        found: bool,
    }

    impl<'ast> syn::visit::Visit<'ast> for Visitor {
        fn visit_lifetime(&mut self, lifetime: &'ast syn::Lifetime) {
            self.found |= lifetime.ident == "_";
        }

        fn visit_type_reference(&mut self, reference: &'ast syn::TypeReference) {
            self.found |= reference.lifetime.is_none();
            syn::visit::visit_type_reference(self, reference);
        }
    }

    let mut visitor = Visitor { found: false };
    syn::visit::Visit::visit_type(&mut visitor, ty);
    visitor.found
}

fn class_argument_has_inferred_type(ty: &syn::Type) -> bool {
    struct Visitor {
        found: bool,
    }

    impl<'ast> syn::visit::Visit<'ast> for Visitor {
        fn visit_type_infer(&mut self, _: &'ast syn::TypeInfer) {
            self.found = true;
        }
    }

    let mut visitor = Visitor { found: false };
    syn::visit::Visit::visit_type(&mut visitor, ty);
    visitor.found
}

fn class_argument_has_impl_trait(ty: &syn::Type) -> bool {
    struct Visitor {
        found: bool,
    }

    impl<'ast> syn::visit::Visit<'ast> for Visitor {
        fn visit_type_impl_trait(&mut self, _: &'ast syn::TypeImplTrait) {
            self.found = true;
        }
    }

    let mut visitor = Visitor { found: false };
    syn::visit::Visit::visit_type(&mut visitor, ty);
    visitor.found
}

fn predicate_binds_replacement_lifetime(
    predicate: &syn::WherePredicate,
    type_replacements: &BTreeMap<Ident, syn::Type>,
    lifetime_replacements: &BTreeMap<Ident, syn::Lifetime>,
    additional_replacements: &[&syn::Lifetime],
) -> bool {
    struct Visitor<'a> {
        replacements: &'a BTreeSet<Ident>,
        conflict: bool,
    }

    impl<'ast> syn::visit::Visit<'ast> for Visitor<'_> {
        fn visit_bound_lifetimes(&mut self, lifetimes: &'ast syn::BoundLifetimes) {
            self.conflict |= lifetimes.lifetimes.iter().any(|parameter| {
                matches!(
                    parameter,
                    syn::GenericParam::Lifetime(parameter)
                        if self.replacements.contains(&parameter.lifetime.ident)
                )
            });
            syn::visit::visit_bound_lifetimes(self, lifetimes);
        }
    }

    let mut replacements = lifetime_replacements
        .values()
        .chain(additional_replacements.iter().copied())
        .map(|lifetime| lifetime.ident.clone())
        .collect::<BTreeSet<_>>();
    struct ReplacementVisitor<'a>(&'a mut BTreeSet<Ident>);

    impl<'ast> syn::visit::Visit<'ast> for ReplacementVisitor<'_> {
        fn visit_lifetime(&mut self, lifetime: &'ast syn::Lifetime) {
            if lifetime.ident != "static" && lifetime.ident != "_" {
                self.0.insert(lifetime.ident.clone());
            }
        }
    }

    let mut replacement_visitor = ReplacementVisitor(&mut replacements);
    for replacement in type_replacements.values() {
        syn::visit::Visit::visit_type(&mut replacement_visitor, replacement);
    }
    let mut visitor = Visitor {
        replacements: &replacements,
        conflict: false,
    };
    syn::visit::Visit::visit_where_predicate(&mut visitor, predicate);
    visitor.conflict
}

impl<'a> FnClassGenerics<'a> {
    /// Adds a new function bound, checking it is not already a bound
    fn add_fn_bound(&mut self, bound: syn::WherePredicate) {
        if !self.fn_bounds.iter().any(|existing| **existing == bound) {
            self.fn_bounds.push(Cow::Owned(bound));
        }
    }

    fn add_class_bounds(
        &mut self,
        class_generics: &syn::Generics,
        class: &syn::Type,
        additional_conflicting_lifetimes: &[&syn::Lifetime],
    ) -> Result<(), Diagnostic> {
        let Some(arguments) = class_path_arguments(class) else {
            return Ok(());
        };

        let mut type_replacements = BTreeMap::new();
        let mut lifetime_replacements = BTreeMap::new();
        let mut arguments = arguments.iter();
        for parameter in &class_generics.params {
            match (parameter, arguments.next()) {
                (syn::GenericParam::Type(parameter), Some(syn::GenericArgument::Type(ty))) => {
                    type_replacements.insert(parameter.ident.clone(), ty.clone());
                }
                (
                    syn::GenericParam::Lifetime(parameter),
                    Some(syn::GenericArgument::Lifetime(lifetime)),
                ) => {
                    lifetime_replacements
                        .insert(parameter.lifetime.ident.clone(), lifetime.clone());
                }
                (syn::GenericParam::Lifetime(_), _) => {
                    let syn::Type::Path(syn::TypePath { path, .. }) = class else {
                        unreachable!("class_path_arguments accepted a non-path class")
                    };
                    let segment = path.segments.last().unwrap();
                    bail_span!(
                        segment,
                        "imported classes with lifetime parameters require explicit lifetime arguments on generated impls; name the lifetime as a function parameter (e.g. `fn get<'a, T, U>(this: &'a Holder<'a, T>) -> U`)"
                    );
                }
                // Omitted type arguments use the default added by the parser.
                (syn::GenericParam::Type(parameter), None) if parameter.default.is_some() => {
                    let mut default = parameter.default.clone().unwrap().1;
                    if !substitute_class_type(
                        &mut default,
                        &type_replacements,
                        &lifetime_replacements,
                    ) {
                        bail_span!(
                            class,
                            "wasm-bindgen cannot substitute a projected imported-type default for a non-path class argument"
                        );
                    }
                    type_replacements.insert(parameter.ident.clone(), default);
                }
                _ => return Ok(()),
            }
        }

        let bounds = generics::generic_bounds(class_generics)
            .into_iter()
            .map(Cow::into_owned)
            .collect::<Vec<_>>();
        for mut bound in bounds {
            if predicate_binds_replacement_lifetime(
                &bound,
                &type_replacements,
                &lifetime_replacements,
                additional_conflicting_lifetimes,
            ) {
                bail_span!(
                    class,
                    "an imported-class bound conflicts with a function lifetime of the same name"
                );
            }
            if !substitute_class_predicate(&mut bound, &type_replacements, &lifetime_replacements) {
                bail_span!(
                    class,
                    "wasm-bindgen cannot substitute an imported-type bound that projects a non-path class argument"
                );
            }
            if !self.class_bounds.contains(&bound) {
                self.class_bounds.push(bound);
            }
        }
        Ok(())
    }

    /// Whether anything was hoisted onto the enclosing `impl` block's own
    /// generic header, i.e. whether the parameterised `impl<..> Class<..>`
    /// form is needed at all rather than a bare `impl Class`.
    fn has_class_generics(&self) -> bool {
        !self.class_generic_params.is_empty()
            || !self.class_lifetime_params.is_empty()
            || !self.class_bound_lifetime_params.is_empty()
    }

    /// Assembles the `impl` header that the generated wrapper method is
    /// emitted into, shared by the type-erasure and per-monomorphisation
    /// paths so the two cannot drift apart.
    ///
    /// `class` must already have had the generic arguments stripped from its
    /// last path segment; any that need to survive are re-emitted here from
    /// `class_lifetime_args` and `class_generic_exprs`, both of which are in
    /// the order the class type's argument list was written in — unlike the
    /// alphabetically-ordered `class_generic_params` set and the deduplicated
    /// `class_lifetime_params`, which are the impl *header* and so must list
    /// each parameter exactly once.
    ///
    /// `hoist` must be false for shapes that cannot carry class-level
    /// generics — a static method that is neither a constructor nor
    /// self-returning — which impl on the class's own parameter defaults
    /// instead.
    fn class_impl_def(&self, class: &syn::Type, hoist: bool) -> TokenStream {
        if !hoist {
            return quote! { impl #class };
        }
        // Type lifetimes: declared on the impl (deduplicated, since a header
        // may bind each name once) AND passed to the type (positionally, as
        // written — `Holder<'b, 'a>` must not come back out as `Holder<'a, 'b>`,
        // and `Holder<'a, 'a>` must not collapse to a single argument).
        //
        // The two lists diverge as soon as a class type carries more than one
        // lifetime argument, which a multi-lifetime declaration (`type Foo<'a,
        // 'b>;`) expresses directly — the reference-conversion impls declare
        // the type's own lifetime params (see `reference_impl_generics`),
        // so that declaration compiles. Keeping the header and the self type on
        // separate lists is therefore load-bearing, not merely defensive.
        let class_lifetime_params = &self.class_lifetime_params;
        let class_lifetime_args = &self.class_lifetime_args;
        // Bound-only lifetimes: appear on the impl but are NOT passed to the type.
        let class_bound_lifetime_params = &self.class_bound_lifetime_params;
        let class_generic_params = &self.class_generic_params;
        let class_generic_exprs = &self.class_generic_exprs;

        if !self.has_class_generics() {
            // Nothing of the function's own generics is hoisted, so the impl
            // header stays empty — but the class type may still carry a fully
            // concrete argument list (`&Holder<u32>`), and that list has to be
            // re-emitted regardless. `class` arrives with its arguments already
            // stripped, so dropping them here would target the class's own
            // parameter *defaults* (`impl Holder`) rather than the type as
            // written, i.e. a receiver-type mismatch on the generated method.
            //
            // With no arguments at all, the bare `impl Holder` *is* the type as
            // written, and binding the defaults is the intended behaviour.
            if class_lifetime_args.is_empty() && class_generic_exprs.is_empty() {
                return quote! { impl #class };
            }
            return quote! {
                impl #class <#(#class_lifetime_args,)* #(#class_generic_exprs),*>
            };
        }

        // Bounds on hoisted parameters have to become *impl-level* predicates.
        // A predicate left on the wrapper method's own `where` clause does not
        // constrain a parameter declared on the impl header (RFC 447), so
        // anything hoisted transitively out of a bound — `Ret` in
        // `F: JsFunction<Ret = Ret>`, say — would otherwise be an unconstrained
        // impl parameter, i.e. an E0207 reported against generated code.
        let impl_where_clause = if self.class_bounds.is_empty() {
            quote! {}
        } else {
            let class_bounds = self.class_bounds.iter();
            quote! { where #(#class_bounds),* }
        };
        quote! {
            impl<#(#class_lifetime_params,)* #(#class_bound_lifetime_params,)* #(#class_generic_params),*>
                #class <#(#class_lifetime_args,)* #(#class_generic_exprs),*>
            #impl_where_clause
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
        let all_lifetime_params = generics::lifetime_args(&self.generics);
        let mut fn_lifetime_params: Vec<&syn::Lifetime> = all_lifetime_params.clone();

        let mut class_bounds: Vec<syn::WherePredicate> = Vec::new();
        let mut fn_bounds = generics::generic_bounds(&self.generics);
        let mut class_generic_params = BTreeSet::new();
        let mut class_lifetime_params_set = BTreeSet::new();
        let mut class_bound_lifetime_params_set: BTreeSet<syn::Lifetime> = BTreeSet::new();
        let mut class_generic_exprs = Vec::new();
        let mut class_lifetime_args: Vec<syn::Lifetime> = Vec::new();

        let mut class = None;
        if let ast::ImportFunctionKind::Method {
            ty,
            kind: ast::MethodKind::Operation(_),
            ..
        } = &self.kind
        {
            let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
                unreachable!(); // validated at parse time
            };
            if class_path_arguments(get_ty(ty)).is_some_and(|arguments| !arguments.is_empty()) {
                class = Some(path);
            }
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
                    // Handle lifetime arguments for hoisting.
                    //
                    // Two separate lists come out of this. `class_lifetime_args`
                    // records the argument *as written*, in position, including
                    // repeats and lifetimes the function does not declare
                    // (`'static`): it is re-emitted verbatim as the self type's
                    // leading generic arguments, so it has to preserve the
                    // declared arity. `class_lifetime_params_set` is the subset
                    // that names one of the function's own lifetime parameters,
                    // i.e. the ones that also have to be *declared* on the impl
                    // header, where each may appear only once.
                    if let syn::GenericArgument::Lifetime(lt) = gen_arg {
                        class_lifetime_args.push(lt.clone());
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

                    // Also find lifetimes used *inside* this class generic
                    // expression (e.g. `Holder<&'a u32>`). These go in the
                    // bound-only bucket: `class_lifetime_params` is re-emitted
                    // as a *leading generic argument* of the self type, but a
                    // lifetime nested in a type argument is already carried by
                    // that argument's own tokens. Adding it again as a separate
                    // argument would give the self type the wrong arity
                    // (`impl<'a, T> Holder<'a, &'a u32>` for a one-parameter
                    // `Holder`). It still has to appear on the impl *header*,
                    // hence the bound-only bucket.
                    let used_lifetimes = generics::used_lifetimes_in_type(ty, &all_lifetime_params);
                    for lt in used_lifetimes {
                        if !class_lifetime_params_set.contains(&lt) {
                            class_bound_lifetime_params_set.insert(lt);
                        }
                    }
                }

                // Transitively hoist generic params and lifetimes from associated-type
                // equality values in bounds on already-hoisted params. `Ret` in
                // `F: JsFunction<Ret = Ret>` is determined by that equality, but
                // `U` in `F: Trait<U>` is merely a trait argument and remains
                // unconstrained by `Holder<F>` (E0207 if placed on the impl).
                // We only inspect bounds where the bounded type IS a class param.
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
                        let mut predicate_params_to_add = BTreeSet::new();
                        // Only process bounds where the bounded type IS a class param
                        // e.g., for `F: JsFunction<Ret = Ret>`, bounded_ty is `F`
                        if let syn::WherePredicate::Type(pred_type) = bound.as_ref() {
                            if let syn::Type::Path(type_path) = &pred_type.bounded_ty {
                                if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
                                    let bounded_ident = &type_path.path.segments[0].ident;
                                    if class_generic_params.contains(bounded_ident) {
                                        // Only associated-type equality RHSs constrain
                                        // another impl parameter. Ordinary trait arguments
                                        // and associated-type constraints do not.
                                        for type_bound in &pred_type.bounds {
                                            let syn::TypeParamBound::Trait(trait_bound) =
                                                type_bound
                                            else {
                                                continue;
                                            };
                                            for segment in &trait_bound.path.segments {
                                                let syn::PathArguments::AngleBracketed(arguments) =
                                                    &segment.arguments
                                                else {
                                                    continue;
                                                };
                                                for argument in &arguments.args {
                                                    let syn::GenericArgument::AssocType(binding) =
                                                        argument
                                                    else {
                                                        continue;
                                                    };
                                                    if !generics::type_is_constraining_for(
                                                        &binding.ty,
                                                        &remaining_fn_params,
                                                    ) {
                                                        continue;
                                                    }
                                                    // An equality RHS that depends on a
                                                    // function lifetime stays on the
                                                    // method; moving it to the impl loses
                                                    // the argument's implied bounds.
                                                    if !generics::used_lifetimes_in_type(
                                                        &binding.ty,
                                                        &remaining_fn_lifetimes,
                                                    )
                                                    .is_empty()
                                                    {
                                                        continue;
                                                    }
                                                    let mut found_set = BTreeSet::new();
                                                    let mut visitor =
                                                        generics::GenericNameVisitor::new(
                                                            &remaining_fn_params,
                                                            &mut found_set,
                                                        );
                                                    syn::visit::Visit::visit_type(
                                                        &mut visitor,
                                                        &binding.ty,
                                                    );
                                                    predicate_params_to_add.extend(found_set);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // A predicate can move to the impl only when every
                        // function parameter it names is determined by an
                        // associated-type equality, and it touches no remaining
                        // function lifetime. For `F: Rel<V, Output = U>`, `V`
                        // remains function-level, so `U` must too.
                        let mut predicate_params = BTreeSet::new();
                        let mut visitor = generics::GenericNameVisitor::new(
                            &remaining_fn_params,
                            &mut predicate_params,
                        );
                        syn::visit::Visit::visit_where_predicate(&mut visitor, bound);
                        if predicate_params == predicate_params_to_add
                            && generics::used_lifetimes_in_predicate(bound, &remaining_fn_lifetimes)
                                .is_empty()
                        {
                            params_to_add.extend(predicate_params_to_add);
                        }

                        // A lifetime predicate which mentions a class lifetime
                        // also belongs on the impl header. Hoist any other
                        // function lifetimes it relates so the predicate does
                        // not leave the wrapper referring to an undeclared
                        // lifetime after the class lifetime moves to the impl.
                        if let syn::WherePredicate::Lifetime(_) = bound.as_ref() {
                            let class_lifetimes: Vec<&syn::Lifetime> = class_lifetime_params_set
                                .iter()
                                .chain(class_bound_lifetime_params_set.iter())
                                .collect();
                            if !generics::used_lifetimes_in_predicate(bound, &class_lifetimes)
                                .is_empty()
                            {
                                let used = generics::used_lifetimes_in_predicate(
                                    bound,
                                    &remaining_fn_lifetimes,
                                );
                                lifetimes_to_add.extend(used);
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
                    let class_lifetimes: Vec<&syn::Lifetime> = class_lifetime_params_set
                        .iter()
                        .chain(class_bound_lifetime_params_set.iter())
                        .collect();
                    let uses_class_params =
                        generics::generics_predicate_uses(bound, &class_generic_params_refs)
                            || !generics::used_lifetimes_in_predicate(bound, &class_lifetimes)
                                .is_empty();
                    let uses_fn_params =
                        generics::generics_predicate_uses(bound, &fn_generic_params)
                            || !generics::used_lifetimes_in_predicate(bound, &fn_lifetime_params)
                                .is_empty();
                    if uses_class_params && !uses_fn_params {
                        class_bounds.push(bound.clone().into_owned());
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

        // Convert class_bound_lifetime_params_set to Vec, maintaining order from
        // original params. The two buckets are both emitted on the impl header,
        // so a lifetime in both would be declared twice; the type-argument
        // bucket wins since it also has to be passed to the type.
        let class_bound_lifetime_params: Vec<syn::Lifetime> = all_lifetime_params
            .iter()
            .copied()
            .filter(|lt| {
                class_bound_lifetime_params_set.contains(*lt)
                    && !class_lifetime_params_set.contains(*lt)
            })
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
            class_lifetime_args,
            class_bound_lifetime_params,
            fn_lifetime_params,
        })
    }

    /// Rejects class-type shapes whose generic argument list the `impl`
    /// assembly in [`FnClassGenerics::class_impl_def`] cannot faithfully
    /// reproduce, so the user gets a diagnostic pointed at their own signature
    /// rather than a rustc error against generated code they never wrote,
    /// pointed at the `#[wasm_bindgen]` attribute.
    ///
    /// `class` is the type the wrapper will be emitted into an `impl` block
    /// for: the receiver's type for a method, or the return type for a
    /// constructor or self-returning static method. The final check applies
    /// whenever the class arguments are emitted directly on the impl header.
    fn validate_class_shape(
        &self,
        class: &syn::Type,
        require_constraining_arguments: bool,
    ) -> Result<(), Diagnostic> {
        let syn::Type::Path(syn::TypePath {
            attrs: _,
            qself: None,
            path,
        }) = class
        else {
            return Ok(());
        };
        let Some(syn::PathSegment {
            arguments: syn::PathArguments::AngleBracketed(gen_args),
            ..
        }) = path.segments.last()
        else {
            return Ok(());
        };

        // An elided lifetime argument on the class type (`Holder<'_, T>`) is
        // not a parameter that can
        // be hoisted onto the `impl` header. The argument itself survives into
        // the self type (`class_lifetime_args` re-emits it verbatim), but
        // nothing declares it there, so it is an elided lifetime in a position
        // that forbids elision (`E0726`). A concrete `'static` argument needs
        // no declaration and is preserved in the self type.
        //
        // This applies to every hoisting shape, not just a receiver:
        let fn_lifetimes = generics::lifetime_args(&self.generics);
        for gen_arg in gen_args.args.iter() {
            if let syn::GenericArgument::Lifetime(lt) = gen_arg {
                if lt.ident != "static" && !fn_lifetimes.contains(&lt) {
                    bail_span!(
                        lt,
                        "wasm-bindgen does not support a lifetime argument on the class type \
                         that the function does not itself declare; name it as a lifetime \
                         parameter of the function (e.g. `fn f<'a, T>(this: &'a Holder<'a, T>)`)"
                    );
                }
            }
            if let syn::GenericArgument::Type(ty) = gen_arg {
                if class_argument_has_elided_lifetime(ty) {
                    bail_span!(
                        ty,
                        "wasm-bindgen does not support elided lifetimes in class type arguments; name the lifetime as a function parameter"
                    );
                }
                if class_argument_has_inferred_type(ty) {
                    bail_span!(
                        ty,
                        "wasm-bindgen does not support inferred (`_`) class type arguments; use a concrete type or named function type parameter"
                    );
                }
                // `impl Trait` desugars to an anonymous parameter of the
                // function, which the generated `impl` header cannot name.
                if class_argument_has_impl_trait(ty) {
                    bail_span!(
                        ty,
                        "wasm-bindgen does not support `impl Trait` in class type arguments; name it as a function type parameter (e.g. `fn f<T: Trait>(this: &Holder<T>)`)"
                    );
                }
            }
        }

        // A class type parameterised by *both* a lifetime and a type parameter
        // can be hoisted safely: the reference-conversion impls (`&T`'s
        // `IntoWasmAbi`/`OptionIntoWasmAbi`) declare the type's own lifetime
        // params separately from the fresh reference lifetime, so the class's own lifetime
        // is never forced to unify with — and therefore never forced to
        // outlive — the borrow of `&self`. Without that unification there is
        // no `E0521` to guard against here.

        // An argument list that hoists nothing is fine either way: a fully
        // concrete one (`&Holder<u32>`) is re-emitted verbatim by
        // `class_impl_def`, so the wrapper still lands on `impl Holder<u32>`
        // rather than on the class's own parameter defaults.

        // A constructor or inferred self-returning static method can decline
        // to hoist a non-constraining return and bind the class defaults instead.
        // In those cases the class comes from the return type, where stripping the
        // arguments and binding the class's own defaults (`impl Promise` for a
        // `-> Promise<T::Resolution>`) is the established, working behaviour
        // shared with the type-erasure path: `class_return_path` already
        // declines to hoist a non-constraining argument list, so the arguments
        // never reach `class_impl_def` at all and the resulting inherent method
        // is perfectly valid — it simply hangs off the defaulted class.
        // Rejecting it here would break real imports, e.g.
        // `js_sys::Promise::new_typed<T: Promising>(..) -> Promise<<T as
        // Promising>::Resolution>`.
        if !require_constraining_arguments {
            return Ok(());
        }

        // A type argument that mentions a parameter without *constraining* it —
        // `&Holder<T::Assoc>` — hoists `T` onto the `impl` header without the
        // self type ever determining it, which is `E0207: the type parameter
        // `T` is not constrained`. `class_return_path` already applies this
        // check for the constructor/self-returning-static shape; a method
        // receiver needs it too.
        let fn_params: Vec<&Ident> = generics::generic_params(&self.generics)
            .iter()
            .map(|p| p.0)
            .collect();
        if !generics::args_are_constraining_for(&gen_args.args, &fn_params) {
            bail_span!(
                self.rust_name,
                "wasm-bindgen requires each generic argument of an impl class type to be either \
                 concrete or to determine a generic parameter of the function (e.g. \
                 `&Holder<u32>`, `&Holder<T>`, `&Holder<Option<T>>`); an argument like \
                 `&Holder<T::Assoc>` mentions `T` without determining it, so the generated `impl` \
                 block cannot constrain it"
            );
        }

        Ok(())
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
            ty: class_ty, kind, ..
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
            attrs: _,
            qself: None,
            ref path,
        }) = get_ty(ret_ty)
        else {
            return None;
        };

        let seg = path.segments.last()?;
        if !is_constructor {
            let syn::Type::Path(syn::TypePath {
                attrs: _,
                qself: None,
                path: class_path,
            }) = get_ty(class_ty)
            else {
                return None;
            };
            if !same_class_path(path, class_path) {
                return None;
            }
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

    fn validate_unhoisted_class_return_lifetimes(&self) -> Result<(), Diagnostic> {
        let ast::ImportFunctionKind::Method {
            ty: class_ty, kind, ..
        } = &self.kind
        else {
            return Ok(());
        };
        let Some(ret_ty) = &self.js_ret else {
            return Ok(());
        };
        let syn::Type::Path(syn::TypePath {
            attrs: _,
            qself: None,
            path,
        }) = get_ty(ret_ty)
        else {
            return Ok(());
        };
        let Some(segment) = path.segments.last() else {
            return Ok(());
        };
        let syn::Type::Path(syn::TypePath {
            attrs: _,
            qself: None,
            path: class_path,
        }) = get_ty(class_ty)
        else {
            return Ok(());
        };
        if !matches!(kind, ast::MethodKind::Constructor) && !same_class_path(path, class_path) {
            return Ok(());
        }
        let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
            return Ok(());
        };
        if arguments
            .args
            .iter()
            .any(|argument| matches!(argument, syn::GenericArgument::Lifetime(_)))
        {
            bail_span!(
                ret_ty,
                "experimental_generic_mono cannot use a constructor or self-returning static method whose class return mixes a lifetime argument with a non-constraining type argument; use a fully constraining return class or the type-erasure generic path"
            );
        }
        Ok(())
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
        // Per-monomorphisation generic imports describe their real signatures
        // elsewhere: each monomorphisation emits its own `(key, signature)`
        // descriptor from inside `breaks_if_inlined`, terminated by the
        // `__wbindgen_describe_generic_import` marker. So there is nothing
        // useful to say here.
        //
        // We nevertheless emit a descriptor, because the *export* has a job to
        // do that has nothing to do with its contents: it anchors this crate's
        // `#[link_section = "__wasm_bindgen_unstable"]` static into the link.
        // That static lives in the same object file, and wasm-ld only pulls an
        // archive member out of an rlib if something in the link references one
        // of its symbols. The monomorphised shim is instantiated in the
        // *downstream* crate's CGU, so it does not reference anything here — an
        // upstream `extern "C"` block containing only `experimental_generic_mono`
        // imports would leave no referenced symbol at all, the member would
        // never be pulled, and the crate's AST metadata would silently go
        // missing. The CLI then fails with "generic import monomorphisation
        // references unknown shim". A `#[no_mangle]` export is always kept, so
        // emitting one forces the member in.
        //
        // The body is deliberately meaningless: the trivial zero-argument,
        // unit-returning function shape, written with the same two
        // `describe_ret` calls the real path uses so `Function` can decode
        // `arguments`, `ret` and `inner_ret`. Nothing consumes it —
        // `import_function` in cli-support drops the entry, and
        // `execute_exports` deletes the export from the module after
        // interpreting it, so none of this reaches the output.
        //
        // Going through `Descriptor` rather than hand-rolling the export gets
        // us the `#[no_mangle]` naming, the wasm-only `#[cfg]` gate, and the
        // `DESCRIPTORS_EMITTED` dedup for free.
        if f.generic_per_mono {
            let mut attrs = f.function.rust_attrs.clone();
            attrs.extend(self.class_cfg_attrs.iter().cloned());
            Descriptor {
                ident: &f.shim,
                inner: quote! {
                    inform(FUNCTION);
                    inform(0);
                    inform(0);
                    <() as WasmDescribe>::describe();
                    <() as WasmDescribe>::describe();
                },
                attrs,
                wasm_bindgen: self.wasm_bindgen,
            }
            .to_tokens(tokens);
            return Ok(());
        }
        let fn_class_generics = f.get_fn_generics()?;
        let fn_lifetime_params = generics::lifetime_args(&f.generics);
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
                // Must match the ABI rewrite in `ImportFunction::try_to_tokens`;
                // both go through the same helper. Non-slice args under a fn- or
                // block-level `slice_to_array` fall through to their default
                // describe.
                if arg.slice_to_array {
                    if let Some(describe_ty) = slice_to_array_describe_ty(self.wasm_bindgen, &ty) {
                        return Ok(describe_ty);
                    }
                }
                Ok(ty)
            })
            .collect::<Result<Vec<syn::Type>, Diagnostic>>()?;
        let nargs = f.function.arguments.len() as u32;
        // Concretising the return type is only needed when it is what actually gets
        // described. An `async` import always describes the `Promise` handle
        // instead, so its resolved type never reaches the descriptor — see
        // `import_describe_ret`, which both import paths share precisely so this
        // rule cannot drift between them. A `suspending` import likewise receives
        // the settled value as a raw externref return (the shim hands the Promise
        // to `WebAssembly.Suspending` and the conversion happens in Rust
        // post-resume), so it describes an externref too.
        let ret_is_externref =
            f.function.r#async || (f.suspending && (f.js_ret.is_some() || f.catch));
        let concrete_ret = match (&f.js_ret, ret_is_externref) {
            (Some(t), false) => Some(generics::generic_to_concrete(
                t.clone(),
                &fn_class_generics.concrete_defaults,
                &fn_lifetime_params,
            )?),
            _ => None,
        };
        let inform_ret =
            import_describe_ret(self.wasm_bindgen, concrete_ret.as_ref(), ret_is_externref);

        let mut attrs = f.function.rust_attrs.clone();
        attrs.extend(self.class_cfg_attrs.iter().cloned());
        Descriptor {
            ident: &f.shim,
            inner: quote! {
                inform(FUNCTION);
                inform(0);
                inform(#nargs);
                #(<#argtys as WasmDescribe>::describe();)*
                #inform_ret
                #inform_ret
            },
            attrs,
            wasm_bindgen: self.wasm_bindgen,
        }
        .to_tokens(tokens);
        Ok(())
    }
}

impl ToTokens for ast::Enum {
    fn to_tokens(&self, into: &mut TokenStream) {
        let enum_name = &self.rust_name;
        let name_str = shared::qualified_name(self.js_namespace.as_deref(), &self.js_name);
        let name_len = name_str.chars().count() as u32;
        let name_chars = name_str.chars().map(|c| c as u32);
        let unique_crate_identifier = crate::hash::unique_crate_identifier();
        let unique_crate_identifier_len = unique_crate_identifier.chars().count() as u32;
        let unique_crate_identifier_chars = unique_crate_identifier.chars().map(|c| c as u32);
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
                fn describe() {
                    use #wasm_bindgen::describe::*;
                    inform(ENUM);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                    inform(#hole);
                    inform(#unique_crate_identifier_len);
                    #(inform(#unique_crate_identifier_chars);)*
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

        Descriptor {
            ident: &self.shim,
            inner: quote! {
                <#ty as WasmDescribe>::describe();
            },
            attrs: vec![],
            wasm_bindgen: &self.wasm_bindgen,
        }
        .to_tokens(into);
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

/// Emits the necessary glue tokens for "descriptor", generating an appropriate
/// symbol name as well as attributes around the descriptor function itself.
struct Descriptor<'a, T> {
    ident: &'a Ident,
    inner: T,
    attrs: Vec<syn::Attribute>,
    wasm_bindgen: &'a syn::Path,
}

impl<T: ToTokens> ToTokens for Descriptor<'_, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // It's possible for the same descriptor to be emitted in two different
        // modules (aka a value imported twice in a crate, each in a separate
        // module). In this case no need to emit duplicate descriptors (which
        // leads to duplicate symbol errors), instead just emit one.
        //
        // It's up to the descriptors themselves to ensure they have unique
        // names for unique items imported, currently done via `ShortHash` and
        // hashing appropriate data into the symbol name.
        thread_local! {
            static DESCRIPTORS_EMITTED: RefCell<HashSet<String>> = RefCell::default();
        }

        let ident = self.ident;

        if !DESCRIPTORS_EMITTED.with(|list| list.borrow_mut().insert(ident.to_string())) {
            return;
        }

        let name = Ident::new(&format!("__wbindgen_describe_{ident}"), ident.span());
        let inner = &self.inner;
        let attrs = &self.attrs;
        let wasm_bindgen = &self.wasm_bindgen;
        (quote! {
            #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
            #[automatically_derived]
            const _: () = {
                #wasm_bindgen::__wbindgen_coverage! {
                #(#attrs)*
                #[no_mangle]
                #[doc(hidden)]
                pub extern "C-unwind" fn #name() {
                    use #wasm_bindgen::describe::*;
                    // See definition of `link_mem_intrinsics` for what this is doing
                    #wasm_bindgen::__rt::link_mem_intrinsics();
                    #inner
                }
                }
            };
        })
        .to_tokens(tokens);
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
///
/// `span` is where a failure to resolve `<abi as WasmAbi>` should be reported.
/// That matters because the well-formedness obligation for `abi` itself (for an
/// import argument, `<#ty as IntoWasmAbi>::Abi`) is attributed to this outer
/// projection, so leaving it at `Span::call_site()` sends the error to the
/// `#[wasm_bindgen]` attribute instead of the argument. Pass
/// `Span::call_site()` to keep the previous behaviour.
fn splat(
    wasm_bindgen: &syn::Path,
    name: &Ident,
    abi: &TokenStream,
    span: Span,
) -> (Vec<TokenStream>, Vec<Ident>) {
    let mut args = Vec::new();
    let mut names = Vec::new();

    for n in 1_u32..=4 {
        let arg_name = format_ident!("{}_{}", name, n, span = span);
        let prim_name = format_ident!("Prim{}", n);
        args.push(quote_spanned! { span =>
            #arg_name: <#abi as #wasm_bindgen::convert::WasmAbi>::#prim_name
        });
        names.push(arg_name);
    }

    (args, names)
}

/// Like [`respan`], but rewrites *every* token in the tree, including the
/// contents of nested groups.
///
/// [`respan`] only touches the top-level tokens, which is enough to relocate a
/// whole item but leaves everything inside its parentheses and braces at
/// whatever span `quote!` gave it — normally `Span::call_site()`, i.e. the
/// `#[wasm_bindgen]` attribute. That is exactly where the interesting errors
/// (unresolvable `IntoWasmAbi`/`FromWasmAbi` projections in a generated shim
/// signature) come from.
fn respan_all(input: TokenStream, span: Span) -> TokenStream {
    input
        .into_iter()
        .map(|mut token| {
            if let proc_macro2::TokenTree::Group(g) = &token {
                let mut new = proc_macro2::Group::new(g.delimiter(), respan_all(g.stream(), span));
                new.set_span(span);
                return proc_macro2::TokenTree::Group(new);
            }
            token.set_span(span);
            token
        })
        .collect()
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

/// Emits the `WasmDescribe::describe()` call that states what an imported
/// function actually returns *across the ABI*.
///
/// `ret_ty` is the import's declared return type, already concretised, or `None`
/// for a unit return. `is_async` selects the promise shape.
///
/// The subtle case is `async`. An `async` import hands back a `Promise` handle —
/// an externref — no matter what it resolves to; the resolved value is converted
/// separately, inside `JsFuture<T>`. So the descriptor has to say externref.
/// Describing the *resolved* type instead makes cli-support marshal the promise
/// handle as if it were a `T`, which silently produces garbage for every `T` that
/// is not itself handle-shaped (`async fn f() -> u32` being the obvious case).
/// That this only affects non-handle types is why it went unnoticed: the existing
/// async-import tests all resolve to `JsValue`/`JsString`.
///
/// Both import codegen paths must agree here — [`DescribeImport`] for ordinary
/// imports and [`ImportFunction::try_to_tokens_generic`] for `experimental_generic_mono`
/// ones. They used to carry separate copies of this logic with a "keep the two in
/// sync" comment; this helper is what makes that drift impossible.
fn import_describe_ret(
    wasm_bindgen: &syn::Path,
    ret_ty: Option<&syn::Type>,
    is_async: bool,
) -> TokenStream {
    let describe = quote! { #wasm_bindgen::describe::WasmDescribe };
    if is_async {
        // The `Promise` handle is what crosses, not the resolved value.
        return quote! { <#wasm_bindgen::JsValue as #describe>::describe(); };
    }
    match ret_ty {
        Some(ty) => quote! { <#ty as #describe>::describe(); },
        None => quote! { <() as #describe>::describe(); },
    }
}

fn get_ty(mut ty: &syn::Type) -> &syn::Type {
    while let syn::Type::Group(g) = ty {
        ty = &g.elem;
    }
    ty
}

/// A slice-shaped argument recognised by the `slice_to_array` codegen.
pub(crate) struct SliceArg {
    /// The slice's element type `T`.
    pub elem_ty: syn::Type,
    /// Whether the slice was wrapped in an outer `Option`.
    pub is_option: bool,
    /// Whether the reference was `&mut` rather than `&`.
    ///
    /// `slice_to_array` cannot honour this: it hands JS an owned `Array`, and
    /// mutations to that `Array` are not written back into the caller's slice.
    /// The parser rejects the combination rather than silently dropping the
    /// write-back; see `check_slice_to_array_arg` in `parser.rs`.
    pub is_mut: bool,
}

/// Recognise `&[T]`, `&mut [T]`, and either wrapped in `Option`. Used by the
/// `slice_to_array` codegen to rewrite the ABI path, and by the parser to
/// reject the mutable and generic-element forms.
pub(crate) fn detect_slice_or_option_slice(ty: &syn::Type) -> Option<SliceArg> {
    // Direct `&[T]` / `&mut [T]`.
    if let syn::Type::Reference(syn::TypeReference {
        elem, mutability, ..
    }) = ty
    {
        if let syn::Type::Slice(syn::TypeSlice { elem: inner, .. }) = &**elem {
            return Some(SliceArg {
                elem_ty: (**inner).clone(),
                is_option: false,
                is_mut: mutability.is_some(),
            });
        }
    }
    // `Option<&[T]>` — match shape `Option<...>` and recurse once.
    if let syn::Type::Path(syn::TypePath {
        attrs: _,
        qself: None,
        path,
    }) = ty
    {
        if let Some(seg) = path.segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if args.args.len() == 1 {
                        if let syn::GenericArgument::Type(inner) = &args.args[0] {
                            if let Some(inner) = detect_slice_or_option_slice(inner) {
                                if !inner.is_option {
                                    return Some(SliceArg {
                                        is_option: true,
                                        ..inner
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// The error type `E` of a `Result<T, E>` return type, if `ty` looks like a
/// `Result` with two type arguments.
///
/// Best-effort and purely syntactic, like the rest of the `catch` handling: a
/// `Result` alias or a re-ordered alias will not be recognised, which only means
/// a diagnostic is skipped.
fn result_err_ty(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(syn::TypePath {
        attrs: _,
        qself: None,
        path,
    }) = get_ty(ty)
    else {
        return None;
    };
    let syn::PathArguments::AngleBracketed(args) = &path.segments.last()?.arguments else {
        return None;
    };
    if args.args.len() != 2 {
        return None;
    }
    match &args.args[1] {
        syn::GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

/// The type to hand `WasmDescribe::describe` for a `slice_to_array` argument,
/// or `None` if `ty` is not slice-shaped.
///
/// Describing through `&Vec<T>` / `Option<&Vec<T>>` makes the descriptor
/// `Ref(Vector(T))` / `Option(Ref(Vector(T)))`, which is what cli-support
/// recognises as "hand JS an owned `Array`" rather than "hand JS a typed-array
/// view". This must stay in lockstep with the ABI rewrite in
/// [`slice_to_array_rewrite`]: the descriptor is what selects the JS shim, and
/// the ABI is what the shim is handed.
fn slice_to_array_describe_ty(wasm_bindgen: &syn::Path, ty: &syn::Type) -> Option<syn::Type> {
    let SliceArg {
        elem_ty, is_option, ..
    } = detect_slice_or_option_slice(ty)?;
    // `alloc`, not `std`: `wasm-bindgen` is `#![no_std]` and supports `no_std`
    // consumers, so generated code must never name `::std`.
    let vec = quote! { #wasm_bindgen::__rt::alloc::vec::Vec };
    // `Option` likewise goes through the `__rt::core` re-export rather than a
    // bare `::core`, for the same call-site-hygiene reason as `Vec` above.
    let option = quote! { #wasm_bindgen::__rt::core::option::Option };
    Some(if is_option {
        parse_quote! { #option<&#vec<#elem_ty>> }
    } else {
        parse_quote! { &#vec<#elem_ty> }
    })
}

/// The pieces of the `slice_to_array` rewrite for one argument.
struct SliceToArrayRewrite {
    /// Flattened wasm ABI parameters to splice into the shim signature.
    abi_args: Vec<TokenStream>,
    /// The names `conversion` binds, in ABI order.
    prim_names: Vec<Ident>,
    /// Statements converting the user-facing argument into `prim_names`.
    conversion: TokenStream,
    /// The type to describe, per [`slice_to_array_describe_ty`].
    describe_ty: syn::Type,
}

/// Build the `slice_to_array` rewrite for one argument, or `None` if `ty` is not
/// slice-shaped (`&[T]` / `Option<&[T]>`).
///
/// This re-routes the argument through `<T as VectorRefIntoWasmAbi>` instead of
/// the default `&[T]: IntoWasmAbi`. The user-facing parameter is unchanged; only
/// the ABI and describe paths move. `VectorRefIntoWasmAbi`'s impls cover the two
/// genuine ABI shapes (zero-copy borrow for primitive elements, freshly
/// allocated `Box<[u32]>` for handle-shaped ones), so no `T: Clone` bound is
/// introduced. The wire format is `WasmSlice` either way; cli-support picks the
/// right JS shim from the element `VectorKind` in the descriptor.
///
/// `slice_to_array` is set per-fn or per-`extern "C"` block and applies to every
/// slice-shaped argument of every fn it covers. Arguments that are not
/// slice-shaped (the `this: &Foo` receiver of a method, a `Vec<T>`, any scalar)
/// return `None` and take the default ABI path — there is no per-argument
/// opt-out in Rust attribute syntax to require, so a silent no-op is the only
/// sensible behaviour.
///
/// Neither `&mut [T]` nor a generic element type reaches here: the parser
/// rejects both up front (see `check_slice_to_array_arg` in `parser.rs`).
fn slice_to_array_rewrite(
    wasm_bindgen: &syn::Path,
    name: &Ident,
    var: &TokenStream,
    ty: &syn::Type,
) -> Option<SliceToArrayRewrite> {
    let SliceArg {
        elem_ty, is_option, ..
    } = detect_slice_or_option_slice(ty)?;
    let describe_ty = slice_to_array_describe_ty(wasm_bindgen, ty)?;

    let abi = quote! { #wasm_bindgen::convert::WasmSlice };
    let (abi_args, prim_names) = splat(wasm_bindgen, name, &abi, Span::call_site());

    let body = if is_option {
        quote! {
            match #var {
                #wasm_bindgen::__rt::core::option::Option::Some(s) =>
                    <#elem_ty as #wasm_bindgen::convert::VectorRefIntoWasmAbi>
                        ::slice_into_abi(s),
                #wasm_bindgen::__rt::core::option::Option::None =>
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
    let conversion = quote! {
        let #name: #wasm_bindgen::convert::WasmSlice = #body;
        let (#(#prim_names),*) =
            <#wasm_bindgen::convert::WasmSlice as #wasm_bindgen::convert::WasmAbi>
                ::split(#name);
    };

    Some(SliceToArrayRewrite {
        abi_args,
        prim_names,
        conversion,
        describe_ty,
    })
}

/// Detects whether a type is a raw `&dyn Fn(...)` or `&mut dyn FnMut(...)` argument.
///
/// Returns `Some((is_mut, fn_trait_bounds))` where:
/// - `is_mut` is `true` for `&mut dyn FnMut`, `false` for `&dyn Fn`
/// - `fn_trait_bounds` are the `TypeParamBound`s from the `dyn` trait object (e.g. `FnMut(A)->R`)
///
/// This is used by the import function codegen to auto-inject `MaybeUnwindSafe`
/// bounds for closure arguments, ensuring unwind safety when `panic = "unwind"`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_replacement_does_not_recurse_into_its_arguments() {
        let mut predicate: syn::WherePredicate = syn::parse_quote!(A::Assoc: Trait<B>);
        let type_replacements = BTreeMap::from([
            (syn::parse_quote!(A), syn::parse_quote!(Projector<B>)),
            (syn::parse_quote!(B), syn::parse_quote!(T)),
        ]);

        assert!(substitute_class_predicate(
            &mut predicate,
            &type_replacements,
            &BTreeMap::new(),
        ));
        assert_eq!(
            predicate.to_token_stream().to_string(),
            "Projector < B > :: Assoc : Trait < T >"
        );
    }

    #[test]
    fn qualified_self_replacement_is_applied_once() {
        let mut predicate: syn::WherePredicate = syn::parse_quote!(<A as Trait>::Item: Bound<B>);
        let type_replacements = BTreeMap::from([
            (syn::parse_quote!(A), syn::parse_quote!(B)),
            (syn::parse_quote!(B), syn::parse_quote!(A)),
        ]);

        assert!(substitute_class_predicate(
            &mut predicate,
            &type_replacements,
            &BTreeMap::new(),
        ));
        assert_eq!(
            predicate.to_token_stream().to_string(),
            "< B as Trait > :: Item : Bound < A >"
        );
    }
}
