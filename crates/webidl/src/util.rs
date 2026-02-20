use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use weedle::attribute::{
    ExtendedAttribute, ExtendedAttributeIdent, ExtendedAttributeList, ExtendedAttributeNoArgs,
    IdentifierOrString,
};
use weedle::common::Identifier;
use weedle::literal::{ConstValue as ConstValueLit, FloatLit, IntegerLit};
use weedle::types::{MayBeNull, NonAnyType, SingleType};

use crate::constants::{
    BREAKING_ALLOW_SHARED, BREAKING_GETTER_THROWS, BREAKING_SETTER_THROWS, FIXED_INTERFACES,
    IMMUTABLE_SLICE_WHITELIST,
};
use crate::first_pass::{FirstPassRecord, OperationData, OperationId, Signature};
use crate::generator::{ConstValue, InterfaceMethod, InterfaceMethodKind};
use crate::wbg_type::{IdentifierType, ToWbgType, WbgType};
use crate::Options;
use syn::parse_quote;

/// For variadic operations an overload with a `js_sys::Array` argument is generated alongside with
/// `operation_name_0`, `operation_name_1`, `operation_name_2`, ..., `operation_name_n` overloads
/// which have the count of arguments for passing values to the variadic argument
/// in their names, where `n` is this constant.
const MAX_VARIADIC_ARGUMENTS_COUNT: usize = 7;

/// Similar to std::fs::read_dir except it returns a sorted Vec,
/// which is important to make the code generation deterministic.
pub(crate) fn read_dir<P>(path: P) -> std::io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut entries = fs::read_dir(path)?
        .map(|entry| Ok(entry?.path()))
        .collect::<std::io::Result<Vec<_>>>()?;

    entries.sort();

    Ok(entries)
}

/// Take a type and create an immutable shared reference to that type.
pub(crate) fn shared_ref(ty: syn::Type, mutable: bool) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: if mutable {
            Some(syn::token::Mut::default())
        } else {
            None
        },
        elem: Box::new(ty),
    }
    .into()
}

/// Fix case of identifiers like `HTMLBRElement` or `texImage2D`
fn fix_ident(identifier: &str) -> String {
    identifier
        .replace("HTML", "HTML_")
        .replace("1D", "_1d")
        .replace("2D", "_2d")
        .replace("3D", "_3d")
}

/// Convert an identifier to camel case
pub fn camel_case_ident(identifier: &str) -> String {
    fix_ident(identifier).to_upper_camel_case()
}

/// Convert an identifier to shouty snake case
pub fn shouty_snake_case_ident(identifier: &str) -> String {
    fix_ident(identifier).to_shouty_snake_case()
}

/// Convert an identifier to snake case
pub fn snake_case_ident(identifier: &str) -> String {
    fix_ident(identifier).to_snake_case()
}

/// Wrap [`TypePosition::Return`] type into an `Option` if not already and if not a `JsValue`.
pub fn optional_return_ty(ty: syn::Type) -> syn::Type {
    if let syn::Type::Path(path) = &ty {
        if let Some(segment) = path.path.segments.first() {
            if segment.ident == "Option" {
                return ty;
            } else if path.path.leading_colon.is_some() && segment.ident == "wasm_bindgen" {
                if let Some(segment) = path.path.segments.iter().nth(1) {
                    if segment.ident == "JsValue" {
                        return ty;
                    }
                }
            }
        }
    }

    option_ty(ty)
}

// Returns a link to MDN
pub fn mdn_doc(class: &str, method: Option<&str>) -> String {
    let mut link = format!("https://developer.mozilla.org/en-US/docs/Web/API/{class}");
    if let Some(method) = method {
        link.push_str(&format!("/{method}"));
    }
    format!("[MDN Documentation]({link})")
}

// Array type is borrowed for arguments (`&mut [T]` or `&[T]`) and owned for return value (`Vec<T>`).
pub(crate) fn array(base_ty: &str, pos: TypePosition, immutable: bool) -> syn::Type {
    if pos.is_argument() && !pos.inner {
        shared_ref(
            slice_ty(ident_ty(raw_ident(base_ty))),
            /*mutable =*/ !immutable,
        )
    } else {
        vec_ty(ident_ty(raw_ident(base_ty)))
    }
}

/// Map a webidl const value to the correct wasm-bindgen const value
pub fn webidl_const_v_to_backend_const_v(v: &ConstValueLit) -> ConstValue {
    match *v {
        ConstValueLit::Boolean(b) => ConstValue::Boolean(b.0),
        ConstValueLit::Float(FloatLit::NegInfinity(_)) => ConstValue::Float(f64::NEG_INFINITY),
        ConstValueLit::Float(FloatLit::Infinity(_)) => ConstValue::Float(f64::INFINITY),
        ConstValueLit::Float(FloatLit::NaN(_)) => ConstValue::Float(f64::NAN),
        ConstValueLit::Float(FloatLit::Value(s)) => ConstValue::Float(s.0.parse().unwrap()),
        ConstValueLit::Integer(lit) => {
            let mklit = |orig_text: &str, base: u32, offset: usize| {
                let (negative, text) = if let Some(text) = orig_text.strip_prefix('-') {
                    (true, text)
                } else {
                    (false, orig_text)
                };
                if text == "0" {
                    return ConstValue::SignedInteger(0);
                }
                let text = &text[offset..];
                let n = u64::from_str_radix(text, base)
                    .unwrap_or_else(|_| panic!("literal too big: {orig_text}"));
                if negative {
                    let n = if n > (i64::MIN as u64).wrapping_neg() {
                        panic!("literal too big: {orig_text}")
                    } else {
                        n.wrapping_neg() as i64
                    };
                    ConstValue::SignedInteger(n)
                } else {
                    ConstValue::UnsignedInteger(n)
                }
            };
            match lit {
                IntegerLit::Hex(h) => mklit(h.0, 16, 2), // leading 0x
                IntegerLit::Oct(h) => mklit(h.0, 8, 1),  // leading 0
                IntegerLit::Dec(h) => mklit(h.0, 10, 0),
            }
        }
        ConstValueLit::Null(_) => unimplemented!(),
    }
}

/// From `T` create `[T]`.
pub(crate) fn slice_ty(t: syn::Type) -> syn::Type {
    syn::TypeSlice {
        bracket_token: Default::default(),
        elem: Box::new(t),
    }
    .into()
}

/// From `T` create `alloc::Vec<T>`.
pub(crate) fn vec_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![syn::GenericArgument::Type(t)]),
        gt_token: Default::default(),
    });

    let mut path = syn::Path {
        leading_colon: Some(Default::default()),
        segments: Punctuated::new(),
    };
    path.segments.push(raw_ident("alloc").into());
    path.segments.push(raw_ident("vec").into());
    path.segments.push(syn::PathSegment {
        ident: raw_ident("Vec"),
        arguments,
    });
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// From `T` create `Option<T>`
pub(crate) fn option_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![syn::GenericArgument::Type(t)]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Option");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// From `T` create `::js_sys::JsOption<T>`
///
/// Used for nullable types nested inside generic containers (e.g., `Promise<JsOption<Foo>>`).
/// Unlike `Option<T>` which is a Rust ABI, `JsOption<T>` is a valid erasable generic type.
pub(crate) fn js_option_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![syn::GenericArgument::Type(t)]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("JsOption");
    let seg = syn::PathSegment { ident, arguments };
    let path = syn::Path {
        leading_colon: Some(Default::default()),
        segments: FromIterator::from_iter(vec![syn::PathSegment::from(raw_ident("js_sys")), seg]),
    };
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// Check if a type is `::wasm_bindgen::JsValue`
pub(crate) fn is_js_value(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        let segments: Vec<_> = type_path.path.segments.iter().collect();
        if segments.len() == 2 {
            return segments[0].ident == "wasm_bindgen" && segments[1].ident == "JsValue";
        }
    }
    false
}

/// From `base_path` and `T` create `base_path<T>`, unless T is JsValue (then just return base)
/// For example: `js_sys::Array` + `i32` → `js_sys::Array<i32>`
/// But: `js_sys::Promise` + `JsValue` → `js_sys::Promise`
pub(crate) fn generic_ty(base: syn::Type, type_arg: syn::Type) -> syn::Type {
    // If inner type is JsValue, omit the generic (JsValue is the default)
    if is_js_value(&type_arg) {
        return base;
    }

    // Extract the path from the base type
    let mut path = match base {
        syn::Type::Path(type_path) => type_path.path,
        _ => panic!("Expected TypePath for generic base, got {base:?}"),
    };

    // Add generic argument to the last segment
    if let Some(last_seg) = path.segments.last_mut() {
        last_seg.arguments =
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: FromIterator::from_iter(vec![syn::GenericArgument::Type(type_arg)]),
                gt_token: Default::default(),
            });
    }

    syn::Type::Path(syn::TypePath { qself: None, path })
}

/// Direction of data flow across the JS/Wasm boundary.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Data flowing from Rust to JS (function arguments, callback returns)
    Argument,
    /// Data flowing from JS to Rust (function returns, callback arguments)
    Return,
}

/// Position of a type in a function signature.
///
/// This models where a type appears, which affects how it's converted to Rust:
/// - Top-level positions (`inner: false`) can use Rust-native types (`String`, `Option<T>`)
/// - Inner positions (`inner: true`) must use JS-compatible types (`JsString`, `JsOption<T>`)
///
/// Inner positions include:
/// - Nested inside generic type parameters (e.g., inside `Promise<T>`, `Array<T>`)
/// - Callback function signatures (since callbacks become `&Function`, types are erased)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TypePosition {
    pub direction: Direction,
    /// Whether this type is nested inside a generic or callback.
    /// When true, must use JS-compatible types.
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

    /// Convert to inner position (for generic type parameters or callbacks).
    pub fn to_inner(self) -> Self {
        Self {
            direction: self.direction,
            inner: true,
        }
    }

    /// Check if this is an argument position (top-level or inner).
    pub fn is_argument(self) -> bool {
        matches!(self.direction, Direction::Argument)
    }

    /// Check if this is a return position (top-level or inner).
    pub fn is_return(self) -> bool {
        matches!(self.direction, Direction::Return)
    }
}

impl<'src> FirstPassRecord<'src> {
    pub fn create_imports(
        &self,
        type_name: Option<&str>,
        container_attrs: Option<&ExtendedAttributeList<'src>>,
        id: &'src OperationId<'src>,
        data: &'src OperationData<'src>,
        unstable: bool,
        unstable_types: &HashSet<Identifier>,
    ) -> Vec<InterfaceMethod<'_>> {
        let is_static = data.is_static;

        // First up, prune all signatures that reference unsupported arguments.
        // We won't consider these until said arguments are implemented.
        //
        // Note that we handle optional arguments as well. Optional arguments
        // should only appear at the end of argument lists and when we see one
        // we can simply push our signature so far onto the list for the
        // signature where that and all remaining optional arguments are
        // undefined.
        let mut signatures = Vec::new();
        let saved_next_unstable = self.options.next_unstable.get();
        for signature in data.signatures.iter() {
            // Signatures from unstable IDL definitions always use typed generics
            // for WbgType expansion (callbacks become typed, etc.)
            if unstable || signature.stability.is_unstable() {
                self.options.next_unstable.set(true);
            }

            fn pass<'src>(
                this: &FirstPassRecord<'src>,
                id: &'src OperationId<'_>,
                signatures: &mut Vec<(&Signature<'src>, Vec<Option<WbgType<'src>>>)>,
                signature: &'src Signature<'_>,
                mut idl_args: Vec<Option<WbgType<'src>>>,
            ) {
                for (i, arg) in signature.args.iter().enumerate().skip(idl_args.len()) {
                    if arg.optional {
                        if signature.args[i..]
                            .iter()
                            .all(|arg| arg.optional || arg.variadic)
                        {
                            signatures.push((signature, idl_args.clone()));
                        } else if signature.args.get(i + 1).is_some() {
                            let mut idl_args = idl_args.clone();
                            idl_args.push(None);
                            pass(this, id, signatures, signature, idl_args)
                        }
                    }

                    let idl_type = arg.ty.to_wbg_type(this);
                    let idl_type = this.maybe_adjust(arg.attributes, idl_type, id);
                    idl_args.push(Some(idl_type));
                }
                signatures.push((signature, idl_args));
            }

            let idl_args = Vec::with_capacity(signature.args.len());
            pass(self, id, &mut signatures, signature, idl_args);

            // Restore the original setting
            self.options.next_unstable.set(saved_next_unstable);
        }

        // Next expand all the signatures in `data` into all signatures that
        // we're going to generate. These signatures will be used to determine
        // the names for all the various functions.
        #[derive(Clone, PartialEq)]
        struct ExpandedSig<'a> {
            orig: &'a Signature<'a>,
            args: Vec<Option<WbgType<'a>>>,
        }

        let mut actual_signatures = Vec::new();
        for (signature, idl_args) in signatures.iter() {
            let start = actual_signatures.len();

            // Start off with an empty signature, this'll handle zero-argument
            // cases and otherwise the loop below will continue to add on to this.
            actual_signatures.push(ExpandedSig {
                orig: signature,
                args: Vec::with_capacity(signature.args.len()),
            });

            for (i, idl_type) in idl_args.iter().enumerate() {
                // small sanity check
                assert!(start < actual_signatures.len());

                // The first element of the flattened type gets pushed directly
                // in-place, but all other flattened types will cause new
                // signatures to be created.
                let cur = actual_signatures.len();

                if let Some(idl_type) = idl_type {
                    for (j, idl_type) in idl_type
                        .flatten(signature.attrs.as_ref())
                        .into_iter()
                        .enumerate()
                    {
                        for k in start..cur {
                            if j == 0 {
                                actual_signatures[k].args.push(Some(idl_type.clone()));
                            } else {
                                let mut sig = actual_signatures[k].clone();
                                assert_eq!(sig.args.len(), i + 1);
                                sig.args.truncate(i);
                                sig.args.push(Some(idl_type.clone()));
                                actual_signatures.push(sig);
                            }
                        }
                    }
                } else {
                    for signature in actual_signatures.iter_mut().take(cur).skip(start) {
                        signature.args.push(None);
                    }
                }
            }
        }
        let (js_name, kind, force_structural, force_throws) = match id {
            // Constructors aren't annotated with `[Throws]` extended attributes
            // (how could they be, since they themselves are extended
            // attributes?) so we must conservatively assume that they can
            // always throw.
            //
            // From https://heycam.github.io/webidl/#Constructor (emphasis
            // mine):
            //
            // > The prose definition of a constructor must either return an IDL
            // > value of a type corresponding to the interface the
            // > `[Constructor]` extended attribute appears on, **or throw an
            // > exception**.
            OperationId::Constructor(_) => {
                ("new", InterfaceMethodKind::Constructor(None), false, true)
            }
            OperationId::NamedConstructor(n) => (
                "new",
                InterfaceMethodKind::Constructor(Some(n.0.to_string())),
                false,
                true,
            ),
            OperationId::Operation(Some(s)) => (*s, InterfaceMethodKind::Regular, false, false),
            OperationId::Operation(None) => {
                log::warn!("unsupported unnamed operation");
                return Vec::new();
            }
            OperationId::IndexingGetter => {
                ("get", InterfaceMethodKind::IndexingGetter, true, false)
            }
            OperationId::IndexingSetter => {
                ("set", InterfaceMethodKind::IndexingSetter, true, false)
            }
            OperationId::IndexingDeleter => {
                ("delete", InterfaceMethodKind::IndexingDeleter, true, false)
            }
        };

        // Classify each expanded signature as stable or unstable.
        let mut stable_signatures: Vec<usize> = Vec::new();
        let mut unstable_signatures: Vec<usize> = Vec::new();

        for (idx, signature) in actual_signatures.iter().enumerate() {
            let has_unstable_args = signature.args.iter().any(|arg| {
                arg.as_ref()
                    .is_some_and(|arg| is_idl_type_unstable(arg, unstable_types))
            });
            let sig_unstable =
                unstable || signature.orig.stability.is_unstable() || has_unstable_args;

            if sig_unstable {
                unstable_signatures.push(idx);
            } else {
                stable_signatures.push(idx);
            }
        }

        // For signatures from the SAME original definition as an unstable signature,
        // include them in the unstable set too. This handles optional unstable args:
        // e.g. `read(optional UnstableType x = {})` expands to `read()` and `read(x)`.
        // `read()` is stable, but it's a sibling of the unstable `read(x)` (same orig),
        // so it should appear in both sets.
        //
        // This does NOT apply across different definitions: e.g. stable `put(f64)`
        // and unstable `put(i32)` come from different definitions, so the stable version
        // is NOT added to the unstable set.
        {
            let unstable_origs: HashSet<&Signature<'_>> = unstable_signatures
                .iter()
                .map(|&idx| actual_signatures[idx].orig)
                .collect();
            for (idx, signature) in actual_signatures.iter().enumerate() {
                if !unstable_signatures.contains(&idx) && unstable_origs.contains(signature.orig) {
                    unstable_signatures.push(idx);
                }
            }
        }

        fn idl_arguments<'a: 'b, 'b>(
            args: impl Iterator<Item = (String, &'b WbgType<'a>)>,
        ) -> Option<Vec<(Ident, WbgType<'a>)>> {
            let mut output = vec![];
            for (name, idl_type) in args {
                if idl_type
                    .to_syn_type(TypePosition::ARGUMENT, false, true)
                    .is_err()
                {
                    return None;
                }
                output.push((rust_ident(&snake_case_ident(&name[..])), idl_type.clone()));
            }
            Some(output)
        }

        fn compute_rust_name<'a>(
            signature: &ExpandedSig<'a>,
            disambiguate_against: &[usize],
            all_signatures: &[ExpandedSig<'a>],
            js_name: &str,
        ) -> String {
            let mut rust_name = snake_case_ident(js_name);
            let mut first = true;

            for (i, arg) in signature
                .args
                .iter()
                .enumerate()
                .filter_map(|(i, ty)| ty.as_ref().map(|ty| (i, ty)))
            {
                let mut any_same_name = false;
                let mut any_different_type = false;
                let mut any_different = false;
                let arg_name = signature.orig.args[i].name;

                for &other_idx in disambiguate_against.iter() {
                    let other = &all_signatures[other_idx];
                    if signature == other {
                        continue;
                    }
                    if other.orig.args.get(i).map(|s| s.name) == Some(arg_name) {
                        any_same_name = true;
                    }
                    if let Some(Some(other_arg)) = other.args.get(i) {
                        if other_arg != arg {
                            any_different_type = true;
                            any_different = true;
                        }
                    } else {
                        any_different = true;
                    }
                }

                if !any_different {
                    continue;
                }
                if first {
                    rust_name.push_str("_with_");
                    first = false;
                } else {
                    rust_name.push_str("_and_");
                }

                if any_same_name && any_different_type {
                    arg.push_snake_case_name(&mut rust_name);
                } else {
                    rust_name.push_str(&snake_case_ident(arg_name));
                }
            }

            rust_name
        }

        fn create_method<'a>(
            first_pass: &FirstPassRecord<'a>,
            signature: &ExpandedSig<'a>,
            rust_name: &str,
            js_name: &str,
            type_name: Option<&str>,
            kind: &InterfaceMethodKind,
            id: &OperationId<'_>,
            is_static: bool,
            force_structural: bool,
            force_throws: bool,
            container_attrs: Option<&ExtendedAttributeList<'_>>,
            unstable_flag: bool,
            has_unstable_override: bool,
        ) -> Option<InterfaceMethod<'a>> {
            let ret_ty = signature.orig.ret.to_wbg_type(first_pass);
            let structural =
                force_structural || is_structural(signature.orig.attrs.as_ref(), container_attrs);
            let catch = force_throws
                || throws(signature.orig.attrs)
                || (signature
                    .args
                    .iter()
                    .filter_map(Option::as_ref)
                    .any(arg_throws)
                    && type_name
                        .and_then(|type_name| BREAKING_ALLOW_SHARED.get(type_name))
                        .filter(|list| list.contains(&js_name))
                        .is_none());
            let deprecated = get_rust_deprecated(signature.orig.attrs);
            let ret_ty = if id == &OperationId::IndexingGetter {
                match ret_ty {
                    WbgType::JsOption(_) => ret_ty,
                    ref ty => {
                        if catch {
                            ret_ty
                        } else {
                            WbgType::JsOption(Box::new(ty.clone()))
                        }
                    }
                }
            } else {
                ret_ty
            };
            let variadic = signature.args.len() == signature.orig.args.len()
                && signature
                    .orig
                    .args
                    .last()
                    .map(|arg| arg.variadic)
                    .unwrap_or(false);

            let arguments =
                idl_arguments(signature.args.iter().zip(&signature.orig.args).filter_map(
                    |(idl_type, orig_arg)| {
                        idl_type
                            .as_ref()
                            .map(|idl_type| (orig_arg.name.to_string(), idl_type))
                    },
                ))?;

            let mut rust_name = rust_name.to_string();
            if let Some(map) = type_name.and_then(|type_name| FIXED_INTERFACES.get(type_name)) {
                if let Some(fixed) = map.get(rust_name.as_str()) {
                    rust_name = fixed.to_string();
                }
            }

            let variadic_type = if variadic {
                signature.args.last().and_then(|arg| arg.clone())
            } else {
                None
            };

            Some(InterfaceMethod {
                name: rust_ident(&rust_name),
                js_name: js_name.to_string(),
                deprecated,
                arguments,
                variadic_type,
                ret_wbg_ty: Some(ret_ty),
                kind: kind.clone(),
                is_static,
                structural,
                catch,
                variadic,
                unstable: unstable_flag,
                has_unstable_override,
            })
        }

        // Helper to build a method set from signature indices, with variadic expansion.
        let build_method_set = |sig_indices: &[usize],
                                unstable_flag: bool|
         -> Vec<InterfaceMethod<'_>> {
            let mut methods = Vec::new();
            for &sig_idx in sig_indices {
                let signature = &actual_signatures[sig_idx];
                let rust_name =
                    compute_rust_name(signature, sig_indices, &actual_signatures, js_name);

                if let Some(method) = create_method(
                    self,
                    signature,
                    &rust_name,
                    js_name,
                    type_name,
                    &kind,
                    id,
                    is_static,
                    force_structural,
                    force_throws,
                    container_attrs,
                    unstable_flag,
                    false,
                ) {
                    methods.push(method.clone());

                    if method.variadic && !self.options.next_unstable.get() {
                        let last_idl_type = signature.args.last().unwrap().as_ref().unwrap();
                        let last_name = signature.orig.args.last().unwrap().name;
                        for i in 0..=MAX_VARIADIC_ARGUMENTS_COUNT {
                            let arguments = idl_arguments(
                                signature.args[..signature.args.len() - 1]
                                    .iter()
                                    .zip(&signature.orig.args)
                                    .filter_map(|(idl_type, orig_arg)| {
                                        idl_type
                                            .as_ref()
                                            .map(|idl_type| (orig_arg.name.to_string(), idl_type))
                                    })
                                    .chain(
                                        (1..=i)
                                            .map(|j| (format!("{last_name}_{j}"), last_idl_type)),
                                    ),
                            );
                            if let Some(arguments) = arguments {
                                let mut name = format!("{}_{i}", &rust_name);
                                if let Some(map) = type_name.and_then(|t| FIXED_INTERFACES.get(t)) {
                                    if let Some(fixed) = map.get(name.as_str()) {
                                        name = fixed.to_string();
                                    }
                                }
                                methods.push(InterfaceMethod {
                                    name: rust_ident(&name),
                                    arguments,
                                    variadic: false,
                                    variadic_type: Some(last_idl_type.clone()),
                                    ..method.clone()
                                });
                            }
                        }
                    }
                }
            }
            methods
        };

        // Check if any unstable signature comes from an actual unstable IDL definition
        // (as opposed to a stable definition that merely uses an unstable type).
        // The authoritative expansion model only applies when there's a real IDL override.
        let has_unstable_idl_override = unstable_signatures.iter().any(|&idx| {
            let sig = &actual_signatures[idx];
            unstable || sig.orig.stability.is_unstable()
        });

        let stable_methods = build_method_set(&stable_signatures, false);

        // Unstable IDL signatures use typed generics for return type conversion.
        if has_unstable_idl_override {
            self.options.next_unstable.set(true);
        }
        let unstable_methods = build_method_set(&unstable_signatures, true);
        self.options.next_unstable.set(saved_next_unstable);

        // If only one set has methods, no gating needed
        if unstable_methods.is_empty() {
            return stable_methods;
        }
        if stable_methods.is_empty() {
            return unstable_methods;
        }

        if !has_unstable_idl_override {
            // No actual IDL override — just stable methods that happen to use unstable types.
            // Emit stable methods with no gate, unstable-type methods with unstable gate.
            let mut ret = stable_methods;
            ret.extend(unstable_methods);
            return ret;
        }

        // Both sets have methods from an actual IDL override - determine gating by comparing
        let mut ret: Vec<InterfaceMethod<'_>> = Vec::new();

        for mut method in stable_methods {
            let merged = unstable_methods.iter().any(|um| method.same_signature(um));
            // Merged = in both sets, no gate. Otherwise gate with not(unstable).
            method.has_unstable_override = !merged;
            ret.push(method);
        }

        for method in unstable_methods {
            let merged = ret.iter().any(|sm| sm.same_signature(&method));
            if !merged {
                // Only in unstable set - emit with unstable gate
                ret.push(method);
            }
        }

        ret
    }

    /// When generating our web_sys APIs we default to setting slice references that
    /// get passed to JS as mutable in case they get mutated in JS.
    ///
    /// In certain cases we know for sure that the slice will not get mutated - for
    /// example when working with the WebGlRenderingContext APIs.
    ///
    /// Here we implement a whitelist for those cases. This whitelist is currently
    /// maintained by hand.
    ///
    /// When adding to this whitelist add tests to crates/web-sys/tests/wasm/whitelisted_immutable_slices.rs
    fn maybe_adjust<'a>(
        &self,
        attributes: &'src Option<ExtendedAttributeList<'src>>,
        mut idl_type: WbgType<'a>,
        id: &'a OperationId,
    ) -> WbgType<'a> {
        if has_named_attribute(attributes.as_ref(), "AllowShared") {
            flag_slices_allow_shared(&mut idl_type)
        }

        let op = match id {
            OperationId::Operation(Some(op)) => op,
            OperationId::Constructor(Some(op)) => op,
            _ => return idl_type,
        };

        if IMMUTABLE_SLICE_WHITELIST.contains(op) {
            flag_slices_immutable(&mut idl_type)
        }

        idl_type
    }
}

pub fn is_type_unstable(ty: &weedle::types::Type, unstable_types: &HashSet<Identifier>) -> bool {
    match ty {
        weedle::types::Type::Single(SingleType::NonAny(NonAnyType::Identifier(i))) => {
            // Check if the type in the unstable type list
            unstable_types.contains(&i.type_)
        }
        _ => false,
    }
}

fn is_idl_type_unstable(ty: &WbgType, unstable_types: &HashSet<Identifier>) -> bool {
    match ty {
        WbgType::Identifier {
            ty: IdentifierType::Dictionary(name) | IdentifierType::Interface(name),
            ..
        } => unstable_types.contains(&Identifier(name)),
        _ => false,
    }
}

/// Search for an attribute by name in some webidl object's attributes.
fn has_named_attribute(list: Option<&ExtendedAttributeList>, attribute: &str) -> bool {
    let list = match list {
        Some(list) => list,
        None => return false,
    };
    list.body.list.iter().any(|attr| match attr {
        ExtendedAttribute::NoArgs(name) => (name.0).0 == attribute,
        _ => false,
    })
}

fn has_ident_attribute(list: Option<&ExtendedAttributeList>, ident: &str) -> bool {
    let list = match list {
        Some(list) => list,
        None => return false,
    };
    list.body.list.iter().any(|attr| match attr {
        ExtendedAttribute::Ident(id) => id.lhs_identifier.0 == ident,
        ExtendedAttribute::IdentList(id) => id.identifier.0 == ident,
        _ => false,
    })
}

/// ChromeOnly is for things that are only exposed to privileged code in Firefox.
pub fn is_chrome_only(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs.as_ref(), "ChromeOnly")
}

/// Whether a webidl object is marked as a no interface object.
pub fn is_no_interface_object(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs.as_ref(), "NoInterfaceObject")
}

pub fn get_rust_deprecated(ext_attrs: &Option<ExtendedAttributeList>) -> Option<Option<String>> {
    ext_attrs
        .as_ref()?
        .body
        .list
        .iter()
        .filter_map(|attr| match attr {
            ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(id)) => Some((id, None)),
            ExtendedAttribute::Ident(ExtendedAttributeIdent {
                lhs_identifier: id,
                rhs,
                ..
            }) => Some((id, Some(rhs))),
            _ => None,
        })
        .filter(|(id, _)| id.0 == "RustDeprecated")
        .find_map(|(_, rhs)| match rhs {
            None => Some(None),
            Some(IdentifierOrString::String(s)) => Some(Some(s.0.to_owned())),
            _ => unimplemented!(),
        })
}

/// Whether a webidl object is marked as structural.
pub fn is_structural(
    item_attrs: Option<&ExtendedAttributeList>,
    container_attrs: Option<&ExtendedAttributeList>,
) -> bool {
    // Note that once host bindings is implemented we'll want to switch this
    // from `true` to `false`, and then we'll want to largely read information
    // from the WebIDL about whether to use structural bindings or not.
    true || has_named_attribute(item_attrs, "Unforgeable")
        || has_named_attribute(container_attrs, "Unforgeable")
        || has_ident_attribute(container_attrs, "Global")
}

/// Whether a webidl object is marked as throwing.
pub fn throws(attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(attrs.as_ref(), "Throws")
}

fn arg_throws(ty: &WbgType<'_>) -> bool {
    match ty {
        WbgType::DataView { allow_shared }
        | WbgType::Int8Array { allow_shared, .. }
        | WbgType::Uint8Array { allow_shared, .. }
        | WbgType::Uint8ClampedArray { allow_shared, .. }
        | WbgType::Int16Array { allow_shared, .. }
        | WbgType::Uint16Array { allow_shared, .. }
        | WbgType::Int32Array { allow_shared, .. }
        | WbgType::Uint32Array { allow_shared, .. }
        | WbgType::Float32Array { allow_shared, .. }
        | WbgType::Float64Array { allow_shared, .. }
        | WbgType::ArrayBufferView { allow_shared, .. }
        | WbgType::BufferSource { allow_shared, .. }
        | WbgType::Identifier {
            ty:
                IdentifierType::Int8Slice { allow_shared, .. }
                | IdentifierType::Uint8Slice { allow_shared, .. }
                | IdentifierType::Uint8ClampedSlice { allow_shared, .. }
                | IdentifierType::Int16Slice { allow_shared, .. }
                | IdentifierType::Uint16Slice { allow_shared, .. }
                | IdentifierType::Int32Slice { allow_shared, .. }
                | IdentifierType::Uint32Slice { allow_shared, .. }
                | IdentifierType::Float32Slice { allow_shared, .. }
                | IdentifierType::Float64Slice { allow_shared, .. },
            ..
        } => !allow_shared,
        WbgType::JsOption(item) => arg_throws(item),
        WbgType::Union(list) => list.iter().any(arg_throws),
        // catch-all for everything else like Object
        _ => false,
    }
}

/// Whether a getter is marked as throwing.
pub fn getter_throws(
    parent_js_name: &str,
    js_name: &str,
    attrs: &Option<ExtendedAttributeList>,
) -> bool {
    if let Some(parent) = BREAKING_GETTER_THROWS.get(parent_js_name) {
        if parent.contains(&js_name) {
            return false;
        }
    }

    has_named_attribute(attrs.as_ref(), "GetterThrows")
}

/// Whether a setter is marked as throwing.
pub fn setter_throws(
    parent_js_name: &str,
    js_name: &str,
    attrs: &Option<ExtendedAttributeList>,
) -> bool {
    if let Some(parent) = BREAKING_SETTER_THROWS.get(parent_js_name) {
        if parent.contains(&js_name) {
            return false;
        }
    }

    has_named_attribute(attrs.as_ref(), "SetterThrows")
}

fn flag_slices_immutable(ty: &mut WbgType) {
    match ty {
        WbgType::Int8Array { immutable, .. }
        | WbgType::Uint8Array { immutable, .. }
        | WbgType::Uint8ClampedArray { immutable, .. }
        | WbgType::Int16Array { immutable, .. }
        | WbgType::Uint16Array { immutable, .. }
        | WbgType::Int32Array { immutable, .. }
        | WbgType::Uint32Array { immutable, .. }
        | WbgType::Float32Array { immutable, .. }
        | WbgType::Float64Array { immutable, .. }
        | WbgType::ArrayBufferView { immutable, .. }
        | WbgType::BufferSource { immutable, .. }
        | WbgType::Identifier {
            ty: IdentifierType::AllowSharedBufferSource { immutable },
            ..
        } => *immutable = true,
        WbgType::JsOption(item) => flag_slices_immutable(item),
        WbgType::Union(list) => {
            for item in list {
                flag_slices_immutable(item);
            }
        }
        // catch-all for everything else like Object
        _ => {}
    }
}

fn flag_slices_allow_shared(ty: &mut WbgType) {
    match ty {
        WbgType::DataView { allow_shared }
        | WbgType::Int8Array { allow_shared, .. }
        | WbgType::Uint8Array { allow_shared, .. }
        | WbgType::Uint8ClampedArray { allow_shared, .. }
        | WbgType::Int16Array { allow_shared, .. }
        | WbgType::Uint16Array { allow_shared, .. }
        | WbgType::Int32Array { allow_shared, .. }
        | WbgType::Uint32Array { allow_shared, .. }
        | WbgType::Float32Array { allow_shared, .. }
        | WbgType::Float64Array { allow_shared, .. }
        | WbgType::ArrayBufferView { allow_shared, .. }
        | WbgType::BufferSource { allow_shared, .. } => *allow_shared = true,
        WbgType::JsOption(item) => flag_slices_allow_shared(item),
        WbgType::FrozenArray(item) => flag_slices_allow_shared(item),
        WbgType::Sequence(item) => flag_slices_allow_shared(item),
        WbgType::ObservableArray(item) => flag_slices_allow_shared(item),
        WbgType::Promise(item) => flag_slices_allow_shared(item),
        WbgType::Record(item1, item2) => {
            flag_slices_allow_shared(item1);
            flag_slices_allow_shared(item2);
        }
        WbgType::Union(list) => {
            for item in list {
                flag_slices_allow_shared(item);
            }
        }
        // catch-all for everything else like Object
        _ => {}
    }
}

pub fn required_doc_string(options: &Options, features: &BTreeSet<String>) -> Option<String> {
    if !options.features || features.is_empty() {
        return None;
    }
    let list = features
        .iter()
        .map(|ident| format!("`{ident}`"))
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "\n\n*This API requires the following crate features \
         to be activated: {list}*",
    ))
}

pub fn get_cfg_features(options: &Options, features: &BTreeSet<String>) -> Option<syn::Attribute> {
    let len = features.len();

    if !options.features || len == 0 {
        None
    } else {
        let features = features
            .iter()
            .map(|feature| quote!( feature = #feature, ))
            .collect::<TokenStream>();

        // This is technically unneeded but it generates more idiomatic code
        if len == 1 {
            Some(syn::parse_quote!( #[cfg(#features)] ))
        } else {
            Some(syn::parse_quote!( #[cfg(all(#features))] ))
        }
    }
}

pub fn nullable(mut ty: weedle::types::Type) -> weedle::types::Type {
    use weedle::types::Type;

    fn make_nullable<T>(mb: &mut MayBeNull<T>) {
        mb.q_mark = Some(weedle::term::QMark);
    }

    match &mut ty {
        Type::Single(SingleType::Any(_) | SingleType::NonAny(NonAnyType::Promise(_))) => (),
        Type::Single(SingleType::NonAny(NonAnyType::Integer(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::FloatingPoint(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Boolean(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Byte(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Octet(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::ByteString(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::DOMString(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::USVString(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Sequence(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Object(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Symbol(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Error(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::ArrayBuffer(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::DataView(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Int8Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Int16Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Int32Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Uint8Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Uint16Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Uint32Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Uint8ClampedArray(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Float32Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Float64Array(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::ArrayBufferView(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::BufferSource(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::FrozenArrayType(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::ObservableArrayType(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::RecordType(mb))) => make_nullable(mb),
        Type::Single(SingleType::NonAny(NonAnyType::Identifier(mb))) => make_nullable(mb),
        Type::Union(mb) => make_nullable(mb),
    }

    ty
}

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
        Ident::new(&format!("{name}_"), proc_macro2::Span::call_site())

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
        Ident::new(&format!("N{name}"), proc_macro2::Span::call_site())
    } else {
        Ident::new(name, proc_macro2::Span::call_site())
    }
}

/// Create an `Ident` without checking to see if it conflicts with a Rust
/// keyword.
pub fn raw_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

/// Create a global path type from the given segments. For example an iterator
/// yielding the idents `[foo, bar, baz]` will result in the path type
/// `::foo::bar::baz`.
pub fn leading_colon_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    let segments = segments.into_iter();
    parse_quote!(::#(#segments)::*)
}

/// Create a path type with a single segment from a given Identifier
pub fn ident_ty(ident: Ident) -> syn::Type {
    parse_quote!(#ident)
}
