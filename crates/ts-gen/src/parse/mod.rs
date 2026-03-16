//! Parser orchestration: parse `.d.ts` files into IR.

pub mod classify;
pub mod docs;
pub mod first_pass;
pub mod members;
pub mod merge;
pub mod resolve;
pub mod scope;
pub mod types;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

use crate::context::GlobalContext;
use crate::ir::Module;
use crate::parse::scope::ScopeId;

/// Parse one or more `.d.ts` files and produce a `Module` + `GlobalContext`.
pub fn parse_dts_files(
    paths: &[impl AsRef<Path>],
    lib_name: Option<&str>,
) -> Result<(Module, GlobalContext)> {
    let mut gctx = GlobalContext::new();
    let mut input_type_ids = Vec::new();
    let mut input_file_scopes = Vec::new();

    // Root scope: built-in types (js_sys globals, etc.)
    let builtin = gctx.create_root_scope();
    populate_builtin_scope(&mut gctx, builtin);

    // Track which files we've already parsed to avoid cycles
    let mut parsed_files: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    // All file scopes (including deps) for import resolution
    let mut all_file_scopes = Vec::new();
    // Map scope → source file parent directory (for relative import resolution)
    let mut scope_dirs: std::collections::HashMap<ScopeId, PathBuf> =
        std::collections::HashMap::new();

    for path in paths {
        let path = path.as_ref();
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        parsed_files.insert(canonical);

        let source = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let file_scope = gctx.create_child_scope(builtin);
        input_file_scopes.push(file_scope);
        all_file_scopes.push(file_scope);
        if let Some(parent) = path.parent() {
            scope_dirs.insert(file_scope, parent.to_path_buf());
        }
        let type_ids = parse_single_file(&source, path, lib_name, &mut gctx, file_scope)?;
        input_type_ids.extend(type_ids);
    }

    // Resolve imports: parse dependency files for scope resolution.
    resolve_imports(
        &mut gctx,
        builtin,
        &mut all_file_scopes,
        &mut scope_dirs,
        &mut parsed_files,
        lib_name,
    )?;

    Ok((
        Module {
            types: input_type_ids,
            lib_name: lib_name.map(|s| s.to_string()),
            builtin_scope: builtin,
            file_scopes: input_file_scopes,
        },
        gctx,
    ))
}

/// Parse a single `.d.ts` source string into a `Module` + `GlobalContext`.
pub fn parse_single_source(
    source: &str,
    lib_name: Option<&str>,
) -> Result<(Module, GlobalContext)> {
    let mut gctx = GlobalContext::new();
    let builtin = gctx.create_root_scope();
    populate_builtin_scope(&mut gctx, builtin);
    let file_scope = gctx.create_child_scope(builtin);
    let path = Path::new("<input>");
    let type_ids = parse_single_file(source, path, lib_name, &mut gctx, file_scope)?;
    Ok((
        Module {
            types: type_ids,
            lib_name: lib_name.map(|s| s.to_string()),
            builtin_scope: builtin,
            file_scopes: vec![file_scope],
        },
        gctx,
    ))
}

fn parse_single_file(
    source: &str,
    path: &Path,
    lib_name: Option<&str>,
    gctx: &mut GlobalContext,
    file_scope: ScopeId,
) -> Result<Vec<crate::context::TypeId>> {
    let allocator = Allocator::default();
    let source_type = SourceType::d_ts();

    gctx.diagnostics.set_file(path, source);

    let parser_return = Parser::new(&allocator, source, source_type).parse();

    if !parser_return.errors.is_empty() {
        for error in &parser_return.errors {
            gctx.warn(format!("Parse error in {}: {}", path.display(), error));
        }
    }

    let program = &parser_return.program;
    let doc_comments = docs::DocComments::new(&program.comments, source);

    // Phase 1: Collect type names and populate file scope
    // Swap out diagnostics temporarily to avoid double-borrowing gctx.
    let mut diag = std::mem::take(&mut gctx.diagnostics);
    let registry = first_pass::collect_type_names(program, lib_name, &mut diag, gctx, file_scope);
    gctx.diagnostics = diag;

    gctx.info(format!(
        "Collected {} type names from {}",
        registry.types.len(),
        path.display()
    ));

    // Log pending imports found for this file's scope
    let import_logs: Vec<String> = gctx
        .pending_imports
        .iter()
        .filter(|imp| imp.scope == file_scope)
        .map(|imp| {
            format!(
                "Import: {} from \"{}\" (pending)",
                imp.local_name, imp.from_module
            )
        })
        .collect();
    for msg in import_logs {
        gctx.info(msg);
    }

    // Phase 2: Populate declarations
    // Take immutable snapshots before the mutable borrow on diagnostics.
    let scopes_snapshot = gctx.scopes.clone();
    let type_arena_snapshot = gctx.type_arena().to_vec();
    let declarations = first_pass::populate_declarations(
        program,
        &registry,
        lib_name,
        &doc_comments,
        &mut gctx.diagnostics,
        &scopes_snapshot,
        &type_arena_snapshot,
        file_scope,
    );

    // Post-processing: merge, dedup, then insert into the global type arena
    let merged = merge_class_pairs(declarations);
    let merged = merge_namespaces(merged);
    let merged = dedup_function_overloads(merged);

    // Detect script vs module — script files have global declarations.
    let is_script = !first_pass::is_module(program);

    let type_ids: Vec<crate::context::TypeId> = merged
        .into_iter()
        .map(|mut decl| {
            // Script files: all declarations are implicitly exported (global).
            if is_script {
                decl.exported = true;
            }
            let name = declaration_name(&decl.kind);
            let type_id = gctx.insert_type(decl);
            // Insert/upgrade the type in the scope so codegen can resolve it.
            if let Some(ref name) = name {
                gctx.scopes.insert(file_scope, name.clone(), type_id);
            }
            // Script files: hoist global declarations to the builtin scope
            // so they're visible across all files (TypeScript "script" semantics).
            if is_script {
                if let Some(parent) = gctx.scopes.get(file_scope).parent {
                    if let Some(name) = name {
                        gctx.scopes.insert(parent, name, type_id);
                    }
                }
            }
            type_id
        })
        .collect();

    Ok(type_ids)
}

/// Deduplicate function overloads: when multiple `FunctionDecl` share the same
/// name and module context, keep only the one with the most parameters (the most
/// general signature). TypeScript overloads are a single JS function at runtime.
fn dedup_function_overloads(
    declarations: Vec<crate::ir::TypeDeclaration>,
) -> Vec<crate::ir::TypeDeclaration> {
    use crate::ir::{ModuleContext, TypeDeclaration, TypeKind};

    // Key: (function name, module context)
    let mut best: std::collections::HashMap<(String, ModuleContext), usize> =
        std::collections::HashMap::new();
    let mut result: Vec<TypeDeclaration> = Vec::new();
    let mut skip: std::collections::HashSet<usize> = std::collections::HashSet::new();

    // First pass: find the best (most params) overload for each name
    for (i, decl) in declarations.iter().enumerate() {
        if let TypeKind::Function(ref f) = decl.kind {
            let key = (f.name.clone(), decl.module_context.clone());
            if let Some(&existing_idx) = best.get(&key) {
                // Compare param counts — keep the one with more params
                if let TypeKind::Function(ref existing_f) = declarations[existing_idx].kind {
                    if f.params.len() > existing_f.params.len() {
                        skip.insert(existing_idx);
                        best.insert(key, i);
                    } else {
                        skip.insert(i);
                    }
                }
            } else {
                best.insert(key, i);
            }
        }
    }

    for (i, decl) in declarations.into_iter().enumerate() {
        if !skip.contains(&i) {
            result.push(decl);
        }
    }

    result
}

/// Resolve unresolved imports by finding and parsing dependency files.
///
/// Iterates until no new files are discovered (handles transitive deps).
fn resolve_imports(
    gctx: &mut GlobalContext,
    builtin: ScopeId,
    file_scopes: &mut Vec<ScopeId>,
    scope_dirs: &mut std::collections::HashMap<ScopeId, PathBuf>,
    parsed_files: &mut std::collections::HashSet<PathBuf>,
    lib_name: Option<&str>,
) -> Result<()> {
    // Track modules that we've already tried and failed to resolve,
    // so we don't re-attempt them on every iteration.
    let mut failed_modules: std::collections::HashSet<String> = std::collections::HashSet::new();

    loop {
        // Drain pending imports that haven't been resolved yet.
        let pending: Vec<scope::PendingImport> = gctx
            .pending_imports
            .drain(..)
            .filter(|p| !failed_modules.contains(&p.from_module))
            .collect();

        if pending.is_empty() {
            break;
        }

        let mut new_files_parsed = false;
        let mut still_pending = Vec::new();

        for import in pending {
            // Check if we already have this module registered
            if let Some(module_id) = gctx.find_module(&import.from_module) {
                // Look up the name in the target module's scope
                let target_scope = gctx.get_module(module_id).scope;
                if let Some(type_id) = gctx.scopes.resolve(target_scope, &import.original_name) {
                    gctx.scopes
                        .insert(import.scope, import.local_name.clone(), type_id);
                } else {
                    gctx.warn(format!(
                        "Import `{}` not found in module \"{}\"",
                        import.original_name, import.from_module
                    ));
                }
                continue;
            }

            // Try to resolve the module specifier to a file.
            let base_dir = scope_dirs
                .get(&import.scope)
                .cloned()
                .unwrap_or_else(|| PathBuf::from("."));
            let resolved_path = resolve::resolve_module(&import.from_module, &base_dir);

            if let Some(path) = resolved_path {
                let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());

                if !parsed_files.contains(&canonical) {
                    gctx.info(format!(
                        "Resolving import \"{}\" → {}",
                        import.from_module,
                        path.display()
                    ));

                    match std::fs::read_to_string(&path) {
                        Ok(source) => {
                            let dep_scope = gctx.create_child_scope(builtin);
                            file_scopes.push(dep_scope);
                            parsed_files.insert(canonical);
                            if let Some(parent) = path.parent() {
                                scope_dirs.insert(dep_scope, parent.to_path_buf());
                            }

                            let module_id =
                                gctx.register_module(import.from_module.clone(), dep_scope);

                            match parse_single_file(&source, &path, lib_name, gctx, dep_scope) {
                                Ok(dep_type_ids) => {
                                    gctx.get_module_mut(module_id).types = dep_type_ids;
                                    new_files_parsed = true;

                                    // Now resolve the imported name from the dep's scope
                                    if let Some(type_id) =
                                        gctx.scopes.resolve(dep_scope, &import.original_name)
                                    {
                                        gctx.scopes.insert(
                                            import.scope,
                                            import.local_name.clone(),
                                            type_id,
                                        );
                                    } else {
                                        gctx.warn(format!(
                                            "Import `{}` not found in \"{}\"",
                                            import.original_name, import.from_module
                                        ));
                                    }
                                }
                                Err(e) => {
                                    gctx.warn(format!(
                                        "Failed to parse dependency \"{}\" ({}): {e}",
                                        import.from_module,
                                        path.display()
                                    ));
                                    failed_modules.insert(import.from_module.clone());
                                }
                            }
                        }
                        Err(e) => {
                            gctx.warn(format!(
                                "Failed to read dependency \"{}\" ({}): {e}",
                                import.from_module,
                                path.display()
                            ));
                            failed_modules.insert(import.from_module.clone());
                        }
                    }
                } else {
                    // File already parsed but module wasn't found — retry next iteration
                    still_pending.push(import);
                }
            } else {
                gctx.warn(format!(
                    "Could not resolve import \"{}\" — use --external to map this type",
                    import.from_module
                ));
                failed_modules.insert(import.from_module.clone());
            }
        }

        // Re-add imports that couldn't be resolved yet
        gctx.pending_imports.extend(still_pending);

        if !new_files_parsed {
            break;
        }
    }

    Ok(())
}

/// Populate the builtin scope with well-known global JS type names.
///
/// These are types that exist in every TypeScript environment without imports:
/// Array, Map, Set, Promise, Error, Date, typed arrays, etc.
/// The parser recognizes most of these and maps them to TypeRef variants directly,
/// but registering them in the builtin scope ensures that scope resolution
/// correctly identifies them as built-in rather than unresolved.
fn populate_builtin_scope(gctx: &mut GlobalContext, scope: ScopeId) {
    // js_sys types: real types available via `use js_sys::*`.
    // Registered as opaque interfaces — resolve_alias won't look through them.
    for &name in crate::codegen::typemap::JS_SYS_RESERVED {
        let type_id = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::Interface(crate::ir::InterfaceDecl {
                name: name.to_string(),
                js_name: name.to_string(),
                type_params: vec![],
                extends: vec![],
                members: vec![],
                classification: crate::ir::InterfaceClassification::ClassLike,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: scope,
            exported: false,
        });
        gctx.scopes.insert(scope, name.to_string(), type_id);
    }

    // Web platform built-in types (ReadableStream, Request, etc.).
    // Registered as opaque interfaces — codegen emits them as bare identifiers.
    // Users provide actual definitions via --external or input files.
    for name in &[
        "ReadableStream",
        "WritableStream",
        "TransformStream",
        "Request",
        "Response",
        "Headers",
        "Blob",
        "File",
        "FormData",
        "URL",
        "URLSearchParams",
        "Event",
        "EventTarget",
        "AbortController",
        "AbortSignal",
        "WebSocket",
        "Worker",
        "Crypto",
        "CryptoKey",
        "SubtleCrypto",
        "TextEncoder",
        "TextDecoder",
    ] {
        let type_id = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::Interface(crate::ir::InterfaceDecl {
                name: name.to_string(),
                js_name: name.to_string(),
                type_params: vec![],
                extends: vec![],
                members: vec![],
                classification: crate::ir::InterfaceClassification::ClassLike,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: scope,
            exported: false,
        });
        gctx.scopes.insert(scope, name.to_string(), type_id);
    }
}

/// Merge namespace declarations with the same name into a single `NamespaceDecl`.
///
/// TypeScript allows multiple `namespace Foo { ... }` blocks that get merged.
/// We consolidate them so codegen produces a single `mod foo { ... }`.
fn merge_namespaces(
    declarations: Vec<crate::ir::TypeDeclaration>,
) -> Vec<crate::ir::TypeDeclaration> {
    use crate::ir::{TypeDeclaration, TypeKind};
    use std::collections::HashMap;

    let mut ns_map: HashMap<String, usize> = HashMap::new();
    let mut result: Vec<TypeDeclaration> = Vec::new();

    for decl in declarations {
        if let TypeKind::Namespace(ref ns_decl) = decl.kind {
            if let Some(&existing_idx) = ns_map.get(&ns_decl.name) {
                // Merge into existing namespace
                if let TypeKind::Namespace(ref mut existing) = result[existing_idx].kind {
                    existing.declarations.extend(ns_decl.declarations.clone());
                }
                continue;
            }
            let name = ns_decl.name.clone();
            let idx = result.len();
            ns_map.insert(name, idx);
        }
        result.push(decl);
    }

    result
}

/// Extract the name from a declaration kind, if it has one.
fn declaration_name(kind: &crate::ir::TypeKind) -> Option<String> {
    use crate::ir::TypeKind;
    match kind {
        TypeKind::Class(c) => Some(c.name.clone()),
        TypeKind::Interface(i) => Some(i.name.clone()),
        TypeKind::TypeAlias(a) => Some(a.name.clone()),
        TypeKind::StringEnum(e) => Some(e.name.clone()),
        TypeKind::NumericEnum(e) => Some(e.name.clone()),
        TypeKind::Function(f) => Some(f.name.clone()),
        TypeKind::Variable(v) => Some(v.name.clone()),
        TypeKind::Namespace(n) => Some(n.name.clone()),
    }
}

/// Merge two member lists, deduplicating by (kind, name).
/// Members from `incoming` override same-named members in `base`.
fn merge_members(base: &mut Vec<crate::ir::Member>, incoming: Vec<crate::ir::Member>) {
    use std::collections::HashMap;

    // Build a map of existing members by key
    let mut by_key: HashMap<MemberKey, usize> = HashMap::new();
    for (i, m) in base.iter().enumerate() {
        by_key.insert(member_key(m), i);
    }

    for m in incoming {
        let key = member_key(&m);
        if let Some(&idx) = by_key.get(&key) {
            // Override existing
            base[idx] = m;
        } else {
            by_key.insert(key, base.len());
            base.push(m);
        }
    }
}

/// A lightweight key for member deduplication.
///
/// Getters and setters for the same JS property name get distinct keys,
/// since they are independent bindings that should not overwrite each other.
#[derive(PartialEq, Eq, Hash)]
enum MemberKey {
    Constructor,
    StaticMethod(String),
    StaticGetter(String),
    StaticSetter(String),
    Proto(String),
    ProtoGetter(String),
    ProtoSetter(String),
}

fn member_key(member: &crate::ir::Member) -> MemberKey {
    match member {
        crate::ir::Member::Constructor(_) => MemberKey::Constructor,
        crate::ir::Member::StaticMethod(m) => MemberKey::StaticMethod(m.name.clone()),
        crate::ir::Member::StaticGetter(g) => MemberKey::StaticGetter(g.js_name.clone()),
        crate::ir::Member::StaticSetter(s) => MemberKey::StaticSetter(s.js_name.clone()),
        crate::ir::Member::Method(m) => MemberKey::Proto(m.name.clone()),
        crate::ir::Member::Getter(g) => MemberKey::ProtoGetter(g.js_name.clone()),
        crate::ir::Member::Setter(s) => MemberKey::ProtoSetter(s.js_name.clone()),
        crate::ir::Member::IndexSignature(_) => MemberKey::Proto("[index]".to_string()),
    }
}

/// Merge declarations with the same name:
/// - Interface + Interface → merge members (TypeScript declaration merging)
/// - Interface + Class → merge interface members into class (var+interface pattern)
/// - Class + Class → merge members
fn merge_class_pairs(
    declarations: Vec<crate::ir::TypeDeclaration>,
) -> Vec<crate::ir::TypeDeclaration> {
    use crate::ir::{TypeDeclaration, TypeKind};
    use std::collections::HashMap;

    let mut class_map: HashMap<String, usize> = HashMap::new();
    let mut iface_map: HashMap<String, usize> = HashMap::new();
    let mut result: Vec<TypeDeclaration> = Vec::new();

    for decl in declarations {
        match &decl.kind {
            TypeKind::Class(class_decl) => {
                let name = class_decl.name.clone();

                // Merge into existing class
                if let Some(&existing_idx) = class_map.get(&name) {
                    if let TypeKind::Class(ref mut existing) = result[existing_idx].kind {
                        merge_members(&mut existing.members, class_decl.members.clone());
                    }
                    continue;
                }

                // Merge into existing interface (promote to class)
                if let Some(&iface_idx) = iface_map.get(&name) {
                    // Replace the interface with this class, merging members
                    let mut new_class = class_decl.clone();
                    if let TypeKind::Interface(ref iface) = result[iface_idx].kind {
                        merge_members(&mut new_class.members, iface.members.clone());
                        if new_class.extends.is_none() {
                            new_class.extends = iface.extends.first().cloned();
                        }
                        if new_class.type_params.is_empty() {
                            new_class.type_params = iface.type_params.clone();
                        }
                    }
                    result[iface_idx] = TypeDeclaration {
                        kind: TypeKind::Class(new_class),
                        module_context: decl.module_context.clone(),
                        doc: decl.doc.clone(),
                        scope_id: decl.scope_id,
                        exported: decl.exported,
                    };
                    class_map.insert(name.clone(), iface_idx);
                    iface_map.remove(&name);
                    continue;
                }

                let idx = result.len();
                class_map.insert(name, idx);
                result.push(decl);
            }
            TypeKind::Interface(iface_decl) => {
                let name = iface_decl.name.clone();

                // Merge into existing class
                if let Some(&class_idx) = class_map.get(&name) {
                    if let TypeKind::Class(ref mut class) = result[class_idx].kind {
                        merge_members(&mut class.members, iface_decl.members.clone());
                        if class.extends.is_none() {
                            class.extends = iface_decl.extends.first().cloned();
                        }
                        if class.type_params.is_empty() {
                            class.type_params = iface_decl.type_params.clone();
                        }
                    }
                    continue;
                }

                // Merge into existing interface
                if let Some(&existing_idx) = iface_map.get(&name) {
                    if let TypeKind::Interface(ref mut existing) = result[existing_idx].kind {
                        merge_members(&mut existing.members, iface_decl.members.clone());
                        // Merge extends
                        for ext in &iface_decl.extends {
                            if !existing.extends.contains(ext) {
                                existing.extends.push(ext.clone());
                            }
                        }
                    }
                    continue;
                }

                let idx = result.len();
                iface_map.insert(name, idx);
                result.push(decl);
            }
            _ => {
                result.push(decl);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{GetterMember, Member, MethodMember, TypeRef};

    fn method(name: &str) -> Member {
        Member::Method(MethodMember {
            name: name.to_string(),
            js_name: name.to_string(),
            type_params: vec![],
            params: vec![],
            return_type: TypeRef::Void,
            optional: false,
            doc: None,
        })
    }

    fn getter(name: &str) -> Member {
        Member::Getter(GetterMember {
            js_name: name.to_string(),
            type_ref: TypeRef::String,
            optional: false,
            doc: None,
        })
    }

    #[test]
    fn test_merge_members_dedup() {
        let mut base = vec![method("read"), method("write"), getter("name")];
        let incoming = vec![method("write"), method("end")];
        merge_members(&mut base, incoming);

        // write is overridden (not duplicated), end is appended
        assert_eq!(base.len(), 4);
        assert!(matches!(&base[0], Member::Method(m) if m.name == "read"));
        assert!(matches!(&base[1], Member::Method(m) if m.name == "write"));
        assert!(matches!(&base[2], Member::Getter(g) if g.js_name == "name"));
        assert!(matches!(&base[3], Member::Method(m) if m.name == "end"));
    }

    #[test]
    fn test_merge_members_no_overlap() {
        let mut base = vec![method("foo")];
        let incoming = vec![method("bar")];
        merge_members(&mut base, incoming);
        assert_eq!(base.len(), 2);
    }

    #[test]
    fn test_merge_members_getter_and_method_coexist() {
        // ProtoGetter and Proto are different keys — both survive
        let mut base = vec![getter("data")];
        let incoming = vec![method("data")];
        merge_members(&mut base, incoming);
        assert_eq!(base.len(), 2);
        assert!(matches!(&base[0], Member::Getter(g) if g.js_name == "data"));
        assert!(matches!(&base[1], Member::Method(m) if m.name == "data"));
    }
}
