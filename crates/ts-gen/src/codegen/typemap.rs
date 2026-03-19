//! TypeRef → syn::Type mapping with unified position-based system.
//!
//! Follows the wasm-bindgen WebIDL approach: a single `to_syn_type` function
//! that uses `TypePosition` to determine how types are lowered to Rust.
//!
//! `TypePosition` is a struct with two fields:
//! - `direction`: `Argument` or `Return` — controls borrowing (e.g., `&str` vs `String`)
//! - `inner`: whether we're nested inside a generic container (e.g., `Promise<T>`)
//!
//! When `inner` is true:
//! - Primitives map to JS wrapper types (`Number`, `JsString`, `Boolean`, `Undefined`)
//! - `Nullable` becomes `JsOption<T>` instead of `Option<T>`
//! - Argument-position types are NOT borrowed (owned `T`, not `&T`)

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

use crate::context::GlobalContext;
use crate::ir::{self, TypeKind, TypeRef};
use crate::parse::scope::ScopeId;
use crate::util::diagnostics::DiagnosticCollector;

/// js_sys type names reserved by the `use js_sys::*` glob import.
/// User-defined types that collide with these will be renamed.
pub const JS_SYS_RESERVED: &[&str] = &[
    "Array",
    "ArrayBuffer",
    "ArrayTuple",
    "AsyncGenerator",
    "AsyncIterator",
    "BigInt",
    "BigInt64Array",
    "BigUint64Array",
    "Boolean",
    "DataView",
    "Date",
    "Error",
    "EvalError",
    "Float32Array",
    "Float64Array",
    "Function",
    "Generator",
    "Global",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Iterator",
    "IteratorNext",
    "JsOption",
    "JsString",
    "Map",
    "Number",
    "Object",
    "Promise",
    "Proxy",
    "RangeError",
    "ReferenceError",
    "RegExp",
    "Set",
    "SharedArrayBuffer",
    "Symbol",
    "SyntaxError",
    "TypeError",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Undefined",
    "UriError",
    "WeakMap",
    "WeakRef",
    "WeakSet",
];

/// Direction of data flow at the FFI boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Data flowing from Rust to JS (function arguments).
    Argument,
    /// Data flowing from JS to Rust (function returns).
    Return,
}

/// Position context for type mapping, following the wasm-bindgen WebIDL pattern.
///
/// Combines a direction (Argument/Return) with an inner flag indicating
/// whether we're inside a generic container. When `inner` is true,
/// primitives use their JS wrapper types and nullable uses `JsOption`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypePosition {
    pub direction: Direction,
    /// Whether this type is nested inside a generic or callback.
    /// When true, must use JS-compatible wrapper types.
    pub inner: bool,
}

impl TypePosition {
    /// Top-level function argument position.
    pub const ARGUMENT: Self = Self {
        direction: Direction::Argument,
        inner: false,
    };
    /// Top-level function return position.
    pub const RETURN: Self = Self {
        direction: Direction::Return,
        inner: false,
    };

    /// Convert to inner position (for generic type parameters).
    /// Preserves direction but sets `inner: true`.
    pub fn to_inner(self) -> Self {
        Self {
            direction: self.direction,
            inner: true,
        }
    }

    pub fn is_argument(self) -> bool {
        matches!(self.direction, Direction::Argument)
    }
}

/// Context for codegen that tracks locally-defined types and resolved type aliases.
///
/// This allows `to_syn_type` to distinguish between locally-generated types
/// and types that should resolve via `use js_sys::*`.
pub struct CodegenContext<'a> {
    /// Read-only access to the global context (scopes, modules, external map).
    pub gctx: &'a GlobalContext,
    /// Set of type names defined in this codegen unit (classes, interfaces, enums, etc.).
    pub local_types: HashSet<String>,
    /// Type aliases whose target is a union or other non-representable type.

    /// Local types that collide with js_sys reserved names — maps original name → renamed name.
    pub renamed_locals: HashMap<String, String>,
    /// Builtin (root) scope id.
    pub root_scope: ScopeId,
    /// Per-file scopes (children of root, contain imports + local types).
    pub file_scopes: Vec<ScopeId>,
    /// External type use aliases collected during codegen: (local_name, rust_path).
    pub external_uses: RefCell<HashMap<String, String>>,
    /// Diagnostics collected during code generation.
    pub diagnostics: RefCell<DiagnosticCollector>,
}

impl<'a> CodegenContext<'a> {
    /// Build a `CodegenContext` from a parsed IR module + global context.
    pub fn from_module(module: &ir::Module, gctx: &'a GlobalContext) -> Self {
        let mut ctx = CodegenContext {
            gctx,
            local_types: HashSet::new(),
            renamed_locals: HashMap::new(),
            root_scope: module.builtin_scope,
            file_scopes: module.file_scopes.clone(),
            external_uses: RefCell::new(HashMap::new()),
            diagnostics: RefCell::new(DiagnosticCollector::new()),
        };
        for &type_id in &module.types {
            let decl = gctx.get_type(type_id);
            ctx.collect_declaration(&decl.kind);
        }
        ctx.resolve_collisions();
        ctx
    }

    /// Create an empty context (for tests). Requires a valid root scope.
    pub fn empty(gctx: &'a GlobalContext, root_scope: ScopeId) -> Self {
        CodegenContext {
            gctx,
            local_types: HashSet::new(),
            renamed_locals: HashMap::new(),
            root_scope,
            file_scopes: vec![],
            external_uses: RefCell::new(HashMap::new()),
            diagnostics: RefCell::new(DiagnosticCollector::new()),
        }
    }

    /// Register an external type use alias.
    /// Returns the local name to use in generated code.
    fn register_external(&self, local_name: &str, rust_path: &str) {
        self.external_uses
            .borrow_mut()
            .insert(local_name.to_string(), rust_path.to_string());
    }

    /// Generate `use` statements for all external type aliases.
    pub fn external_use_tokens(&self) -> TokenStream {
        let uses = self.external_uses.borrow();
        let mut entries: Vec<_> = uses.iter().collect();
        entries.sort_by_key(|(name, _)| (*name).clone());

        let stmts: Vec<TokenStream> = entries
            .into_iter()
            .map(|(local_name, rust_path)| {
                let local_ident = make_ident(local_name);
                // Parse the rust path into tokens
                let path: TokenStream = rust_path.parse().unwrap_or_else(|_| {
                    // Fallback: just use JsValue
                    quote! { JsValue }
                });
                if rust_path == "JsValue" || rust_path.ends_with("::JsValue") {
                    // JsValue fallback: use JsValue as LocalName
                    quote! { #[allow(dead_code)] use JsValue as #local_ident; }
                } else {
                    quote! { #[allow(dead_code)] use #path as #local_ident; }
                }
            })
            .collect();

        quote! { #(#stmts)* }
    }

    /// Resolve an external type through the external map.
    pub fn resolve_external(
        &self,
        type_name: &str,
        from_module: &str,
    ) -> Option<crate::external_map::RustPath> {
        self.gctx.external_map.resolve(type_name, from_module)
    }

    /// Resolve a named type through the scope chain, chasing the full alias chain
    /// until a non-alias terminal type is reached.
    ///
    /// Returns the final `TypeRef` target if the name resolves to a type alias
    /// (or a chain of aliases). Returns `None` if the name resolves to a
    /// non-alias declaration (Class, Interface, Enum, etc.) or is not found.
    ///
    /// Uses a visited set to detect and break circular alias chains.
    pub fn resolve_alias(&self, name: &str, scope: ScopeId) -> Option<&ir::TypeRef> {
        let mut visited = HashSet::new();
        self.resolve_alias_impl(name, scope, &mut visited)
    }

    fn resolve_alias_impl<'b>(
        &'b self,
        name: &str,
        scope: ScopeId,
        visited: &mut HashSet<String>,
    ) -> Option<&'b ir::TypeRef> {
        if !visited.insert(name.to_string()) {
            return None; // circular alias chain
        }
        if let Some(type_id) = self.gctx.scopes.resolve(scope, name) {
            let decl = self.gctx.get_type(type_id);
            if let TypeKind::TypeAlias(ref alias) = decl.kind {
                // If the target is itself a named reference, keep resolving.
                if let ir::TypeRef::Named(ref inner_name) = alias.target {
                    if let Some(resolved) = self.resolve_alias_impl(inner_name, scope, visited) {
                        return Some(resolved);
                    }
                }
                return Some(&alias.target);
            }
        }
        None
    }

    /// Emit an error diagnostic during code generation.
    pub fn error(&self, message: impl Into<String>) {
        self.diagnostics.borrow_mut().error(message);
    }

    /// Emit a warning diagnostic during code generation.
    pub fn warn(&self, message: impl Into<String>) {
        self.diagnostics.borrow_mut().warn(message);
    }

    /// Take ownership of the collected diagnostics.
    pub fn take_diagnostics(&self) -> DiagnosticCollector {
        self.diagnostics.take()
    }

    /// Detect collisions between local type names and the js_sys glob import.
    /// Colliding local types get renamed with a trailing underscore.
    fn resolve_collisions(&mut self) {
        let reserved: HashSet<&str> = JS_SYS_RESERVED.iter().copied().collect();

        for name in &reserved {
            if self.local_types.contains(*name) {
                let mut renamed = format!("{name}_");
                let mut i = 2;
                while self.local_types.contains(&renamed) || reserved.contains(renamed.as_str()) {
                    renamed = format!("{name}_{i}");
                    i += 1;
                }
                self.renamed_locals.insert(name.to_string(), renamed);
            }
        }
    }

    fn collect_declaration(&mut self, kind: &ir::TypeKind) {
        match kind {
            ir::TypeKind::Class(c) => {
                self.local_types.insert(c.name.clone());
            }
            ir::TypeKind::Interface(i) => {
                self.local_types.insert(i.name.clone());
            }
            ir::TypeKind::StringEnum(e) => {
                self.local_types.insert(e.name.clone());
            }
            ir::TypeKind::NumericEnum(e) => {
                self.local_types.insert(e.name.clone());
            }
            ir::TypeKind::TypeAlias(_) => {
                // Type aliases are resolved through the scope during codegen.
                // No special collection needed.
            }
            ir::TypeKind::Namespace(ns) => {
                for inner in &ns.declarations {
                    self.collect_declaration(&inner.kind);
                }
            }
            ir::TypeKind::Function(_) | ir::TypeKind::Variable(_) => {}
        }
    }
}

/// Map an IR `TypeRef` to a `proc_macro2::TokenStream` representing the Rust type.
///
/// This is the unified type mapping function, following the wasm-bindgen WebIDL
/// `to_syn_type` pattern. A single function handles all positions:
///
/// - When `pos.inner` is true, primitives become JS wrapper types
///   (`Number`, `JsString`, `Boolean`, `Undefined`), nullable becomes `JsOption`,
///   and argument-position types are NOT borrowed.
/// - When `pos.inner` is false, standard Rust types are used (`f64`, `&str`/`String`,
///   `bool`, `()`), nullable becomes `Option<T>`, and argument-position types
///   may be borrowed.
pub fn to_syn_type(
    ty: &TypeRef,
    pos: TypePosition,
    ctx: Option<&CodegenContext<'_>>,
    scope: ScopeId,
) -> TokenStream {
    // When inner, intercept primitives and nullable early to use JS wrapper forms
    if pos.inner {
        match ty {
            TypeRef::Boolean | TypeRef::BooleanLiteral(_) => return quote! { Boolean },
            TypeRef::Number | TypeRef::NumberLiteral(_) => return quote! { Number },
            TypeRef::String | TypeRef::StringLiteral(_) => return quote! { JsString },
            TypeRef::Void | TypeRef::Undefined => return quote! { Undefined },
            TypeRef::Nullable(inner) => {
                let inner_ty = to_syn_type(inner, pos, ctx, scope);
                return quote! { JsOption<#inner_ty> };
            }
            _ => {}
        }
    }

    // Helper: should this type get `&` in argument position?
    // Returns true for all JS/non-Rust types (anything that crosses the FFI boundary
    // as a wasm-bindgen reference). Rust-native primitives (bool, f64, ()) do NOT get `&`.
    let borrow = pos.is_argument() && !pos.inner;

    match ty {
        // === Primitives (outer position only reaches here) ===
        TypeRef::Boolean => quote! { bool },
        TypeRef::Number => quote! { f64 },
        TypeRef::String => {
            if borrow {
                quote! { &str }
            } else {
                quote! { String }
            }
        }
        TypeRef::BigInt => maybe_ref(quote! { BigInt }, borrow),
        TypeRef::Void => quote! { () },
        TypeRef::Undefined => maybe_ref(quote! { Undefined }, borrow),
        TypeRef::Null => maybe_ref(quote! { Null }, borrow),
        TypeRef::Any => maybe_ref(quote! { JsValue }, borrow),
        TypeRef::Unknown => maybe_ref(quote! { JsValue }, borrow),
        TypeRef::Object => maybe_ref(quote! { Object }, borrow),
        TypeRef::Symbol => maybe_ref(quote! { JsValue }, borrow),

        // === Typed Arrays ===
        TypeRef::Int8Array => maybe_ref(quote! { Int8Array }, borrow),
        TypeRef::Uint8Array => maybe_ref(quote! { Uint8Array }, borrow),
        TypeRef::Uint8ClampedArray => maybe_ref(quote! { Uint8ClampedArray }, borrow),
        TypeRef::Int16Array => maybe_ref(quote! { Int16Array }, borrow),
        TypeRef::Uint16Array => maybe_ref(quote! { Uint16Array }, borrow),
        TypeRef::Int32Array => maybe_ref(quote! { Int32Array }, borrow),
        TypeRef::Uint32Array => maybe_ref(quote! { Uint32Array }, borrow),
        TypeRef::Float32Array => maybe_ref(quote! { Float32Array }, borrow),
        TypeRef::Float64Array => maybe_ref(quote! { Float64Array }, borrow),
        TypeRef::BigInt64Array => maybe_ref(quote! { BigInt64Array }, borrow),
        TypeRef::BigUint64Array => maybe_ref(quote! { BigUint64Array }, borrow),
        TypeRef::ArrayBuffer => maybe_ref(quote! { ArrayBuffer }, borrow),
        TypeRef::ArrayBufferView => maybe_ref(quote! { Object }, borrow),
        TypeRef::DataView => maybe_ref(quote! { DataView }, borrow),

        // === Built-in Generic Containers ===
        TypeRef::Promise(inner) => maybe_ref(
            generic_container(quote! { Promise }, inner, pos, ctx, scope),
            borrow,
        ),
        TypeRef::Array(inner) => maybe_ref(
            generic_container(quote! { Array }, inner, pos, ctx, scope),
            borrow,
        ),
        TypeRef::Record(_k, v) => maybe_ref(
            generic_container(quote! { Object }, v, pos, ctx, scope),
            borrow,
        ),
        TypeRef::Map(k, v) => {
            let inner_pos = pos.to_inner();
            let k_arg = to_syn_type(k, inner_pos, ctx, scope);
            let v_arg = to_syn_type(v, inner_pos, ctx, scope);
            let base = if is_jsvalue_arg(&k_arg) && is_jsvalue_arg(&v_arg) {
                quote! { Map }
            } else {
                quote! { Map<#k_arg, #v_arg> }
            };
            maybe_ref(base, borrow)
        }
        TypeRef::Set(inner) => maybe_ref(
            generic_container(quote! { Set }, inner, pos, ctx, scope),
            borrow,
        ),

        // === Structural Types ===
        TypeRef::Nullable(inner) => {
            if pos.inner {
                let inner_ty = to_syn_type(inner, pos, ctx, scope);
                quote! { JsOption<#inner_ty> }
            } else {
                let inner_ty = to_syn_type(inner, pos, ctx, scope);
                quote! { Option<#inner_ty> }
            }
        }
        TypeRef::Union(_) => maybe_ref(quote! { JsValue }, borrow),
        TypeRef::Intersection(_) => maybe_ref(quote! { JsValue }, borrow),
        TypeRef::Tuple(elems) => {
            let base = if elems.is_empty() {
                quote! { Array }
            } else {
                let inner_pos = pos.to_inner();
                let elem_types: Vec<TokenStream> = elems
                    .iter()
                    .map(|e| to_syn_type(e, inner_pos, ctx, scope))
                    .collect();
                quote! { ArrayTuple<(#(#elem_types),*)> }
            };
            maybe_ref(base, borrow)
        }
        TypeRef::Function(sig) => {
            let inner_pos = pos.to_inner();
            let params: Vec<TokenStream> = sig
                .params
                .iter()
                .take(8)
                .map(|p| to_syn_type(&p.type_ref, inner_pos, ctx, scope))
                .collect();
            let ret = to_syn_type(&sig.return_type, inner_pos, ctx, scope);
            let base = if params.iter().all(is_jsvalue_arg) && is_jsvalue_arg(&ret) {
                quote! { Function }
            } else {
                quote! { Function<fn(#(#params),*) -> #ret> }
            };
            maybe_ref(base, borrow)
        }

        // === Literal Types ===
        TypeRef::StringLiteral(_) => {
            if borrow {
                quote! { &str }
            } else {
                quote! { String }
            }
        }
        TypeRef::NumberLiteral(_) => quote! { f64 },
        TypeRef::BooleanLiteral(_) => quote! { bool },

        // === Named References ===
        TypeRef::Named(name) => {
            // Resolve through type aliases before falling back to named_type_to_rust.
            if let Some(c) = ctx {
                if let Some(target) = c.resolve_alias(name, scope) {
                    let target = target.clone();
                    return to_syn_type(&target, pos, ctx, scope);
                }
            }
            maybe_ref(named_type_to_rust(name, ctx), borrow)
        }
        TypeRef::GenericInstantiation(name, _args) => {
            // TODO (Phase 3): preserve generic type arguments once wasm_bindgen
            // generic support is wired through. For now, emit just the base type.
            if let Some(c) = ctx {
                c.warn(format!(
                    "generic type arguments on `{name}<...>` are not yet emitted, using bare `{name}`"
                ));
            }
            maybe_ref(named_type_to_rust(name, ctx), borrow)
        }

        // === Special ===
        TypeRef::Date => maybe_ref(quote! { Date }, borrow),
        TypeRef::RegExp => maybe_ref(quote! { RegExp }, borrow),
        TypeRef::Error => maybe_ref(quote! { Error }, borrow),

        // === Fallback ===
        TypeRef::Unresolved(desc) => {
            if let Some(cgctx) = ctx {
                cgctx.warn(format!("unresolved type `{desc}`, falling back to JsValue"));
            }
            maybe_ref(quote! { JsValue }, borrow)
        }
    }
}

/// Wrap a type in `&` when in argument position (the `externref` pattern from wasm-bindgen WebIDL).
///
/// All JS object types (anything that isn't a Rust `Copy` primitive like `bool`/`f64`)
/// are passed by reference in argument position at the top level.
fn maybe_ref(ty: TokenStream, borrow: bool) -> TokenStream {
    if borrow {
        quote! { &#ty }
    } else {
        ty
    }
}

/// Helper: emit `Base<T'>` or just `Base` if T' is JsValue (the default).
fn generic_container(
    base: TokenStream,
    inner: &TypeRef,
    pos: TypePosition,
    ctx: Option<&CodegenContext<'_>>,
    scope: ScopeId,
) -> TokenStream {
    let arg = to_syn_type(inner, pos.to_inner(), ctx, scope);
    if is_jsvalue_arg(&arg) {
        base
    } else {
        quote! { #base<#arg> }
    }
}

/// Check if a generic argument token stream represents `JsValue` (the default).
/// When it is the default, we elide the generic parameter.
fn is_jsvalue_arg(tokens: &TokenStream) -> bool {
    let s = tokens.to_string();
    s == "JsValue"
}

/// Emit a type name as Rust tokens.
///
/// Single unified path for ALL type name emission:
/// 1. Resolve name → TypeId through scope
/// 2. Get canonical name (last segment for dotted paths)
/// 3. Local type (in our output)? → emit directly (with js_sys collision rename)
/// 4. Not local → external map lookup → use alias
/// 5. Not in external map → js_sys type? → emit directly (glob import covers it)
/// 6. Nothing? → error + `use JsValue as Foo;`
fn emit_type_name(name: &str, ctx: &CodegenContext<'_>) -> TokenStream {
    // Resolve through scope
    let resolved = ctx.file_scopes.iter().find_map(|&scope| {
        if name.contains('.') {
            ctx.gctx.resolve_path(scope, name)
        } else {
            ctx.gctx.scopes.resolve(scope, name)
        }
    });

    // Canonical ident name (last segment for dotted paths)
    let ident_name = name.rsplit('.').next().unwrap_or(name);

    // If resolved to a namespace (not a type), emit JsValue
    if let Some(type_id) = resolved {
        if matches!(&ctx.gctx.get_type(type_id).kind, TypeKind::Namespace(_)) {
            return quote! { JsValue };
        }
    }

    // Local type (defined in our output) → emit directly
    if ctx.local_types.contains(ident_name) {
        if let Some(renamed) = ctx.renamed_locals.get(ident_name) {
            let ident = make_ident(renamed);
            return quote! { #ident };
        }
        let ident = make_ident(ident_name);
        return quote! { #ident };
    }

    // External map
    if let Some(rust_path) = ctx.gctx.external_map.resolve_type(ident_name) {
        ctx.register_external(ident_name, &rust_path.path);
        let ident = make_ident(ident_name);
        return quote! { #ident };
    }

    // Type resolved through scope but is not local and not in external map.
    // It's a dependency type — register as JsValue alias (user needs --external).
    if resolved.is_some() {
        ctx.error(format!(
            "Non-local type `{name}` resolved but has no external mapping. \
             Use --external to map this type."
        ));
        ctx.register_external(ident_name, "JsValue");
        let ident = make_ident(ident_name);
        return quote! { #ident };
    }

    // Type did NOT resolve through scope at all.
    // js_sys type? (available via `use js_sys::*`)
    if JS_SYS_RESERVED.contains(&ident_name) {
        let ident = make_ident(ident_name);
        return quote! { #ident };
    }

    // Truly unresolved — error + JsValue alias
    ctx.error(format!(
        "Unresolved type `{name}`. Use --external to map this type."
    ));
    ctx.register_external(ident_name, "JsValue");
    let ident = make_ident(ident_name);
    quote! { #ident }
}

/// Backward-compatible wrapper: calls `emit_type_name` when ctx is available.
fn named_type_to_rust(name: &str, ctx: Option<&CodegenContext<'_>>) -> TokenStream {
    match ctx {
        Some(ctx) => emit_type_name(name, ctx),
        None => quote! { JsValue },
    }
}

/// Create a `syn::Ident`, sanitizing invalid characters and escaping keywords.
pub(crate) fn make_ident(name: &str) -> syn::Ident {
    // Strip characters that aren't valid in Rust identifiers
    let sanitized: String = name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    let sanitized = if sanitized.is_empty() {
        "__unknown__".to_string()
    } else if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{sanitized}")
    } else {
        sanitized
    };
    // Try as a normal identifier first.
    if let Ok(ident) = syn::parse_str::<syn::Ident>(&sanitized) {
        return ident;
    }
    // `self`, `Self`, `super`, `crate` cannot be raw identifiers — append `_`.
    match sanitized.as_str() {
        "self" | "Self" | "super" | "crate" => {
            syn::Ident::new(&format!("{sanitized}_"), proc_macro2::Span::call_site())
        }
        // All other keywords can use r# raw identifiers.
        _ => syn::Ident::new_raw(&sanitized, proc_macro2::Span::call_site()),
    }
}

/// Map an IR `TypeRef` to the type used in a wasm_bindgen return position,
/// wrapping in `Result<T, JsValue>` when `catch` is true.
pub fn to_return_type(
    ty: &TypeRef,
    catch: bool,
    ctx: Option<&CodegenContext<'_>>,
    scope: ScopeId,
) -> TokenStream {
    let inner = to_syn_type(ty, TypePosition::RETURN, ctx, scope);
    if catch {
        quote! { Result<#inner, JsValue> }
    } else {
        inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parse::scope::ScopeId;

    // Helper to run to_syn_type with ARGUMENT position
    fn arg_type(ty: &TypeRef) -> String {
        // Scope is unused when ctx is None — use a dummy value.
        to_syn_type(ty, TypePosition::ARGUMENT, None, ScopeId(0)).to_string()
    }

    fn ret_type(ty: &TypeRef) -> String {
        to_syn_type(ty, TypePosition::RETURN, None, ScopeId(0)).to_string()
    }

    fn inner_type(ty: &TypeRef) -> String {
        to_syn_type(ty, TypePosition::RETURN.to_inner(), None, ScopeId(0)).to_string()
    }

    #[test]
    fn test_string_positions() {
        assert_eq!(arg_type(&TypeRef::String), "& str");
        assert_eq!(ret_type(&TypeRef::String), "String");
    }

    #[test]
    fn test_string_inner_position() {
        // Inner position: string → JsString
        assert_eq!(inner_type(&TypeRef::String), "JsString");
    }

    #[test]
    fn test_number_inner_position() {
        // Inner position: number → Number
        assert_eq!(inner_type(&TypeRef::Number), "Number");
    }

    #[test]
    fn test_boolean_inner_position() {
        // Inner position: boolean → Boolean
        assert_eq!(inner_type(&TypeRef::Boolean), "Boolean");
    }

    #[test]
    fn test_void_inner_position() {
        // Inner position: void → Undefined
        assert_eq!(inner_type(&TypeRef::Void), "Undefined");
    }

    #[test]
    fn test_nullable() {
        let ty = TypeRef::Nullable(Box::new(TypeRef::String));
        // Option<T> passes through position — Option<String> at return position
        let result = ret_type(&ty);
        assert_eq!(result, "Option < String >");
    }

    #[test]
    fn test_promise_with_named_type_unresolved() {
        // Without ctx, Foo is unresolved → JsValue, so Promise<JsValue> elides to Promise
        let ty = TypeRef::Promise(Box::new(TypeRef::Named("Foo".into())));
        assert_eq!(ret_type(&ty), "Promise");
    }

    #[test]
    fn test_nullable_inner() {
        // Nullable inside generic (inner position) → JsOption
        let ty = TypeRef::Nullable(Box::new(TypeRef::String));
        let result = inner_type(&ty);
        assert_eq!(result, "JsOption < JsString >");
    }

    #[test]
    fn test_promise_with_string() {
        let ty = TypeRef::Promise(Box::new(TypeRef::String));
        let result = ret_type(&ty);
        assert_eq!(result, "Promise < JsString >");
    }

    #[test]
    fn test_promise_with_any_elides_generic() {
        let ty = TypeRef::Promise(Box::new(TypeRef::Any));
        let result = ret_type(&ty);
        assert_eq!(result, "Promise");
    }

    #[test]
    fn test_promise_with_void() {
        let ty = TypeRef::Promise(Box::new(TypeRef::Void));
        let result = ret_type(&ty);
        assert_eq!(result, "Promise < Undefined >");
    }

    #[test]
    fn test_nullable_named_type_unresolved() {
        // Without ctx, Foo is unresolved → JsValue
        let ty = TypeRef::Nullable(Box::new(TypeRef::Named("Foo".into())));
        assert_eq!(arg_type(&ty), "Option < & JsValue >");
        assert_eq!(ret_type(&ty), "Option < JsValue >");
    }

    #[test]
    fn test_promise_with_arraybuffer() {
        let ty = TypeRef::Promise(Box::new(TypeRef::ArrayBuffer));
        let result = ret_type(&ty);
        assert_eq!(result, "Promise < ArrayBuffer >");
    }

    #[test]
    fn test_array_with_type() {
        let ty = TypeRef::Array(Box::new(TypeRef::Number));
        let result = ret_type(&ty);
        assert_eq!(result, "Array < Number >");
    }

    #[test]
    fn test_array_with_any_elides() {
        let ty = TypeRef::Array(Box::new(TypeRef::Any));
        let result = ret_type(&ty);
        assert_eq!(result, "Array");
    }

    #[test]
    fn test_set_with_type() {
        let ty = TypeRef::Set(Box::new(TypeRef::String));
        let result = ret_type(&ty);
        assert_eq!(result, "Set < JsString >");
    }

    #[test]
    fn test_map_with_types() {
        let ty = TypeRef::Map(Box::new(TypeRef::String), Box::new(TypeRef::Number));
        let result = ret_type(&ty);
        assert_eq!(result, "Map < JsString , Number >");
    }

    #[test]
    fn test_record_erases_key() {
        let ty = TypeRef::Record(Box::new(TypeRef::String), Box::new(TypeRef::Number));
        let result = ret_type(&ty);
        assert_eq!(result, "Object < Number >");
    }

    #[test]
    fn test_promise_nullable_inner() {
        // Promise<string | null> → Promise<JsOption<JsString>>
        let ty = TypeRef::Promise(Box::new(TypeRef::Nullable(Box::new(TypeRef::String))));
        let result = ret_type(&ty);
        assert_eq!(result, "Promise < JsOption < JsString > >");
    }

    #[test]
    fn test_function_typed() {
        let sig = ir::FunctionSig {
            params: vec![ir::Param {
                name: "x".into(),
                type_ref: TypeRef::Number,
                optional: false,
                variadic: false,
            }],
            return_type: Box::new(TypeRef::Boolean),
        };
        let ty = TypeRef::Function(sig);
        let result = ret_type(&ty);
        assert_eq!(result, "Function < fn (Number) -> Boolean >");
    }

    #[test]
    fn test_function_untyped() {
        let sig = ir::FunctionSig {
            params: vec![ir::Param {
                name: "x".into(),
                type_ref: TypeRef::Any,
                optional: false,
                variadic: false,
            }],
            return_type: Box::new(TypeRef::Any),
        };
        let ty = TypeRef::Function(sig);
        let result = ret_type(&ty);
        assert_eq!(result, "Function");
    }

    #[test]
    fn test_named_unresolved_without_ctx() {
        // Without a CodegenContext, unknown types fall back to JsValue
        let ty = TypeRef::Named("Request".into());
        assert_eq!(ret_type(&ty), "JsValue");
    }

    #[test]
    fn test_named_unknown_without_ctx() {
        let ty = TypeRef::Named("MyCustomType".into());
        assert_eq!(ret_type(&ty), "JsValue");
    }

    #[test]
    fn test_return_with_catch() {
        let ty = TypeRef::Promise(Box::new(TypeRef::Void));
        let result = to_return_type(&ty, true, None, ScopeId(0)).to_string();
        assert_eq!(result, "Result < Promise < Undefined > , JsValue >");
    }

    #[test]
    fn test_union_erases() {
        let ty = TypeRef::Union(vec![TypeRef::String, TypeRef::Number]);
        // Unions erase to JsValue, but in argument position they're borrowed
        assert_eq!(arg_type(&ty), "& JsValue");
        assert_eq!(ret_type(&ty), "JsValue");
    }

    fn test_gctx() -> (GlobalContext, ScopeId) {
        let mut gctx = GlobalContext::new();
        let scope = gctx.create_root_scope();
        (gctx, scope)
    }

    #[test]
    fn test_local_type_overrides_web_sys() {
        let (gctx, scope) = test_gctx();
        let mut ctx = CodegenContext::empty(&gctx, scope);
        ctx.local_types.insert("Response".into());
        let ty = TypeRef::Named("Response".into());
        let result = to_syn_type(&ty, TypePosition::RETURN, Some(&ctx), scope).to_string();
        assert_eq!(result, "Response");
    }

    #[test]
    fn test_union_alias_resolves_to_jsvalue() {
        // A type alias to a union resolves through the scope and erases to JsValue.
        let (mut gctx, scope) = test_gctx();
        let alias_id = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::TypeAlias(crate::ir::TypeAliasDecl {
                name: "BodyInit".to_string(),
                type_params: vec![],
                target: TypeRef::Union(vec![TypeRef::String, TypeRef::ArrayBuffer]),
                from_module: None,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: scope,
            exported: false,
        });
        gctx.scopes.insert(scope, "BodyInit".to_string(), alias_id);

        let ctx = CodegenContext::empty(&gctx, scope);
        let ty = TypeRef::Named("BodyInit".into());
        let result = to_syn_type(&ty, TypePosition::RETURN, Some(&ctx), scope).to_string();
        assert_eq!(result, "JsValue");
    }

    #[test]
    fn test_unresolved_with_ctx_registers_jsvalue_alias() {
        let (gctx, scope) = test_gctx();
        let ctx = CodegenContext::empty(&gctx, scope);
        let ty = TypeRef::Named("Response".into());
        let result = to_syn_type(&ty, TypePosition::RETURN, Some(&ctx), scope).to_string();
        // With ctx, unresolved types emit the name (aliased to JsValue via use statement)
        assert_eq!(result, "Response");
        // Verify the JsValue alias was registered
        let uses = ctx.external_uses.borrow();
        assert_eq!(uses.get("Response"), Some(&"JsValue".to_string()));
    }

    #[test]
    fn test_local_type_in_promise() {
        let (gctx, scope) = test_gctx();
        let mut ctx = CodegenContext::empty(&gctx, scope);
        ctx.local_types.insert("MyThing".into());
        let ty = TypeRef::Promise(Box::new(TypeRef::Named("MyThing".into())));
        let result = to_syn_type(&ty, TypePosition::RETURN, Some(&ctx), scope).to_string();
        assert_eq!(result, "Promise < MyThing >");
    }

    // === New tests for the unified approach ===

    #[test]
    fn test_to_inner_preserves_direction() {
        let pos = TypePosition::ARGUMENT.to_inner();
        assert!(pos.is_argument());
        assert!(pos.inner);

        let pos = TypePosition::RETURN.to_inner();
        assert!(!pos.is_argument());
        assert!(pos.inner);
    }

    #[test]
    fn test_inner_position_named_type_unresolved() {
        // Without ctx, unresolved named types → JsValue
        let ty = TypeRef::Named("Response".into());
        assert_eq!(inner_type(&ty), "JsValue");
        assert_eq!(ret_type(&ty), "JsValue");
    }

    #[test]
    fn test_inner_position_typed_array_unchanged() {
        // Typed arrays pass through in inner position
        let ty = TypeRef::Uint8Array;
        assert_eq!(inner_type(&ty), "Uint8Array");
        assert_eq!(ret_type(&ty), "Uint8Array");
    }

    #[test]
    fn test_tuple_generates_array_tuple() {
        // Without ctx, named types are unresolved → JsValue, so Array<JsValue> elides to Array
        let ty = TypeRef::Tuple(vec![
            TypeRef::Array(Box::new(TypeRef::Named("ImportSpecifier".into()))),
            TypeRef::Array(Box::new(TypeRef::Named("ExportSpecifier".into()))),
            TypeRef::Boolean,
            TypeRef::Boolean,
        ]);
        let result = ret_type(&ty);
        assert_eq!(result, "ArrayTuple < (Array , Array , Boolean , Boolean) >");
    }

    #[test]
    fn test_empty_tuple_is_bare_array() {
        let ty = TypeRef::Tuple(vec![]);
        assert_eq!(ret_type(&ty), "Array");
    }

    #[test]
    fn test_type_position_all_variants() {
        // Verify TypePosition constants and to_inner() work correctly
        let ty = TypeRef::String;
        assert_eq!(
            to_syn_type(&ty, TypePosition::ARGUMENT, None, ScopeId(0)).to_string(),
            "& str"
        );
        assert_eq!(
            to_syn_type(&ty, TypePosition::RETURN, None, ScopeId(0)).to_string(),
            "String"
        );
        // to_inner() → inner:true, so should give JsString
        assert_eq!(
            to_syn_type(&ty, TypePosition::RETURN.to_inner(), None, ScopeId(0)).to_string(),
            "JsString"
        );
        // Argument inner also gives JsString (inner overrides borrowing)
        assert_eq!(
            to_syn_type(&ty, TypePosition::ARGUMENT.to_inner(), None, ScopeId(0)).to_string(),
            "JsString"
        );
    }
}
