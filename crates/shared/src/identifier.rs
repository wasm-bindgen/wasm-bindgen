use alloc::string::String;

/// Returns whether a character is a valid JS identifier start character.
///
/// This is only ever-so-slightly different from `XID_Start` in a few edge
/// cases, so we handle those edge cases manually and delegate everything else
/// to `unicode-ident`.
fn is_id_start(c: char) -> bool {
    match c {
        '\u{037A}' | '\u{0E33}' | '\u{0EB3}' | '\u{309B}' | '\u{309C}' | '\u{FC5E}'
        | '\u{FC5F}' | '\u{FC60}' | '\u{FC61}' | '\u{FC62}' | '\u{FC63}' | '\u{FDFA}'
        | '\u{FDFB}' | '\u{FE70}' | '\u{FE72}' | '\u{FE74}' | '\u{FE76}' | '\u{FE78}'
        | '\u{FE7A}' | '\u{FE7C}' | '\u{FE7E}' | '\u{FF9E}' | '\u{FF9F}' => true,
        '$' | '_' => true,
        _ => unicode_ident::is_xid_start(c),
    }
}

/// Returns whether a character is a valid JS identifier continue character.
///
/// This is only ever-so-slightly different from `XID_Continue` in a few edge
/// cases, so we handle those edge cases manually and delegate everything else
/// to `unicode-ident`.
fn is_id_continue(c: char) -> bool {
    match c {
        '\u{037A}' | '\u{309B}' | '\u{309C}' | '\u{FC5E}' | '\u{FC5F}' | '\u{FC60}'
        | '\u{FC61}' | '\u{FC62}' | '\u{FC63}' | '\u{FDFA}' | '\u{FDFB}' | '\u{FE70}'
        | '\u{FE72}' | '\u{FE74}' | '\u{FE76}' | '\u{FE78}' | '\u{FE7A}' | '\u{FE7C}'
        | '\u{FE7E}' => true,
        '$' | '\u{200C}' | '\u{200D}' => true,
        _ => unicode_ident::is_xid_continue(c),
    }
}

fn maybe_valid_chars(name: &str) -> impl Iterator<Item = Option<char>> + '_ {
    let mut chars = name.chars();
    // Always emit at least one `None` item - that way `is_valid_ident` can fail without
    // a separate check for empty strings, and `to_valid_ident` will always produce at least
    // one underscore.
    core::iter::once(chars.next().filter(|&c| is_id_start(c))).chain(chars.map(|c| {
        if is_id_continue(c) {
            Some(c)
        } else {
            None
        }
    }))
}

static RESERVED_WORDS: &[&str] = &[
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "enum",
    "await",
    "implements",
    "interface",
    "let",
    "package",
    "private",
    "protected",
    "public",
    "static",
    "null",
    "true",
    "false",
];

/// Returns whether a string is a valid JavaScript identifier.
/// Defined at https://tc39.es/ecma262/#prod-IdentifierName.
pub fn is_reserved_ident(name: &str) -> bool {
    RESERVED_WORDS.contains(&name)
}

/// Returns whether a string is a valid JavaScript identifier.
/// Defined at https://tc39.es/ecma262/#prod-IdentifierName.
pub fn is_valid_ident(name: &str) -> bool {
    maybe_valid_chars(name).all(|opt| opt.is_some())
}

/// Converts a string to a valid JavaScript identifier by replacing invalid
/// characters with underscores.
pub fn to_valid_ident(name: &str) -> String {
    let result: String = maybe_valid_chars(name)
        .map(|opt| opt.unwrap_or('_'))
        .collect();

    if is_reserved_ident(&result) {
        let mut prefixed = String::with_capacity(result.len() + 1);
        prefixed.push('_');
        prefixed.push_str(&result);
        prefixed
    } else {
        result
    }
}
