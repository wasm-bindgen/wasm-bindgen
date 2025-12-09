//! Normalizes Rust compiler-generated hash suffixes in WASM exports.

use anyhow::Result;
use std::collections::HashMap;
use walrus::{ExportItem, Module};

/// Known prefixes for wasm-bindgen internal exports that may have hash suffixes.
/// Only exports with these prefixes will be normalized to avoid accidentally
/// renaming user exports that happen to end with a hash-like pattern.
const INTERNAL_PREFIXES: &[&str] = &[
    "wasm_bindgen__convert__closures_____invoke__",
    "wasm_bindgen__closure__destroy__",
];

/// Check if an export name is a wasm-bindgen internal with a hash suffix.
/// Returns the prefix if it matches, None otherwise.
fn is_internal_export_with_hash(name: &str) -> Option<&'static str> {
    for &prefix in INTERNAL_PREFIXES {
        if let Some(suffix) = name.strip_prefix(prefix) {
            // Check if suffix matches h[0-9a-f]{16} exactly
            if suffix.len() == 17
                && suffix.starts_with('h')
                && suffix[1..].chars().all(|c| c.is_ascii_hexdigit())
            {
                return Some(prefix);
            }
        }
    }
    None
}

/// Normalizes exports with compiler-generated hash suffixes.
///
/// Only wasm-bindgen internal exports (with known prefixes) are normalized.
/// Exports like `wasm_bindgen__closure__destroy__h7a3b9c2d1e4f5678` are renamed
/// to `wasm_bindgen__closure__destroy__h0000000000000000`, etc., based on sorting
pub fn normalize_exports(module: &mut Module) -> Result<()> {
    let mut to_normalize: Vec<_> = module
        .exports
        .iter()
        .filter_map(|export| {
            let ExportItem::Function(func_id) = export.item else {
                return None;
            };
            let prefix = is_internal_export_with_hash(&export.name)?;
            let type_id = module.funcs.get(func_id).ty();
            Some((export.id(), prefix, type_id))
        })
        .collect();

    // Sort by (prefix, function_type) for deterministic ordering.
    to_normalize.sort_by(|(_, prefix_a, ty_a), (_, prefix_b, ty_b)| {
        (prefix_a, module.types.get(*ty_a)).cmp(&(prefix_b, module.types.get(*ty_b)))
    });

    // Assign normalized sequential hashes
    let mut counters: HashMap<&'static str, usize> = HashMap::new();
    for (export_id, prefix, _) in to_normalize {
        let counter = counters.entry(prefix).or_insert(0);
        module.exports.get_mut(export_id).name = format!("{prefix}h{:016x}", *counter);
        *counter += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_internal_export_with_hash;

    #[test]
    fn test_internal_export_detection() {
        // Should match internal exports with valid hash suffixes
        assert!(is_internal_export_with_hash(
            "wasm_bindgen__convert__closures_____invoke__h0123456789abcdef"
        )
        .is_some());
        assert!(
            is_internal_export_with_hash("wasm_bindgen__closure__destroy__he0c82e5427fd1a46")
                .is_some()
        );

        // Should NOT match user exports (no known prefix)
        assert!(is_internal_export_with_hash("compute_h0123456789abcdef").is_none());
        assert!(is_internal_export_with_hash("foo__h0123456789abcdef").is_none());

        // Should NOT match if hash is wrong length
        assert!(is_internal_export_with_hash("wasm_bindgen__closure__destroy__h123").is_none());
        assert!(
            is_internal_export_with_hash("wasm_bindgen__closure__destroy__h0123456789abcdefg")
                .is_none()
        );
    }
}
