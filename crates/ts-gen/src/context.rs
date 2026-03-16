//! Global context: owns all arenas and provides the central data store
//! for the entire parse → codegen pipeline.
//!
//! The `GlobalContext` is created once, mutated during parsing (Phase 1+2),
//! then borrowed immutably for codegen.

use crate::external_map::ExternalMap;
use crate::ir::{TypeDeclaration, TypeKind};
use crate::parse::scope::{PendingImport, ScopeArena, ScopeId};
use crate::util::diagnostics::DiagnosticCollector;

/// Well-typed index into the type arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeId(u32);

impl TypeId {
    /// Convert to a `usize` for indexing into slices.
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Well-typed index into the module registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleId(u32);

/// A parsed source file / module.
#[derive(Clone, Debug)]
pub struct ParsedModule {
    /// The module specifier (e.g. `"node:buffer"`, `"es-module-lexer"`).
    pub specifier: String,
    /// The file scope in the arena.
    pub scope: ScopeId,
    /// Type ids owned by this module.
    pub types: Vec<TypeId>,
}

/// The global data store shared across parsing and codegen.
///
/// Owns:
/// - `ScopeArena`: type scopes (file, namespace, module, builtin)
/// - Type arena: type definitions
/// - Module registry: parsed source files
/// - `ExternalMap`: external type mappings
/// - `DiagnosticCollector`: warnings and info messages
///
/// Arenas are append-only during parsing and read-only during codegen.
#[derive(Clone, Debug)]
pub struct GlobalContext {
    pub scopes: ScopeArena,
    pub diagnostics: DiagnosticCollector,
    pub external_map: ExternalMap,
    /// Pending imports that need to be resolved during import resolution.
    pub pending_imports: Vec<PendingImport>,
    types: Vec<TypeDeclaration>,
    modules: Vec<ParsedModule>,
    /// Map from module specifier to ModuleId for fast lookup.
    module_index: std::collections::HashMap<String, ModuleId>,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            scopes: ScopeArena::new(),
            diagnostics: DiagnosticCollector::new(),
            external_map: ExternalMap::new(),
            pending_imports: Vec::new(),
            types: Vec::new(),
            modules: Vec::new(),
            module_index: std::collections::HashMap::new(),
        }
    }

    /// Emit a warning diagnostic.
    pub fn warn(&mut self, message: impl Into<String>) {
        self.diagnostics.warn(message);
    }

    /// Emit an info diagnostic.
    pub fn info(&mut self, message: impl Into<String>) {
        self.diagnostics.info(message);
    }

    // ─── Scope operations (delegate to ScopeArena) ───────────────────

    /// Create the root (builtin) scope.
    pub fn create_root_scope(&mut self) -> ScopeId {
        self.scopes.create_root()
    }

    /// Create a child scope.
    pub fn create_child_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.create_child(parent)
    }

    // ─── Module registry operations ────────────────────────────────

    /// Register a parsed module, returning its `ModuleId`.
    pub fn register_module(&mut self, specifier: String, scope: ScopeId) -> ModuleId {
        let id = ModuleId(self.modules.len() as u32);
        self.module_index.insert(specifier.clone(), id);
        self.modules.push(ParsedModule {
            specifier,
            scope,
            types: Vec::new(),
        });
        id
    }

    /// Look up a module by specifier.
    pub fn find_module(&self, specifier: &str) -> Option<ModuleId> {
        self.module_index.get(specifier).copied()
    }

    /// Get a reference to a parsed module by `ModuleId`.
    pub fn get_module(&self, id: ModuleId) -> &ParsedModule {
        &self.modules[id.0 as usize]
    }

    /// Get a mutable reference to a parsed module by `ModuleId`.
    pub fn get_module_mut(&mut self, id: ModuleId) -> &mut ParsedModule {
        &mut self.modules[id.0 as usize]
    }

    // ─── Type arena operations ───────────────────────────────────────

    /// Insert a declaration into the type arena, returning its `TypeId`.
    pub fn insert_type(&mut self, decl: TypeDeclaration) -> TypeId {
        let id = TypeId(self.types.len() as u32);
        self.types.push(decl);
        id
    }

    /// Get a reference to a declaration by `TypeId`.
    pub fn get_type(&self, id: TypeId) -> &TypeDeclaration {
        &self.types[id.index()]
    }

    /// Read-only access to the full type arena slice.
    pub fn type_arena(&self) -> &[TypeDeclaration] {
        &self.types
    }

    /// Resolve a dotted type path like `"NodeJS.TypedArray"` through namespace scopes.
    ///
    /// Splits on `.`, resolves the first segment in the starting scope, then
    /// follows `Namespace` declarations for subsequent segments.
    pub fn resolve_path(&self, scope: ScopeId, path: &str) -> Option<TypeId> {
        let mut segments = path.split('.');
        let first = segments.next()?;

        let mut current_id = self.scopes.resolve(scope, first)?;

        for segment in segments {
            let decl = self.get_type(current_id);
            match &decl.kind {
                TypeKind::Namespace(ns) => {
                    current_id = self.scopes.resolve(ns.child_scope, segment)?;
                }
                _ => return None, // Non-namespace in the middle of a path
            }
        }

        Some(current_id)
    }

    /// Iterate all types in the arena.
    pub fn iter_types(&self) -> impl Iterator<Item = (TypeId, &TypeDeclaration)> {
        self.types
            .iter()
            .enumerate()
            .map(|(i, d)| (TypeId(i as u32), d))
    }
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self::new()
    }
}
