//! Individual declaration converters: AST → IR.
//!
//! Pure functions that convert oxc AST nodes into IR declarations.
//! Used by both Phase 1 (for classification) and Phase 2 (for full population).

use oxc_ast::ast;

use crate::ir;
use crate::parse::classify::classify_interface;
use crate::parse::docs::DocComments;
use crate::parse::members::{convert_class_element, convert_ts_signature};
use crate::parse::types::{convert_formal_params, convert_type_params};
use crate::util::diagnostics::DiagnosticCollector;
use crate::util::naming::to_snake_case;

pub fn convert_class_decl(
    class: &ast::Class<'_>,
    ctx: &ir::ModuleContext,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Option<ir::ClassDecl> {
    let name = class.id.as_ref()?.name.to_string();
    let js_name = name.clone();

    let type_params = convert_type_params(class.type_parameters.as_ref(), diag);

    let extends = class
        .super_class
        .as_ref()
        .and_then(|sc| match expression_to_dotted_name(sc) {
            Some(name) => Some(ir::TypeRef::Named(name)),
            None => {
                diag.warn("Complex super class expression is not supported");
                None
            }
        });

    let implements: Vec<ir::TypeRef> = class
        .implements
        .iter()
        .map(|i| convert_ts_type_name_to_ref(&i.expression))
        .collect();

    let is_abstract = class.r#abstract;

    let members: Vec<ir::Member> = class
        .body
        .body
        .iter()
        .flat_map(|elem| convert_class_element(elem, docs, diag))
        .collect();

    Some(ir::ClassDecl {
        name,
        js_name,
        type_params,
        extends,
        implements,
        is_abstract,
        members,
        type_module_context: ctx.clone(),
    })
}

pub fn convert_interface_decl(
    iface: &ast::TSInterfaceDeclaration<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> ir::InterfaceDecl {
    let name = iface.id.name.to_string();
    let js_name = name.clone();

    let type_params = convert_type_params(iface.type_parameters.as_ref(), diag);

    let extends: Vec<ir::TypeRef> = iface
        .extends
        .iter()
        .map(|ext| convert_ts_type_from_heritage(&ext.expression, diag))
        .collect();

    let members: Vec<ir::Member> = iface
        .body
        .body
        .iter()
        .flat_map(|sig| convert_ts_signature(sig, docs, diag))
        .collect();

    let classification = classify_interface(&members);

    ir::InterfaceDecl {
        name,
        js_name,
        type_params,
        extends,
        members,
        classification,
    }
}

pub fn convert_function_decl(
    func: &ast::Function<'_>,
    diag: &mut DiagnosticCollector,
) -> Option<ir::FunctionDecl> {
    let name = func.id.as_ref()?.name.to_string();
    let js_name = name.clone();
    let rust_name = to_snake_case(&name);

    let type_params = convert_type_params(func.type_parameters.as_ref(), diag);

    // Build scope from function type parameters so generic references get erased
    let scope: std::collections::HashSet<&str> = func
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.name.as_str()).collect())
        .unwrap_or_default();

    let params = convert_formal_params(&func.params, diag);
    let return_type = func
        .return_type
        .as_ref()
        .map(|rt| crate::parse::types::convert_ts_type_scoped(&rt.type_annotation, &scope, diag))
        .unwrap_or(ir::TypeRef::Void);

    Some(ir::FunctionDecl {
        name: rust_name,
        js_name,
        type_params,
        params,
        return_type,
        overloads: vec![],
    })
}

pub fn convert_string_enum(name: &str, ts_type: &ast::TSType<'_>) -> Option<ir::StringEnumDecl> {
    let variants = match ts_type {
        ast::TSType::TSUnionType(union) => union
            .types
            .iter()
            .filter_map(|t| {
                if let ast::TSType::TSLiteralType(lit) = t {
                    if let ast::TSLiteral::StringLiteral(s) = &lit.literal {
                        let js_value = s.value.to_string();
                        let rust_name = crate::util::naming::to_enum_variant(&js_value);
                        return Some(ir::StringEnumVariant {
                            rust_name,
                            js_value,
                        });
                    }
                }
                None
            })
            .collect(),
        ast::TSType::TSLiteralType(lit) => {
            if let ast::TSLiteral::StringLiteral(s) = &lit.literal {
                let js_value = s.value.to_string();
                let rust_name = crate::util::naming::to_enum_variant(&js_value);
                vec![ir::StringEnumVariant {
                    rust_name,
                    js_value,
                }]
            } else {
                return None;
            }
        }
        _ => return None,
    };

    // Deduplicate variant names (e.g., "text-plain" and "textPlain" both → "TextPlain")
    let mut rust_names: Vec<String> = variants.iter().map(|v| v.rust_name.clone()).collect();
    crate::util::naming::dedup_names(&mut rust_names);
    let variants: Vec<_> = variants
        .into_iter()
        .zip(rust_names)
        .map(|(mut v, name)| {
            v.rust_name = name;
            v
        })
        .collect();

    Some(ir::StringEnumDecl {
        name: name.to_string(),
        variants,
    })
}

/// Classify a TS enum as string or numeric based on its member initializers.
pub fn classify_ts_enum_kind(enum_decl: &ast::TSEnumDeclaration<'_>) -> ir::RegisteredKind {
    let mut has_string = false;
    let mut has_numeric = false;

    for member in &enum_decl.body.members {
        match &member.initializer {
            Some(ast::Expression::StringLiteral(_)) => has_string = true,
            Some(ast::Expression::NumericLiteral(_)) => has_numeric = true,
            Some(ast::Expression::UnaryExpression(_)) => has_numeric = true,
            None => {
                has_numeric = true;
            }
            _ => {}
        }
    }

    if has_string && !has_numeric {
        ir::RegisteredKind::StringEnum
    } else if has_numeric {
        ir::RegisteredKind::NumericEnum
    } else {
        ir::RegisteredKind::StringEnum
    }
}

/// Convert a TS enum with string values to our StringEnumDecl IR.
pub fn convert_string_ts_enum(enum_decl: &ast::TSEnumDeclaration<'_>) -> ir::StringEnumDecl {
    let name = enum_decl.id.name.to_string();
    let variants: Vec<_> = enum_decl
        .body
        .members
        .iter()
        .filter_map(|member| {
            let member_name = match &member.id {
                ast::TSEnumMemberName::Identifier(id) => id.name.to_string(),
                ast::TSEnumMemberName::String(s) => s.value.to_string(),
                _ => return None,
            };
            let js_value = match &member.initializer {
                Some(ast::Expression::StringLiteral(s)) => s.value.to_string(),
                _ => member_name.clone(),
            };
            let rust_name = crate::util::naming::to_enum_variant(&member_name);
            Some(ir::StringEnumVariant {
                rust_name,
                js_value,
            })
        })
        .collect();

    let mut rust_names: Vec<String> = variants.iter().map(|v| v.rust_name.clone()).collect();
    crate::util::naming::dedup_names(&mut rust_names);
    let variants: Vec<_> = variants
        .into_iter()
        .zip(rust_names)
        .map(|(mut v, name)| {
            v.rust_name = name;
            v
        })
        .collect();

    ir::StringEnumDecl { name, variants }
}

/// Convert a TS enum with numeric values to our NumericEnumDecl IR.
pub fn convert_numeric_enum(
    enum_decl: &ast::TSEnumDeclaration<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> ir::NumericEnumDecl {
    let name = enum_decl.id.name.to_string();
    let mut next_value: i64 = 0;

    let variants: Vec<_> = enum_decl
        .body
        .members
        .iter()
        .filter_map(|member| {
            let member_name = match &member.id {
                ast::TSEnumMemberName::Identifier(id) => id.name.to_string(),
                ast::TSEnumMemberName::String(s) => s.value.to_string(),
                _ => return None,
            };

            let value = match &member.initializer {
                Some(ast::Expression::NumericLiteral(n)) => {
                    let v = f64_to_i64(n.value, &member_name, &name, diag);
                    next_value = v + 1;
                    v
                }
                Some(ast::Expression::UnaryExpression(unary)) => {
                    if let ast::Expression::NumericLiteral(n) = &unary.argument {
                        let raw = f64_to_i64(n.value, &member_name, &name, diag);
                        let v = match unary.operator.as_str() {
                            "-" => -raw,
                            "~" => !raw,
                            _ => raw,
                        };
                        next_value = v + 1;
                        v
                    } else {
                        let v = next_value;
                        next_value += 1;
                        v
                    }
                }
                None => {
                    let v = next_value;
                    next_value += 1;
                    v
                }
                _ => {
                    let v = next_value;
                    next_value += 1;
                    v
                }
            };

            let doc = docs.for_span(member.span.start);
            let rust_name = crate::util::naming::to_enum_variant(&member_name);

            Some(ir::NumericEnumVariant {
                rust_name,
                js_name: member_name,
                value,
                doc,
            })
        })
        .collect();

    let mut rust_names: Vec<String> = variants.iter().map(|v| v.rust_name.clone()).collect();
    crate::util::naming::dedup_names(&mut rust_names);
    let variants: Vec<_> = variants
        .into_iter()
        .zip(rust_names)
        .map(|(mut v, name)| {
            v.rust_name = name;
            v
        })
        .collect();

    ir::NumericEnumDecl { name, variants }
}

fn convert_ts_type_from_heritage(
    expr: &ast::Expression<'_>,
    diag: &mut DiagnosticCollector,
) -> ir::TypeRef {
    match expression_to_dotted_name(expr) {
        Some(name) => ir::TypeRef::Named(name),
        None => {
            diag.warn("Unsupported heritage expression, falling back to Object");
            ir::TypeRef::Named("Object".to_string())
        }
    }
}

/// Extract a dotted name from an expression (e.g., `Foo.Bar.Baz` → `"Foo.Bar.Baz"`).
pub fn expression_to_dotted_name(expr: &ast::Expression<'_>) -> Option<String> {
    match expr {
        ast::Expression::Identifier(ident) => Some(ident.name.to_string()),
        ast::Expression::StaticMemberExpression(member) => {
            let left = expression_to_dotted_name(&member.object)?;
            Some(format!("{left}.{}", member.property.name))
        }
        _ => None,
    }
}

fn convert_ts_type_name_to_ref(type_name: &ast::TSTypeName<'_>) -> ir::TypeRef {
    match type_name {
        ast::TSTypeName::IdentifierReference(ident) => ir::TypeRef::Named(ident.name.to_string()),
        ast::TSTypeName::QualifiedName(qualified) => {
            let left = convert_ts_type_name_to_string(&qualified.left);
            let right = &qualified.right.name;
            ir::TypeRef::Named(format!("{left}.{right}"))
        }
        ast::TSTypeName::ThisExpression(_) => ir::TypeRef::Unresolved("this".to_string()),
    }
}

/// Debug name for an `ExportDefaultDeclarationKind` variant.
pub fn export_default_kind_name(kind: &ast::ExportDefaultDeclarationKind<'_>) -> &'static str {
    match kind {
        ast::ExportDefaultDeclarationKind::ClassDeclaration(_) => "ClassDeclaration",
        ast::ExportDefaultDeclarationKind::FunctionDeclaration(_) => "FunctionDeclaration",
        ast::ExportDefaultDeclarationKind::TSInterfaceDeclaration(_) => "TSInterfaceDeclaration",
        _ => "Expression",
    }
}

/// Convert f64 (JS number) to i64 with a diagnostic if the value is not
/// an exact integer or is out of i64 range.
fn f64_to_i64(
    value: f64,
    member_name: &str,
    enum_name: &str,
    diag: &mut DiagnosticCollector,
) -> i64 {
    if value.fract() != 0.0 {
        diag.warn(format!(
            "Enum `{enum_name}::{member_name}` has non-integer value {value}, truncating to {}",
            value as i64
        ));
    } else if value > i64::MAX as f64 || value < i64::MIN as f64 {
        diag.warn(format!(
            "Enum `{enum_name}::{member_name}` value {value} is out of i64 range, truncating"
        ));
    }
    let result = value as i64;
    // Warn if value won't fit in the codegen repr (i32 for signed, u32 for unsigned)
    if i32::try_from(result).is_err() && u32::try_from(result).is_err() {
        diag.warn(format!(
            "Enum `{enum_name}::{member_name}` value {result} exceeds i32/u32 range, \
             will be truncated in generated code"
        ));
    }
    result
}

fn convert_ts_type_name_to_string(type_name: &ast::TSTypeName<'_>) -> String {
    match type_name {
        ast::TSTypeName::IdentifierReference(ident) => ident.name.to_string(),
        ast::TSTypeName::QualifiedName(qualified) => {
            let left = convert_ts_type_name_to_string(&qualified.left);
            let right = &qualified.right.name;
            format!("{left}.{right}")
        }
        ast::TSTypeName::ThisExpression(_) => "this".to_string(),
    }
}
