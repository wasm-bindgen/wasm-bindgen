//! Phase 2: Fully populate IR declarations using the registry for resolution.
//!
//! Uses `PopulateCtx` to group shared state and avoid long parameter lists.
//! Declaration-level logic is in `populate_declaration`, shared between
//! top-level statements and `export` blocks (Item 21 dedup).

use oxc_ast::ast;

use crate::ir;
use crate::parse::docs::DocComments;
use crate::parse::merge::{extract_var_members, var_declarator_name};
use crate::parse::scope::{ScopeArena, ScopeId};
use crate::parse::types::{convert_ts_type, convert_type_params};
use crate::util::diagnostics::DiagnosticCollector;
use crate::util::naming::to_snake_case;

use super::converters::{
    convert_class_decl, convert_function_decl, convert_interface_decl, convert_numeric_enum,
    convert_string_enum, convert_string_ts_enum, export_default_kind_name,
};

/// Shared context for Phase 2 declaration population.
struct PopulateCtx<'a> {
    registry: &'a ir::TypeRegistry,
    lib_name: Option<&'a str>,
    docs: &'a DocComments<'a>,
    diag: &'a mut DiagnosticCollector,
    scopes: &'a ScopeArena,
    /// Read-only Phase 1 declarations (for looking up namespace child scopes).
    type_arena: &'a [ir::TypeDeclaration],
}

/// Walk the AST again and fully populate the IR declarations.
#[allow(clippy::too_many_arguments)]
pub fn populate_declarations(
    program: &ast::Program<'_>,
    registry: &ir::TypeRegistry,
    lib_name: Option<&str>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
    scopes: &ScopeArena,
    type_arena: &[ir::TypeDeclaration],
    scope: ScopeId,
) -> Vec<ir::TypeDeclaration> {
    let mut declarations = Vec::new();
    let mut pcx = PopulateCtx {
        registry,
        lib_name,
        docs,
        diag,
        scopes,
        type_arena,
    };

    for stmt in &program.body {
        pcx.populate_statement(
            stmt,
            &ir::ModuleContext::Global,
            &mut declarations,
            scope,
            false,
        );
    }

    declarations
}

/// Per-declaration context passed to `populate_declaration`.
struct DeclCtx<'a> {
    module_context: &'a ir::ModuleContext,
    export_span_start: Option<u32>,
    scope: ScopeId,
    exported: bool,
}

impl<'a> DeclCtx<'a> {
    /// Build a `TypeDeclaration` with this context's module/scope/exported fields.
    fn decl(&self, kind: ir::TypeKind, doc: Option<String>) -> ir::TypeDeclaration {
        ir::TypeDeclaration {
            kind,
            module_context: self.module_context.clone(),
            doc,
            scope_id: self.scope,
            exported: self.exported,
        }
    }
}

impl<'a> PopulateCtx<'a> {
    /// Populate from a top-level statement.
    fn populate_statement(
        &mut self,
        stmt: &ast::Statement<'_>,
        ctx: &ir::ModuleContext,
        declarations: &mut Vec<ir::TypeDeclaration>,
        scope: ScopeId,
        exported: bool,
    ) {
        let dcx = DeclCtx {
            module_context: ctx,
            export_span_start: None,
            scope,
            exported,
        };

        match stmt {
            // Declaration variants — delegate to shared handler
            ast::Statement::ClassDeclaration(class) => {
                self.populate_class(class, &dcx, declarations);
            }
            ast::Statement::TSInterfaceDeclaration(iface) => {
                self.populate_interface(iface, &dcx, declarations);
            }
            ast::Statement::TSTypeAliasDeclaration(alias) => {
                self.populate_type_alias(alias, &dcx, declarations);
            }
            ast::Statement::FunctionDeclaration(func) => {
                self.populate_function(func, &dcx, declarations);
            }
            ast::Statement::VariableDeclaration(var_decl) => {
                self.populate_variable_decl(var_decl, &dcx, declarations);
            }
            ast::Statement::TSModuleDeclaration(module) => {
                self.populate_module(module, ctx, declarations, scope, exported);
            }
            ast::Statement::TSEnumDeclaration(enum_decl) => {
                self.populate_ts_enum(enum_decl, &dcx, declarations);
            }
            ast::Statement::TSGlobalDeclaration(global) => {
                for s in &global.body.body {
                    self.populate_statement(
                        s,
                        &ir::ModuleContext::Global,
                        declarations,
                        scope,
                        true,
                    );
                }
            }

            // ModuleDeclaration variants
            ast::Statement::ExportNamedDeclaration(export) => {
                if let Some(ref decl) = export.declaration {
                    let export_ctx = if let Some(lib) = self.lib_name {
                        ir::ModuleContext::Module(lib.into())
                    } else {
                        ctx.clone()
                    };
                    self.populate_oxc_declaration(
                        decl,
                        &export_ctx,
                        declarations,
                        Some(export.span.start),
                        scope,
                    );
                }
                for spec in &export.specifiers {
                    let exported_name = spec.exported.name().to_string();
                    let local = spec.local.name().to_string();
                    if exported_name == local && export.source.is_none() {
                        continue;
                    }
                    let from_module = export.source.as_ref().map(|s| s.value.to_string());
                    declarations.push(ir::TypeDeclaration {
                        kind: ir::TypeKind::TypeAlias(ir::TypeAliasDecl {
                            name: exported_name,
                            type_params: vec![],
                            target: ir::TypeRef::Named(local),
                            from_module,
                        }),
                        module_context: ctx.clone(),
                        doc: None,
                        scope_id: scope,
                        exported: true,
                    });
                }
            }
            ast::Statement::ExportDefaultDeclaration(export) => match &export.declaration {
                ast::ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                    let doc = self
                        .docs
                        .for_span(export.span.start)
                        .or_else(|| self.docs.for_span(class.span.start));
                    if let Some(decl) = convert_class_decl(class, ctx, self.docs, self.diag) {
                        declarations.push(ir::TypeDeclaration {
                            kind: ir::TypeKind::Class(decl),
                            module_context: ctx.clone(),
                            doc,
                            scope_id: scope,
                            exported: true,
                        });
                    }
                }
                ast::ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                    let doc = self
                        .docs
                        .for_span(export.span.start)
                        .or_else(|| self.docs.for_span(func.span.start));
                    if let Some(decl) = convert_function_decl(func, self.diag) {
                        declarations.push(ir::TypeDeclaration {
                            kind: ir::TypeKind::Function(decl),
                            module_context: ctx.clone(),
                            doc,
                            scope_id: scope,
                            exported: true,
                        });
                    }
                }
                other => {
                    self.diag.warn(format!(
                        "Unsupported export default declaration kind: {}",
                        export_default_kind_name(other)
                    ));
                }
            },

            ast::Statement::TSExportAssignment(_) => {
                self.diag
                    .info("Skipping `export =` (namespace contents already emitted)");
            }
            ast::Statement::TSNamespaceExportDeclaration(decl) => {
                self.diag.info(format!(
                    "Skipping `export as namespace {}` (UMD namespace export)",
                    decl.id.name
                ));
            }
            ast::Statement::ExportAllDeclaration(decl) => {
                let source = decl.source.value.as_str();
                self.diag.warn(format!(
                    "Re-export (`export * from \"{source}\"`) is not yet supported, skipping"
                ));
            }
            ast::Statement::ImportDeclaration(_) => {}
            ast::Statement::TSImportEqualsDeclaration(_) => {}
            _ => {}
        }
    }

    /// Populate from an `ast::Declaration` inside an export block.
    /// Dispatches to the same per-type methods as `populate_statement`.
    fn populate_oxc_declaration(
        &mut self,
        decl: &ast::Declaration<'_>,
        ctx: &ir::ModuleContext,
        declarations: &mut Vec<ir::TypeDeclaration>,
        export_span_start: Option<u32>,
        scope: ScopeId,
    ) {
        let dcx = DeclCtx {
            module_context: ctx,
            export_span_start,
            scope,
            exported: true,
        };

        match decl {
            ast::Declaration::ClassDeclaration(class) => {
                self.populate_class(class, &dcx, declarations);
            }
            ast::Declaration::TSInterfaceDeclaration(iface) => {
                self.populate_interface(iface, &dcx, declarations);
            }
            ast::Declaration::TSTypeAliasDeclaration(alias) => {
                self.populate_type_alias(alias, &dcx, declarations);
            }
            ast::Declaration::FunctionDeclaration(func) => {
                self.populate_function(func, &dcx, declarations);
            }
            ast::Declaration::VariableDeclaration(var_decl) => {
                self.populate_variable_decl(var_decl, &dcx, declarations);
            }
            ast::Declaration::TSModuleDeclaration(module) => {
                self.populate_module(module, ctx, declarations, scope, true);
            }
            ast::Declaration::TSEnumDeclaration(enum_decl) => {
                self.populate_ts_enum(enum_decl, &dcx, declarations);
            }
            ast::Declaration::TSGlobalDeclaration(global) => {
                for s in &global.body.body {
                    self.populate_statement(
                        s,
                        &ir::ModuleContext::Global,
                        declarations,
                        scope,
                        true,
                    );
                }
            }
            ast::Declaration::TSImportEqualsDeclaration(_) => {}
        }
    }

    // ─── Per-type populate methods (shared between statement & declaration) ───

    fn populate_class(
        &mut self,
        class: &ast::Class<'_>,
        dcx: &DeclCtx<'_>,
        declarations: &mut Vec<ir::TypeDeclaration>,
    ) {
        let doc = self.lookup_doc(dcx.export_span_start, class.span.start);
        if let Some(d) = convert_class_decl(class, dcx.module_context, self.docs, self.diag) {
            declarations.push(dcx.decl(ir::TypeKind::Class(d), doc));
        }
    }

    fn populate_interface(
        &mut self,
        iface: &ast::TSInterfaceDeclaration<'_>,
        dcx: &DeclCtx<'_>,
        declarations: &mut Vec<ir::TypeDeclaration>,
    ) {
        let doc = self.lookup_doc(dcx.export_span_start, iface.span.start);
        let iface_decl = convert_interface_decl(iface, self.docs, self.diag);
        declarations.push(dcx.decl(ir::TypeKind::Interface(iface_decl), doc));
    }

    fn populate_type_alias(
        &mut self,
        alias: &ast::TSTypeAliasDeclaration<'_>,
        dcx: &DeclCtx<'_>,
        declarations: &mut Vec<ir::TypeDeclaration>,
    ) {
        let name = alias.id.name.to_string();
        let doc = self.lookup_doc(dcx.export_span_start, alias.span.start);

        if self
            .registry
            .types
            .get(&name)
            .map(|info| info.kind == ir::RegisteredKind::StringEnum)
            .unwrap_or(false)
        {
            if let Some(enum_decl) = convert_string_enum(&name, &alias.type_annotation) {
                declarations.push(dcx.decl(ir::TypeKind::StringEnum(enum_decl), doc));
                return;
            }
        }

        let type_params = convert_type_params(alias.type_parameters.as_ref(), self.diag);
        let target = convert_ts_type(&alias.type_annotation, self.diag);
        declarations.push(dcx.decl(
            ir::TypeKind::TypeAlias(ir::TypeAliasDecl {
                name,
                type_params,
                target,
                from_module: None,
            }),
            doc,
        ));
    }

    fn populate_function(
        &mut self,
        func: &ast::Function<'_>,
        dcx: &DeclCtx<'_>,
        declarations: &mut Vec<ir::TypeDeclaration>,
    ) {
        let doc = self.lookup_doc(dcx.export_span_start, func.span.start);
        if let Some(d) = convert_function_decl(func, self.diag) {
            declarations.push(dcx.decl(ir::TypeKind::Function(d), doc));
        }
    }

    fn populate_variable_decl(
        &mut self,
        var_decl: &ast::VariableDeclaration<'_>,
        dcx: &DeclCtx<'_>,
        declarations: &mut Vec<ir::TypeDeclaration>,
    ) {
        let doc = self.lookup_doc(dcx.export_span_start, var_decl.span.start);
        for declarator in &var_decl.declarations {
            if let Some(name) = var_declarator_name(declarator) {
                if self
                    .registry
                    .types
                    .get(&name)
                    .map(|info| info.kind == ir::RegisteredKind::MergedClassLike)
                    .unwrap_or(false)
                {
                    if let Some(type_ann) = &declarator.type_annotation {
                        if let ast::TSType::TSTypeLiteral(lit) = &type_ann.type_annotation {
                            let (ctor, static_members) =
                                extract_var_members(lit, self.docs, self.diag);

                            let mut members = Vec::new();
                            if let Some(c) = ctor {
                                members.push(ir::Member::Constructor(c));
                            }
                            members.extend(static_members);

                            declarations.push(dcx.decl(
                                ir::TypeKind::Class(ir::ClassDecl {
                                    name: name.clone(),
                                    js_name: name,
                                    type_params: vec![],
                                    extends: None,
                                    implements: vec![],
                                    is_abstract: false,
                                    members,
                                    type_module_context: dcx.module_context.clone(),
                                }),
                                doc.clone(),
                            ));
                            continue;
                        }
                    }
                }

                let type_ref = declarator
                    .type_annotation
                    .as_ref()
                    .map(|ann| convert_ts_type(&ann.type_annotation, self.diag))
                    .unwrap_or(ir::TypeRef::Any);

                if let ir::TypeRef::Function(sig) = type_ref {
                    declarations.push(dcx.decl(
                        ir::TypeKind::Function(ir::FunctionDecl {
                            name: to_snake_case(&name),
                            js_name: name,
                            type_params: vec![],
                            params: sig.params,
                            return_type: *sig.return_type,
                            overloads: vec![],
                        }),
                        doc.clone(),
                    ));
                } else {
                    let is_const = matches!(var_decl.kind, ast::VariableDeclarationKind::Const);
                    declarations.push(dcx.decl(
                        ir::TypeKind::Variable(ir::VariableDecl {
                            name: to_snake_case(&name),
                            js_name: name,
                            type_ref,
                            is_const,
                        }),
                        doc.clone(),
                    ));
                }
            }
        }
    }

    fn populate_module(
        &mut self,
        module: &ast::TSModuleDeclaration<'_>,
        parent_ctx: &ir::ModuleContext,
        declarations: &mut Vec<ir::TypeDeclaration>,
        scope: ScopeId,
        exported: bool,
    ) {
        let doc = self.docs.for_span(module.span.start);
        match &module.id {
            ast::TSModuleDeclarationName::StringLiteral(s) => {
                let module_ctx = ir::ModuleContext::Module(s.value.as_str().into());
                if let Some(ast::TSModuleDeclarationBody::TSModuleBlock(block)) = &module.body {
                    for stmt in &block.body {
                        self.populate_statement(stmt, &module_ctx, declarations, scope, false);
                    }
                }
            }
            ast::TSModuleDeclarationName::Identifier(id) => {
                let ns_name = id.name.to_string();
                let is_inside_module = matches!(parent_ctx, ir::ModuleContext::Module(_));
                // Use the namespace's child scope from Phase 1 (fixes Item 10).
                let ns_scope = self.resolve_namespace_scope(&ns_name, scope);

                if is_inside_module {
                    // Flatten: emit declarations directly into the parent module.
                    if let Some(ast::TSModuleDeclarationBody::TSModuleBlock(block)) = &module.body {
                        for stmt in &block.body {
                            self.populate_statement(
                                stmt,
                                parent_ctx,
                                declarations,
                                ns_scope,
                                exported,
                            );
                        }
                    }
                } else {
                    let mut ns_decls = Vec::new();
                    if let Some(ast::TSModuleDeclarationBody::TSModuleBlock(block)) = &module.body {
                        for stmt in &block.body {
                            self.populate_statement(
                                stmt,
                                &ir::ModuleContext::Global,
                                &mut ns_decls,
                                ns_scope,
                                exported,
                            );
                        }
                    }
                    declarations.push(ir::TypeDeclaration {
                        kind: ir::TypeKind::Namespace(ir::NamespaceDecl {
                            name: ns_name,
                            declarations: ns_decls,
                            child_scope: ns_scope,
                        }),
                        module_context: ir::ModuleContext::Global,
                        doc,
                        scope_id: scope,
                        exported,
                    });
                }
            }
        }
    }

    fn populate_ts_enum(
        &mut self,
        enum_decl: &ast::TSEnumDeclaration<'_>,
        dcx: &DeclCtx<'_>,
        declarations: &mut Vec<ir::TypeDeclaration>,
    ) {
        let name = enum_decl.id.name.to_string();
        let doc = self.docs.for_span(enum_decl.span.start);

        let is_numeric = self
            .registry
            .types
            .get(&name)
            .map(|info| info.kind == ir::RegisteredKind::NumericEnum)
            .unwrap_or(false);

        if is_numeric {
            let decl = convert_numeric_enum(enum_decl, self.docs, self.diag);
            declarations.push(dcx.decl(ir::TypeKind::NumericEnum(decl), doc));
        } else {
            let decl = convert_string_ts_enum(enum_decl);
            declarations.push(dcx.decl(ir::TypeKind::StringEnum(decl), doc));
        }
    }

    // ─── Helpers ─────────────────────────────────────────────────────

    fn lookup_doc(&self, export_span_start: Option<u32>, inner_span_start: u32) -> Option<String> {
        export_span_start
            .and_then(|s| self.docs.for_span(s))
            .or_else(|| self.docs.for_span(inner_span_start))
    }

    /// Look up the child scope that Phase 1 created for a namespace.
    /// Falls back to the parent scope if not found.
    fn resolve_namespace_scope(&self, ns_name: &str, parent_scope: ScopeId) -> ScopeId {
        if let Some(type_id) = self.scopes.resolve(parent_scope, ns_name) {
            let decl = &self.type_arena[type_id.index()];
            if let ir::TypeKind::Namespace(ref ns) = decl.kind {
                return ns.child_scope;
            }
        }
        parent_scope
    }
}
