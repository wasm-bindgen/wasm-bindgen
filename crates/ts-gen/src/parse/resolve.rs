//! Module specifier resolution using `oxc_resolver`.
//!
//! Resolves TypeScript module specifiers (e.g. `"node:buffer"`, `"es-module-lexer"`)
//! to `.d.ts` file paths on disk using the `types` export condition.

use std::path::{Path, PathBuf};

use oxc_resolver::{ResolveOptions, Resolver};

/// Create a resolver configured for TypeScript declaration resolution.
///
/// Uses the `types`, `import`, and `node` conditions to match how TypeScript
/// resolves module specifiers in declaration files.
fn create_resolver() -> Resolver {
    Resolver::new(ResolveOptions {
        condition_names: vec!["types".into(), "import".into(), "node".into()],
        extensions: vec![".d.ts".into(), ".ts".into()],
        main_fields: vec!["types".into(), "typings".into()],
        ..ResolveOptions::default()
    })
}

/// Attempt to resolve a module specifier to a `.d.ts` file path.
///
/// `base_dir` is the directory containing the file that has the import statement.
/// `oxc_resolver` finds `node_modules` automatically by walking up from `base_dir`.
///
/// Returns `None` if the file can't be found.
pub fn resolve_module(specifier: &str, base_dir: &Path) -> Option<PathBuf> {
    let resolver = create_resolver();

    // For node: builtins, resolve via @types/node
    let effective_specifier = if let Some(module_name) = specifier.strip_prefix("node:") {
        format!("@types/node/{module_name}")
    } else {
        specifier.to_string()
    };

    match resolver.resolve(base_dir, &effective_specifier) {
        Ok(resolution) => Some(resolution.into_path_buf()),
        Err(_) => None,
    }
}

/// Resolve a module specifier from a directory path (convenience for build scripts).
pub fn resolve(specifier: &str, from_dir: &Path) -> Option<PathBuf> {
    resolve_module(specifier, from_dir)
}
