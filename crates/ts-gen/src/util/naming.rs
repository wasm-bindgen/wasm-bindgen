//! JS name → Rust name conversion utilities.

use convert_case::{Case, Casing};

/// Convert a JS identifier to a Rust snake_case name (for functions, methods, variables).
///
/// Does NOT escape Rust keywords — that's handled by `make_ident` at the
/// codegen boundary using `r#` raw identifiers, so that name composition
/// (prefixes like `try_`, `set_`, suffixes like `_with_foo`) works correctly.
pub fn to_snake_case(name: &str) -> String {
    name.to_case(Case::Snake)
}

/// Convert a JS identifier to a Rust PascalCase name (for types, enums).
pub fn to_pascal_case(name: &str) -> String {
    name.to_case(Case::Pascal)
}

/// Convert a string literal to a PascalCase enum variant name.
/// Handles things like `"v8"` → `"V8"`, `"text"` → `"Text"`.
pub fn to_enum_variant(s: &str) -> String {
    // Special-case: if the string is all lowercase/digits, just PascalCase it
    let pascal = s.to_case(Case::Pascal);
    if pascal.is_empty() {
        "Empty".to_string()
    } else {
        pascal
    }
}

/// Deduplicate names in-place by appending `_2`, `_3`, etc. to collisions.
///
/// Takes a slice of `(name, setter)` pairs where `setter` is a closure that
/// updates the name on the original item. This avoids coupling to specific
/// enum variant types.
pub fn dedup_names(names: &mut [String]) {
    use std::collections::HashMap;
    let mut counts: HashMap<String, usize> = HashMap::new();
    for name in names.iter_mut() {
        let count = counts.entry(name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            *name = format!("{name}_{count}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("getUserById"), "get_user_by_id");
        assert_eq!(to_snake_case("HTMLElement"), "html_element");
        assert_eq!(to_snake_case("send"), "send");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("readableStream"), "ReadableStream");
        assert_eq!(to_pascal_case("my_type"), "MyType");
    }

    #[test]
    fn test_keywords_not_escaped() {
        // Keywords are NOT escaped by to_snake_case — make_ident handles
        // them with r# at the codegen boundary, after name composition.
        assert_eq!(to_snake_case("type"), "type");
        assert_eq!(to_snake_case("match"), "match");
        assert_eq!(to_snake_case("return"), "return");
        assert_eq!(to_snake_case("raw"), "raw");
    }

    #[test]
    fn test_enum_variant() {
        assert_eq!(to_enum_variant("text"), "Text");
        assert_eq!(to_enum_variant("bytes"), "Bytes");
        assert_eq!(to_enum_variant("json"), "Json");
    }

    #[test]
    fn test_dedup_names_no_collision() {
        let mut names = vec!["Foo".to_string(), "Bar".to_string(), "Baz".to_string()];
        dedup_names(&mut names);
        assert_eq!(names, &["Foo", "Bar", "Baz"]);
    }

    #[test]
    fn test_dedup_names_collision() {
        // "text-plain" and "textPlain" both produce "TextPlain"
        let mut names = vec![
            "TextPlain".to_string(),
            "TextPlain".to_string(),
            "Other".to_string(),
        ];
        dedup_names(&mut names);
        assert_eq!(names, &["TextPlain", "TextPlain_2", "Other"]);
    }

    #[test]
    fn test_dedup_names_triple_collision() {
        let mut names = vec!["A".to_string(), "A".to_string(), "A".to_string()];
        dedup_names(&mut names);
        assert_eq!(names, &["A", "A_2", "A_3"]);
    }
}
