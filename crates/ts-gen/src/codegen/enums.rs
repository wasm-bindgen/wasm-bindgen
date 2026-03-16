//! Enum code generation: string enums and numeric enums.
//!
//! String enums use wasm_bindgen's native string enum support:
//!
//! ```rust,ignore
//! #[wasm_bindgen]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum QueueContentType {
//!     #[wasm_bindgen(js_name = "text")]
//!     Text,
//!     #[wasm_bindgen(js_name = "bytes")]
//!     Bytes,
//! }
//! ```
//!
//! Numeric enums use `#[repr(u32)]` or `#[repr(i32)]` (selected based on
//! discriminant values) with wasm_bindgen:
//!
//! ```rust,ignore
//! #[wasm_bindgen]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! #[repr(u32)]
//! pub enum ImportType {
//!     Static = 1,
//!     Dynamic = 2,
//!     ImportMeta = 3,
//! }
//! ```

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::{NumericEnumDecl, StringEnumDecl};

/// Generate a wasm_bindgen string enum.
pub fn generate_string_enum(decl: &StringEnumDecl) -> TokenStream {
    let name = super::typemap::make_ident(&decl.name);

    let variants: Vec<_> = decl
        .variants
        .iter()
        .map(|v| {
            let rust_name = super::typemap::make_ident(&v.rust_name);
            let js_value = &v.js_value;
            quote! {
                #[wasm_bindgen(js_name = #js_value)]
                #rust_name
            }
        })
        .collect();

    quote! {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #name {
            #(#variants,)*
        }
    }
}

/// Determine the appropriate repr for a numeric enum based on its discriminant values.
fn numeric_enum_repr(decl: &NumericEnumDecl) -> TokenStream {
    let min = decl.variants.iter().map(|v| v.value).min().unwrap_or(0);
    let max = decl.variants.iter().map(|v| v.value).max().unwrap_or(0);

    if min >= 0 && max <= u32::MAX as i64 {
        quote! { u32 }
    } else if min >= i32::MIN as i64 && max <= i32::MAX as i64 {
        quote! { i32 }
    } else {
        // Fallback — wasm_bindgen doesn't support i64 repr, so clamp to i32
        // and emit the best we can. This is an extreme edge case.
        quote! { i32 }
    }
}

/// Generate a wasm_bindgen numeric enum with automatically selected repr.
pub fn generate_numeric_enum(decl: &NumericEnumDecl) -> TokenStream {
    let name = super::typemap::make_ident(&decl.name);
    let repr = numeric_enum_repr(decl);

    let has_negative = decl.variants.iter().any(|v| v.value < 0);

    let variants: Vec<_> = decl
        .variants
        .iter()
        .map(|v| {
            let rust_name = super::typemap::make_ident(&v.rust_name);
            let doc = crate::codegen::doc_tokens(&v.doc);
            if has_negative {
                let value = i32::try_from(v.value).unwrap_or({
                    // Value out of i32 range — truncation is lossy but we
                    // already warned at parse time if out of i64 range.
                    v.value as i32
                });
                quote! {
                    #doc
                    #rust_name = #value
                }
            } else {
                let value = u32::try_from(v.value).unwrap_or(v.value as u32);
                quote! {
                    #doc
                    #rust_name = #value
                }
            }
        })
        .collect();

    quote! {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(#repr)]
        pub enum #name {
            #(#variants,)*
        }
    }
}
