//! Procedural macros for creating JavaScript objects and arrays in WebAssembly.
//!
//! This crate provides two macros:
//! - [`json!`] - Creates `js_sys::Object` instances using JavaScript object literal syntax
//! - [`array!`] - Creates `js_sys::Array` instances using array literal syntax
//!
//! Both macros support any Rust value that implements `Into<wasm_bindgen::JsValue>`, including:
//! - **Strings**: `&'static str`, `String` and `&str`
//! - **Numbers**: Integers and floating-point
//! - **Booleans**
//! - **Nested objects and arrays**: Using `{}` and `[]` syntax
//! - **`null` and `undefined` JavaScript values**
//! - **Custom types**: Type must implement `Into<wasm_bindgen::JsValue>`
//!
//! # Example
//! ```rust,no_run
//! use js_sys::{Array, Object};
//! use wasm_bindgen_creation_macros::{array, json};
//!
//! // Create a JavaScript object with nested structure
//! let person = json! {
//!     name: "Alice",
//!     age: 30,
//!     hobbies: ["reading", "coding"],
//!     address: {
//!         street: "123 Main St",
//!         city: "Techville"
//!     }
//! };
//!
//! // Create a JavaScript array with mixed types
//! let data = array![
//!     "string",
//!     42,
//!     true,
//!     { key: "value" }
//! ];
//! ```
//!
//! See the individual macro documentation for more details and examples.

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    token::{Brace, Bracket},
    Ident, LitBool, LitFloat, LitInt, LitStr, Token,
};

#[derive(Clone)]
struct JsonObject(JsonValue);

#[derive(Clone)]
struct JsonArray(Vec<JsonValue>);

impl JsonObject {
    fn to_tokens(&self) -> TokenStream2 {
        self.0.to_tokens()
    }
}

impl Parse for JsonObject {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(JsonObject(JsonValue::Object(vec![])));
        }

        Ok(JsonObject(JsonValue::parse_object_inner(input)?))
    }
}

impl Parse for JsonArray {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(JsonArray(JsonValue::parse_array_inner(input)?))
    }
}

impl JsonArray {
    fn to_tokens(&self) -> TokenStream2 {
        JsonValue::array_to_tokens(&self.0)
    }
}

#[derive(Clone)]
enum JsonValue {
    Object(Vec<(Ident, JsonValue)>),
    External(Ident), // Local object, array, or rust variable passed to macro
    Array(Vec<JsonValue>),
    String(String),
    Number(String),
    Boolean(bool),
    Null,
    Undefined,
}

impl Parse for JsonValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = if input.peek(Brace) {
            let content;
            braced!(content in input);
            Self::parse_object_inner(&content)?
        } else if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            JsonValue::Array(Self::parse_array_inner(&content)?)
        } else if input.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            if input.peek2(Token![:]) {
                return Err(syn::Error::new(
                    lit.span(),
                    "String literal keys are not supported",
                ));
            }
            JsonValue::String(lit.value())
        } else if input.peek(LitInt) || input.peek(LitFloat) {
            JsonValue::Number(Self::parse_number(input)?)
        } else if input.peek(LitBool) {
            JsonValue::Boolean(input.parse::<LitBool>()?.value)
        } else if input.peek(Ident) {
            Self::parse_ident(input)?
        } else {
            return Err(syn::Error::new(input.span(), "Expected valid JSON"));
        };

        Ok(value)
    }
}

impl JsonValue {
    fn array_to_tokens(elements: &[JsonValue]) -> TokenStream2 {
        let element_tokens = elements.iter().map(JsonValue::to_tokens);
        quote! {
            {
                let array = js_sys::Array::new();
                #(
                    array.push(&#element_tokens.into());
                )*
                array
            }
        }
    }

    fn parse_object_inner(content: &ParseBuffer<'_>) -> syn::Result<JsonValue> {
        let mut pairs = Vec::new();

        while content.peek(Ident) {
            let key = content.parse::<Ident>()?;
            content.parse::<Token![:]>()?;
            let value = content.parse::<JsonValue>()?;
            pairs.push((key, value));

            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        if !content.is_empty() {
            return Err(syn::Error::new(
                content.span(),
                "Expected ident, found something else",
            ));
        }

        Ok(JsonValue::Object(pairs))
    }

    fn parse_array_inner(content: &ParseBuffer<'_>) -> syn::Result<Vec<JsonValue>> {
        let mut elements = Vec::new();
        while !content.is_empty() {
            elements.push(content.parse::<JsonValue>()?);
            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }
        Ok(elements)
    }

    fn parse_ident(content: &ParseBuffer<'_>) -> syn::Result<JsonValue> {
        const NULL: &str = "null";
        const UNDEFINED: &str = "undefined";

        let ident = content.parse::<Ident>()?;
        Ok(if content.peek(Token![:]) {
            content.parse::<JsonValue>()?
        } else if ident == NULL {
            JsonValue::Null
        } else if ident == UNDEFINED {
            JsonValue::Undefined
        } else {
            JsonValue::External(ident)
        })
    }

    fn parse_number(content: &ParseBuffer<'_>) -> syn::Result<String> {
        assert!(content.peek(LitInt) || content.peek(LitFloat));
        let value = content.step(|cursor| {
            let mut rest = *cursor;
            let mut content = String::new();

            if let Some((TokenTree::Literal(lit), next)) = rest.token_tree() {
                content.push_str(&lit.to_string());
                rest = next;
            }
            Ok((content, rest))
        })?;
        Ok(value)
    }

    fn to_tokens(&self) -> TokenStream2 {
        match self {
            JsonValue::Object(pairs) => {
                let keys = pairs.iter().map(|(key, _)| key);
                let values = pairs.iter().map(|(_, value)| value.to_tokens());
                quote! {
                    {
                        let obj = js_sys::Object::new();
                        #(
                            js_sys::Reflect::set(&obj, &stringify!(#keys).into(), &#values.into()).unwrap();
                        )*
                        obj
                    }
                }
            }
            JsonValue::Array(elements) => Self::array_to_tokens(elements),
            JsonValue::External(ident) => quote! { #ident },
            JsonValue::String(s) => quote! { #s },
            JsonValue::Number(n) => {
                let n = n.parse::<f64>().unwrap_or(0.0);
                quote! { #n }
            }
            JsonValue::Boolean(b) => quote! { #b },
            JsonValue::Null => quote! { wasm_bindgen::JsValue::NULL },
            JsonValue::Undefined => quote! { wasm_bindgen::JsValue::UNDEFINED },
        }
    }
}

/// The `json!` macro allows you to create `js_sys::Object`s using JavaScript object literal syntax.
/// It supports all basic JavaScript types and can handle nested structures.
///
/// # Supported Types
/// - **Strings**: `&'static str`, `String` and `&str`
/// - **Numbers**: Integers and floating-point
/// - **Booleans**
/// - **JavaScript array literals**: Using `[]` syntax
/// - **JavaScript object literals**: Using `{}` syntax
/// - **`null` and `undefined` JavaScript values/keywords**
/// - **Variables**: Any Rust variable whose type implements `Into<wasm_bindgen::JsValue>` can be passed to `json!`
/// - **Custom types**: Type must implement `Into<wasm_bindgen::JsValue>` (See [Syntax Limitations](#syntax-limitations))
///
/// # Basic Usage
/// ```rust,no_run
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::json;
///
/// let obj = json! {
///     name: "John",
///     age: 30,
///     is_student: true,
///     hobbies: ["reading", "coding"]
/// };
/// ```
///
/// # Nested Structures
/// The `json!` macro supports nested structures of arbitrary depth.
/// ```rust,no_run
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::json;
///
/// let obj = json! {
///     name: "John",
///     address: {
///         street: "123 Main St",
///         city: "Anytown"
///     },
///     friends: [
///         { name: "Jane", age: 25 },
///         { name: "Jim", age: 30 }
///     ]
/// };
/// ```
///
/// # Variable Usage
/// All Rust values that implement `Into<wasm_bindgen::JsValue>` can be used in the macro. For simple types,
/// literals can be added directly. For more complex types, they should be stored in a variable first (see [Syntax Limitations](#syntax-limitations)).
/// ```rust,no_run
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::json;
///
/// let name = "John";
/// let hobbies = vec!["reading".to_string(), "coding".to_string()];
/// let address_obj = json! {
///         street: "123 Main St",
///         city: "Anytown"
/// };
///
/// let obj = json! {
///     name: name,
///     hobbies: hobbies,
///     address: address_obj
/// };
/// ```
///
/// # Comments
/// The macro supports Rust-style comments:
/// ```rust,no_run
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::json;
///
/// let obj = json! {
///     name: "John", // The person's name
///     age: 30 // Their age
/// };
/// ```
///
/// # Syntax Limitations
/// The parser is unsophisticated; it only supports simple Rust literals. Expressions, struct instantiations, etc. should be
/// stored in variables first:
///
/// ```compile_fail
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::{array, json};
///
/// struct CustomJsValue(u32);
/// impl Into<JsValue> for CustomJsValue {
///     fn into(self) -> JsValue {
///         self.0.into()
///     }
/// }
///
/// let obj = json! {
///     custom: CustomJsValue(42),
///     array: array![1, 2, 3]
/// };
/// ```
///
/// Do this instead:
/// ```rust,no_run
/// use js_sys::Object;
/// use wasm_bindgen::JsValue;
/// use wasm_bindgen_creation_macros::{array, json};
///
/// struct CustomJsValue(u32);
/// impl Into<JsValue> for CustomJsValue {
///     fn into(self) -> JsValue {
///         self.0.into()
///     }
/// }
///
/// let custom = CustomJsValue(42);
/// let array = array![1, 2, 3];
/// let obj = json! {
///     custom: custom,
///     array: array
/// };
/// ```
///
/// String literal keys are not (currently) supported:
/// ```compile_fail
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::json;
///
/// let obj = json! {
///     "key": 42
/// };
/// ```
///
/// Do this instead:
/// ```rust,no_run
/// use js_sys::Object;
/// use wasm_bindgen_creation_macros::json;
///
/// let obj = json! {
///     key: 42
/// };
/// ```
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as JsonObject);
    value.to_tokens().into()
}

/// The `array!` macro provides a convenient way to create `js_sys::Array` instances.
///
/// # Supported Types
/// - **Strings**: `&'static str`, `String` and `&str`
/// - **Numbers**: Integers and floating-point
/// - **Booleans**
/// - **Nested arrays**: Using `[]` syntax
/// - **JavaScript object literals**: Using `{}` syntax
/// - **`null` and `undefined` JavaScript values**
/// - **Variables**: Any Rust variable whose type implements `Into<wasm_bindgen::JsValue>` can be passed to `array!`
/// - **Custom types**: Type must implement `Into<wasm_bindgen::JsValue>` (See [Syntax Limitations](#syntax-limitations))
///
/// # Basic Usage
/// ```rust,no_run
/// use js_sys::Array;
/// use wasm_bindgen_creation_macros::array;
///
/// let numbers = array![1, 2, 3, 4, 5];
/// let strings = array!["hello", "world"];
/// ```
///
/// # Nested Arrays
/// ```rust,no_run
/// use js_sys::Array;
/// use wasm_bindgen_creation_macros::array;
///
/// let matrix = array![
///     [1, 2, 3],
///     [4, 5, 6]
/// ];
/// ```
///
/// # Variable Usage
/// ```rust,no_run
/// use js_sys::Array;
/// use wasm_bindgen_creation_macros::array;
///
/// let name = "John".to_string();
/// let arr = array![name, "Jane", "Jim"];
/// ```
///
/// # Custom Types
/// Works with any type that implements `Into<wasm_bindgen::JsValue>`:
/// ```rust,no_run
/// use js_sys::Array;
/// use wasm_bindgen::JsValue;
/// use wasm_bindgen_creation_macros::array;
///
/// struct CustomJsValue(u32);
/// impl Into<JsValue> for CustomJsValue {
///     fn into(self) -> JsValue {
///         self.0.into()
///     }
/// }
///
/// let custom = CustomJsValue(42);
/// let arr = array![custom];
/// ```
///
/// # Comments
/// Supports Rust-style comments:
/// ```rust,no_run
/// use js_sys::Array;
/// use wasm_bindgen_creation_macros::array;
///
/// let arr = array![
///     1, // First element
///     2, // Second element
///     3  // Third element
/// ];
/// ```
///
/// # Syntax Limitations
/// The parser only supports simple Rust literals. Complex expressions or struct instantiations should be
/// stored in variables first:
///
/// ```compile_fail
/// use js_sys::Array;
/// use wasm_bindgen::JsValue;
/// use wasm_bindgen_creation_macros::array;
///
/// struct CustomJsValue(u32);
/// impl Into<JsValue> for CustomJsValue {
///     fn into(self) -> JsValue {
///         self.0.into()
///     }
/// }
///
/// let arr = array![CustomJsValue(42), array![1, 2, 3]];
/// ```
///
/// Do this instead:
/// ```rust,no_run
/// use js_sys::Array;
/// use wasm_bindgen::JsValue;
/// use wasm_bindgen_creation_macros::array;
///
/// struct CustomJsValue(u32);
/// impl Into<JsValue> for CustomJsValue {
///     fn into(self) -> JsValue {
///         self.0.into()
///     }
/// }
///
/// let custom = CustomJsValue(42);
/// let inner_array = array![1, 2, 3];
/// let arr = array![custom, inner_array];
/// ```
#[proc_macro]
pub fn array(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as JsonArray);
    value.to_tokens().into()
}
