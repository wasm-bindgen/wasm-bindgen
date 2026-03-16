//! Phase 1: Collect all type names into the `TypeRegistry` and scope arena.
//!
//! Uses `CollectCtx` to group shared mutable state.

use oxc_ast::ast;

use crate::ir;
use crate::parse::merge::{is_class_constructor_var, var_declarator_name};
use crate::parse::scope::ScopeId;
use crate::util::diagnostics::DiagnosticCollector;

use super::converters::{classify_ts_enum_kind, export_default_kind_name};
use super::is_string_literal_union;

/// Shared context for Phase 1 name collection.
struct CollectCtx<'a> {
    lib_name: Option<&'a str>,
    registry: &'a mut ir::TypeRegistry,
    gctx: &'a mut crate::context::GlobalContext,
    diag: &'a mut DiagnosticCollector,
}

/// Walk the AST and collect all type names into the registry and scope arena.
pub fn collect_type_names(
    program: &ast::Program<'_>,
    lib_name: Option<&str>,
    diag: &mut DiagnosticCollector,
    gctx: &mut crate::context::GlobalContext,
    root_scope: ScopeId,
) -> ir::TypeRegistry {
    let mut registry = ir::TypeRegistry::default();
    let mut ccx = CollectCtx {
        lib_name,
        registry: &mut registry,
        gctx,
        diag,
    };

    for stmt in &program.body {
        ccx.collect_statement(stmt, &ir::ModuleContext::Global, root_scope, false);
    }

    registry
}

impl<'a> CollectCtx<'a> {
    /// Collect type names from a top-level statement.
    fn collect_statement(
        &mut self,
        stmt: &ast::Statement<'_>,
        ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        match stmt {
            // Declaration variants
            ast::Statement::ClassDeclaration(class) => {
                if let Some(ref id) = class.id {
                    self.register_type(&id.name, ir::RegisteredKind::Class, ctx, scope, exported);
                }
            }
            ast::Statement::TSInterfaceDeclaration(iface) => {
                self.collect_interface_name(&iface.id.name, ctx, scope, exported);
            }
            ast::Statement::TSTypeAliasDeclaration(alias) => {
                self.collect_type_alias_name(alias, ctx, scope, exported);
            }
            ast::Statement::FunctionDeclaration(func) => {
                if let Some(ref id) = func.id {
                    self.register_type(
                        &id.name,
                        ir::RegisteredKind::Function,
                        ctx,
                        scope,
                        exported,
                    );
                }
            }
            ast::Statement::VariableDeclaration(var_decl) => {
                self.collect_variable_names(var_decl, ctx, scope, exported);
            }
            ast::Statement::TSModuleDeclaration(module) => {
                self.collect_module_names(module, ctx, scope, exported);
            }
            ast::Statement::TSEnumDeclaration(enum_decl) => {
                let name = enum_decl.id.name.to_string();
                let kind = classify_ts_enum_kind(enum_decl);
                self.register_type(&name, kind, ctx, scope, exported);
            }
            ast::Statement::TSGlobalDeclaration(global) => {
                for s in &global.body.body {
                    self.collect_statement(s, &ir::ModuleContext::Global, scope, true);
                }
            }

            // ModuleDeclaration variants
            ast::Statement::ExportNamedDeclaration(export) => {
                if let Some(ref decl) = export.declaration {
                    let export_ctx = if let Some(lib) = self.lib_name {
                        ir::ModuleContext::Module(lib.into())
                    } else {
                        self.diag.warn(
                            "Found export declaration without --lib-name; treating as global",
                        );
                        ctx.clone()
                    };
                    self.collect_declaration(decl, &export_ctx, scope, true);
                }
                if let Some(ref source_lit) = export.source {
                    let source = source_lit.value.to_string();
                    for spec in &export.specifiers {
                        let exported_name = spec.exported.name().to_string();
                        let imported = spec.local.name().to_string();
                        self.register_import(&exported_name, &imported, &source, scope);
                    }
                }
            }
            ast::Statement::ExportDefaultDeclaration(export) => match &export.declaration {
                ast::ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                    if let Some(ref id) = class.id {
                        self.register_type(&id.name, ir::RegisteredKind::Class, ctx, scope, true);
                    }
                }
                ast::ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                    if let Some(ref id) = func.id {
                        self.register_type(
                            &id.name,
                            ir::RegisteredKind::Function,
                            ctx,
                            scope,
                            true,
                        );
                    }
                }
                other => {
                    self.diag.warn(format!(
                        "Unsupported export default declaration kind: {}",
                        export_default_kind_name(other)
                    ));
                }
            },

            ast::Statement::TSExportAssignment(_) => {}
            ast::Statement::TSNamespaceExportDeclaration(_) => {}

            ast::Statement::ExportAllDeclaration(export_all) => {
                let source = export_all.source.value.to_string();
                self.register_import("*", "*", &source, scope);
                self.diag.info(format!(
                    "Registered `export * from \"{source}\"` for dependency resolution"
                ));
            }

            ast::Statement::ImportDeclaration(import) => {
                let source = import.source.value.to_string();
                if let Some(ref specifiers) = import.specifiers {
                    for spec in specifiers {
                        match spec {
                            ast::ImportDeclarationSpecifier::ImportSpecifier(s) => {
                                self.register_import(
                                    &s.local.name,
                                    &s.imported.name(),
                                    &source,
                                    scope,
                                );
                            }
                            ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                                self.register_import(&s.local.name, "default", &source, scope);
                            }
                            ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                                self.register_import(&s.local.name, "*", &source, scope);
                            }
                        }
                    }
                }
            }

            ast::Statement::TSImportEqualsDeclaration(decl) => {
                let local = decl.id.name.to_string();
                match &decl.module_reference {
                    ast::TSModuleReference::ExternalModuleReference(ext) => {
                        self.register_import(&local, "*", &ext.expression.value, scope);
                    }
                    ast::TSModuleReference::IdentifierReference(_)
                    | ast::TSModuleReference::QualifiedName(_) => {}
                }
            }

            _ => {}
        }
    }

    /// Collect names from an `ast::Declaration` (inside export blocks).
    fn collect_declaration(
        &mut self,
        decl: &ast::Declaration<'_>,
        ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        match decl {
            ast::Declaration::ClassDeclaration(class) => {
                if let Some(ref id) = class.id {
                    self.register_type(&id.name, ir::RegisteredKind::Class, ctx, scope, exported);
                }
            }
            ast::Declaration::TSInterfaceDeclaration(iface) => {
                self.collect_interface_name(&iface.id.name, ctx, scope, exported);
            }
            ast::Declaration::TSTypeAliasDeclaration(alias) => {
                self.collect_type_alias_name(alias, ctx, scope, exported);
            }
            ast::Declaration::FunctionDeclaration(func) => {
                if let Some(ref id) = func.id {
                    self.register_type(
                        &id.name,
                        ir::RegisteredKind::Function,
                        ctx,
                        scope,
                        exported,
                    );
                }
            }
            ast::Declaration::VariableDeclaration(var_decl) => {
                self.collect_variable_names(var_decl, ctx, scope, exported);
            }
            ast::Declaration::TSModuleDeclaration(module) => {
                self.collect_module_names(module, ctx, scope, exported);
            }
            ast::Declaration::TSEnumDeclaration(enum_decl) => {
                let name = enum_decl.id.name.to_string();
                let kind = classify_ts_enum_kind(enum_decl);
                self.register_type(&name, kind, ctx, scope, exported);
            }
            ast::Declaration::TSGlobalDeclaration(global) => {
                for s in &global.body.body {
                    self.collect_statement(s, &ir::ModuleContext::Global, scope, true);
                }
            }
            ast::Declaration::TSImportEqualsDeclaration(_) => {}
        }
    }

    // ─── Per-type helpers ────────────────────────────────────────────

    fn collect_interface_name(
        &mut self,
        name: &str,
        ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        if let Some(info) = self.registry.types.get_mut(name) {
            if info.kind == ir::RegisteredKind::Variable {
                info.kind = ir::RegisteredKind::MergedClassLike;
                return;
            }
        }
        self.register_type(name, ir::RegisteredKind::Interface, ctx, scope, exported);
    }

    fn collect_type_alias_name(
        &mut self,
        alias: &ast::TSTypeAliasDeclaration<'_>,
        ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        let name = alias.id.name.to_string();
        if is_string_literal_union(&alias.type_annotation) {
            self.register_type(&name, ir::RegisteredKind::StringEnum, ctx, scope, exported);
        } else {
            self.register_type(&name, ir::RegisteredKind::TypeAlias, ctx, scope, exported);
        }
    }

    fn collect_variable_names(
        &mut self,
        var_decl: &ast::VariableDeclaration<'_>,
        ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        for declarator in &var_decl.declarations {
            if let Some(name) = var_declarator_name(declarator) {
                if is_class_constructor_var(declarator) {
                    if let Some(info) = self.registry.types.get_mut(&name) {
                        if info.kind == ir::RegisteredKind::Interface {
                            info.kind = ir::RegisteredKind::MergedClassLike;
                            continue;
                        }
                    }
                    self.register_type(&name, ir::RegisteredKind::Variable, ctx, scope, exported);
                } else {
                    self.register_type(&name, ir::RegisteredKind::Variable, ctx, scope, exported);
                }
            }
        }
    }

    fn collect_module_names(
        &mut self,
        module: &ast::TSModuleDeclaration<'_>,
        parent_ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        match &module.id {
            ast::TSModuleDeclarationName::StringLiteral(s) => {
                let module_ctx = ir::ModuleContext::Module(s.value.as_str().into());
                if let Some(ast::TSModuleDeclarationBody::TSModuleBlock(block)) = &module.body {
                    for stmt in &block.body {
                        self.collect_statement(stmt, &module_ctx, scope, false);
                    }
                }
            }
            ast::TSModuleDeclarationName::Identifier(id) => {
                let name = id.name.to_string();
                let is_inside_module = matches!(parent_ctx, ir::ModuleContext::Module(_));

                if is_inside_module {
                    if let Some(ast::TSModuleDeclarationBody::TSModuleBlock(block)) = &module.body {
                        for stmt in &block.body {
                            self.collect_statement(stmt, parent_ctx, scope, false);
                        }
                    }
                } else {
                    let ns_scope = if let Some(type_id) = self.gctx.scopes.get(scope).get(&name) {
                        let decl = self.gctx.get_type(type_id);
                        if let ir::TypeKind::Namespace(ns) = &decl.kind {
                            ns.child_scope
                        } else {
                            self.create_namespace_scope(&name, scope, false)
                        }
                    } else {
                        self.create_namespace_scope(&name, scope, exported)
                    };

                    if let Some(ast::TSModuleDeclarationBody::TSModuleBlock(block)) = &module.body {
                        for stmt in &block.body {
                            self.collect_statement(
                                stmt,
                                &ir::ModuleContext::Global,
                                ns_scope,
                                false,
                            );
                        }
                    }
                }
            }
        }
    }

    // ─── Registration helpers ────────────────────────────────────────

    fn register_type(
        &mut self,
        name: &str,
        kind: ir::RegisteredKind,
        ctx: &ir::ModuleContext,
        scope: ScopeId,
        exported: bool,
    ) {
        super::register_type(name, kind, ctx, self.registry, self.gctx, scope, exported);
    }

    fn register_import(
        &mut self,
        local_name: &str,
        original_name: &str,
        from_module: &str,
        scope: ScopeId,
    ) {
        super::register_import(local_name, original_name, from_module, self.gctx, scope);
    }

    /// Create a new namespace child scope and register it.
    fn create_namespace_scope(&mut self, name: &str, scope: ScopeId, exported: bool) -> ScopeId {
        let ns_scope = self.gctx.scopes.create_child(scope);
        let ns_type_id = self.gctx.insert_type(ir::TypeDeclaration {
            kind: ir::TypeKind::Namespace(ir::NamespaceDecl {
                name: name.to_string(),
                declarations: vec![],
                child_scope: ns_scope,
            }),
            module_context: ir::ModuleContext::Global,
            doc: None,
            scope_id: scope,
            exported,
        });
        self.gctx.scopes.insert(scope, name.to_string(), ns_type_id);
        self.registry.types.insert(
            name.to_string(),
            ir::TypeInfo {
                kind: ir::RegisteredKind::Namespace,
                primary_context: ir::ModuleContext::Global,
            },
        );
        ns_scope
    }
}
