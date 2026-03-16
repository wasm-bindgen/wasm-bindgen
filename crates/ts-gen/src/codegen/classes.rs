//! ClassDecl / ClassLike InterfaceDecl → wasm_bindgen `extern "C"` block generation.
//!
//! Generates the standard pattern seen in worker-sys:
//!
//! ```rust,ignore
//! #[wasm_bindgen]
//! extern "C" {
//!     #[wasm_bindgen(extends = js_sys::Object, js_name = "MyClass")]
//!     #[derive(Debug, Clone, PartialEq, Eq)]
//!     pub type MyClass;
//!
//!     #[wasm_bindgen(constructor, catch)]
//!     pub fn new(arg: &str) -> Result<MyClass, JsValue>;
//!
//!     #[wasm_bindgen(method, getter)]
//!     pub fn name(this: &MyClass) -> String;
//!
//!     #[wasm_bindgen(method, js_name = "doThing")]
//!     pub fn do_thing(this: &MyClass, x: f64);
//! }
//! ```

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::signatures::{
    dedupe_name, expand_signatures, generate_concrete_params, is_void_return, ExpandedSignature,
    SignatureKind,
};
use crate::codegen::typemap::{to_return_type, to_syn_type, CodegenContext, TypePosition};
use crate::ir::{
    ClassDecl, GetterMember, InterfaceClassification, InterfaceDecl, Member, ModuleContext,
    SetterMember, StaticGetterMember, StaticSetterMember, TypeRef,
};
use crate::parse::scope::ScopeId;
use crate::util::naming::to_snake_case;

/// Configuration for generating a class-like extern block.
struct ClassConfig<'a> {
    /// Rust type name.
    rust_name: String,
    /// JS class name (for `js_name` / `js_class` attributes).
    js_name: String,
    /// The `extends` parents (wasm_bindgen supports chained `extends = ...`).
    extends: Vec<TokenStream>,
    /// Module specifier for `#[wasm_bindgen(module = "...")]`.
    module: Option<std::rc::Rc<str>>,
    /// JS namespace (e.g., `"WebAssembly"`) for types inside a namespace.
    js_namespace: Option<String>,
    /// Whether this is an abstract class (skip constructor).
    is_abstract: bool,
    /// Members to generate.
    members: Vec<Member>,
    /// Codegen context for type resolution.
    cgctx: Option<&'a CodegenContext<'a>>,
    /// Scope for type reference resolution.
    scope: ScopeId,
}

impl<'a> ClassConfig<'a> {
    fn from_class(
        decl: &ClassDecl,
        ctx: &ModuleContext,
        cgctx: Option<&'a CodegenContext>,
        scope: ScopeId,
    ) -> Self {
        let extends = match &decl.extends {
            Some(e) => vec![extends_tokens(e, cgctx, scope)],
            None => vec![quote! { Object }],
        };
        let module = match ctx {
            ModuleContext::Module(m) => Some(m.clone()),
            ModuleContext::Global => None,
        };

        ClassConfig {
            rust_name: decl.name.clone(),
            js_name: decl.js_name.clone(),
            extends,
            module,
            js_namespace: None,
            is_abstract: decl.is_abstract,
            members: decl.members.clone(),
            cgctx,
            scope,
        }
    }

    fn from_interface(
        decl: &InterfaceDecl,
        ctx: &ModuleContext,
        cgctx: Option<&'a CodegenContext>,
        scope: ScopeId,
    ) -> Self {
        let extends = if decl.extends.is_empty() {
            vec![quote! { Object }]
        } else {
            decl.extends
                .iter()
                .map(|e| extends_tokens(e, cgctx, scope))
                .collect()
        };
        let module = match ctx {
            ModuleContext::Module(m) => Some(m.clone()),
            ModuleContext::Global => None,
        };

        ClassConfig {
            rust_name: decl.name.clone(),
            js_name: decl.js_name.clone(),
            extends,
            module,
            js_namespace: None,
            is_abstract: false,
            members: decl.members.clone(),
            cgctx,
            scope,
        }
    }
}

/// Generate a complete `extern "C"` block for a class-like declaration.
pub fn generate_class(
    decl: &ClassDecl,
    ctx: &ModuleContext,
    cgctx: Option<&CodegenContext<'_>>,
    scope: ScopeId,
) -> TokenStream {
    let config = ClassConfig::from_class(decl, ctx, cgctx, scope);
    generate_extern_block(&config)
}

/// Generate a complete `extern "C"` block for a class-like interface.
pub fn generate_class_like_interface(
    decl: &InterfaceDecl,
    ctx: &ModuleContext,
    cgctx: Option<&CodegenContext<'_>>,
    js_namespace: Option<&str>,
    scope: ScopeId,
) -> TokenStream {
    debug_assert!(
        matches!(
            decl.classification,
            InterfaceClassification::ClassLike | InterfaceClassification::Unclassified
        ),
        "expected ClassLike or Unclassified, got {:?}",
        decl.classification
    );
    let mut config = ClassConfig::from_interface(decl, ctx, cgctx, scope);
    config.js_namespace = js_namespace.map(|s| s.to_string());
    generate_extern_block(&config)
}

/// Generate a complete `extern "C"` block for a class inside a namespace, with `js_namespace`.
pub fn generate_class_with_js_namespace(
    decl: &ClassDecl,
    ctx: &ModuleContext,
    js_namespace: &str,
    cgctx: Option<&CodegenContext<'_>>,
    scope: ScopeId,
) -> TokenStream {
    let mut config = ClassConfig::from_class(decl, ctx, cgctx, scope);
    config.js_namespace = Some(js_namespace.to_string());
    generate_extern_block(&config)
}

/// Generate a simple extern "C" block for a dictionary interface.
/// Temporary until M5 implements proper dictionary builders.
pub fn generate_dictionary_extern(
    decl: &InterfaceDecl,
    ctx: &ModuleContext,
    cgctx: Option<&CodegenContext<'_>>,
    js_namespace: Option<&str>,
    scope: ScopeId,
) -> TokenStream {
    let mut config = ClassConfig::from_interface(decl, ctx, cgctx, scope);
    config.js_namespace = js_namespace.map(|s| s.to_string());

    let extern_block = generate_extern_block(&config);
    let factory = generate_dictionary_factory(&config);

    quote! {
        #extern_block
        #factory
    }
}

/// Generate a Rust `impl` block with factory constructors for a dictionary interface.
///
/// Produces `new()` plus expanded variants like `new_with_status(status: f64)`,
/// `new_with_status_and_status_text(status: f64, status_text: &str)`, etc.
/// Each factory creates a bare `Object`, sets the provided properties via their
/// setters, and returns it cast to the dictionary type.
/// Generate a Rust `impl` block with `new()` and `builder()` for a dictionary interface.
///
/// Produces:
/// ```ignore
/// impl ResponseInit {
///     pub fn new() -> Self { ... }
///     pub fn builder() -> ResponseInitBuilder { ... }
/// }
///
/// pub struct ResponseInitBuilder { inner: ResponseInit }
/// impl ResponseInitBuilder {
///     pub fn status(self, val: f64) -> Self { ... }
///     pub fn headers(self, val: &Headers) -> Self { ... }
///     pub fn build(self) -> ResponseInit { ... }
/// }
/// ```
fn generate_dictionary_factory(config: &ClassConfig) -> TokenStream {
    let rust_type = super::typemap::make_ident(&config.rust_name);
    let builder_name = super::typemap::make_ident(&format!("{}Builder", config.rust_name));

    // If any getter lacks a corresponding setter the type has readonly
    // properties, which means it is not constructible via setters — skip
    // the builder entirely and only emit a bare `new()`.
    let setter_names: std::collections::HashSet<&str> = config
        .members
        .iter()
        .filter_map(|m| {
            if let Member::Setter(s) = m {
                Some(s.js_name.as_str())
            } else {
                None
            }
        })
        .collect();
    let has_readonly = config.members.iter().any(|m| {
        if let Member::Getter(g) = m {
            !setter_names.contains(g.js_name.as_str())
        } else {
            false
        }
    });
    if has_readonly {
        return quote! {
            impl #rust_type {
                #[allow(clippy::new_without_default)]
                pub fn new() -> Self {
                    #[allow(unused_unsafe)]
                    unsafe { JsValue::from(js_sys::Object::new()).unchecked_into() }
                }
            }
        };
    }

    // Collect getter properties for builder methods
    let getters: Vec<&crate::ir::GetterMember> = config
        .members
        .iter()
        .filter_map(|m| {
            if let Member::Getter(g) = m {
                Some(g)
            } else {
                None
            }
        })
        .collect();

    // Identify required properties (non-optional getters) and assign bitmask positions
    let required_props: Vec<(usize, &str)> = getters
        .iter()
        .enumerate()
        .filter(|(_, g)| !g.optional)
        .take(64) // u64 bitmask supports up to 64 required properties
        .map(|(i, g)| (i, g.js_name.as_str()))
        .collect();
    let has_required = !required_props.is_empty();

    // Build a map: getter index → bitmask bit (only for required props)
    let mut required_bit: Vec<Option<u64>> = vec![None; getters.len()];
    for (bit, &(getter_idx, _)) in required_props.iter().enumerate() {
        required_bit[getter_idx] = Some(bit as u64);
    }
    let full_mask: u64 = if required_props.len() >= 64 {
        u64::MAX
    } else {
        (1u64 << required_props.len()) - 1
    };

    // Generate builder setter methods
    let mut builder_methods = Vec::new();

    for (getter_idx, g) in getters.iter().enumerate() {
        let setter_param = crate::ir::Param {
            name: "val".to_string(),
            type_ref: g.type_ref.clone(),
            optional: false,
            variadic: false,
        };

        let mut setter_used = HashSet::new();
        let setter_sigs = expand_signatures(
            &g.js_name,
            &[&[setter_param]],
            &crate::ir::TypeRef::Void,
            SignatureKind::Setter,
            &None,
            &mut setter_used,
            config.cgctx,
            config.scope,
        );

        let bit_clear = required_bit[getter_idx].map(|bit| {
            let mask = !(1u64 << bit);
            quote! { self.required &= #mask; }
        });

        for sig in &setter_sigs {
            let builder_method_name = sig.rust_name.strip_prefix("set_").unwrap_or(&sig.rust_name);
            let method_ident = super::typemap::make_ident(builder_method_name);
            let setter_ident = super::typemap::make_ident(&sig.rust_name);
            let params = generate_concrete_params(&sig.params, config.cgctx, config.scope);

            let param_idents: Vec<_> = sig
                .params
                .iter()
                .map(|p| super::typemap::make_ident(&p.name))
                .collect();

            builder_methods.push(quote! {
                pub fn #method_ident(mut self, #params) -> Self {
                    self.inner.#setter_ident(#(#param_idents),*);
                    #bit_clear
                    self
                }
            });
        }
    }

    // Build method: infallible if no required props, Result if there are
    let build_method = if has_required {
        let missing_checks: Vec<TokenStream> = required_props
            .iter()
            .enumerate()
            .map(|(bit, (_, name))| {
                let mask = 1u64 << bit;
                let msg = format!("missing required property `{name}`");
                quote! {
                    if self.required & #mask != 0 {
                        missing.push(#msg);
                    }
                }
            })
            .collect();

        quote! {
            pub fn build(self) -> Result<#rust_type, JsValue> {
                if self.required != 0 {
                    let mut missing = Vec::new();
                    #(#missing_checks)*
                    return Err(JsValue::from_str(&format!(
                        "{}: {}", stringify!(#rust_type), missing.join(", ")
                    )));
                }
                Ok(self.inner)
            }
        }
    } else {
        quote! {
            pub fn build(self) -> #rust_type {
                self.inner
            }
        }
    };

    // Builder struct: with or without required bitmask
    let builder_struct = if has_required {
        quote! {
            pub struct #builder_name {
                inner: #rust_type,
                required: u64,
            }
        }
    } else {
        quote! {
            pub struct #builder_name {
                inner: #rust_type,
            }
        }
    };

    let builder_init = if has_required {
        quote! { #builder_name { inner: Self::new(), required: #full_mask } }
    } else {
        quote! { #builder_name { inner: Self::new() } }
    };

    quote! {
        impl #rust_type {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                #[allow(unused_imports)]
                use wasm_bindgen::JsCast;
                JsCast::unchecked_into(js_sys::Object::new())
            }

            pub fn builder() -> #builder_name {
                #builder_init
            }
        }

        #builder_struct

        #[allow(unused_mut)]
        impl #builder_name {
            #(#builder_methods)*

            #build_method
        }
    }
}

/// Build the full `#[wasm_bindgen] extern "C" { ... }` block.
///
/// All naming happens through a single `used_names` set that spans the entire
/// extern block. Members are processed in declaration order. Methods with the
/// same `js_name` (TypeScript overloads) are grouped and expanded together as
/// one unit — overloads feed into the same expansion, producing disambiguated
/// `_with_`/`_and_` suffixes across all overloads rather than opaque `_1` suffixes.
///
/// Each name — including `try_` variants — is assigned via `dedupe_name`, which
/// guarantees uniqueness by appending numeric suffixes on collision.
fn generate_extern_block(config: &ClassConfig) -> TokenStream {
    use crate::ir::{ConstructorMember, MethodMember, Param, StaticMethodMember};
    use std::collections::HashMap;

    let mut items = Vec::new();
    let mut used_names: HashSet<String> = HashSet::new();

    // Pre-group methods/statics/constructors by js_name.
    // We iterate config.members in declaration order, so the first occurrence of
    // each js_name determines where its expanded signatures appear in the output.
    let mut method_groups: HashMap<String, Vec<&MethodMember>> = HashMap::new();
    let mut static_method_groups: HashMap<String, Vec<&StaticMethodMember>> = HashMap::new();
    let mut constructor_overloads: Vec<&ConstructorMember> = Vec::new();

    for member in &config.members {
        match member {
            Member::Constructor(ctor) if !config.is_abstract => {
                constructor_overloads.push(ctor);
            }
            Member::Method(m) => {
                method_groups.entry(m.js_name.clone()).or_default().push(m);
            }
            Member::StaticMethod(m) => {
                static_method_groups
                    .entry(m.js_name.clone())
                    .or_default()
                    .push(m);
            }
            _ => {}
        }
    }

    // Track which method groups have been expanded (by js_name).
    let mut expanded_methods: HashSet<String> = HashSet::new();
    let mut expanded_static_methods: HashSet<String> = HashSet::new();
    let mut expanded_constructors = false;

    // 1. Type declaration with attributes
    items.push(generate_type_decl(config));

    // 2. Process all members in declaration order through the single naming pass.
    //    When we encounter the first member of a method group, expand all overloads
    //    of that group together. Skip subsequent members of the same group.
    for member in &config.members {
        match member {
            Member::Constructor(_) if !config.is_abstract => {
                if expanded_constructors {
                    continue;
                }
                expanded_constructors = true;

                let overloads: Vec<&[Param]> = constructor_overloads
                    .iter()
                    .map(|c| c.params.as_slice())
                    .collect();
                let doc = constructor_overloads.first().and_then(|c| c.doc.clone());
                let sigs = expand_signatures(
                    &config.js_name,
                    &overloads,
                    &TypeRef::Named(config.rust_name.clone()),
                    SignatureKind::Constructor,
                    &doc,
                    &mut used_names,
                    config.cgctx,
                    config.scope,
                );
                for sig in &sigs {
                    items.push(generate_expanded_constructor(config, sig));
                }
            }
            Member::Method(m) => {
                if expanded_methods.contains(&m.js_name) {
                    continue;
                }
                expanded_methods.insert(m.js_name.clone());

                let group = &method_groups[&m.js_name];
                let overloads: Vec<&[Param]> = group.iter().map(|m| m.params.as_slice()).collect();
                let doc = group.first().and_then(|m| m.doc.clone());
                let return_type = &group[0].return_type;
                let sigs = expand_signatures(
                    &m.js_name,
                    &overloads,
                    return_type,
                    SignatureKind::Method,
                    &doc,
                    &mut used_names,
                    config.cgctx,
                    config.scope,
                );
                for sig in &sigs {
                    items.push(generate_expanded_method(config, sig));
                }
            }
            Member::StaticMethod(m) => {
                if expanded_static_methods.contains(&m.js_name) {
                    continue;
                }
                expanded_static_methods.insert(m.js_name.clone());

                let group = &static_method_groups[&m.js_name];
                let overloads: Vec<&[Param]> = group.iter().map(|m| m.params.as_slice()).collect();
                let doc = group.first().and_then(|m| m.doc.clone());
                let return_type = &group[0].return_type;
                let sigs = expand_signatures(
                    &m.js_name,
                    &overloads,
                    return_type,
                    SignatureKind::StaticMethod,
                    &doc,
                    &mut used_names,
                    config.cgctx,
                    config.scope,
                );
                for sig in &sigs {
                    items.push(generate_expanded_static_method(config, sig));
                }
            }
            Member::Getter(g) => {
                items.push(generate_getter(config, g, &mut used_names));
            }
            Member::Setter(s) => {
                items.extend(generate_setter(config, s, &mut used_names));
            }
            Member::StaticGetter(g) => {
                items.push(generate_static_getter(config, g, &mut used_names));
            }
            Member::StaticSetter(s) => {
                items.extend(generate_static_setter(config, s, &mut used_names));
            }
            Member::IndexSignature(_) | Member::Constructor(_) => {
                // IndexSignature: not yet supported in codegen
                // Constructor on abstract class: skip
            }
        }
    }

    // Build the extern block with optional module attribute
    let wb_extern_attr = match &config.module {
        Some(m) => quote! { #[wasm_bindgen(module = #m)] },
        None => quote! { #[wasm_bindgen] },
    };

    quote! {
        #wb_extern_attr
        extern "C" {
            #(#items)*
        }
    }
}

/// Generate the type declaration:
///
/// ```rust,ignore
/// #[wasm_bindgen(extends = ..., js_name = "FooBar")]
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub type FooBar;
/// ```
fn generate_type_decl(config: &ClassConfig) -> TokenStream {
    let rust_ident = super::typemap::make_ident(&config.rust_name);
    let js_name = &config.js_name;

    // Build wasm_bindgen attribute parts
    let mut wb_parts: Vec<TokenStream> = Vec::new();

    let mut has_object = false;
    for extends in &config.extends {
        let extends_str = extends.to_string();
        // Skip extends that resolve to JsValue (implicit, causes conflicting impls)
        if extends_str == "JsValue" {
            continue;
        }
        if let Some(cgctx) = config.cgctx {
            let uses = cgctx.external_uses.borrow();
            if uses.get(&extends_str).is_some_and(|v| v == "JsValue") {
                continue;
            }
        }
        if extends_str == "Object" {
            has_object = true;
        }
        wb_parts.push(quote! { extends = #extends });
    }
    // Every type extends Object at minimum
    if !has_object {
        wb_parts.push(quote! { extends = Object });
    }

    // Only emit js_name if it differs from the Rust name
    if config.js_name != config.rust_name {
        wb_parts.push(quote! { js_name = #js_name });
    }

    // Namespace for types inside a JS namespace (e.g., WebAssembly.Module)
    if let Some(ns) = &config.js_namespace {
        wb_parts.push(quote! { js_namespace = #ns });
    }

    let wb_attr = if wb_parts.is_empty() {
        quote! {}
    } else {
        quote! { #[wasm_bindgen(#(#wb_parts),*)] }
    };

    quote! {
        #wb_attr
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type #rust_ident;
    }
}

/// Generate a constructor binding from an expanded signature.
fn generate_expanded_constructor(config: &ClassConfig, sig: &ExpandedSignature) -> TokenStream {
    let rust_ident = super::typemap::make_ident(&sig.rust_name);
    let rust_type = super::typemap::make_ident(&config.rust_name);
    let params = generate_concrete_params(&sig.params, config.cgctx, config.scope);
    let doc = super::doc_tokens(&sig.doc);

    let ret = if sig.catch {
        quote! { Result<#rust_type, JsValue> }
    } else {
        quote! { #rust_type }
    };

    let mut wb_parts = vec![quote! { constructor }];
    if sig.catch {
        wb_parts.push(quote! { catch });
    }
    // For non-"new" overloads, we need js_name so wasm_bindgen maps them
    // to the same JS constructor.
    if sig.rust_name != "new" {
        let js_name = &config.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }

    quote! {
        #doc
        #[wasm_bindgen(#(#wb_parts),*)]
        pub fn #rust_ident(#params) -> #ret;
    }
}

/// Generate an instance method binding from an expanded signature.
fn generate_expanded_method(config: &ClassConfig, sig: &ExpandedSignature) -> TokenStream {
    let rust_ident = super::typemap::make_ident(&sig.rust_name);
    let this_type = super::typemap::make_ident(&config.rust_name);
    let params = generate_concrete_params(&sig.params, config.cgctx, config.scope);
    let doc = super::doc_tokens(&sig.doc);
    let has_variadic = sig.params.last().is_some_and(|p| p.variadic);

    let mut wb_parts: Vec<TokenStream> = vec![quote! { method }];
    if has_variadic {
        wb_parts.push(quote! { variadic });
    }
    if sig.catch {
        wb_parts.push(quote! { catch });
    }
    // Emit js_name when the JS name differs from the Rust name.
    if sig.rust_name != sig.js_name {
        let js_name = &sig.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }

    let ret_ty = to_return_type(&sig.return_type, sig.catch, config.cgctx, config.scope);
    let ret = if is_void_return(&sig.return_type) && !sig.catch {
        quote! {}
    } else {
        quote! { -> #ret_ty }
    };

    quote! {
        #doc
        #[wasm_bindgen(#(#wb_parts),*)]
        pub fn #rust_ident(this: &#this_type, #params) #ret;
    }
}

/// Generate a static method binding from an expanded signature.
fn generate_expanded_static_method(config: &ClassConfig, sig: &ExpandedSignature) -> TokenStream {
    let rust_ident = super::typemap::make_ident(&sig.rust_name);
    let class_ident = super::typemap::make_ident(&config.rust_name);
    let params = generate_concrete_params(&sig.params, config.cgctx, config.scope);
    let doc = super::doc_tokens(&sig.doc);
    let has_variadic = sig.params.last().is_some_and(|p| p.variadic);

    let mut wb_parts: Vec<TokenStream> = vec![quote! { static_method_of = #class_ident }];
    if has_variadic {
        wb_parts.push(quote! { variadic });
    }
    if sig.catch {
        wb_parts.push(quote! { catch });
    }
    if sig.rust_name != sig.js_name {
        let js_name = &sig.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }

    let ret_ty = to_return_type(&sig.return_type, sig.catch, config.cgctx, config.scope);
    let ret = if is_void_return(&sig.return_type) && !sig.catch {
        quote! {}
    } else {
        quote! { -> #ret_ty }
    };

    quote! {
        #doc
        #[wasm_bindgen(#(#wb_parts),*)]
        pub fn #rust_ident(#params) #ret;
    }
}

/// Generate an instance getter binding.
fn generate_getter(
    config: &ClassConfig,
    getter: &GetterMember,
    used_names: &mut HashSet<String>,
) -> TokenStream {
    let this_type = super::typemap::make_ident(&config.rust_name);
    let doc = super::doc_tokens(&getter.doc);

    let candidate = to_snake_case(&getter.js_name);
    let rust_name = dedupe_name(&candidate, used_names);
    let rust_ident = super::typemap::make_ident(&rust_name);

    let getter_type = if getter.optional {
        // Unwrap Nullable to avoid Option<Option<T>> — the optionality from `?`
        // already provides the outer Option.
        let unwrapped = match &getter.type_ref {
            TypeRef::Nullable(inner) => inner.as_ref(),
            other => other,
        };
        let inner = to_syn_type(unwrapped, TypePosition::RETURN, config.cgctx, config.scope);
        quote! { Option<#inner> }
    } else {
        to_syn_type(
            &getter.type_ref,
            TypePosition::RETURN,
            config.cgctx,
            config.scope,
        )
    };

    let mut wb_parts: Vec<TokenStream> = vec![quote! { method }, quote! { getter }];
    if rust_name != getter.js_name {
        let js_name = &getter.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }

    quote! {
        #doc
        #[wasm_bindgen(#(#wb_parts),*)]
        pub fn #rust_ident(this: &#this_type) -> #getter_type;
    }
}

/// Generate instance setter bindings, expanding union types into separate overloads.
fn generate_setter(
    config: &ClassConfig,
    setter: &SetterMember,
    used_names: &mut HashSet<String>,
) -> Vec<TokenStream> {
    let this_type = super::typemap::make_ident(&config.rust_name);
    let doc = setter.doc.clone();

    // Treat the setter as a single-param method and expand through signatures
    let param = crate::ir::Param {
        name: "val".to_string(),
        type_ref: setter.type_ref.clone(),
        optional: false,
        variadic: false,
    };

    let sigs = expand_signatures(
        &setter.js_name,
        &[&[param]],
        &crate::ir::TypeRef::Void,
        SignatureKind::Setter,
        &doc,
        used_names,
        config.cgctx,
        config.scope,
    );

    sigs.iter()
        .map(|sig| {
            let rust_ident = super::typemap::make_ident(&sig.rust_name);
            let params = generate_concrete_params(&sig.params, config.cgctx, config.scope);

            let mut wb_parts: Vec<TokenStream> = vec![quote! { method }, quote! { setter }];
            if sig.rust_name != format!("set_{}", setter.js_name) {
                let js_name = &setter.js_name;
                wb_parts.push(quote! { js_name = #js_name });
            }

            let doc = super::doc_tokens(&sig.doc);
            quote! {
                #doc
                #[wasm_bindgen(#(#wb_parts),*)]
                pub fn #rust_ident(this: &#this_type, #params);
            }
        })
        .collect()
}

/// Generate a static getter binding.
fn generate_static_getter(
    config: &ClassConfig,
    getter: &StaticGetterMember,
    used_names: &mut HashSet<String>,
) -> TokenStream {
    let class_ident = super::typemap::make_ident(&config.rust_name);
    let doc = super::doc_tokens(&getter.doc);

    let candidate = to_snake_case(&getter.js_name);
    let rust_name = dedupe_name(&candidate, used_names);
    let rust_ident = super::typemap::make_ident(&rust_name);

    let getter_type = to_syn_type(
        &getter.type_ref,
        TypePosition::RETURN,
        config.cgctx,
        config.scope,
    );

    let mut wb_parts: Vec<TokenStream> = vec![
        quote! { static_method_of = #class_ident },
        quote! { getter },
    ];
    if rust_name != getter.js_name {
        let js_name = &getter.js_name;
        wb_parts.push(quote! { js_name = #js_name });
    }

    quote! {
        #doc
        #[wasm_bindgen(#(#wb_parts),*)]
        pub fn #rust_ident() -> #getter_type;
    }
}

/// Generate static setter bindings, expanding union types into separate overloads.
fn generate_static_setter(
    config: &ClassConfig,
    setter: &StaticSetterMember,
    used_names: &mut HashSet<String>,
) -> Vec<TokenStream> {
    let class_ident = super::typemap::make_ident(&config.rust_name);
    let doc = setter.doc.clone();

    let param = crate::ir::Param {
        name: "val".to_string(),
        type_ref: setter.type_ref.clone(),
        optional: false,
        variadic: false,
    };

    let sigs = expand_signatures(
        &setter.js_name,
        &[&[param]],
        &crate::ir::TypeRef::Void,
        SignatureKind::StaticSetter,
        &doc,
        used_names,
        config.cgctx,
        config.scope,
    );

    sigs.iter()
        .map(|sig| {
            let rust_ident = super::typemap::make_ident(&sig.rust_name);
            let params = generate_concrete_params(&sig.params, config.cgctx, config.scope);

            let mut wb_parts: Vec<TokenStream> = vec![
                quote! { static_method_of = #class_ident },
                quote! { setter },
            ];
            if sig.rust_name != format!("set_{}", setter.js_name) {
                let js_name = &setter.js_name;
                wb_parts.push(quote! { js_name = #js_name });
            }

            let doc = super::doc_tokens(&sig.doc);
            quote! {
                #doc
                #[wasm_bindgen(#(#wb_parts),*)]
                pub fn #rust_ident(#params);
            }
        })
        .collect()
}

// ─── Helpers ─────────────────────────────────────────────────────────

/// Convert concrete params to `fn` parameter token stream.
/// Convert a `TypeRef` representing an extends target into tokens for
/// the `extends = ...` attribute.
///
/// Falls back to `Object` for unresolved types — `extends = JsValue` is
/// never useful (it's implicit and causes conflicting trait impls).
fn extends_tokens(ty: &TypeRef, cgctx: Option<&CodegenContext<'_>>, scope: ScopeId) -> TokenStream {
    let tokens = match ty {
        TypeRef::Named(_) | TypeRef::GenericInstantiation(_, _) => {
            super::typemap::to_syn_type(ty, TypePosition::ARGUMENT.to_inner(), cgctx, scope)
        }
        _ => {
            if let Some(ctx) = cgctx {
                ctx.warn(format!(
                    "unsupported extends type `{ty:?}`, falling back to Object"
                ));
            }
            quote! { Object }
        }
    };
    // JsValue is the root of all wasm_bindgen types — extending it is
    // implicit, so fall back to Object (which is always safe).
    if tokens.to_string() == "JsValue" {
        quote! { Object }
    } else {
        tokens
    }
}
