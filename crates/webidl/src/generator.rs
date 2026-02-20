use crate::util::{camel_case_ident, leading_colon_path_ty, raw_ident, rust_ident, shared_ref};
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::collections::BTreeSet;
use syn::{Ident, Type};

use crate::constants::{BUILTIN_IDENTS, POLYFILL_INTERFACES};
use crate::traverse::TraverseType;
use crate::util::{get_cfg_features, mdn_doc, required_doc_string};
use crate::wbg_type::WbgType;
use crate::Options;

fn add_features(features: &mut BTreeSet<String>, ty: &impl TraverseType) {
    ty.traverse_type(&mut |ident| {
        let ident = ident.to_string();

        if !BUILTIN_IDENTS.contains(ident.as_str()) {
            features.insert(ident);
        }
    });
}

fn get_features_doc(options: &Options, name: String) -> Option<String> {
    let mut features = BTreeSet::new();
    features.insert(name);
    required_doc_string(options, &features)
}

fn comment(mut comment: String, features: &Option<String>) -> TokenStream {
    if let Some(s) = features {
        comment.push_str(s);
    }

    let lines = comment.lines().map(|doc| quote!( #[doc = #doc] ));

    quote! {
        #(#lines)*
    }
}

fn maybe_unstable_attr(unstable: bool) -> Option<proc_macro2::TokenStream> {
    if unstable {
        Some(quote! {
            #[cfg(web_sys_unstable_apis)]
        })
    } else {
        None
    }
}

fn maybe_unstable_docs(unstable: bool) -> Option<proc_macro2::TokenStream> {
    if unstable {
        Some(quote! {
            #[doc = ""]
            #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
            #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
        })
    } else {
        None
    }
}

fn generate_arguments(
    arguments: &[(Ident, WbgType<'_>)],
    variadic: bool,
    variadic_type: Option<&WbgType<'_>>,
    generics_compat: bool,
) -> Option<Vec<TokenStream>> {
    let mut output = Vec::with_capacity(arguments.len());
    for (i, (name, wbg_ty)) in arguments.iter().enumerate() {
        if variadic && i + 1 == arguments.len() {
            // In next_unstable mode (generics_compat=false), use typed slice &[T]
            if !generics_compat {
                if let Some(vt) = variadic_type {
                    // For slice elements:
                    // - Primitives (i16, f32, etc.) use Rust types directly (efficient IntoWasmAbi)
                    // - Non-primitives (Object, interfaces) use JS types without & prefix
                    let elem_ty = if vt.is_slice_primitive() {
                        // Use top-level argument conversion for primitives (gives i16, f32, etc.)
                        vt.to_syn_type(crate::util::TypePosition::ARGUMENT, false, false)
                    } else {
                        // Use inner conversion for non-primitives (gives Object, etc. without &)
                        vt.to_syn_type(crate::util::TypePosition::ARGUMENT.to_inner(), false, false)
                    };
                    if let Ok(Some(elem_ty)) = elem_ty {
                        output.push(quote!( #name: &[#elem_ty] ));
                        continue;
                    }
                }
            }
            // Fallback to untyped Array
            output.push(quote!( #name: &::js_sys::Array ));
        } else {
            let ty = match wbg_ty.to_syn_type(
                crate::util::TypePosition::ARGUMENT,
                false,
                generics_compat,
            ) {
                Ok(Some(ty)) => ty,
                Ok(None) => {
                    if !generics_compat {
                        log::warn!(
                            "generate_arguments: arg {name} returned None, wbg_ty={wbg_ty:?}"
                        );
                    }
                    return None;
                }
                Err(e) => {
                    if !generics_compat {
                        log::warn!(
                            "generate_arguments: arg {name} failed: {e:?}, wbg_ty={wbg_ty:?}"
                        );
                    }
                    return None;
                }
            };
            output.push(quote!( #name: #ty ));
        }
    }
    Some(output)
}

fn generate_variadic(variadic: bool) -> Option<TokenStream> {
    if variadic {
        Some(quote!(variadic,))
    } else {
        None
    }
}

pub struct EnumVariant {
    pub name: Ident,
    pub value: String,
}

impl EnumVariant {
    fn generate(&self) -> TokenStream {
        let EnumVariant { name, value } = self;

        quote!( #name = #value )
    }
}

pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    pub unstable: bool,
}

impl Enum {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Enum {
            name,
            variants,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let doc_comment = comment(
            format!("The `{name}` enum."),
            &get_features_doc(options, name.to_string()),
        );

        let variants = variants
            .iter()
            .map(|variant| variant.generate())
            .collect::<Vec<_>>();

        quote! {
            #![allow(unused_imports)]
            #![allow(clippy::all)]
            use wasm_bindgen::prelude::*;

            #unstable_attr
            #[wasm_bindgen]
            #doc_comment
            #unstable_docs
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum #name {
                #(#variants),*
            }
        }
    }
}

pub enum ConstValue {
    Boolean(bool),
    Float(f64),
    SignedInteger(i64),
    UnsignedInteger(u64),
}

impl ConstValue {
    fn generate(&self) -> TokenStream {
        use ConstValue::*;

        match self {
            Boolean(false) => quote!(false),
            Boolean(true) => quote!(true),
            // the actual type is unknown because of typedefs
            // so we cannot use std::fxx::INFINITY
            // but we can use type inference
            Float(f) if f.is_infinite() && f.is_sign_positive() => quote!(1.0 / 0.0),
            Float(f) if f.is_infinite() && f.is_sign_negative() => quote!(-1.0 / 0.0),
            Float(f) if f.is_nan() => quote!(0.0 / 0.0),
            // again no suffix
            // panics on +-inf, nan
            Float(f) => {
                let f = Literal::f64_suffixed(*f);
                quote!(#f)
            }
            SignedInteger(i) => {
                let i = Literal::i64_suffixed(*i);
                quote!(#i)
            }
            UnsignedInteger(i) => {
                let i = Literal::u64_suffixed(*i);
                quote!(#i)
            }
        }
    }
}

pub struct Const {
    pub name: Ident,
    pub js_name: String,
    pub ty: syn::Type,
    pub value: ConstValue,
    pub unstable: bool,
}

impl Const {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: &str,
        deprecated: &Option<Option<String>>,
    ) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        let js_name = &self.js_name;
        let value = self.value.generate();
        let unstable = self.unstable;

        let unstable_attr = maybe_unstable_attr(unstable);
        let unstable_docs = maybe_unstable_docs(unstable);

        let doc_comment = comment(
            format!("The `{parent_js_name}.{js_name}` const."),
            &get_features_doc(options, parent_name.to_string()),
        );

        let deprecated = deprecated.as_ref().map(|msg| match msg {
            Some(msg) => quote!( #[deprecated(note = #msg)] ),
            None => quote!( #[deprecated] ),
        });

        quote! {
            #unstable_attr
            #doc_comment
            #unstable_docs
            #deprecated
            pub const #name: #ty = #value as #ty;
        }
    }
}

pub enum InterfaceAttributeKind {
    Getter,
    Setter,
}

pub struct InterfaceAttribute {
    pub js_name: String,
    pub rust_name: String,
    pub deprecated: Option<Option<String>>,
    pub ty: Type,
    pub is_static: bool,
    pub structural: bool,
    pub catch: bool,
    pub kind: InterfaceAttributeKind,
    pub unstable: bool,
    /// True if this is a stable attribute that has an unstable override with
    /// the same name but different type. When true, this attribute is gated
    /// behind `#[cfg(not(web_sys_unstable_apis))]`.
    pub has_unstable_override: bool,
}

impl InterfaceAttribute {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: &str,
        parents: &[Ident],
        parent_deprecated: &Option<Option<String>>,
    ) -> TokenStream {
        let InterfaceAttribute {
            js_name,
            rust_name,
            deprecated,
            ty,
            is_static,
            structural,
            catch,
            kind,
            unstable,
            has_unstable_override,
        } = self;

        // If this is a stable attribute that has an unstable override,
        // gate it behind `not(web_sys_unstable_apis)` so it's excluded
        // when the unstable version is enabled.
        let unstable_attr = if *has_unstable_override {
            Some(quote! {
                #[cfg(not(web_sys_unstable_apis))]
            })
        } else {
            maybe_unstable_attr(*unstable)
        };
        let unstable_docs = if *has_unstable_override {
            None
        } else {
            maybe_unstable_docs(*unstable)
        };

        let mdn_docs = mdn_doc(parent_js_name, Some(js_name));

        let mut features = BTreeSet::new();

        add_features(&mut features, ty);

        for parent in parents {
            features.remove(&parent.to_string());
        }

        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name.to_string());

        let doc_comment = required_doc_string(options, &features);

        let structural = if *structural {
            quote!(structural,)
        } else {
            quote!(final,)
        };

        let (method, this) = if *is_static {
            (quote!( static_method_of = #parent_name, ), None)
        } else {
            (quote!(method,), Some(quote!( this: &#parent_name, )))
        };

        let (prefix, attr, def) = match kind {
            InterfaceAttributeKind::Getter => {
                let rust_name = rust_ident(rust_name);

                let ty = if *catch {
                    quote!( Result<#ty, JsValue> )
                } else {
                    quote!( #ty )
                };

                (
                    "Getter",
                    quote!(getter,),
                    quote!( pub fn #rust_name(#this) -> #ty; ),
                )
            }

            InterfaceAttributeKind::Setter => {
                let rust_name = rust_ident(rust_name);

                let ret_ty = if *catch {
                    Some(quote!( -> Result<(), JsValue> ))
                } else {
                    None
                };

                (
                    "Setter",
                    quote!(setter,),
                    quote!( pub fn #rust_name(#this value: #ty) #ret_ty; ),
                )
            }
        };

        let catch = if *catch { Some(quote!(catch,)) } else { None };
        let deprecated = deprecated
            .as_ref()
            .or(parent_deprecated.as_ref())
            .map(|msg| match msg {
                Some(msg) => quote!( #[deprecated(note = #msg)] ),
                None => quote!( #[deprecated] ),
            });

        let doc_comment = comment(
            format!("{prefix} for the `{js_name}` field of this object.\n\n{mdn_docs}"),
            &doc_comment,
        );

        let js_name = raw_ident(js_name);

        quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #structural
                #catch
                #method
                #attr
                js_class = #parent_js_name,
                js_name = #js_name
            )]
            #doc_comment
            #unstable_docs
            #deprecated
            #def
        }
    }
}

#[derive(Debug, Clone)]
pub enum InterfaceMethodKind {
    Constructor(Option<String>),
    Regular,
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
}

#[derive(Clone)]
pub struct InterfaceMethod<'a> {
    pub name: Ident,
    pub js_name: String,
    pub deprecated: Option<Option<String>>,
    pub arguments: Vec<(Ident, WbgType<'a>)>,
    pub variadic_type: Option<WbgType<'a>>,
    pub ret_wbg_ty: Option<WbgType<'a>>,
    pub kind: InterfaceMethodKind,
    pub is_static: bool,
    pub structural: bool,
    pub catch: bool,
    pub variadic: bool,
    pub unstable: bool,
    /// True if this is a stable method that has an unstable override with
    /// the same name/signature but different return type. When true, this method
    /// is gated behind `#[cfg(not(web_sys_unstable_apis))]`.
    pub has_unstable_override: bool,
}

impl<'a> InterfaceMethod<'a> {
    /// Returns true if this method has the same effective signature as `other`.
    ///
    /// Two methods have the same signature when they would produce the same Rust
    /// binding: same JS name, same argument types, same return type, and same
    /// throws behavior. Used for both deduplication (across operations) and
    /// merge detection (stable/unstable gating).
    pub fn same_signature(&self, other: &InterfaceMethod<'_>) -> bool {
        self.js_name == other.js_name
            && self.variadic == other.variadic
            && self.variadic_type == other.variadic_type
            && self.ret_wbg_ty == other.ret_wbg_ty
            && self.catch == other.catch
            && self
                .arguments
                .iter()
                .map(|(_, wbg_ty)| wbg_ty)
                .eq(other.arguments.iter().map(|(_, wbg_ty)| wbg_ty))
    }
}

impl InterfaceMethod<'_> {
    pub(crate) fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: String,
        parents: &[Ident],
        parent_deprecated: &Option<Option<String>>,
    ) -> Option<TokenStream> {
        let InterfaceMethod {
            name,
            js_name,
            deprecated,
            arguments,
            variadic_type,
            ret_wbg_ty,
            kind,
            is_static,
            structural,
            catch,
            variadic,
            unstable,
            has_unstable_override,
        } = self;

        // If this is a stable method that has an unstable override,
        // gate it behind `not(web_sys_unstable_apis)` so it's excluded
        // when the unstable version is enabled.
        let unstable_attr = if *has_unstable_override {
            Some(quote! {
                #[cfg(not(web_sys_unstable_apis))]
            })
        } else {
            maybe_unstable_attr(*unstable)
        };
        let unstable_docs = if *has_unstable_override {
            None
        } else {
            maybe_unstable_docs(*unstable)
        };
        // Unstable APIs always use typed generics (generics_compat=false).
        // Stable APIs use legacy types by default, typed generics if next_unstable is set.
        let generics_compat = if *unstable {
            false
        } else {
            !options.next_unstable.get()
        };

        // Convert WbgType to syn::Type during code generation
        use crate::util::TypePosition;
        let ret_ty = match ret_wbg_ty {
            Some(wbg_ty) => {
                match wbg_ty.to_syn_type(TypePosition::RETURN, false, generics_compat) {
                    Ok(ty) => ty,
                    Err(e) => {
                        log::warn!("SKIP {name} on {parent_name}: ret type failed: {e:?}");
                        return None;
                    }
                }
            }
            None => None,
        };

        let mut is_constructor = false;

        let mut extra_args = vec![quote!( js_class = #parent_js_name )];

        let doc_comment = match kind {
            InterfaceMethodKind::Constructor(name) => {
                is_constructor = true;
                if let Some(name) = name {
                    extra_args[0] = quote!( js_class = #name );
                }
                format!(
                    "The `new {parent_name}(..)` constructor, creating a new \
                     instance of `{parent_name}`.\n\n{}",
                    mdn_doc(&parent_js_name, Some(&parent_js_name))
                )
            }
            InterfaceMethodKind::Regular => {
                {
                    let js_name = raw_ident(js_name);
                    extra_args.push(quote!( js_name = #js_name ));
                }
                let method = if *is_static {
                    &format!("{js_name}_static")
                } else {
                    js_name
                };
                format!(
                    "The `{js_name}()` method.\n\n{}",
                    mdn_doc(&parent_js_name, Some(method))
                )
            }
            InterfaceMethodKind::IndexingGetter => {
                extra_args.push(quote!(indexing_getter));
                "Indexing getter. As in the literal Javascript `this[key]`.\n\n".to_string()
            }
            InterfaceMethodKind::IndexingSetter => {
                extra_args.push(quote!(indexing_setter));
                "Indexing setter. As in the literal Javascript `this[key] = value`.\n\n".to_string()
            }
            InterfaceMethodKind::IndexingDeleter => {
                extra_args.push(quote!(indexing_deleter));
                "Indexing deleter. As in the literal Javascript `delete this[key]`.\n\n".to_string()
            }
        };

        // Compute feature set for the generated variant
        let mut features = BTreeSet::new();

        // Add features from argument types
        for (_, wbg_ty) in arguments.iter() {
            if let Ok(Some(ty)) =
                wbg_ty.to_syn_type(crate::util::TypePosition::ARGUMENT, false, generics_compat)
            {
                add_features(&mut features, &ty);
            }
        }

        // Add features from return type
        if let Some(ref ty) = ret_ty {
            add_features(&mut features, ty);
        }

        // Remove parent types from feature set
        for parent in parents {
            features.remove(&parent.to_string());
        }

        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        // For documentation
        let mut features_doc = features.clone();
        features_doc.insert(parent_name.to_string());

        let doc_comment = comment(doc_comment, &required_doc_string(options, &features_doc));

        let deprecated = deprecated
            .as_ref()
            .or(parent_deprecated.as_ref())
            .map(|msg| match msg {
                Some(msg) => quote!( #[deprecated(note = #msg)] ),
                None => quote!( #[deprecated] ),
            });

        let catch_attr = if *catch { Some(quote!(catch,)) } else { None };

        let (method, this) = if is_constructor {
            assert!(!is_static);

            (quote!(constructor,), None)
        } else if *is_static {
            (quote!( static_method_of = #parent_name, ), None)
        } else {
            let structural = if *structural {
                quote!(structural)
            } else {
                quote!(final)
            };

            (
                quote!( method, #structural, ),
                Some(quote!( this: &#parent_name, )),
            )
        };

        let variadic_attr = generate_variadic(*variadic);

        // Generate arguments
        let arguments = match generate_arguments(
            arguments,
            *variadic,
            variadic_type.as_ref(),
            generics_compat,
        ) {
            Some(args) => args,
            None => {
                log::warn!(
                    "SKIPPING method {name} on {parent_name}: args failed, args: {arguments:?}"
                );
                return None;
            }
        };

        // Build the return type token
        let ret = {
            let ret = ret_ty.as_ref().map(|ret| quote!( #ret ));
            let ret = if *catch {
                let ret = ret.unwrap_or_else(|| quote!(()));
                Some(quote!( Result<#ret, JsValue> ))
            } else {
                ret
            };
            ret.as_ref().map(|ret| quote!( -> #ret ))
        };

        Some(quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #catch_attr
                #method
                #variadic_attr
                #(#extra_args),*
            )]
            #doc_comment
            #unstable_docs
            #deprecated
            pub fn #name(#this #(#arguments),*) #ret;
        })
    }
}

pub struct Interface<'a> {
    pub name: Ident,
    pub js_name: String,
    pub deprecated: Option<Option<String>>,
    pub has_interface: bool,
    pub parents: Vec<Ident>,
    pub consts: Vec<Const>,
    pub attributes: Vec<InterfaceAttribute>,
    pub methods: Vec<InterfaceMethod<'a>>,
    pub unstable: bool,
}

impl Interface<'_> {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Interface {
            name,
            js_name,
            deprecated,
            has_interface,
            parents,
            consts,
            attributes,
            methods,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let doc_comment = comment(
            format!("The `{name}` class.\n\n{}", mdn_doc(js_name, None)),
            &get_features_doc(options, name.to_string()),
        );

        let is_type_of = if *has_interface {
            None
        } else {
            Some(quote!(is_type_of = |_| false,))
        };

        let prefixes = if POLYFILL_INTERFACES.contains(js_name.as_str()) {
            Some(quote!(vendor_prefix = webkit,))
        } else {
            None
        };

        let extends = parents
            .iter()
            .map(|x| quote!( extends = #x, ))
            .collect::<Vec<_>>();

        let consts = consts
            .iter()
            .map(|x| x.generate(options, name, js_name, deprecated))
            .collect::<Vec<_>>();

        let consts = if consts.is_empty() {
            None
        } else {
            Some(quote! {
                #unstable_attr
                impl #name {
                    #(#consts)*
                }
            })
        };

        let attributes = attributes
            .iter()
            .map(|x| x.generate(options, name, js_name, parents, deprecated))
            .collect::<Vec<_>>();

        let methods = methods
            .iter()
            .filter_map(|x| x.generate(options, name, js_name.to_string(), parents, deprecated))
            .collect::<Vec<_>>();

        let deprecated = deprecated.as_ref().map(|msg| match msg {
            Some(msg) => quote!( #[deprecated(note = #msg)] ),
            None => quote!( #[deprecated] ),
        });
        let js_ident = raw_ident(js_name);

        quote! {
            #![allow(unused_imports)]
            #![allow(clippy::all)]
            use super::*;
            use wasm_bindgen::prelude::*;

            #unstable_attr
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(
                    #is_type_of
                    #prefixes
                    #(#extends)*
                    extends = ::js_sys::Object,
                    js_name = #js_ident,
                    typescript_type = #js_name
                )]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #doc_comment
                #unstable_docs
                #deprecated
                pub type #name;

                #(#attributes)*
                #(#methods)*
            }

            #consts
        }
    }
}

/// A single setter variant for a dictionary field (one per union member when expanded)
pub struct DictionaryFieldSetter {
    pub ty: Type,
    pub name_suffix: Option<String>,
    /// Whether this setter is deprecated (for backward-compat setters superseded by type-safe ones)
    pub deprecated: bool,
}

pub struct DictionaryField {
    pub name: String,
    pub js_name: String,
    /// Primary type (used for the getter return and builder method)
    pub ty: Type,
    pub return_ty: Type,
    /// All setter variants - for union types, one per union member
    pub setter_types: Vec<DictionaryFieldSetter>,
    /// When true, the nullable type collapses: the setter takes `&JsValue` instead of
    /// `Option<&JsValue>`, and the builder uses `unwrap_or(&JsValue::NULL)`.
    pub is_js_value_ref_option_type: bool,
    pub required: bool,
    pub unstable: bool,
    pub deprecated: Option<Option<String>>,
}

impl DictionaryField {
    fn generate_rust_shim(
        &self,
        parent_ident: &Ident,
        options: &Options,
        features: &BTreeSet<String>,
        cfg_features: &Option<syn::Attribute>,
    ) -> TokenStream {
        let return_ty = &self.return_ty;
        let getter_name = format_ident!("get_{}", self.name);
        let js_name = &self.js_name;

        let unstable_attr = maybe_unstable_attr(self.unstable);
        let unstable_docs = maybe_unstable_docs(self.unstable);

        let deprecated = self.deprecated.as_ref().map(|msg| match msg {
            Some(msg) => quote!( #[deprecated(note = #msg)] ),
            None => quote!( #[deprecated] ),
        });

        let getter_doc_comment = comment(
            format!("Get the `{js_name}` field of this object."),
            &required_doc_string(options, features),
        );

        let setter_doc_comment = comment(
            format!("Change the `{js_name}` field of this object."),
            &required_doc_string(options, features),
        );

        // When is_js_value_ref_option_type is set, the nullable type collapses
        // to &JsValue for backwards compatibility. This applies to the deprecated
        // fallback setter (no suffix) — typed setters keep their real types.
        let js_value_ref_type = if self.is_js_value_ref_option_type {
            Some(shared_ref(
                leading_colon_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")]),
                false,
            ))
        } else {
            None
        };

        // Generate setters for each type variant.
        // In stable mode (!next_unstable), the first variant (deprecated fallback)
        // gets the JsValue override for backwards compat. In next_unstable mode,
        // all variants use their real typed signatures.
        let setters = self.setter_types.iter().enumerate().map(|(idx, setter)| {
            let setter_name = match &setter.name_suffix {
                Some(suffix) => format_ident!("set_{}_{}", self.name, suffix),
                None => format_ident!("set_{}", self.name),
            };
            let ty = if idx == 0 && !options.next_unstable.get() {
                js_value_ref_type.as_ref().unwrap_or(&setter.ty)
            } else {
                &setter.ty
            };

            // Get features for this specific setter type
            let mut setter_features = BTreeSet::new();
            add_features(&mut setter_features, ty);
            setter_features.remove(&parent_ident.to_string());
            let setter_cfg_features = get_cfg_features(options, &setter_features);

            // Deprecate backward-compat setters that have type-safe alternatives
            // But don't add our deprecation if the field is already deprecated
            let setter_deprecated = if setter.deprecated && deprecated.is_none() {
                let name = &self.name;
                let alternatives: Vec<_> = self
                    .setter_types
                    .iter()
                    .filter_map(|s| s.name_suffix.as_ref())
                    .map(|s| format!("`set_{name}_{s}()`"))
                    .collect();
                let msg = format!("Use {} instead.", alternatives.join(" or "));
                Some(quote!( #[deprecated(note = #msg)] ))
            } else {
                None
            };

            quote! {
                #unstable_attr
                #setter_cfg_features
                #setter_doc_comment
                #unstable_docs
                #setter_deprecated
                #deprecated
                #[wasm_bindgen(method, setter = #js_name)]
                pub fn #setter_name(this: &#parent_ident, val: #ty);
            }
        });

        quote! {
            #unstable_attr
            #cfg_features
            #getter_doc_comment
            #unstable_docs
            #deprecated
            #[wasm_bindgen(method, getter = #js_name)]
            pub fn #getter_name(this: &#parent_ident) -> #return_ty;

            #(#setters)*
        }
    }

    fn generate_rust_setter(&self, cfg_features: &Option<syn::Attribute>) -> TokenStream {
        let DictionaryField {
            name,
            js_name: _,
            ty,
            return_ty: _,
            setter_types: _,
            is_js_value_ref_option_type: _,
            required: _,
            unstable,
            deprecated: _,
        } = self;

        let name = rust_ident(name);
        let unstable_attr = maybe_unstable_attr(*unstable);

        let setter_name = self.setter_name();
        let deprecated = format!("Use `{setter_name}()` instead.");

        // When is_js_value_ref_option_type is set, the first setter takes &JsValue
        // but the builder takes Option<&JsValue>, so unwrap_or bridges the types.
        let shim_args = if self.is_js_value_ref_option_type {
            quote! { val.unwrap_or(&::wasm_bindgen::JsValue::NULL) }
        } else {
            quote! { val }
        };

        quote! {
            #unstable_attr
            #cfg_features
            #[deprecated = #deprecated]
            pub fn #name(&mut self, val: #ty) -> &mut Self {
                self.#setter_name(#shim_args);
                self
            }
        }
    }

    fn features(
        &self,
        options: &Options,
        parent_name: String,
    ) -> (BTreeSet<String>, Option<syn::Attribute>) {
        let mut features = BTreeSet::new();

        // Only collect features from the return type (getter).
        // Each setter computes its own features independently in generate_rust_shim.
        add_features(&mut features, &self.return_ty);

        features.remove(&parent_name);

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name);

        (features, cfg_features)
    }

    fn setter_name(&self) -> Ident {
        // Use the first setter's name (which may include a suffix for union types)
        match self
            .setter_types
            .first()
            .and_then(|s| s.name_suffix.as_ref())
        {
            Some(suffix) => format_ident!("set_{}_{}", self.name, suffix),
            None => format_ident!("set_{}", self.name),
        }
    }
}

pub struct Dictionary {
    pub name: Ident,
    pub js_name: String,
    pub fields: Vec<DictionaryField>,
    pub unstable: bool,
    pub deprecated: Option<Option<String>>,
}

impl Dictionary {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Dictionary {
            name,
            js_name,
            fields,
            unstable,
            deprecated,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);
        let deprecated = deprecated.as_ref().map(|msg| match msg {
            Some(msg) => quote!( #[deprecated(note = #msg)] ),
            None => quote!( #[deprecated] ),
        });

        let js_name = raw_ident(js_name);

        let mut required_features = BTreeSet::new();
        let mut required_args = vec![];
        let mut required_calls = vec![];

        for field in fields.iter() {
            if field.required {
                let name = rust_ident(&field.name);
                let set_name = rust_ident(&format!("set_{}", field.name));
                let ty = &field.ty;
                required_args.push(quote!( #name: #ty ));
                required_calls.push(quote!( ret.#set_name(#name); ));
                add_features(&mut required_features, &field.ty);
            }
        }

        required_features.remove(&name.to_string());

        let cfg_features = get_cfg_features(options, &required_features);

        required_features.insert(name.to_string());

        let doc_comment = comment(
            format!("The `{name}` dictionary."),
            &get_features_doc(options, name.to_string()),
        );
        let ctor_doc_comment = comment(
            format!("Construct a new `{name}`."),
            &required_doc_string(options, &required_features),
        );

        let (field_features, field_cfg_features): (Vec<_>, Vec<_>) = fields
            .iter()
            .map(|field| field.features(options, name.to_string()))
            .unzip();

        let field_shims = fields
            .iter()
            .zip(field_features.iter())
            .zip(field_cfg_features.iter())
            .map(|((field, features), cfg_features)| {
                field.generate_rust_shim(name, options, features, cfg_features)
            })
            .collect::<Vec<_>>();

        let fields = fields
            .iter()
            .zip(field_cfg_features.iter())
            .map(|(field, cfg_features)| field.generate_rust_setter(cfg_features))
            .collect::<Vec<_>>();

        let mut base_stream = quote! {
            #![allow(unused_imports)]
            #![allow(clippy::all)]
            use super::*;
            use wasm_bindgen::prelude::*;

            #unstable_attr
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(extends = ::js_sys::Object, js_name = #js_name)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #doc_comment
                #unstable_docs
                #deprecated
                pub type #name;

                #(#field_shims)*
            }

            #unstable_attr
            impl #name {
                #cfg_features
                #ctor_doc_comment
                #unstable_docs
                #deprecated
                pub fn new(#(#required_args),*) -> Self {
                    #[allow(unused_mut)]
                    let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
                    #(#required_calls)*
                    ret
                }

                #(#fields)*
            }
        };

        if required_args.is_empty() {
            let default_impl = quote! {
                #unstable_attr
                impl Default for #name {
                    fn default() -> Self {
                        Self::new()
                    }
                }
            };

            base_stream.extend(default_impl);
        }

        base_stream
    }
}

pub enum NamespaceAttributeKind {
    Getter,
    /// Note: Per the WebIDL spec, namespace attributes are always readonly,
    /// so this variant is currently unused. It's kept for potential future use
    /// or non-standard WebIDL extensions.
    #[allow(dead_code)]
    Setter,
}

pub struct NamespaceAttribute {
    pub js_name: String,
    pub rust_name: String,
    pub ty: Type,
    pub catch: bool,
    pub kind: NamespaceAttributeKind,
    pub unstable: bool,
}

impl NamespaceAttribute {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        ns_type_name: &Ident,
        parent_js_name: &str,
    ) -> TokenStream {
        let NamespaceAttribute {
            js_name,
            rust_name,
            ty,
            catch,
            kind,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let mdn_docs = mdn_doc(parent_js_name, Some(js_name));

        let mut features = BTreeSet::new();

        add_features(&mut features, ty);
        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name.to_string());

        let doc_comment = required_doc_string(options, &features);

        let (prefix, attr, def) = match kind {
            NamespaceAttributeKind::Getter => {
                let rust_name = rust_ident(rust_name);

                let ty = if *catch {
                    quote!( Result<#ty, JsValue> )
                } else {
                    quote!( #ty )
                };

                (
                    "Getter",
                    quote!(getter,),
                    quote!( pub fn #rust_name() -> #ty; ),
                )
            }

            NamespaceAttributeKind::Setter => {
                let rust_name = rust_ident(rust_name);

                let ret_ty = if *catch {
                    Some(quote!( -> Result<(), JsValue> ))
                } else {
                    None
                };

                (
                    "Setter",
                    quote!(setter,),
                    quote!( pub fn #rust_name(value: #ty) #ret_ty; ),
                )
            }
        };

        let catch = if *catch { Some(quote!(catch,)) } else { None };

        let doc_comment = comment(
            format!("{prefix} for the `{parent_js_name}.{js_name}` field.\n\n{mdn_docs}"),
            &doc_comment,
        );

        let js_name_ident = raw_ident(js_name);

        quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #catch
                static_method_of = #ns_type_name,
                js_class = #parent_js_name,
                #attr
                js_name = #js_name_ident
            )]
            #doc_comment
            #unstable_docs
            #def
        }
    }
}

pub struct Function<'a> {
    pub name: Ident,
    pub js_name: String,
    pub arguments: Vec<(Ident, WbgType<'a>)>,
    pub variadic_type: Option<WbgType<'a>>,
    pub ret_wbg_ty: Option<WbgType<'a>>,
    pub catch: bool,
    pub variadic: bool,
    pub unstable: bool,
}

impl Function<'_> {
    pub(crate) fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: String,
    ) -> Option<TokenStream> {
        let Function {
            name,
            js_name,
            arguments,
            variadic_type,
            ret_wbg_ty,
            catch,
            variadic,
            unstable,
        } = self;

        // Unstable APIs always use typed generics (generics_compat=false).
        // Stable APIs use legacy types by default, typed generics if next_unstable is set.
        let generics_compat = if *unstable {
            false
        } else {
            !options.next_unstable.get()
        };

        // Convert WbgType to syn::Type during code generation
        use crate::util::TypePosition;
        let ret_ty = match ret_wbg_ty {
            Some(wbg_ty) => {
                match wbg_ty.to_syn_type(TypePosition::RETURN, false, generics_compat) {
                    Ok(ty) => ty,
                    Err(_) => return None,
                }
            }
            None => None,
        };

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let js_namespace = raw_ident(&parent_js_name);

        let doc_comment = format!(
            "The `{parent_js_name}.{js_name}()` function.\n\n{}",
            mdn_doc(&parent_js_name, Some(js_name))
        );

        // Compute feature set
        let mut features = BTreeSet::new();

        // Add features from argument types
        for (_, wbg_ty) in arguments.iter() {
            if let Ok(Some(ty)) =
                wbg_ty.to_syn_type(crate::util::TypePosition::ARGUMENT, false, generics_compat)
            {
                add_features(&mut features, &ty);
            }
        }

        // Add features from return type
        if let Some(ref ty) = ret_ty {
            add_features(&mut features, ty);
        }

        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        // For documentation
        let mut features_doc = features.clone();
        features_doc.insert(parent_name.to_string());

        let doc_comment = comment(doc_comment, &required_doc_string(options, &features_doc));

        let catch_attr = if *catch { Some(quote!(catch,)) } else { None };

        let variadic_attr = generate_variadic(*variadic);

        let js_name_ident = raw_ident(js_name);

        // Generate arguments
        let arguments = generate_arguments(
            arguments,
            *variadic,
            variadic_type.as_ref(),
            generics_compat,
        )?;

        // Build the return type token
        let ret = {
            let ret = ret_ty.as_ref().map(|ret| quote!( #ret ));
            let ret = if *catch {
                let ret = ret.unwrap_or_else(|| quote!(()));
                Some(quote!( Result<#ret, JsValue> ))
            } else {
                ret
            };
            ret.as_ref().map(|ret| quote!( -> #ret ))
        };

        Some(quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #catch_attr
                #variadic_attr
                js_namespace = #js_namespace,
                js_name = #js_name_ident
            )]
            #doc_comment
            #unstable_docs
            pub fn #name(#(#arguments),*) #ret;
        })
    }
}

pub struct Namespace<'a> {
    pub name: Ident,
    pub js_name: String,
    pub consts: Vec<Const>,
    pub attributes: Vec<NamespaceAttribute>,
    pub functions: Vec<Function<'a>>,
    pub unstable: bool,
}

impl Namespace<'_> {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Namespace {
            name,
            js_name,
            consts,
            attributes,
            functions,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let functions = functions
            .iter()
            .filter_map(|x| x.generate(options, name, js_name.to_string()))
            .collect::<Vec<_>>();

        // For namespace attributes, we need a type binding that represents the namespace
        // so we can use static_method_of to access properties on it
        let ns_type_name = rust_ident(&format!("JsNamespace{}", camel_case_ident(js_name)));
        let js_namespace = raw_ident(js_name);

        let attributes = attributes
            .iter()
            .map(|x| x.generate(options, name, &ns_type_name, js_name))
            .collect::<Vec<_>>();

        // Only generate the namespace type if we have attributes that need it
        let ns_type_binding = if attributes.is_empty() {
            None
        } else {
            Some(quote! {
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_name = #js_namespace)]
                    pub type #ns_type_name;
                }
            })
        };

        let extern_block = if functions.is_empty() && attributes.is_empty() {
            None
        } else {
            Some(quote! {
                #[wasm_bindgen]
                extern "C" {
                    #(#attributes)*
                    #(#functions)*
                }
            })
        };

        let consts = consts
            .iter()
            .map(|x| x.generate(options, name, js_name, &None))
            .collect::<Vec<_>>();

        quote! {
            #unstable_attr
            #unstable_docs
            pub mod #name {
                #![allow(unused_imports)]
                #![allow(clippy::all)]
                use super::super::*;
                use wasm_bindgen::prelude::*;

                #(#consts)*

                #ns_type_binding

                #extern_block
            }
        }
    }
}
