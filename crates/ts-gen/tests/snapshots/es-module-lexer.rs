#[allow(unused_imports)]
use js_sys::*;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[doc = r" Extension trait for awaiting `js_sys::Promise<T>`."]
#[doc = r""]
#[doc = r" Since `IntoFuture` can't be implemented for `js_sys::Promise` from"]
#[doc = r" generated code (orphan rule), use `.into_future().await` instead:"]
#[doc = r" ```ignore"]
#[doc = r" use bindings::PromiseExt;"]
#[doc = r" let data: ArrayBuffer = promise.into_future().await?;"]
#[doc = r" ```"]
#[allow(dead_code)]
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
    use super::*;
    use js_sys::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ImportType {
        #[doc = " A normal static using any syntax variations"]
        #[doc = "   import .. from 'module'"]
        Static = 1u32,
        #[doc = " A dynamic import expression `import(specifier)`"]
        #[doc = " or `import(specifier, opts)`"]
        Dynamic = 2u32,
        #[doc = " An import.meta expression"]
        ImportMeta = 3u32,
        #[doc = " A source phase import"]
        #[doc = "   import source x from 'module'"]
        StaticSourcePhase = 4u32,
        #[doc = " A dynamic source phase import"]
        #[doc = "   import.source('module')"]
        DynamicSourcePhase = 5u32,
        #[doc = " A defer phase import"]
        #[doc = "   import defer * as x from 'module'"]
        StaticDeferPhase = 6u32,
        #[doc = " A dynamic defer phase import"]
        #[doc = "   import.defer('module')"]
        DynamicDeferPhase = 7u32,
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type ImportSpecifier;
        #[doc = " Module name"]
        #[doc = " "]
        #[doc = " To handle escape sequences in specifier strings, the .n field of imported specifiers will be provided where possible."]
        #[doc = " "]
        #[doc = " For dynamic import expressions, this field will be empty if not a valid JS string."]
        #[doc = " For static import expressions, this field will always be populated."]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const [imports1, exports1] = parse(String.raw`import './\\u0061\\u0062.js'`);"]
        #[doc = " imports1[0].n;"]
        #[doc = " // Returns \"./ab.js\""]
        #[doc = " "]
        #[doc = " const [imports2, exports2] = parse(`import(\"./ab.js\")`);"]
        #[doc = " imports2[0].n;"]
        #[doc = " // Returns \"./ab.js\""]
        #[doc = " "]
        #[doc = " const [imports3, exports3] = parse(`import(\"./\" + \"ab.js\")`);"]
        #[doc = " imports3[0].n;"]
        #[doc = " // Returns undefined"]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn n(this: &ImportSpecifier) -> Option<String>;
        #[doc = " Type of import statement"]
        #[wasm_bindgen(method, getter)]
        pub fn t(this: &ImportSpecifier) -> ImportType;
        #[doc = " Start of module specifier"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `import { a } from 'asdf'`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " source.substring(imports[0].s, imports[0].e);"]
        #[doc = " // Returns \"asdf\""]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn s(this: &ImportSpecifier) -> f64;
        #[doc = " End of module specifier"]
        #[wasm_bindgen(method, getter)]
        pub fn e(this: &ImportSpecifier) -> f64;
        #[doc = " Start of import statement"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `import { a } from 'asdf'`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " source.substring(imports[0].ss, imports[0].se);"]
        #[doc = " // Returns \"import { a } from 'asdf';\""]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn ss(this: &ImportSpecifier) -> f64;
        #[doc = " End of import statement"]
        #[wasm_bindgen(method, getter)]
        pub fn se(this: &ImportSpecifier) -> f64;
        #[doc = " If this import keyword is a dynamic import, this is the start value."]
        #[doc = " If this import keyword is a static import, this is -1."]
        #[doc = " If this import keyword is an import.meta expresion, this is -2."]
        #[wasm_bindgen(method, getter)]
        pub fn d(this: &ImportSpecifier) -> f64;
        #[doc = " If this import has an import attribute, this is the start value."]
        #[doc = " Otherwise this is `-1`."]
        #[wasm_bindgen(method, getter)]
        pub fn a(this: &ImportSpecifier) -> f64;
        #[doc = " Parsed import attributes as an array of [key, value] tuples."]
        #[doc = " If this import has no attributes, this is `null`."]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `import foo from 'bar' with { type: \"json\" }`;"]
        #[doc = " const [imports] = parse(source);"]
        #[doc = " imports[0].at;"]
        #[doc = " // Returns [['type', 'json']]"]
        #[doc = " ```"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `import foo from 'bar' with { type: \"json\", integrity: \"sha384-...\" }`;"]
        #[doc = " const [imports] = parse(source);"]
        #[doc = " imports[0].at;"]
        #[doc = " // Returns [['type', 'json'], ['integrity', 'sha384-...']]"]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn at(this: &ImportSpecifier) -> Option<Array<ArrayTuple<(JsString, JsString)>>>;
    }
    impl ImportSpecifier {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_unsafe)]
            unsafe {
                JsValue::from(js_sys::Object::new()).unchecked_into()
            }
        }
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type ExportSpecifier;
        #[doc = " Exported name"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `export default []`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " exports[0].n;"]
        #[doc = " // Returns \"default\""]
        #[doc = " ```"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `export const asdf = 42`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " exports[0].n;"]
        #[doc = " // Returns \"asdf\""]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn n(this: &ExportSpecifier) -> String;
        #[doc = " Local name, or undefined."]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `export default []`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " exports[0].ln;"]
        #[doc = " // Returns undefined"]
        #[doc = " ```"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const asdf = 42;"]
        #[doc = " const source = `export { asdf as a }`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " exports[0].ln;"]
        #[doc = " // Returns \"asdf\""]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn ln(this: &ExportSpecifier) -> Option<String>;
        #[doc = " Start of exported name"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `export default []`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " source.substring(exports[0].s, exports[0].e);"]
        #[doc = " // Returns \"default\""]
        #[doc = " ```"]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const source = `export { 42 as asdf }`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " source.substring(exports[0].s, exports[0].e);"]
        #[doc = " // Returns \"asdf\""]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn s(this: &ExportSpecifier) -> f64;
        #[doc = " End of exported name"]
        #[wasm_bindgen(method, getter)]
        pub fn e(this: &ExportSpecifier) -> f64;
        #[doc = " Start of local name, or -1."]
        #[doc = " "]
        #[doc = " ## Example"]
        #[doc = " "]
        #[doc = " ```js"]
        #[doc = " const asdf = 42;"]
        #[doc = " const source = `export { asdf as a }`;"]
        #[doc = " const [imports, exports] = parse(source);"]
        #[doc = " source.substring(exports[0].ls, exports[0].le);"]
        #[doc = " // Returns \"asdf\""]
        #[doc = " ```"]
        #[wasm_bindgen(method, getter)]
        pub fn ls(this: &ExportSpecifier) -> f64;
        #[doc = " End of local name, or -1."]
        #[wasm_bindgen(method, getter)]
        pub fn le(this: &ExportSpecifier) -> f64;
    }
    impl ExportSpecifier {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_unsafe)]
            unsafe {
                JsValue::from(js_sys::Object::new()).unchecked_into()
            }
        }
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        # [wasm_bindgen (extends = Object)]
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
                return Err(JsValue::from_str(&format!(
                    "{}: {}",
                    stringify!(ParseError),
                    missing.join(", ")
                )));
            }
            Ok(self.inner)
        }
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[doc = " Outputs the list of exports and locations of import specifiers,"]
        #[doc = " including dynamic import and import meta handling."]
        #[doc = " "]
        #[doc = " ## Arguments"]
        #[doc = " "]
        #[doc = " * `source` - Source code to parser"]
        #[doc = " * `name` - Optional sourcename"]
        #[doc = " "]
        #[doc = " ## Returns"]
        #[doc = " "]
        #[doc = " Tuple contaning imports list and exports list."]
        pub fn parse(
            source: &str,
        ) -> ArrayTuple<(
            Array<ImportSpecifier>,
            Array<ExportSpecifier>,
            Boolean,
            Boolean,
        )>;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[doc = " Outputs the list of exports and locations of import specifiers,"]
        #[doc = " including dynamic import and import meta handling."]
        #[doc = " "]
        #[doc = " ## Arguments"]
        #[doc = " "]
        #[doc = " * `source` - Source code to parser"]
        #[doc = " * `name` - Optional sourcename"]
        #[doc = " "]
        #[doc = " ## Returns"]
        #[doc = " "]
        #[doc = " Tuple contaning imports list and exports list."]
        #[wasm_bindgen(catch, js_name = "parse")]
        pub fn try_parse(
            source: &str,
        ) -> Result<
            ArrayTuple<(
                Array<ImportSpecifier>,
                Array<ExportSpecifier>,
                Boolean,
                Boolean,
            )>,
            JsValue,
        >;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[doc = " Outputs the list of exports and locations of import specifiers,"]
        #[doc = " including dynamic import and import meta handling."]
        #[doc = " "]
        #[doc = " ## Arguments"]
        #[doc = " "]
        #[doc = " * `source` - Source code to parser"]
        #[doc = " * `name` - Optional sourcename"]
        #[doc = " "]
        #[doc = " ## Returns"]
        #[doc = " "]
        #[doc = " Tuple contaning imports list and exports list."]
        #[wasm_bindgen(js_name = "parse")]
        pub fn parse_with_name(
            source: &str,
            name: &str,
        ) -> ArrayTuple<(
            Array<ImportSpecifier>,
            Array<ExportSpecifier>,
            Boolean,
            Boolean,
        )>;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[doc = " Outputs the list of exports and locations of import specifiers,"]
        #[doc = " including dynamic import and import meta handling."]
        #[doc = " "]
        #[doc = " ## Arguments"]
        #[doc = " "]
        #[doc = " * `source` - Source code to parser"]
        #[doc = " * `name` - Optional sourcename"]
        #[doc = " "]
        #[doc = " ## Returns"]
        #[doc = " "]
        #[doc = " Tuple contaning imports list and exports list."]
        #[wasm_bindgen(catch, js_name = "parse")]
        pub fn try_parse_with_name(
            source: &str,
            name: &str,
        ) -> Result<
            ArrayTuple<(
                Array<ImportSpecifier>,
                Array<ExportSpecifier>,
                Boolean,
                Boolean,
            )>,
            JsValue,
        >;
    }
    #[wasm_bindgen(module = "es-module-lexer")]
    extern "C" {
        #[doc = " Wait for init to resolve before calling `parse`."]
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
