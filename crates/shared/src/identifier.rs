/// Returns whether a character has the Unicode `ID_Start` properly.
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
        _ => unicode_ident::is_xid_start(c),
    }
}

/// Returns whether a character has the Unicode `ID_Continue` properly.
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
        _ => unicode_ident::is_xid_continue(c),
    }
}

/// Returns whether a string is a valid JavaScript identifier.
/// Defined at https://tc39.es/ecma262/#prod-IdentifierName.
pub fn is_valid_ident(name: &str) -> bool {
    !name.is_empty()
        && name.chars().enumerate().all(|(i, char)| {
            if i == 0 {
                is_id_start(char) || char == '$' || char == '_'
            } else {
                is_id_continue(char) || char == '$' || char == '\u{200C}' || char == '\u{200D}'
            }
        })
}

/// An arbitrary code for describing a type map
/// This is hash of `TYPE_DESCRIBE_MAP` truncated to u32
/// using `std::hash::DefaultHasher`
pub const TYPE_DESCRIBE_MAP: u32 = 2728578646;
