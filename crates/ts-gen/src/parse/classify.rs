//! Interface classification: determine if an interface is class-like or a dictionary.

use crate::ir::*;

/// Classify an interface based on its members.
///
/// - **Dictionary**: Only has properties (getters/setters), no methods — gets
///   builder pattern and property accessors. Includes interfaces with required
///   properties (the builder validates them at runtime).
/// - **ClassLike**: Has methods, constructors, or static members
/// - **Unclassified**: Empty or unclear
pub fn classify_interface(members: &[Member]) -> InterfaceClassification {
    if members.is_empty() {
        return InterfaceClassification::Unclassified;
    }

    let mut has_methods = false;
    let mut has_properties = false;

    for member in members {
        match member {
            Member::Method(_) | Member::StaticMethod(_) => {
                has_methods = true;
            }
            Member::Constructor(_) => {
                has_methods = true;
            }
            Member::Getter(_) | Member::Setter(_) => {
                has_properties = true;
            }
            Member::StaticGetter(_) | Member::StaticSetter(_) => {
                has_methods = true;
            }
            Member::IndexSignature(_) => {}
        }
    }

    if has_methods {
        InterfaceClassification::ClassLike
    } else if has_properties {
        InterfaceClassification::Dictionary
    } else {
        InterfaceClassification::Unclassified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn getter(name: &str, optional: bool) -> Member {
        Member::Getter(GetterMember {
            js_name: name.to_string(),
            type_ref: TypeRef::Any,
            optional,
            doc: None,
        })
    }

    fn static_getter(name: &str) -> Member {
        Member::StaticGetter(StaticGetterMember {
            js_name: name.to_string(),
            type_ref: TypeRef::Any,
            doc: None,
        })
    }

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

    #[test]
    fn empty_is_unclassified() {
        assert_eq!(
            classify_interface(&[]),
            InterfaceClassification::Unclassified
        );
    }

    #[test]
    fn all_optional_getters_is_dictionary() {
        let members = vec![getter("foo", true), getter("bar", true)];
        assert_eq!(
            classify_interface(&members),
            InterfaceClassification::Dictionary
        );
    }

    #[test]
    fn required_getter_is_dictionary() {
        // Properties-only interfaces are dictionaries regardless of optionality.
        // The builder validates required properties at runtime.
        let members = vec![getter("foo", false)];
        assert_eq!(
            classify_interface(&members),
            InterfaceClassification::Dictionary
        );
    }

    #[test]
    fn methods_is_class_like() {
        let members = vec![method("do_thing")];
        assert_eq!(
            classify_interface(&members),
            InterfaceClassification::ClassLike
        );
    }

    #[test]
    fn static_getter_is_class_like() {
        let members = vec![static_getter("instance")];
        assert_eq!(
            classify_interface(&members),
            InterfaceClassification::ClassLike
        );
    }

    #[test]
    fn optional_getters_with_static_is_class_like() {
        // Regression: static properties should prevent Dictionary classification
        let members = vec![getter("foo", true), static_getter("bar")];
        assert_eq!(
            classify_interface(&members),
            InterfaceClassification::ClassLike
        );
    }
}
