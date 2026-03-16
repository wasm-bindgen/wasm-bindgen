#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use js_sys::*;
/// Extension trait for awaiting `js_sys::Promise<T>`.
///
/// Since `IntoFuture` can't be implemented for `js_sys::Promise` from
/// generated code (orphan rule), use `.into_future().await` instead:
/// ```ignore
/// use bindings::PromiseExt;
/// let data: ArrayBuffer = promise.into_future().await?;
/// ```
pub trait PromiseExt {
    type Output;
    fn into_future(self) -> wasm_bindgen_futures::JsFuture<Self::Output>;
}
impl<T: 'static + wasm_bindgen::convert::FromWasmAbi> PromiseExt for js_sys::Promise<T> {
    type Output = T;
    fn into_future(self) -> wasm_bindgen_futures::JsFuture<T> {
        wasm_bindgen_futures::JsFuture::from(self)
    }
}
#[allow(dead_code)]
use JsValue as Error;
pub mod es_module_lexer {
    use wasm_bindgen::prelude::*;
    use js_sys::*;
    use super::*;
    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ImportType {
        /// A normal static using any syntax variations
        ///   import .. from 'module'
        Static = 1u32,
        /// A dynamic import expression `import(specifier)`
        /// or `import(specifier, opts)`
        Dynamic = 2u32,
        /// An import.meta expression
        ImportMeta = 3u32,
        /// A source phase import
        ///   import source x from 'module'
        StaticSourcePhase = 4u32,
        /// A dynamic source phase import
        ///   import.source('module')
        DynamicSourcePhase = 5u32,
        /// A defer phase import
        ///   import defer * as x from 'module'
        StaticDeferPhase = 6u32,
        /// A dynamic defer phase import
        ///   import.defer('module')
        DynamicDeferPhase = 7u32,
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type ImportSpecifier;
        /// Module name
        ///
        /// To handle escape sequences in specifier strings, the .n field of imported specifiers will be provided where possible.
        ///
        /// For dynamic import expressions, this field will be empty if not a valid JS string.
        /// For static import expressions, this field will always be populated.
        ///
        /// ## Example
        ///
        /// ```js
        /// const [imports1, exports1] = parse(String.raw`import './\u0061\u0062.js'`);
        /// imports1[0].n;
        /// // Returns "./ab.js"
        ///
        /// const [imports2, exports2] = parse(`import("./ab.js")`);
        /// imports2[0].n;
        /// // Returns "./ab.js"
        ///
        /// const [imports3, exports3] = parse(`import("./" + "ab.js")`);
        /// imports3[0].n;
        /// // Returns undefined
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn n(this: &ImportSpecifier) -> Option<String>;
        /// Type of import statement
        #[wasm_bindgen(method, getter)]
        pub fn t(this: &ImportSpecifier) -> ImportType;
        /// Start of module specifier
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `import { a } from 'asdf'`;
        /// const [imports, exports] = parse(source);
        /// source.substring(imports[0].s, imports[0].e);
        /// // Returns "asdf"
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn s(this: &ImportSpecifier) -> f64;
        /// End of module specifier
        #[wasm_bindgen(method, getter)]
        pub fn e(this: &ImportSpecifier) -> f64;
        /// Start of import statement
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `import { a } from 'asdf'`;
        /// const [imports, exports] = parse(source);
        /// source.substring(imports[0].ss, imports[0].se);
        /// // Returns "import { a } from 'asdf';"
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn ss(this: &ImportSpecifier) -> f64;
        /// End of import statement
        #[wasm_bindgen(method, getter)]
        pub fn se(this: &ImportSpecifier) -> f64;
        /// If this import keyword is a dynamic import, this is the start value.
        /// If this import keyword is a static import, this is -1.
        /// If this import keyword is an import.meta expresion, this is -2.
        #[wasm_bindgen(method, getter)]
        pub fn d(this: &ImportSpecifier) -> f64;
        /// If this import has an import attribute, this is the start value.
        /// Otherwise this is `-1`.
        #[wasm_bindgen(method, getter)]
        pub fn a(this: &ImportSpecifier) -> f64;
        /// Parsed import attributes as an array of [key, value] tuples.
        /// If this import has no attributes, this is `null`.
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `import foo from 'bar' with { type: "json" }`;
        /// const [imports] = parse(source);
        /// imports[0].at;
        /// // Returns [['type', 'json']]
        /// ```
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `import foo from 'bar' with { type: "json", integrity: "sha384-..." }`;
        /// const [imports] = parse(source);
        /// imports[0].at;
        /// // Returns [['type', 'json'], ['integrity', 'sha384-...']]
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn at(
            this: &ImportSpecifier,
        ) -> Option<Array<ArrayTuple<(JsString, JsString)>>>;
    }
    impl ImportSpecifier {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_unsafe)]
            unsafe { JsValue::from(js_sys::Object::new()).unchecked_into() }
        }
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type ExportSpecifier;
        /// Exported name
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `export default []`;
        /// const [imports, exports] = parse(source);
        /// exports[0].n;
        /// // Returns "default"
        /// ```
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `export const asdf = 42`;
        /// const [imports, exports] = parse(source);
        /// exports[0].n;
        /// // Returns "asdf"
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn n(this: &ExportSpecifier) -> String;
        /// Local name, or undefined.
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `export default []`;
        /// const [imports, exports] = parse(source);
        /// exports[0].ln;
        /// // Returns undefined
        /// ```
        ///
        /// ## Example
        ///
        /// ```js
        /// const asdf = 42;
        /// const source = `export { asdf as a }`;
        /// const [imports, exports] = parse(source);
        /// exports[0].ln;
        /// // Returns "asdf"
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn ln(this: &ExportSpecifier) -> Option<String>;
        /// Start of exported name
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `export default []`;
        /// const [imports, exports] = parse(source);
        /// source.substring(exports[0].s, exports[0].e);
        /// // Returns "default"
        /// ```
        ///
        /// ## Example
        ///
        /// ```js
        /// const source = `export { 42 as asdf }`;
        /// const [imports, exports] = parse(source);
        /// source.substring(exports[0].s, exports[0].e);
        /// // Returns "asdf"
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn s(this: &ExportSpecifier) -> f64;
        /// End of exported name
        #[wasm_bindgen(method, getter)]
        pub fn e(this: &ExportSpecifier) -> f64;
        /// Start of local name, or -1.
        ///
        /// ## Example
        ///
        /// ```js
        /// const asdf = 42;
        /// const source = `export { asdf as a }`;
        /// const [imports, exports] = parse(source);
        /// source.substring(exports[0].ls, exports[0].le);
        /// // Returns "asdf"
        /// ```
        #[wasm_bindgen(method, getter)]
        pub fn ls(this: &ExportSpecifier) -> f64;
        /// End of local name, or -1.
        #[wasm_bindgen(method, getter)]
        pub fn le(this: &ExportSpecifier) -> f64;
    }
    impl ExportSpecifier {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_unsafe)]
            unsafe { JsValue::from(js_sys::Object::new()).unchecked_into() }
        }
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type ParseError;
        #[wasm_bindgen(method, getter)]
        pub fn idx(this: &ParseError) -> f64;
        #[wasm_bindgen(method, setter)]
        pub fn set_idx(this: &ParseError, val: f64);
    }
    impl ParseError {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_imports)]
            use wasm_bindgen::JsCast;
            JsCast::unchecked_into(js_sys::Object::new())
        }
        pub fn builder() -> ParseErrorBuilder {
            ParseErrorBuilder {
                inner: Self::new(),
                required: 1u64,
            }
        }
    }
    pub struct ParseErrorBuilder {
        inner: ParseError,
        required: u64,
    }
    #[allow(unused_mut)]
    impl ParseErrorBuilder {
        pub fn idx(mut self, val: f64) -> Self {
            self.inner.set_idx(val);
            self.required &= 18446744073709551614u64;
            self
        }
        pub fn build(self) -> Result<ParseError, JsValue> {
            if self.required != 0 {
                let mut missing = Vec::new();
                if self.required & 1u64 != 0 {
                    missing.push("missing required property `idx`");
                }
                return Err(
                    JsValue::from_str(
                        &format!("{}: {}", stringify!(ParseError), missing.join(", ")),
                    ),
                );
            }
            Ok(self.inner)
        }
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        /// Outputs the list of exports and locations of import specifiers,
        /// including dynamic import and import meta handling.
        ///
        /// ## Arguments
        ///
        /// * `source` - Source code to parser
        /// * `name` - Optional sourcename
        ///
        /// ## Returns
        ///
        /// Tuple contaning imports list and exports list.
        pub fn parse(
            source: &str,
        ) -> ArrayTuple<
            (Array<ImportSpecifier>, Array<ExportSpecifier>, Boolean, Boolean),
        >;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "parse")]
        pub fn try_parse(
            source: &str,
        ) -> Result<
            ArrayTuple<
                (Array<ImportSpecifier>, Array<ExportSpecifier>, Boolean, Boolean),
            >,
            JsValue,
        >;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(js_name = "parse")]
        pub fn parse_with_name(
            source: &str,
            name: &str,
        ) -> ArrayTuple<
            (Array<ImportSpecifier>, Array<ExportSpecifier>, Boolean, Boolean),
        >;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "parse")]
        pub fn try_parse_with_name(
            source: &str,
            name: &str,
        ) -> Result<
            ArrayTuple<
                (Array<ImportSpecifier>, Array<ExportSpecifier>, Boolean, Boolean),
            >,
            JsValue,
        >;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        /// Wait for init to resolve before calling `parse`.
        #[wasm_bindgen(thread_local_v2)]
        pub static init: Promise<Undefined>;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(js_name = "initSync")]
        pub fn init_sync();
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "initSync")]
        pub fn try_init_sync() -> Result<(), JsValue>;
    }
}
