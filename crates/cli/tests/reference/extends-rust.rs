use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InheritanceParent {
    name: String,
}

#[wasm_bindgen]
impl InheritanceParent {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> InheritanceParent {
        InheritanceParent { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen(extends = InheritanceParent)]
pub struct InheritanceChild {
    extra: String,
}

#[wasm_bindgen]
impl InheritanceChild {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, extra: String) -> InheritanceChild {
        InheritanceChild {
            parent: InheritanceParent::new(name).into(),
            extra,
        }
    }

    pub fn extra(&self) -> String {
        self.extra.clone()
    }
}

// Three-level chain: Grandchild -> Child -> Parent. Exercises the upcast
// chain in __wrap (each ancestor pointer must be populated by walking the
// chain) and the FinalizationRegistry token shape (token carries one
// per-class pointer per ancestor).
#[wasm_bindgen(extends = InheritanceChild)]
pub struct InheritanceGrandchild {
    tag: String,
}

#[wasm_bindgen]
impl InheritanceGrandchild {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, extra: String, tag: String) -> InheritanceGrandchild {
        InheritanceGrandchild {
            parent: InheritanceChild::new(name, extra).into(),
            tag,
        }
    }

    pub fn tag(&self) -> String {
        self.tag.clone()
    }
}

#[wasm_bindgen]
pub fn inheritance_borrow_parent(p: &InheritanceParent) -> String {
    p.name()
}

// Namespaced parent/child: both classes carry a js_namespace, so their
// qualified_name (`ns.NsParent`) differs from their rust_name (`NsParent`).
// Exercises the qualified_name vs rust_name split in
// `populate_inheritance_chains` and the upcast wasm-symbol derivation
// (which must use the rust_name segment, not the dotted qualified form).
#[wasm_bindgen(js_namespace = ns)]
pub struct NsParent {
    label: String,
}

#[wasm_bindgen]
impl NsParent {
    #[wasm_bindgen(constructor)]
    pub fn new(label: String) -> NsParent {
        NsParent { label }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}

#[wasm_bindgen(js_namespace = ns, extends = NsParent)]
pub struct NsChild {
    note: String,
}

#[wasm_bindgen]
impl NsChild {
    #[wasm_bindgen(constructor)]
    pub fn new(label: String, note: String) -> NsChild {
        NsChild {
            parent: NsParent::new(label).into(),
            note,
        }
    }

    pub fn note(&self) -> String {
        self.note.clone()
    }
}
