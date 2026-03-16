//! TypeScript type scope tracking with arena-based ownership.
//!
//! Each scope maps type names to `TypeId`s — indices into the global type arena.
//! Scopes form a tree via parent links, enabling JS-like scoping where child
//! scopes shadow or extend parent scopes.
//!
//! Scopes are stored in a flat arena (`ScopeArena`) and referenced by
//! well-typed `ScopeId` indices.

use std::collections::HashMap;

use crate::context::TypeId;

/// Index into a `ScopeArena`. Lightweight, Copy, and well-typed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScopeId(pub(crate) u32);

/// A single level of type scope.
#[derive(Clone, Debug)]
pub struct TypeScope {
    /// Optional parent scope — resolution walks up the chain.
    pub parent: Option<ScopeId>,
    /// Types defined or imported in this scope: name → TypeId.
    names: HashMap<String, TypeId>,
}

impl TypeScope {
    fn new(parent: Option<ScopeId>) -> Self {
        Self {
            parent,
            names: HashMap::new(),
        }
    }

    /// Insert a name into this scope.
    pub fn insert(&mut self, name: String, type_id: TypeId) {
        self.names.insert(name, type_id);
    }

    /// Look up a name in this scope only (not parent scopes).
    pub fn get(&self, name: &str) -> Option<TypeId> {
        self.names.get(name).copied()
    }

    /// Iterate over all names in this scope.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &TypeId)> {
        self.names.iter()
    }
}

/// Arena that owns all scopes. Scopes are created via `create_root` and
/// `create_child`, and referenced by `ScopeId`.
#[derive(Clone, Debug)]
pub struct ScopeArena {
    scopes: Vec<TypeScope>,
}

impl ScopeArena {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    /// Create a root scope (no parent).
    pub fn create_root(&mut self) -> ScopeId {
        let id = ScopeId(self.scopes.len() as u32);
        self.scopes.push(TypeScope::new(None));
        id
    }

    /// Create a child scope with the given parent.
    pub fn create_child(&mut self, parent: ScopeId) -> ScopeId {
        let id = ScopeId(self.scopes.len() as u32);
        self.scopes.push(TypeScope::new(Some(parent)));
        id
    }

    /// Get a reference to a scope by id.
    pub fn get(&self, id: ScopeId) -> &TypeScope {
        &self.scopes[id.0 as usize]
    }

    /// Get a mutable reference to a scope by id.
    pub fn get_mut(&mut self, id: ScopeId) -> &mut TypeScope {
        &mut self.scopes[id.0 as usize]
    }

    /// Insert a name into the given scope.
    pub fn insert(&mut self, scope: ScopeId, name: String, type_id: TypeId) {
        self.get_mut(scope).insert(name, type_id);
    }

    /// Resolve a simple name by walking up the scope chain from the given scope.
    pub fn resolve(&self, scope: ScopeId, name: &str) -> Option<TypeId> {
        let s = self.get(scope);
        if let Some(type_id) = s.get(name) {
            return Some(type_id);
        }
        if let Some(parent) = s.parent {
            return self.resolve(parent, name);
        }
        None
    }
}

impl Default for ScopeArena {
    fn default() -> Self {
        Self::new()
    }
}

/// An unresolved import that needs to be resolved during import resolution.
/// Stored in a side table on GlobalContext, not in the scope.
#[derive(Clone, Debug)]
pub struct PendingImport {
    /// The scope that contains this import.
    pub scope: ScopeId,
    /// The local name in the importing scope.
    pub local_name: String,
    /// The module specifier from the import statement.
    pub from_module: String,
    /// The original name in the source module.
    pub original_name: String,
}

#[cfg(test)]
mod tests {
    use crate::context::GlobalContext;

    #[test]
    fn test_basic_resolution() {
        let mut gctx = GlobalContext::new();
        let root = gctx.create_root_scope();
        let type_id = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::Interface(crate::ir::InterfaceDecl {
                name: "Foo".to_string(),
                js_name: "Foo".to_string(),
                type_params: vec![],
                extends: vec![],
                members: vec![],
                classification: crate::ir::InterfaceClassification::ClassLike,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: root,
            exported: false,
        });
        gctx.scopes.insert(root, "Foo".to_string(), type_id);

        assert!(gctx.scopes.resolve(root, "Foo").is_some());
        assert!(gctx.scopes.resolve(root, "Bar").is_none());
    }

    #[test]
    fn test_child_scope_shadows_parent() {
        let mut gctx = GlobalContext::new();
        let parent = gctx.create_root_scope();
        let child = gctx.scopes.create_child(parent);

        let id_a = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::Interface(crate::ir::InterfaceDecl {
                name: "Foo".to_string(),
                js_name: "Foo".to_string(),
                type_params: vec![],
                extends: vec![],
                members: vec![],
                classification: crate::ir::InterfaceClassification::ClassLike,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: parent,
            exported: false,
        });
        let id_b = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::Interface(crate::ir::InterfaceDecl {
                name: "Foo".to_string(),
                js_name: "Foo".to_string(),
                type_params: vec![],
                extends: vec![],
                members: vec![],
                classification: crate::ir::InterfaceClassification::ClassLike,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: child,
            exported: false,
        });

        gctx.scopes.insert(parent, "Foo".to_string(), id_a);
        gctx.scopes.insert(child, "Foo".to_string(), id_b);

        // Child shadows parent
        assert_eq!(gctx.scopes.resolve(child, "Foo"), Some(id_b));
        // Parent still resolves to id_a
        assert_eq!(gctx.scopes.resolve(parent, "Foo"), Some(id_a));
    }

    #[test]
    fn test_child_inherits_parent() {
        let mut gctx = GlobalContext::new();
        let parent = gctx.create_root_scope();
        let child = gctx.scopes.create_child(parent);

        let id = gctx.insert_type(crate::ir::TypeDeclaration {
            kind: crate::ir::TypeKind::Interface(crate::ir::InterfaceDecl {
                name: "Foo".to_string(),
                js_name: "Foo".to_string(),
                type_params: vec![],
                extends: vec![],
                members: vec![],
                classification: crate::ir::InterfaceClassification::ClassLike,
            }),
            module_context: crate::ir::ModuleContext::Global,
            doc: None,
            scope_id: parent,
            exported: false,
        });
        gctx.scopes.insert(parent, "Foo".to_string(), id);

        // Child inherits from parent
        assert_eq!(gctx.scopes.resolve(child, "Foo"), Some(id));
    }
}
