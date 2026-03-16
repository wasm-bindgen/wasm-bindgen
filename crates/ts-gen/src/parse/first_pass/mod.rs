//! Two-phase first pass over the oxc AST to build the IR.
//!
//! - Phase 1 (`collect`): Collect all type names into the `TypeRegistry`.
//! - Phase 2 (`populate`): Fully populate IR declarations using the registry for resolution.
//! - `converters`: Pure AST → IR conversion functions shared by both phases.

use oxc_ast::ast;

use crate::ir;
use crate::parse::scope::ScopeId;

mod collect;
pub mod converters;
mod populate;

// Re-export public entry points.
pub use collect::collect_type_names;
pub use populate::populate_declarations;

// ─── Script vs Module Detection ─────────────────────────────────────

/// Detect whether a file is a TypeScript "script" or "module".
///
/// A file is a **module** if any top-level statement is an `import` or `export`.
/// Script files have all top-level declarations in global scope.
/// Module files have declarations local to the file — only `declare global {}`
/// blocks affect the global scope.
pub fn is_module(program: &ast::Program<'_>) -> bool {
    program.body.iter().any(|stmt| {
        matches!(
            stmt,
            ast::Statement::ImportDeclaration(_)
                | ast::Statement::ExportAllDeclaration(_)
                | ast::Statement::ExportDefaultDeclaration(_)
                | ast::Statement::ExportNamedDeclaration(_)
                | ast::Statement::TSExportAssignment(_)
        )
    })
}

// ─── Shared Helpers ─────────────────────────────────────────────────

fn is_string_literal_union(ts_type: &ast::TSType<'_>) -> bool {
    match ts_type {
        ast::TSType::TSUnionType(union) => union.types.iter().all(|t| {
            matches!(
                t,
                ast::TSType::TSLiteralType(lit) if matches!(&lit.literal, ast::TSLiteral::StringLiteral(_))
            )
        }),
        ast::TSType::TSLiteralType(lit) => matches!(&lit.literal, ast::TSLiteral::StringLiteral(_)),
        _ => false,
    }
}

fn register_type(
    name: &str,
    kind: ir::RegisteredKind,
    ctx: &ir::ModuleContext,
    registry: &mut ir::TypeRegistry,
    gctx: &mut crate::context::GlobalContext,
    scope: ScopeId,
    exported: bool,
) {
    registry.types.insert(
        name.to_string(),
        ir::TypeInfo {
            kind: kind.clone(),
            primary_context: ctx.clone(),
        },
    );

    // Create a placeholder TypeDeclaration based on the registered kind.
    let name_str = name.to_string();
    let placeholder_kind = match kind {
        ir::RegisteredKind::Class | ir::RegisteredKind::MergedClassLike => {
            ir::TypeKind::Class(ir::ClassDecl {
                name: name_str.clone(),
                js_name: name_str.clone(),
                type_params: vec![],
                extends: None,
                implements: vec![],
                is_abstract: false,
                members: vec![],
                type_module_context: ctx.clone(),
            })
        }
        ir::RegisteredKind::Interface => ir::TypeKind::Interface(ir::InterfaceDecl {
            name: name_str.clone(),
            js_name: name_str.clone(),
            type_params: vec![],
            extends: vec![],
            members: vec![],
            classification: ir::InterfaceClassification::Unclassified,
        }),
        ir::RegisteredKind::StringEnum => ir::TypeKind::StringEnum(ir::StringEnumDecl {
            name: name_str.clone(),
            variants: vec![],
        }),
        ir::RegisteredKind::NumericEnum => ir::TypeKind::NumericEnum(ir::NumericEnumDecl {
            name: name_str.clone(),
            variants: vec![],
        }),
        ir::RegisteredKind::TypeAlias => ir::TypeKind::TypeAlias(ir::TypeAliasDecl {
            name: name_str.clone(),
            type_params: vec![],
            target: ir::TypeRef::Any,
            from_module: None,
        }),
        ir::RegisteredKind::Function => ir::TypeKind::Function(ir::FunctionDecl {
            name: name_str.clone(),
            js_name: name_str.clone(),
            type_params: vec![],
            params: vec![],
            return_type: ir::TypeRef::Any,
            overloads: vec![],
        }),
        ir::RegisteredKind::Variable => ir::TypeKind::Variable(ir::VariableDecl {
            name: name_str.clone(),
            js_name: name_str.clone(),
            type_ref: ir::TypeRef::Any,
            is_const: false,
        }),
        ir::RegisteredKind::Namespace => {
            let ns_scope = gctx.scopes.create_child(scope);
            ir::TypeKind::Namespace(ir::NamespaceDecl {
                name: name_str.clone(),
                declarations: vec![],
                child_scope: ns_scope,
            })
        }
    };

    let type_id = gctx.insert_type(ir::TypeDeclaration {
        kind: placeholder_kind,
        module_context: ctx.clone(),
        doc: None,
        scope_id: scope,
        exported,
    });
    gctx.scopes.insert(scope, name_str, type_id);
}

/// Register an import as a pending import.
fn register_import(
    local_name: &str,
    original_name: &str,
    from_module: &str,
    gctx: &mut crate::context::GlobalContext,
    scope: ScopeId,
) {
    gctx.pending_imports
        .push(crate::parse::scope::PendingImport {
            scope,
            local_name: local_name.to_string(),
            from_module: from_module.to_string(),
            original_name: original_name.to_string(),
        });
}
