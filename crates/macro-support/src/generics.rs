use std::borrow::Cow;
use std::collections::BTreeSet;
use syn::visit_mut::VisitMut;
use syn::{visit::Visit, Ident};

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
                    if self.generic_params.contains(&&first_segment.ident) {
                        self.found_set.insert(first_segment.ident.clone());
                        return;
                    }
                } else {
                    if self.generic_params.contains(&&first_segment.ident) {
                        self.found_set.insert(first_segment.ident.clone());
                    }

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
            if self.generic_params.contains(&&first_segment.ident) {
                self.found_set.insert(first_segment.ident.clone());
            }
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
                        syn::visit::visit_type(self, input);
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
        .map(|tp| (&tp.ident, tp.default.as_ref()))
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
/// Obtain the generic bounds, both inline and where clauses together
pub(crate) fn generic_bounds<'a>(generics: &'a syn::Generics) -> Vec<Cow<'a, syn::WherePredicate>> {
    let mut bounds = Vec::new();
    for param in &generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            if !type_param.bounds.is_empty() {
                let ident = &type_param.ident;
                let predicate = syn::WherePredicate::Type(syn::PredicateType {
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

/// Obtain all lifetime parameters from generics
pub(crate) fn lifetime_params(generics: &syn::Generics) -> Vec<&syn::Lifetime> {
    generics.lifetimes().map(|lp| &lp.lifetime).collect()
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
                            if type_mentions_any(input, generic_names) {
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
        // Pointer / BareFn / TraitObject / ImplTrait / Infer / Never / Macro:
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

#[cfg(test)]
mod tests {
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

    /// Parse a type whose last path segment carries generic args and hand them
    /// to `args_are_constraining_for`. This mirrors how `class_return_path()`
    /// feeds the gate.
    fn args_are_constraining(ty_src: &str, params: &[&str]) -> bool {
        let ty: syn::Type = syn::parse_str(ty_src).expect("valid type");
        let path = match ty {
            syn::Type::Path(syn::TypePath { qself: None, path }) => path,
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
}
