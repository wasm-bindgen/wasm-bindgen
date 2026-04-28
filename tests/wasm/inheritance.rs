use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/inheritance.js")]
extern "C" {
    fn js_instanceof_works();
    fn js_inherited_method_dispatch_works();
    fn js_super_does_not_double_alloc();
    fn js_owned_parent_rejects_subclass_works();
    fn js_owned_self_method_rejects_subclass_works();
    fn js_renamed_parent_extends_works();
    fn js_skip_typescript_parent_child_works();
    fn js_borrowed_parent_with_subclass_works();
    fn js_borrowed_parent_method_with_subclass_works();
}

#[wasm_bindgen]
pub fn inheritance_take_animal_by_value(_a: InheritanceAnimal) {}

// Borrowed-parameter free function. Exercises `I32FromExternrefRustBorrow`
// in free-function position: when invoked from JS with a subclass instance
// (e.g. an `InheritanceDog`), the lowering must emit
// `dog.__wbg_ptr_InheritanceAnimal` (the upcast ancestor pointer), not
// `dog.__wbg_ptr` (the descendant pointer). `_assertClass` only does an
// `instanceof` check so a Dog passes the Animal check; it's the pointer
// routing that prevents type confusion at the wasm shim.
#[wasm_bindgen]
pub fn inheritance_take_animal_by_ref(a: &InheritanceAnimal) -> String {
    a.name.clone()
}

// Same `I32FromExternrefRustBorrow` lowering, but in *method* position:
// `InheritanceObserver::describe_animal(&self, &InheritanceAnimal)`. The
// borrowed-parameter routing must work whether the borrow is in a free
// function's argument list or another class's method's argument list.
#[wasm_bindgen]
pub struct InheritanceObserver {
    label: String,
}

#[wasm_bindgen]
impl InheritanceObserver {
    #[wasm_bindgen(constructor)]
    pub fn new(label: String) -> InheritanceObserver {
        InheritanceObserver { label }
    }

    pub fn describe_animal(&self, a: &InheritanceAnimal) -> String {
        format!("{}: {}", self.label, a.name)
    }
}

#[wasm_bindgen]
pub struct InheritanceAnimal {
    name: String,
}

#[wasm_bindgen]
impl InheritanceAnimal {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> InheritanceAnimal {
        INHERITANCE_ANIMAL_CTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        InheritanceAnimal { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    // self-by-value method on the parent — exercises the subclass-dispatch
    // guard. Calling this on a wasm-bindgen subclass instance must throw.
    pub fn into_name(self) -> String {
        self.name
    }
}

#[wasm_bindgen(extends = InheritanceAnimal)]
pub struct InheritanceDog {
    breed: String,
}

#[wasm_bindgen]
impl InheritanceDog {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, breed: String) -> InheritanceDog {
        INHERITANCE_DOG_CTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        InheritanceDog {
            parent: InheritanceAnimal::new(name).into(),
            breed,
        }
    }

    pub fn breed(&self) -> String {
        self.breed.clone()
    }

    pub fn parent_name(&self) -> String {
        self.parent.borrow().name()
    }
}

// Renamed parent + child whose rust_name sorts ALPHABETICALLY BEFORE the
// parent's exported js_name. Exercises two reviewer concerns at once:
//   (a) macro/cli upcast symbol agreement when the parent has `js_name`
//       (the macro derives the symbol from `extends = <Path>` last segment,
//       so cli-support must use rust_name there, not qualified_name).
//   (b) emission ordering: with this rename, BTreeMap export-name iteration
//       would put `class InheritanceBetaChild extends RenamedAnimal` before
//       `class RenamedAnimal` is initialized — TDZ ReferenceError at module
//       load — unless cli-support topo-sorts class definitions.
#[wasm_bindgen(js_name = "RenamedAnimal")]
pub struct InheritanceAlphaParent {
    label: String,
}

#[wasm_bindgen]
impl InheritanceAlphaParent {
    #[wasm_bindgen(constructor)]
    pub fn new(label: String) -> InheritanceAlphaParent {
        InheritanceAlphaParent { label }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}

#[wasm_bindgen(extends = InheritanceAlphaParent)]
pub struct InheritanceBetaChild {
    extra: String,
}

#[wasm_bindgen]
impl InheritanceBetaChild {
    #[wasm_bindgen(constructor)]
    pub fn new(label: String, extra: String) -> InheritanceBetaChild {
        InheritanceBetaChild {
            parent: InheritanceAlphaParent::new(label).into(),
            extra,
        }
    }

    pub fn extra(&self) -> String {
        self.extra.clone()
    }
}

// Counters observable from JS to verify super-skip behavior.
static INHERITANCE_ANIMAL_CTOR_COUNT: AtomicUsize = AtomicUsize::new(0);
static INHERITANCE_DOG_CTOR_COUNT: AtomicUsize = AtomicUsize::new(0);

#[wasm_bindgen]
pub fn inheritance_animal_ctor_count() -> usize {
    INHERITANCE_ANIMAL_CTOR_COUNT.load(Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn inheritance_dog_ctor_count() -> usize {
    INHERITANCE_DOG_CTOR_COUNT.load(Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn inheritance_reset_counters() {
    INHERITANCE_ANIMAL_CTOR_COUNT.store(0, Ordering::SeqCst);
    INHERITANCE_DOG_CTOR_COUNT.store(0, Ordering::SeqCst);
}

#[wasm_bindgen_test]
fn rust_parent_borrow_returns_parent_data() {
    let dog = InheritanceDog::new("Rex".into(), "Labrador".into());
    assert_eq!(dog.parent_name(), "Rex");
}

// The macro derives `impl AsRef<wasm_bindgen::Parent<Animal>> for Dog`,
// letting generic Rust code accept any descendant where it expects a
// `&Parent<Animal>`. Verify it both type-checks and walks to the right
// data.
#[wasm_bindgen_test]
fn rust_as_ref_parent_walks_chain() {
    fn read_animal_name<T: AsRef<wasm_bindgen::Parent<InheritanceAnimal>>>(t: &T) -> String {
        t.as_ref().borrow().name()
    }
    let dog = InheritanceDog::new("Rex".into(), "Labrador".into());
    assert_eq!(read_animal_name(&dog), "Rex");
}

#[wasm_bindgen_test]
fn js_instanceof() {
    js_instanceof_works();
}

#[wasm_bindgen_test]
fn js_inherited_method_dispatch() {
    js_inherited_method_dispatch_works();
}

#[wasm_bindgen_test]
fn js_super_skips_parent_ctor_body() {
    js_super_does_not_double_alloc();
}

#[wasm_bindgen_test]
fn js_owned_parent_rejects_subclass() {
    js_owned_parent_rejects_subclass_works();
}

#[wasm_bindgen_test]
fn js_owned_self_method_rejects_subclass() {
    js_owned_self_method_rejects_subclass_works();
}

#[wasm_bindgen_test]
fn js_renamed_parent_extends() {
    js_renamed_parent_extends_works();
}

#[wasm_bindgen_test]
fn js_skip_typescript_parent_child() {
    js_skip_typescript_parent_child_works();
}

#[wasm_bindgen_test]
fn js_borrowed_parent_with_subclass() {
    js_borrowed_parent_with_subclass_works();
}

#[wasm_bindgen_test]
fn js_borrowed_parent_method_with_subclass() {
    js_borrowed_parent_method_with_subclass_works();
}

// Parent with `skip_typescript` and a child that extends it. The child
// must keep the runtime `extends` in `.js` (so prototype-chain dispatch
// works) but drop it from `.d.ts` (the parent has no declaration to
// reference there). Runtime check: instanceof and inherited methods
// still work. The .d.ts shape is verified at build time by inspecting
// the emitted file (see js side).
#[wasm_bindgen(skip_typescript)]
pub struct InheritanceSkippedParent {
    n: u32,
}

#[wasm_bindgen]
impl InheritanceSkippedParent {
    #[wasm_bindgen(constructor)]
    pub fn new(n: u32) -> InheritanceSkippedParent {
        InheritanceSkippedParent { n }
    }

    pub fn n(&self) -> u32 {
        self.n
    }
}

#[wasm_bindgen(extends = InheritanceSkippedParent)]
pub struct InheritanceChildOfSkipped {
    extra: u32,
}

#[wasm_bindgen]
impl InheritanceChildOfSkipped {
    #[wasm_bindgen(constructor)]
    pub fn new(n: u32, extra: u32) -> InheritanceChildOfSkipped {
        InheritanceChildOfSkipped {
            parent: InheritanceSkippedParent::new(n).into(),
            extra,
        }
    }

    pub fn extra(&self) -> u32 {
        self.extra
    }
}
