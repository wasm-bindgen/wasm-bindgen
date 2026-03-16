//! Convert oxc TypeScript type AST nodes to our IR `TypeRef`.

use std::collections::HashSet;

use oxc_ast::ast::*;

use crate::ir::TypeRef;
use crate::util::diagnostics::DiagnosticCollector;
use crate::util::naming::to_snake_case;

/// Set of in-scope generic type parameter names.
/// Names in this set resolve to `TypeRef::Any` instead of `TypeRef::Named`.
pub type TypeParamScope<'a> = HashSet<&'a str>;

/// Convert an oxc `TSType` to our IR `TypeRef`.
///
/// If the type appears inside a generic declaration (class, method, function),
/// pass the in-scope type parameter names via `convert_ts_type_scoped` instead.
pub fn convert_ts_type(ts_type: &TSType<'_>, diag: &mut DiagnosticCollector) -> TypeRef {
    convert_ts_type_scoped(ts_type, &HashSet::new(), diag)
}

/// Convert an oxc `TSType` to our IR `TypeRef`, with type parameter scope.
///
/// Type parameters in `scope` are erased to `TypeRef::Any` since they can't
/// be represented at the wasm_bindgen FFI boundary.
pub fn convert_ts_type_scoped(
    ts_type: &TSType<'_>,
    scope: &TypeParamScope<'_>,
    diag: &mut DiagnosticCollector,
) -> TypeRef {
    // Unwrap parenthesized types
    let ts_type = ts_type.without_parenthesized();

    match ts_type {
        // === Keyword types ===
        TSType::TSAnyKeyword(_) => TypeRef::Any,
        TSType::TSBooleanKeyword(_) => TypeRef::Boolean,
        TSType::TSBigIntKeyword(_) => TypeRef::BigInt,
        TSType::TSNeverKeyword(_) => TypeRef::Any, // erase `never` to `any`
        TSType::TSNullKeyword(_) => TypeRef::Null,
        TSType::TSNumberKeyword(_) => TypeRef::Number,
        TSType::TSObjectKeyword(_) => TypeRef::Object,
        TSType::TSStringKeyword(_) => TypeRef::String,
        TSType::TSSymbolKeyword(_) => TypeRef::Symbol,
        TSType::TSUndefinedKeyword(_) => TypeRef::Undefined,
        TSType::TSUnknownKeyword(_) => TypeRef::Unknown,
        TSType::TSVoidKeyword(_) => TypeRef::Void,
        TSType::TSIntrinsicKeyword(_) => TypeRef::Any,

        // === Type reference (named type, generic instantiation) ===
        TSType::TSTypeReference(type_ref) => convert_type_reference_scoped(type_ref, scope, diag),

        // === Array types ===
        TSType::TSArrayType(arr) => {
            let inner = convert_ts_type_scoped(&arr.element_type, scope, diag);
            TypeRef::Array(Box::new(inner))
        }

        // === Union types ===
        TSType::TSUnionType(union_type) => {
            let types: Vec<TypeRef> = union_type
                .types
                .iter()
                .map(|t| convert_ts_type_scoped(t, scope, diag))
                .collect();
            simplify_union(types)
        }

        // === Intersection types ===
        TSType::TSIntersectionType(inter) => {
            let types: Vec<TypeRef> = inter
                .types
                .iter()
                .map(|t| convert_ts_type_scoped(t, scope, diag))
                .collect();
            TypeRef::Intersection(types)
        }

        // === Tuple types ===
        TSType::TSTupleType(tuple) => {
            let types: Vec<TypeRef> = tuple
                .element_types
                .iter()
                .map(|elem| convert_tuple_element_scoped(elem, scope, diag))
                .collect();
            TypeRef::Tuple(types)
        }

        // === Function types ===
        TSType::TSFunctionType(func) => {
            let sig = convert_function_type_scoped(func, scope, diag);
            TypeRef::Function(sig)
        }

        // === Literal types ===
        TSType::TSLiteralType(lit) => convert_literal_type(lit, diag),

        // === Type literal (object type) ===
        TSType::TSTypeLiteral(_) => TypeRef::Object,

        // === Constructor type ===
        TSType::TSConstructorType(_) => {
            diag.warn("Constructor types are not supported, erasing to JsValue");
            TypeRef::Any
        }

        // === Conditional types, mapped types, template literals, etc. ===
        TSType::TSConditionalType(_) => {
            diag.warn("Conditional types are not supported, erasing to JsValue");
            TypeRef::Unresolved("conditional type".to_string())
        }
        TSType::TSMappedType(_) => {
            diag.warn("Mapped types are not supported, erasing to JsValue");
            TypeRef::Unresolved("mapped type".to_string())
        }
        TSType::TSTemplateLiteralType(_) => TypeRef::String,
        TSType::TSIndexedAccessType(_) => {
            diag.warn("Indexed access types are not supported, erasing to JsValue");
            TypeRef::Unresolved("indexed access type".to_string())
        }
        TSType::TSInferType(_) => {
            diag.warn("Infer types are not supported, erasing to JsValue");
            TypeRef::Unresolved("infer type".to_string())
        }
        TSType::TSTypeOperatorType(op) => match op.operator {
            TSTypeOperatorOperator::Readonly => {
                convert_ts_type_scoped(&op.type_annotation, scope, diag)
            }
            _ => {
                diag.warn_with_source(
                    "Type operator not supported, erasing to JsValue",
                    format!("{:?}", op.operator),
                );
                TypeRef::Unresolved("type operator".to_string())
            }
        },
        TSType::TSTypePredicate(_) => TypeRef::Boolean,
        TSType::TSTypeQuery(_) => {
            diag.warn("typeof type queries are not supported, erasing to JsValue");
            TypeRef::Unresolved("typeof query".to_string())
        }
        TSType::TSImportType(_) => {
            diag.warn("Import types are not supported, erasing to JsValue");
            TypeRef::Unresolved("import type".to_string())
        }
        TSType::TSThisType(_) => TypeRef::Unresolved("this".to_string()),

        // TSNamedTupleMember is also a direct variant of TSType via inherit_variants
        TSType::TSNamedTupleMember(member) => {
            // member.element_type is a TSTupleElement, convert it
            convert_tuple_element_scoped(&member.element_type, scope, diag)
        }

        TSType::TSParenthesizedType(paren) => {
            convert_ts_type_scoped(&paren.type_annotation, scope, diag)
        }

        // JSDoc types
        TSType::JSDocNullableType(nullable) => {
            let inner = convert_ts_type_scoped(&nullable.type_annotation, scope, diag);
            TypeRef::Nullable(Box::new(inner))
        }
        TSType::JSDocNonNullableType(non_nullable) => {
            convert_ts_type_scoped(&non_nullable.type_annotation, scope, diag)
        }
        TSType::JSDocUnknownType(_) => TypeRef::Any,
    }
}

/// Convert a type reference with type parameter scope.
fn convert_type_reference_scoped(
    type_ref: &TSTypeReference<'_>,
    scope: &TypeParamScope<'_>,
    diag: &mut DiagnosticCollector,
) -> TypeRef {
    let name = type_name_to_string(&type_ref.type_name);

    // If the name is an in-scope type parameter, erase to Any
    if scope.contains(name.as_str()) {
        return TypeRef::Any;
    }

    // Collect generic type arguments if present (field is `type_arguments` in oxc 0.118)
    let type_args: Vec<TypeRef> = type_ref
        .type_arguments
        .as_ref()
        .map(|args| {
            args.params
                .iter()
                .map(|t| convert_ts_type_scoped(t, scope, diag))
                .collect()
        })
        .unwrap_or_default();

    // Handle well-known generic types
    match name.as_str() {
        "Promise" | "PromiseLike" => {
            let inner = type_args.into_iter().next().unwrap_or(TypeRef::Any);
            TypeRef::Promise(Box::new(inner))
        }
        "Array" | "ReadonlyArray" => {
            let inner = type_args.into_iter().next().unwrap_or(TypeRef::Any);
            TypeRef::Array(Box::new(inner))
        }
        "Record" => {
            let mut args = type_args.into_iter();
            let key = args.next().unwrap_or(TypeRef::String);
            let value = args.next().unwrap_or(TypeRef::Any);
            TypeRef::Record(Box::new(key), Box::new(value))
        }
        "Map" | "ReadonlyMap" => {
            let mut args = type_args.into_iter();
            let key = args.next().unwrap_or(TypeRef::Any);
            let value = args.next().unwrap_or(TypeRef::Any);
            TypeRef::Map(Box::new(key), Box::new(value))
        }
        "Set" | "ReadonlySet" => {
            let inner = type_args.into_iter().next().unwrap_or(TypeRef::Any);
            TypeRef::Set(Box::new(inner))
        }

        "Int8Array" => TypeRef::Int8Array,
        "Uint8Array" => TypeRef::Uint8Array,
        "Uint8ClampedArray" => TypeRef::Uint8ClampedArray,
        "Int16Array" => TypeRef::Int16Array,
        "Uint16Array" => TypeRef::Uint16Array,
        "Int32Array" => TypeRef::Int32Array,
        "Uint32Array" => TypeRef::Uint32Array,
        "Float32Array" => TypeRef::Float32Array,
        "Float64Array" => TypeRef::Float64Array,
        "BigInt64Array" => TypeRef::BigInt64Array,
        "BigUint64Array" => TypeRef::BigUint64Array,
        "ArrayBuffer" | "SharedArrayBuffer" => TypeRef::ArrayBuffer,
        "ArrayBufferView" => TypeRef::ArrayBufferView,
        "DataView" => TypeRef::DataView,
        "Date" => TypeRef::Date,
        "RegExp" => TypeRef::RegExp,
        "Error" | "TypeError" | "RangeError" | "SyntaxError" | "DOMException" => TypeRef::Error,
        "Boolean" => TypeRef::Boolean,
        "Number" => TypeRef::Number,
        "String" => TypeRef::String,
        "Object" => TypeRef::Object,
        "Symbol" => TypeRef::Symbol,
        "Function" => TypeRef::Function(crate::ir::FunctionSig {
            params: vec![],
            return_type: Box::new(TypeRef::Any),
        }),
        "Partial"
        | "Required"
        | "Pick"
        | "Omit"
        | "Exclude"
        | "Extract"
        | "NonNullable"
        | "ReturnType"
        | "Parameters"
        | "ConstructorParameters"
        | "InstanceType"
        | "ThisParameterType"
        | "OmitThisParameter"
        | "ThisType"
        | "Awaited" => type_args.into_iter().next().unwrap_or(TypeRef::Object),
        _ => {
            if type_args.is_empty() {
                TypeRef::Named(name)
            } else {
                TypeRef::GenericInstantiation(name, type_args)
            }
        }
    }
}

/// Convert a `TSTypeName` to a string.
fn type_name_to_string(type_name: &TSTypeName<'_>) -> String {
    match type_name {
        TSTypeName::IdentifierReference(ident) => ident.name.to_string(),
        TSTypeName::QualifiedName(qualified) => {
            let left = type_name_to_string(&qualified.left);
            let right = &qualified.right.name;
            format!("{left}.{right}")
        }
        TSTypeName::ThisExpression(_) => "this".to_string(),
    }
}

/// Convert a tuple element to a `TypeRef`, with type parameter scope.
fn convert_tuple_element_scoped(
    elem: &TSTupleElement<'_>,
    scope: &TypeParamScope<'_>,
    diag: &mut DiagnosticCollector,
) -> TypeRef {
    match elem {
        TSTupleElement::TSNamedTupleMember(member) => {
            convert_tuple_element_scoped(&member.element_type, scope, diag)
        }
        TSTupleElement::TSRestType(rest) => {
            convert_ts_type_scoped(&rest.type_annotation, scope, diag)
        }
        TSTupleElement::TSOptionalType(opt) => {
            let inner = convert_ts_type_scoped(&opt.type_annotation, scope, diag);
            TypeRef::Nullable(Box::new(inner))
        }
        // All remaining variants are TSType variants flattened by inherit_variants!
        other => {
            if let Some(ts_type) = tuple_element_as_ts_type(other) {
                convert_ts_type_scoped(ts_type, scope, diag)
            } else {
                diag.warn("Unsupported tuple element type");
                TypeRef::Any
            }
        }
    }
}

/// Try to get a reference to the inner TSType from a TSTupleElement.
/// TSTupleElement inherits all TSType variants via `inherit_variants!`,
/// and provides `as_ts_type()` to access the underlying TSType.
fn tuple_element_as_ts_type<'a>(elem: &'a TSTupleElement<'a>) -> Option<&'a TSType<'a>> {
    elem.as_ts_type()
}

/// Convert a `TSFunctionType` to our IR `FunctionSig`, with type parameter scope.
fn convert_function_type_scoped(
    func: &TSFunctionType<'_>,
    scope: &TypeParamScope<'_>,
    diag: &mut DiagnosticCollector,
) -> crate::ir::FunctionSig {
    // Extend scope with this function's own type parameters
    let mut inner_scope = scope.clone();
    if let Some(tp) = &func.type_parameters {
        for p in &tp.params {
            inner_scope.insert(p.name.name.as_str());
        }
    }

    let params = convert_formal_params(&func.params, diag);
    let return_type = convert_ts_type_scoped(&func.return_type.type_annotation, &inner_scope, diag);

    crate::ir::FunctionSig {
        params,
        return_type: Box::new(return_type),
    }
}

/// Convert oxc `FormalParameters` to our IR `Param` list.
pub fn convert_formal_params(
    params: &FormalParameters<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<crate::ir::Param> {
    let mut result = Vec::new();
    for (i, param) in params.items.iter().enumerate() {
        let name = binding_pattern_name(&param.pattern)
            .map(|n| to_snake_case(&n))
            .unwrap_or_else(|| format!("arg{i}"));

        // In oxc 0.118, type_annotation and optional are on FormalParameter directly
        let type_ref = param
            .type_annotation
            .as_ref()
            .map(|ann| convert_ts_type(&ann.type_annotation, diag))
            .unwrap_or(TypeRef::Any);

        let optional = param.optional;

        result.push(crate::ir::Param {
            name,
            type_ref,
            optional,
            variadic: false,
        });
    }

    // Handle rest parameter
    if let Some(rest) = &params.rest {
        let name = binding_pattern_name(&rest.rest.argument).unwrap_or_else(|| "rest".to_string());

        let type_ref = rest
            .type_annotation
            .as_ref()
            .map(|ann| convert_ts_type(&ann.type_annotation, diag))
            .unwrap_or(TypeRef::Array(Box::new(TypeRef::Any)));

        result.push(crate::ir::Param {
            name,
            type_ref,
            optional: false,
            variadic: true,
        });
    }

    result
}

/// Extract a name from a binding pattern (only handles simple identifier patterns).
/// In oxc 0.118, BindingPattern is an enum directly.
fn binding_pattern_name(pattern: &BindingPattern<'_>) -> Option<String> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident.name.to_string()),
        _ => None,
    }
}

/// Convert a `TSLiteralType` to our IR `TypeRef`.
fn convert_literal_type(lit: &TSLiteralType<'_>, _diag: &mut DiagnosticCollector) -> TypeRef {
    match &lit.literal {
        TSLiteral::BooleanLiteral(b) => TypeRef::BooleanLiteral(b.value),
        TSLiteral::NumericLiteral(n) => TypeRef::NumberLiteral(n.value),
        TSLiteral::StringLiteral(s) => TypeRef::StringLiteral(s.value.to_string()),
        TSLiteral::BigIntLiteral(_) => TypeRef::BigInt,
        TSLiteral::TemplateLiteral(_) => TypeRef::String,
        TSLiteral::UnaryExpression(_) => TypeRef::Number,
    }
}

/// Simplify a union type.
fn simplify_union(types: Vec<TypeRef>) -> TypeRef {
    let mut non_null_types = Vec::new();
    let mut has_null = false;
    let mut has_undefined = false;

    for ty in types {
        match ty {
            TypeRef::Null => has_null = true,
            TypeRef::Undefined => has_undefined = true,
            TypeRef::Void => has_undefined = true,
            other => non_null_types.push(other),
        }
    }

    let core_type = if non_null_types.len() == 1 {
        non_null_types.pop().unwrap()
    } else if non_null_types.is_empty() {
        return TypeRef::Null;
    } else {
        TypeRef::Union(non_null_types)
    };

    if has_null || has_undefined {
        TypeRef::Nullable(Box::new(core_type))
    } else {
        core_type
    }
}

/// Convert `TSTypeParameterDeclaration` to our IR `TypeParam` list.
pub fn convert_type_params(
    type_params: Option<&oxc_allocator::Box<'_, TSTypeParameterDeclaration<'_>>>,
    diag: &mut DiagnosticCollector,
) -> Vec<crate::ir::TypeParam> {
    type_params
        .map(|tp| {
            tp.params
                .iter()
                .map(|p| crate::ir::TypeParam {
                    name: p.name.to_string(),
                    constraint: p.constraint.as_ref().map(|c| convert_ts_type(c, diag)),
                    default: p.default.as_ref().map(|d| convert_ts_type(d, diag)),
                })
                .collect()
        })
        .unwrap_or_default()
}
