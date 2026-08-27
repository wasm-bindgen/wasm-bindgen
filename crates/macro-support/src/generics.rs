use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use syn::parse_quote;
use syn::visit_mut::{self, VisitMut};
use syn::{visit::Visit, Ident, Type};

use crate::error::Diagnostic;

/// Visitor to replace wasm bindgen generics with their concrete types
/// The concrete type is the default type on the import if specified when it was defined.
struct GenericRenameVisitor<'a> {
    renames: &'a BTreeMap<&'a Ident, Option<Cow<'a, syn::Type>>>,
    err: Option<Diagnostic>,
}

impl<'a> VisitMut for GenericRenameVisitor<'a> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if self.err.is_some() {
            return;
        }
        if let Type::Path(type_path) = ty {
            // Handle <T as Trait>::AssocType
            if let Some(qself) = &mut type_path.qself {
                if let Type::Path(qself_path) = &mut *qself.ty {
                    if qself_path.qself.is_none() && qself_path.path.segments.len() == 1 {
                        let ident = &qself_path.path.segments[0].ident;
                        if let Some((_, concrete)) = self.renames.get_key_value(ident) {
                            *qself.ty = if let Some(concrete) = concrete {
                                concrete.clone().into_owned()
                            } else {
                                parse_quote! { JsValue }
                            };
                            return;
                        }
                    }
                }
            }
            // Normal T::...
            if type_path.qself.is_none() && !type_path.path.segments.is_empty() {
                let first_seg = &type_path.path.segments[0];

                if let Some((_, concrete)) = self.renames.get_key_value(&first_seg.ident) {
                    if let Some(concrete) = concrete {
                        if type_path.path.segments.len() == 1 {
                            *ty = concrete.clone().into_owned();
                        } else if let Type::Path(concrete_path) = concrete.as_ref() {
                            let remaining: Vec<_> =
                                type_path.path.segments.iter().skip(1).cloned().collect();
                            type_path.path.segments = concrete_path.path.segments.clone();
                            type_path.path.segments.extend(remaining);
                        }
                    } else {
                        *ty = parse_quote! { JsValue };
                    }
                    return;
                }
            }
        }
        visit_mut::visit_type_mut(self, ty);
    }
}

/// Helper visitor for generic parameter usage
#[derive(Debug)]
pub struct GenericNameVisitor<'a, 'b> {
    generic_params: &'a Vec<&'a Ident>,
    /// The generic params that were found
    found_set: &'b mut BTreeSet<Ident>,
}

/// Helper visitor for generic parameter usage
impl<'a, 'b> GenericNameVisitor<'a, 'b> {
    /// Construct a new generic name visitors with a param search set,
    /// and optionally a second parameter search set.
    pub fn new(generic_params: &'a Vec<&'a Ident>, found_set: &'b mut BTreeSet<Ident>) -> Self {
        Self {
            generic_params,
            found_set,
        }
    }

    fn generic_param(&self, ident: &Ident) -> Option<&Ident> {
        self.generic_params
            .iter()
            .copied()
            .find(|param| *param == ident)
    }

    fn record_generic_param(&mut self, ident: &Ident) -> bool {
        let Some(param) = self.generic_param(ident).cloned() else {
            return false;
        };
        // Keep the declaration's span when this identifier is later emitted as
        // a generated generic parameter, so rustc suggestions target the
        // declaration rather than an occurrence in a type argument.
        self.found_set.insert(param);
        true
    }
}

impl<'a, 'b> Visit<'a> for GenericNameVisitor<'a, 'b> {
    fn visit_type_reference(&mut self, type_ref: &'a syn::TypeReference) {
        if let syn::Type::Path(type_path) = &*type_ref.elem {
            // Handle <T as Trait>::AssocType - visit the qself type
            if let Some(qself) = &type_path.qself {
                syn::visit::visit_type(self, &qself.ty);
                // Also visit the path segments for any generic args
                for segment in &type_path.path.segments {
                    syn::visit::visit_path_segment(self, segment);
                }
                return;
            }

            if let Some(first_segment) = type_path.path.segments.first() {
                if type_path.path.segments.len() == 1 && first_segment.arguments.is_empty() {
                    if self.record_generic_param(&first_segment.ident) {
                        return;
                    }
                } else {
                    self.record_generic_param(&first_segment.ident);

                    syn::visit::visit_path_arguments(self, &first_segment.arguments);

                    for segment in type_path.path.segments.iter().skip(1) {
                        syn::visit::visit_path_segment(self, segment);
                    }
                    return;
                }
            }
        }

        // For other cases, continue normal visiting
        syn::visit::visit_type_reference(self, type_ref);
    }

    fn visit_path(&mut self, path: &'a syn::Path) {
        if let Some(first_segment) = path.segments.first() {
            self.record_generic_param(&first_segment.ident);
        }

        for segment in &path.segments {
            match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Type(ty) => {
                                syn::visit::visit_type(self, ty);
                            }
                            syn::GenericArgument::AssocType(binding) => {
                                // Don't visit binding.ident, only visit binding.ty
                                syn::visit::visit_type(self, &binding.ty);
                            }
                            _ => {
                                syn::visit::visit_generic_argument(self, arg);
                            }
                        }
                    }
                }
                syn::PathArguments::Parenthesized(args) => {
                    // Handle function syntax like FnMut(T) -> Result<R, JsValue>
                    for input in &args.inputs {
                        syn::visit::visit_type(self, &input.ty);
                    }
                    if let syn::ReturnType::Type(_, return_type) = &args.output {
                        syn::visit::visit_type(self, return_type);
                    }
                }
                syn::PathArguments::None => {}
            }
        }
    }
}

/// Obtain the generic parameters and their optional defaults
pub(crate) fn generic_params(generics: &syn::Generics) -> Vec<(&Ident, Option<&syn::Type>)> {
    generics
        .type_params()
        .map(|tp| (&tp.ident, tp.default.as_ref().map(|(_, ty)| ty)))
        .collect()
}

/// Returns a vector of token streams representing generic type parameters with their bounds.
/// For example, `<T: Clone, U: Display>` returns `[quote!(T: Clone), quote!(U: Display)]`.
/// This is useful for constructing impl blocks that need to add lifetimes while preserving bounds.
pub(crate) fn type_params_with_bounds(generics: &syn::Generics) -> Vec<proc_macro2::TokenStream> {
    generics
        .type_params()
        .map(|tp| {
            let ident = &tp.ident;
            let bounds = &tp.bounds;
            if bounds.is_empty() {
                quote::quote! { #ident }
            } else {
                quote::quote! { #ident: #bounds }
            }
        })
        .collect()
}

/// Returns a vector of token streams representing generic lifetime parameters
/// with their inline bounds. For example, `<'a: 'b, 'b>` returns
/// `[quote!('a: 'b), quote!('b)]`. This is useful for redeclaring lifetime
/// parameters on a nested item (e.g. a monomorphised shim `fn`), which does
/// not inherit the enclosing function's generics and must repeat them
/// (including their bounds) explicitly.
pub(crate) fn lifetime_params_with_bounds(
    generics: &syn::Generics,
) -> Vec<proc_macro2::TokenStream> {
    generics
        .lifetimes()
        .map(|lp| {
            let lifetime = &lp.lifetime;
            let bounds = &lp.bounds;
            if bounds.is_empty() {
                quote::quote! { #lifetime }
            } else {
                quote::quote! { #lifetime: #bounds }
            }
        })
        .collect()
}

/// Obtain the generic bounds, both inline and where clauses together
pub(crate) fn generic_bounds<'a>(generics: &'a syn::Generics) -> Vec<Cow<'a, syn::WherePredicate>> {
    let mut bounds = Vec::new();
    for param in &generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            if !type_param.bounds.is_empty() {
                let ident = &type_param.ident;
                let predicate = syn::WherePredicate::Type(syn::PredicateType {
                    attrs: Vec::new(),
                    lifetimes: None,
                    bounded_ty: syn::parse_quote!(#ident),
                    colon_token: syn::Token![:](proc_macro2::Span::call_site()),
                    bounds: type_param.bounds.clone(),
                });
                bounds.push(Cow::Owned(predicate));
            }
        }
    }
    if let Some(where_clause) = &generics.where_clause {
        bounds.extend(where_clause.predicates.iter().map(Cow::Borrowed));
    }
    bounds
}

/// Replace specified lifetime parameters with 'static.
/// This is used when generating concrete ABI types for extern blocks,
/// which cannot have lifetime parameters from the outer scope.
/// Only the lifetimes in `lifetimes_to_staticize` are replaced.
pub(crate) fn staticize_lifetimes(
    mut ty: syn::Type,
    lifetimes_to_staticize: &[&syn::Lifetime],
) -> syn::Type {
    struct LifetimeStaticizer<'a> {
        lifetimes: &'a [&'a syn::Lifetime],
    }
    impl VisitMut for LifetimeStaticizer<'_> {
        fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
            if self.lifetimes.iter().any(|lt| lt.ident == lifetime.ident) {
                *lifetime = syn::Lifetime::new("'static", lifetime.span());
            }
        }
    }
    LifetimeStaticizer {
        lifetimes: lifetimes_to_staticize,
    }
    .visit_type_mut(&mut ty);
    ty
}

/// Obtain the generic type parameter names
pub(crate) fn generic_param_names(generics: &syn::Generics) -> Vec<&Ident> {
    generics.type_params().map(|tp| &tp.ident).collect()
}

/// Obtain all lifetime parameters from generics
pub(crate) fn lifetime_params(generics: &syn::Generics) -> Vec<&syn::Lifetime> {
    generics.lifetimes().map(|lp| &lp.lifetime).collect()
}

/// Obtain both lifetime and type parameter names from generics
pub(crate) fn all_param_names(generics: &syn::Generics) -> (Vec<&syn::Lifetime>, Vec<&Ident>) {
    (lifetime_params(generics), generic_param_names(generics))
}

/// Helper visitor for lifetime usage detection in types
pub struct LifetimeVisitor<'a> {
    lifetime_params: &'a [&'a syn::Lifetime],
    found_set: BTreeSet<syn::Lifetime>,
}

impl<'a> LifetimeVisitor<'a> {
    pub fn new(lifetime_params: &'a [&'a syn::Lifetime]) -> Self {
        Self {
            lifetime_params,
            found_set: BTreeSet::new(),
        }
    }

    pub fn into_found(self) -> BTreeSet<syn::Lifetime> {
        self.found_set
    }
}

impl<'ast> syn::visit::Visit<'ast> for LifetimeVisitor<'_> {
    fn visit_lifetime(&mut self, lifetime: &'ast syn::Lifetime) {
        if self.lifetime_params.contains(&lifetime) {
            self.found_set.insert(lifetime.clone());
        }
    }
}

/// Find all lifetimes from the given set that are used in a type
pub(crate) fn used_lifetimes_in_type<'a>(
    ty: &syn::Type,
    lifetime_params: &'a [&'a syn::Lifetime],
) -> BTreeSet<syn::Lifetime> {
    let mut visitor = LifetimeVisitor::new(lifetime_params);
    syn::visit::Visit::visit_type(&mut visitor, ty);
    visitor.into_found()
}

pub(crate) fn uses_generic_params(ty: &syn::Type, generic_names: &Vec<&Ident>) -> bool {
    let mut found_set = Default::default();
    let mut visitor = GenericNameVisitor::new(generic_names, &mut found_set);
    visitor.visit_type(ty);
    !found_set.is_empty()
}

/// Visitor that detects a reference to a generic type parameter appearing
/// anywhere within a type, including when nested inside other types (e.g.
/// `&T`, `&&T`, `Option<&T>`, `(T, &T)`, `[&T; N]`, `Box<&T>`).
struct RefToGenericVisitor<'a> {
    generic_params: &'a Vec<&'a Ident>,
    found: bool,
}

impl<'a, 'ast> Visit<'ast> for RefToGenericVisitor<'a> {
    fn visit_type_reference(&mut self, type_ref: &'ast syn::TypeReference) {
        // A reference whose referent mentions a generic type parameter would
        // require impls that don't generally exist (e.g. a higher-ranked
        // `for<'a> &'a T: IntoWasmAbi`), so flag it.
        if uses_generic_params(&type_ref.elem, self.generic_params) {
            self.found = true;
        }
        // Keep recursing to catch further-nested references.
        syn::visit::visit_type_reference(self, type_ref);
    }
}

/// Returns `true` if `ty` contains a reference (`&_`) whose referent mentions
/// one of the given generic type parameters, at any nesting depth.
pub(crate) fn references_generic_param(ty: &syn::Type, generic_names: &Vec<&Ident>) -> bool {
    let mut visitor = RefToGenericVisitor {
        generic_params: generic_names,
        found: false,
    };
    visitor.visit_type(ty);
    visitor.found
}

/// Visitor that detects whether an `impl Trait` type appears anywhere within
/// a type, including when nested inside other types (e.g. `&impl Trait`,
/// `Vec<impl Trait>`).
struct ImplTraitPresenceVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for ImplTraitPresenceVisitor {
    fn visit_type_impl_trait(&mut self, i: &'ast syn::TypeImplTrait) {
        self.found = true;
        // Keep recursing in case a bound itself nests another `impl Trait`
        // (not legal today, but cheap to keep general).
        syn::visit::visit_type_impl_trait(self, i);
    }
}

/// Returns `true` if `ty` contains an `impl Trait` type at any nesting depth.
///
/// Argument-position `impl Trait` desugars to an anonymous generic type
/// parameter, so it never shows up in `fn.generics` — callers that need to
/// know whether a signature is "generic" at all (e.g. deciding whether a
/// block-level `experimental_generic_mono` applies) have to check argument types
/// directly rather than `fn.generics.type_params()`.
pub(crate) fn has_impl_trait(ty: &syn::Type) -> bool {
    let mut visitor = ImplTraitPresenceVisitor { found: false };
    visitor.visit_type(ty);
    visitor.found
}

/// Visitor that rewrites every `impl Trait` type it finds (including nested,
/// e.g. `Vec<impl Trait>`) into a synthesized, named type parameter, pushing
/// one [`syn::GenericParam::Type`] per occurrence — carrying the `impl
/// Trait`'s own bounds — onto `params`.
///
/// Argument-position `impl Trait` desugars to an anonymous generic type
/// parameter that never appears in `fn.generics`, so there is no name for
/// `experimental_generic_mono` codegen to hang a `where` bound, shim generic
/// parameter, or `breaks_if_inlined::<..>` turbofish argument off of. This
/// visitor "un-desugars" it back into the named form a user would otherwise
/// have to write by hand (`fn f<T: Trait>(x: T)`), giving each occurrence a
/// synthesized, hygienic name so the rest of per-mono codegen can treat it
/// exactly like any other type parameter.
struct ImplTraitDesugar<'a> {
    params: &'a mut Vec<syn::GenericParam>,
}

impl<'a> VisitMut for ImplTraitDesugar<'a> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        let syn::Type::ImplTrait(impl_trait) = ty else {
            // Keep recursing to reach a nested `impl Trait` (e.g. inside
            // `Vec<impl Trait>` or a tuple).
            visit_mut::visit_type_mut(self, ty);
            return;
        };
        // Spanned at the original `impl Trait` so a bound that fails to hold
        // (e.g. `T: IntoWasmAbi`) points back at the user's own type rather
        // than at generated code.
        let ident = Ident::new(
            &format!("__WasmBindgenImplTrait{}", self.params.len()),
            syn::spanned::Spanned::span(&*impl_trait),
        );
        self.params.push(syn::GenericParam::Type(syn::TypeParam {
            attrs: Vec::new(),
            ident: ident.clone(),
            colon_token: Some(Default::default()),
            bounds: impl_trait.bounds.clone(),
            default: None,
        }));
        *ty = syn::Type::Path(syn::TypePath {
            attrs: Vec::new(),
            qself: None,
            path: ident.into(),
        });
        // The replacement is a bare type-parameter path, which has nothing
        // left to recurse into.
    }
}

/// Rewrites every `impl Trait` type appearing within `ty` (at any nesting
/// depth) into a synthesized, named type parameter, appending one generic
/// parameter per occurrence to `params`. `ty` is left unchanged if it
/// contains no `impl Trait`.
///
/// `params` is threaded across every argument of a single function so that
/// synthesized names stay unique across the whole signature; pass the same
/// `Vec` for each call.
pub(crate) fn desugar_impl_trait(ty: &mut syn::Type, params: &mut Vec<syn::GenericParam>) {
    let mut visitor = ImplTraitDesugar { params };
    visitor.visit_type_mut(ty);
}

pub(crate) fn uses_lifetime_params(ty: &syn::Type, lifetime_params: &[&syn::Lifetime]) -> bool {
    !used_lifetimes_in_type(ty, lifetime_params).is_empty()
}

/// Find all lifetimes from the given set that are used in type param bounds
pub(crate) fn used_lifetimes_in_bounds<'a>(
    bounds: &syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>,
    lifetime_params: &'a [&'a syn::Lifetime],
) -> BTreeSet<syn::Lifetime> {
    let mut visitor = LifetimeVisitor::new(lifetime_params);
    for bound in bounds {
        syn::visit::Visit::visit_type_param_bound(&mut visitor, bound);
    }
    visitor.into_found()
}

pub(crate) fn used_generic_params<'a>(
    ty: &'a syn::Type,
    generic_names: &'a Vec<&Ident>,
    mut used_params: BTreeSet<Ident>,
) -> BTreeSet<Ident> {
    let mut visitor = GenericNameVisitor::new(generic_names, &mut used_params);
    visitor.visit_type(ty);
    used_params
}

/// Checks whether every occurrence of each ident in `generic_names` that appears
/// anywhere inside `args` is in a *structurally constraining* position —
/// i.e. a position from which rustc can read the parameter off of the
/// constructed type. This mirrors the E0207 rule for type parameters on
/// `impl` blocks: a parameter must appear in `Self` (or a trait ref) in a
/// structurally determined slot, otherwise Rust can't infer it at use sites.
///
/// Constraining positions (for a param appearing somewhere inside):
///   - Bare: `T`
///   - As a type argument of a nominal path `Foo<..., T, ...>` (recursive)
///   - Under references, arrays, slices, tuples, parens (recursive)
///
/// Non-constraining positions:
///   - Under a QSelf / projection: `<T as Trait>::X` or `T::X`
///   - Inside a `fn(T) -> U` / `dyn Fn(T)` / `impl Fn(T)` — function-pointer
///     and trait-object / `impl Trait` slots do not constrain.
///   - Inside an associated-type binding's RHS (those project through the
///     outer trait, so they are not injective).
///
/// Returns `true` if the args are safe to hoist (all occurrences constraining,
/// or no occurrences at all), `false` if any occurrence is non-constraining.
pub(crate) fn args_are_constraining_for(
    args: &syn::punctuated::Punctuated<syn::GenericArgument, syn::Token![,]>,
    generic_names: &[&Ident],
) -> bool {
    for arg in args {
        match arg {
            syn::GenericArgument::Type(ty) if !type_is_constraining(ty, generic_names) => {
                return false;
            }
            // Associated type bindings (`Trait<Item = T>`) project through the
            // outer trait, so any fn generics inside the RHS are behind a
            // projection — not constraining.
            syn::GenericArgument::AssocType(binding)
                if type_mentions_any(&binding.ty, generic_names) =>
            {
                return false;
            }
            // Anything else (lifetimes, consts, already-constraining types,
            // future arg kinds) doesn't disqualify the args.
            _ => {}
        }
    }
    true
}

/// A type is "constraining" for the fn generics it contains iff every
/// occurrence of any `generic_names` ident within it is in a constraining
/// position. See [`args_are_constraining_for`] for the rules.
fn type_is_constraining(ty: &syn::Type, generic_names: &[&Ident]) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            // QSelf -> projection like `<T as Trait>::Assoc`. Any fn generic
            // appearing anywhere inside is behind a projection.
            if type_path.qself.is_some() {
                return !type_mentions_any(ty, generic_names);
            }

            // Bare `T` where T is a fn generic: constraining.
            if type_path.path.segments.len() == 1 {
                let seg = &type_path.path.segments[0];
                if matches!(seg.arguments, syn::PathArguments::None)
                    && generic_names.contains(&&seg.ident)
                {
                    return true;
                }
            }

            // `T::Foo...` (multi-segment path whose head is a fn generic)
            // is a projection through the head's implicit trait — any fn
            // generic inside is non-constraining.
            if type_path.path.segments.len() > 1 {
                if let Some(first) = type_path.path.segments.first() {
                    if generic_names.contains(&&first.ident) {
                        return !type_mentions_any(ty, generic_names);
                    }
                }
            }

            // Nominal path `Foo<..args..>`: recurse into the last segment's
            // args. Leading segments (module path) don't carry generics that
            // mention fn params.
            for seg in &type_path.path.segments {
                match &seg.arguments {
                    syn::PathArguments::None => {}
                    syn::PathArguments::AngleBracketed(a) => {
                        if !args_are_constraining_for(&a.args, generic_names) {
                            return false;
                        }
                    }
                    syn::PathArguments::Parenthesized(p) => {
                        // `Fn(T) -> U` sugar: function-pointer-like,
                        // non-constraining.
                        for input in &p.inputs {
                            if type_mentions_any(&input.ty, generic_names) {
                                return false;
                            }
                        }
                        if let syn::ReturnType::Type(_, ret) = &p.output {
                            if type_mentions_any(ret, generic_names) {
                                return false;
                            }
                        }
                    }
                }
            }
            true
        }
        syn::Type::Reference(r) => type_is_constraining(&r.elem, generic_names),
        syn::Type::Array(a) => type_is_constraining(&a.elem, generic_names),
        syn::Type::Slice(s) => type_is_constraining(&s.elem, generic_names),
        syn::Type::Group(g) => type_is_constraining(&g.elem, generic_names),
        syn::Type::Paren(p) => type_is_constraining(&p.elem, generic_names),
        syn::Type::Tuple(t) => t
            .elems
            .iter()
            .all(|e| type_is_constraining(e, generic_names)),
        // Pointer / FnPtr / TraitObject / ImplTrait / Infer / Never / Macro:
        // any fn-generic mention here is non-constraining (fn-ptr, dyn, impl
        // Trait are explicitly non-constraining per RFC 0447; the rest are
        // handled conservatively).
        _ => !type_mentions_any(ty, generic_names),
    }
}

/// Whether `ty` mentions any of the given idents anywhere (constraining or not).
fn type_mentions_any(ty: &syn::Type, generic_names: &[&Ident]) -> bool {
    let vec: Vec<&Ident> = generic_names.to_vec();
    let mut found = BTreeSet::new();
    let mut visitor = GenericNameVisitor::new(&vec, &mut found);
    visitor.visit_type(ty);
    !found.is_empty()
}

/// Usage visitor for generic bounds
pub(crate) fn generics_predicate_uses(
    predicate: &syn::WherePredicate,
    generic_names: &Vec<&Ident>,
) -> bool {
    let mut found_set = Default::default();
    let mut visitor = GenericNameVisitor::new(generic_names, &mut found_set);
    visitor.visit_where_predicate(predicate);
    !found_set.is_empty()
}

/// Concrete type replacement visitor application.
/// Replaces generic type parameters with their concrete types (or JsValue if no default),
/// and replaces specified lifetime parameters with 'static (since extern blocks cannot have
/// lifetime parameters from the outer scope).
pub(crate) fn generic_to_concrete<'a>(
    mut ty: syn::Type,
    generic_names: &BTreeMap<&'a Ident, Option<Cow<'a, syn::Type>>>,
    lifetimes_to_staticize: &[&syn::Lifetime],
) -> Result<syn::Type, Diagnostic> {
    // First, replace type parameters with their concrete types
    if !generic_names.is_empty() {
        let mut visitor = GenericRenameVisitor {
            renames: generic_names,
            err: None,
        };
        visitor.visit_type_mut(&mut ty);
        if let Some(err) = visitor.err {
            return Err(err);
        }
    }
    // Then, replace specified lifetimes with 'static for ABI compatibility
    Ok(staticize_lifetimes(ty, lifetimes_to_staticize))
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    #[test]
    fn test_generic_name_visitor() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let u_ident = syn::Ident::new("U", proc_macro2::Span::call_site());
        let generic_params = vec![&t_ident, &u_ident];

        // Test T as value
        let ty: syn::Type = syn::parse_quote!(T);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));

        // Test &T as reference
        let ty: syn::Type = syn::parse_quote!(&T);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));

        // Test T<U> - both found
        let ty: syn::Type = syn::parse_quote!(T<U>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));
        assert!(visitor.found_set.contains(&u_ident));

        // Test &T<U> - both found
        let ty: syn::Type = syn::parse_quote!(&T<U>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));
        assert!(visitor.found_set.contains(&u_ident));

        // Test T::<U>::Foo - T and U found, Foo ignored
        let ty: syn::Type = syn::parse_quote!(T::<U>::Foo);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));
        assert!(visitor.found_set.contains(&u_ident));

        // Test Vec<T> - T found, Vec ignored
        let ty: syn::Type = syn::parse_quote!(Vec<T>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));
        assert!(!visitor.found_set.contains(&u_ident));
    }

    #[test]
    fn test_associated_type_binding() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let u_ident = syn::Ident::new("U", proc_macro2::Span::call_site());
        let generic_params = vec![&t_ident, &u_ident];

        // Test SomeTrait<T = U> - should find U (RHS) but NOT T (LHS assoc type name)
        let ty: syn::Type = syn::parse_quote!(SomeTrait<T = U>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(!visitor.found_set.contains(&t_ident)); // T is LHS assoc type name
        assert!(visitor.found_set.contains(&u_ident)); // U is RHS generic parameter

        // Test SomeTrait<U = T> - should find T (RHS) but NOT U (LHS assoc type name)
        let ty: syn::Type = syn::parse_quote!(SomeTrait<U = T>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident)); // T is RHS generic parameter
        assert!(!visitor.found_set.contains(&u_ident)); // U is LHS assoc type name
    }

    #[test]
    fn test_nested_references() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let u_ident = syn::Ident::new("U", proc_macro2::Span::call_site());
        let generic_params = vec![&t_ident, &u_ident];

        // Test &T
        let ty: syn::Type = syn::parse_quote!(&T);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));

        // Test &&T
        let ty: syn::Type = syn::parse_quote!(&&T);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));

        // Test &&&T
        let ty: syn::Type = syn::parse_quote!(&&&T);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));

        // Test &T<U>
        let ty: syn::Type = syn::parse_quote!(&T<U>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));
        assert!(visitor.found_set.contains(&u_ident));
    }

    #[test]
    fn test_mixed_usage() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let generic_params = vec![&t_ident];

        // Test T appearing in multiple places
        let ty: syn::Type = syn::parse_quote!(SomeTrait<Item = T> + OtherTrait<Ref = &T>);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_set.contains(&t_ident));
    }

    #[test]
    fn test_ref_qself_trait_assoc_type() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let generic_params = vec![&t_ident];

        // Test &<T as JsFunction1>::Arg1 - T should be found
        let ty: syn::Type = syn::parse_quote!(&<T as JsFunction1>::Arg1);
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(
            visitor.found_set.contains(&t_ident),
            "T should be found in &<T as JsFunction1>::Arg1"
        );
    }

    #[test]
    fn test_complex_reference_with_closure() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let r_ident = syn::Ident::new("R", proc_macro2::Span::call_site());
        let generic_params = vec![&t_ident, &r_ident];

        let ty: syn::Type = syn::parse_quote!(&Closure<dyn FnMut(T) -> Result<R, JsValue>>);

        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_type(&mut visitor, &ty);

        assert!(visitor.found_set.contains(&t_ident));
        assert!(visitor.found_set.contains(&r_ident));
    }

    #[test]
    fn test_generic_args_to_concrete() {
        use std::borrow::Cow;
        use std::collections::BTreeMap;

        // T -> String replacement
        let t = syn::parse_quote!(T);
        let str: syn::Type = syn::parse_quote!(String);
        let generic_names: BTreeMap<&syn::Ident, Option<Cow<syn::Type>>> = {
            let mut map = BTreeMap::new();
            map.insert(&t, Some(Cow::Borrowed(&str)));
            map
        };

        // T gets replaced with String
        let generic_type: syn::Type = syn::parse_quote!(Promise<T>);
        let result =
            crate::generics::generic_to_concrete(generic_type, &generic_names, &[]).unwrap();
        let expected: syn::Type = syn::parse_quote!(Promise<String>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Mixed: i32 stays, T becomes String
        let mixed_type: syn::Type = syn::parse_quote!(Promise<i32, T>);
        let result = crate::generics::generic_to_concrete(mixed_type, &generic_names, &[]).unwrap();
        let expected: syn::Type = syn::parse_quote!(Promise<i32, String>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // No generics to replace - unchanged
        let concrete_type: syn::Type = syn::parse_quote!(Promise<i32, bool>);
        let result =
            crate::generics::generic_to_concrete(concrete_type, &generic_names, &[]).unwrap();
        let expected: syn::Type = syn::parse_quote!(Promise<i32, bool>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );
    }

    #[test]
    fn test_generic_associated_type_replacement() {
        use std::borrow::Cow;
        use std::collections::BTreeMap;

        let t: syn::Ident = syn::parse_quote!(T);
        let concrete: syn::Type = syn::parse_quote!(MyConcreteType);
        let generic_names: BTreeMap<&syn::Ident, Option<Cow<syn::Type>>> = {
            let mut map = BTreeMap::new();
            map.insert(&t, Some(Cow::Borrowed(&concrete)));
            map
        };

        // T::DurableObjectStub -> MyConcreteType::DurableObjectStub
        let assoc_type: syn::Type = syn::parse_quote!(T::DurableObjectStub);
        let result = crate::generics::generic_to_concrete(assoc_type, &generic_names, &[]).unwrap();
        let expected: syn::Type = syn::parse_quote!(MyConcreteType::DurableObjectStub);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Nested: Vec<T::Item> -> Vec<MyConcreteType::Item>
        let nested: syn::Type = syn::parse_quote!(Vec<T::Item>);
        let result = crate::generics::generic_to_concrete(nested, &generic_names, &[]).unwrap();
        let expected: syn::Type = syn::parse_quote!(Vec<MyConcreteType::Item>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Complex: WasmRet<<T::Stub as FromWasmAbi>::Abi>
        let complex: syn::Type = syn::parse_quote!(WasmRet<<T::Stub as FromWasmAbi>::Abi>);
        let result = crate::generics::generic_to_concrete(complex, &generic_names, &[]).unwrap();
        let expected: syn::Type =
            syn::parse_quote!(WasmRet<<MyConcreteType::Stub as FromWasmAbi>::Abi>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // T<Foo> gets fully replaced, args discarded
        let with_args: syn::Type = syn::parse_quote!(T<SomeArg>);
        let result = crate::generics::generic_to_concrete(with_args, &generic_names, &[]).unwrap();
        let expected: syn::Type = syn::parse_quote!(MyConcreteType);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // QSelf: <T::DurableObjectStub as FromWasmAbi>::Abi
        let qself_type: syn::Type = syn::parse_quote!(<T::DurableObjectStub as FromWasmAbi>::Abi);
        let result = crate::generics::generic_to_concrete(qself_type, &generic_names, &[]).unwrap();
        let expected: syn::Type =
            syn::parse_quote!(<MyConcreteType::DurableObjectStub as FromWasmAbi>::Abi);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // QSelf with trait: <T as DurableObject>::DurableObjectStub
        let qself_trait: syn::Type = syn::parse_quote!(<T as DurableObject>::DurableObjectStub);
        let result =
            crate::generics::generic_to_concrete(qself_trait, &generic_names, &[]).unwrap();
        let expected: syn::Type =
            syn::parse_quote!(<MyConcreteType as DurableObject>::DurableObjectStub);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Reference to QSelf with trait: &<T as DurableObject>::DurableObjectStub
        let ref_qself_trait: syn::Type =
            syn::parse_quote!(&<T as DurableObject>::DurableObjectStub);
        let result =
            crate::generics::generic_to_concrete(ref_qself_trait, &generic_names, &[]).unwrap();
        let expected: syn::Type =
            syn::parse_quote!(&<MyConcreteType as DurableObject>::DurableObjectStub);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );
    }

    #[test]
    fn test_where_predicate_assoc_type_binding() {
        // Test that generics_predicate_uses finds generic params in associated type bindings
        // This is the pattern: F: JsFunction<Ret = Ret>
        // Both F and Ret should be detected as used

        let f_ident = syn::Ident::new("F", proc_macro2::Span::call_site());
        let ret_ident = syn::Ident::new("Ret", proc_macro2::Span::call_site());

        // Test with both F and Ret in the search set
        let generic_params = vec![&f_ident, &ret_ident];
        let predicate: syn::WherePredicate = syn::parse_quote!(F: JsFunction<Ret = Ret>);

        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_where_predicate(&mut visitor, &predicate);

        assert!(
            found_set.contains(&f_ident),
            "F should be found in 'F: JsFunction<Ret = Ret>'"
        );
        assert!(
            found_set.contains(&ret_ident),
            "Ret should be found in 'F: JsFunction<Ret = Ret>' (RHS of assoc type binding)"
        );
    }

    #[test]
    fn test_where_predicate_assoc_type_binding_only_rhs() {
        let f_ident = syn::Ident::new("F", proc_macro2::Span::call_site());
        let ret_ident = syn::Ident::new("Ret", proc_macro2::Span::call_site());

        // Ret in the search set
        let generic_params = vec![&ret_ident];
        let predicate: syn::WherePredicate = syn::parse_quote!(F: JsFunction<Ret = Ret>);

        let uses = crate::generics::generics_predicate_uses(&predicate, &generic_params);
        assert!(
            uses,
            "Ret should be detected as used in 'F: JsFunction<Ret = Ret>'"
        );

        // F in the search set
        let not_generic_params = vec![&f_ident];
        let uses = crate::generics::generics_predicate_uses(&predicate, &not_generic_params);
        assert!(
            uses,
            "F should not be detected as used in 'F: JsFunction<Ret = Ret>'"
        );
    }

    #[test]
    fn test_where_predicate_assoc_type_binding_only_bounded() {
        // Test that only F (not Ret) is found when Ret is not in the search set
        let f_ident = syn::Ident::new("F", proc_macro2::Span::call_site());
        let ret_ident = syn::Ident::new("Ret", proc_macro2::Span::call_site());

        // Only F in the search set
        let generic_params = vec![&f_ident];
        let predicate: syn::WherePredicate = syn::parse_quote!(F: JsFunction<Ret = Ret>);

        let uses = crate::generics::generics_predicate_uses(&predicate, &generic_params);
        assert!(
            uses,
            "F should be detected as used in 'F: JsFunction<Ret = Ret>'"
        );

        // Also verify Ret is NOT found when not in the search set
        let mut found_set = Default::default();
        let mut visitor = crate::generics::GenericNameVisitor::new(&generic_params, &mut found_set);
        syn::visit::visit_where_predicate(&mut visitor, &predicate);

        assert!(found_set.contains(&f_ident), "F should be found");
        assert!(
            !found_set.contains(&ret_ident),
            "Ret should NOT be found when not in search set"
        );
    }

    #[test]
    fn test_lifetime_params_with_bounds() {
        // No lifetimes -> empty
        let generics: syn::Generics = syn::parse_quote!(<T>);
        let result = crate::generics::lifetime_params_with_bounds(&generics);
        assert!(result.is_empty());

        // Unbounded lifetime
        let generics: syn::Generics = syn::parse_quote!(<'a, T>);
        let result = crate::generics::lifetime_params_with_bounds(&generics);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].to_string(), quote::quote!('a).to_string());

        // Bounded lifetime: 'a: 'b
        let generics: syn::Generics = syn::parse_quote!(<'a: 'b, 'b, T>);
        let result = crate::generics::lifetime_params_with_bounds(&generics);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].to_string(), quote::quote!('a: 'b).to_string());
        assert_eq!(result[1].to_string(), quote::quote!('b).to_string());
    }

    #[test]
    fn test_staticize_specific_lifetimes() {
        // Test that specified lifetimes in types are replaced with 'static
        let lifetime_a: syn::Lifetime = syn::parse_quote!('a);
        let lifetimes = [&lifetime_a];

        let ty: syn::Type = syn::parse_quote!(ScopedClosure<'a, dyn FnMut(T) -> R>);
        let result = crate::generics::staticize_lifetimes(ty, &lifetimes);
        let expected: syn::Type = syn::parse_quote!(ScopedClosure<'static, dyn FnMut(T) -> R>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Test multiple lifetimes - only staticize specified ones
        let lifetime_b: syn::Lifetime = syn::parse_quote!('b);
        let lifetimes_both = [&lifetime_a, &lifetime_b];
        let ty: syn::Type = syn::parse_quote!(&'a SomeType<'b, T>);
        let result = crate::generics::staticize_lifetimes(ty, &lifetimes_both);
        let expected: syn::Type = syn::parse_quote!(&'static SomeType<'static, T>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Test selective staticization - only 'a, not 'b
        let ty: syn::Type = syn::parse_quote!(&'a SomeType<'b, T>);
        let result = crate::generics::staticize_lifetimes(ty, &[&lifetime_a]);
        let expected: syn::Type = syn::parse_quote!(&'static SomeType<'b, T>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Test no lifetimes to staticize (should be unchanged)
        let ty: syn::Type = syn::parse_quote!(Vec<T>);
        let result = crate::generics::staticize_lifetimes(ty, &[]);
        let expected: syn::Type = syn::parse_quote!(Vec<T>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );
    }

    #[test]
    fn test_generic_to_concrete_with_lifetimes() {
        use std::borrow::Cow;
        use std::collections::BTreeMap;

        // Test that generic_to_concrete replaces both type params AND specified lifetimes
        let t: syn::Ident = syn::parse_quote!(T);
        let concrete: syn::Type = syn::parse_quote!(JsValue);
        let generic_names: BTreeMap<&syn::Ident, Option<Cow<syn::Type>>> = {
            let mut map = BTreeMap::new();
            map.insert(&t, Some(Cow::Borrowed(&concrete)));
            map
        };

        // Create the lifetime 'a that we want to staticize
        let lifetime_a: syn::Lifetime = syn::parse_quote!('a);
        let lifetimes_to_staticize = [&lifetime_a];

        // ScopedClosure<'a, dyn FnMut(T)> -> ScopedClosure<'static, dyn FnMut(JsValue)>
        let ty: syn::Type = syn::parse_quote!(ScopedClosure<'a, dyn FnMut(T)>);
        let result =
            crate::generics::generic_to_concrete(ty, &generic_names, &lifetimes_to_staticize)
                .unwrap();
        let expected: syn::Type = syn::parse_quote!(ScopedClosure<'static, dyn FnMut(JsValue)>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );

        // Test that lifetimes NOT in the list are preserved
        let _lifetime_b: syn::Lifetime = syn::parse_quote!('b);
        let lifetimes_only_a = [&lifetime_a];
        let ty: syn::Type = syn::parse_quote!(Foo<'a, 'b>);
        let result =
            crate::generics::generic_to_concrete(ty, &BTreeMap::new(), &lifetimes_only_a).unwrap();
        let expected: syn::Type = syn::parse_quote!(Foo<'static, 'b>);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );
    }

    /// Parse a type whose last path segment carries generic args and hand them
    /// to `args_are_constraining_for`. This mirrors how `class_return_path()`
    /// feeds the gate.
    fn args_are_constraining(ty_src: &str, params: &[&str]) -> bool {
        let ty: syn::Type = syn::parse_str(ty_src).expect("valid type");
        let path = match ty {
            syn::Type::Path(syn::TypePath {
                attrs: _,
                qself: None,
                path,
            }) => path,
            _ => panic!("test helper expects a bare path type"),
        };
        let seg = path.segments.last().expect("at least one segment");
        let args = match &seg.arguments {
            syn::PathArguments::AngleBracketed(a) => a.args.clone(),
            syn::PathArguments::None => Default::default(),
            syn::PathArguments::Parenthesized(_) => {
                panic!("test helper doesn't handle paren-style args at the top")
            }
        };
        let idents: Vec<syn::Ident> = params
            .iter()
            .map(|p| syn::Ident::new(p, proc_macro2::Span::call_site()))
            .collect();
        let refs: Vec<&syn::Ident> = idents.iter().collect();
        crate::generics::args_are_constraining_for(&args, &refs)
    }

    #[test]
    fn hoist_gate_accepts_bare_idents() {
        // `Array<T>` — bare param, trivially constraining.
        assert!(args_are_constraining("Array<T>", &["T"]));
        // Multiple bare params.
        assert!(args_are_constraining("Map<K, V>", &["K", "V"]));
    }

    #[test]
    fn hoist_gate_accepts_nested_nominal() {
        // `Array<Option<T>>` — T is nested inside a nominal path, still
        // constraining. This was wrongly rejected by the old bare-ident gate.
        assert!(args_are_constraining("Array<Option<T>>", &["T"]));
        // Deeply nested.
        assert!(args_are_constraining("Array<Vec<Box<T>>>", &["T"]));
        // References, arrays, tuples preserve constraining-ness.
        assert!(args_are_constraining("Foo<&T>", &["T"]));
        assert!(args_are_constraining("Foo<[T; 4]>", &["T"]));
        assert!(args_are_constraining("Foo<(T, U)>", &["T", "U"]));
    }

    #[test]
    fn hoist_gate_accepts_when_param_absent() {
        // T doesn't appear at all → nothing to hoist → vacuously safe.
        assert!(args_are_constraining("Array<i32>", &["T"]));
        assert!(args_are_constraining("Promise<JsValue>", &["T"]));
    }

    #[test]
    fn hoist_gate_rejects_qself_projection() {
        // `Promise<<T as Promising>::Resolution>` — T only appears behind a
        // projection, which is NOT constraining. This is the shape that
        // produced E0207 before the fix.
        assert!(!args_are_constraining(
            "Promise<<T as Promising>::Resolution>",
            &["T"]
        ));
    }

    #[test]
    fn hoist_gate_rejects_bare_projection() {
        // `Array<T::Item>` — T appears as the head of a multi-segment path,
        // which Rust resolves through an implicit projection. Non-constraining.
        assert!(!args_are_constraining("Array<T::Item>", &["T"]));
        // Even if U is constraining, T's non-constraining presence disqualifies
        // the whole return path (partial hoisting would still be ill-formed).
        assert!(!args_are_constraining("Foo<T::Item, U>", &["T", "U"]));
    }

    #[test]
    fn hoist_gate_rejects_fn_ptr_and_fn_sugar() {
        // `fn(T) -> U` and `Fn(T) -> U` sugar are both non-constraining slots.
        assert!(!args_are_constraining("Foo<fn(T) -> i32>", &["T"]));
        assert!(!args_are_constraining("Foo<Box<dyn Fn(T) -> i32>>", &["T"]));
        // Return-position of the parenthesized sugar also counts.
        assert!(!args_are_constraining("Foo<Box<dyn Fn(i32) -> T>>", &["T"]));
    }

    #[test]
    fn hoist_gate_rejects_assoc_type_binding_rhs() {
        // `Trait<Item = T>` — T sits behind the outer trait's projection.
        assert!(!args_are_constraining(
            "Foo<Box<dyn Iterator<Item = T>>>",
            &["T"]
        ));
    }

    #[test]
    fn desugar_impl_trait_rewrites_top_level_and_nested() {
        use super::desugar_impl_trait;

        // Top-level `impl Trait` becomes a bare synthesized type-parameter path.
        let mut ty: syn::Type = syn::parse_quote!(impl Clone);
        let mut params = Vec::new();
        desugar_impl_trait(&mut ty, &mut params);
        assert_eq!(params.len(), 1);
        let syn::GenericParam::Type(tp) = &params[0] else {
            panic!("expected a synthesized type parameter");
        };
        assert_eq!(tp.bounds.to_token_stream().to_string(), "Clone");
        // The type was rewritten to a bare path naming the synthesized param.
        assert_eq!(ty.to_token_stream().to_string(), tp.ident.to_string());

        // Nested `impl Trait` (e.g. `Vec<impl Trait>`) is rewritten in place
        // without disturbing the rest of the type.
        let mut ty: syn::Type = syn::parse_quote!(Vec<impl Clone>);
        let mut params = Vec::new();
        desugar_impl_trait(&mut ty, &mut params);
        assert_eq!(params.len(), 1);
        let syn::GenericParam::Type(tp) = &params[0] else {
            panic!("expected a synthesized type parameter");
        };
        assert_eq!(
            ty.to_token_stream().to_string(),
            format!("Vec < {} >", tp.ident)
        );

        // A type with no `impl Trait` is left completely unchanged and no
        // params are synthesized.
        let mut ty: syn::Type = syn::parse_quote!(Vec<T>);
        let original = ty.to_token_stream().to_string();
        let mut params = Vec::new();
        desugar_impl_trait(&mut ty, &mut params);
        assert!(params.is_empty());
        assert_eq!(ty.to_token_stream().to_string(), original);

        // Multiple occurrences in one type each get a distinct name, and
        // names stay unique across repeated calls sharing one `params` list
        // (mirroring how one function's multiple arguments are processed).
        let mut ty: syn::Type = syn::parse_quote!((impl Clone, impl Iterator));
        let mut params = Vec::new();
        desugar_impl_trait(&mut ty, &mut params);
        assert_eq!(params.len(), 2);
        let names: Vec<String> = params
            .iter()
            .map(|p| match p {
                syn::GenericParam::Type(tp) => tp.ident.to_string(),
                _ => panic!("expected a type parameter"),
            })
            .collect();
        assert_ne!(names[0], names[1]);

        let mut ty2: syn::Type = syn::parse_quote!(impl Debug);
        desugar_impl_trait(&mut ty2, &mut params);
        assert_eq!(params.len(), 3);
        // The third synthesized name must not collide with the first two.
        let third = match &params[2] {
            syn::GenericParam::Type(tp) => tp.ident.to_string(),
            _ => panic!("expected a type parameter"),
        };
        assert!(!names.contains(&third));
    }
}
