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

/// Returns whether a string is a valid JavaScript identifier.
/// Defined at https://tc39.es/ecma262/#prod-IdentifierName.
pub fn is_valid_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if is_id_start(c) => {}
        _ => return false,
    }
    chars.all(|c| is_id_continue(c))
}

/// Converts a string to a valid JavaScript identifier by replacing invalid
/// characters with underscores.
pub fn to_valid_ident(name: &str) -> String {
    let mut chars = name.chars();
    let mut result = String::new();
    match chars.next() {
        Some(c) if is_id_start(c) => result.push(c),
        Some(_) => result.push('_'),
        None => return "_".into(),
    }
    result.extend(chars.map(|c| if is_id_continue(c) { c } else { '_' }));
    result
}
