//! This crate contains implementation APIs for the `#[wasm_bindgen]` attribute.

#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro-support/0.2")]

#[macro_use]
mod error;

mod ast;
mod codegen;
mod encode;
mod generics;
mod hash;
mod parser;

use codegen::TryToTokens;
use error::Diagnostic;
pub use parser::BindgenAttrs;
use parser::{ConvertToAst, MacroParse};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::Token;

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    parser::reset_attrs_used();
    // if struct is encountered, add `derive` attribute and let everything happen there (workaround
    // to help parsing cfg_attr correctly).
    let item = syn::parse2::<syn::Item>(input)?;
    if let syn::Item::Struct(s) = item {
        let opts: BindgenAttrs = syn::parse2(attr.clone())?;
        let wasm_bindgen = opts
            .wasm_bindgen()
            .cloned()
            .unwrap_or_else(|| syn::parse_quote! { ::wasm_bindgen });

        let item = quote! {
            #[derive(#wasm_bindgen::__rt::BindgenedStruct)]
            #[wasm_bindgen(#attr)]
            #s
        };
        return Ok(item);
    }

    let opts = syn::parse2(attr)?;
    let mut tokens = proc_macro2::TokenStream::new();
    let mut program = ast::Program::default();
    item.macro_parse(&mut program, (Some(opts), &mut tokens))?;
    program.try_to_tokens(&mut tokens)?;

    // If we successfully got here then we should have used up all attributes
    // and considered all of them to see if they were used. If one was forgotten
    // that's a bug on our end, so sanity check here.
    parser::check_unused_attrs(&mut tokens);

    Ok(tokens)
}

/// Takes the parsed input from a `wasm_bindgen::link_to` macro and returns the generated link
pub fn expand_link_to(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    parser::reset_attrs_used();
    let opts = syn::parse2(input)?;

    let mut tokens = proc_macro2::TokenStream::new();
    let link = parser::link_to(opts)?;
    link.try_to_tokens(&mut tokens)?;

    Ok(tokens)
}

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand_class_marker(
    attr: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Diagnostic> {
    parser::reset_attrs_used();
    let mut item = syn::parse2::<syn::ImplItemFn>(input)?;
    let opts: ClassMarker = syn::parse2(attr)?;

    let mut program = ast::Program::default();
    item.macro_parse(&mut program, &opts)?;

    // This is where things are slightly different, we are being expanded in the
    // context of an impl so we can't inject arbitrary item-like tokens into the
    // output stream. If we were to do that then it wouldn't parse!
    //
    // Instead what we want to do is to generate the tokens for `program` into
    // the header of the function. This'll inject some no_mangle functions and
    // statics and such, and they should all be valid in the context of the
    // start of a function.
    //
    // We manually implement `ToTokens for ImplItemFn` here, injecting our
    // program's tokens before the actual method's inner body tokens.
    let mut tokens = proc_macro2::TokenStream::new();
    tokens.append_all(
        item.attrs
            .iter()
            .filter(|attr| matches!(attr.style, syn::AttrStyle::Outer)),
    );
    item.vis.to_tokens(&mut tokens);
    item.sig.to_tokens(&mut tokens);
    let mut err = None;
    item.block.brace_token.surround(&mut tokens, |tokens| {
        if let Err(e) = program.try_to_tokens(tokens) {
            err = Some(e);
        }
        parser::check_unused_attrs(tokens); // same as above
        tokens.append_all(
            item.attrs
                .iter()
                .filter(|attr| matches!(attr.style, syn::AttrStyle::Inner(_))),
        );
        tokens.append_all(&item.block.stmts);
    });

    if let Some(err) = err {
        return Err(err);
    }

    Ok(tokens)
}

struct ClassMarker {
    class: syn::Ident,
    js_class: String,
    wasm_bindgen: syn::Path,
    wasm_bindgen_futures: syn::Path,
}

impl Parse for ClassMarker {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let class = input.parse::<syn::Ident>()?;
        input.parse::<Token![=]>()?;
        let mut js_class = input.parse::<syn::LitStr>()?.value();
        js_class = js_class
            .strip_prefix("r#")
            .map(String::from)
            .unwrap_or(js_class);

        let mut wasm_bindgen = None;
        let mut wasm_bindgen_futures = None;

        loop {
            if input.parse::<Option<Token![,]>>()?.is_some() {
                let ident = input.parse::<syn::Ident>()?;

                if ident == "wasm_bindgen" {
                    if wasm_bindgen.is_some() {
                        return Err(syn::Error::new(
                            ident.span(),
                            "found duplicate `wasm_bindgen`",
                        ));
                    }

                    input.parse::<Token![=]>()?;
                    wasm_bindgen = Some(input.parse::<syn::Path>()?);
                } else if ident == "wasm_bindgen_futures" {
                    if wasm_bindgen_futures.is_some() {
                        return Err(syn::Error::new(
                            ident.span(),
                            "found duplicate `wasm_bindgen_futures`",
                        ));
                    }

                    input.parse::<Token![=]>()?;
                    wasm_bindgen_futures = Some(input.parse::<syn::Path>()?);
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "expected `wasm_bindgen` or `wasm_bindgen_futures`",
                    ));
                }
            } else {
                break;
            }
        }

        Ok(ClassMarker {
            class,
            js_class,
            wasm_bindgen: wasm_bindgen.unwrap_or_else(|| syn::parse_quote! { wasm_bindgen }),
            wasm_bindgen_futures: wasm_bindgen_futures
                .unwrap_or_else(|| syn::parse_quote! { wasm_bindgen_futures }),
        })
    }
}

pub fn expand_struct_marker(item: TokenStream) -> Result<TokenStream, Diagnostic> {
    parser::reset_attrs_used();

    let mut s: syn::ItemStruct = syn::parse2(item)?;

    let mut program = ast::Program::default();
    program.structs.push((&mut s).convert(&program)?);

    let mut tokens = proc_macro2::TokenStream::new();
    program.try_to_tokens(&mut tokens)?;

    parser::check_unused_attrs(&mut tokens);

    Ok(tokens)
}

/// Expands the `#[derive(UpcastCore)]` macro
pub fn expand_upcast_core(item: TokenStream) -> Result<TokenStream, Diagnostic> {
    let input = syn::parse2::<syn::DeriveInput>(item)?;

    let name = &input.ident;
    let generics = &input.generics;

    // Extract type parameters only (not lifetimes or consts)
    let type_params: Vec<_> = generics.type_params().collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = quote! {};

    // Always generate Upcast<JsValue> impl
    (quote! {
        #[automatically_derived]
        impl #impl_generics ::wasm_bindgen::convert::Upcast<::wasm_bindgen::JsValue>
            for #name #ty_generics
        #where_clause
        {
        }
    })
    .to_tokens(&mut output);

    // For non-generic types, generate identity impl (Upcast<Self>)
    if type_params.is_empty() {
        (quote! {
            #[automatically_derived]
            impl #impl_generics ::wasm_bindgen::convert::Upcast<#name #ty_generics>
                for #name #ty_generics
            #where_clause
            {
            }
        })
        .to_tokens(&mut output);
    } else {
        // For generic types, generate structural covariance impl
        // Build impl generics: all original params plus a Target param for each
        let mut impl_generics_extended = generics.clone();
        let target_param_names: Vec<syn::Ident> = type_params
            .iter()
            .enumerate()
            .map(|(i, _tp)| {
                let target_name = quote::format_ident!("Target{}", i);
                impl_generics_extended
                    .params
                    .push(syn::parse_quote!(#target_name));
                target_name
            })
            .collect();

        // Build where clause: T: Upcast<Target>
        let mut where_clause_extended =
            generics
                .where_clause
                .clone()
                .unwrap_or_else(|| syn::WhereClause {
                    where_token: Default::default(),
                    predicates: Default::default(),
                });

        for (type_param, target_name) in type_params.iter().zip(&target_param_names) {
            let param_ident = &type_param.ident;
            where_clause_extended.predicates.push(syn::parse_quote!(
                #param_ident: ::wasm_bindgen::convert::Upcast<#target_name>
            ));
            // where_clause_extended.predicates.push(syn::parse_quote!(
            //     #param_ident: ::wasm_bindgen::ErasableGeneric<Repr = ::wasm_bindgen::JsValue>
            // ));
            // where_clause_extended.predicates.push(syn::parse_quote!(
            //     #target_name: ::wasm_bindgen::ErasableGeneric<Repr = ::wasm_bindgen::JsValue>
            // ));
        }

        // Build the target type with Target params
        let target_type = quote! { #name<#(#target_param_names),*> };

        let (impl_generics_split, _, _) = impl_generics_extended.split_for_impl();

        // Structural covariance - Type<T1, T2, ...> can widen to Type<Target0, Target1, ...>
        (quote! {
            #[automatically_derived]
            impl #impl_generics_split ::wasm_bindgen::convert::Upcast<#target_type> for #name #ty_generics
            #where_clause_extended
            {
            }
        })
        .to_tokens(&mut output);
    }

    Ok(output)
}

/// Expands the `#[derive(Upcast)]` macro
/// This assumes `Object` is in scope (from js-sys)
pub fn expand_upcast(item: TokenStream) -> Result<TokenStream, Diagnostic> {
    // First call the base UpcastCore derive
    let mut output = expand_upcast_core(item.clone())?;

    let input = syn::parse2::<syn::DeriveInput>(item)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Add Upcast<Object> impl using unqualified path
    // Object must be in scope for this to work (it always is in js-sys)
    (quote! {
        #[automatically_derived]
        impl #impl_generics ::wasm_bindgen::convert::Upcast<::js_sys::Object>
            for #name #ty_generics
        #where_clause
        {
        }
    })
    .to_tokens(&mut output);

    Ok(output)
}
