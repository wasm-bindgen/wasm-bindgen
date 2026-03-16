//! External type mapping: resolve imported types to Rust paths.
//!
//! When an imported type can't be resolved by parsing its source file,
//! the external map provides the Rust path to use instead.
//!
//! ## CLI format
//!
//! ```text
//! --external "node:*=node_sys::*"              # wildcard module mapping
//! --external "Blob=::web_sys::Blob"            # explicit type mapping
//! --external "node:buffer=node_buffer_sys"     # specific module mapping
//! ```
//!
//! Multiple mappings can be comma-separated or specified with multiple flags.

use std::collections::HashMap;

/// A resolved Rust path for an external type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustPath {
    /// The full Rust path (e.g. `::web_sys::Blob`, `node_sys::buffer::Blob`)
    pub path: String,
}

/// Maps TypeScript module specifiers and type names to Rust paths.
#[derive(Clone, Debug, Default)]
pub struct ExternalMap {
    /// Explicit type mappings: `"Blob" → "::web_sys::Blob"`
    type_map: HashMap<String, String>,
    /// Module mappings: `"node:buffer" → "node_buffer_sys"`
    module_map: HashMap<String, String>,
    /// Wildcard module mappings: `"node:" → "node_sys"`
    /// Stored as (prefix, rust_crate) pairs.
    wildcard_map: Vec<(String, String)>,
}

impl ExternalMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a CLI external mapping string.
    ///
    /// Format: `"LHS=RHS"` where:
    /// - `"Blob=::web_sys::Blob"` → explicit type map
    /// - `"node:buffer=node_buffer_sys"` → module map
    /// - `"node:*=node_sys::*"` → wildcard module map
    pub fn add_mapping(&mut self, mapping: &str) {
        let Some((lhs, rhs)) = mapping.split_once('=') else {
            return;
        };
        let lhs = lhs.trim();
        let rhs = rhs.trim();

        if lhs.ends_with('*') && rhs.ends_with('*') {
            // Wildcard: "node:*=node_sys::*"
            let prefix = lhs.trim_end_matches('*');
            let rust_prefix = rhs.trim_end_matches('*').trim_end_matches("::");
            self.wildcard_map
                .push((prefix.to_string(), rust_prefix.to_string()));
            // Sort by prefix length descending so longer (more specific) prefixes match first
            self.wildcard_map
                .sort_by_key(|b| std::cmp::Reverse(b.0.len()));
        } else if lhs.contains(':') || lhs.contains('/') {
            // Module mapping: "node:buffer=node_buffer_sys"
            self.module_map.insert(lhs.to_string(), rhs.to_string());
        } else {
            // Explicit type: "Blob=::web_sys::Blob"
            self.type_map.insert(lhs.to_string(), rhs.to_string());
        }
    }

    /// Parse multiple mappings from a comma-separated string.
    pub fn add_mappings(&mut self, mappings: &str) {
        for mapping in mappings.split(',') {
            let mapping = mapping.trim();
            if !mapping.is_empty() {
                self.add_mapping(mapping);
            }
        }
    }

    /// Resolve a type name imported from a module to a Rust path.
    ///
    /// Returns `None` if no mapping exists (caller should fall back to JsValue).
    pub fn resolve(&self, type_name: &str, from_module: &str) -> Option<RustPath> {
        // 1. Explicit type map (highest priority)
        if let Some(rust_path) = self.type_map.get(type_name) {
            return Some(RustPath {
                path: rust_path.clone(),
            });
        }

        // 2. Specific module map
        if let Some(rust_crate) = self.module_map.get(from_module) {
            return Some(RustPath {
                path: format!("{rust_crate}::{type_name}"),
            });
        }

        // 3. Wildcard module map
        for (prefix, rust_crate) in &self.wildcard_map {
            if from_module.starts_with(prefix) {
                let module_suffix = &from_module[prefix.len()..];
                let rust_module = module_suffix.replace('/', "::");
                if rust_module.is_empty() {
                    return Some(RustPath {
                        path: format!("{rust_crate}::{type_name}"),
                    });
                } else {
                    return Some(RustPath {
                        path: format!("{rust_crate}::{rust_module}::{type_name}"),
                    });
                }
            }
        }

        None
    }

    /// Resolve a type name without a known module (explicit type map only).
    pub fn resolve_type(&self, type_name: &str) -> Option<RustPath> {
        self.type_map.get(type_name).map(|rust_path| RustPath {
            path: rust_path.clone(),
        })
    }

    /// Check if any mappings have been configured.
    pub fn is_empty(&self) -> bool {
        self.type_map.is_empty() && self.module_map.is_empty() && self.wildcard_map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_type_mapping() {
        let mut map = ExternalMap::new();
        map.add_mapping("Blob=::web_sys::Blob");

        let result = map.resolve("Blob", "node:buffer");
        assert_eq!(result.unwrap().path, "::web_sys::Blob");

        // Unknown type returns None
        assert!(map.resolve("Unknown", "node:buffer").is_none());
    }

    #[test]
    fn test_module_mapping() {
        let mut map = ExternalMap::new();
        map.add_mapping("node:buffer=node_buffer_sys");

        let result = map.resolve("Blob", "node:buffer");
        assert_eq!(result.unwrap().path, "node_buffer_sys::Blob");

        // Different module not mapped
        assert!(map.resolve("Foo", "node:http").is_none());
    }

    #[test]
    fn test_wildcard_mapping() {
        let mut map = ExternalMap::new();
        map.add_mapping("node:*=node_sys::*");

        let result = map.resolve("Blob", "node:buffer");
        assert_eq!(result.unwrap().path, "node_sys::buffer::Blob");

        let result2 = map.resolve("Server", "node:http");
        assert_eq!(result2.unwrap().path, "node_sys::http::Server");
    }

    #[test]
    fn test_explicit_overrides_wildcard() {
        let mut map = ExternalMap::new();
        map.add_mapping("node:*=node_sys::*");
        map.add_mapping("Blob=::web_sys::Blob");

        // Explicit wins
        let result = map.resolve("Blob", "node:buffer");
        assert_eq!(result.unwrap().path, "::web_sys::Blob");

        // Wildcard for non-explicit types
        let result2 = map.resolve("Buffer", "node:buffer");
        assert_eq!(result2.unwrap().path, "node_sys::buffer::Buffer");
    }

    #[test]
    fn test_comma_separated() {
        let mut map = ExternalMap::new();
        map.add_mappings("Blob=::web_sys::Blob, node:*=node_sys::*");

        assert_eq!(
            map.resolve("Blob", "node:buffer").unwrap().path,
            "::web_sys::Blob"
        );
        assert_eq!(
            map.resolve("Server", "node:http").unwrap().path,
            "node_sys::http::Server"
        );
    }

    #[test]
    fn test_subpath_wildcard() {
        let mut map = ExternalMap::new();
        map.add_mapping("node:*=node_sys::*");

        let result = map.resolve("ReadableStream", "node:stream/web");
        assert_eq!(
            result.unwrap().path,
            "node_sys::stream::web::ReadableStream"
        );
    }
}
