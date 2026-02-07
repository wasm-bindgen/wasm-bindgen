use crate::intrinsic::Intrinsic;
use crate::wit::AdapterId;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use walrus::TypedCustomSectionId;

/// A synthetic custom section which is not standardized, never will be, and
/// cannot be serialized or parsed. This is synthesized from all of the
/// compiler-emitted wasm-bindgen sections and then immediately removed to be
/// processed in the JS generation pass.
#[derive(Default, Debug)]
pub struct WasmBindgenAux {
    /// Extra typescript annotations that should be appended to the generated
    /// TypeScript file. This is provided via a custom attribute in Rust code.
    pub extra_typescript: Vec<String>,

    /// A map from identifier to the contents of each local module defined via
    /// the `#[wasm_bindgen(module = "/foo.js")]` import options.
    pub local_modules: HashMap<String, String>,

    /// A map from unique crate identifier to the list of inline JS snippets for
    /// that crate identifier.
    pub snippets: BTreeMap<String, Vec<String>>,

    /// A list of all `package.json` files that are intended to be included in
    /// the final build.
    pub package_jsons: HashSet<PathBuf>,

    /// A map from exported function id to where it's expected to be exported
    /// to.
    pub export_map: HashMap<AdapterId, AuxExport>,

    /// A map from imported function id to what it's expected to import.
    pub import_map: HashMap<AdapterId, AuxImport>,

    /// List of imports that should be re-exported, keyed by reexport name.
    pub reexports: BTreeMap<String, JsImport>,

    /// Small bits of metadata about imports.
    pub imports_with_catch: HashSet<AdapterId>,
    pub imports_with_variadic: HashSet<AdapterId>,
    pub imports_with_assert_no_shim: HashSet<AdapterId>,

    /// Auxiliary information to go into JS/TypeScript bindings describing the
    /// exported enums from Rust.
    pub enums: HashMap<String, AuxEnum>,
    /// Auxiliary information to go into JS/TypeScript bindings describing the
    /// exported string enums from Rust.
    pub string_enums: HashMap<String, AuxStringEnum>,

    /// Auxiliary information to go into JS/TypeScript bindings describing the
    /// exported structs from Rust and their fields they've got exported.
    pub structs: Vec<AuxStruct>,

    /// Information about various internal functions used to manage the `externref`
    /// table, later used to process JS bindings.
    pub externref_table: Option<walrus::TableId>,
    pub function_table: Option<walrus::TableId>,
    pub externref_alloc: Option<walrus::FunctionId>,
    pub externref_drop: Option<walrus::FunctionId>,
    pub externref_drop_slice: Option<walrus::FunctionId>,

    /// Various intrinsics used for JS glue generation
    pub exn_store: Option<walrus::FunctionId>,
    pub stack_pointer: Option<walrus::GlobalId>,
    pub thread_destroy: Option<walrus::FunctionId>,

    /// The imported JSTag for catching JavaScript exceptions in Wasm.
    /// When this is `Some`, all imports with `catch` use Wasm catch wrappers
    /// instead of JS `handleError` wrappers.
    pub js_tag: Option<walrus::TagId>,
}

pub type WasmBindgenAuxId = TypedCustomSectionId<WasmBindgenAux>;

#[derive(Debug)]
pub struct AuxExport {
    /// When generating errors about this export, a helpful name to remember it
    /// by.
    pub debug_name: String,
    /// Comments parsed in Rust and forwarded here to show up in JS bindings.
    pub comments: String,
    /// Function's argument info in Rust forwarded here to configure the signature
    /// that shows up in bindings.
    pub args: Option<Vec<AuxFunctionArgumentData>>,
    /// Whether this is an async function, to configure the TypeScript return value.
    pub asyncness: bool,
    /// What kind of function this is and where it shows up
    pub kind: AuxExportKind,
    /// The namespace to export the item through, if any
    pub js_namespace: Option<Vec<String>>,
    /// Whether typescript bindings should be generated for this export.
    pub generate_typescript: bool,
    /// Whether jsdoc comments should be generated for this export.
    pub generate_jsdoc: bool,
    /// Whether typescript bindings should be generated for this export.
    pub variadic: bool,
    /// Function's return overriding type
    pub fn_ret_ty_override: Option<String>,
    /// Function's return description
    pub fn_ret_desc: Option<String>,
}

/// Information about a functions' argument
#[derive(Debug, Clone)]
pub struct AuxFunctionArgumentData {
    /// Specifies the argument name
    pub name: String,
    /// Specifies the function argument type override
    pub ty_override: Option<String>,
    /// Specifies the argument description
    pub desc: Option<String>,
}

/// All possible kinds of exports from a Wasm module.
///
/// This `enum` says where to place an exported Wasm function. For example it
/// may want to get hooked up to a JS class, or it may want to be exported as a
/// free function (etc).
///
/// TODO: it feels like this should not really be here per se. We probably want
/// to either construct the JS object itself from within Wasm or somehow move
/// more of this information into some other section. Really what this is is
/// sort of an "export map" saying how to wire up all the free functions from
/// the Wasm module into the output expected JS module. All our functions here
/// currently take integer parameters and require a JS wrapper, but ideally
/// we'd change them one day to taking/receiving `externref` which then use some
/// sort of webidl import to customize behavior or something like that. In any
/// case this doesn't feel quite right in terms of privilege separation, so
/// we'll want to work on this. For now though it works.
#[derive(Debug)]
pub enum AuxExportKind {
    /// A free function that's just listed on the exported module
    Function(String),

    /// A free function that receives JS `this` as its first parameter
    FunctionThis(String),

    /// A function that's used to create an instance of a class. The function
    /// actually return just an integer which is put on an JS object currently.
    Constructor(String),

    /// A function that's associated with a class.
    ///
    /// This can either be a static method (indicated by `AuxReceiverKind::None`),
    /// which is basically just a free function namespaced under the class, or
    /// a proper method.
    ///
    /// It can also be a getter or a setter for a (possibly static) field of
    /// the class, in which case `name` is the name of the field.
    ///
    /// If the function isn't static, the first argument is the index of the
    /// Rust object in the JS heap.
    Method {
        class: String,
        name: String,
        receiver: AuxReceiverKind,
        kind: AuxExportedMethodKind,
    },
}

/// All the possible kinds of exported methods.
#[derive(Debug, Clone, Copy)]
pub enum AuxExportedMethodKind {
    /// A regular method.
    Method,
    /// A getter for a field.
    Getter,
    /// A setter for a field.
    Setter,
}

/// The 'receiver' of a method; in other words, the type that the method is called on.
///
/// This is `None` if the method is static, or `Borrowed` or `Owned` if the
/// method takes `&[mut] self` or `self` respectively.
#[derive(Debug, Clone, Copy)]
pub enum AuxReceiverKind {
    None,
    Borrowed,
    Owned,
}

impl AuxReceiverKind {
    /// Returns whether this is `AuxReceiverKind::None` (in other words,
    /// whether the method with this receiver is static).
    pub fn is_static(self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Debug)]
pub struct AuxEnum {
    /// The name of this enum
    pub name: String,
    /// The copied Rust comments to forward to JS
    pub comments: String,
    /// A list of variants with their name, value and comments
    /// and whether typescript bindings should be generated for each variant
    pub variants: Vec<(String, i64, String)>,
    /// Whether typescript bindings should be generated for this enum.
    pub generate_typescript: bool,
    /// Whether to not export this enum from the module exports
    pub private: bool,
    /// The namespace to export the enum through, if any
    pub js_namespace: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct AuxStringEnum {
    /// The name of this enum
    pub name: String,
    /// The copied Rust comments to forward to JS
    pub comments: String,
    /// A list of variants values
    pub variant_values: Vec<String>,
    /// Whether typescript bindings should be generated for this enum.
    pub generate_typescript: bool,
    /// The namespace to export the enum through, if any
    /// Note: Currently unused as string enums don't generate exports,
    /// but kept for consistency and potential future use.
    #[allow(dead_code)]
    pub js_namespace: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct AuxStruct {
    /// The name of this struct
    pub name: String,
    /// The copied Rust comments to forward to JS
    pub comments: String,
    /// Whether to generate helper methods for inspecting the class
    pub is_inspectable: bool,
    /// Whether typescript bindings should be generated for this struct.
    pub generate_typescript: bool,
    /// Whether to not export this struct from the module exports
    pub private: bool,
    /// The namespace to export the struct through, if any
    pub js_namespace: Option<Vec<String>>,
}

/// All possible types of imports that can be imported by a Wasm module.
///
/// This `enum` is intended to map out what an imported value is. For example
/// this contains a ton of shims and various ways you can call a function. The
/// base variant here is `Value` which simply means "hook this up to the import"
/// and the signatures will match up.
///
/// Note that this is *not* the same as the webidl bindings section. This is
/// intended to be coupled with that to map out what actually gets hooked up to
/// an import in the Wasm module. The two work in tandem.
/// Represents the source of a re-export.
///
/// Some of these items here are native to JS (like `Value`, indexing
/// operations, etc). Others are shims generated by wasm-bindgen (like `Closure`
/// or `Instanceof`).
#[derive(Debug)]
pub enum AuxImport {
    /// This import is expected to simply be whatever is the value that's
    /// imported
    Value(AuxValue),

    /// A static method on a class is being imported, and the `this` of the
    /// function call is expected to always be the class.
    ValueWithThis(JsImport, String),

    /// This import is expected to be a function that takes an `externref` and
    /// returns a `bool`. It's expected that it tests if the argument is an
    /// instance of (using `instanceof`) the name specified.
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    Instanceof(JsImport),

    /// This import is expected to be a shim that returns the JS value named by
    /// `JsImport`.
    Static { js: JsImport, optional: bool },

    /// This import is expected to be a shim that returns an exported `JsString`.
    String(String),

    /// This import is a generic cast function, monomorphised for a specific `fn(A) -> R` signature.
    Cast { sig_comment: String },

    /// This import is expected to be a shim that simply calls the `foo` method
    /// on the first object, passing along all other parameters and returning
    /// the resulting value.
    StructuralMethod(String),

    /// This import is a "structural getter" which simply returns the `.field`
    /// value of the first argument as an object.
    ///
    /// e.g. `function(x) { return x.foo; }`
    StructuralGetter(String),

    /// This import is a "structural getter" which simply returns the `.field`
    /// value of the specified class
    ///
    /// e.g. `function() { return TheClass.foo; }`
    StructuralClassGetter(JsImport, String),

    /// This import is a "structural setter" which simply sets the `.field`
    /// value of the first argument to the second argument.
    ///
    /// e.g. `function(x, y) { x.foo = y; }`
    StructuralSetter(String),

    /// This import is a "structural setter" which simply sets the `.field`
    /// value of the specified class to the first argument of the function.
    ///
    /// e.g. `function(x) { TheClass.foo = x; }`
    StructuralClassSetter(JsImport, String),

    /// This import is expected to be a shim that is an indexing getter of the
    /// JS class here, where the first argument of the function is the field to
    /// look up. The return value is the value of the field.
    ///
    /// e.g. `function(x) { return TheClass[x]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingGetterOfClass(JsImport),

    /// This import is expected to be a shim that is an indexing getter of the
    /// first argument interpreted as an object where the field to look up is
    /// the second argument.
    ///
    /// e.g. `function(x, y) { return x[y]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingGetterOfObject,

    /// This import is expected to be a shim that is an indexing setter of the
    /// JS class here, where the first argument of the function is the field to
    /// set and the second is the value to set it to.
    ///
    /// e.g. `function(x, y) { TheClass[x] = y; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingSetterOfClass(JsImport),

    /// This import is expected to be a shim that is an indexing setter of the
    /// first argument interpreted as an object where the next two fields are
    /// the field to set and the value to set it to.
    ///
    /// e.g. `function(x, y, z) { x[y] = z; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingSetterOfObject,

    /// This import is expected to be a shim that is an indexing deleter of the
    /// JS class here, where the first argument of the function is the field to
    /// delete.
    ///
    /// e.g. `function(x) { delete TheClass[x]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingDeleterOfClass(JsImport),

    /// This import is expected to be a shim that is an indexing deleter of the
    /// first argument interpreted as an object where the second argument is
    /// the field to delete.
    ///
    /// e.g. `function(x, y) { delete x[y]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingDeleterOfObject,

    /// This import is a generated shim which will wrap the provided pointer in
    /// a JS object corresponding to the Class name given here. The class name
    /// is one that is exported from the Rust/wasm.
    ///
    /// TODO: sort of like the export map below we should ideally create the
    /// `externref` from within Rust itself and then return it directly rather than
    /// requiring an intrinsic here to do so.
    WrapInExportedClass(String),

    /// This is an intrinsic function expected to be implemented with a JS glue
    /// shim. Each intrinsic has its own expected signature and implementation.
    Intrinsic(Intrinsic),

    /// This is a function which returns a URL pointing to a specific file,
    /// usually a JS snippet. The supplied path is relative to the JS glue shim.
    /// The Option may contain the contents of the linked file, so it can be
    /// embedded.
    LinkTo(String, Option<String>),

    /// This import is a generated shim which will attempt to unwrap JsValue to an
    /// instance of the given exported class. The class name is one that is
    /// exported from the Rust/wasm.
    UnwrapExportedClass(String),
}

/// Values that can be imported verbatim to hook up to an import.
#[derive(Debug)]
pub enum AuxValue {
    /// A bare JS value, no transformations, just put it in the slot.
    Bare(JsImport),

    /// A getter function for the class listed for the field, acquired using
    /// `getOwnPropertyDescriptor`.
    Getter(JsImport, String),

    /// Like `Getter`, but accesses a field of a class instead of an instance
    /// of the class.
    ClassGetter(JsImport, String),

    /// Like `Getter`, except the `set` property.
    Setter(JsImport, String),

    /// Like `Setter`, but for class fields instead of instance fields.
    ClassSetter(JsImport, String),
}

/// What can actually be imported and typically a value in each of the variants
/// above of `AuxImport`
///
/// A `JsImport` is intended to indicate what exactly is being imported for a
/// particular operation.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct JsImport {
    /// The base of whatever is being imported, either from a module, the global
    /// namespace, or similar.
    pub name: JsImportName,
    /// Various field accesses (like `.foo.bar.baz`) to hang off the `name`
    /// above.
    pub fields: Vec<String>,
}

/// Return value of `determine_import` which is where we look at an imported
/// function AST and figure out where it's actually being imported from
/// (performing some validation checks and whatnot).
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum JsImportName {
    /// An item is imported from the global scope. The `name` is what's
    /// imported.
    Global { name: String },
    /// Same as `Global`, except the `name` is imported via an ESM import from
    /// the specified `module` path.
    Module { module: String, name: String },
    /// Same as `Module`, except we're importing from a local module defined in
    /// a local JS snippet.
    LocalModule { module: String, name: String },
    /// Same as `Module`, except we're importing from an `inline_js` attribute
    InlineJs {
        unique_crate_identifier: String,
        snippet_idx_in_crate: usize,
        name: String,
    },
    /// A global import which may have a number of vendor prefixes associated
    /// with it, like `webkitAudioPrefix`. The `name` is the name to test
    /// whether it's prefixed.
    VendorPrefixed { name: String, prefixes: Vec<String> },
}

impl walrus::CustomSection for WasmBindgenAux {
    fn name(&self) -> &str {
        "wasm-bindgen custom section"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<'_, [u8]> {
        panic!("shouldn't emit custom sections just yet");
    }

    fn add_gc_roots(&self, roots: &mut walrus::passes::Roots) {
        if let Some(id) = self.externref_table {
            roots.push_table(id);
        }
        if let Some(id) = self.function_table {
            roots.push_table(id);
        }
        if let Some(id) = self.externref_alloc {
            roots.push_func(id);
        }
        if let Some(id) = self.externref_drop_slice {
            roots.push_func(id);
        }
        if let Some(id) = self.exn_store {
            roots.push_func(id);
        }
        if let Some(id) = self.stack_pointer {
            roots.push_global(id);
        }
        if let Some(id) = self.thread_destroy {
            roots.push_func(id);
        }
        if let Some(id) = self.js_tag {
            roots.push_tag(id);
        }
    }
}
