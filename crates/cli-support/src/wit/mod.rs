use crate::decode::{ImportModule, LocalModule};
use crate::descriptor::{Descriptor, Function};
use crate::descriptors::WasmBindgenDescriptorsSection;
use crate::intrinsic::Intrinsic;
use crate::transforms::threads::ThreadCount;
use crate::{decode, wasm_conventions, Bindgen, PLACEHOLDER_MODULE};
use anyhow::{anyhow, bail, ensure, Error};
use std::collections::{BTreeSet, HashMap};
use std::str;
use walrus::ir::VisitorMut;
use walrus::{ConstExpr, ElementItems, ExportId, FunctionId, ImportId, MemoryId, Module};
use wasm_bindgen_shared::struct_function_export_name;

mod incoming;
mod nonstandard;
mod outgoing;
mod standard;
pub use self::nonstandard::*;
pub use self::standard::*;

struct Context<'a> {
    start_found: bool,
    module: &'a mut Module,
    adapters: NonstandardWitSection,
    aux: WasmBindgenAux,
    /// All of the Wasm module's exported functions.
    function_exports: HashMap<String, (ExportId, FunctionId)>,
    /// All of the Wasm module's imported functions.
    function_imports: HashMap<String, (ImportId, FunctionId)>,
    memory: Option<MemoryId>,
    vendor_prefixes: HashMap<String, Vec<String>>,
    unique_crate_identifier: &'a str,
    descriptors: HashMap<String, Descriptor>,
    externref_enabled: bool,
    thread_count: Option<ThreadCount>,
    support_start: bool,
    linked_modules: bool,
    /// Tracks the descriptor signature (arguments, ret, inner_ret) used when
    /// creating each export adapter. Used to avoid incorrect deduplication
    /// when wasm-ld ICF merges invoke functions for different closure types
    /// into the same export.
    export_adapter_sigs: HashMap<AdapterId, (Vec<Descriptor>, Descriptor, Option<Descriptor>)>,
}

struct InstructionBuilder<'a, 'b> {
    input: Vec<AdapterType>,
    output: Vec<AdapterType>,
    instructions: Vec<InstructionData>,
    cx: &'a mut Context<'b>,
    return_position: bool,
}

pub fn process(
    bindgen: &mut Bindgen,
    module: &mut Module,
    programs: Vec<decode::Program>,
    thread_count: Option<ThreadCount>,
) -> Result<(NonstandardWitSectionId, WasmBindgenAuxId), Error> {
    let mut cx = Context {
        adapters: Default::default(),
        aux: Default::default(),
        function_exports: Default::default(),
        function_imports: Default::default(),
        vendor_prefixes: Default::default(),
        descriptors: Default::default(),
        unique_crate_identifier: "",
        memory: wasm_conventions::get_memory(module).ok(),
        module,
        start_found: false,
        externref_enabled: bindgen.externref,
        thread_count,
        support_start: bindgen.emit_start,
        linked_modules: bindgen.split_linked_modules,
        export_adapter_sigs: Default::default(),
    };
    cx.init()?;

    for program in programs {
        cx.program(program)?;
    }

    if !cx.start_found {
        cx.discover_main()?;
    }
    cx.find_exn_store();

    cx.verify()?;

    cx.unexport_intrinsics();

    // Sort adapter exports by function signature for deterministic output.
    // Only sort adapters that are ContextAdapterKind::Adapter (not in export_map, not in implements).
    let implements_set: std::collections::HashSet<_> = cx
        .adapters
        .implements
        .iter()
        .map(|(_, _, id)| *id)
        .collect();

    // Filter and collect adapters that need sorting
    let mut adapter_exports: Vec<_> = cx
        .adapters
        .exports
        .iter()
        .filter(|(_, adapter_id)| {
            !cx.aux.export_map.contains_key(adapter_id) && !implements_set.contains(adapter_id)
        })
        .map(|(export_id, _)| {
            let export = cx.module.exports.get(*export_id);
            let sort_key = match export.item {
                walrus::ExportItem::Function(func_id) => {
                    let ty_id = cx.module.funcs.get(func_id).ty();
                    let ty = cx.module.types.get(ty_id);
                    format!("{:?}-{:?}", ty.params(), ty.results())
                }
                _ => String::new(),
            };
            (*export_id, sort_key, export.name.clone(), export.item)
        })
        .collect();

    // Sort bare adapters by signature only to avoid machine-specific mangling non-determinism.
    // Resorting with Walrus requires deleting and re-injecting, so we then update the stored ID again.
    adapter_exports.sort_by(|a, b| a.1.cmp(&b.1));

    // Build mapping of old to new ExportIds
    let mut old_to_new: std::collections::HashMap<walrus::ExportId, walrus::ExportId> =
        std::collections::HashMap::new();

    for (old_export_id, _, name, item) in adapter_exports.iter() {
        cx.module.exports.delete(*old_export_id);
        let new_export_id = cx.module.exports.add(name, *item);
        old_to_new.insert(*old_export_id, new_export_id);

        // Update cx.adapters.exports
        if let Some(pos) = cx
            .adapters
            .exports
            .iter()
            .position(|(eid, _)| eid == old_export_id)
        {
            cx.adapters.exports[pos].0 = new_export_id;
        }
    }

    // Update CallExport instructions in all adapters to use new ExportIds
    for adapter in cx.adapters.adapters.values_mut() {
        let instructions = match &mut adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => continue,
        };
        for instr in instructions {
            if let Instruction::CallExport(export_id) = &mut instr.instr {
                if let Some(&new_id) = old_to_new.get(export_id) {
                    *export_id = new_id;
                }
            }
        }
    }

    let adapters = cx.module.customs.add(cx.adapters);
    let aux = cx.module.customs.add(cx.aux);
    Ok((adapters, aux))
}

impl<'a> Context<'a> {
    fn init(&mut self) -> Result<(), Error> {
        self.aux.stack_pointer = wasm_conventions::get_stack_pointer(self.module);

        // Make a map from string name to ids of all exports
        for export in self.module.exports.iter() {
            if let walrus::ExportItem::Function(f) = export.item {
                self.function_exports
                    .insert(export.name.clone(), (export.id(), f));
            }
        }

        // Make a map from string name to ids of all imports from our
        // placeholder module name which we'll want to be sure that we've got a
        // location listed of what to import there for each item.
        let mut duplicate_import_map = HashMap::new();
        // The order in which imports are deleted later might matter, so we
        // use an ordered set here to make everything deterministic.
        let mut imports_to_delete = BTreeSet::new();
        for import in self.module.imports.iter() {
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }
            let f = match import.kind {
                walrus::ImportKind::Function(f) => f,
                _ => continue,
            };

            match self.function_imports.get(&import.name) {
                // If this `import.name` already exists in our import map, then
                // we need to delete `import`. We also need to replace any
                // references to it with `prev_func`, so register that here to
                // happen later.
                Some((_, prev_func)) => {
                    imports_to_delete.insert(import.id());
                    duplicate_import_map.insert(f, *prev_func);
                }

                // Otherwise this is brand new, so insert it into the map.
                None => {
                    self.function_imports
                        .insert(import.name.clone(), (import.id(), f));
                }
            }
        }
        self.add_aux_import_to_import_map(
            "__wbindgen_object_clone_ref",
            vec![Descriptor::Ref(Box::new(Descriptor::Externref))],
            Descriptor::Externref,
            AuxImport::Intrinsic(Intrinsic::ObjectCloneRef),
        )?;
        self.add_aux_import_to_import_map(
            "__wbindgen_object_drop_ref",
            vec![Descriptor::Externref],
            Descriptor::Unit,
            AuxImport::Intrinsic(Intrinsic::ObjectDropRef),
        )?;
        self.add_aux_import_to_import_map(
            "__wbindgen_object_is_null_or_undefined",
            vec![Descriptor::Ref(Box::new(Descriptor::Externref))],
            Descriptor::Boolean,
            AuxImport::Intrinsic(Intrinsic::ObjectIsNullOrUndefined),
        )?;
        self.add_aux_import_to_import_map(
            "__wbindgen_object_is_undefined",
            vec![Descriptor::Ref(Box::new(Descriptor::Externref))],
            Descriptor::Boolean,
            AuxImport::Intrinsic(Intrinsic::ObjectIsUndefined),
        )?;
        for import in imports_to_delete {
            self.module.imports.delete(import);
        }

        self.inject_externref_initialization()?;

        if let Some(custom) = self
            .module
            .customs
            .delete_typed::<WasmBindgenDescriptorsSection>()
        {
            let WasmBindgenDescriptorsSection {
                descriptors,
                cast_imports,
            } = *custom;
            // Store all the executed descriptors in our own field so we have
            // access to them while processing programs.
            self.descriptors.extend(descriptors);

            // Sort cast imports by signature for deterministic output.
            let mut sorted_casts: Vec<_> = cast_imports
                .into_iter()
                .map(|(descriptor, orig_func_ids)| {
                    let signature = descriptor.unwrap_function();
                    let [arg] = &signature.arguments[..] else {
                        unreachable!("Cast function must take exactly one argument");
                    };
                    let sig_comment = format!("{arg:?} -> {:?}", &signature.ret);
                    (sig_comment, signature, orig_func_ids)
                })
                .collect();
            sorted_casts.sort_by(|a, b| a.0.cmp(&b.0));

            for (idx, (sig_comment, signature, orig_func_ids)) in
                sorted_casts.into_iter().enumerate()
            {
                // Use the sort index for a deterministic import name.
                let import_name = format!("__wbindgen_cast_{:016x}", idx + 1);

                // Manufacture an import for this cast.
                let ty = self.module.funcs.get(orig_func_ids[0]).ty();
                let (import_func_id, import_id) =
                    self.module
                        .add_import_func(PLACEHOLDER_MODULE, &import_name, ty);
                self.module.funcs.get_mut(import_func_id).name = Some(sig_comment.clone());
                let adapter_id =
                    self.import_adapter(import_id, signature, AdapterJsImportKind::Normal)?;
                self.aux
                    .import_map
                    .insert(adapter_id, AuxImport::Cast { sig_comment });

                // Mark all original functions for replacement with the new import.
                duplicate_import_map
                    .extend(orig_func_ids.into_iter().map(|id| (id, import_func_id)));
            }
        }

        self.handle_duplicate_imports(&duplicate_import_map);

        self.aux.thread_destroy = self.thread_destroy();

        Ok(())
    }

    /// The same name function from the same module may be imported at different
    /// points in a program. The compiler may synthesize two `import`
    /// statements, both with the same module/name, to match these two function
    /// imports. This is handled here.
    ///
    /// Almost all of our handling of directives and such is string-based (eew)
    /// instead of ID based due to the way the macro works right now. This means
    /// that we don't work well with these duplicate imports. As a result when
    /// we see these duplicate imports we fixup the module to ensure that only
    /// one import is used, deleting all the other imports. This is what's
    /// wanted anyway in terms of semantics.
    ///
    /// The map provided here is a map where the key is a function id to replace
    /// and the value is what to replace it with.
    fn handle_duplicate_imports(&mut self, map: &HashMap<FunctionId, FunctionId>) {
        struct Replace<'a> {
            map: &'a HashMap<FunctionId, FunctionId>,
        }
        impl VisitorMut for Replace<'_> {
            fn visit_function_id_mut(&mut self, function: &mut FunctionId) {
                if let Some(replacement) = self.map.get(function) {
                    *function = *replacement;
                }
            }
        }
        let mut replace = Replace { map };
        for (_id, func) in self.module.funcs.iter_local_mut() {
            let entry = func.entry_block();
            walrus::ir::dfs_pre_order_mut(&mut replace, func, entry);
        }
        for elems in self.module.elements.iter_mut() {
            match &mut elems.items {
                ElementItems::Functions(funcs) => {
                    for func in funcs.iter_mut() {
                        replace.visit_function_id_mut(func);
                    }
                }
                ElementItems::Expressions(_, exprs) => {
                    for expr in exprs {
                        if let ConstExpr::RefFunc(func) = expr {
                            replace.visit_function_id_mut(func);
                        }
                    }
                }
            }
        }
    }

    // Discover a function `main(i32, i32) -> i32` and, if it exists, make that function run at module start.
    fn discover_main(&mut self) -> Result<(), Error> {
        // find a `main(i32, i32) -> i32`
        let main_id = self
            .module
            .exports
            .iter()
            .filter_map(|export| match export.item {
                walrus::ExportItem::Function(id) => Some((export, self.module.funcs.get(id))),
                _ => None,
            })
            .find(|(export, func)| {
                use walrus::ValType::I32;

                // name has to be `main`
                let name_matches = export.name == "main";
                // type has to be `(i32, i32) -> i32`
                let ty = self.module.types.get(func.ty());
                let type_matches = ty.params() == [I32, I32] && ty.results() == [I32];
                // Having the correct name and signature doesn't necessarily mean that it's
                // actually a `main` function. Unfortunately, there doesn't seem to be any 100%
                // reliable way to make sure that it is, but we can at least rule out any
                // `#[wasm_bindgen]` exported functions.
                let unknown = !self
                    .adapters
                    .exports
                    .iter()
                    .any(|(export_id, _)| *export_id == export.id());
                name_matches && type_matches && unknown
            })
            .map(|(_, func)| func.id());

        let main_id = match main_id {
            Some(x) => x,
            None => return Ok(()),
        };

        // build a wrapper to zero out the arguments and ignore the return value
        let mut wrapper = walrus::FunctionBuilder::new(&mut self.module.types, &[], &[]);
        wrapper
            .func_body()
            .i32_const(0)
            .i32_const(0)
            .call(main_id)
            .drop()
            .return_();
        let wrapper = wrapper.finish(vec![], &mut self.module.funcs);

        // call that wrapper when the module starts
        self.add_start_function(wrapper)?;

        Ok(())
    }

    // Ensure that the `start` function for this module calls the
    // `__wbindgen_init_externref_table` function. This'll ensure that all
    // instances of this module have the initial slots of the externref table
    // initialized correctly.
    //
    // Note that this is disabled if WebAssembly interface types are enabled
    // since that's a slightly different environment for now which doesn't have
    // quite the same initialization.
    //
    // TODO: can we just move this to the JS generation?
    fn inject_externref_initialization(&mut self) -> Result<(), Error> {
        if !self.externref_enabled {
            return Ok(());
        }

        let ty = self.module.types.add(&[], &[]);
        let (import, import_id) =
            self.module
                .add_import_func(PLACEHOLDER_MODULE, "__wbindgen_init_externref_table", ty);

        if self.module.start.is_some() {
            let builder = wasm_conventions::get_or_insert_start_builder(self.module);
            builder.func_body().call_at(0, import);
        } else {
            self.module.start = Some(import);
        }

        let adapter_id = self.import_adapter(
            import_id,
            Function {
                shim_idx: 0,
                arguments: vec![],
                ret: Descriptor::Unit,
                inner_ret: None,
            },
            AdapterJsImportKind::Normal,
        )?;
        self.aux.import_map.insert(
            adapter_id,
            AuxImport::Intrinsic(Intrinsic::InitExternrefTable),
        );

        Ok(())
    }

    fn link_module(
        &mut self,
        id: ImportId,
        module: &decode::ImportModule,
        offset: usize,
        local_modules: &[LocalModule],
        inline_js: &[&str],
    ) -> Result<(), Error> {
        let descriptor = Function {
            shim_idx: 0,
            arguments: Vec::new(),
            ret: Descriptor::String,
            inner_ret: None,
        };
        let id = self.import_adapter(id, descriptor, AdapterJsImportKind::Normal)?;
        let (path, content) = match module {
            decode::ImportModule::Named(n) => (
                format!("snippets/{n}"),
                local_modules
                    .iter()
                    .find(|m| m.identifier == *n)
                    .map(|m| m.contents),
            ),
            decode::ImportModule::RawNamed(n) => (n.to_string(), None),
            decode::ImportModule::Inline(idx) => (
                format!(
                    "snippets/{}/inline{}.js",
                    self.unique_crate_identifier,
                    *idx as usize + offset
                ),
                Some(inline_js[*idx as usize]),
            ),
        };
        self.aux
            .import_map
            .insert(id, AuxImport::LinkTo(path, content.map(str::to_string)));
        Ok(())
    }

    fn program(&mut self, program: decode::Program<'a>) -> Result<(), Error> {
        self.unique_crate_identifier = program.unique_crate_identifier;
        let decode::Program {
            exports,
            enums,
            imports,
            structs,
            typescript_custom_sections,
            local_modules,
            inline_js,
            unique_crate_identifier,
            package_json,
            linked_modules,
        } = program;

        for module in local_modules
            .iter()
            .filter(|module| self.linked_modules || !module.linked_module)
        {
            // All local modules we find should be unique, but the same module
            // may have showed up in a few different blocks. If that's the case
            // all the same identifiers should have the same contents.
            if let Some(prev) = self
                .aux
                .local_modules
                .insert(module.identifier.to_string(), module.contents.to_string())
            {
                assert_eq!(prev, module.contents);
            }
        }
        if let Some(s) = package_json {
            self.aux.package_jsons.insert(s.into());
        }
        for export in exports {
            self.export(export)?;
        }

        let offset = self
            .aux
            .snippets
            .get(unique_crate_identifier)
            .map(|s| s.len())
            .unwrap_or(0);
        for module in linked_modules {
            if let Some((id, _)) = self.function_imports.remove(module.link_function_name) {
                self.link_module(
                    id,
                    &module.module,
                    offset,
                    &local_modules[..],
                    &inline_js[..],
                )?;
            }
        }

        // Register vendor prefixes for all types before we walk over all the
        // imports to ensure that if a vendor prefix is listed somewhere it'll
        // apply to all the imports.
        for import in imports.iter() {
            if let decode::ImportKind::Type(ty) = &import.kind {
                if ty.vendor_prefixes.is_empty() {
                    continue;
                }
                self.vendor_prefixes
                    .entry(ty.name.to_string())
                    .or_default()
                    .extend(ty.vendor_prefixes.iter().map(|s| s.to_string()));
            }
        }
        for import in imports {
            self.import(import)?;
        }

        for enum_ in enums {
            self.enum_(enum_)?;
        }
        for struct_ in structs {
            self.struct_(struct_)?;
        }

        // Collect custom sections to be sorted later when all CGUs encountered
        self.aux
            .extra_typescript
            .extend(typescript_custom_sections.iter().map(|s| s.to_string()));
        self.aux
            .snippets
            .entry(unique_crate_identifier.to_string())
            .or_default()
            .extend(inline_js.iter().map(|s| s.to_string()));
        Ok(())
    }

    fn export(&mut self, export: decode::Export<'_>) -> Result<(), Error> {
        let wasm_name = match &export.class {
            Some(class) => struct_function_export_name(class, export.function.name),
            None => {
                let base_name = export.function.name.to_string();
                if let Some(ref ns) = export.js_namespace {
                    format!("{}_{base_name}", ns.join("_"))
                } else {
                    base_name
                }
            }
        };
        let mut descriptor = match self.descriptors.remove(&wasm_name) {
            None => return Ok(()),
            Some(d) => d.unwrap_function(),
        };

        let Some((export_id, id)) = self.function_exports.get(&wasm_name).copied() else {
            bail!("{wasm_name} symbol is missing, \
                may be because there are multiple exports with the same name but different signatures, \
                and discarded by wasm-ld.");
        };

        if export.start {
            self.add_start_function(id)?;
        }

        let classless_this = matches!(
            &export.method_kind,
            decode::MethodKind::Operation(op) if matches!(op.kind, decode::OperationKind::RegularThis)
        );

        let kind = match export.class {
            Some(class) => {
                let class = class.to_string();
                match export.method_kind {
                    decode::MethodKind::Constructor => {
                        verify_constructor_return(&class, &descriptor.ret)?;
                        AuxExportKind::Constructor(class)
                    }
                    decode::MethodKind::Operation(op) => {
                        if !op.is_static {
                            // Make the first argument be the index of the receiver.
                            descriptor.arguments.insert(0, Descriptor::I32);
                        }

                        let (name, kind) = match op.kind {
                            decode::OperationKind::Getter(f) => (f, AuxExportedMethodKind::Getter),
                            decode::OperationKind::Setter(f) => (f, AuxExportedMethodKind::Setter),
                            _ => (export.function.name, AuxExportedMethodKind::Method),
                        };

                        AuxExportKind::Method {
                            class,
                            name: name.to_owned(),
                            receiver: if op.is_static {
                                AuxReceiverKind::None
                            } else if export.consumed {
                                AuxReceiverKind::Owned
                            } else {
                                AuxReceiverKind::Borrowed
                            },
                            kind,
                        }
                    }
                }
            }
            _ => {
                if classless_this {
                    AuxExportKind::FunctionThis(export.function.name.to_string())
                } else {
                    AuxExportKind::Function(export.function.name.to_string())
                }
            }
        };

        let args = Some(
            export
                .function
                .args
                .into_iter()
                .map(|v| AuxFunctionArgumentData {
                    name: v.name,
                    ty_override: v.ty_override.map(String::from),
                    desc: v.desc.map(String::from),
                })
                .collect::<Vec<_>>(),
        );
        let id = self.export_adapter(export_id, descriptor)?;
        self.aux.export_map.insert(
            id,
            AuxExport {
                debug_name: wasm_name,
                comments: concatenate_comments(&export.comments),
                args,
                asyncness: export.function.asyncness,
                kind,
                js_namespace: export
                    .js_namespace
                    .map(|ns| ns.iter().map(|s| s.to_string()).collect()),
                generate_typescript: export.function.generate_typescript,
                generate_jsdoc: export.function.generate_jsdoc,
                variadic: export.function.variadic,
                fn_ret_ty_override: export.function.ret_ty_override.map(String::from),
                fn_ret_desc: export.function.ret_desc.map(String::from),
            },
        );
        Ok(())
    }

    fn add_start_function(&mut self, id: FunctionId) -> Result<(), Error> {
        if self.start_found {
            bail!("cannot specify two `start` functions");
        }
        self.start_found = true;

        // Skip this when we're generating tests
        if !self.support_start {
            return Ok(());
        }

        if let Some(thread_count) = self.thread_count {
            let builder = wasm_conventions::get_or_insert_start_builder(self.module);
            thread_count.wrap_start(builder, id);
        } else if self.module.start.is_some() {
            let builder = wasm_conventions::get_or_insert_start_builder(self.module);

            // Note that we leave the previous start function, if any, first. This is
            // because the start function currently only shows up when it's injected
            // through thread/externref transforms. These injected start functions
            // need to happen before user code, so we always schedule them first.
            builder.func_body().call(id);
        } else {
            self.module.start = Some(id);
        }

        Ok(())
    }

    fn import(&mut self, import: decode::Import<'_>) -> Result<(), Error> {
        match &import.kind {
            decode::ImportKind::Function(_) => self.import_function(import),
            decode::ImportKind::Static(_) => self.import_static(import),
            decode::ImportKind::String(s) => self.import_string(s),
            decode::ImportKind::Type(_) => self.import_type(import),
            decode::ImportKind::Enum(e) => self.string_enum(e),
        }
    }

    fn import_function(&mut self, import: decode::Import<'_>) -> Result<(), Error> {
        let decode::ImportKind::Function(function) = import.kind else {
            unreachable!();
        };
        let decode::ImportFunction {
            shim,
            catch,
            variadic,
            method,
            structural,
            function,
            assert_no_shim,
        } = function;
        let (import_id, _id) = match self.function_imports.get(shim) {
            Some(pair) => *pair,
            None => {
                if let Some(reexport_name) = import.reexport {
                    self.aux.reexports.insert(
                        reexport_name,
                        self.determine_import(&import.module, &import.js_namespace, function.name)?,
                    );
                }
                return Ok(());
            }
        };
        let descriptor = match self.descriptors.remove(shim) {
            None => {
                return Ok(());
            }
            Some(d) => d.unwrap_function(),
        };

        // Perform two functions here. First we're saving off our adapter
        // signature, indicating what we think our import is going to be. Next
        // we're saving off other metadata indicating where this item is going
        // to be imported from. The `import_map` table will record, for each
        // import, what is getting hooked up to that slot of the import table
        // to the WebAssembly instance.
        let (id, aux_import) = match method {
            Some(data) => {
                let class =
                    self.determine_import(&import.module, &import.js_namespace, data.class)?;
                match data.kind {
                    // NB: `structural` is ignored for constructors since the
                    // js type isn't expected to change anyway.
                    decode::MethodKind::Constructor => {
                        let id = self.import_adapter(
                            import_id,
                            descriptor,
                            AdapterJsImportKind::Constructor,
                        )?;
                        (id, AuxImport::Value(AuxValue::Bare(class)))
                    }
                    decode::MethodKind::Operation(op) => {
                        let (import, method) =
                            self.determine_import_op(class, &function, structural, op)?;
                        let kind = if method {
                            AdapterJsImportKind::Method
                        } else {
                            AdapterJsImportKind::Normal
                        };
                        (self.import_adapter(import_id, descriptor, kind)?, import)
                    }
                }
            }

            // NB: `structural` is ignored for free functions since it's
            // expected that the binding isn't changing anyway.
            None => {
                let id = self.import_adapter(import_id, descriptor, AdapterJsImportKind::Normal)?;
                let aux_import = match import.module {
                    Some(ImportModule::RawNamed(PLACEHOLDER_MODULE)) => {
                        let intrinsic = function.name.parse()?;
                        if let Intrinsic::FunctionTable = intrinsic {
                            self.aux.function_table = self.module.tables.main_function_table()?;
                        }
                        AuxImport::Intrinsic(intrinsic)
                    }
                    _ => {
                        let js_import = self.determine_import(
                            &import.module,
                            &import.js_namespace,
                            function.name,
                        )?;

                        if let Some(reexport_name) = import.reexport {
                            self.aux.reexports.insert(reexport_name, js_import.clone());
                        }

                        AuxImport::Value(AuxValue::Bare(js_import))
                    }
                };
                (id, aux_import)
            }
        };

        // Record this for later as it affects JS binding generation, but note
        // that this doesn't affect the WebIDL interface at all.
        if variadic {
            self.aux.imports_with_variadic.insert(id);
        }

        // Note that `catch`/`assert_no_shim` is applied not to the import
        // itself but to the adapter shim we generated, so fetch that shim id
        // and flag it as catch here. This basically just needs to be kept in
        // sync with `js/mod.rs`.
        //
        // For `catch` once we see that we'll need an internal intrinsic later
        // for JS glue generation, so be sure to find that here.
        let adapter = self.adapters.implements.last().unwrap().2;
        if catch {
            self.aux.imports_with_catch.insert(adapter);
            if self.aux.exn_store.is_none() {
                self.find_exn_store();
            }
        }
        if assert_no_shim {
            self.aux.imports_with_assert_no_shim.insert(adapter);
        }

        self.aux.import_map.insert(id, aux_import);

        Ok(())
    }

    /// The `bool` returned indicates whether the imported value should be
    /// invoked as a method (first arg is implicitly `this`) or if the imported
    /// value is a simple function-like shim
    fn determine_import_op(
        &mut self,
        mut class: JsImport,
        function: &decode::Function<'_>,
        structural: bool,
        op: decode::Operation<'_>,
    ) -> Result<(AuxImport, bool), Error> {
        match op.kind {
            decode::OperationKind::Regular => {
                if op.is_static {
                    Ok((
                        AuxImport::ValueWithThis(class, function.name.to_string()),
                        false,
                    ))
                } else if structural {
                    Ok((
                        AuxImport::StructuralMethod(function.name.to_string()),
                        false,
                    ))
                } else {
                    class.fields.push("prototype".to_string());
                    class.fields.push(function.name.to_string());
                    Ok((AuxImport::Value(AuxValue::Bare(class)), true))
                }
            }

            decode::OperationKind::RegularThis => {
                bail!("RegularThis operation kind should only appear on exports, not imports")
            }

            decode::OperationKind::Getter(field) => {
                if structural {
                    if op.is_static {
                        Ok((
                            AuxImport::StructuralClassGetter(class, field.to_string()),
                            false,
                        ))
                    } else {
                        Ok((AuxImport::StructuralGetter(field.to_string()), false))
                    }
                } else {
                    let val = if op.is_static {
                        AuxValue::ClassGetter(class, field.to_string())
                    } else {
                        AuxValue::Getter(class, field.to_string())
                    };
                    Ok((AuxImport::Value(val), true))
                }
            }

            decode::OperationKind::Setter(field) => {
                if structural {
                    if op.is_static {
                        Ok((
                            AuxImport::StructuralClassSetter(class, field.to_string()),
                            false,
                        ))
                    } else {
                        Ok((AuxImport::StructuralSetter(field.to_string()), false))
                    }
                } else {
                    let val = if op.is_static {
                        AuxValue::ClassSetter(class, field.to_string())
                    } else {
                        AuxValue::Setter(class, field.to_string())
                    };
                    Ok((AuxImport::Value(val), true))
                }
            }

            decode::OperationKind::IndexingGetter => {
                if !structural {
                    bail!("indexing getters must always be structural");
                }
                if op.is_static {
                    Ok((AuxImport::IndexingGetterOfClass(class), false))
                } else {
                    Ok((AuxImport::IndexingGetterOfObject, false))
                }
            }

            decode::OperationKind::IndexingSetter => {
                if !structural {
                    bail!("indexing setters must always be structural");
                }
                if op.is_static {
                    Ok((AuxImport::IndexingSetterOfClass(class), false))
                } else {
                    Ok((AuxImport::IndexingSetterOfObject, false))
                }
            }

            decode::OperationKind::IndexingDeleter => {
                if !structural {
                    bail!("indexing deleters must always be structural");
                }
                if op.is_static {
                    Ok((AuxImport::IndexingDeleterOfClass(class), false))
                } else {
                    Ok((AuxImport::IndexingDeleterOfObject, false))
                }
            }
        }
    }

    fn import_static(&mut self, import: decode::Import<'_>) -> Result<(), Error> {
        let decode::ImportKind::Static(static_) = import.kind else {
            unreachable!();
        };
        let (import_id, _id) = match self.function_imports.get(static_.shim) {
            Some(pair) => *pair,
            None => {
                if let Some(reexport_name) = import.reexport {
                    self.aux.reexports.insert(
                        reexport_name,
                        self.determine_import(&import.module, &import.js_namespace, static_.name)?,
                    );
                }
                return Ok(());
            }
        };

        let descriptor = match self.descriptors.remove(static_.shim) {
            None => return Ok(()),
            Some(d) => d,
        };
        let optional = matches!(descriptor, Descriptor::Option(_));

        // Register the signature of this imported shim
        let id = self.import_adapter(
            import_id,
            Function {
                arguments: Vec::new(),
                shim_idx: 0,
                ret: descriptor,
                inner_ret: None,
            },
            AdapterJsImportKind::Normal,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        let js = self.determine_import(&import.module, &import.js_namespace, static_.name)?;

        if let Some(reexport_name) = import.reexport {
            self.aux.reexports.insert(reexport_name, js.clone());
        }

        self.aux
            .import_map
            .insert(id, AuxImport::Static { js, optional });
        Ok(())
    }

    fn import_string(&mut self, string: &decode::ImportString<'_>) -> Result<(), Error> {
        let (import_id, _id) = match self.function_imports.get(string.shim) {
            Some(pair) => *pair,
            None => return Ok(()),
        };

        // Register the signature of this imported shim
        let id = self.import_adapter(
            import_id,
            Function {
                arguments: Vec::new(),
                shim_idx: 0,
                ret: Descriptor::Externref,
                inner_ret: None,
            },
            AdapterJsImportKind::Normal,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        self.aux
            .import_map
            .insert(id, AuxImport::String(string.string.to_owned()));
        Ok(())
    }

    fn import_type(&mut self, import: decode::Import<'_>) -> Result<(), Error> {
        let decode::ImportKind::Type(type_) = import.kind else {
            unreachable!();
        };

        let (import_id, _id) = match self.function_imports.get(type_.instanceof_shim) {
            Some(pair) => *pair,
            None => {
                // If this type has a reexport attribute but no instanceof shim (e.g., from inline_js),
                // we still need to return a JsImport so it can be re-exported
                if let Some(reexport_name) = import.reexport {
                    self.aux.reexports.insert(
                        reexport_name,
                        self.determine_import(&import.module, &import.js_namespace, type_.name)?,
                    );
                }
                return Ok(());
            }
        };

        // Register the signature of this imported shim
        let id = self.import_adapter(
            import_id,
            Function {
                arguments: vec![Descriptor::Ref(Box::new(Descriptor::Externref))],
                shim_idx: 0,
                ret: Descriptor::Boolean,
                inner_ret: None,
            },
            AdapterJsImportKind::Normal,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        let js_import = self.determine_import(&import.module, &import.js_namespace, type_.name)?;
        if let Some(reexport_name) = import.reexport {
            self.aux.reexports.insert(reexport_name, js_import.clone());
        }
        self.aux
            .import_map
            .insert(id, AuxImport::Instanceof(js_import));
        Ok(())
    }

    fn string_enum(&mut self, string_enum: &decode::StringEnum<'_>) -> Result<(), Error> {
        let aux = AuxStringEnum {
            name: string_enum.name.to_string(),
            comments: concatenate_comments(&string_enum.comments),
            variant_values: string_enum
                .variant_values
                .iter()
                .map(|v| v.to_string())
                .collect(),
            generate_typescript: string_enum.generate_typescript,
            js_namespace: string_enum
                .js_namespace
                .as_ref()
                .map(|ns| ns.iter().map(|s| s.to_string()).collect()),
        };
        let mut result = Ok(());
        self.aux
            .string_enums
            .entry(aux.name.clone())
            .and_modify(|existing| {
                result = Err(anyhow!("duplicate string enums:\n{existing:?}\n{aux:?}"));
            })
            .or_insert(aux);
        result
    }

    fn enum_(&mut self, enum_: decode::Enum<'_>) -> Result<(), Error> {
        let signed = enum_.signed;
        let aux = AuxEnum {
            name: enum_.name.to_string(),
            comments: concatenate_comments(&enum_.comments),
            variants: enum_
                .variants
                .iter()
                .map(|v| {
                    let value = if signed {
                        v.value as i32 as i64
                    } else {
                        v.value as i64
                    };
                    (v.name.to_string(), value, concatenate_comments(&v.comments))
                })
                .collect(),
            generate_typescript: enum_.generate_typescript,
            js_namespace: enum_
                .js_namespace
                .as_ref()
                .map(|ns| ns.iter().map(|s| s.to_string()).collect()),
            private: enum_.private,
        };
        let mut result = Ok(());
        self.aux
            .enums
            .entry(aux.name.clone())
            .and_modify(|existing| {
                result = Err(anyhow!("duplicate enums:\n{existing:?}\n{aux:?}"));
            })
            .or_insert(aux);
        result
    }

    fn struct_(&mut self, struct_: decode::Struct<'_>) -> Result<(), Error> {
        for field in struct_.fields {
            let getter = wasm_bindgen_shared::struct_field_get(struct_.name, field.name);
            let setter = wasm_bindgen_shared::struct_field_set(struct_.name, field.name);
            let descriptor = match self.descriptors.remove(&getter) {
                None => continue,
                Some(d) => d,
            };

            // Register a webidl transformation for the getter
            let (getter_id, _) = self.function_exports[&getter];
            let getter_descriptor = Function {
                arguments: vec![Descriptor::I32],
                shim_idx: 0,
                ret: descriptor.clone(),
                inner_ret: Some(descriptor.clone()),
            };
            let getter_id = self.export_adapter(getter_id, getter_descriptor)?;
            self.aux.export_map.insert(
                getter_id,
                AuxExport {
                    debug_name: format!("getter for `{}::{}`", struct_.name, field.name),
                    args: None,
                    asyncness: false,
                    comments: concatenate_comments(&field.comments),
                    kind: AuxExportKind::Method {
                        class: struct_.name.to_string(),
                        name: field.name.to_string(),
                        receiver: AuxReceiverKind::Borrowed,
                        kind: AuxExportedMethodKind::Getter,
                    },
                    js_namespace: None,
                    generate_typescript: field.generate_typescript,
                    generate_jsdoc: field.generate_jsdoc,
                    variadic: false,
                    fn_ret_ty_override: None,
                    fn_ret_desc: None,
                },
            );

            // If present, register information for the setter as well.
            if field.readonly {
                continue;
            }

            let (setter_id, _) = self.function_exports[&setter];
            let setter_descriptor = Function {
                arguments: vec![Descriptor::I32, descriptor],
                shim_idx: 0,
                ret: Descriptor::Unit,
                inner_ret: None,
            };
            let setter_id = self.export_adapter(setter_id, setter_descriptor)?;
            self.aux.export_map.insert(
                setter_id,
                AuxExport {
                    debug_name: format!("setter for `{}::{}`", struct_.name, field.name),
                    args: None,
                    asyncness: false,
                    comments: concatenate_comments(&field.comments),
                    kind: AuxExportKind::Method {
                        class: struct_.name.to_string(),
                        name: field.name.to_string(),
                        receiver: AuxReceiverKind::Borrowed,
                        kind: AuxExportedMethodKind::Setter,
                    },
                    js_namespace: None,
                    generate_typescript: field.generate_typescript,
                    generate_jsdoc: field.generate_jsdoc,
                    variadic: false,
                    fn_ret_ty_override: None,
                    fn_ret_desc: None,
                },
            );
        }
        let aux = AuxStruct {
            name: struct_.name.to_string(),
            comments: concatenate_comments(&struct_.comments),
            is_inspectable: struct_.is_inspectable,
            generate_typescript: struct_.generate_typescript,
            js_namespace: struct_
                .js_namespace
                .as_ref()
                .map(|ns| ns.iter().map(|s| s.to_string()).collect()),
            private: struct_.private,
        };
        self.aux.structs.push(aux);

        let wrap_constructor = wasm_bindgen_shared::new_function(struct_.name);
        self.add_aux_import_to_import_map(
            &wrap_constructor,
            vec![Descriptor::I32],
            Descriptor::Externref,
            AuxImport::WrapInExportedClass(struct_.name.to_string()),
        )?;

        let unwrap_fn = wasm_bindgen_shared::unwrap_function(struct_.name);
        self.add_aux_import_to_import_map(
            &unwrap_fn,
            vec![Descriptor::Ref(Box::new(Descriptor::Externref))],
            Descriptor::I32,
            AuxImport::UnwrapExportedClass(struct_.name.to_string()),
        )?;

        Ok(())
    }

    fn add_aux_import_to_import_map(
        &mut self,
        fn_name: &str,
        arguments: Vec<Descriptor>,
        ret: Descriptor,
        aux_import: AuxImport,
    ) -> Result<(), Error> {
        if let Some((import_id, _id)) = self.function_imports.get(fn_name).cloned() {
            let signature = Function {
                shim_idx: 0,
                arguments,
                ret,
                inner_ret: None,
            };
            let id = self.import_adapter(import_id, signature, AdapterJsImportKind::Normal)?;
            self.aux.import_map.insert(id, aux_import);
        }

        Ok(())
    }

    fn determine_import(
        &self,
        module: &Option<ImportModule<'_>>,
        js_namespace: &Option<Vec<String>>,
        item: &str,
    ) -> Result<JsImport, Error> {
        // Similar to `--target no-modules`, only allow vendor prefixes
        // basically for web apis, shouldn't be necessary for things like npm
        // packages or other imported items.
        let vendor_prefixes = self.vendor_prefixes.get(item);
        if let Some(vendor_prefixes) = vendor_prefixes {
            assert!(!vendor_prefixes.is_empty());

            if let Some(decode::ImportModule::Inline(_) | decode::ImportModule::Named(_)) = module {
                bail!(
                    "local JS snippets do not support vendor prefixes for \
                     the import of `{item}` with a polyfill of `{}`",
                    &vendor_prefixes[0]
                );
            }
            if let Some(decode::ImportModule::RawNamed(module)) = module {
                bail!(
                    "import of `{item}` from `{module}` has a polyfill of `{}` listed, but
                     vendor prefixes aren't supported when importing from modules",
                    &vendor_prefixes[0],
                );
            }
            if let Some(ns) = js_namespace {
                bail!(
                    "import of `{item}` through js namespace `{}` isn't supported \
                     right now when it lists a polyfill",
                    ns.join(".")
                );
            }
            return Ok(JsImport {
                name: JsImportName::VendorPrefixed {
                    name: item.to_string(),
                    prefixes: vendor_prefixes.clone(),
                },
                fields: Vec::new(),
            });
        }

        let (name, fields) = match js_namespace {
            Some(ref ns) => {
                let mut tail = ns[1..].to_owned();
                tail.push(item.to_string());
                (ns[0].to_owned(), tail)
            }
            None => (item.to_owned(), Vec::new()),
        };

        let name = match module {
            Some(decode::ImportModule::Named(module)) => JsImportName::LocalModule {
                module: module.to_string(),
                name,
            },
            Some(decode::ImportModule::RawNamed(module)) => JsImportName::Module {
                module: module.to_string(),
                name,
            },
            Some(decode::ImportModule::Inline(idx)) => {
                let offset = self
                    .aux
                    .snippets
                    .get(self.unique_crate_identifier)
                    .map(|s| s.len())
                    .unwrap_or(0);
                JsImportName::InlineJs {
                    unique_crate_identifier: self.unique_crate_identifier.to_string(),
                    snippet_idx_in_crate: *idx as usize + offset,
                    name,
                }
            }
            None => JsImportName::Global { name },
        };
        Ok(JsImport { name, fields })
    }

    /// Perform a small verification pass over the module to perform some
    /// internal sanity checks.
    fn verify(&self) -> Result<(), Error> {
        // First up verify that all imports in the Wasm module from our
        // `$PLACEHOLDER_MODULE` are connected to an adapter via the
        // `implements` section.
        let mut implemented = HashMap::new();
        for (core, _, adapter) in self.adapters.implements.iter() {
            implemented.insert(core, adapter);
        }
        for import in self.module.imports.iter() {
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }
            match import.kind {
                walrus::ImportKind::Function(_) => {}
                _ => bail!("import from `{PLACEHOLDER_MODULE}` was not a function"),
            }

            // These are special intrinsics which were handled in the descriptor
            // phase, but we don't have an implementation for them. We don't
            // need to error about them in this verification pass though,
            // having them lingering in the module is normal.
            if import.name == "__wbindgen_describe" || import.name == "__wbindgen_describe_cast" {
                continue;
            }
            if implemented.remove(&import.id()).is_none() {
                bail!("import of `{}` doesn't have an adapter listed", import.name);
            }
        }
        if !implemented.is_empty() {
            bail!("more implementations listed than imports");
        }

        // Next up verify that all imported adapter functions have a listing of
        // where they're imported from.
        let mut imports_counted = 0;
        for (id, adapter) in self.adapters.adapters.iter() {
            let name = match &adapter.kind {
                AdapterKind::Import { name, .. } => name,
                AdapterKind::Local { .. } => continue,
            };
            if !self.aux.import_map.contains_key(id) {
                bail!("import of `{name}` doesn't have an import map item listed");
            }

            imports_counted += 1;
        }
        // Make sure there's no extraneous adapters that weren't actually
        // imported in the module.
        if self.aux.import_map.len() != imports_counted {
            bail!("import map is larger than the number of imports");
        }

        // Make sure all entries in the export map have an adapter.
        // The adapter may not be in the statically known exports list if it's
        // generated by us from a closure table entry.
        for id in self.aux.export_map.keys() {
            ensure!(
                self.adapters.adapters.contains_key(id),
                "export map has an entry that the adapters map does not"
            );
        }

        Ok(())
    }

    /// Creates an import adapter for the `import` which will have the given
    /// `signature`.
    ///
    /// Note that the JS function imported will be invoked as `kind`.
    fn import_adapter(
        &mut self,
        import: ImportId,
        signature: Function,
        kind: AdapterJsImportKind,
    ) -> Result<AdapterId, Error> {
        let import = self.module.imports.get(import);
        let import_name = import.name.clone();
        let import_id = import.id();
        let core_id = match import.kind {
            walrus::ImportKind::Function(f) => f,
            _ => bail!("bound import must be assigned to function"),
        };

        // Process the returned type first to see if it needs an out-pointer. This
        // happens if the results of the incoming arguments translated to Wasm take
        // up more than one type.
        let mut ret = self.instruction_builder(true);
        ret.incoming(&signature.ret)?;
        let uses_retptr = ret.output.len() > 1;

        // Process the argument next, allocating space of the return value if one
        // was present. Additionally configure the `module` and `adapters` to allow
        // usage of closures going out to the import.
        let mut args = ret.cx.instruction_builder(false);
        if uses_retptr {
            args.input.push(AdapterType::I32);
        }
        for arg in signature.arguments.iter() {
            args.outgoing(arg)?;
        }

        // Build up the list of instructions for our adapter function. We start out
        // with all the outgoing instructions which convert all Wasm params to the
        // desired types to call our import...
        let mut instructions = args.instructions;

        // ... and then we actually call our import. We synthesize an adapter
        // definition for it with the appropriate types here on the fly.
        let f = args.cx.adapters.append(
            args.output,
            ret.input,
            vec![],
            AdapterKind::Import {
                name: import_name,
                kind,
            },
        );
        instructions.push(InstructionData {
            instr: Instruction::CallAdapter(f),
            stack_change: StackChange::Unknown,
        });

        // ... and then we follow up with a conversion of the incoming type
        // back to wasm.
        instructions.extend(ret.instructions);

        // ... and if a return pointer is in use then we need to store the types on
        // the stack into the Wasm return pointer. Note that we iterate in reverse
        // here because the last result is the top value on the stack.
        let results = if uses_retptr {
            let mem = args.cx.memory()?;
            for (i, ty) in ret.output.into_iter().enumerate().rev() {
                instructions.push(InstructionData {
                    instr: Instruction::StoreRetptr { offset: i, ty, mem },
                    stack_change: StackChange::Modified {
                        pushed: 0,
                        popped: 1,
                    },
                });
            }
            Vec::new()
        } else {
            ret.output
        };
        let id = args.cx.adapters.append(
            args.input,
            results,
            vec![],
            AdapterKind::Local { instructions },
        );
        args.cx.adapters.implements.push((import_id, core_id, id));
        Ok(f)
    }

    /// Creates an adapter function for the `export` given to have the
    /// `signature` specified.
    fn export_adapter(
        &mut self,
        mut export: ExportId,
        signature: Function,
    ) -> Result<AdapterId, Error> {
        // Same export might be requested multiple times due to codegen-units,
        // or because wasm-ld ICF merged invoke functions for different closure
        // types into the same function. Only reuse an existing adapter if the
        // signature also matches (ignoring shim_idx which varies across
        // codegen-units).
        let sig_key = (
            signature.arguments.clone(),
            signature.ret.clone(),
            signature.inner_ret.clone(),
        );
        if let Some((_, adapter_id)) = self
            .adapters
            .exports
            .iter()
            .find(|(export_id, _)| *export_id == export)
        {
            if self.export_adapter_sigs.get(adapter_id) == Some(&sig_key) {
                // Same ExportId and signature (codegen-units duplicate).
                return Ok(*adapter_id);
            } else {
                // Same ExportId but different signature: ICF merged two different
                // closure types. Create a new export with a unique name so each
                // adapter gets its own JS function.
                let old_export = self.module.exports.get(export);
                let name = format!("{}_{}", old_export.name, self.adapters.exports.len());
                let func_id = match old_export.item {
                    walrus::ExportItem::Function(f) => f,
                    _ => unreachable!(),
                };
                export = self.module.exports.add(&name, func_id);
            }
        }

        // Figure out how to translate all the incoming arguments ...
        let mut args = self.instruction_builder(false);
        for arg in signature.arguments.iter() {
            args.incoming(arg)?;
        }

        // ... then the returned value being translated back

        let inner_ret_output = if let Some(sig_inner_ret) = &signature.inner_ret {
            let mut inner_ret = args.cx.instruction_builder(true);
            inner_ret.outgoing(sig_inner_ret)?;
            inner_ret.output
        } else {
            vec![]
        };

        let mut ret = args.cx.instruction_builder(true);
        ret.outgoing(&signature.ret)?;
        let uses_retptr = ret.input.len() > 1;

        // Our instruction stream starts out with the return pointer as the first
        // argument to the Wasm function, if one is in use. Then we convert
        // everything to Wasm types.
        //
        // After calling the core Wasm function we need to load all the return
        // pointer arguments if there were any, otherwise we simply convert
        // everything into the outgoing arguments.
        let mut instructions = Vec::new();
        if uses_retptr {
            let size = ret.input.iter().fold(0, |sum, ty| {
                let size = match ty {
                    AdapterType::I32 => 4,
                    AdapterType::I64 => 8,
                    AdapterType::F32 => 4,
                    AdapterType::F64 => 8,
                    _ => panic!("unsupported type in retptr {ty:?}"),
                };
                let sum_rounded_up = (sum + (size - 1)) & (!(size - 1));
                sum_rounded_up + size
            });
            // Round the number of bytes up to a 16-byte alignment to ensure the
            // stack pointer is always 16-byte aligned (which LLVM currently
            // requires).
            let size = (size + 15) & (!15);
            instructions.push(InstructionData {
                instr: Instruction::Retptr { size },
                stack_change: StackChange::Modified {
                    pushed: 1,
                    popped: 0,
                },
            });
        }
        instructions.extend(args.instructions);
        instructions.push(InstructionData {
            instr: Instruction::CallExport(export),
            stack_change: StackChange::Unknown,
        });
        if uses_retptr {
            let mem = ret.cx.memory()?;
            let mut unpacker = StructUnpacker::new();
            for ty in ret.input.into_iter() {
                let offset = unpacker.read_ty(&ty)?;
                instructions.push(InstructionData {
                    instr: Instruction::LoadRetptr { offset, ty, mem },
                    stack_change: StackChange::Modified {
                        pushed: 1,
                        popped: 0,
                    },
                });
            }
        }
        instructions.extend(ret.instructions);

        let id = ret.cx.adapters.append(
            args.input,
            ret.output,
            inner_ret_output,
            AdapterKind::Local { instructions },
        );

        self.adapters.exports.push((export, id));
        self.export_adapter_sigs.insert(id, sig_key);

        Ok(id)
    }

    fn instruction_builder<'b>(&'b mut self, return_position: bool) -> InstructionBuilder<'b, 'a> {
        InstructionBuilder {
            cx: self,
            input: Vec::new(),
            output: Vec::new(),
            instructions: Vec::new(),
            return_position,
        }
    }

    fn malloc(&self) -> Result<FunctionId, Error> {
        self.function_exports
            .get("__wbindgen_malloc")
            .cloned()
            .map(|p| p.1)
            .ok_or_else(|| anyhow!("failed to find declaration of `__wbindgen_malloc` in module"))
    }

    fn realloc(&self) -> Option<FunctionId> {
        self.function_exports
            .get("__wbindgen_realloc")
            .cloned()
            .map(|p| p.1)
    }

    fn free(&self) -> Result<FunctionId, Error> {
        self.function_exports
            .get("__wbindgen_free")
            .cloned()
            .map(|p| p.1)
            .ok_or_else(|| anyhow!("failed to find declaration of `__wbindgen_free` in module"))
    }

    fn thread_destroy(&self) -> Option<FunctionId> {
        self.function_exports
            .get("__wbindgen_thread_destroy")
            .cloned()
            .map(|p| p.1)
    }

    fn memory(&self) -> Result<MemoryId, Error> {
        self.memory
            .ok_or_else(|| anyhow!("failed to find memory declaration in module"))
    }

    /// Removes the export item for all `__wbindgen` intrinsics which are
    /// generally only purely internal helpers.
    ///
    /// References to these functions are preserved through adapter instructions
    /// if necessary, otherwise they can all be gc'd out. By the time this
    /// function is called our discovery of these intrinsics has completed and
    /// there's no need to keep around these exports.
    fn unexport_intrinsics(&mut self) {
        let mut to_remove = Vec::new();
        for export in self.module.exports.iter() {
            match export.name.as_str() {
                n if n.starts_with("__wbindgen") => {
                    to_remove.push(export.id());
                }
                _ => {}
            }
        }
        for id in to_remove {
            self.module.exports.delete(id);
        }
    }

    /// Attempts to locate the `__wbindgen_exn_store` intrinsic and stores it in
    /// our auxiliary information.
    ///
    /// This is only invoked if the intrinsic will actually be needed for JS
    /// glue generation somewhere.
    fn find_exn_store(&mut self) {
        self.aux.exn_store = self
            .module
            .exports
            .iter()
            .find(|e| e.name == "__wbindgen_exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            });
    }
}

/// Verifies exported constructor return value is not a JS primitive type
fn verify_constructor_return(class: &str, ret: &Descriptor) -> Result<(), Error> {
    match ret {
        Descriptor::I8
        | Descriptor::U8
        | Descriptor::ClampedU8
        | Descriptor::I16
        | Descriptor::U16
        | Descriptor::I32
        | Descriptor::U32
        | Descriptor::F32
        | Descriptor::F64
        | Descriptor::I64
        | Descriptor::U64
        | Descriptor::Boolean
        | Descriptor::Char
        | Descriptor::CachedString
        | Descriptor::String
        | Descriptor::Option(_)
        | Descriptor::Enum { .. }
        | Descriptor::Unit => {
            bail!("The constructor for class `{class}` tries to return a JS primitive type, which would cause the return value to be ignored. Use a builder instead (remove the `constructor` attribute).");
        }
        Descriptor::Result(ref d) | Descriptor::Ref(ref d) | Descriptor::RefMut(ref d) => {
            verify_constructor_return(class, d)
        }
        _ => Ok(()),
    }
}

/// Extract all of the `Program`s encoded in our custom section.
///
/// `program_storage` is used to squirrel away the raw bytes of the custom
///  section, so that they can be referenced by the `Program`s we return.
pub fn extract_programs<'a>(
    module: &mut Module,
    program_storage: &'a mut Vec<Vec<u8>>,
) -> Result<Vec<decode::Program<'a>>, Error> {
    let my_version = wasm_bindgen_shared::version();
    assert!(program_storage.is_empty());

    while let Some(raw) = module.customs.remove_raw("__wasm_bindgen_unstable") {
        log::debug!(
            "custom section '{}' looks like a Wasm bindgen section",
            raw.name
        );
        program_storage.push(raw.data);
    }

    let mut ret = Vec::new();
    for program in program_storage.iter() {
        let mut payload = &program[..];
        while let Some(data) = get_remaining(&mut payload) {
            // Historical versions of wasm-bindgen have used JSON as the custom
            // data section format. Newer versions, however, are using a custom
            // serialization protocol that looks much more like the Wasm spec.
            //
            // We, however, want a sanity check to ensure that if we're running
            // against the wrong wasm-bindgen we get a nicer error than an
            // internal decode error. To that end we continue to verify a tiny
            // bit of json at the beginning of each blob before moving to the
            // next blob. This should keep us compatible with older wasm-bindgen
            // instances as well as forward-compatible for now.
            //
            // Note, though, that as `wasm-pack` picks up steam it's hoped we
            // can just delete this entirely. The `wasm-pack` project already
            // manages versions for us, so we in theory should need this check
            // less and less over time.
            if let Some(their_version) = verify_schema_matches(data)? {
                bail!(
                    "

it looks like the Rust project used to create this Wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust Wasm file schema version: {their_version}
     this binary schema version: {my_version}

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or
the wasm-bindgen dependency in the Rust project.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen --precise {my_version}

don't forget to recompile your Wasm file! Alternatively, you can update the
binary with:

    cargo install -f wasm-bindgen-cli --version {their_version}

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/wasm-bindgen/wasm-bindgen/issues!
"
                );
            }
            let next = get_remaining(&mut payload).unwrap();
            log::debug!("found a program of length {}", next.len());
            ret.push(<decode::Program as decode::Decode>::decode_all(next));
        }
    }
    Ok(ret)
}

fn get_remaining<'a>(data: &mut &'a [u8]) -> Option<&'a [u8]> {
    if data.is_empty() {
        return None;
    }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let (a, b) = data[4..].split_at(len);
    *data = b;
    Some(a)
}

fn verify_schema_matches(data: &[u8]) -> Result<Option<&str>, Error> {
    macro_rules! bad {
        () => {
            bail!("failed to decode what looked like wasm-bindgen data")
        };
    }
    let data = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => bad!(),
    };
    log::debug!("found version specifier {data}");
    if !data.starts_with('{') || !data.ends_with('}') {
        bad!()
    }
    let needle = "\"schema_version\":\"";
    let rest = match data.find(needle) {
        Some(i) => &data[i + needle.len()..],
        None => bad!(),
    };
    let their_schema_version = match rest.find('"') {
        Some(i) => &rest[..i],
        None => bad!(),
    };
    if their_schema_version == wasm_bindgen_shared::SCHEMA_VERSION {
        return Ok(None);
    }
    let needle = "\"version\":\"";
    let rest = match data.find(needle) {
        Some(i) => &data[i + needle.len()..],
        None => bad!(),
    };
    let their_version = match rest.find('"') {
        Some(i) => &rest[..i],
        None => bad!(),
    };
    Ok(Some(their_version))
}

fn concatenate_comments(comments: &[&str]) -> String {
    comments.join("\n")
}

/// The C struct packing algorithm, in terms of u32.
struct StructUnpacker {
    next_offset: usize,
}

impl StructUnpacker {
    fn new() -> Self {
        Self { next_offset: 0 }
    }
    fn align_up(&mut self, alignment_pow2: usize) -> usize {
        let mask = alignment_pow2 - 1;
        self.next_offset = (self.next_offset + mask) & (!mask);
        self.next_offset
    }
    fn append(&mut self, quads: usize, alignment_pow2: usize) -> usize {
        let ret = self.align_up(alignment_pow2);
        self.next_offset += quads;
        ret
    }
    /// Returns the offset for this member, with the offset in multiples of u32.
    fn read_ty(&mut self, ty: &AdapterType) -> Result<usize, Error> {
        let (quads, alignment) = match ty {
            AdapterType::I32 | AdapterType::U32 | AdapterType::F32 => (1, 1),
            AdapterType::I64 | AdapterType::U64 | AdapterType::F64 => (2, 2),
            other => bail!("invalid aggregate return type {other:?}"),
        };
        Ok(self.append(quads, alignment))
    }
}

#[test]
fn test_struct_packer() {
    let mut unpacker = StructUnpacker::new();
    let i32___ = &AdapterType::I32;
    let double = &AdapterType::F64;
    let mut read_ty = |ty| unpacker.read_ty(ty).unwrap();
    assert_eq!(read_ty(i32___), 0); // u32
    assert_eq!(read_ty(i32___), 1); // u32
    assert_eq!(read_ty(double), 2); // f64, already aligned
    assert_eq!(read_ty(i32___), 4); // u32, already aligned
    assert_eq!(read_ty(double), 6); // f64, NOT already aligned, skips up to offset 6
}
