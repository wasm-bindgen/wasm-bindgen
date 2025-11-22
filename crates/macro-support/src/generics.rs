use syn::visit::Visit;

use crate::error::Diagnostic;

/// Helper visitor for generic parameter usage
#[derive(Debug)]
pub struct GenericNameVisitor<'a> {
    name_set_a: &'a Vec<&'a syn::Ident>,
    name_set_b: Option<&'a Vec<&'a syn::Ident>>,
    /// Was a generic parameter in name set A found?
    pub found_a: bool,
    /// Were all generic parameters in name set A reference usage?
    pub a_ref_only: bool,
    /// Was a generic parameter in name set B found?
    pub found_b: bool,
    /// Were all generic parameters in name set B reference usage?
    pub b_ref_only: bool,
}

/// Helper visitor for generic parameter usage
impl<'a> GenericNameVisitor<'a> {
    /// Construct a new generic name visitors with a param search set,
    /// and optionally a second parameter search set.
    pub fn new(
        name_set_a: &'a Vec<&'a syn::Ident>,
        name_set_b: Option<&'a Vec<&'a syn::Ident>>,
    ) -> Self {
        Self {
            name_set_a,
            name_set_b,
            found_a: false,
            a_ref_only: true,
            found_b: false,
            b_ref_only: true,
        }
    }
}

/// Adds a lifetime parameter to the generics if not already present.
/// Returns the lifetime that was added or found.
pub(crate) fn add_lifetime(generics: &mut syn::Generics, lifetime_name: &str) -> syn::Lifetime {
    let lifetime: syn::Lifetime = syn::parse_str(lifetime_name)
        .unwrap_or_else(|_| panic!("Invalid lifetime name: {}", lifetime_name));
    generics.params.insert(
        0,
        syn::GenericParam::Lifetime(syn::LifetimeParam {
            attrs: vec![],
            lifetime: lifetime.clone(),
            colon_token: None,
            bounds: syn::punctuated::Punctuated::new(),
        }),
    );
    lifetime
}

impl<'a> Visit<'_> for GenericNameVisitor<'a> {
    fn visit_type_reference(&mut self, type_ref: &syn::TypeReference) {
        if let syn::Type::Path(type_path) = &*type_ref.elem {
            if let Some(first_segment) = type_path.path.segments.first() {
                if type_path.path.segments.len() == 1 && first_segment.arguments.is_empty() {
                    if self.name_set_a.contains(&&first_segment.ident) {
                        self.found_a = true;
                        return;
                    }
                    if let Some(name_set_b) = self.name_set_b {
                        if name_set_b.contains(&&first_segment.ident) {
                            self.found_b = true;
                            return;
                        }
                    }
                } else {
                    if self.name_set_a.contains(&&first_segment.ident) {
                        self.found_a = true;
                    }
                    if let Some(name_set_b) = self.name_set_b {
                        if name_set_b.contains(&&first_segment.ident) {
                            self.found_b = true;
                        }
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

    fn visit_path(&mut self, path: &syn::Path) {
        if let Some(first_segment) = path.segments.first() {
            if self.name_set_a.contains(&&first_segment.ident) {
                self.found_a = true;
                self.a_ref_only = false; // This is value usage
            }
            if let Some(name_set_b) = self.name_set_b {
                if name_set_b.contains(&&first_segment.ident) {
                    self.found_b = true;
                    self.b_ref_only = false;
                }
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

/// Get the list of generic parameter identifier names
pub(crate) fn generic_params(generics: &syn::Generics) -> Vec<&syn::Ident> {
    generics.type_params().map(|tp| &tp.ident).collect()
}

pub(crate) fn uses_generic_params(ty: &syn::Type, generic_names: &Vec<&syn::Ident>) -> bool {
    let mut visitor = GenericNameVisitor::new(generic_names, None);
    syn::visit::visit_type(&mut visitor, ty);
    visitor.found_a
}

// TODO: (1) this should be recursive, (2) this should erase all of <A, B, C, ...> if any of A/B/C are generic.
pub(crate) fn strip_local_generic_args(
    ty: &syn::Type,
    generic_names: &Vec<&syn::Ident>,
) -> Result<syn::Type, Diagnostic> {
    match ty {
        syn::Type::Path(type_path) => {
            let mut new_path = type_path.clone();

            for segment in &mut new_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let mut has_local = false;
                    let mut has_concrete = false;

                    for arg in &args.args {
                        if let syn::GenericArgument::Type(ty) = arg {
                            let mut visitor = GenericNameVisitor::new(generic_names, None);
                            syn::visit::visit_type(&mut visitor, ty);

                            if visitor.found_a {
                                has_local = true;
                            } else {
                                has_concrete = true;
                            }
                        }
                    }

                    // Error if mixing concrete and local generics
                    if has_local && has_concrete {
                        bail_span!(
                            segment,
                            "Type mixes concrete type arguments with local generic parameters, which is not supported for imported function bindgen"
                        );
                    }

                    // Strip if uses local generics
                    if has_local {
                        segment.arguments = syn::PathArguments::None;
                    }
                }
            }

            Ok(syn::Type::Path(new_path))
        }
        syn::Type::Reference(type_ref) => Ok(syn::Type::Reference(syn::TypeReference {
            and_token: type_ref.and_token,
            lifetime: type_ref.lifetime.clone(),
            mutability: type_ref.mutability,
            elem: Box::new(strip_local_generic_args(&type_ref.elem, generic_names)?),
        })),
        syn::Type::Ptr(type_ptr) => Ok(syn::Type::Ptr(syn::TypePtr {
            star_token: type_ptr.star_token,
            const_token: type_ptr.const_token,
            mutability: type_ptr.mutability,
            elem: Box::new(strip_local_generic_args(&type_ptr.elem, generic_names)?),
        })),
        // For other types, return as-is
        _ => Ok(ty.clone()),
    }
}

/// Normalizes generics by moving inline trait bounds to where clauses.
/// This makes it easier to hoist bounds during code generation.
pub(crate) fn normalize_generics(generics: &mut syn::Generics) {
    let mut new_predicates =
        syn::punctuated::Punctuated::<syn::WherePredicate, syn::Token![,]>::new();

    for param in &mut generics.params {
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
                new_predicates.push(predicate);
                type_param.bounds.clear();
            }
        }
    }

    if !new_predicates.is_empty() {
        generics
            .make_where_clause()
            .predicates
            .extend(new_predicates);
    }
}

mod tests {
    #[test]
    fn test_generic_name_visitor() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let u_ident = syn::Ident::new("U", proc_macro2::Span::call_site());
        let name_set_a = vec![&t_ident];
        let name_set_b = Some(vec![&u_ident]);

        // Test T as value
        let ty: syn::Type = syn::parse_quote!(T);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(!visitor.a_ref_only);

        // Test &T as reference
        let ty: syn::Type = syn::parse_quote!(&T);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(visitor.a_ref_only);

        // Test T<U> - T as value, U as value
        let ty: syn::Type = syn::parse_quote!(T<U>);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(!visitor.a_ref_only);
        assert!(visitor.found_b);
        assert!(!visitor.b_ref_only);

        // Test &T<U> - T as reference, U as value
        let ty: syn::Type = syn::parse_quote!(&T<U>);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(visitor.a_ref_only);
        assert!(visitor.found_b);
        assert!(!visitor.b_ref_only);

        // Test T::<U>::Foo - T as value, U as value, Foo ignored
        let ty: syn::Type = syn::parse_quote!(T::<U>::Foo);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(!visitor.a_ref_only);
        assert!(visitor.found_b);
        assert!(!visitor.b_ref_only);

        // Test Vec<T> - T as value, Vec ignored
        let ty: syn::Type = syn::parse_quote!(Vec<T>);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(!visitor.a_ref_only);
    }

    #[test]
    fn test_associated_type_binding() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let u_ident = syn::Ident::new("U", proc_macro2::Span::call_site());
        let name_set_a = vec![&t_ident];
        let name_set_b = Some(vec![&u_ident]);

        // Test SomeTrait<T = U> - should find U (RHS) but NOT T (LHS assoc type name)
        let ty: syn::Type = syn::parse_quote!(SomeTrait<T = U>);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(!visitor.found_a); // T is LHS assoc type name, should NOT be counted
        assert!(visitor.found_b); // U is RHS generic parameter, should be counted
        assert!(!visitor.b_ref_only);

        // Test SomeTrait<U = T> - should find T (RHS) but NOT U (LHS assoc type name)
        let ty: syn::Type = syn::parse_quote!(SomeTrait<U = T>);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a); // T is RHS generic parameter, should be counted
        assert!(!visitor.a_ref_only);
        assert!(!visitor.found_b); // U is LHS assoc type name, should NOT be counted
    }

    #[test]
    fn test_nested_references() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let u_ident = syn::Ident::new("U", proc_macro2::Span::call_site());
        let name_set_a = vec![&t_ident];
        let name_set_b = Some(vec![&u_ident]);

        // Test &T - should be ref
        let ty: syn::Type = syn::parse_quote!(&T);
        let mut visitor = crate::generics::GenericNameVisitor::new(&name_set_a, None);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(visitor.a_ref_only);

        // Test &&T - should be ref
        let ty: syn::Type = syn::parse_quote!(&&T);
        let mut visitor = crate::generics::GenericNameVisitor::new(&name_set_a, None);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(visitor.a_ref_only);

        // Test &&&T - should be ref
        let ty: syn::Type = syn::parse_quote!(&&&T);
        let mut visitor = crate::generics::GenericNameVisitor::new(&name_set_a, None);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(visitor.a_ref_only);

        // Test &T<U> - T should be ref, U should be value
        let ty: syn::Type = syn::parse_quote!(&T<U>);
        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(visitor.a_ref_only);
        assert!(visitor.found_b);
        assert!(!visitor.b_ref_only);
    }

    #[test]
    fn test_mixed_usage() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let name_set_a = vec![&t_ident];

        // Test T + &T - should find both, ref_only = false
        let ty: syn::Type = syn::parse_quote!(SomeTrait<Item = T> + OtherTrait<Ref = &T>);
        let mut visitor = crate::generics::GenericNameVisitor::new(&name_set_a, None);
        syn::visit::visit_type(&mut visitor, &ty);
        assert!(visitor.found_a);
        assert!(!visitor.a_ref_only); // Found both ref and value usage
    }

    #[test]
    fn test_complex_reference_with_closure() {
        let t_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
        let r_ident = syn::Ident::new("R", proc_macro2::Span::call_site());
        let name_set_a = vec![&t_ident];
        let name_set_b = Some(vec![&r_ident]);

        let ty: syn::Type = syn::parse_quote!(&Closure<dyn FnMut(T) -> Result<R, JsValue>>);

        let mut visitor =
            crate::generics::GenericNameVisitor::new(&name_set_a, name_set_b.as_ref());
        syn::visit::visit_type(&mut visitor, &ty);

        assert!(visitor.found_a);
        assert!(!visitor.a_ref_only);
        assert!(visitor.found_b);
        assert!(!visitor.b_ref_only);
    }

    #[test]
    fn test_strip_local_generic_args_mixed_errors() {
        let generic_t: syn::Ident = syn::parse_quote!(T);
        let generic_names = vec![&generic_t];

        let mixed_type: syn::Type = syn::parse_quote!(Promise<i32, T>);
        assert!(crate::generics::strip_local_generic_args(&mixed_type, &generic_names).is_err());

        let concrete_type: syn::Type = syn::parse_quote!(Promise<i32, String>);
        assert!(crate::generics::strip_local_generic_args(&concrete_type, &generic_names).is_ok());

        let generic_type: syn::Type = syn::parse_quote!(Promise<T>);
        let result =
            crate::generics::strip_local_generic_args(&generic_type, &generic_names).unwrap();
        let expected: syn::Type = syn::parse_quote!(Promise);
        assert_eq!(
            quote::quote!(#result).to_string(),
            quote::quote!(#expected).to_string()
        );
    }
}
