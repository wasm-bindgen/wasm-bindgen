//! FunctionDecl / VariableDecl → free function and static bindings.

use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::signatures::{
    expand_signatures, generate_concrete_params, is_void_return, ExpandedSignature, SignatureKind,
};
use crate::codegen::typemap::{to_return_type, to_syn_type, CodegenContext, TypePosition};
use crate::parse::scope::ScopeId;
use std::collections::HashSet;

use crate::ir::{FunctionDecl, ModuleContext, VariableDecl};

/// Generate wasm_bindgen extern blocks for a free function.
///
/// Expands optional params into multiple overloads, and generates `try_` variants.
pub fn generate_function(
    decl: &FunctionDecl,
    ctx: &ModuleContext,
    cgctx: Option<&CodegenContext<'_>>,
    doc: &Option<String>,
    scope: ScopeId,
) -> TokenStream {
    let mut used_names = HashSet::new();
    let sigs = expand_signatures(
        &decl.js_name,
        &[decl.params.as_slice()],
        &decl.return_type,
        SignatureKind::Function,
        doc,
        &mut used_names,
        cgctx,
        scope,
    );

    let items: Vec<TokenStream> = sigs
        .iter()
        .map(|sig| generate_expanded_free_function(sig, ctx, cgctx, None, scope))
        .collect();

    quote! { #(#items)* }
}

/// Generate wasm_bindgen extern blocks for a free function inside a namespace.
pub fn generate_function_with_js_namespace(
    decl: &FunctionDecl,
    ctx: &ModuleContext,
    js_namespace: &str,
    cgctx: Option<&CodegenContext<'_>>,
    doc: &Option<String>,
    scope: ScopeId,
) -> TokenStream {
    let mut used_names = HashSet::new();
    let sigs = expand_signatures(
        &decl.js_name,
        &[decl.params.as_slice()],
        &decl.return_type,
        SignatureKind::Function,
        doc,
        &mut used_names,
        cgctx,
        scope,
    );

    let items: Vec<TokenStream> = sigs
        .iter()
        .map(|sig| generate_expanded_free_function(sig, ctx, cgctx, Some(js_namespace), scope))
        .collect();

    quote! { #(#items)* }
}

/// Generate a single extern block for one expanded free function signature.
fn generate_expanded_free_function(
    sig: &ExpandedSignature,
    ctx: &ModuleContext,
    cgctx: Option<&CodegenContext<'_>>,
    js_namespace: Option<&str>,
    scope: ScopeId,
) -> TokenStream {
    let rust_ident = super::typemap::make_ident(&sig.rust_name);
    let params = generate_concrete_params(&sig.params, cgctx, scope);
    let ret_ty = to_return_type(&sig.return_type, sig.catch, cgctx, scope);
    let doc = super::doc_tokens(&sig.doc);
    let has_variadic = sig.params.last().is_some_and(|p| p.variadic);

    let mut wb_parts: Vec<TokenStream> = Vec::new();
    if has_variadic {
        wb_parts.push(quote! { variadic });
    }
    if sig.catch {
        wb_parts.push(quote! { catch });
    }
    // Emit js_name when the JS name differs from the Rust name.
    // wasm-bindgen uses the Rust fn name as the JS name by default,
    // so we need js_name for any camelCase → snake_case conversion.
    if sig.rust_name != sig.js_name {
        let js_name = &sig.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }
    if let Some(ns) = js_namespace {
        wb_parts.push(quote! { js_namespace = #ns });
    }

    let wb_attr = if wb_parts.is_empty() {
        quote! {}
    } else {
        quote! { #[wasm_bindgen(#(#wb_parts),*)] }
    };

    let ret = if is_void_return(&sig.return_type) && !sig.catch {
        quote! {}
    } else {
        quote! { -> #ret_ty }
    };

    let wb_extern_attr = match ctx {
        ModuleContext::Module(m) => quote! { #[wasm_bindgen(module = #m)] },
        ModuleContext::Global => quote! { #[wasm_bindgen] },
    };

    quote! {
        #wb_extern_attr
        extern "C" {
            #doc
            #wb_attr
            pub fn #rust_ident(#params) #ret;
        }
    }
}

/// Generate a wasm_bindgen extern block for a global constant/variable.
pub fn generate_variable(
    decl: &VariableDecl,
    ctx: &ModuleContext,
    cgctx: Option<&CodegenContext<'_>>,
    doc: &Option<String>,
    js_namespace: Option<&str>,
    scope: ScopeId,
) -> TokenStream {
    let rust_ident = super::typemap::make_ident(&decl.name);
    let ty = to_syn_type(&decl.type_ref, TypePosition::RETURN, cgctx, scope);
    let doc = super::doc_tokens(doc);

    let mut wb_parts: Vec<TokenStream> = vec![quote! { thread_local_v2 }];
    if decl.js_name != decl.name {
        let js_name = &decl.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }
    if let Some(ns) = js_namespace {
        wb_parts.push(quote! { js_namespace = #ns });
    }

    let wb_attr = quote! { #[wasm_bindgen(#(#wb_parts),*)] };

    let wb_extern_attr = match ctx {
        ModuleContext::Module(m) => quote! { #[wasm_bindgen(module = #m)] },
        ModuleContext::Global => quote! { #[wasm_bindgen] },
    };

    quote! {
        #wb_extern_attr
        extern "C" {
            #doc
            #wb_attr
            pub static #rust_ident: #ty;
        }
    }
}
