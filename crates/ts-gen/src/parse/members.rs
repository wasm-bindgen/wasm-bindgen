//! Convert oxc class/interface member AST nodes to our IR `Member`.

use oxc_ast::ast::*;

use std::collections::HashSet;

use crate::ir::*;
use crate::parse::docs::DocComments;
use crate::parse::types::{
    convert_formal_params, convert_ts_type, convert_ts_type_scoped, convert_type_params,
};
use crate::util::diagnostics::DiagnosticCollector;

/// Convert a `TSSignature` (interface body member) to our IR `Member`(s).
///
/// Returns zero or more members. Plain properties produce both a getter and
/// setter; explicit `get`/`set` accessors produce one each.
pub fn convert_ts_signature(
    sig: &TSSignature<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    match sig {
        TSSignature::TSPropertySignature(prop) => convert_property_signature(prop, docs, diag),
        TSSignature::TSMethodSignature(method) => convert_method_signature(method, docs, diag),
        TSSignature::TSIndexSignature(idx) => {
            convert_index_signature(idx, diag).into_iter().collect()
        }
        TSSignature::TSConstructSignatureDeclaration(ctor) => {
            convert_construct_signature(ctor, docs, diag)
                .into_iter()
                .collect()
        }
        TSSignature::TSCallSignatureDeclaration(_) => {
            diag.warn("Call signatures on interfaces are not supported, skipping");
            vec![]
        }
    }
}

/// Convert a `ClassElement` (class body member) to our IR `Member`(s).
///
/// Returns zero or more members. Plain properties produce both a getter and
/// setter; explicit `get`/`set` accessors produce one each.
pub fn convert_class_element(
    elem: &ClassElement<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    match elem {
        ClassElement::MethodDefinition(method) => convert_class_method(method, docs, diag),
        ClassElement::PropertyDefinition(prop) => convert_class_property(prop, docs, diag),
        ClassElement::AccessorProperty(acc) => convert_accessor_property(acc, docs, diag),
        ClassElement::TSIndexSignature(idx) => {
            convert_index_signature(idx, diag).into_iter().collect()
        }
        ClassElement::StaticBlock(_) => vec![],
    }
}

// ─── Interface member conversions ────────────────────────────────────

fn convert_property_signature(
    prop: &TSPropertySignature<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    let js_name = match property_key_name(&prop.key) {
        Some(n) => n,
        None => return vec![],
    };
    let doc = docs.for_span(prop.span.start);

    let type_ref = prop
        .type_annotation
        .as_ref()
        .map(|ann| convert_ts_type(&ann.type_annotation, diag))
        .unwrap_or(TypeRef::Any);

    let mut members = vec![Member::Getter(GetterMember {
        js_name: js_name.clone(),
        type_ref: type_ref.clone(),
        optional: prop.optional,
        doc,
    })];

    if !prop.readonly {
        members.push(Member::Setter(SetterMember {
            js_name,
            type_ref,
            doc: None,
        }));
    }

    members
}

fn convert_method_signature(
    method: &TSMethodSignature<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    let js_name = match property_key_name(&method.key) {
        Some(n) => n,
        None => return vec![],
    };
    let doc = docs.for_span(method.span.start);

    let type_params = convert_type_params(method.type_parameters.as_ref(), diag);

    // Build scope from method type parameters so references like `T` in
    // `json<T>(): Promise<T>` get erased to Any instead of Named("T")
    let scope: HashSet<&str> = method
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.name.as_str()).collect())
        .unwrap_or_default();

    let params = convert_formal_params(&method.params, diag);
    let return_type = method
        .return_type
        .as_ref()
        .map(|rt| convert_ts_type_scoped(&rt.type_annotation, &scope, diag))
        .unwrap_or(TypeRef::Void);

    match method.kind {
        TSMethodSignatureKind::Get => vec![Member::Getter(GetterMember {
            js_name,
            type_ref: return_type,
            optional: method.optional,
            doc,
        })],
        TSMethodSignatureKind::Set => {
            let type_ref = params
                .into_iter()
                .next()
                .map(|p| p.type_ref)
                .unwrap_or(TypeRef::Any);
            vec![Member::Setter(SetterMember {
                js_name,
                type_ref,
                doc,
            })]
        }
        TSMethodSignatureKind::Method => vec![Member::Method(MethodMember {
            name: crate::util::naming::to_snake_case(&js_name),
            js_name,
            type_params,
            params,
            return_type,
            optional: method.optional,
            doc,
        })],
    }
}

fn convert_index_signature(
    idx: &TSIndexSignature<'_>,
    diag: &mut DiagnosticCollector,
) -> Option<Member> {
    let key_type = idx
        .parameters
        .first()
        .map(|p| convert_ts_type(&p.type_annotation.type_annotation, diag))
        .unwrap_or(TypeRef::String);

    // type_annotation is Box<TSTypeAnnotation> (not Option) in oxc 0.118
    let value_type = convert_ts_type(&idx.type_annotation.type_annotation, diag);

    Some(Member::IndexSignature(IndexSigMember {
        key_type,
        value_type,
        readonly: idx.readonly,
    }))
}

fn convert_construct_signature(
    ctor: &TSConstructSignatureDeclaration<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Option<Member> {
    let params = convert_formal_params(&ctor.params, diag);
    let doc = docs.for_span(ctor.span.start);
    Some(Member::Constructor(ConstructorMember { params, doc }))
}

// ─── Class member conversions ────────────────────────────────────────

fn convert_class_method(
    method: &MethodDefinition<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    let js_name = match property_key_name(&method.key) {
        Some(n) => n,
        None => return vec![],
    };
    let doc = docs.for_span(method.span.start);

    let func = &method.value;
    let type_params = convert_type_params(func.type_parameters.as_ref(), diag);

    // Build scope from method type parameters
    let scope: HashSet<&str> = func
        .type_parameters
        .as_ref()
        .map(|tp| tp.params.iter().map(|p| p.name.name.as_str()).collect())
        .unwrap_or_default();

    let params = convert_formal_params(&func.params, diag);
    let return_type = func
        .return_type
        .as_ref()
        .map(|rt| convert_ts_type_scoped(&rt.type_annotation, &scope, diag))
        .unwrap_or(TypeRef::Void);

    let is_static = method.r#static;

    match method.kind {
        MethodDefinitionKind::Constructor => {
            vec![Member::Constructor(ConstructorMember { params, doc })]
        }
        MethodDefinitionKind::Get => {
            if is_static {
                vec![Member::StaticGetter(StaticGetterMember {
                    js_name,
                    type_ref: return_type,
                    doc,
                })]
            } else {
                vec![Member::Getter(GetterMember {
                    js_name,
                    type_ref: return_type,
                    optional: method.optional,
                    doc,
                })]
            }
        }
        MethodDefinitionKind::Set => {
            let type_ref = params
                .into_iter()
                .next()
                .map(|p| p.type_ref)
                .unwrap_or(TypeRef::Any);
            if is_static {
                vec![Member::StaticSetter(StaticSetterMember {
                    js_name,
                    type_ref,
                    doc,
                })]
            } else {
                vec![Member::Setter(SetterMember {
                    js_name,
                    type_ref,
                    doc,
                })]
            }
        }
        MethodDefinitionKind::Method => {
            if is_static {
                vec![Member::StaticMethod(StaticMethodMember {
                    name: crate::util::naming::to_snake_case(&js_name),
                    js_name,
                    type_params,
                    params,
                    return_type,
                    doc,
                })]
            } else {
                vec![Member::Method(MethodMember {
                    name: crate::util::naming::to_snake_case(&js_name),
                    js_name,
                    type_params,
                    params,
                    return_type,
                    optional: method.optional,
                    doc,
                })]
            }
        }
    }
}

fn convert_class_property(
    prop: &PropertyDefinition<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    let js_name = match property_key_name(&prop.key) {
        Some(n) => n,
        None => return vec![],
    };
    let doc = docs.for_span(prop.span.start);

    let type_ref = prop
        .type_annotation
        .as_ref()
        .map(|ann| convert_ts_type(&ann.type_annotation, diag))
        .unwrap_or(TypeRef::Any);

    if prop.r#static {
        let mut members = vec![Member::StaticGetter(StaticGetterMember {
            js_name: js_name.clone(),
            type_ref: type_ref.clone(),
            doc,
        })];
        if !prop.readonly {
            members.push(Member::StaticSetter(StaticSetterMember {
                js_name,
                type_ref,
                doc: None,
            }));
        }
        members
    } else {
        let mut members = vec![Member::Getter(GetterMember {
            js_name: js_name.clone(),
            type_ref: type_ref.clone(),
            optional: prop.optional,
            doc,
        })];
        if !prop.readonly {
            members.push(Member::Setter(SetterMember {
                js_name,
                type_ref,
                doc: None,
            }));
        }
        members
    }
}

fn convert_accessor_property(
    acc: &AccessorProperty<'_>,
    docs: &DocComments<'_>,
    diag: &mut DiagnosticCollector,
) -> Vec<Member> {
    let js_name = match property_key_name(&acc.key) {
        Some(n) => n,
        None => return vec![],
    };
    let doc = docs.for_span(acc.span.start);

    let type_ref = acc
        .type_annotation
        .as_ref()
        .map(|ann| convert_ts_type(&ann.type_annotation, diag))
        .unwrap_or(TypeRef::Any);

    if acc.r#static {
        vec![
            Member::StaticGetter(StaticGetterMember {
                js_name: js_name.clone(),
                type_ref: type_ref.clone(),
                doc,
            }),
            Member::StaticSetter(StaticSetterMember {
                js_name,
                type_ref,
                doc: None,
            }),
        ]
    } else {
        vec![
            Member::Getter(GetterMember {
                js_name: js_name.clone(),
                type_ref: type_ref.clone(),
                optional: false,
                doc,
            }),
            Member::Setter(SetterMember {
                js_name,
                type_ref,
                doc: None,
            }),
        ]
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────

/// Extract a string name from a `PropertyKey`.
pub fn property_key_name(key: &PropertyKey<'_>) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(ident) => Some(ident.name.to_string()),
        PropertyKey::StringLiteral(s) => Some(s.value.to_string()),
        PropertyKey::NumericLiteral(n) => Some(n.value.to_string()),
        _ => None,
    }
}
