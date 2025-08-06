//! Common utility function for manipulating syn types and
//! handling parsed values

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use syn::visit::Visit;

use crate::ast;
use proc_macro2::{self, Ident};

/// Check whether a given `&str` is a Rust keyword
#[rustfmt::skip]
fn is_rust_keyword(name: &str) -> bool {
    matches!(name,
        "abstract" | "alignof" | "as" | "become" | "box" | "break" | "const" | "continue"
        | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut"
        | "offsetof" | "override" | "priv" | "proc" | "pub" | "pure" | "ref" | "return"
        | "Self" | "self" | "sizeof" | "static" | "struct" | "super" | "trait" | "true"
        | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
        | "yield" | "bool" | "_"
    )
}

/// Create an `Ident`, possibly mangling it if it conflicts with a Rust keyword.
pub fn rust_ident(name: &str) -> Ident {
    if name.is_empty() {
        panic!("tried to create empty Ident (from \"\")");
    } else if is_rust_keyword(name) {
        Ident::new(&format!("{}_", name), proc_macro2::Span::call_site())

    // we didn't historically have `async` in the `is_rust_keyword` list above,
    // so for backwards compatibility reasons we need to generate an `async`
    // identifier as well, but we'll be sure to use a raw identifier to ease
    // compatibility with the 2018 edition.
    //
    // Note, though, that `proc-macro` doesn't support a normal way to create a
    // raw identifier. To get around that we do some wonky parsing to
    // roundaboutly create one.
    } else if name == "async" {
        let ident = "r#async"
            .parse::<proc_macro2::TokenStream>()
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        match ident {
            proc_macro2::TokenTree::Ident(i) => i,
            _ => unreachable!(),
        }
    } else if name.chars().next().unwrap().is_ascii_digit() {
        Ident::new(&format!("N{}", name), proc_macro2::Span::call_site())
    } else {
        raw_ident(name)
    }
}

/// Create an `Ident` without checking to see if it conflicts with a Rust
/// keyword.
pub fn raw_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

/// Create a path type from the given segments. For example an iterator yielding
/// the idents `[foo, bar, baz]` will result in the path type `foo::bar::baz`.
pub fn simple_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    path_ty(false, segments)
}

/// Create a global path type from the given segments. For example an iterator
/// yielding the idents `[foo, bar, baz]` will result in the path type
/// `::foo::bar::baz`.
pub fn leading_colon_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    path_ty(true, segments)
}

fn path_ty<I>(leading_colon: bool, segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    let segments: Vec<_> = segments
        .into_iter()
        .map(|i| syn::PathSegment {
            ident: i,
            arguments: syn::PathArguments::None,
        })
        .collect();

    syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: if leading_colon {
                Some(Default::default())
            } else {
                None
            },
            segments: syn::punctuated::Punctuated::from_iter(segments),
        },
    }
    .into()
}

/// Create a path type with a single segment from a given Identifier
pub fn ident_ty(ident: Ident) -> syn::Type {
    simple_path_ty(Some(ident))
}

/// Convert an ImportFunction into the more generic Import type, wrapping the provided function
pub fn wrap_import_function(function: ast::ImportFunction) -> ast::Import {
    ast::Import {
        module: None,
        js_namespace: None,
        kind: ast::ImportKind::Function(function),
    }
}

/// Small utility used when generating symbol names.
///
/// Hashes the public field here along with a few cargo-set env vars to
/// distinguish between runs of the procedural macro.
#[derive(Debug)]
pub struct ShortHash<T>(pub T);

impl<T: Hash> fmt::Display for ShortHash<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        static HASHED: AtomicBool = AtomicBool::new(false);
        static HASH: AtomicUsize = AtomicUsize::new(0);

        // Try to amortize the cost of loading env vars a lot as we're gonna be
        // hashing for a lot of symbols.
        if !HASHED.load(SeqCst) {
            let mut h = DefaultHasher::new();
            env::var("CARGO_PKG_NAME")
                .expect("should have CARGO_PKG_NAME env var")
                .hash(&mut h);
            env::var("CARGO_PKG_VERSION")
                .expect("should have CARGO_PKG_VERSION env var")
                .hash(&mut h);
            // This may chop off 32 bits on 32-bit platforms, but that's ok, we
            // just want something to mix in below anyway.
            HASH.store(h.finish() as usize, SeqCst);
            HASHED.store(true, SeqCst);
        }

        let mut h = DefaultHasher::new();
        HASH.load(SeqCst).hash(&mut h);
        self.0.hash(&mut h);
        write!(f, "{:016x}", h.finish())
    }
}

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
