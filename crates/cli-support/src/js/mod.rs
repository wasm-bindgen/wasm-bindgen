use crate::descriptor::VectorKind;
use crate::intrinsic::Intrinsic;
use crate::transforms::{
    has_local_exception_tags, threads as threads_xform, unstart_start_function,
};
use crate::wasm_conventions::get_memory;
use crate::wit::{
    Adapter, AdapterId, AdapterJsImportKind, AuxExportedMethodKind, AuxReceiverKind, AuxStringEnum,
    AuxValue,
};
use crate::wit::{AdapterKind, Instruction, InstructionData};
use crate::wit::{AuxEnum, AuxExport, AuxExportKind, AuxImport, AuxStruct};
use crate::wit::{JsImport, JsImportName, NonstandardWitSection, WasmBindgenAux};
use crate::{Bindgen, EncodeInto, OutputMode, PLACEHOLDER_MODULE};
use anyhow::{anyhow, bail, Context as _, Error};
use binding::TsReference;
use std::borrow::Cow;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::{fmt, mem};
use walrus::{FunctionId, ImportId, MemoryId, Module, TableId, ValType};
use wasm_bindgen_shared::escape_string;
use wasm_bindgen_shared::identifier::{is_valid_ident, to_valid_ident};

mod binding;

macro_rules! region {
    ($ctx:expr, $name:literal, $code:block) => {
        if $ctx.config.debug {
            $ctx.globals.push_str(concat!("\n//#region ", $name, "\n"));
        }
        $code
        if $ctx.config.debug {
            $ctx.globals.push_str("\n//#endregion\n");
        }
    };
}

pub struct Context<'a> {
    globals: String,
    /// ES module `import` statements collected during codegen, emitted at the
    /// top of the output in `finalize()`. In bundler mode these go into the
    /// `_bg.js` file; the bundler entry file has its own hardcoded imports.
    es_module_imports: String,
    intrinsics: Option<BTreeMap<Cow<'static, str>, Cow<'static, str>>>,
    emscripten_library: String,
    imports_post: String,
    export_name_list: Vec<String>,
    typescript: String,
    typescript_emscripten_classes: String,
    config: &'a Bindgen,
    pub module: &'a mut Module,
    aux: &'a WasmBindgenAux,
    wit: &'a NonstandardWitSection,

    /// A map representing the `import` statements we'll be generating in the JS
    /// glue. The key is the module we're importing from and the value is the
    /// list of identifier we're importing from the module, with optional
    /// renames for each identifier.
    js_imports: HashMap<String, Vec<(String, Option<String>)>>,

    /// A map of each Wasm import and what JS to hook up to it.
    wasm_import_definitions: HashMap<ImportId, String>,

    /// A map from an import to the name we've locally imported it as.
    imported_names: HashMap<JsImportName, String>,

    /// A set of all defined identifiers through either exports or imports to
    /// the number of times they've been used, used to generate new
    /// identifiers.
    defined_identifiers: HashMap<String, usize>,

    /// A set of all (tracked) symbols referenced from within type definitions,
    /// function signatures, etc.
    typescript_refs: HashSet<TsReference>,

    /// String enums that are used internally by the generated bindings.
    ///
    /// This tracks which string enums are used independently from whether their
    /// type is used, because users may only use them in a way that doesn't
    /// require the type or requires only the type.
    used_string_enums: HashSet<String>,

    exported_classes: BTreeMap<String, ExportedClass>,

    /// Public module exports
    exports: BTreeMap<String, ExportEntry>,

    /// A map of the name of npm dependencies we've loaded so far to the path
    /// they're defined in as well as their version specification.
    pub npm_dependencies: HashMap<String, (PathBuf, String)>,

    /// A mapping from the memory IDs as we see them to an index for that memory,
    /// used in function names, as well as all the kinds of views we've created
    /// of that memory.
    ///
    /// `BTreeMap` and `BTreeSet` are used to make the ordering deterministic.
    memories: BTreeMap<MemoryId, (usize, BTreeSet<&'static str>)>,
    table_indices: HashMap<TableId, usize>,

    /// A flag to track if the stack pointer setter shim has been injected.
    stack_pointer_shim_injected: bool,

    /// If threading is enabled.
    threads_enabled: bool,

    /// If exception handling / unwinding is enabled.
    unwind_enabled: bool,

    /// Whether reinit machinery should be emitted. True when `schedule_reinit()`
    /// is used (auto-detected via the `__wbindgen_reinit` intrinsic) or when
    /// `--experimental-reset-state-function` is passed. Controls instance-id
    /// tracking, the `__wbg_instance_id` global, and the private
    /// `__wbg_reset_state` function.
    generate_reinit: bool,

    /// Mapping from qualified name (used in WasmDescribe) to rust_name (used as exported_classes key).
    pub(crate) qualified_to_rust_name: HashMap<String, String>,

    /// Mapping from qualified name (used in WasmDescribe) to the unique declaration identifier.
    pub(crate) qualified_to_identifier: HashMap<String, String>,
    /// Tracks dependencies (Emscripten imports) for the current adapter being generated.
    /// Must be cleared at the start of `generate_adapter`.
    adapter_deps: BTreeSet<String>,

    /// Tracks global emscripten dependencies as opposed to adapter-level dependencies.
    emscripten_global_deps: BTreeSet<String>,

    /// Tracks the specific Emscripten dependencies for each individual Wasm import.
    /// These are gathered from `adapter_deps` during adapter generation.
    emscripten_import_deps: BTreeMap<ImportId, BTreeSet<String>>,

    /// `true` when the module's memory is a memory64 (wasm64) memory.
    memory64: bool,
}

/// Definition of a module export
enum ExportEntry {
    /// Any export definition
    Definition(ExportDefinition),
    /// Namespace export
    Namespace(ExportedNamespace),
}

struct ExportDefinition {
    /// The identifier for the declaration, if distinct from the export name
    /// This allows invalid identifier export names (like "default").
    identifier: String,
    comments: Option<String>,
    definition: String,

    ts_comments: Option<String>,
    ts_definition: String,

    /// Whether this is a private export, so not actually exposed on the module exports interface
    private: bool,
}

/// Module namespace export
#[derive(Default)]
struct ExportedNamespace {
    /// Namespace id.
    id: Option<String>,
    /// Namespace entries.
    ns: BTreeMap<String, ExportEntry>,
}

#[derive(Default)]
struct ExportedClass {
    comments: String,
    contents: String,
    identifier: String,
    /// The TypeScript for the class's methods.
    typescript: String,
    /// Whether TypeScript for this class should be emitted (i.e., `skip_typescript` wasn't specified).
    generate_typescript: bool,
    /// Whether to skip exporting this class from the module exports
    private: bool,
    has_constructor: bool,
    wrap_needed: bool,
    unwrap_needed: bool,
    /// Whether to generate helper methods for inspecting the class
    is_inspectable: bool,
    /// All readable properties of the class
    readable_properties: Vec<String>,
    /// Map from field to information about those fields
    typescript_fields: HashMap<FieldLocation, FieldInfo>,
    /// The namespace to export the class through, if any
    js_namespace: Option<Vec<String>>,
    /// The JS-facing name of this class (used for JS output)
    js_name: Option<String>,
    /// The namespace-qualified name (used for wasm symbol references)
    qualified_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FieldLocation {
    name: String,
    is_static: bool,
}
#[derive(Debug)]
struct FieldInfo {
    name: String,
    is_static: bool,
    order: usize,
    getter: Option<FieldAccessor>,
    setter: Option<FieldAccessor>,
}
/// A getter or setter for a field.
#[derive(Debug)]
struct FieldAccessor {
    ty: String,
    docs: String,
    is_optional: bool,
}

const INITIAL_HEAP_VALUES: &[&str] = &["undefined", "null", "true", "false"];
// Must be kept in sync with `src/lib.rs` of the `wasm-bindgen` crate
const INITIAL_HEAP_OFFSET: usize = 1024;

impl<'a> Context<'a> {
    pub fn new(
        module: &'a mut Module,
        config: &'a Bindgen,
        wit: &'a NonstandardWitSection,
        aux: &'a WasmBindgenAux,
    ) -> Result<Context<'a>, Error> {
        let memory64 = module.memories.iter().next().is_some_and(|m| m.memory64);
        Ok(Context {
            globals: String::new(),
            es_module_imports: String::new(),
            intrinsics: Some(Default::default()),
            imports_post: String::new(),
            export_name_list: Vec::new(),
            typescript: "/* tslint:disable */\n/* eslint-disable */\n".to_string(),
            typescript_emscripten_classes: String::new(),
            imported_names: Default::default(),
            js_imports: Default::default(),
            defined_identifiers: Default::default(),
            wasm_import_definitions: Default::default(),
            typescript_refs: Default::default(),
            used_string_enums: Default::default(),
            exported_classes: Default::default(),
            exports: Default::default(),
            config,
            threads_enabled: threads_xform::is_enabled(module),
            unwind_enabled: has_local_exception_tags(module),
            generate_reinit: aux.uses_reinit || config.generate_reset_state,
            module,
            npm_dependencies: Default::default(),
            wit,
            aux,
            memories: Default::default(),
            table_indices: Default::default(),
            stack_pointer_shim_injected: false,
            qualified_to_rust_name: Default::default(),
            qualified_to_identifier: Default::default(),
            emscripten_library: String::new(),
            adapter_deps: Default::default(),
            emscripten_global_deps: Default::default(),
            emscripten_import_deps: Default::default(),
            memory64,
        })
    }

    fn has_intrinsic(&self, name: &str) -> bool {
        self.intrinsics.as_ref().unwrap().contains_key(name)
    }

    /// Suffix ` >>> 0` on wasm32, empty on wasm64. Append to a pointer-sized
    /// expression about to be used as a typed-array / DataView index.
    fn coerce_ptr_suffix(&self) -> &'static str {
        if self.memory64 {
            ""
        } else {
            " >>> 0"
        }
    }

    /// Statement form of [`Self::coerce_ptr_suffix`]: normalizes `name` in
    /// place on wasm32, expands to nothing on wasm64.
    fn coerce_ptr_assign(&self, name: &str) -> String {
        if self.memory64 {
            String::new()
        } else {
            format!("{name} = {name} >>> 0;")
        }
    }

    /// A helper function to add any necessary addToLibrary wrappings for Emscripten
    fn export_to_emscripten(&mut self, js_name: &str, raw_js: &str) {
        let content = raw_js.trim().to_string();

        if content.is_empty() {
            return;
        }

        self.emscripten_library(&content);

        if !content.contains("addToLibrary") {
            self.emscripten_library(&format!("addToLibrary({{ '${js_name}': {js_name} }});"));
        }
    }

    fn intrinsic(
        &mut self,
        name: Cow<'static, str>,
        js_name: Option<&str>,
        val: Cow<'static, str>,
    ) {
        if self.intrinsics.as_ref().unwrap().contains_key(&name) {
            return;
        }

        if matches!(self.config.mode, OutputMode::Emscripten) {
            let actual_js_name: &str = js_name.unwrap_or(&name);

            // Only add as a dependency and export if there's actual content.
            // Empty content means the intrinsic is handled elsewhere (e.g. via
            // emscripten_global_deps), so registering a dep here would create a
            // reference to a non-existent library variable.
            if !val.trim().is_empty() {
                self.adapter_deps.insert(actual_js_name.to_string());
                self.export_to_emscripten(actual_js_name, &val);
            }

            // Register empty string so standard generation skips this key
            self.intrinsics.as_mut().unwrap().insert(name, "".into());
        } else {
            self.intrinsics.as_mut().unwrap().insert(name, val);
        }
    }
    /// Writes an ExportDefinition to global and typescript buffers.
    /// Handles realising for invalid identifier export names.
    /// define_only used when only locally declared but not explicitly exported.
    fn export_def(&mut self, export_name: Option<&str>, def: &ExportDefinition) {
        let ExportDefinition {
            definition: decl,
            ts_definition: ts_decl,
            identifier: id,
            comments,
            ts_comments,
            private,
        } = def;
        self.globals.push('\n');
        if self.config.typescript && !ts_decl.is_empty() {
            self.typescript.push('\n');
        }
        // Unless it is `export ...declaration...` form (common case), write the declaration first
        // and then export.
        if export_name.map(|name| name != id).unwrap_or(true)
            || matches!(
                self.config.mode,
                OutputMode::Node { module: false } | OutputMode::NoModules { .. }
            )
            || *private
        {
            if let Some(c) = comments {
                self.globals.push_str(c);
            }
            self.globals.push_str(decl);
        } else if let Some(c) = comments {
            self.globals.push_str(c);
        }
        if self.config.typescript && !ts_decl.is_empty() {
            if export_name.map(|name| name != id).unwrap_or(true) || *private {
                if let Some(c) = ts_comments {
                    self.typescript.push_str(c);
                }
                // in nomodules, we output into a namespace, which is already ambient
                if !self.config.mode.no_modules() && !self.config.mode.emscripten() {
                    self.typescript.push_str("declare ");
                }
                self.typescript.push_str(ts_decl);
            } else if let Some(c) = ts_comments {
                self.typescript.push_str(c);
            }
        }
        if *private {
            return;
        }
        if let Some(export_name) = export_name {
            match self.config.mode {
                OutputMode::Node { module: false } | OutputMode::NoModules { .. } => self
                    .globals
                    .push_str(&format!("exports.{export_name} = {id};\n")),
                OutputMode::Bundler { .. }
                | OutputMode::Node { module: true }
                | OutputMode::Web
                | OutputMode::Module
                | OutputMode::Deno => {
                    if export_name == id {
                        if !decl.is_empty() {
                            self.globals.push_str(&format!("export {decl}"));
                        } else {
                            // reexport case
                            self.globals.push_str(&format!("export {{ {id} }}\n"));
                        }
                    } else if is_valid_ident(export_name) {
                        self.globals
                            .push_str(&format!("export {{ {id} as {export_name} }}\n"));
                    } else {
                        self.globals
                            .push_str(&format!("export {{ {id} as '{export_name}' }}\n"));
                    }
                }
                OutputMode::Emscripten => {
                    if let Some(c) = comments {
                        self.globals.push_str(c);
                    }

                    if decl.trim_start().starts_with("const") {
                        self.globals.push_str("export ");
                        self.globals.push_str(decl);
                    } else {
                        self.globals.push_str(decl);

                        if export_name == id {
                            self.global(&format!("Module.{export_name} = {id};\n"));
                        } else {
                            self.global(&format!("export {{ {id} as {export_name} }};\n"));
                        }
                    }
                }
            }

            if self.config.typescript && !ts_decl.is_empty() {
                if export_name == "default" {
                    self.typescript.push_str(&format!("export default {id};\n"));
                } else if export_name == id {
                    if !ts_decl.is_empty() {
                        self.typescript.push_str(&format!("export {ts_decl}"));
                    }
                } else if is_valid_ident(export_name) {
                    self.typescript
                        .push_str(&format!("export {{ {id} as {export_name} }}\n"));
                } else {
                    self.typescript
                        .push_str(&format!("// export {{ {id} as '{export_name}' }}\n"));
                }
            }
        }
    }

    pub fn finalize(
        &mut self,
        module_name: &str,
    ) -> Result<(String, String, Option<String>), Error> {
        // Finalize all bindings for JS classes. This is where we'll generate JS
        // glue for all classes as well as finish up a few final imports like
        // `__wrap` and such.
        self.write_classes()?;
        let emscripten_classes_out = if matches!(self.config.mode, OutputMode::Emscripten) {
            // EMSCRIPTEN-only: classes and exports are written inside of $initBindgen,
            // via generate_wasm_loading.
            std::mem::take(&mut self.globals)
        } else {
            String::new()
        };

        // Process reexports
        for (export_name, (js_import, generate_typescript)) in self.aux.reexports.clone() {
            let import_name = self.import_name(&js_import)?;
            let ts_definition = if generate_typescript {
                format!("let {import_name}: unknown;\n")
            } else {
                String::new()
            };
            define_export(
                &mut self.exports,
                &export_name,
                &[],
                ExportEntry::Definition(ExportDefinition {
                    identifier: import_name,
                    comments: None,
                    definition: "".to_string(),
                    ts_comments: None,
                    ts_definition,
                    private: false,
                }),
            )?;
        }

        let body = std::mem::take(&mut self.globals);
        let body = body.trim();

        if self.config.typescript && !self.config.mode.no_modules() && !self.config.mode.bundler() {
            // jsr-self-types directive
            let directive = format!("/* @ts-self-types=\"./{module_name}.d.ts\" */\n");
            self.globals.push_str(&directive);
        }

        if !self.js_imports.is_empty() {
            self.js_import_header()?;
        }

        if !self.exports.is_empty() {
            // Write out all exports
            region!(self, "exports", {
                self.write_exports()?;
            });
        }
        let emscripten_exports_out = if matches!(self.config.mode, OutputMode::Emscripten) {
            std::mem::take(&mut self.globals)
        } else {
            String::new()
        };

        if let Some(mem) = self.module.memories.iter().next() {
            if let Some(id) = mem.import {
                self.module.imports.get_mut(id).module = PLACEHOLDER_MODULE.to_owned();
                let mut init_memory = "new WebAssembly.Memory({".to_string();
                init_memory.push_str(&format!("initial:{}", mem.initial));
                if let Some(max) = mem.maximum {
                    init_memory.push_str(&format!(",maximum:{max}"));
                }
                if mem.shared {
                    init_memory.push_str(",shared:true");
                }
                init_memory.push_str("})");
                self.wasm_import_definitions.insert(id, init_memory);
            }
        }

        let mut has_memory = false;
        if let Some(mem) = self.module.memories.iter().next() {
            if let Some(id) = mem.import {
                if let Some(def) = self.wasm_import_definitions.get_mut(&id) {
                    if !self.config.mode.bundler() {
                        def.insert_str(0, "memory || ");
                    }
                    has_memory = true;
                }
            }
        }

        region!(self, "wasm imports", {
            let imports = self.generate_imports(module_name, has_memory);
            self.globals.push_str(&imports);
        });

        let imports_post = std::mem::take(&mut self.imports_post);
        let imports_post = imports_post.trim();

        if !imports_post.is_empty() {
            self.globals.push_str(imports_post);
            self.globals.push('\n');
        }
        if !body.is_empty() {
            self.globals.push_str(body);
            self.globals.push_str("\n\n");
        }

        let intrinsics = self.intrinsics.take().unwrap();
        if !intrinsics.is_empty() {
            region!(self, "intrinsics", {
                for code in intrinsics.values() {
                    self.globals.push_str(code.trim());
                    self.globals.push_str("\n\n");
                }
            });
        }

        // Initialization is just flat out tricky and not something we
        // understand super well. To try to handle various issues that have come
        // up we always remove the `start` function if one is present. The JS
        // bindings glue then manually calls the start function (if it was
        // previously present).
        let needs_manual_start = unstart_start_function(self.module);
        region!(self, "wasm loading", {
            let wrapped_content = if matches!(self.config.mode, OutputMode::Emscripten) {
                format!("{emscripten_classes_out}\n{emscripten_exports_out}")
            } else {
                String::new()
            };

            let wasm_loading = self.generate_wasm_loading(
                module_name,
                needs_manual_start,
                has_memory,
                &wrapped_content,
            );
            self.globals.push_str(&wasm_loading);
        });

        let mut start = self
            .config
            .mode
            .bundler()
            .then(|| self.generate_bundler_start(module_name, needs_manual_start));

        if let Some(start) = &mut start {
            mem::swap(&mut self.globals, start);
        }

        let already_exports_wasm_for_node_esm_threads =
            self.threads_enabled && matches!(self.config.mode, OutputMode::Node { module: true });
        if self.config.debug
            && !already_exports_wasm_for_node_esm_threads
            && !matches!(self.config.mode, OutputMode::Emscripten)
        {
            if self.config.mode.uses_es_modules() {
                self.globals.push_str("export { wasm as __wasm }");
            } else {
                self.globals.push_str("exports.__wasm = wasm;");
            }
        }

        // Insert collected ES module imports at the top of the output.
        if !self.es_module_imports.is_empty() {
            let mut es_imports = std::mem::take(&mut self.es_module_imports);
            es_imports.push('\n');
            if let Some(bg_js) = &mut start {
                // Bundler: self.globals is the entry file (hardcoded imports),
                // bg_js is the _bg.js glue file that needs these imports.
                bg_js.insert_str(0, &es_imports);
            } else {
                // Other ESM modes: insert after the @ts-self-types directive.
                let insert_pos = if self.globals.starts_with("/* @ts-self-types") {
                    self.globals.find('\n').map(|p| p + 1).unwrap_or(0)
                } else {
                    0
                };
                self.globals.insert_str(insert_pos, &es_imports);
            }
        }

        let mut ts = String::new();

        if self.config.mode.no_modules() {
            let mut iife = "
            let wasm_bindgen = (function(exports) {
            let script_src;
            if (typeof document !== 'undefined' && document.currentScript !== null) {
                script_src = new URL(document.currentScript.src, location.href).toString();
            }
            "
            .to_owned();
            iife.push_str(&self.globals);
            iife.push_str(
                "
                return Object.assign(__wbg_init, { initSync }, exports);
                })({ __proto__: null });
                ",
            );
            self.globals = iife;
            ts = String::from("declare namespace wasm_bindgen {\n");
            ts.push_str(&self.typescript);
            ts.push_str("\n}");
        } else if matches!(self.config.mode, OutputMode::Emscripten) {
            ts = self.typescript_emscripten_classes.clone();

            ts.push_str("interface BindgenModule {\n");
            for line in self.typescript.lines() {
                ts.push_str("  ");
                ts.push_str(line);
                ts.push('\n');
            }
            ts.push_str("}\n\n");
        } else {
            ts.push_str(&self.typescript);
        }

        // Generate TypeScript definitions for init functions in web and no-modules modes
        if self.config.typescript
            && matches!(
                self.config.mode,
                OutputMode::Web | OutputMode::NoModules { .. }
            )
        {
            let has_module_or_path_optional = !self.config.omit_default_module_path;
            let init_ts = self.ts_for_init_fn(has_memory, has_module_or_path_optional)?;
            ts.push_str(&init_ts);
        }

        // Generate TypeScript definitions for Node.js with threads enabled
        if self.config.typescript
            && matches!(self.config.mode, OutputMode::Node { .. })
            && self.threads_enabled
        {
            let node_atomics_ts = self.ts_for_node_atomics()?;
            ts.push_str(&node_atomics_ts);
        }

        Ok((self.globals.to_owned(), ts, start))
    }

    fn generate_esm_cjs_imports(&mut self, module_name: &str, has_memory: bool) -> String {
        let mut imports = String::new();
        let init_memory_arg = if has_memory { "memory" } else { "" };
        let mut fn_def = format!(
            "function __wbg_get_imports({init_memory_arg}) {{
            const import0 = {{
            __proto__: null,
        "
        );

        let self_module_name = format!("./{module_name}_bg.js");
        let mut return_stmt = format!(
            r#"
        return {{
            __proto__: null,
            "{self_module_name}": import0,
        "#
        );

        // e.g. snippets without parameters
        let import_modules = self
            .module
            .imports
            .iter()
            .map(|import| &import.module)
            .filter(|module| module.as_str() != PLACEHOLDER_MODULE);
        for (i, module) in import_modules.enumerate() {
            let i = i + 1;
            if self.config.mode.uses_es_modules() {
                self.es_module_imports
                    .push_str(&format!("import * as import{i} from \"{module}\"\n"));
            } else {
                imports.push_str(&format!(r#"const import{i} = require("{module}");"#));
                imports.push('\n');
            }

            return_stmt.push_str(&format!(r#""{module}": import{i},"#));
            return_stmt.push('\n');
        }
        return_stmt.push_str("};\n");

        for (id, js) in iter_by_import(&self.wasm_import_definitions, self.module) {
            let import = self.module.imports.get_mut(*id);
            fn_def.push_str(&format!("{}: {},\n", &import.name, js.trim()));
            import.module = self_module_name.clone();
        }

        fn_def.push_str("};");
        fn_def.push_str(&return_stmt);
        fn_def.push_str("}\n");

        if imports.is_empty() {
            format!("{fn_def}\n")
        } else {
            format!("{imports}\n{fn_def}\n")
        }
    }

    fn generate_bundler_imports(&mut self, module_name: &str) -> String {
        let mut imports = String::new();
        let self_module_name = format!("./{module_name}_bg.js");
        for (id, js) in iter_by_import(&self.wasm_import_definitions, self.module) {
            let import = self.module.imports.get_mut(*id);
            if let Some(body) = js.strip_prefix("function") {
                imports.push_str("export function ");
                imports.push_str(&import.name);
                imports.push_str(body.trim());
                imports.push('\n');
            } else {
                imports.push_str("\nexport const ");
                imports.push_str(&import.name);
                imports.push_str(" = ");
                imports.push_str(js.trim());
                imports.push_str(";\n");
            }
            import.module = self_module_name.clone();
        }
        imports
    }

    fn generate_emscripten_imports(&mut self) -> String {
        let mut imports = String::new();

        for import in self.module.imports.iter_mut() {
            if import.module == crate::PLACEHOLDER_MODULE
                || import.module == "__wbindgen_placeholder__"
            {
                import.module = "env".to_string();
            }
        }
        // Inject Intrinsics
        if !self.emscripten_library.is_empty() {
            imports.push_str(&self.emscripten_library);
            imports.push('\n');
        }

        imports.push_str("addToLibrary({\n");
        // Declare `wasm` as a library variable so it can be assigned in initBindgen.
        // In emscripten's modularized output, implicit global assignment is not allowed
        // (strict mode), so we need an explicit declaration.
        imports.push_str("$wasm: \"null\",\n");
        // Inject Global Dependencies
        for global_dep in self.emscripten_global_deps.iter() {
            if global_dep == "WASM_VECTOR_LEN" {
                imports.push_str("$WASM_VECTOR_LEN: '0',\n");
            } else if global_dep == "cachedTextEncoder" {
                imports.push_str("$cachedTextEncoder: \"new TextEncoder()\",\n");
            } else if global_dep == "cachedTextDecoder" {
                imports.push_str("$cachedTextDecoder: \"new TextDecoder()\",\n");
            } else if global_dep == "heap" {
                imports.push_str(&format!(
                    "$heap: \"new Array({INITIAL_HEAP_OFFSET}).fill(undefined)\",\n$heap__postset: \"heap.push({}); heap_next = heap.length;\",\n",
                    INITIAL_HEAP_VALUES.join(", ")
                ));
            } else if global_dep == "heap_next" {
                imports.push_str("$heap_next: '0',\n");
            } else if global_dep == "stack_pointer" {
                imports.push_str(&format!("$stack_pointer : \"{INITIAL_HEAP_OFFSET}\",\n"));
            }
        }
        imports.push_str("});\n\n");

        // Inject Imports
        for (id, js) in iter_by_import(&self.wasm_import_definitions, self.module) {
            let import = self.module.imports.get(*id);
            let name = &import.name;

            let trimmed_js = js.trim();
            imports.push_str("addToLibrary({\n");
            imports.push_str(&format!("  {name}: {trimmed_js},\n"));

            if let Some(deps) = self.emscripten_import_deps.get(id) {
                let formatted_deps: Vec<String> =
                    deps.iter().map(|dep| format!("'${dep}'")).collect();

                imports.push_str(&format!(
                    "  {name}__deps: [{}],\n",
                    formatted_deps.join(", ")
                ));
            }
            imports.push_str("});\n\n");
        }
        imports
    }

    fn generate_imports(&mut self, module_name: &str, has_memory: bool) -> String {
        match self.config.mode {
            OutputMode::Bundler { .. } => self.generate_bundler_imports(module_name),
            OutputMode::Emscripten => self.generate_emscripten_imports(),
            _ => self.generate_esm_cjs_imports(module_name, has_memory),
        }
    }

    /// Collects JS module import statements from `self.js_imports`.
    /// For ESM modes, writes `import { ... }` to `self.es_module_imports`.
    /// For CJS Node / Emscripten, writes inline to `self.globals`.
    fn js_import_header(&mut self) -> Result<(), Error> {
        if self.config.omit_imports {
            return Ok(());
        }

        match &self.config.mode {
            OutputMode::NoModules { .. } => {
                if let Some((module, _items)) = self.js_imports.iter().next() {
                    bail!("importing from `{module}` isn't supported with `--target no-modules`");
                }
            }

            OutputMode::Node { module: false } => {
                for (module, items) in crate::sorted_iter(&self.js_imports) {
                    self.globals.push_str("const { ");
                    for (i, (item, rename)) in items.iter().enumerate() {
                        if i > 0 {
                            self.globals.push_str(", ");
                        }
                        if is_valid_ident(item) {
                            self.globals.push_str(item);
                        } else {
                            assert!(rename.is_some());
                            self.globals.push('\'');
                            self.globals.push_str(&escape_string(item));
                            self.globals.push('\'');
                        }
                        if let Some(other) = rename {
                            self.globals.push_str(": ");
                            self.globals.push_str(other)
                        }
                    }
                    if module.starts_with('.') || PathBuf::from(module).is_absolute() {
                        self.globals.push_str(" } = require(String.raw`");
                    } else {
                        self.globals.push_str(" } = require(`");
                    }
                    self.globals.push_str(module);
                    self.globals.push_str("`);\n");
                }
            }

            OutputMode::Emscripten => {
                for (module, items) in crate::sorted_iter(&self.js_imports) {
                    write_es_import(&mut self.globals, module, items);
                }
            }

            OutputMode::Bundler { .. }
            | OutputMode::Node { module: true }
            | OutputMode::Web
            | OutputMode::Module
            | OutputMode::Deno => {
                for (module, items) in crate::sorted_iter(&self.js_imports) {
                    write_es_import(&mut self.es_module_imports, module, items);
                }
            }
        }
        Ok(())
    }

    fn ts_for_init_fn(
        &self,
        has_memory: bool,
        has_module_or_path_optional: bool,
    ) -> Result<String, Error> {
        if matches!(self.config.mode, OutputMode::Emscripten) {
            return Ok("".to_string());
        }
        let output = crate::wasm2es6js::interface(self.module)?;

        let (memory_doc, memory_param) = if has_memory {
            (
                "* @param {WebAssembly.Memory} memory - Deprecated.\n",
                ", memory?: WebAssembly.Memory",
            )
        } else {
            ("", "")
        };
        let stack_size = if self.threads_enabled {
            ", thread_stack_size?: number"
        } else {
            ""
        };
        let arg_optional = if has_module_or_path_optional { "?" } else { "" };
        // With TypeScript 3.8.3, I'm seeing that any "export"s at the root level cause TypeScript to ignore all "declare" statements.
        // So using "declare" everywhere for at least the NoModules option.
        // Also in (at least) the NoModules, the `init()` method is renamed to `wasm_bindgen()`.
        let setup_function_declaration;
        let mut sync_init_function = String::new();
        let declare_or_export;
        if self.config.mode.no_modules() {
            declare_or_export = "declare";
            setup_function_declaration = "declare function wasm_bindgen";
        } else {
            declare_or_export = "export";

            sync_init_function.push_str(&format!(
                "\
                {declare_or_export} type SyncInitInput = BufferSource | WebAssembly.Module;\n\n\
                /**\n\
                * Instantiates the given `module`, which can either be bytes or\n\
                * a precompiled `WebAssembly.Module`.\n\
                *\n\
                * @param {{{{ module: SyncInitInput{memory_param}{stack_size} }}}} module - Passing `SyncInitInput` directly is deprecated.\n\
                {memory_doc}\
                *\n\
                * @returns {{InitOutput}}\n\
                */\n\
                export function initSync(module: {{ module: SyncInitInput{memory_param}{stack_size} }} | SyncInitInput{memory_param}): InitOutput;\n\n\
                "
            ));

            setup_function_declaration = "export default function __wbg_init";
        }
        Ok(format!(
            "\n\
            {declare_or_export} type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;\n\
            \n\
            {declare_or_export} interface InitOutput {{\n\
            {output}}}\n\
            \n\
            {sync_init_function}\
            /**\n\
            * If `module_or_path` is {{RequestInfo}} or {{URL}}, makes a request and\n\
            * for everything else, calls `WebAssembly.instantiate` directly.\n\
            *\n\
            * @param {{{{ module_or_path: InitInput | Promise<InitInput>{memory_param}{stack_size} }}}} module_or_path - Passing `InitInput` directly is deprecated.\n\
            {memory_doc}\
            *\n\
            * @returns {{Promise<InitOutput>}}\n\
            */\n\
            {setup_function_declaration} \
                (module_or_path{arg_optional}: {{ module_or_path: InitInput | Promise<InitInput>{memory_param}{stack_size} }} | InitInput | Promise<InitInput>{memory_param}): Promise<InitOutput>;\n",
        ))
    }

    /// Generate TypeScript definitions for Node.js targets with threads/atomics enabled.
    fn ts_for_node_atomics(&self) -> Result<String, Error> {
        let output = crate::wasm2es6js::interface(self.module)?;

        Ok(format!(
            r#"
export type SyncInitInput = BufferSource | WebAssembly.Module;

export interface InitOutput {{
{output}}}

export interface InitSyncOptions {{
    module?: SyncInitInput;
    memory?: WebAssembly.Memory;
    thread_stack_size?: number;
}}

/**
 * Initialize the WebAssembly module synchronously.
 *
 * For the main thread, this is called automatically on import.
 * Worker threads should call this explicitly with shared module and memory:
 *
 * ```js
 * initSync({{ module: __wbg_wasm_module, memory: __wbg_memory }});
 * ```
 *
 * @param opts - Initialization options
 * @returns The exports object
 */
export function initSync(opts?: InitSyncOptions): InitOutput;

/**
 * Get the imports object for WebAssembly instantiation.
 *
 * @param memory - Optional shared memory to use instead of creating new
 * @returns The imports object for WebAssembly.Instance
 */
export function __wbg_get_imports(memory?: WebAssembly.Memory): WebAssembly.Imports;

/** The compiled WebAssembly module. Can be shared with workers. */
export const __wbg_wasm_module: WebAssembly.Module;

/** The shared WebAssembly memory. */
export const __wbg_memory: WebAssembly.Memory;
"#
        ))
    }

    fn generate_module_wasm_loading(
        &mut self,
        module_name: &str,
        needs_manual_start: bool,
    ) -> String {
        self.es_module_imports.push_str(&format!(
            "import source wasmModule from \"./{module_name}_bg.wasm\";\n"
        ));
        format!(
            "let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());\n\
             let wasm = wasmInstance.exports;\n\
             {start}",
            start = if needs_manual_start {
                "wasm.__wbindgen_start();\n"
            } else {
                ""
            },
        )
    }

    fn generate_bundler_wasm_loading(&self) -> String {
        r#"
        let wasm;
        export function __wbg_set_wasm(val) {
            wasm = val;
        }
        "#
        .to_string()
    }

    fn generate_bundler_start(&self, module_name: &str, needs_manual_start: bool) -> String {
        let mut start = String::new();

        if self.config.typescript {
            // jsr-self-types directive
            start.push_str(&format!(r#"/* @ts-self-types="./{module_name}.d.ts" */"#));
            start.push('\n');
        }

        start.push_str(&format!(
            "\
            import * as wasm from \"./{module_name}_bg.wasm\";\n\
            import {{ __wbg_set_wasm }} from \"./{module_name}_bg.js\";\n\n\
            __wbg_set_wasm(wasm);
            "
        ));

        if needs_manual_start {
            start.push_str("wasm.__wbindgen_start();");
        }

        if !self.export_name_list.is_empty() {
            start.push_str("\nexport {\n");
            if let Some((last, list)) = self.export_name_list.split_last() {
                for name in list {
                    if is_valid_ident(name) {
                        start.push_str(&format!("{name}, "));
                    } else {
                        start.push_str(&format!(r#""{name}", "#));
                    }
                }
                if is_valid_ident(last) {
                    start.push_str(last);
                } else {
                    start.push_str(&format!(r#""{last}""#));
                }
            }
            start.push('\n');
            start.push_str(r#"} from "./"#);
            start.push_str(module_name);
            start.push_str(r#"_bg.js";"#);
            start.push('\n');
        }

        start
    }

    fn generate_web_loading(
        &self,
        needs_manual_start: bool,
        default_module_path: &str,
        has_memory: bool,
    ) -> String {
        let mut init_memviews = String::new();
        for &(num, ref views) in self.memories.values() {
            for kind in views {
                writeln!(
                    init_memviews,
                    // Reset the memory views to null in case `init` gets called multiple times.
                    // Without this, the `length = 0` check would never detect that the view was
                    // outdated.
                    "cached{kind}Memory{num} = null;",
                )
                .unwrap()
            }
        }
        format!(
            "let wasmModule, wasmInstance, wasm;
            function __wbg_finalize_init(instance, module{init_stack_size_arg}) {{
                wasmInstance = instance;
                wasm = instance.exports;
                wasmModule = module;
                {init_memviews}{init_stack_size_check}{start}return wasm;
            }}

            async function __wbg_load(module, imports) {{
                if (typeof Response === 'function' && module instanceof Response) {{
                    if (typeof WebAssembly.instantiateStreaming === 'function') {{
                        try {{
                            return await WebAssembly.instantiateStreaming(module, imports);
                        }} catch (e) {{
                            const validResponse = module.ok && expectedResponseType(module.type);

                            if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {{
                                console.warn(\"`WebAssembly.instantiateStreaming` failed \
                                                because your server does not serve Wasm with \
                                                `application/wasm` MIME type. Falling back to \
                                                `WebAssembly.instantiate` which is slower. Original \
                                                error:\\n\", e);

                            }} else {{ throw e; }}
                        }}
                    }}

                    const bytes = await module.arrayBuffer();
                    return await WebAssembly.instantiate(bytes, imports);
                }} else {{
                    const instance = await WebAssembly.instantiate(module, imports);

                    if (instance instanceof WebAssembly.Instance) {{
                        return {{ instance, module }};
                    }} else {{
                        return instance;
                    }}
                }}

                function expectedResponseType(type) {{
                    switch (type) {{
                        case 'basic': case 'cors': case 'default': return true;
                    }}
                    return false;
                }}
            }}

            function initSync(module{init_memory_arg}) {{
                if (wasm !== undefined) return wasm;

                {init_stack_size}
                if (module !== undefined) {{
                    if (Object.getPrototypeOf(module) === Object.prototype) {{
                        ({{module{init_memory_arg}{init_stack_size_arg}}} = module)
                    }} else {{
                        console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
                    }}
                }}

                const imports = __wbg_get_imports({init_memory_arg_alone});
                if (!(module instanceof WebAssembly.Module)) {{
                    module = new WebAssembly.Module(module);
                }}
                const instance = new WebAssembly.Instance(module, imports);
                return __wbg_finalize_init(instance, module{init_stack_size_arg});
            }}

            async function __wbg_init(module_or_path{init_memory_arg}) {{
                if (wasm !== undefined) return wasm;

                {init_stack_size}
                if (module_or_path !== undefined) {{
                    if (Object.getPrototypeOf(module_or_path) === Object.prototype) {{
                        ({{module_or_path{init_memory_arg}{init_stack_size_arg}}} = module_or_path)
                    }} else {{
                        console.warn('using deprecated parameters for the initialization function; pass a single object instead')
                    }}
                }}

                {default_module_path}
                const imports = __wbg_get_imports({init_memory_arg_alone});

                if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {{
                    module_or_path = fetch(module_or_path);
                }}

                const {{ instance, module }} = await __wbg_load(await module_or_path, imports);

                return __wbg_finalize_init(instance, module{init_stack_size_arg});
            }}
            ",
            start = if needs_manual_start && self.threads_enabled {
                "wasm.__wbindgen_start(thread_stack_size);\n"
            } else if needs_manual_start {
                "wasm.__wbindgen_start();\n"
            } else {
                ""
            },
            init_stack_size = if self.threads_enabled {
                "let thread_stack_size"
            } else {
                ""
            },
            init_stack_size_arg = if self.threads_enabled {
                ", thread_stack_size"
            } else {
                ""
            },
            init_stack_size_check = if self.threads_enabled {
                format!(
                    "if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % {} !== 0)) {{
                        throw new Error('invalid stack size');
                    }}\n\n",
                    threads_xform::PAGE_SIZE,
                )
            } else {
                String::new()
            },
            init_memory_arg = if has_memory {
                ", memory"
            } else {
                ""
            },
            init_memory_arg_alone = if has_memory {
                "memory"
            } else {
                ""
            },
        )
    }

    fn generate_deno_wasm_loading(&self, module_name: &str, needs_manual_start: bool) -> String {
        // Deno added support for .wasm imports in 2024 in https://github.com/denoland/deno/issues/2552.
        // It's fairly recent, so use old-school Wasm loading for broader compat for now.
        format!(
            "const wasmUrl = new URL('{module_name}_bg.wasm', import.meta.url);
            const wasmInstantiated = await WebAssembly.instantiateStreaming(fetch(wasmUrl), __wbg_get_imports());
            const wasmInstance = wasmInstantiated.instance;
            const wasm = wasmInstance.exports;
            {start}",
            start = if needs_manual_start {
                "wasm.__wbindgen_start();\n"
            } else {
                ""
            },
        )
    }

    fn generate_node_esm_wasm_loading(
        &mut self,
        module_name: &str,
        needs_manual_start: bool,
    ) -> String {
        self.es_module_imports
            .push_str("import { readFileSync } from 'node:fs';\n");

        if self.threads_enabled {
            self.es_module_imports
                .push_str("import { isMainThread } from 'node:worker_threads';\n");

            let start_call = if needs_manual_start {
                format!(
                    r#"
    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % {page_size} !== 0)) {{
        throw new Error('invalid stack size');
    }}

    wasm.__wbindgen_start(thread_stack_size);"#,
                    page_size = crate::transforms::threads::PAGE_SIZE,
                )
            } else {
                String::new()
            };

            format!(
                r#"let wasm, wasmInstance, wasmModule, memory;
let __initialized = false;

export function initSync(opts = {{}}) {{
    if (__initialized) return wasm;

    let {{ module, memory: mem, thread_stack_size }} = opts;

    if (module === undefined) {{
        const wasmUrl = new URL('{module_name}_bg.wasm', import.meta.url);
        module = readFileSync(wasmUrl);
    }}

    if (!(module instanceof WebAssembly.Module)) {{
        wasmModule = new WebAssembly.Module(module);
    }} else {{
        wasmModule = module;
    }}

    const wasmImports = __wbg_get_imports(mem);
    wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
    wasm = wasmInstance.exports;
    memory = wasmImports['./{module_name}_bg.js'].memory;
{start_call}
    __initialized = true;
    return wasm;
}}

// Auto-initialize for backwards compatibility (only on main thread)
// Worker threads should call initSync({{ module, memory }}) explicitly
if (isMainThread) {{
    initSync();
}}

export {{ wasm as __wasm, wasmModule as __wbg_wasm_module, memory as __wbg_memory, __wbg_get_imports }};
"#
            )
        } else {
            format!(
                "\
                const wasmUrl = new URL('{module_name}_bg.wasm', import.meta.url);\n\
                const wasmBytes = readFileSync(wasmUrl);\n\
                const wasmModule = new WebAssembly.Module(wasmBytes);\n\
                let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());\n\
                let wasm = wasmInstance.exports;\n\
                {start}",
                start = if needs_manual_start {
                    "wasm.__wbindgen_start();\n"
                } else {
                    ""
                },
            )
        }
    }

    fn generate_node_cjs_wasm_loading(
        &self,
        module_name: &str,
        needs_manual_start: bool,
    ) -> String {
        if self.threads_enabled {
            // For threads: generate initSync that accepts custom memory
            let start_call = if needs_manual_start {
                format!(
                    r#"
    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % {page_size} !== 0)) {{
        throw new Error('invalid stack size');
    }}

    wasm.__wbindgen_start(thread_stack_size);"#,
                    page_size = crate::transforms::threads::PAGE_SIZE,
                )
            } else {
                String::new()
            };

            format!(
                r#"let wasm, wasmInstance, wasmModule, memory;
let __initialized = false;

// Export __wbg_get_imports for workers to use
exports.__wbg_get_imports = __wbg_get_imports;

exports.initSync = function(opts) {{
    if (opts === undefined) opts = {{}};
    if (__initialized) return wasm;

    let module = opts.module;
    let mem = opts.memory;
    let thread_stack_size = opts.thread_stack_size;

    if (module === undefined) {{
        const wasmPath = `${{__dirname}}/{module_name}_bg.wasm`;
        module = require('fs').readFileSync(wasmPath);
    }}

    if (!(module instanceof WebAssembly.Module)) {{
        wasmModule = new WebAssembly.Module(module);
    }} else {{
        wasmModule = module;
    }}

    const wasmImports = __wbg_get_imports(mem);
    wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
    wasm = wasmInstance.exports;
    memory = wasmImports['./{module_name}_bg.js'].memory;
    exports.__wasm = wasm;
    exports.__wbg_wasm_module = wasmModule;
    exports.__wbg_memory = memory;
{start_call}
    __initialized = true;
    return wasm;
}};

// Auto-initialize for backwards compatibility (only on main thread)
// Worker threads should call initSync({{ module, memory }}) explicitly
if (require('worker_threads').isMainThread) {{
    exports.initSync();
}}
"#
            )
        } else {
            format!(
                r#"const wasmPath = `${{__dirname}}/{module_name}_bg.wasm`;
            const wasmBytes = require('fs').readFileSync(wasmPath);
            const wasmModule = new WebAssembly.Module(wasmBytes);
            let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
            let wasm = wasmInstance.exports;
            {start}"#,
                start = if needs_manual_start {
                    "wasm.__wbindgen_start();\n"
                } else {
                    ""
                },
            )
        }
    }

    fn generate_emscripten_wasm_loading(
        &self,
        needs_manual_start: bool,
        classes_and_exports: &str,
    ) -> String {
        let formatted_deps: Vec<String> = self
            .emscripten_global_deps
            .iter()
            .map(|dep| format!("'${dep}'"))
            .collect();

        let start_logic = if needs_manual_start {
            "wasmExports.__wbindgen_start();"
        } else {
            ""
        };

        format!(
            r#"
            addToLibrary({{
                $initBindgen__deps: ['$addOnInit'],
                $initBindgen__postset: 'addOnInit(initBindgen);',
                $initBindgen: () => {{
                    wasm = wasmExports;
                    // Call emscripten's _initialize to run static constructors
                    // (needed for --no-entry builds)
                    if (wasmExports['_initialize']) {{
                        wasmExports['_initialize']();
                    }}
                    {start_logic}
                    {classes_and_exports}
                }}
            }});

            extraLibraryFuncs.push('$initBindgen', '$addOnInit', '$wasm', {formatted_deps});
            "#,
            formatted_deps = formatted_deps.join(", ")
        )
    }

    fn generate_wasm_loading(
        &mut self,
        module_name: &str,
        needs_manual_start: bool,
        has_memory: bool,
        classes_and_exports: &str,
    ) -> String {
        match self.config.mode {
            OutputMode::Module => {
                self.generate_module_wasm_loading(module_name, needs_manual_start)
            }
            OutputMode::Bundler { .. } => self.generate_bundler_wasm_loading(),
            OutputMode::Deno => self.generate_deno_wasm_loading(module_name, needs_manual_start),
            OutputMode::Node { module: true } => {
                self.generate_node_esm_wasm_loading(module_name, needs_manual_start)
            }
            OutputMode::Node { module: false } => {
                self.generate_node_cjs_wasm_loading(module_name, needs_manual_start)
            }
            OutputMode::Web => {
                let default_module_path = if self.config.omit_default_module_path {
                    ""
                } else {
                    &format!(
                        "if (module_or_path === undefined) {{
                            module_or_path = new URL('{module_name}_bg.wasm', import.meta.url);
                        }}"
                    )
                };
                let mut loading =
                    self.generate_web_loading(needs_manual_start, default_module_path, has_memory);

                loading.push_str("\nexport { initSync, __wbg_init as default };");

                loading
            }
            OutputMode::Emscripten => {
                self.generate_emscripten_wasm_loading(needs_manual_start, classes_and_exports)
            }
            OutputMode::NoModules { .. } => {
                let default_module_path = if self.config.omit_default_module_path {
                    ""
                } else {
                    r#"if (module_or_path === undefined && script_src !== undefined) {
                        module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
                    }"#
                };
                self.generate_web_loading(needs_manual_start, default_module_path, has_memory)
            }
        }
    }

    /// Resolve a class name to the key used in `exported_classes`.
    /// The name could be a `rust_name` (direct key) or a `qualified_name`
    /// (from WasmDescribe), which needs to be mapped to the `rust_name`.
    pub(crate) fn resolve_class_name<'b>(&'b self, name: &'b str) -> &'b str {
        if self.exported_classes.contains_key(name) {
            return name;
        }
        if let Some(rust_name) = self.qualified_to_rust_name.get(name) {
            return rust_name;
        }
        name
    }

    fn require_class<'b>(&'b mut self, name: &str) -> &'b mut ExportedClass {
        // Resolve qualified_name to rust_name if needed
        let key = if let Some(rust_name) = self.qualified_to_rust_name.get(name) {
            rust_name.clone()
        } else {
            name.to_string()
        };
        if self
            .exported_classes
            .get(&key)
            .is_none_or(|cls| cls.identifier.is_empty())
        {
            let identifier_name = self
                .exported_classes
                .get(&key)
                .and_then(|cls| cls.qualified_name.clone().or_else(|| cls.js_name.clone()))
                .unwrap_or_else(|| key.clone());
            let identifier = self.generate_identifier(&identifier_name);
            let class = self.exported_classes.entry(key.clone()).or_default();
            class.identifier = identifier.clone();
        }
        self.exported_classes.get_mut(&key).unwrap()
    }

    fn write_classes(&mut self) -> Result<(), Error> {
        let exported_classes = std::mem::take(&mut self.exported_classes);
        for (class, exports) in exported_classes {
            self.write_class(&class, exports)?;
        }
        Ok(())
    }

    fn write_class(&mut self, name: &str, class: ExportedClass) -> Result<(), Error> {
        let identifier = &class.identifier;
        // Use js_name for JS output, falling back to the key name
        let js_name = class.js_name.as_deref().unwrap_or(name);
        // Use qualified_name for wasm symbol references, falling back to js_name
        let qualified = class.qualified_name.as_deref().unwrap_or(js_name);
        let mut dst = format!("class {identifier} {{\n");
        let mut ts_dst = dst.clone();

        if !class.has_constructor {
            // declare the constructor as private to prevent direct instantiation
            ts_dst.push_str("  private constructor();\n");

            if self.config.debug {
                dst.push_str(
                    "\
                        constructor() {
                            throw new Error('cannot invoke `new` directly');
                        }
                    ",
                );
            }
        }

        if class.wrap_needed {
            let (ptr_assignment, register_data) = if self.generate_reinit {
                (
                    "\
                    obj.__wbg_ptr = ptr;
                    Object.defineProperty(obj, '__wbg_inst', { value: __wbg_instance_id, writable: true });
                    ",
                    "{ ptr, instance: __wbg_instance_id }",
                )
            } else {
                ("obj.__wbg_ptr = ptr;", "obj.__wbg_ptr")
            };

            dst.push_str(&format!(
                "\
                static __wrap(ptr) {{
                    const obj = Object.create({identifier}.prototype);
                    {ptr_assignment}
                    {identifier}Finalization.register(obj, {register_data}, obj);
                    return obj;
                }}
                "
            ));
        }

        if class.unwrap_needed {
            dst.push_str(&format!(
                "\
                static __unwrap(jsValue) {{
                    if (!(jsValue instanceof {identifier})) {{
                        return 0;
                    }}
                    return jsValue.__destroy_into_raw();
                }}
                ",
            ));
        }

        let finalization_callback = if self.generate_reinit {
            format!(
                "({{ ptr, instance }}) => {{
                if (instance === __wbg_instance_id) wasm.{}(ptr, 1);
            }}",
                wasm_bindgen_shared::free_function(qualified)
            )
        } else {
            format!(
                "ptr => wasm.{}(ptr, 1)",
                wasm_bindgen_shared::free_function(qualified)
            )
        };

        self.globals.push_str(&format!(
            "const {identifier}Finalization = (typeof FinalizationRegistry === 'undefined')
                ? {{ register: () => {{}}, unregister: () => {{}} }}
                : new FinalizationRegistry({finalization_callback});\n"
        ));

        // If the class is inspectable, generate `toJSON` and `toString`
        // to expose all readable properties of the class. Otherwise,
        // the class shows only the "ptr" property when logged or serialized
        if class.is_inspectable {
            // Creates a `toJSON` method which returns an object of all readable properties
            // This object looks like { a: this.a, b: this.b }
            dst.push_str(&format!(
                "\
                toJSON() {{
                    return {{{}}};
                }}
                toString() {{
                    return JSON.stringify(this);
                }}
                ",
                class
                    .readable_properties
                    .iter()
                    .fold(String::from("\n"), |fields, field_name| {
                        format!("{fields}{field_name}: this.{field_name},\n")
                    })
            ));
            // Also add definitions to the .d.ts file.
            ts_dst.push_str(
                "\
            /**\n*\
            * Return copy of self without private attributes.\n\
            */\n  toJSON(): Object;\n\
            /**\n\
            * Return stringified version of self.\n\
            */\n  toString(): string;\n",
            );

            if self.config.mode.nodejs() {
                // `util.inspect` must be imported in Node.js to define [inspect.custom]
                let module_name = self.import_name(&JsImport {
                    name: JsImportName::Module {
                        module: "util".to_string(),
                        name: "inspect".to_string(),
                    },
                    fields: Vec::new(),
                })?;

                // Node.js supports a custom inspect function to control the
                // output of `console.log` and friends. The constructor is set
                // to display the class name as a typical JavaScript class would
                dst.push_str(&format!(
                    "\
                    [{module_name}.custom]() {{
                        return Object.assign(Object.create({{constructor: this.constructor}}), this.toJSON());
                    }}
                    "
                ));
            }
        }

        let mut free = format!(
            "wasm.{}(ptr, 0)",
            wasm_bindgen_shared::free_function(qualified)
        );
        free = binding::maybe_wrap_export_call(
            &free,
            if self.aux.wrapped_js_tag.is_some() {
                binding::ExportGuard::Abort
            } else if self.generate_reinit {
                binding::ExportGuard::GuardOnly
            } else {
                binding::ExportGuard::None
            },
        );
        dst.push_str(&format!(
            "\
            __destroy_into_raw() {{
                const ptr = this.__wbg_ptr;
                this.__wbg_ptr = 0;
                {identifier}Finalization.unregister(this);
                return ptr;
            }}
            free() {{
                const ptr = this.__destroy_into_raw();
                {free}
            }}
            "
        ));
        ts_dst.push_str("  free(): void;\n");
        ts_dst.push_str("  [Symbol.dispose](): void;\n");
        dst.push_str(&class.contents);
        ts_dst.push_str(&class.typescript);

        self.write_class_field_types(&class, &mut ts_dst);

        dst.push_str("}\n");
        ts_dst.push_str("}\n");

        let ts_definition = if class.generate_typescript {
            if matches!(self.config.mode, OutputMode::Emscripten) {
                self.typescript_emscripten_classes.push_str(&class.comments);
                self.typescript_emscripten_classes.push_str("export ");
                self.typescript_emscripten_classes.push_str(&ts_dst);
                self.typescript_emscripten_classes.push('\n');

                self.typescript
                    .push_str(&format!("{js_name}: typeof {identifier};\n"));

                String::new()
            } else {
                // For hidden classes, add export type statement
                if class.private {
                    ts_dst.push_str(&format!("export type {{ {identifier} }};\n"));
                }
                ts_dst
            }
        } else {
            String::new()
        };

        dst.push_str(&format!(
            "if (Symbol.dispose) {identifier}.prototype[Symbol.dispose] = {identifier}.prototype.free;\n"
        ));

        let ts_comments = if class.generate_typescript {
            Some(class.comments.clone())
        } else {
            None
        };

        define_export(
            &mut self.exports,
            js_name,
            class.js_namespace.as_deref().unwrap_or_default(),
            ExportEntry::Definition(ExportDefinition {
                identifier: class.identifier.clone(),
                comments: Some(class.comments),
                definition: dst,
                ts_definition,
                ts_comments,
                private: class.private,
            }),
        )?;

        Ok(())
    }

    fn write_class_field_types(&mut self, class: &ExportedClass, ts_dst: &mut String) {
        let mut fields: Vec<&FieldInfo> = class.typescript_fields.values().collect();
        fields.sort_by_key(|f| f.order); // make sure we have deterministic output

        for FieldInfo {
            name,
            is_static,
            getter,
            setter,
            ..
        } in fields
        {
            let is_static = if *is_static { "static " } else { "" };

            let write_docs = |ts_dst: &mut String, docs: &str| {
                if docs.is_empty() {
                    return;
                }
                // indent by 2 spaces
                for line in docs.lines() {
                    ts_dst.push_str("  ");
                    ts_dst.push_str(line);
                    ts_dst.push('\n');
                }
            };
            let write_getter = |ts_dst: &mut String, getter: &FieldAccessor| {
                write_docs(ts_dst, &getter.docs);
                ts_dst.push_str("  ");
                ts_dst.push_str(is_static);
                ts_dst.push_str("get ");
                ts_dst.push_str(name);
                ts_dst.push_str("(): ");
                ts_dst.push_str(&getter.ty);
                ts_dst.push_str(";\n");
            };
            let write_setter = |ts_dst: &mut String, setter: &FieldAccessor| {
                write_docs(ts_dst, &setter.docs);
                ts_dst.push_str("  ");
                ts_dst.push_str(is_static);
                ts_dst.push_str("set ");
                ts_dst.push_str(name);
                ts_dst.push_str("(value: ");
                ts_dst.push_str(&setter.ty);
                if setter.is_optional {
                    ts_dst.push_str(" | undefined");
                }
                ts_dst.push_str(");\n");
            };

            match (getter, setter) {
                (None, None) => unreachable!("field without getter or setter"),
                (Some(getter), None) => {
                    // readonly property
                    write_docs(ts_dst, &getter.docs);
                    ts_dst.push_str("  ");
                    ts_dst.push_str(is_static);
                    ts_dst.push_str("readonly ");
                    ts_dst.push_str(name);
                    ts_dst.push_str(if getter.is_optional { "?: " } else { ": " });
                    ts_dst.push_str(&getter.ty);
                    ts_dst.push_str(";\n");
                }
                (None, Some(setter)) => {
                    // write-only property

                    // Note: TypeScript does not handle the types of write-only
                    // properties correctly and will allow reads from write-only
                    // properties. This isn't a wasm-bindgen issue, but a
                    // TypeScript issue.
                    write_setter(ts_dst, setter);
                }
                (Some(getter), Some(setter)) => {
                    // read-write property

                    // Here's the tricky part. The getter and setter might have
                    // different types. Obviously, we can only declare a
                    // property as `foo: T` if both the getter and setter have
                    // the same type `T`. If they don't, we have to declare the
                    // getter and setter separately.

                    // We current generate types for optional arguments and
                    // return values differently. This is why for the field
                    // `foo: Option<T>`, the setter will have type `T` with
                    // `is_optional` set, while the getter has type
                    // `T | undefined`. Because of this difference, we have to
                    // "normalize" the type of the setter.
                    let same_type = if setter.is_optional {
                        getter.ty == setter.ty.clone() + " | undefined"
                    } else {
                        getter.ty == setter.ty
                    };

                    if same_type {
                        // simple property, e.g. foo: T

                        // Prefer the docs of the getter over the setter's
                        let docs = if !getter.docs.is_empty() {
                            &getter.docs
                        } else {
                            &setter.docs
                        };
                        write_docs(ts_dst, docs);
                        ts_dst.push_str("  ");
                        ts_dst.push_str(is_static);
                        ts_dst.push_str(name);
                        ts_dst.push_str(if setter.is_optional { "?: " } else { ": " });
                        ts_dst.push_str(&setter.ty);
                        ts_dst.push_str(";\n");
                    } else {
                        // separate getter and setter
                        write_getter(ts_dst, getter);
                        write_setter(ts_dst, setter);
                    }
                }
            };
        }
    }

    fn write_exports(&mut self) -> Result<(), Error> {
        let exports = std::mem::take(&mut self.exports);
        for (ref export_name, export) in exports {
            match export {
                ExportEntry::Definition(def) => {
                    self.export_name_list.push(export_name.clone());
                    self.export_def(Some(export_name), &def);
                }
                ExportEntry::Namespace(ns) => {
                    let (identifier, existing) = match ns.id {
                        Some(id) => (id, true),
                        None => (self.generate_identifier(export_name), false),
                    };
                    let ns_dst = self.write_namespace(&identifier, &ns.ns, existing)?;
                    let ts_dst = if self.config.typescript {
                        Self::write_namespace_ts(&ns.ns, "")?
                    } else {
                        String::new()
                    };
                    let definition = if !existing {
                        format!("const {identifier} = {{}};\n{ns_dst}")
                    } else {
                        self.global(&ns_dst);
                        "".to_string()
                    };
                    let ts_definition = format!("let {identifier}: {ts_dst};\n");
                    self.export_def(
                        Some(export_name),
                        &ExportDefinition {
                            comments: None,
                            definition,
                            ts_comments: None,
                            ts_definition,
                            identifier,
                            private: false,
                        },
                    );
                }
            }
        }
        Ok(())
    }

    fn write_namespace(
        &mut self,
        name: &str,
        namespace: &BTreeMap<String, ExportEntry>,
        existing: bool,
    ) -> Result<String, Error> {
        let mut output = String::new();
        for (key, entry) in namespace {
            let full_name = if is_valid_ident(key) {
                format!("{name}.{key}")
            } else {
                format!("{name}['{key}']")
            };
            match entry {
                ExportEntry::Namespace(ns) => {
                    output.push_str(&format!(
                        "{full_name} {}= {{}};\n",
                        if existing { "||" } else { "" }
                    ));
                    output.push_str(&self.write_namespace(&full_name, &ns.ns, existing)?);
                }
                ExportEntry::Definition(def) => {
                    self.export_def(None, def);
                    output.push_str(&format!("{full_name} = {};\n", def.identifier));
                }
            }
        }
        Ok(output)
    }

    fn write_namespace_ts(
        namespace: &BTreeMap<String, ExportEntry>,
        indent: &str,
    ) -> Result<String, Error> {
        let mut ts_dst = String::from("{\n");
        for (name, entry) in namespace {
            let indent = format!("  {indent}");
            let entry_ts = match entry {
                ExportEntry::Namespace(ns) => Self::write_namespace_ts(&ns.ns, &indent)?,
                ExportEntry::Definition(def) => format!("typeof {}", def.identifier),
            };
            if is_valid_ident(name) {
                ts_dst.push_str(&format!("{indent}{name}: {entry_ts},\n"));
            } else {
                ts_dst.push_str(&format!("{indent}'{name}': {entry_ts},\n"));
            }
        }
        ts_dst.push_str(&format!("{indent}}}"));
        Ok(ts_dst)
    }

    fn expose_drop_ref(&mut self) {
        // Note that here we check if `idx` shouldn't actually be dropped. This
        // is due to the fact that `JsValue::null()` and friends can be passed
        // by value to JS where we'll automatically call this method. Those
        // constants, however, cannot be dropped. See #1054 for removing this
        // branch.
        //
        // Otherwise the free operation here is pretty simple, just appending to
        // the linked list of heap slots that are free.
        self.expose_global_heap();
        self.expose_global_heap_next();
        self.intrinsic(
            "drop_ref".into(),
            "dropObject".into(),
            format!(
                "
                function dropObject(idx) {{
                    if (idx < {}) return;
                    heap[idx] = heap_next;
                    heap_next = idx;
                }}
                ",
                INITIAL_HEAP_OFFSET + INITIAL_HEAP_VALUES.len(),
            )
            .into(),
        );
    }

    fn expose_global_heap(&mut self) {
        assert!(!self.config.externref);
        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_global_deps.insert("heap".to_string());
            return;
        }
        self.intrinsic(
            "heap".into(),
            None,
            format!(
                "
                let heap = new Array({INITIAL_HEAP_OFFSET}).fill(undefined);
                heap.push({});
                ",
                INITIAL_HEAP_VALUES.join(", ")
            )
            .into(),
        );
    }

    fn expose_global_heap_next(&mut self) {
        self.expose_global_heap();
        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_global_deps.insert("heap_next".to_string());
            return;
        }
        self.intrinsic(
            "heap_next".into(),
            None,
            "\nlet heap_next = heap.length;\n".into(),
        );
    }

    fn expose_get_object(&mut self) {
        // Accessing a heap object is just a simple index operation due to how
        // the stack/heap are laid out.
        self.expose_global_heap();
        self.intrinsic(
            "get_object".into(),
            "getObject".into(),
            "\nfunction getObject(idx) { return heap[idx]; }\n".into(),
        );
    }

    fn expose_not_defined(&mut self) {
        self.intrinsic("not_defined".into(), "notDefined".into(),
            "\nfunction notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }\n".into()
        );
    }

    fn expose_assert_num(&mut self) {
        self.intrinsic("assert_num".into(), "_assertNum".into(),
            "
            function _assertNum(n) {
                if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
            }
            ".into()
        );
    }

    fn expose_assert_bigint(&mut self) {
        self.intrinsic("assert_bigint".into(), "_assertBigInt".into(),
            "
            function _assertBigInt(n) {
                if (typeof(n) !== 'bigint') throw new Error(`expected a bigint argument, found ${typeof(n)}`);
            }
            ".into()
        );
    }

    fn expose_assert_bool(&mut self) {
        self.intrinsic(
            "assert_bool".into(),
            "_assertBoolean".into(),
            "
            function _assertBoolean(n) {
                if (typeof(n) !== 'boolean') {
                    throw new Error(`expected a boolean argument, found ${typeof(n)}`);
                }
            }
            "
            .into(),
        );
    }

    fn expose_wasm_vector_len(&mut self) {
        self.adapter_deps.insert("WASM_VECTOR_LEN".to_string());

        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_global_deps
                .insert("WASM_VECTOR_LEN".to_string());
            self.intrinsic(
                "wasm_vector_len".into(),
                "WASM_VECTOR_LEN".into(),
                "".into(),
            );
        } else {
            self.intrinsic(
                "wasm_vector_len".into(),
                None,
                "\nlet WASM_VECTOR_LEN = 0;\n".into(),
            );
        }
    }

    fn expose_pass_string_to_wasm(&mut self, memory: MemoryId) -> MemView {
        self.expose_wasm_vector_len();

        let mem = self.expose_uint8_memory(memory);
        let ret = MemView {
            name: "passStringToWasm".into(),
            num: mem.num,
        };

        // Ensure the encoder and its polyfills are registered
        self.expose_text_encoder(memory);

        self.intrinsic(
        ret.to_string().into(),
        None,

        {
        let is_emscripten = matches!(self.config.mode, OutputMode::Emscripten);

        let mem_formatted = mem.access(is_emscripten);

        let debug = if self.config.debug {
            "if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);\n"
        } else {
            ""
        };
        let debug_end = if self.config.debug {
            "if (ret.read !== arg.length) throw new Error('failed to pass whole string');"
        } else {
            ""
        };

        // A fast path that directly writes char codes into Wasm memory as long
        // as it finds only ASCII characters.
        //
        // This is much faster for common ASCII strings because it can avoid
        // calling out into C++ TextEncoder code.
        //
        // This might be not very intuitive, but such calls are usually more
        // expensive in mainstream engines than staying in the JS, and
        // charCodeAt on ASCII strings is usually optimised to raw bytes.
        let ptr_coerce = self.coerce_ptr_suffix();
        let encode_as_ascii = format!(
            "\
                if (realloc === undefined) {{
                    const buf = cachedTextEncoder.encode(arg);
                    const ptr = malloc(buf.length, 1){ptr_coerce};
                    {mem_formatted}.subarray(ptr, ptr + buf.length).set(buf);
                    WASM_VECTOR_LEN = buf.length;
                    return ptr;
                }}

                let len = arg.length;
                let ptr = malloc(len, 1){ptr_coerce};

                const mem = {mem_formatted};

                let offset = 0;

                for (; offset < len; offset++) {{
                    const code = arg.charCodeAt(offset);
                    if (code > 0x7F) break;
                    mem[ptr + offset] = code;
                }}
            ",
        );

        let func_decl = format!(
                "
                function {ret}(arg, malloc, realloc) {{
                    {debug}{encode_as_ascii}if (offset !== len) {{
                        if (offset !== 0) {{
                            arg = arg.slice(offset);
                        }}
                        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1){ptr_coerce};
                        const view = {mem_formatted}.subarray(ptr + offset, ptr + len);
                        const ret = cachedTextEncoder.encodeInto(arg, view);
                        {debug_end}
                        offset += ret.written;
                        ptr = realloc(ptr, len, offset, 1){ptr_coerce};
                    }}

                    WASM_VECTOR_LEN = offset;
                    return ptr;
                }}
                ",
            );


        if is_emscripten {
            format!(
                "{func_decl}
                addToLibrary({{
                    '${ret}': {ret},
                    '${ret}__deps': ['$cachedTextEncoder', '$WASM_VECTOR_LEN']
                }});\n"
            ).into()
        } else {
            func_decl.into()
        }
        },
    );

        ret
    }

    fn expose_pass_array8_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint8_memory(memory);
        self.pass_array_to_wasm("passArray8ToWasm", view, 1)
    }

    fn expose_pass_array16_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint16_memory(memory);
        self.pass_array_to_wasm("passArray16ToWasm", view, 2)
    }

    fn expose_pass_array32_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint32_memory(memory);
        self.pass_array_to_wasm("passArray32ToWasm", view, 4)
    }

    fn expose_pass_array64_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint64_memory(memory);
        self.pass_array_to_wasm("passArray64ToWasm", view, 8)
    }

    fn expose_pass_array_f32_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_f32_memory(memory);
        self.pass_array_to_wasm("passArrayF32ToWasm", view, 4)
    }

    fn expose_pass_array_f64_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_f64_memory(memory);
        self.pass_array_to_wasm("passArrayF64ToWasm", view, 8)
    }

    fn expose_pass_array_jsvalue_to_wasm(&mut self, memory: MemoryId) -> MemView {
        let mem = self.expose_dataview_memory(memory);
        let ret = MemView {
            name: "passArrayJsValueToWasm".into(),
            num: mem.num,
        };
        self.expose_wasm_vector_len();
        let mem_formatted: String = mem.access(self.config.mode.emscripten());
        let ptr_coerce = self.coerce_ptr_suffix();
        match (self.aux.externref_table, self.aux.externref_alloc) {
            (Some(table), Some(alloc)) => {
                // TODO: using `addToExternrefTable` goes back and forth between wasm
                // and JS a lot, we should have a bulk operation for this.
                let add = self.expose_add_to_externref_table(table, alloc);

                if self.config.mode.emscripten() {
                    self.adapter_deps.insert(add.to_string());
                }

                self.intrinsic(ret.to_string().into(), None, {
                    format!(
                        "
                        function {ret}(array, malloc) {{
                            const ptr = malloc(array.length * 4, 4){ptr_coerce};
                            for (let i = 0; i < array.length; i++) {{
                                const add = {add}(array[i]);
                                {mem_formatted}.setUint32(ptr + 4 * i, add, true);
                            }}
                            WASM_VECTOR_LEN = array.length;
                            return ptr;
                        }}
                    "
                    )
                    .into()
                });
            }
            _ => {
                self.expose_add_heap_object();
                self.intrinsic(ret.to_string().into(), None, {
                    format!(
                        "
                        function {ret}(array, malloc) {{
                            const ptr = malloc(array.length * 4, 4){ptr_coerce};
                            const mem = {mem_formatted};
                            for (let i = 0; i < array.length; i++) {{
                                mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
                            }}
                            WASM_VECTOR_LEN = array.length;
                            return ptr;
                        }}
                    "
                    )
                    .into()
                });
            }
        }
        ret
    }

    fn pass_array_to_wasm(&mut self, name: &'static str, view: MemView, size: usize) -> MemView {
        let ret = MemView {
            name: name.into(),
            num: view.num,
        };
        self.expose_wasm_vector_len();
        let ptr_coerce = self.coerce_ptr_suffix();
        self.intrinsic(ret.to_string().into(), None, {
            format!(
                "
                function {ret}(arg, malloc) {{
                    const ptr = malloc(arg.length * {size}, {size}){ptr_coerce};
                    {view}().set(arg, ptr / {size});
                    WASM_VECTOR_LEN = arg.length;
                    return ptr;
                }}
                "
            )
            .into()
        });
        ret
    }

    fn expose_text_encoder(&mut self, memory: MemoryId) {
        let is_emscripten = matches!(self.config.mode, OutputMode::Emscripten);
        if is_emscripten {
            self.emscripten_global_deps
                .insert("cachedTextEncoder".to_string());
            self.adapter_deps.insert("cachedTextEncoder".to_string());
        }
        self.intrinsic("text_encoder".into(), "textEncoder".into(), {
            if is_emscripten {
                "".into()
            } else {
                let mut dst = Self::write_text_processor(
                    self.module,
                    memory,
                    "const",
                    "TextEncoder",
                    "()",
                    None,
                    self.config.mode.clone(),
                );

                let polyfill_encode_into = "cachedTextEncoder.encodeInto = function (arg, view) {
                const buf = cachedTextEncoder.encode(arg);
                view.set(buf);
                return {
                    read: arg.length,
                    written: buf.length
                };
            };";

                // `encodeInto` doesn't currently work in any browsers when the memory passed
                // in is backed by a `SharedArrayBuffer`, so force usage of `encode` if
                // a `SharedArrayBuffer` is in use.
                let shared = self.module.memories.get(memory).shared;

                match self.config.encode_into {
                    EncodeInto::Always if !shared => {}
                    EncodeInto::Test if !shared => {
                        dst.push_str(&format!(
                            "
                        if (!('encodeInto' in cachedTextEncoder)) {{
                            {polyfill_encode_into}
                        }}
                        "
                        ));
                    }
                    _ => {
                        // Support audio worklets when able to spawn them.
                        if shared {
                            dst.push_str(&format!(
                                "
                            if (cachedTextEncoder) {{
                                {polyfill_encode_into}
                            }}
                            "
                            ));
                        } else {
                            dst.push_str(polyfill_encode_into);
                        }
                    }
                }

                dst.into()
            }
        });
    }

    fn expose_text_decoder(&mut self, mem: &MemView, memory: MemoryId) {
        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.adapter_deps.insert("cachedTextDecoder".to_string());
            self.emscripten_global_deps
                .insert("cachedTextDecoder".to_string());
        }

        self.intrinsic("text_decoder".into(), "decodeText".into(), {
            // This is needed to workaround a bug in Safari
            // See: https://github.com/wasm-bindgen/wasm-bindgen/issues/1825
            let init = Some("cachedTextDecoder.decode();");

            // `ignoreBOM` is needed so that the BOM will be preserved when sending a string from Rust to JS
            // `fatal` is needed to catch any weird encoding bugs when sending a string from Rust to JS
            let mut dst = if matches!(self.config.mode, OutputMode::Emscripten) {
                String::new()
            } else {
                Self::write_text_processor(
                    self.module,
                    memory,
                    "let",
                    "TextDecoder",
                    "('utf-8', { ignoreBOM: true, fatal: true })",
                    init,
                    self.config.mode.clone(),
                )
            };

            // Typically we try to give a raw view of memory out to `TextDecoder` to
            // avoid copying too much data. If, however, a `SharedArrayBuffer` is
            // being used it looks like that is rejected by `TextDecoder` or
            // otherwise doesn't work with it. When we detect a shared situation we
            // use `slice` which creates a new array instead of `subarray` which
            // creates just a view. That way in shared mode we copy more data but in
            // non-shared mode there's no need to copy the data except for the
            // string itself.
            let text_decoder_decode = {
                let is_shared = self.module.memories.get(memory).shared;
                let method = if is_shared { "slice" } else { "subarray" };
                format!("cachedTextDecoder.decode({}.{method}(ptr, ptr + len))", mem.access(self.config.mode.emscripten()))
            };

            match &self.config.mode {
                OutputMode::Bundler { .. } | OutputMode::Web => {
                    // For targets that can run in a browser, we need a workaround for the fact that
                    // (at least) Safari 16 to 18 has a TextDecoder that can't decode anymore after
                    // processing 2GiB of data. The workaround is that we keep track of how much the
                    // decoder has decoded and just create a new decoder when we're getting close to
                    // the limit.
                    // See MAX_SAFARI_DECODE_BYTES below for link to bug report.

                    // Maximum number of bytes Safari can handle for one TextDecoder is 2GiB (0x80000000 bytes)
                    // but empirically it seems to crash a bit before the end, so we remove 1MiB (0x100000 bytes)
                    // of margin.
                    // Workaround for a bug in Safari.
                    // See https://github.com/rustwasm/wasm-bindgen/issues/4471
                    const MAX_SAFARI_DECODE_BYTES: u32 = 0x80000000 - 0x100000;
                    dst.push_str(&format!(
                        "
                        const MAX_SAFARI_DECODE_BYTES = {MAX_SAFARI_DECODE_BYTES};
                        let numBytesDecoded = 0;
                        function decodeText(ptr, len) {{
                            numBytesDecoded += len;
                            if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {{
                                cachedTextDecoder = new TextDecoder('utf-8', {{ ignoreBOM: true, fatal: true }});
                                cachedTextDecoder.decode();
                                numBytesDecoded = len;
                            }}
                            return {text_decoder_decode};
                        }}
                        ",
                    ));
                }
                _ => {
                    // For any non-browser target (including Emscripten), we can just use the TextDecoder without any workarounds.
                    // For browser-targets, see the workaround for Safari above.
                    dst.push_str(&format!(
                        "
                        function decodeText(ptr, len) {{
                            return {text_decoder_decode};
                        }}
                        ",
                    ));
                }
            }

            dst.into()
        })
    }

    fn write_text_processor(
        module: &walrus::Module,
        memory: MemoryId,
        decl_kind: &str,
        s: &str,
        args: &str,
        init: Option<&str>,
        config_mode: OutputMode,
    ) -> String {
        let mut dst: String = String::new();
        if matches!(config_mode, OutputMode::Emscripten) {
            return dst;
        }
        // Audio worklets don't support `TextDe/Encoder`. When using audio worklets directly,
        // users will have to make sure themselves not to use any corresponding APIs. But
        // when spawning audio worklets, its fine to have `TextDe/Encoder` in a "normal worker"
        // while not using corresponding APIs in the audio worklet itself.
        if module.memories.get(memory).shared {
            dst.push_str(&format!(
                "{decl_kind} cached{s} = (typeof {s} !== 'undefined' ? new {s}{args} : undefined);\n"
            ));

            if let Some(init) = init {
                dst.push_str(&format!("if (cached{s}) {init}\n"));
            }
        } else {
            dst.push_str(&format!("{decl_kind} cached{s} = new {s}{args};\n"));

            if let Some(init) = init {
                dst.push_str(init);
            }
        }
        dst
    }

    fn expose_get_string_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let mem = self.expose_uint8_memory(memory);
        self.expose_text_decoder(&mem, memory);
        let ret = MemView {
            name: "getStringFromWasm".into(),
            num: mem.num,
        };
        let ptr_coerce = self.coerce_ptr_suffix();
        self.intrinsic(ret.to_string().into(), None, {
            format!(
                "
                function {ret}(ptr, len) {{
                    return decodeText(ptr{ptr_coerce}, len);
                }}
                ",
            )
            .into()
        });
        ret
    }

    fn expose_get_cached_string_from_wasm(
        &mut self,
        memory: MemoryId,
        table: Option<TableId>,
    ) -> MemView {
        let get_object = if let Some(table) = table {
            self.expose_get_from_externref_table(table).to_string()
        } else {
            self.expose_get_object();
            "getObject".to_string()
        };
        let get_string = self.expose_get_string_from_wasm(memory);
        let ret = MemView {
            name: "getCachedStringFromWasm".into(),
            num: get_string.num,
        };
        // This has support for both `&str` and `Option<&str>`.
        //
        // If `ptr` is not `0` then we know that it's a `&str` or `Some(&str)`, so we just decode it.
        //
        // If `ptr` is `0` then the `len` is a pointer to the cached `JsValue`, so we return that.
        //
        // If `ptr` and `len` are both `0` then that means it's `None`, in that case we rely upon
        // the fact that `getObject(0)` is guaranteed to be `undefined`.
        self.intrinsic(ret.to_string().into(), None, {
            format!(
                "
                function {ret}(ptr, len) {{
                    if (ptr === 0) {{
                        return {get_object}(len);
                    }} else {{
                        return {get_string}(ptr, len);
                    }}
                }}
                "
            )
            .into()
        });
        ret
    }

    fn expose_get_array_js_value_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let mem: MemView = self.expose_dataview_memory(memory);
        let ret = MemView {
            name: "getArrayJsValueFromWasm".into(),
            num: mem.num,
        };
        match (self.aux.externref_table, self.aux.externref_drop_slice) {
            (Some(table), Some(drop)) => {
                let table = self.export_name_of(table);
                let drop = self.export_name_of(drop);
                let ptr_fixup = self.coerce_ptr_assign("ptr");
                self.intrinsic(ret.to_string().into(), None, {
                    format!(
                        "
                        function {ret}(ptr, len) {{
                            {ptr_fixup}
                            const mem = {mem}();
                            const result = [];
                            for (let i = ptr; i < ptr + 4 * len; i += 4) {{
                                result.push(wasm.{table}.get(mem.getUint32(i, true)));
                            }}
                            wasm.{drop}(ptr, len);
                            return result;
                        }}
                        ",
                    )
                    .into()
                });
            }
            _ => {
                self.expose_take_object();
                let ptr_fixup = self.coerce_ptr_assign("ptr");
                self.intrinsic(ret.to_string().into(), None, {
                    format!(
                        "
                        function {ret}(ptr, len) {{
                            {ptr_fixup}
                            const mem = {mem}();
                            const result = [];
                            for (let i = ptr; i < ptr + 4 * len; i += 4) {{
                                result.push(takeObject(mem.getUint32(i, true)));
                            }}
                            return result;
                        }}
                        ",
                    )
                    .into()
                });
            }
        }
        ret
    }

    /// Like `expose_get_array_js_value_from_wasm` but for borrowed slices.
    /// Uses `getObject` instead of `takeObject` and doesn't drop the heap entries.
    fn expose_get_array_js_value_view_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let mem = self.expose_dataview_memory(memory);
        let ret = MemView {
            name: "getArrayJsValueViewFromWasm".into(),
            num: mem.num,
        };
        match self.aux.externref_table {
            Some(table) => {
                let table = self.export_name_of(table);
                let ptr_fixup = self.coerce_ptr_assign("ptr");
                self.intrinsic(ret.to_string().into(), None, {
                    format!(
                        "
                        function {ret}(ptr, len) {{
                            {ptr_fixup}
                            const mem = {mem}();
                            const result = [];
                            for (let i = ptr; i < ptr + 4 * len; i += 4) {{
                                result.push(wasm.{table}.get(mem.getUint32(i, true)));
                            }}
                            return result;
                        }}
                        ",
                    )
                    .into()
                });
            }
            _ => {
                self.expose_get_object();
                let ptr_fixup = self.coerce_ptr_assign("ptr");
                self.intrinsic(ret.to_string().into(), None, {
                    format!(
                        "
                        function {ret}(ptr, len) {{
                            {ptr_fixup}
                            const mem = {mem}();
                            const result = [];
                            for (let i = ptr; i < ptr + 4 * len; i += 4) {{
                                result.push(getObject(mem.getUint32(i, true)));
                            }}
                            return result;
                        }}
                        ",
                    )
                    .into()
                });
            }
        }
        ret
    }

    fn expose_get_array_i8_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int8_memory(memory);
        self.arrayget("getArrayI8FromWasm", view, 1)
    }

    fn expose_get_array_u8_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint8_memory(memory);
        self.arrayget("getArrayU8FromWasm", view, 1)
    }

    fn expose_get_clamped_array_u8_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_clamped_uint8_memory(memory);
        self.arrayget("getClampedArrayU8FromWasm", view, 1)
    }

    fn expose_get_array_i16_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int16_memory(memory);
        self.arrayget("getArrayI16FromWasm", view, 2)
    }

    fn expose_get_array_u16_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint16_memory(memory);
        self.arrayget("getArrayU16FromWasm", view, 2)
    }

    fn expose_get_array_i32_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int32_memory(memory);
        self.arrayget("getArrayI32FromWasm", view, 4)
    }

    fn expose_get_array_u32_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint32_memory(memory);
        self.arrayget("getArrayU32FromWasm", view, 4)
    }

    fn expose_get_array_i64_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int64_memory(memory);
        self.arrayget("getArrayI64FromWasm", view, 8)
    }

    fn expose_get_array_u64_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint64_memory(memory);
        self.arrayget("getArrayU64FromWasm", view, 8)
    }

    fn expose_get_array_f32_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_f32_memory(memory);
        self.arrayget("getArrayF32FromWasm", view, 4)
    }

    fn expose_get_array_f64_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_f64_memory(memory);
        self.arrayget("getArrayF64FromWasm", view, 8)
    }

    fn arrayget(&mut self, name: &'static str, view: MemView, size: usize) -> MemView {
        let ret = MemView {
            name: name.into(),
            num: view.num,
        };
        let heap_view = view.access(self.config.mode.emscripten());
        let ptr_fixup = self.coerce_ptr_assign("ptr");
        self.intrinsic(name.into(), Some(&ret.to_string()), {
            format!(
                "
                function {ret}(ptr, len) {{
                    {ptr_fixup}
                    return {heap_view}.subarray(ptr / {size}, ptr / {size} + len);
                }}
                ",
            )
            .into()
        });
        ret
    }

    fn expose_int8_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Int8Array", memory)
    }

    fn expose_uint8_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint8Array", memory)
    }

    fn expose_clamped_uint8_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint8ClampedArray", memory)
    }

    fn expose_int16_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Int16Array", memory)
    }

    fn expose_uint16_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint16Array", memory)
    }

    fn expose_int32_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Int32Array", memory)
    }

    fn expose_uint32_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint32Array", memory)
    }

    fn expose_int64_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("BigInt64Array", memory)
    }

    fn expose_uint64_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("BigUint64Array", memory)
    }

    fn expose_f32_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Float32Array", memory)
    }

    fn expose_f64_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Float64Array", memory)
    }

    fn expose_dataview_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("DataView", memory)
    }

    fn memview(&mut self, kind: &'static str, memory: walrus::MemoryId) -> MemView {
        if matches!(self.config.mode, OutputMode::Emscripten) {
            // Emscripten provides its own version of getMemory
            // so don't write out the memory function.
            // See https://emscripten.org/docs/api_reference/preamble.js.html#type-accessors-for-the-memory-model
            // for more details.
            let emscripten_heap: &'static str = match kind {
                "Int8Array" => "HEAP8",
                "Uint8Array" | "Uint8ClampedArray" => "HEAPU8",
                "Int16Array" => "HEAP16",
                "Uint16Array" => "HEAPU16",
                "Int32Array" => "HEAP32",
                "Uint32Array" => "HEAPU32",
                "Float32Array" => "HEAPF32",
                "Float64Array" => "HEAPF64",
                "DataView" => "HEAP_DATA_VIEW",
                _ => "HEAPU8",
            };
            let view = self.memview_memory(emscripten_heap, memory);
            return view;
        }
        let view = self.memview_memory(kind, memory);
        let mem = self.export_name_of(memory);
        self.intrinsic(view.name.to_string().into(), None, {
            let cache = format!("cached{kind}Memory{}", view.num);
            let resized_check = if self.module.memories.get(memory).shared {
                // When it's backed by a `SharedArrayBuffer`, growing the Wasm module's memory
                // doesn't detach old references; instead, it just leaves them pointing to a
                // slice of the up-to-date memory. So in order to check if it's been grown, we
                // have to compare it to the up-to-date buffer.
                format!("{cache}.buffer !== wasm.{mem}.buffer")
            } else if kind == "DataView" {
                // `DataView`s throw when accessing detached memory, including `byteLength`.
                // However this requires JS engine support, so we fallback to comparing the buffer.
                format!("{cache}.buffer.detached === true || ({cache}.buffer.detached === undefined && {cache}.buffer !== wasm.{mem}.buffer)")
            } else {
                // Otherwise, we can do a quicker check of whether the buffer's been detached,
                // which is indicated by a length of 0.
                format!("{cache}.byteLength === 0")
            };
            format!(
                "
                let {cache} = null;
                function {view}() {{
                    if ({cache} === null || {resized_check}) {{
                        {cache} = new {kind}(wasm.{mem}.buffer);
                    }}
                    return {cache};
                }}
                ",
            )
            .into()
        });
        view
    }

    fn memview_memory(&mut self, kind: &'static str, memory: walrus::MemoryId) -> MemView {
        let next = self.memories.len();
        let &mut (num, ref mut kinds) = self
            .memories
            .entry(memory)
            .or_insert((next, Default::default()));
        kinds.insert(kind);
        if matches!(self.config.mode, OutputMode::Emscripten) {
            MemView {
                name: kind.to_string().into(),
                num,
            }
        } else {
            MemView {
                name: format!("get{kind}Memory").into(),
                num,
            }
        }
    }

    fn memview_table(&mut self, name: &'static str, table: walrus::TableId) -> MemView {
        let next = self.table_indices.len();
        let num = *self.table_indices.entry(table).or_insert(next);
        MemView {
            name: name.into(),
            num,
        }
    }

    fn expose_assert_class(&mut self) {
        self.intrinsic("assert_class".into(), "_assertClass".into(), {
            "
            function _assertClass(instance, klass) {
                if (!(instance instanceof klass)) {
                    throw new Error(`expected instance of ${klass.name}`);
                }
            }
            "
            .into()
        });
    }

    fn expose_global_stack_pointer(&mut self) {
        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_global_deps
                .insert("stack_pointer".to_string());
            return;
        }
        self.intrinsic("stack_pointer".into(), None, {
            format!("\nlet stack_pointer = {INITIAL_HEAP_OFFSET};\n").into()
        });
    }

    fn expose_borrowed_objects(&mut self) {
        self.expose_global_heap();
        self.expose_global_stack_pointer();
        // Our `stack_pointer` points to where we should start writing stack
        // objects, and the `stack_pointer` is incremented in a `finally` block
        // after executing this. Once we've reserved stack space we write the
        // value. Eventually underflow will throw an exception, but JS sort of
        // just handles it today...
        self.intrinsic("borrowed_objects".into(), "addBorrowedObject".into(), {
            "
            function addBorrowedObject(obj) {
                if (stack_pointer == 1) throw new Error('out of js stack');
                heap[--stack_pointer] = obj;
                return stack_pointer;
            }
            "
            .into()
        });
    }

    fn expose_take_object(&mut self) {
        self.expose_get_object();
        self.expose_drop_ref();
        self.intrinsic("take_object".into(), "takeObject".into(), {
            "
            function takeObject(idx) {
                const ret = getObject(idx);
                dropObject(idx);
                return ret;
            }
            "
            .into()
        });
    }

    fn expose_add_heap_object(&mut self) {
        self.expose_global_heap();
        self.expose_global_heap_next();

        // Allocating a slot on the heap first goes through the linked list
        // (starting at `heap_next`). Once that linked list is exhausted we'll
        // be pointing beyond the end of the array, at which point we'll reserve
        // one more slot and use that.
        self.intrinsic("add_heap_object".into(), "addHeapObject".into(), {
            format!(
                "
                function addHeapObject(obj) {{
                    if (heap_next === heap.length) heap.push(heap.length + 1);
                    const idx = heap_next;
                    heap_next = heap[idx];
                    {}
                    heap[idx] = obj;
                    return idx;
                }}
                ",
                if self.config.debug {
                    "if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');"
                } else {
                    ""
                }
            )
            .into()
        });
    }

    fn expose_handle_error(&mut self) -> Result<(), Error> {
        if self
            .intrinsics
            .as_ref()
            .unwrap()
            .contains_key("handle_error")
        {
            return Ok(());
        }
        let store = self
            .aux
            .exn_store
            .ok_or_else(|| anyhow!("failed to find `__wbindgen_exn_store` intrinsic"))?;
        let store = self.export_name_of(store);
        match (self.aux.externref_table, self.aux.externref_alloc) {
            (Some(table), Some(alloc)) => {
                let add = self.expose_add_to_externref_table(table, alloc);

                self.intrinsic("handle_error".into(), "handleError".into(), {
                    format!(
                        "
                        function handleError(f, args) {{
                            try {{
                                return f.apply(this, args);
                            }} catch (e) {{
                                const idx = {add}(e);
                                wasm.{store}(idx);
                            }}
                        }}
                        "
                    )
                    .into()
                });
            }
            _ => {
                self.expose_add_heap_object();
                if self.config.mode.emscripten() {
                    self.adapter_deps.insert("addHeapObject".to_string());
                }
                self.intrinsic("handle_error".into(), "handleError".into(), {
                    format!(
                        "
                        function handleError(f, args) {{
                            try {{
                                return f.apply(this, args);
                            }} catch (e) {{
                                wasm.{store}(addHeapObject(e));
                            }}
                        }}
                        "
                    )
                    .into()
                });
            }
        }
        Ok(())
    }

    fn expose_log_error(&mut self) {
        self.intrinsic("log_error".into(), "logError".into(), {
            "
            function logError(f, args) {
                try {
                    return f.apply(this, args);
                } catch (e) {
                    let error = (function () {
                        try {
                            return e instanceof Error \
                                ? `${e.message}\\n\\nStack:\\n${e.stack}` \
                                : e.toString();
                        } catch(_) {
                            return \"<failed to stringify thrown value>\";
                        }
                    }());
                    console.error(\"wasm-bindgen: imported JS function that \
                                    was not marked as `catch` threw an error:\", \
                                    error);
                    throw e;
                }
            }
            "
            .into()
        });
    }

    fn pass_to_wasm_function(&mut self, t: VectorKind, memory: MemoryId) -> MemView {
        match t {
            VectorKind::String => self.expose_pass_string_to_wasm(memory),
            VectorKind::I8 | VectorKind::U8 | VectorKind::ClampedU8 => {
                self.expose_pass_array8_to_wasm(memory)
            }
            VectorKind::U16 | VectorKind::I16 => self.expose_pass_array16_to_wasm(memory),
            VectorKind::I32 | VectorKind::U32 => self.expose_pass_array32_to_wasm(memory),
            VectorKind::I64 | VectorKind::U64 => self.expose_pass_array64_to_wasm(memory),
            VectorKind::F32 => self.expose_pass_array_f32_to_wasm(memory),
            VectorKind::F64 => self.expose_pass_array_f64_to_wasm(memory),
            VectorKind::Externref => self.expose_pass_array_jsvalue_to_wasm(memory),
            VectorKind::NamedExternref(_) => self.expose_pass_array_jsvalue_to_wasm(memory),
        }
    }

    fn expose_get_vector_from_wasm(&mut self, ty: VectorKind, memory: MemoryId) -> MemView {
        match ty {
            VectorKind::String => self.expose_get_string_from_wasm(memory),
            VectorKind::I8 => self.expose_get_array_i8_from_wasm(memory),
            VectorKind::U8 => self.expose_get_array_u8_from_wasm(memory),
            VectorKind::ClampedU8 => self.expose_get_clamped_array_u8_from_wasm(memory),
            VectorKind::I16 => self.expose_get_array_i16_from_wasm(memory),
            VectorKind::U16 => self.expose_get_array_u16_from_wasm(memory),
            VectorKind::I32 => self.expose_get_array_i32_from_wasm(memory),
            VectorKind::U32 => self.expose_get_array_u32_from_wasm(memory),
            VectorKind::I64 => self.expose_get_array_i64_from_wasm(memory),
            VectorKind::U64 => self.expose_get_array_u64_from_wasm(memory),
            VectorKind::F32 => self.expose_get_array_f32_from_wasm(memory),
            VectorKind::F64 => self.expose_get_array_f64_from_wasm(memory),
            VectorKind::Externref => self.expose_get_array_js_value_from_wasm(memory),
            VectorKind::NamedExternref(_) => self.expose_get_array_js_value_from_wasm(memory),
        }
    }

    fn expose_get_inherited_descriptor(&mut self) {
        // It looks like while rare some browsers will move descriptors up the
        // property chain which runs the risk of breaking wasm-bindgen-generated
        // code because we're looking for precise descriptor functions rather
        // than relying on the prototype chain like most "normal JS" projects
        // do.
        //
        // As a result we have a small helper here which will walk the prototype
        // chain looking for a descriptor. For some more information on this see
        // #109
        self.intrinsic(
            "get_inherited_descriptor".into(),
            "GetOwnOrInheritedPropertyDescriptor".into(),
            {
                "
                function GetOwnOrInheritedPropertyDescriptor(obj, id) {
                    while (obj) {
                        let desc = Object.getOwnPropertyDescriptor(obj, id);
                        if (desc) return desc;
                        obj = Object.getPrototypeOf(obj);
                    }
                    return {};
                }
                "
                .into()
            },
        );
    }

    fn expose_is_like_none(&mut self) {
        self.intrinsic("is_like_none".into(), "isLikeNone".into(), {
            "
            function isLikeNone(x) {
                return x === undefined || x === null;
            }
            "
            .into()
        });
    }

    fn expose_assert_non_null(&mut self) {
        self.intrinsic("assert_non_null".into(), "_assertNonNull".into(), {
            "
            function _assertNonNull(n) {
                if (typeof(n) !== 'number' || n === 0) throw new Error(`expected a number argument that is not 0, found ${n}`);
            }
            ".into()
        });
    }

    fn expose_assert_char(&mut self) {
        self.intrinsic("assert_char".into(), "_assertChar".into(), {
            "
            function _assertChar(c) {
                if (typeof(c) === 'number' && (c >= 0x110000 || (c >= 0xD800 && c < 0xE000))) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
            }
            ".into()
        });
    }

    fn expose_make_mut_closure(&mut self) {
        let destroy_state = self.expose_closure_finalization();

        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_global_deps
                .insert("CLOSURE_DTORS".to_string());
        }
        // For mutable closures they can't be invoked recursively.
        // To handle that we swap out the `this.a` pointer with zero
        // while we invoke it. If we finish and the closure wasn't
        // destroyed, then we put back the pointer so a future
        // invocation can succeed.
        self.intrinsic("make_mut_closure".into(), "makeMutClosure".into(), {
            let (state_init, instance_check) = if self.generate_reinit {
                (
                    "const state = { a: arg0, b: arg1, cnt: 1, instance: __wbg_instance_id };",
                    "
                    if (state.instance !== __wbg_instance_id) {
                        throw new Error('Cannot invoke closure from previous WASM instance');
                    }
                    ",
                )
            } else {
                ("const state = { a: arg0, b: arg1, cnt: 1 };", "")
            };
            let mut js = format!(
                "
                function makeMutClosure(arg0, arg1, f) {{
                    {state_init}
                    const real = (...args) => {{
                        {instance_check}
                        // First up with a closure we increment the internal reference
                        // count. This ensures that the Rust closure environment won't
                        // be deallocated while we're invoking it.
                        state.cnt++;
                        const a = state.a;
                        state.a = 0;
                        try {{
                            return f(a, state.b, ...args);
                        }} finally {{
                            state.a = a;
                            real._wbg_cb_unref();
                        }}
                    }};
                    real._wbg_cb_unref = () => {{
                        if (--state.cnt === 0) {{
                            {destroy_state};
                            state.a = 0;
                            CLOSURE_DTORS.unregister(state);
                        }}
                    }};
                    CLOSURE_DTORS.register(real, state, state);
                    return real;
                }}
                "
            );

            if matches!(self.config.mode, OutputMode::Emscripten) {
                js.push_str(
                    r"
                addToLibrary({
                    $makeMutClosure: makeMutClosure,
                    $makeMutClosure__deps: ['$CLOSURE_DTORS']
                });
                ",
                );
            };

            js.into()
        });
    }

    fn expose_make_closure(&mut self) {
        let destroy_state = self.expose_closure_finalization();

        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_global_deps
                .insert("CLOSURE_DTORS".to_string());
        }
        // For shared closures they can be invoked recursively so we
        // just immediately pass through `this.a`. If we end up
        // executing the destructor, however, we clear out the
        // `this.a` pointer to prevent it being used again the
        // future.
        self.intrinsic("make_closure".into(), "makeClosure".into(), {
            let (state_init, instance_check) = if self.generate_reinit {
                (
                    "const state = { a: arg0, b: arg1, cnt: 1, instance: __wbg_instance_id };",
                    "
                    if (state.instance !== __wbg_instance_id) {
                        throw new Error('Cannot invoke closure from previous WASM instance');
                    }
                    ",
                )
            } else {
                ("const state = { a: arg0, b: arg1, cnt: 1 };", "")
            };
            let mut js = format!(
                "
                function makeClosure(arg0, arg1, f) {{
                    {state_init}
                    const real = (...args) => {{
                        {instance_check}
                        // First up with a closure we increment the internal reference
                        // count. This ensures that the Rust closure environment won't
                        // be deallocated while we're invoking it.
                        state.cnt++;
                        try {{
                            return f(state.a, state.b, ...args);
                        }} finally {{
                            real._wbg_cb_unref();
                        }}
                    }};
                    real._wbg_cb_unref = () => {{
                        if (--state.cnt === 0) {{
                            {destroy_state};
                            state.a = 0;
                            CLOSURE_DTORS.unregister(state);
                        }}
                    }};
                    CLOSURE_DTORS.register(real, state, state);
                    return real;
                }}
                "
            );

            if matches!(self.config.mode, OutputMode::Emscripten) {
                js.push_str(
                    "
                addToLibrary({
                    $makeClosure: makeClosure,
                    $makeClosure__deps: ['$CLOSURE_DTORS']
                });
                ",
                );
            };

            js.into()
        });
    }

    /// Exposes the `CLOSURE_DTORS` finalization registry and returns a JS
    /// expression like `wasm.__wbindgen_destroy_closure(state.a, state.b)` for
    /// use in closure destructor code.
    fn expose_closure_finalization(&mut self) -> String {
        let func_id = self
            .aux
            .destroy_closure
            .expect("failed to find `__wbindgen_destroy_closure` intrinsic");
        let dtor = self.export_name_of(func_id);
        let destroy_state = format!("wasm.{dtor}(state.a, state.b)");
        self.intrinsic("closure_finalization".into(), "CLOSURE_DTORS".into(), {
            let prevent_stale = if self.generate_reinit {
                format!(
                    "state => {{
                        if (state.instance === __wbg_instance_id) {{
                            {destroy_state};
                        }}
                    }}"
                )
            } else {
                format!("state => {destroy_state}")
            };

            if matches!(self.config.mode, OutputMode::Emscripten) {
                format!(
                    "addToLibrary({{
                        $CLOSURE_DTORS: {{}},
                        $CLOSURE_DTORS__postset: \"CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined') ? {{ register: () => {{}}, unregister: () => {{}} }} : new FinalizationRegistry({prevent_stale});\"
                    }});\n"
                )
            } else {
                format!(
                    "
                    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
                        ? {{ register: () => {{}}, unregister: () => {{}} }}
                        : new FinalizationRegistry({prevent_stale});
                    "
                )
            }
            .into()
        });

        destroy_state
    }

    fn expose_panic_error(&mut self) {
        self.intrinsic("panic_error".into(), "PanicError".into(), {
            if matches!(self.config.mode, OutputMode::Emscripten) {
                "addToLibrary({
                    $PanicError: function(message) {
                        class PanicError extends Error {}
                        Object.defineProperty(PanicError.prototype, 'name', {
                            value: 'PanicError',
                        });
                        return new PanicError(message);
                    }
                });\n"
                    .into()
            } else {
                "class PanicError extends Error {}
                Object.defineProperty(PanicError.prototype, 'name', {
                    value: PanicError.name,
                });
                "
                .into()
            }
        });
    }

    fn expose_reinit_scheduled(&mut self) {
        self.intrinsic(
            "reinit_scheduled".into(),
            None,
            "
            let __wbg_reinit_scheduled = false;
            "
            .into(),
        );
    }

    /// Emit the `__wbg_reset_state` function and instance-id tracking for the
    /// reinit lifecycle. Called when `schedule_reinit()` is used or
    /// `--experimental-reset-state-function` is passed. The function is private
    /// unless the CLI flag is set.
    fn generate_reinit_wrappers(&mut self) -> Result<(), Error> {
        self.global("let __wbg_instance_id = 0;");

        let mut reset_statements = Vec::new();
        reset_statements.push("__wbg_instance_id++;".to_string());

        for (num, kinds) in self.memories.values() {
            for kind in kinds {
                let memview_name = format!("get{kind}Memory");
                if self.has_intrinsic(memview_name.as_str()) {
                    reset_statements.push(format!("cached{kind}Memory{num} = null;"));
                }
            }
        }

        if self.has_intrinsic("text_decoder") {
            reset_statements.push(
                "if (typeof numBytesDecoded !== 'undefined') numBytesDecoded = 0;".to_string(),
            );
        }

        if self.has_intrinsic("wasm_vector_len") {
            reset_statements.push(
                "if (typeof WASM_VECTOR_LEN !== 'undefined') WASM_VECTOR_LEN = 0;".to_string(),
            );
        }

        if self.has_intrinsic("heap") {
            let mut heap_reset = format!(
                "\
                    if (typeof heap !== 'undefined') {{
                        heap = new Array({INITIAL_HEAP_OFFSET}).fill(undefined);
                        heap = heap.concat([{}]);
                ",
                INITIAL_HEAP_VALUES.join(", ")
            );

            if self.has_intrinsic("heap_next") {
                heap_reset.push_str(
                    "\
                        if (typeof heap_next !== 'undefined')
                            heap_next = heap.length;
                    ",
                );
            }

            if self.has_intrinsic("stack_pointer") {
                heap_reset.push_str(&format!(
                    "\
                        if (typeof stack_pointer !== 'undefined')
                            stack_pointer = {INITIAL_HEAP_OFFSET};
                    "
                ));
            }

            heap_reset.push('}');
            reset_statements.push(heap_reset);
        }

        let has_catch_handler = self.aux.wrapped_js_tag.is_some();
        let abort_reset = if has_catch_handler {
            "\
            __wbg_called_abort = false;
            __wbg_reinit_scheduled = false;"
        } else {
            "__wbg_reinit_scheduled = false;"
        };
        reset_statements.push(format!(
            "{abort_reset}
            wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
            wasm = wasmInstance.exports;
            wasm.__wbindgen_start();
            "
        ));

        let function_body = format!("() {{\n{}}}", reset_statements.join("\n"));

        let identifier = self.generate_identifier("__wbg_reset_state");
        let definition = format!("function {identifier} {function_body}\n");
        define_export(
            &mut self.exports,
            "__wbg_reset_state",
            &[],
            ExportEntry::Definition(ExportDefinition {
                comments: None,
                identifier,
                definition,
                ts_definition: "function __wbg_reset_state(): void;\n".to_string(),
                ts_comments: None,
                // Only publicly exported when --experimental-reset-state-function is passed
                private: !self.config.generate_reset_state,
            }),
        )?;

        Ok(())
    }

    fn global(&mut self, s: &str) {
        let s = s.trim();

        // Ensure a blank line between adjacent items, and ensure everything is
        // terminated with a newline.
        while !self.globals.ends_with("\n\n\n") && !self.globals.ends_with("*/\n") {
            self.globals.push('\n');
        }
        self.globals.push_str(s);
        self.globals.push('\n');
    }

    fn emscripten_library(&mut self, s: &str) {
        let s = s.trim();
        if s.is_empty() {
            return;
        }

        // Check if we need to append a comma separator
        if !self.emscripten_library.is_empty() {
            self.emscripten_library.push_str("\n\n");
        }

        self.emscripten_library.push_str(s);
        self.emscripten_library.push('\n');
    }

    /// Gets the JS identifier for a class, which may be aliased if the original
    /// name conflicts with a JS builtin (e.g., `Array` -> `Array2`).
    pub fn require_class_identifier(&mut self, name: &str) -> String {
        self.require_class(name).identifier.clone()
    }

    fn require_class_wrap(&mut self, name: &str) -> String {
        let cls = self.require_class(name);
        cls.wrap_needed = true;
        cls.identifier.clone()
    }

    fn require_class_unwrap(&mut self, name: &str) -> String {
        let cls = self.require_class(name);
        cls.unwrap_needed = true;
        cls.identifier.clone()
    }

    fn add_module_import(&mut self, module: String, name: &str, actual: &str) {
        let rename = if name == actual {
            None
        } else {
            Some(actual.to_string())
        };
        self.js_imports
            .entry(module)
            .or_default()
            .push((name.to_string(), rename));
    }

    fn import_name(&mut self, import: &JsImport) -> Result<String, Error> {
        if let Some(name) = self.imported_names.get(&import.name) {
            let mut name = name.clone();
            for field in import.fields.iter() {
                name.push('.');
                name.push_str(field);
            }
            return Ok(name.clone());
        }

        let mut name = match &import.name {
            JsImportName::Module { module, name } => {
                let unique_name = self.generate_identifier(name);
                self.add_module_import(module.clone(), name, &unique_name);
                unique_name
            }

            JsImportName::LocalModule { module, name } => {
                let unique_name = self.generate_identifier(name);
                let module = self.config.local_module_name(module);
                self.add_module_import(module, name, &unique_name);
                unique_name
            }

            JsImportName::InlineJs {
                unique_crate_identifier,
                snippet_idx_in_crate,
                name,
            } => {
                let module = self
                    .config
                    .inline_js_module_name(unique_crate_identifier, *snippet_idx_in_crate);
                let unique_name = self.generate_identifier(name);
                self.add_module_import(module, name, &unique_name);
                unique_name
            }

            JsImportName::VendorPrefixed { name, prefixes } => {
                self.imports_post.push_str("const l");
                self.imports_post.push_str(name);
                self.imports_post.push_str(" = ");
                switch(&mut self.imports_post, name, "", prefixes);
                self.imports_post.push_str(";\n");

                fn switch(dst: &mut String, name: &str, prefix: &str, left: &[String]) {
                    dst.push_str("(typeof ");
                    dst.push_str(prefix);
                    dst.push_str(name);
                    dst.push_str(" !== 'undefined' ? ");
                    dst.push_str(prefix);
                    dst.push_str(name);
                    dst.push_str(" : ");
                    if left.is_empty() {
                        dst.push_str("undefined");
                    } else {
                        switch(dst, name, &left[0], &left[1..]);
                    }
                    dst.push(')');
                }
                format!("l{name}")
            }

            JsImportName::Global { name } => {
                // Just register the name for collision detection without modifying it.
                // We should implement separate local / external name handling here in due course
                // and then just use generate_identifier, but for now this retains backwards compat.
                let cnt = self
                    .defined_identifiers
                    .entry(name.to_string())
                    .or_insert(0);
                *cnt += 1;
                if *cnt > 1 {
                    bail!("cannot import `{name}` from two locations");
                }
                name.to_string()
            }
        };
        self.imported_names
            .insert(import.name.clone(), name.clone());

        // After we've got an actual name handle field projections
        for field in import.fields.iter() {
            name.push('.');
            name.push_str(field);
        }
        Ok(name)
    }

    fn import_static(&mut self, import: &JsImport, optional: bool) -> Result<String, Error> {
        let mut name = self.import_name(&JsImport {
            name: import.name.clone(),
            fields: Vec::new(),
        })?;

        // After we've got an actual name handle field projections
        if optional {
            name = format!("typeof {name} === 'undefined' ? null : {name}");

            for field in import.fields.iter() {
                name.push_str("?.");
                name.push_str(field);
            }
        } else {
            for field in import.fields.iter() {
                name.push('.');
                name.push_str(field);
            }
        }

        Ok(name)
    }

    fn expose_get_from_externref_table(&mut self, table: TableId) -> MemView {
        let view = self.memview_table("getFromExternrefTable", table);
        assert!(self.config.externref);
        let table = self.export_name_of(table);
        self.intrinsic(view.to_string().into(), None, {
            format!("\nfunction {view}(idx) {{ return wasm.{table}.get(idx); }}\n").into()
        });
        view
    }

    fn expose_take_from_externref_table(&mut self, table: TableId, drop: FunctionId) -> MemView {
        let view = self.memview_table("takeFromExternrefTable", table);
        assert!(self.config.externref);
        let drop = self.export_name_of(drop);
        let table = self.export_name_of(table);
        self.intrinsic(view.to_string().into(), None, {
            format!(
                "
                function {view}(idx) {{
                    const value = wasm.{table}.get(idx);
                    wasm.{drop}(idx);
                    return value;
                }}
            ",
            )
            .into()
        });

        view
    }

    fn expose_add_to_externref_table(&mut self, table: TableId, alloc: FunctionId) -> MemView {
        let view = self.memview_table("addToExternrefTable", table);
        assert!(self.config.externref);
        let alloc = self.export_name_of(alloc);
        let table = self.export_name_of(table);

        self.intrinsic(view.to_string().into(), None, {
            format!(
                "
                function {view}(obj) {{
                    const idx = wasm.{alloc}();
                    wasm.{table}.set(idx, obj);
                    return idx;
                }}
                ",
            )
            .into()
        });
        view
    }

    pub fn generate(&mut self) -> Result<(), Error> {
        self.prestore_global_import_identifiers()?;

        // Set up qualified-name mappings before processing adapters, so that
        // WasmDescribe lookups can resolve to the right exported class and TS
        // declaration identifier.
        for s in self.aux.structs.iter() {
            self.qualified_to_rust_name
                .insert(s.qualified_name.clone(), s.rust_name.clone());
            let needs_identifier = self
                .exported_classes
                .get(&s.rust_name)
                .is_none_or(|class| class.identifier.is_empty());
            let identifier = if needs_identifier {
                Some(self.generate_identifier(&s.qualified_name))
            } else {
                None
            };
            let class = self
                .exported_classes
                .entry(s.rust_name.clone())
                .or_default();
            class.js_name = Some(s.name.clone());
            class.qualified_name = Some(s.qualified_name.clone());
            if let Some(identifier) = identifier {
                class.identifier = identifier;
            }
            self.qualified_to_identifier
                .insert(s.qualified_name.clone(), class.identifier.clone());
        }
        for e in self.aux.enums.values() {
            self.get_or_create_identifier(&e.qualified_name);
        }

        self.generate_jstag_import();
        self.generate_wrapped_jstag_import()?;
        self.maybe_generate_call_guard()?;

        for (id, adapter, kind) in iter_adapter(self.aux, self.wit, self.module) {
            let instrs = match &adapter.kind {
                AdapterKind::Import { .. } => continue,
                AdapterKind::Local { instructions } => instructions,
            };
            self.generate_adapter(id, adapter, instrs, kind)?;
        }

        // Ensure all imports for reexports are defined
        for (js_import, _) in self.aux.reexports.values() {
            self.import_name(js_import)?;
        }

        let mut pairs = self.aux.export_map.iter().collect::<Vec<_>>();
        pairs.sort_by_key(|(k, _)| *k);
        check_duplicated_getter_and_setter_names(&pairs)?;

        for (_, e) in crate::sorted_iter(&self.aux.enums) {
            self.generate_enum(e)?;
        }
        for (_, e) in crate::sorted_iter(&self.aux.string_enums) {
            self.generate_string_enum(e)?;
        }

        for s in self.aux.structs.iter() {
            self.generate_struct(s)?;
        }

        // Sort custom sections to avoid nondeterminism across CGUs.
        let mut custom_sections: Vec<_> = self.aux.extra_typescript.iter().collect();
        custom_sections.sort_unstable();
        for section in custom_sections {
            self.typescript.push_str(section);
            self.typescript.push_str("\n\n");
        }

        for path in self.aux.package_jsons.iter() {
            self.process_package_json(path)?;
        }

        self.export_destructor();

        // Generate reinit wrappers (__wbg_reset_state + instance-id tracking)
        // when schedule_reinit() is used or --experimental-reset-state-function
        // is set. Must come last to ensure it knows about all other state.
        if self.generate_reinit {
            self.generate_reinit_wrappers()?;
        }

        Ok(())
    }

    fn export_destructor(&mut self) {
        let thread_destroy = match self.aux.thread_destroy {
            Some(id) => id,
            None => return,
        };

        self.export_name_of(thread_destroy);
    }

    /// Generate the import for `WebAssembly.JSTag` if it was used.
    fn generate_jstag_import(&mut self) {
        let Some(js_tag) = self.aux.js_tag else {
            return;
        };

        // Find the import ID for the JSTag
        let import_id = self.module.imports.iter().find_map(|import| {
            let walrus::ImportKind::Tag(tag_id) = import.kind else {
                return None;
            };
            if tag_id == js_tag {
                Some(import.id())
            } else {
                None
            }
        });

        let Some(id) = import_id else {
            return;
        };

        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_library
                .push_str("addToLibrary({\n  __wbindgen_jstag: \"WebAssembly.JSTag\",\n});\n");
        } else {
            self.wasm_import_definitions
                .insert(id, "WebAssembly.JSTag".to_string());
        }
    }

    /// Generate the import for the wrapped JSTag if it was used (abort-reinit mode).
    fn generate_wrapped_jstag_import(&mut self) -> Result<(), Error> {
        let Some(wrapped_js_tag) = self.aux.wrapped_js_tag else {
            return Ok(());
        };

        // Find the import ID for the wrapped JSTag
        let import_id = self.module.imports.iter().find_map(|import| {
            let walrus::ImportKind::Tag(tag_id) = import.kind else {
                return None;
            };
            if tag_id == wrapped_js_tag {
                Some(import.id())
            } else {
                None
            }
        });

        let Some(id) = import_id else {
            return Ok(());
        };

        if matches!(self.config.mode, OutputMode::Emscripten) {
            self.emscripten_library.push_str(
                r#"
addToLibrary({
    __wbindgen_wrapped_jstag: "(globalThis.__wbindgen_wrapped_jstag = new WebAssembly.Tag({ parameters: ['externref'] }))",

    __wbindgen_wrapped_jstag__postset: `
        function __wbg_call_guard() {}

        function __wbg_handle_catch(e) {
            if (e instanceof WebAssembly.Exception && e.is(globalThis.__wbindgen_wrapped_jstag)) {
                throw e.getArg(globalThis.__wbindgen_wrapped_jstag, 0);
            }
            throw e;
        }
    `
});
"#);
            return Ok(());
        }

        // Create a top-level constant for the wrapped tag
        self.global(
            "const __wbindgen_wrapped_jstag = new WebAssembly.Tag({ parameters: ['externref'] });",
        );

        let memory = get_memory(self.module).unwrap();
        let mem_view = self.expose_int32_memory(memory);
        let table = self.export_function_table()?;

        self.global(&format!(
            "
            let __wbg_terminated_addr;
            let __wbg_called_abort = false;
            function __wbg_call_abort_hook() {{
                __wbg_called_abort = true;
                try {{
                    const idx = {mem_view}()[wasm.__abort_handler.value / 4];
                    if (idx) wasm.{table}.get(idx)();
                }} catch(_) {{}}
            }}

            function __wbg_handle_catch(e) {{
                if (e instanceof WebAssembly.Exception && e.is(__wbindgen_wrapped_jstag)) {{
                    throw e.getArg(__wbindgen_wrapped_jstag, 0);
                }}
                {mem_view}()[__wbg_terminated_addr] = 1;
                __wbg_call_abort_hook();
                throw e;
            }}
            "
        ));

        // Use the constant for the import
        self.wasm_import_definitions
            .insert(id, "__wbindgen_wrapped_jstag".to_string());

        Ok(())
    }

    /// Emit a `__wbg_call_guard`, handling hard aborts and reinitialization
    /// - Hard aborts are emitted when we are using js exception tagging.
    /// - For reinitialization, checks __wbg_reinit_scheduled.
    /// - If neither features are required, no call guard is emitted.
    fn maybe_generate_call_guard(&mut self) -> Result<(), Error> {
        // No call guard needed when we dont have hard aborts or reinit
        if self.aux.wrapped_js_tag.is_none() && !self.generate_reinit {
            return Ok(());
        }

        let mut termination_guard = String::from("function __wbg_call_guard() {");
        // Exception handling tags -> hard aborts
        if self.aux.wrapped_js_tag.is_some() {
            let memory = get_memory(self.module)?;
            let mem_view = self.expose_int32_memory(memory);
            termination_guard.push_str(&format!(
                "
                __wbg_terminated_addr ??= wasm.__instance_terminated.value / 4;
                const flag = {mem_view}()[__wbg_terminated_addr];
                if (flag) {{
                    if (!__wbg_called_abort) {{
                        __wbg_call_abort_hook();
                    }}"
            ));
        }
        // reinit guard
        if self.generate_reinit {
            self.expose_reinit_scheduled();
            termination_guard.push_str(
                "
                if (__wbg_reinit_scheduled) {
                    __wbg_reset_state();
                    return;
                }",
            );
        }
        if self.aux.wrapped_js_tag.is_some() {
            termination_guard.push_str("throw new Error('Module terminated');\n");
            if self.generate_reinit {
                termination_guard.push_str(
                    "
                    } else if (__wbg_reinit_scheduled) {
                        __wbg_reset_state();
                    }",
                );
            } else {
                termination_guard.push('}');
            }
        }
        termination_guard.push_str("\n}");
        self.global(&termination_guard);
        Ok(())
    }

    /// Registers import names for all `Global` imports first before we actually
    /// process any adapters.
    ///
    /// `Global` names must be imported as their exact name, so if the same name
    /// from a global is also imported from a module we have to be sure to
    /// import the global first to ensure we don't shadow the actual global
    /// value. Otherwise we have no way of accessing the global value!
    ///
    /// This function will iterate through the import map up-front and generate
    /// a cache entry for each import name which is a `Global`.
    fn prestore_global_import_identifiers(&mut self) -> Result<(), Error> {
        for import in self.aux.import_map.values() {
            let js = match import {
                AuxImport::Value(AuxValue::Bare(js))
                | AuxImport::Value(AuxValue::ClassGetter(js, ..))
                | AuxImport::Value(AuxValue::Getter(js, ..))
                | AuxImport::Value(AuxValue::ClassSetter(js, ..))
                | AuxImport::Value(AuxValue::Setter(js, ..))
                | AuxImport::ValueWithThis(js, ..)
                | AuxImport::Instanceof(js)
                | AuxImport::Static { js, .. }
                | AuxImport::StructuralClassGetter(js, ..)
                | AuxImport::StructuralClassSetter(js, ..)
                | AuxImport::IndexingGetterOfClass(js)
                | AuxImport::IndexingSetterOfClass(js)
                | AuxImport::IndexingDeleterOfClass(js) => js,
                _ => continue,
            };
            if let JsImportName::Global { .. } = js.name {
                self.import_name(js)?;
            }
        }
        Ok(())
    }

    fn generate_adapter(
        &mut self,
        id: AdapterId,
        adapter: &Adapter,
        instrs: &[InstructionData],
        kind: ContextAdapterKind,
    ) -> Result<(), Error> {
        self.adapter_deps.clear();
        let catch = self.aux.imports_with_catch.contains(&id);
        if let ContextAdapterKind::Import(core) = kind {
            if !catch && self.attempt_direct_import(core, instrs)? {
                return Ok(());
            }
        }

        // Construct a JS shim builder, and configure it based on the kind of
        // export that we're generating.
        let mut builder = binding::Builder::new(self);
        builder.log_error(match kind {
            ContextAdapterKind::Export(_) | ContextAdapterKind::Adapter => false,
            ContextAdapterKind::Import(_) => builder.cx.config.debug,
        });
        builder.catch(catch);
        let mut args = &None;
        let mut asyncness = false;
        let mut variadic = false;
        let mut generate_jsdoc = false;
        let mut ret_ty_override = &None;
        let mut ret_desc = &None;
        match kind {
            ContextAdapterKind::Export(export) => {
                args = &export.args;
                asyncness = export.asyncness;
                variadic = export.variadic;
                generate_jsdoc = export.generate_jsdoc;
                ret_ty_override = &export.fn_ret_ty_override;
                ret_desc = &export.fn_ret_desc;
                match &export.kind {
                    AuxExportKind::Function(_) => {}
                    AuxExportKind::FunctionThis(_) => {
                        builder.classless_this();
                    }
                    AuxExportKind::Constructor(class) => builder.constructor(class),
                    AuxExportKind::Method { receiver, .. } => match receiver {
                        AuxReceiverKind::None => {}
                        AuxReceiverKind::Borrowed => builder.method(false),
                        AuxReceiverKind::Owned => builder.method(true),
                    },
                }
            }
            ContextAdapterKind::Import(_) => {}
            ContextAdapterKind::Adapter => {}
        }

        // an internal debug name to help with error messages
        let debug_name = match kind {
            ContextAdapterKind::Import(i) => {
                let i = builder.cx.module.imports.get(i);
                format!("import of `{}::{}`", i.module, i.name)
            }
            ContextAdapterKind::Export(e) => format!("`{}`", e.debug_name),
            ContextAdapterKind::Adapter => format!("adapter {}", id.0),
        };
        // Process the `binding` and generate a bunch of JS/TypeScript/etc.
        let binding::JsFunction {
            ts_sig,
            ts_arg_tys,
            ts_ret_ty,
            ts_refs,
            js_doc,
            ts_doc,
            code,
            might_be_optional_field,
            catch,
            log_error,
        } = builder
            .process(
                adapter,
                instrs,
                args,
                asyncness,
                variadic,
                generate_jsdoc,
                &debug_name,
                ret_ty_override,
                ret_desc,
            )
            .with_context(|| "failed to generates bindings for ".to_string() + &debug_name)?;

        self.typescript_refs.extend(ts_refs);

        // Once we've got all the JS then put it in the right location depending
        // on what's being exported.
        match kind {
            ContextAdapterKind::Export(export) => {
                assert!(!catch);
                assert!(!log_error);

                let ts_sig = export.generate_typescript.then_some(ts_sig.as_str());

                // only include `ts_doc` for format if there were arguments or a return var description
                // this is because if there are no arguments or return var description, `ts_doc`
                // provides no additional value on top of what `ts_sig` already does
                let ts_doc_opts = (ret_desc.is_some()
                    || args
                        .as_ref()
                        .is_some_and(|v| v.iter().any(|arg| arg.desc.is_some())))
                .then_some(ts_doc);

                let js_docs = format_doc_comments(&export.comments, Some(js_doc));
                let ts_docs = format_doc_comments(&export.comments, ts_doc_opts);

                match &export.kind {
                    AuxExportKind::Function(name) | AuxExportKind::FunctionThis(name) => {
                        let qualified_name = wasm_bindgen_shared::qualified_name(
                            export.js_namespace.as_deref(),
                            name,
                        );
                        let identifier = self.get_or_create_identifier(&qualified_name);
                        let (ts_definition, ts_comments) = if let Some(ts_sig) = ts_sig {
                            if matches!(self.config.mode, OutputMode::Emscripten) {
                                // Emscripten: Write "name(args): ret;" directly to buffer
                                self.typescript.push_str(&ts_docs);
                                self.typescript.push_str(&identifier); // No "function" prefix
                                self.typescript.push_str(ts_sig);
                                self.typescript.push_str(";\n");

                                (String::new(), None)
                            } else {
                                let mut typescript = String::new();
                                typescript.push_str("function ");
                                typescript.push_str(&identifier);
                                typescript.push_str(ts_sig);
                                typescript.push_str(";\n");
                                (typescript, Some(ts_docs))
                            }
                        } else {
                            (String::new(), None)
                        };

                        let definition = format!("function {identifier}{code}\n");
                        define_export(
                            &mut self.exports,
                            name,
                            export.js_namespace.as_deref().unwrap_or_default(),
                            ExportEntry::Definition(ExportDefinition {
                                identifier: identifier.clone(),
                                comments: Some(js_docs),
                                definition,
                                ts_definition,
                                ts_comments,
                                private: false,
                            }),
                        )?;
                    }
                    AuxExportKind::Constructor(class) => {
                        let exported = self.require_class(class);

                        if exported.has_constructor {
                            bail!("found duplicate constructor for class `{class}`");
                        }

                        exported.has_constructor = true;
                        exported.push("constructor", "", &js_docs, &code, &ts_docs, ts_sig);
                    }
                    AuxExportKind::Method {
                        class,
                        name,
                        receiver,
                        kind,
                    } => {
                        let exported = self.require_class(class);

                        let mut prefix = String::new();
                        if receiver.is_static() {
                            prefix += "static ";
                        }
                        let ts = match kind {
                            AuxExportedMethodKind::Method => ts_sig,
                            AuxExportedMethodKind::Getter => {
                                prefix += "get ";
                                // For getters and setters, we generate a separate TypeScript definition.
                                if export.generate_typescript {
                                    let location = FieldLocation {
                                        name: name.clone(),
                                        is_static: receiver.is_static(),
                                    };
                                    let accessor = FieldAccessor {
                                        // This is only set to `None` when generating a constructor.
                                        ty: ts_ret_ty.expect("missing return type for getter"),
                                        docs: ts_docs.clone(),
                                        is_optional: false,
                                    };

                                    exported.push_accessor_ts(location, accessor, false);
                                }
                                // Add the getter to the list of readable fields (used to generate `toJSON`)
                                exported.readable_properties.push(name.clone());
                                // Ignore the raw signature.
                                None
                            }
                            AuxExportedMethodKind::Setter => {
                                prefix += "set ";
                                if export.generate_typescript {
                                    let location = FieldLocation {
                                        name: name.clone(),
                                        is_static: receiver.is_static(),
                                    };
                                    let accessor = FieldAccessor {
                                        ty: ts_arg_tys[0].clone(),
                                        docs: ts_docs.clone(),
                                        is_optional: might_be_optional_field,
                                    };

                                    exported.push_accessor_ts(location, accessor, true);
                                }
                                None
                            }
                        };

                        exported.push(name, &prefix, &js_docs, &code, &ts_docs, ts);
                    }
                }
            }
            ContextAdapterKind::Import(core) => {
                // When js_tag is set, all catch imports use wasm catch wrappers
                // instead of the JS handleError wrapper
                let has_wasm_catch = self.aux.js_tag.is_some();

                let code = if catch && !has_wasm_catch {
                    self.expose_handle_error()?;
                    format!("function() {{ return handleError(function {code}, arguments); }}")
                } else if log_error {
                    format!("function() {{ return logError(function {code}, arguments); }}")
                } else {
                    format!("function{code}")
                };

                if matches!(self.config.mode, OutputMode::Emscripten)
                    && !self.adapter_deps.is_empty()
                {
                    self.emscripten_import_deps
                        .insert(core, self.adapter_deps.clone());
                }

                self.wasm_import_definitions.insert(core, code);
            }
            ContextAdapterKind::Adapter => {
                assert!(!catch);
                assert!(!log_error);

                if matches!(self.config.mode, OutputMode::Emscripten) {
                    let code = format!("function {}{code}", &self.export_adapter_name(id));
                    self.export_to_emscripten(&self.export_adapter_name(id), &code);
                } else {
                    self.globals.push_str("function ");
                    self.globals.push_str(&self.export_adapter_name(id));
                    self.globals.push_str(&code);
                    self.globals.push_str("\n\n");
                }
            }
        }
        Ok(())
    }

    /// Returns whether we should disable the logic, in debug mode, to catch an
    /// error, log it, and rethrow it. This is only intended for user-defined
    /// imports, not all imports of everything.
    fn import_never_log_error(&self, import: &AuxImport) -> bool {
        match import {
            // Some intrinsics are intended to exactly throw errors, and in
            // general we shouldn't have exceptions in our intrinsics to debug,
            // so skip these.
            AuxImport::Intrinsic(_) => true,

            // Otherwise assume everything else gets a debug log of errors
            // thrown in debug mode.
            _ => false,
        }
    }

    fn import_never_handle_error(&self, import: &AuxImport) -> bool {
        match import {
            // Some intrinsics are intended to exactly throw errors, and in
            // general we shouldn't have exceptions in our intrinsics to debug,
            // so skip these.
            AuxImport::Intrinsic(Intrinsic::Throw | Intrinsic::Rethrow) => true,

            // Otherwise assume everything else gets a debug log of errors
            // thrown in debug mode.
            _ => false,
        }
    }

    /// Attempts to directly hook up the `id` import in the Wasm module with
    /// the `instrs` specified.
    ///
    /// If this succeeds it returns `Ok(true)`, otherwise if it cannot be
    /// directly imported then `Ok(false)` is returned.
    fn attempt_direct_import(
        &mut self,
        id: ImportId,
        instrs: &[InstructionData],
    ) -> Result<bool, Error> {
        // First up extract the ID of the single called adapter, if any.
        let mut call = None;
        for instr in instrs {
            match instr.instr {
                Instruction::CallAdapter(id) => {
                    if call.is_some() {
                        return Ok(false);
                    } else {
                        call = Some(id);
                    }
                }
                Instruction::CallExport(_) => return Ok(false),
                _ => {}
            }
        }
        let adapter = match call {
            Some(id) => id,
            None => return Ok(false),
        };
        match &self.wit.adapters[&adapter].kind {
            AdapterKind::Import { kind, .. } => match kind {
                AdapterJsImportKind::Normal => {}
                // method/constructors need glue because we either need to
                // invoke them as `new` or we need to invoke them with
                // method-call syntax to get the `this` parameter right.
                AdapterJsImportKind::Method | AdapterJsImportKind::Constructor => return Ok(false),
            },
            // This is an adapter-to-adapter call, so it needs a shim.
            AdapterKind::Local { .. } => return Ok(false),
        }

        // Next up check to make sure that this import is to a bare JS value
        // itself, no extra fluff intended.
        let js = match &self.aux.import_map[&adapter] {
            AuxImport::Value(AuxValue::Bare(js)) => js,
            _ => return Ok(false),
        };

        // Make sure this isn't variadic in any way which means we need some
        // sort of adapter glue.
        if self.aux.imports_with_variadic.contains(&adapter) {
            return Ok(false);
        }

        // Ensure that every single instruction can be represented without JS
        // glue being generated, aka it's covered by the JS ECMAScript bindings
        // for wasm.
        if !self.representable_without_js_glue(instrs) {
            return Ok(false);
        }

        // If there's no field projection happening here and this is a direct
        // import from an ES-looking module, then we can actually just hook this
        // up directly in the Wasm file itself. Note that this is covered in the
        // various output formats as well:
        //
        // * `bundler` - they think Wasm is an ES module anyway
        // * `web` - we're sure to emit more `import` directives during
        //   `gen_init` and we update the import object accordingly.
        // * `nodejs` - the polyfill we have for requiring a Wasm file as a node
        //   module will naturally emit `require` directives for the module
        //   listed on each Wasm import.
        // * `no-modules` - imports aren't allowed here anyway from other
        //   modules and an error is generated.
        if js.fields.is_empty() {
            match &js.name {
                JsImportName::Module { module, name } => {
                    let import = self.module.imports.get_mut(id);
                    import.module.clone_from(module);
                    import.name.clone_from(name);
                    return Ok(true);
                }
                JsImportName::LocalModule { module, name } => {
                    let module = self.config.local_module_name(module);
                    let import = self.module.imports.get_mut(id);
                    import.module = module;
                    import.name.clone_from(name);
                    return Ok(true);
                }
                JsImportName::InlineJs {
                    unique_crate_identifier,
                    snippet_idx_in_crate,
                    name,
                } => {
                    let module = self
                        .config
                        .inline_js_module_name(unique_crate_identifier, *snippet_idx_in_crate);
                    let import = self.module.imports.get_mut(id);
                    import.module = module;
                    import.name.clone_from(name);
                    return Ok(true);
                }

                // Fall through below to requiring a JS shim to create an item
                // that we can import. These are plucked from the global
                // environment so there's no way right now to describe these
                // imports in an ES module-like fashion.
                JsImportName::Global { .. } | JsImportName::VendorPrefixed { .. } => {}
            }
        }

        if let JsImportName::Global { .. } | JsImportName::VendorPrefixed { .. } = js.name {
            // We generally cannot import globals directly, because users can
            // change most globals at runtime.
            //
            // An obvious example of this when the object literally changes
            // (e.g. binding `foo.bar`), but polyfills can also change the
            // object or fundtion.
            //
            // Late binding is another issue. The function might not even be
            // defined when the Wasm module is instantiated. In such cases,
            // there is an observable difference between a direct import and a
            // JS shim calling the function.
            return Ok(false);
        }

        self.expose_not_defined();
        let name = self.import_name(js)?;
        let js = format!("typeof {name} == 'function' ? {name} : notDefined('{name}')",);
        self.wasm_import_definitions.insert(id, js);
        Ok(true)
    }

    fn representable_without_js_glue(&self, instrs: &[InstructionData]) -> bool {
        use Instruction::*;

        let mut last_arg = None;
        let mut saw_call = false;
        for instr in instrs {
            match instr.instr {
                // Fetching arguments is just that, a fetch, so no need for
                // glue. Note though that the arguments must be fetched in order
                // for this to actually work,
                ArgGet(i) => {
                    if saw_call {
                        return false;
                    }
                    match (i, last_arg) {
                        (0, None) => last_arg = Some(0),
                        (n, Some(i)) if n == i + 1 => last_arg = Some(n),
                        _ => return false,
                    }
                }

                // Similarly calling a function is the same as in JS, no glue
                // needed.
                CallAdapter(_) => saw_call = true,

                // Conversions to Wasm integers are always supported since
                // they're coerced into i32/f32/f64 appropriately.
                Int32ToWasm => {}
                Int64ToWasm => {}

                // Converting into a u32 isn't supported because we
                // need to generate glue to change the sign.
                WasmToInt32 { unsigned_32: false } => {}
                // A Wasm `i64` is already a signed JS BigInt, so no glue needed.
                WasmToInt64 { unsigned: false } => {}

                // JS spec automatically coerces boolean values to i32 of 0 or 1
                // depending on true/false
                I32FromBool => {}

                _ => return false,
            }
        }

        true
    }

    /// Generates a JS snippet appropriate for invoking `import`.
    ///
    /// This is generating code for `binding` where `bindings` has more type
    /// information. The `args` array is the list of JS expressions representing
    /// the arguments to pass to JS. Finally `variadic` indicates whether the
    /// last argument is a list to be splatted in a variadic way, and `prelude`
    /// is a location to push some more initialization JS if necessary.
    ///
    /// The returned value here is a JS expression which evaluates to the
    /// purpose of `AuxImport`, which depends on the kind of import.
    fn invoke_import(
        &mut self,
        import: &AuxImport,
        kind: AdapterJsImportKind,
        args: &[String],
        variadic: bool,
        prelude: &mut String,
    ) -> Result<String, Error> {
        let variadic_args = |js_arguments: &[String]| {
            Ok(if !variadic {
                js_arguments.join(", ")
            } else {
                let (last_arg, args) = match js_arguments.split_last() {
                    Some(pair) => pair,
                    None => bail!("a function with no arguments cannot be variadic"),
                };
                if !args.is_empty() {
                    format!("{}, ...{last_arg}", args.join(", "))
                } else {
                    format!("...{last_arg}")
                }
            })
        };

        match import {
            AuxImport::Value(val) => match kind {
                AdapterJsImportKind::Constructor => {
                    let js = match val {
                        AuxValue::Bare(js) => self.import_name(js)?,
                        _ => bail!("invalid import set for constructor"),
                    };
                    Ok(format!("new {js}({})", variadic_args(args)?))
                }
                AdapterJsImportKind::Method => {
                    let descriptor = |anchor: &str, extra: &str, field: &str, which: &str| {
                        format!(
                            "GetOwnOrInheritedPropertyDescriptor({anchor}{extra}, '{field}').{which}"
                        )
                    };
                    let js = match val {
                        AuxValue::Bare(js) => self.import_name(js)?,
                        AuxValue::Getter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, ".prototype", field, "get")
                        }
                        AuxValue::ClassGetter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, "", field, "get")
                        }
                        AuxValue::Setter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, ".prototype", field, "set")
                        }
                        AuxValue::ClassSetter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, "", field, "set")
                        }
                    };
                    Ok(format!("{js}.call({})", variadic_args(args)?))
                }
                AdapterJsImportKind::Normal => {
                    let js = match val {
                        AuxValue::Bare(js) => self.import_name(js)?,
                        _ => bail!("invalid import set for free function"),
                    };
                    Ok(format!("{js}({})", variadic_args(args)?))
                }
            },

            AuxImport::ValueWithThis(class, name) => {
                let class = self.import_name(class)?;
                Ok(format!(
                    "{class}{}({})",
                    property_accessor(name),
                    variadic_args(args)?
                ))
            }

            AuxImport::Instanceof(js) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let js = self.import_name(js)?;
                write!(
                    prelude,
                    "\
                    let result;
                    try {{
                        result = {} instanceof {js};
                    }} catch (_) {{
                        result = false;
                    }}
                    ",
                    args[0],
                )
                .unwrap();
                Ok("result".to_owned())
            }

            AuxImport::Static { js, optional } => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 0);
                self.import_static(js, *optional)
            }

            AuxImport::String(string) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 0);

                let mut escaped = String::with_capacity(string.len());
                string.chars().for_each(|c| match c {
                    '`' | '\\' | '$' => escaped.extend(['\\', c]),
                    _ => escaped.extend([c]),
                });
                Ok(format!("`{escaped}`"))
            }

            AuxImport::Cast { sig_comment } => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);

                writeln!(prelude, "// Cast intrinsic for `{sig_comment}`.")?;
                Ok(args[0].clone())
            }

            AuxImport::StructuralMethod(name) => {
                assert!(kind == AdapterJsImportKind::Normal);
                let (receiver, args) = match args.split_first() {
                    Some(pair) => pair,
                    None => bail!("structural method calls must have at least one argument"),
                };
                Ok(format!(
                    "{receiver}{}({})",
                    property_accessor(name),
                    variadic_args(args)?
                ))
            }

            AuxImport::StructuralGetter(field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(
                    args.len(),
                    1,
                    "The getter '{field}' as more than one args ({n})",
                    n = args.len()
                );
                Ok(format!("{}{}", args[0], property_accessor(field)))
            }

            AuxImport::StructuralClassGetter(class, field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 0);
                let class = self.import_name(class)?;
                Ok(format!("{class}{}", property_accessor(field)))
            }

            AuxImport::StructuralSetter(field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                Ok(format!(
                    "{}{} = {}",
                    args[0],
                    property_accessor(field),
                    args[1]
                ))
            }

            AuxImport::StructuralClassSetter(class, field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let class = self.import_name(class)?;
                Ok(format!("{class}{} = {}", property_accessor(field), args[0]))
            }

            AuxImport::IndexingGetterOfClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let class = self.import_name(class)?;
                Ok(format!("{class}[{}]", args[0]))
            }

            AuxImport::IndexingGetterOfObject => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                Ok(format!("{}[{}]", args[0], args[1]))
            }

            AuxImport::IndexingSetterOfClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                let class = self.import_name(class)?;
                Ok(format!("{class}[{}] = {}", args[0], args[1]))
            }

            AuxImport::IndexingSetterOfObject => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 3);
                Ok(format!("{}[{}] = {}", args[0], args[1], args[2]))
            }

            AuxImport::IndexingDeleterOfClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let class = self.import_name(class)?;
                Ok(format!("delete {class}[{}]", args[0]))
            }

            AuxImport::IndexingDeleterOfObject => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                Ok(format!("delete {}[{}]", args[0], args[1]))
            }

            AuxImport::WrapInExportedClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let identifier = self.require_class_wrap(class);
                Ok(format!("{identifier}.__wrap({})", args[0]))
            }

            AuxImport::Intrinsic(intrinsic) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                self.invoke_intrinsic(intrinsic, args, prelude)
            }

            AuxImport::LinkTo(path, content) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 0);
                if self.config.split_linked_modules {
                    let base = match self.config.mode {
                        OutputMode::Web
                        | OutputMode::Bundler { .. }
                        | OutputMode::Module
                        | OutputMode::Deno
                        | OutputMode::Node { module: true }
                        | OutputMode::Emscripten => "import.meta.url",
                        OutputMode::Node { module: false } => {
                            "require('url').pathToFileURL(__filename)"
                        }
                        OutputMode::NoModules { .. } => {
                            prelude.push_str(
                                "if (script_src === undefined) {
                                    throw new Error(
                                        \"When `--split-linked-modules` is enabled on the `no-modules` target, \
                                          linked modules cannot be used outside of a web page's main thread.\n\
                                          \n\
                                          To fix this, disable `--split-linked-modules`.\"
                                    );
                                 }",
                            );
                            "script_src"
                        }
                    };
                    Ok(format!("new URL('{path}', {base}).toString()"))
                } else if let Some(content) = content {
                    let mut escaped = String::with_capacity(content.len());
                    content.chars().for_each(|c| match c {
                        '`' | '\\' | '$' => escaped.extend(['\\', c]),
                        _ => escaped.extend([c]),
                    });
                    prelude.push_str(&format!("const val = `{escaped}`;\n"));
                    Ok("typeof URL.createObjectURL === 'undefined' ? \
                        \"data:application/javascript,\" + encodeURIComponent(val) : \
                        URL.createObjectURL(new Blob([val], { type: \"text/javascript\" }))"
                        .to_owned())
                } else {
                    Err(anyhow!("wasm-bindgen needs to be invoked with `--split-linked-modules`, because \"{path}\" cannot be embedded.\n\
                        See https://wasm-bindgen.github.io/wasm-bindgen/reference/cli.html#--split-linked-modules for details."))
                }
            }

            AuxImport::UnwrapExportedClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let identifier = self.require_class_unwrap(class);
                Ok(format!("{identifier}.__unwrap({})", args[0]))
            }
        }
    }

    /// Same as `invoke_import` above, except more specialized and only used for
    /// generating the JS expression needed to implement a particular intrinsic.
    fn invoke_intrinsic(
        &mut self,
        intrinsic: &Intrinsic,
        args: &[String],
        prelude: &mut String,
    ) -> Result<String, Error> {
        let expr = match intrinsic {
            Intrinsic::JsvalEq => {
                assert_eq!(args.len(), 2);
                format!("{} === {}", args[0], args[1])
            }

            Intrinsic::JsvalLooseEq => {
                assert_eq!(args.len(), 2);
                format!("{} == {}", args[0], args[1])
            }

            Intrinsic::IsFunction => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'function'", args[0])
            }

            Intrinsic::IsUndefined => {
                assert_eq!(args.len(), 1);
                format!("{} === undefined", args[0])
            }

            Intrinsic::IsNull => {
                assert_eq!(args.len(), 1);
                format!("{} === null", args[0])
            }

            Intrinsic::IsNullOrUndefined => {
                assert_eq!(args.len(), 1);
                format!("{} == null", args[0])
            }

            Intrinsic::IsObject => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const val = {};\n", args[0]));
                "typeof(val) === 'object' && val !== null".to_string()
            }

            Intrinsic::IsSymbol => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'symbol'", args[0])
            }

            Intrinsic::IsString => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'string'", args[0])
            }

            Intrinsic::IsBigInt => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'bigint'", args[0])
            }

            Intrinsic::Typeof => {
                assert_eq!(args.len(), 1);
                format!("typeof {}", args[0])
            }

            Intrinsic::In => {
                assert_eq!(args.len(), 2);
                format!("{} in {}", args[0], args[1])
            }

            Intrinsic::IsFalsy => {
                assert_eq!(args.len(), 1);
                format!("!{}", args[0])
            }

            Intrinsic::TryIntoNumber => {
                assert_eq!(args.len(), 1);
                prelude.push_str("let result;\n");
                writeln!(
                    prelude,
                    "try {{ result = +{} }} catch (e) {{ result = e }}",
                    args[0]
                )
                .unwrap();
                "result".to_owned()
            }

            Intrinsic::Neg => {
                assert_eq!(args.len(), 1);
                format!("-{}", args[0])
            }

            Intrinsic::BitAnd => {
                assert_eq!(args.len(), 2);
                format!("{} & {}", args[0], args[1])
            }

            Intrinsic::BitOr => {
                assert_eq!(args.len(), 2);
                format!("{} | {}", args[0], args[1])
            }

            Intrinsic::BitXor => {
                assert_eq!(args.len(), 2);
                format!("{} ^ {}", args[0], args[1])
            }

            Intrinsic::BitNot => {
                assert_eq!(args.len(), 1);
                format!("~{}", args[0])
            }

            Intrinsic::Shl => {
                assert_eq!(args.len(), 2);
                format!("{} << {}", args[0], args[1])
            }

            Intrinsic::Shr => {
                assert_eq!(args.len(), 2);
                format!("{} >> {}", args[0], args[1])
            }

            Intrinsic::UnsignedShr => {
                assert_eq!(args.len(), 2);
                format!("{} >>> {}", args[0], args[1])
            }

            Intrinsic::Add => {
                assert_eq!(args.len(), 2);
                format!("{} + {}", args[0], args[1])
            }

            Intrinsic::Sub => {
                assert_eq!(args.len(), 2);
                format!("{} - {}", args[0], args[1])
            }

            Intrinsic::Div => {
                assert_eq!(args.len(), 2);
                format!("{} / {}", args[0], args[1])
            }

            Intrinsic::CheckedDiv => {
                assert_eq!(args.len(), 2);
                prelude.push_str("let result;\n");
                writeln!(
                    prelude,
                    "try {{
                        result = {} / {};
                    }} catch (e) {{
                        if (e instanceof RangeError) {{
                            result = e;
                        }} else {{
                            throw e;
                        }}
                    }}",
                    args[0], args[1]
                )
                .unwrap();
                "result".to_owned()
            }

            Intrinsic::Mul => {
                assert_eq!(args.len(), 2);
                format!("{} * {}", args[0], args[1])
            }

            Intrinsic::Rem => {
                assert_eq!(args.len(), 2);
                format!("{} % {}", args[0], args[1])
            }

            Intrinsic::Pow => {
                assert_eq!(args.len(), 2);
                format!("{} ** {}", args[0], args[1])
            }

            Intrinsic::LT => {
                assert_eq!(args.len(), 2);
                format!("{} < {}", args[0], args[1])
            }

            Intrinsic::LE => {
                assert_eq!(args.len(), 2);
                format!("{} <= {}", args[0], args[1])
            }

            Intrinsic::GE => {
                assert_eq!(args.len(), 2);
                format!("{} >= {}", args[0], args[1])
            }

            Intrinsic::GT => {
                assert_eq!(args.len(), 2);
                format!("{} > {}", args[0], args[1])
            }

            Intrinsic::ObjectCloneRef => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::ObjectDropRef => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::NumberGet => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const obj = {};\n", args[0]));
                "typeof(obj) === 'number' ? obj : undefined".to_string()
            }

            Intrinsic::StringGet => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const obj = {};\n", args[0]));
                "typeof(obj) === 'string' ? obj : undefined".to_string()
            }

            Intrinsic::BooleanGet => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const v = {};\n", args[0]));
                "typeof(v) === 'boolean' ? v : undefined".to_string()
            }

            Intrinsic::BigIntGetAsI64 => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const v = {};\n", args[0]));
                "typeof(v) === 'bigint' ? v : undefined".to_string()
            }

            Intrinsic::Throw => {
                assert_eq!(args.len(), 1);
                if self.aux.wrapped_js_tag.is_some() {
                    format!(
                        "throw new WebAssembly.Exception(__wbindgen_wrapped_jstag, [new Error({})])",
                        args[0]
                    )
                } else {
                    format!("throw new Error({})", args[0])
                }
            }

            Intrinsic::Rethrow => {
                assert_eq!(args.len(), 1);
                if self.aux.wrapped_js_tag.is_some() {
                    format!(
                        "throw new WebAssembly.Exception(__wbindgen_wrapped_jstag, [{}])",
                        args[0]
                    )
                } else {
                    format!("throw {}", args[0])
                }
            }

            Intrinsic::Module => {
                assert_eq!(args.len(), 0);

                match self.config.mode {
                    OutputMode::Web
                    | OutputMode::NoModules { .. }
                    | OutputMode::Node { .. }
                    | OutputMode::Module => "wasmModule",
                    _ => bail!(
                        "`wasm_bindgen::module` is currently only supported with \
                         `--target no-modules`, `--target web`, `--target module`, \
                         `--target nodejs` and `--target experimental-nodejs-module`"
                    ),
                }
                .to_string()
            }

            Intrinsic::Instance => {
                assert_eq!(args.len(), 0);
                match self.config.mode {
                    OutputMode::Web
                    | OutputMode::NoModules { .. }
                    | OutputMode::Node { .. }
                    | OutputMode::Module
                    | OutputMode::Deno => "wasmInstance",
                    _ => bail!(
                        "`wasm_bindgen::instance` is currently only supported with \
                         `--target no-modules`, `--target web`, `--target deno`, \
                         `--target module`, `--target nodejs` and \
                         `--target experimental-nodejs-module`"
                    ),
                }
                .to_string()
            }

            Intrinsic::Exports => {
                assert_eq!(args.len(), 0);
                "wasm".to_string()
            }

            Intrinsic::Memory => {
                assert_eq!(args.len(), 0);
                let mut memories = self.module.memories.iter();
                let memory = memories
                    .next()
                    .ok_or_else(|| anyhow!("no memory found to return in memory intrinsic"))?
                    .id();
                if memories.next().is_some() {
                    bail!(
                        "multiple memories found, unsure which to return \
                         from memory intrinsic"
                    );
                }
                drop(memories);
                match self.config.mode {
                    OutputMode::Emscripten => "HEAPU8".to_string(),
                    _ => format!("wasm.{}", self.export_name_of(memory)),
                }
            }

            Intrinsic::FunctionTable => {
                assert_eq!(args.len(), 0);
                let name = self.export_function_table()?;
                format!("wasm.{name}")
            }

            Intrinsic::DebugString => {
                assert_eq!(args.len(), 1);
                self.expose_debug_string();
                self.adapter_deps.insert("debugString".to_string());
                format!("debugString({})", args[0])
            }

            Intrinsic::CopyToTypedArray => {
                assert_eq!(args.len(), 2);
                format!(
                    "new Uint8Array({dst}.buffer, {dst}.byteOffset, {dst}.byteLength).set({src})",
                    src = args[0],
                    dst = args[1]
                )
            }

            Intrinsic::ExternrefHeapLiveCount => {
                assert_eq!(args.len(), 0);
                self.expose_global_heap();
                prelude.push_str(
                    "
                        let free_count = 0;
                        let next = heap_next;
                        while (next < heap.length) {
                            free_count += 1;
                            next = heap[next];
                        }
                    ",
                );
                format!(
                    "heap.length - free_count - {INITIAL_HEAP_OFFSET} - {}",
                    INITIAL_HEAP_VALUES.len(),
                )
            }

            Intrinsic::InitExternrefTable => {
                let table = self
                    .aux
                    .externref_table
                    .ok_or_else(|| anyhow!("must enable externref to use externref intrinsic"))?;
                let mut base = "\n".to_string();
                let name = self.export_name_of(table);

                if matches!(self.config.mode, OutputMode::Emscripten) {
                    base.push_str(&format!("const table = wasmExports['{name}'];\n"));
                } else {
                    base.push_str(&format!("const table = wasm.{name};\n"));
                }

                // Grow the table to insert our initial values, and then also
                // set the 0th slot to `undefined` since that's what we've
                // historically used for our ABI which is that the index of 0
                // returns `undefined` for types like `None` going out.
                let mut base = format!(
                    "
                      const table = wasm.{name};
                      const offset = table.grow({});
                      table.set(0, undefined);
                    ",
                    INITIAL_HEAP_VALUES.len(),
                );
                for (i, value) in INITIAL_HEAP_VALUES.iter().enumerate() {
                    base.push_str(&format!(
                        "{}table.set(offset + {i}, {value})",
                        if i > 0 { ";\n" } else { "" }
                    ));
                }
                base
            }
            Intrinsic::PanicError => {
                assert_eq!(args.len(), 1);
                self.expose_panic_error();
                format!("new PanicError({})", args[0])
            }
            Intrinsic::Reinit => {
                assert_eq!(args.len(), 0);
                "__wbg_reinit_scheduled = true".to_string()
            }
        };
        Ok(expr)
    }

    fn generate_enum(&mut self, enum_: &AuxEnum) -> Result<(), Error> {
        let identifier = self.get_or_create_identifier(&enum_.qualified_name);
        let ts_comments = format_doc_comments(&enum_.comments, None);
        let mut typescript = String::new();
        if enum_.generate_typescript {
            typescript.push_str(&format!("enum {identifier} {{"));
        }

        let mut variants = String::new();
        for (name, value, comments) in enum_.variants.iter() {
            let variant_docs = if comments.is_empty() {
                String::new()
            } else {
                format_doc_comments(comments, None)
            };
            variants.push_str(&variant_docs);
            variants.push_str(&format!("{name}: {value}, "));
            variants.push_str(&format!("\"{value}\": \"{name}\",\n"));
            if enum_.generate_typescript {
                typescript.push('\n');
                if !variant_docs.is_empty() {
                    for line in variant_docs.lines() {
                        typescript.push_str("  ");
                        typescript.push_str(line);
                        typescript.push('\n');
                    }
                }
                typescript.push_str(&format!("  {name} = {value},"));
            }
        }
        if enum_.generate_typescript {
            typescript.push_str("\n}\n");
        }

        // add an `@enum {1 | 2 | 3}` to ensure that enums type-check even without .d.ts
        let mut at_enum = "@enum {".to_string();
        for (i, (_, value, _)) in enum_.variants.iter().enumerate() {
            if i != 0 {
                at_enum.push_str(" | ");
            }
            at_enum.push_str(&value.to_string());
        }
        at_enum.push('}');
        let docs = format_doc_comments(&enum_.comments, Some(at_enum));

        let definition = format!("const {identifier} = Object.freeze({{\n{variants}}});\n");

        define_export(
            &mut self.exports,
            &enum_.name,
            enum_.js_namespace.as_deref().unwrap_or_default(),
            ExportEntry::Definition(ExportDefinition {
                identifier: identifier.clone(),
                comments: Some(docs),
                definition,
                ts_definition: typescript,
                ts_comments: Some(ts_comments),
                private: enum_.private,
            }),
        )?;

        Ok(())
    }

    fn generate_string_enum(&mut self, string_enum: &AuxStringEnum) -> Result<(), Error> {
        let variants: Vec<_> = string_enum
            .variant_values
            .iter()
            .map(|v| format!("\"{v}\""))
            .collect();

        if string_enum.generate_typescript
            && self
                .typescript_refs
                .contains(&TsReference::StringEnum(string_enum.name.clone()))
        {
            let docs = format_doc_comments(&string_enum.comments, None);
            let type_expr = if variants.is_empty() {
                "never".to_string()
            } else {
                variants.join(" | ")
            };

            self.typescript.push_str(&docs);
            self.typescript.push_str("\ntype ");
            self.typescript.push_str(&string_enum.name);
            self.typescript.push_str(" = ");
            self.typescript.push_str(&type_expr);
            self.typescript.push_str(";\n");
        }

        if self.used_string_enums.contains(&string_enum.name) {
            // only generate the internal string enum array if it's actually used
            self.global(&format!(
                "\nconst __wbindgen_enum_{name} = [{values}];\n",
                name = string_enum.name,
                values = variants.join(", ")
            ));
        }

        Ok(())
    }

    fn expose_string_enum(&mut self, string_enum_name: &str) {
        self.used_string_enums.insert(string_enum_name.to_string());
    }

    fn generate_struct(&mut self, struct_: &AuxStruct) -> Result<(), Error> {
        let class = self.require_class(&struct_.rust_name);
        class.comments = format_doc_comments(&struct_.comments, None);
        class.is_inspectable = struct_.is_inspectable;
        class.generate_typescript = struct_.generate_typescript;
        class.private = struct_.private;
        class.js_namespace = struct_.js_namespace.as_ref().map(|ns| ns.to_vec());
        class.js_name = Some(struct_.name.clone());
        class.qualified_name = Some(struct_.qualified_name.clone());
        Ok(())
    }

    fn process_package_json(&mut self, path: &Path) -> Result<(), Error> {
        if self.config.mode.no_modules() {
            bail!(
                "NPM dependencies have been specified in `{}` but \
                 this is incompatible with the `no-modules` target",
                path.display(),
            );
        }

        let contents =
            fs::read_to_string(path).context(format!("failed to read `{}`", path.display()))?;
        let json: serde_json::Value = serde_json::from_str(&contents)?;
        let object = match json.as_object() {
            Some(s) => s,
            None => bail!(
                "expected `package.json` to have an JSON object in `{}`",
                path.display()
            ),
        };
        let iter = object.iter();
        let mut value = None;
        for (key, v) in iter {
            if key == "dependencies" {
                value = Some(v);
                break;
            }
        }
        let value = if let Some(value) = value {
            value
        } else {
            return Ok(());
        };
        let value = match value.as_object() {
            Some(s) => s,
            None => bail!(
                "expected `dependencies` to be a JSON object in `{}`",
                path.display()
            ),
        };

        for (name, value) in value.iter() {
            let value = match value.as_str() {
                Some(s) => s,
                None => bail!(
                    "keys in `dependencies` are expected to be strings in `{}`",
                    path.display()
                ),
            };
            if let Some((prev, _prev_version)) = self.npm_dependencies.get(name) {
                bail!(
                    "dependency on NPM package `{name}` specified in two `package.json` files, \
                     which at the time is not allowed:\n  * {}\n  * {}",
                    path.display(),
                    prev.display(),
                )
            }

            self.npm_dependencies
                .insert(name.to_string(), (path.to_path_buf(), value.to_string()));
        }

        Ok(())
    }

    fn expose_debug_string(&mut self) {
        self.intrinsic("debug_string".into(), "debugString".into(), {
            "
            function debugString(val) {
                // primitive types
                const type = typeof val;
                if (type == 'number' || type == 'boolean' || val == null) {
                    return  `${val}`;
                }
                if (type == 'string') {
                    return `\"${val}\"`;
                }
                if (type == 'symbol') {
                    const description = val.description;
                    if (description == null) {
                        return 'Symbol';
                    } else {
                        return `Symbol(${description})`;
                    }
                }
                if (type == 'function') {
                    const name = val.name;
                    if (typeof name == 'string' && name.length > 0) {
                        return `Function(${name})`;
                    } else {
                        return 'Function';
                    }
                }
                // objects
                if (Array.isArray(val)) {
                    const length = val.length;
                    let debug = '[';
                    if (length > 0) {
                        debug += debugString(val[0]);
                    }
                    for(let i = 1; i < length; i++) {
                        debug += ', ' + debugString(val[i]);
                    }
                    debug += ']';
                    return debug;
                }
                // Test for built-in
                const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));
                let className;
                if (builtInMatches && builtInMatches.length > 1) {
                    className = builtInMatches[1];
                } else {
                    // Failed to match the standard '[object ClassName]'
                    return toString.call(val);
                }
                if (className == 'Object') {
                    // we're a user defined class or Object
                    // JSON.stringify avoids problems with cycles, and is generally much
                    // easier than looping through ownProperties of `val`.
                    try {
                        return 'Object(' + JSON.stringify(val) + ')';
                    } catch (_) {
                        return 'Object';
                    }
                }
                // errors
                if (val instanceof Error) {
                    return `${val.name}: ${val.message}\\n${val.stack}`;
                }
                // TODO we could test for more things here, like `Set`s and `Map`s.
                return className;
            }
            "
            .into()
        });
    }

    fn export_function_table(&mut self) -> Result<String, Error> {
        match self.aux.function_table {
            Some(id) => Ok(self.export_name_of(id)),
            None => bail!("no function table found in module"),
        }
    }

    fn export_name_of(&mut self, id: impl Into<walrus::ExportItem>) -> String {
        use walrus::ExportItem::*;

        let id = id.into();
        let export = self.module.exports.iter().find(|e| match (e.item, id) {
            (Function(a), Function(b)) => a == b,
            (Table(a), Table(b)) => a == b,
            (Memory(a), Memory(b)) => a == b,
            (Global(a), Global(b)) => a == b,
            (Tag(a), Tag(b)) => a == b,
            _ => false,
        });
        if let Some(export) = export {
            return export.name.clone();
        }
        let name = match id {
            Function(f) => self.module.funcs.get(f).name.as_deref(),
            Table(table) => self.module.tables.get(table).name.as_deref(),
            Memory(_) => Some("memory"),
            Global(g) => self.module.globals.get(g).name.as_deref(),
            Tag(t) => self.module.tags.get(t).name.as_deref(),
        }
        .unwrap_or("__wbindgen_export");
        let name = self.generate_identifier(&to_valid_ident(name));
        self.module.exports.add(&name, id);
        name
    }

    fn export_adapter_name(&self, adapter_id: AdapterId) -> String {
        let (export_id, _) = *self
            .wit
            .exports
            .iter()
            .find(|(_, id)| *id == adapter_id)
            .expect("could not find an export adapter");

        self.module.exports.get(export_id).name.clone()
    }

    fn generate_identifier(&mut self, name: &str) -> String {
        Self::generate_identifier_with(&mut self.defined_identifiers, name)
    }

    /// Returns the identifier for a qualified name, reusing a previously
    /// registered one or generating (and storing) a new one.
    fn get_or_create_identifier(&mut self, qualified_name: &str) -> String {
        if let Some(id) = self.qualified_to_identifier.get(qualified_name) {
            return id.clone();
        }
        let id = self.generate_identifier(qualified_name);
        self.qualified_to_identifier
            .insert(qualified_name.to_string(), id.clone());
        id
    }

    fn generate_identifier_with(identifiers: &mut HashMap<String, usize>, name: &str) -> String {
        let name = to_valid_ident(name);
        let cnt = identifiers.entry(name.to_string()).or_insert(0);
        *cnt += 1;
        let mut suffix = *cnt;
        if suffix == 1 {
            name.to_string()
        } else {
            // Keep incrementing until we find an identifier that isn't already taken
            let mut candidate = format!("{name}{suffix}");
            while identifiers.contains_key(&candidate) {
                suffix += 1;
                candidate = format!("{name}{suffix}");
            }
            // Update the counter and reserve the candidate
            *identifiers.get_mut(&*name).unwrap() = suffix;
            identifiers.insert(candidate.clone(), 1);
            candidate
        }
    }

    fn inject_stack_pointer_shim(&mut self) -> Result<(), Error> {
        if self.stack_pointer_shim_injected {
            return Ok(());
        }
        let stack_pointer = match self.aux.stack_pointer {
            Some(s) => s,
            // In theory this shouldn't happen since malloc is included in
            // most Wasm binaries (and may be gc'd out) and that almost
            // always pulls in a stack pointer. We can try to synthesize
            // something here later if necessary.
            None => bail!("failed to find stack pointer"),
        };

        use walrus::ir::*;

        // Shim ABI matches `WasmWord`: `f64` on memory64, `i32` on wasm32.
        // The `__stack_pointer` global is still `i64` on memory64, so we
        // convert at the boundary.
        let val_type = if self.memory64 {
            ValType::F64
        } else {
            ValType::I32
        };

        let mut builder =
            walrus::FunctionBuilder::new(&mut self.module.types, &[val_type], &[val_type]);
        builder.name("__wbindgen_add_to_stack_pointer".to_string());

        let mut body = builder.func_body();
        let arg = self.module.locals.add(val_type);

        // Create a shim function that mutate the stack pointer
        // to avoid exporting a mutable global.
        if self.memory64 {
            body.local_get(arg)
                .unop(UnaryOp::I64TruncSSatF64)
                .global_get(stack_pointer)
                .binop(BinaryOp::I64Add)
                .global_set(stack_pointer)
                .global_get(stack_pointer)
                .unop(UnaryOp::F64ConvertSI64);
        } else {
            body.local_get(arg)
                .global_get(stack_pointer)
                .binop(BinaryOp::I32Add)
                .global_set(stack_pointer)
                .global_get(stack_pointer);
        }

        let add_to_stack_pointer_func = builder.finish(vec![arg], &mut self.module.funcs);

        self.module
            .exports
            .add("__wbindgen_add_to_stack_pointer", add_to_stack_pointer_func);

        self.stack_pointer_shim_injected = true;

        Ok(())
    }
}

/// A categorization of adapters for the purpose of code generation.
///
/// This is different from [`AdapterKind`] and is only used internally in the
/// code generation process.
enum ContextAdapterKind<'a> {
    /// An exported function, method, constrctor, or getter/setter.
    Export(&'a AuxExport),
    /// An imported function or intrinsic.
    Import(walrus::ImportId),
    Adapter,
}
impl<'a> ContextAdapterKind<'a> {
    fn get(id: AdapterId, aux: &'a WasmBindgenAux, wit: &'a NonstandardWitSection) -> Self {
        match aux.export_map.get(&id) {
            Some(export) => ContextAdapterKind::Export(export),
            None => {
                let core = wit.implements.iter().find(|pair| pair.2 == id);
                match core {
                    Some((core, _, _)) => ContextAdapterKind::Import(*core),
                    None => ContextAdapterKind::Adapter,
                }
            }
        }
    }
}

/// Iterate over the adapters in a deterministic order.
fn iter_adapter<'a>(
    aux: &'a WasmBindgenAux,
    wit: &'a NonstandardWitSection,
    module: &Module,
) -> Vec<(AdapterId, &'a Adapter, ContextAdapterKind<'a>)> {
    let mut adapters: Vec<_> = wit
        .adapters
        .iter()
        .map(|(id, adapter)| {
            // we need the kind of the adapter to properly sort them
            let kind = ContextAdapterKind::get(*id, aux, wit);
            (*id, adapter, kind)
        })
        .collect();

    // Sort adapters by kind first (imports, exports, adapters), then by name within each kind
    // to ensure deterministic ordering of generated code.
    adapters.sort_by(|(a_id, _, a), (b_id, _, b)| {
        fn get_kind_order(kind: &ContextAdapterKind) -> u8 {
            match kind {
                ContextAdapterKind::Import(_) => 0,
                ContextAdapterKind::Export(_) => 1,
                ContextAdapterKind::Adapter => 2,
            }
        }

        match (a, b) {
            (ContextAdapterKind::Import(a), ContextAdapterKind::Import(b)) => {
                let a = module.imports.get(*a);
                let b = module.imports.get(*b);
                a.name.cmp(&b.name)
            }
            (ContextAdapterKind::Export(a), ContextAdapterKind::Export(b)) => {
                // Sort exports by debug_name to ensure deterministic identifier generation
                // when multiple exports have the same JS name (e.g., due to js_name attribute).
                a.debug_name.cmp(&b.debug_name)
            }
            (ContextAdapterKind::Adapter, ContextAdapterKind::Adapter) => {
                let export_a = wit.exports.iter().find(|(_, id)| id == a_id);
                let export_b = wit.exports.iter().find(|(_, id)| id == b_id);

                match (export_a, export_b) {
                    (Some((export_id_a, _)), Some((export_id_b, _))) => {
                        let export_a = module.exports.get(*export_id_a);
                        let export_b = module.exports.get(*export_id_b);
                        // We cannot sort mangled names as they are machine-dependent, therefore we instead
                        // sort by function signature.
                        let get_type_key = |export: &walrus::Export| -> String {
                            let func_id = match export.item {
                                walrus::ExportItem::Function(id) => id,
                                _ => return String::new(),
                            };
                            let ty_id = module.funcs.get(func_id).ty();
                            let ty = module.types.get(ty_id);
                            // Create a string representation of the type signature
                            format!("{:?}-{:?}", ty.params(), ty.results())
                        };

                        get_type_key(export_b).cmp(&get_type_key(export_a))
                    }
                    (Some(_), None) => std::cmp::Ordering::Less, // Exported adapters come first
                    (None, Some(_)) => std::cmp::Ordering::Greater, // Exported adapters come first
                    (None, None) => a_id.cmp(b_id), // Both without exports, compare by ID
                }
            }
            _ => get_kind_order(a).cmp(&get_kind_order(b)),
        }
    });

    adapters
}

/// Iterate over the imports in a deterministic order.
fn iter_by_import<'a, T>(
    map: &'a HashMap<ImportId, T>,
    module: &Module,
) -> Vec<(&'a ImportId, &'a T)> {
    let mut items: Vec<_> = map.iter().collect();

    // Sort by import name.
    //
    // Imports have a name and a module, and it's important that we *ignore*
    // the module. The module of an import is set to its final value *during*
    // code generation, so using it here would cause the imports to be sorted
    // differently depending on which part of the code generation process we're
    // in.
    items.sort_by(|&(a, _), &(b, _)| {
        let a = module.imports.get(*a);
        let b = module.imports.get(*b);

        a.name.cmp(&b.name)
    });

    items
}

fn check_duplicated_getter_and_setter_names(
    exports: &[(&AdapterId, &AuxExport)],
) -> Result<(), Error> {
    fn verify_exports(
        first_class: &str,
        first_field: &str,
        first_receiver: &AuxReceiverKind,
        second_class: &str,
        second_field: &str,
        second_receiver: &AuxReceiverKind,
    ) -> Result<(), Error> {
        let both_are_in_the_same_class = first_class == second_class;
        let both_are_referencing_the_same_field = first_field == second_field
            && first_receiver.is_static() == second_receiver.is_static();
        if both_are_in_the_same_class && both_are_referencing_the_same_field {
            bail!(format!(
                "There can be only one getter/setter definition for `{first_field}` in `{first_class}`"
            ));
        }
        Ok(())
    }
    for (idx, (_, first_export)) in exports.iter().enumerate() {
        for (_, second_export) in exports.iter().skip(idx + 1) {
            match (&first_export.kind, &second_export.kind) {
                (
                    AuxExportKind::Method {
                        class: first_class,
                        name: first_name,
                        kind: AuxExportedMethodKind::Getter,
                        receiver: first_receiver,
                    },
                    AuxExportKind::Method {
                        class: second_class,
                        name: second_name,
                        kind: AuxExportedMethodKind::Getter,
                        receiver: second_receiver,
                    },
                ) => verify_exports(
                    first_class,
                    first_name,
                    first_receiver,
                    second_class,
                    second_name,
                    second_receiver,
                )?,
                (
                    AuxExportKind::Method {
                        class: first_class,
                        name: first_name,
                        kind: AuxExportedMethodKind::Setter,
                        receiver: first_receiver,
                    },
                    AuxExportKind::Method {
                        class: second_class,
                        name: second_name,
                        kind: AuxExportedMethodKind::Setter,
                        receiver: second_receiver,
                    },
                ) => verify_exports(
                    first_class,
                    first_name,
                    first_receiver,
                    second_class,
                    second_name,
                    second_receiver,
                )?,
                _ => {}
            }
        }
    }
    Ok(())
}

fn format_doc_comments(comments: &str, js_doc_comments: Option<String>) -> String {
    let body: String = comments.lines().fold(String::new(), |mut output, c| {
        output.push_str(" *");
        if !c.is_empty() && !c.starts_with(' ') {
            output.push(' ');
        }
        output.push_str(c);
        output.push('\n');
        output
    });
    let doc = if let Some(docs) = js_doc_comments {
        docs.lines().fold(String::new(), |mut output: String, l| {
            let _ = writeln!(output, " * {l}");
            output
        })
    } else {
        String::new()
    };
    if body.is_empty() && doc.is_empty() {
        // don't emit empty doc comments
        String::new()
    } else {
        format!("/**\n{body}{doc} */\n")
    }
}

/// Defines an export with an optional namespace path segment
/// Namespaces are defined into the exports map as needed via ExportEntry::Namespace
fn define_export(
    exports: &mut BTreeMap<String, ExportEntry>,
    export_name: &str,
    ns_path: &[String],
    export: ExportEntry,
) -> Result<(), Error> {
    // Namespaces are only defined into exports by this function, not by consumers of this function.
    assert!(!matches!(export, ExportEntry::Namespace(_)));

    if ns_path.is_empty() {
        match exports.entry(export_name.to_string()) {
            Entry::Vacant(e) => {
                e.insert(export);
            }
            Entry::Occupied(mut e) => {
                if let ExportEntry::Definition(def) = export {
                    if def.definition.is_empty() {
                        if let ExportEntry::Namespace(ns) = e.get_mut() {
                            if ns.id.is_none() {
                                ns.id = Some(def.identifier);
                                return Ok(());
                            }
                        }
                    }
                }
                bail!("Cannot define export over existing namespace {export_name}");
            }
        };
    } else {
        let export_entry = exports
            .entry(ns_path[0].to_string())
            .or_insert(ExportEntry::Namespace(Default::default()));
        let ns = match export_entry {
            ExportEntry::Namespace(ns) => ns,
            ExportEntry::Definition(def) if def.definition.is_empty() => {
                *export_entry = ExportEntry::Namespace(ExportedNamespace {
                    id: Some(def.identifier.to_string()),
                    ns: BTreeMap::new(),
                });
                let ExportEntry::Namespace(ns) = export_entry else {
                    unreachable!();
                };
                ns
            }
            _ => {
                bail!("Cannot to define namespace export over existing definition {export_name}");
            }
        };
        define_export(&mut ns.ns, export_name, &ns_path[1..], export)?;
    }
    Ok(())
}

/// Returns a string to tack on to the end of an expression to access a
/// property named `name` of the object that expression resolves to.
///
/// In most cases, this is `.<name>`, generating accesses like `foo.bar`.
/// However, if `name` is not a valid JavaScript identifier, it becomes
/// `["<name>"]` instead, creating accesses like `foo["kebab-case"]`.
fn property_accessor(name: &str) -> String {
    if is_valid_ident(name) {
        format!(".{name}")
    } else {
        format!("[\"{}\"]", name.escape_default())
    }
}

impl ExportedClass {
    fn push(
        &mut self,
        function_name: &str,
        function_prefix: &str,
        js_docs: &str,
        js: &str,
        ts_docs: &str,
        ts: Option<&str>,
    ) {
        self.contents.push_str(js_docs);
        self.contents.push_str(function_prefix);
        self.contents.push_str(function_name);
        self.contents.push_str(js);
        self.contents.push('\n');
        if let Some(ts) = ts {
            if !ts_docs.is_empty() {
                for line in ts_docs.lines() {
                    self.typescript.push_str("  ");
                    self.typescript.push_str(line);
                    self.typescript.push('\n');
                }
            }
            self.typescript.push_str("  ");
            self.typescript.push_str(function_prefix);
            self.typescript.push_str(function_name);
            self.typescript.push_str(ts);
            self.typescript.push_str(";\n");
        }
    }

    fn push_accessor_ts(
        &mut self,
        location: FieldLocation,
        accessor: FieldAccessor,
        is_setter: bool,
    ) {
        let size = self.typescript_fields.len();
        let field = self
            .typescript_fields
            .entry(location)
            .or_insert_with_key(|location| FieldInfo {
                name: location.name.to_string(),
                is_static: location.is_static,
                order: size,
                getter: None,
                setter: None,
            });

        if is_setter {
            field.setter = Some(accessor);
        } else {
            field.getter = Some(accessor);
        }
    }
}

struct MemView {
    name: Cow<'static, str>,
    num: usize,
}

impl MemView {
    /// Formats the MemView specifically for accessing the memory buffer directly
    fn access(&self, is_emscripten: bool) -> String {
        if is_emscripten {
            // Emscripten global arrays (e.g., "HEAPU8") don't use the num suffix
            self.name.to_string()
        } else {
            format!("{self}()")
        }
    }
}

impl fmt::Display for MemView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.num)
    }
}

fn write_es_import(dest: &mut String, module: &str, items: &[(String, Option<String>)]) {
    dest.push_str("import { ");
    for (i, (item, rename)) in items.iter().enumerate() {
        if i > 0 {
            dest.push_str(", ");
        }
        if is_valid_ident(item) {
            dest.push_str(item);
        } else {
            dest.push('\'');
            dest.push_str(item);
            dest.push('\'');
        }
        if let Some(other) = rename {
            dest.push_str(" as ");
            dest.push_str(other);
        }
    }
    dest.push_str(" } from '");
    dest.push_str(module);
    dest.push_str("';\n");
}
