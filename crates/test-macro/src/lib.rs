//! See the README for `wasm-bindgen-test` for a bit more info about what's
//! going on here.

extern crate proc_macro;

use proc_macro2::*;
use quote::quote;
use quote::quote_spanned;

#[proc_macro_attribute]
pub fn wasm_bindgen_bench(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    bindgen(attr, body, true)
}

#[proc_macro_attribute]
pub fn wasm_bindgen_test(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    bindgen(attr, body, false)
}

fn bindgen(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
    is_bench: bool,
) -> proc_macro::TokenStream {
    let mut attributes = Attributes::default();
    let attribute_parser = syn::meta::parser(|meta| attributes.parse(meta));

    syn::parse_macro_input!(attr with attribute_parser);
    let mut should_panic = None;
    let mut ignore = None;

    let mut body = TokenStream::from(body).into_iter().peekable();

    // Skip over other attributes to `fn #ident ...`, and extract `#ident`
    let mut leading_tokens = Vec::new();
    while let Some(token) = body.next() {
        match parse_should_panic(&mut body, &token) {
            Ok(Some((new_should_panic, span))) => {
                if should_panic.replace(new_should_panic).is_some() {
                    return compile_error(span, "duplicate `should_panic` attribute");
                }

                // If we found a `should_panic`, we should skip the `#` and `[...]`.
                // The `[...]` is skipped here, the current `#` is skipped by using `continue`.
                body.next();
                continue;
            }
            Ok(None) => (),
            Err(error) => return error,
        }

        match parse_ignore(&mut body, &token) {
            Ok(Some((new_ignore, span))) => {
                if ignore.replace(new_ignore).is_some() {
                    return compile_error(span, "duplicate `ignore` attribute");
                }

                // If we found a `new_ignore`, we should skip the `#` and `[...]`.
                // The `[...]` is skipped here, the current `#` is skipped by using `continue`.
                body.next();
                continue;
            }
            Ok(None) => (),
            Err(error) => return error,
        }

        leading_tokens.push(token.clone());
        if let TokenTree::Ident(token) = token {
            if token == "async" {
                attributes.r#async = true;
            }
            if token == "fn" {
                break;
            }
        }
    }
    let ident = find_ident(&mut body).expect("expected a function name");

    let mut tokens = Vec::<TokenTree>::new();

    let should_panic_par = match &should_panic {
        Some(Some(lit)) => {
            quote! { ::core::option::Option::Some(::core::option::Option::Some(#lit)) }
        }
        Some(None) => quote! { ::core::option::Option::Some(::core::option::Option::None) },
        None => quote! { ::core::option::Option::None },
    };

    let ignore_par = match &ignore {
        Some(Some(lit)) => {
            quote! { ::core::option::Option::Some(::core::option::Option::Some(#lit)) }
        }
        Some(None) => quote! { ::core::option::Option::Some(::core::option::Option::None) },
        None => quote! { ::core::option::Option::None },
    };

    let exec_ident = if is_bench {
        let body = if attributes.r#async {
            quote! { #ident(&mut bencher).await; }
        } else {
            quote! { #ident(&mut bencher); }
        };
        let bench_ident = quote::format_ident!("__wbg_bench_{ident}");
        tokens.extend(quote! {
            async fn #bench_ident() {
                let mut bencher = Criterion::default()
                    .with_location(file!(), module_path!());
                #body
            }
        });
        bench_ident
    } else {
        ident.clone()
    };

    let test_body = if attributes.r#async || is_bench {
        quote! { cx.execute_async(test_name, #exec_ident, #should_panic_par, #ignore_par); }
    } else {
        quote! { cx.execute_sync(test_name, #exec_ident, #should_panic_par, #ignore_par); }
    };

    let ignore_name = if ignore.is_some() { "$" } else { "" };

    let wasm_bindgen_path = attributes.wasm_bindgen_path;
    let prefix = if is_bench { "__wbgb_" } else { "__wbgt_" };
    tokens.extend(
        quote! {
            const _: () = {
                #wasm_bindgen_path::__rt::wasm_bindgen::__wbindgen_coverage! {
                #[export_name = ::core::concat!(#prefix, #ignore_name, "_", ::core::module_path!(), "::", ::core::stringify!(#ident))]
                #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
                extern "C" fn __wbgt_test(cx: &#wasm_bindgen_path::__rt::Context) {
                    let test_name = ::core::concat!(::core::module_path!(), "::", ::core::stringify!(#ident));
                    #test_body
                }
                }
            };
        },
    );

    if let Some(path) = attributes.unsupported {
        tokens.extend(
            quote! { #[cfg_attr(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))), #path)] },
        );

        if let Some(should_panic) = should_panic {
            let should_panic = if let Some(lit) = should_panic {
                quote! { should_panic = #lit }
            } else {
                quote! { should_panic }
            };

            tokens.extend(
                quote! { #[cfg_attr(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))), #should_panic)] }
            )
        }

        if let Some(ignore) = ignore {
            let ignore = if let Some(lit) = ignore {
                quote! { ignore = #lit }
            } else {
                quote! { ignore }
            };

            tokens.extend(
                quote! { #[cfg_attr(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))), #ignore)] }
            )
        }
    } else {
        tokens.extend(quote! {
            #[cfg_attr(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))), allow(dead_code))]
        });
    }

    tokens.extend(leading_tokens);
    tokens.push(ident.into());
    tokens.extend(body);

    tokens.into_iter().collect::<TokenStream>().into()
}

fn parse_should_panic(
    body: &mut std::iter::Peekable<token_stream::IntoIter>,
    token: &TokenTree,
) -> Result<Option<(Option<Literal>, Span)>, proc_macro::TokenStream> {
    // Start by parsing the `#`
    match token {
        TokenTree::Punct(op) if op.as_char() == '#' => (),
        _ => return Ok(None),
    }

    // Parse `[...]`
    let group = match body.peek() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => group,
        _ => return Ok(None),
    };

    let mut stream = group.stream().into_iter();

    // Parse `should_panic`
    let mut span = match stream.next() {
        Some(TokenTree::Ident(token)) if token == "should_panic" => token.span(),
        _ => return Ok(None),
    };

    let should_panic = span;

    // We are interested in the `expected` attribute or string if there is any
    match stream.next() {
        // Parse the `(...)` in `#[should_panic(...)]`
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            let span = group.span();
            stream = group.stream().into_iter();

            // Parse `expected`
            match stream.next() {
                Some(TokenTree::Ident(token)) if token == "expected" => (),
                _ => {
                    return Err(compile_error(
                        span,
                        "malformed `#[should_panic(...)]` attribute",
                    ))
                }
            }

            // Parse `=`
            match stream.next() {
                Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
                _ => {
                    return Err(compile_error(
                        span,
                        "malformed `#[should_panic(...)]` attribute",
                    ))
                }
            }
        }
        // Parse `=`
        Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
        Some(token) => {
            return Err(compile_error(
                token.span(),
                "malformed `#[should_panic = \"...\"]` attribute",
            ))
        }
        None => {
            return Ok(Some((None, should_panic)));
        }
    }

    // Parse string in `#[should_panic(expected = "string")]` or `#[should_panic = "string"]`
    if let Some(TokenTree::Literal(lit)) = stream.next() {
        span = lit.span();
        let string = lit.to_string();

        // Verify it's a string.
        if string.starts_with('"') && string.ends_with('"') {
            return Ok(Some((Some(lit), should_panic)));
        }
    }

    Err(compile_error(span, "malformed `#[should_panic]` attribute"))
}

fn parse_ignore(
    body: &mut std::iter::Peekable<token_stream::IntoIter>,
    token: &TokenTree,
) -> Result<Option<(Option<Literal>, Span)>, proc_macro::TokenStream> {
    // Start by parsing the `#`
    match token {
        TokenTree::Punct(op) if op.as_char() == '#' => (),
        _ => return Ok(None),
    }

    // Parse `[...]`
    let group = match body.peek() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => group,
        _ => return Ok(None),
    };

    let mut stream = group.stream().into_iter();

    // Parse `ignore`
    let mut span = match stream.next() {
        Some(TokenTree::Ident(token)) if token == "ignore" => token.span(),
        _ => return Ok(None),
    };

    let ignore = span;

    // We are interested in the reason string if there is any
    match stream.next() {
        // Parse `=`
        Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
        Some(token) => {
            return Err(compile_error(
                token.span(),
                "malformed `#[ignore = \"...\"]` attribute",
            ))
        }
        None => {
            return Ok(Some((None, ignore)));
        }
    }

    // Parse string in `#[ignore = "string"]`
    if let Some(TokenTree::Literal(lit)) = stream.next() {
        span = lit.span();
        let string = lit.to_string();

        // Verify it's a string.
        if string.starts_with('"') && string.ends_with('"') {
            return Ok(Some((Some(lit), ignore)));
        }
    }

    Err(compile_error(span, "malformed `#[ignore]` attribute"))
}

fn find_ident(iter: &mut impl Iterator<Item = TokenTree>) -> Option<Ident> {
    match iter.next()? {
        TokenTree::Ident(i) => Some(i),
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
            find_ident(&mut g.stream().into_iter())
        }
        _ => None,
    }
}

fn compile_error(span: Span, msg: &str) -> proc_macro::TokenStream {
    quote_spanned! { span => compile_error!(#msg); }.into()
}

struct Attributes {
    r#async: bool,
    wasm_bindgen_path: syn::Path,
    unsupported: Option<syn::Meta>,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            r#async: false,
            wasm_bindgen_path: syn::parse_quote!(::wasm_bindgen_test),
            unsupported: None,
        }
    }
}

impl Attributes {
    fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("async") {
            self.r#async = true;
        } else if meta.path.is_ident("crate") {
            self.wasm_bindgen_path = meta.value()?.parse::<syn::Path>()?;
        } else if meta.path.is_ident("unsupported") {
            self.unsupported = Some(meta.value()?.parse::<syn::Meta>()?);
        } else {
            return Err(meta.error("unknown attribute"));
        }
        Ok(())
    }
}
