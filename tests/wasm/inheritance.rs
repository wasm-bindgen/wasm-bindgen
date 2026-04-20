use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/inheritance.js")]
extern "C" {
    fn js_instanceof_works();
    fn js_inherited_via_deref_works();
    fn js_super_does_not_double_alloc();
}

// A base class whose JS constructor is user-defined. The constructor is
// invoked via `super(...)` from subclasses; without the sentinel
// short-circuit it would allocate an orphan `InheritanceAnimal` per
// subclass instance.
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
}

#[wasm_bindgen(extends = InheritanceAnimal)]
pub struct InheritanceDog {
    #[wasm_bindgen(parent)]
    parent: InheritanceAnimal,
    breed: String,
}

#[wasm_bindgen]
impl InheritanceDog {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, breed: String) -> InheritanceDog {
        INHERITANCE_DOG_CTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        InheritanceDog {
            parent: InheritanceAnimal::new(name),
            breed,
        }
    }

    pub fn breed(&self) -> String {
        self.breed.clone()
    }

    // Manual re-export of the inherited `name` method. With `Deref<Target =
    // InheritanceAnimal>` in scope this is a one-liner — this is the
    // supported pattern until automatic forwarding lands.
    #[wasm_bindgen(js_name = name)]
    pub fn name_forward(&self) -> String {
        use std::ops::Deref;
        self.deref().name()
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
fn rust_as_ref_coerces_child_to_parent() {
    let dog = InheritanceDog::new("Rex".into(), "Labrador".into());
    let animal: &InheritanceAnimal = dog.as_ref();
    assert_eq!(animal.name(), "Rex");
}

#[wasm_bindgen_test]
fn rust_deref_resolves_parent_methods() {
    use std::ops::Deref;
    let dog = InheritanceDog::new("Buddy".into(), "Poodle".into());
    // Calls InheritanceAnimal::name via Deref auto-deref on method call.
    assert_eq!(dog.deref().name(), "Buddy");
}

#[wasm_bindgen_test]
fn js_instanceof() {
    js_instanceof_works();
}

#[wasm_bindgen_test]
fn js_inherited_via_deref() {
    js_inherited_via_deref_works();
}

#[wasm_bindgen_test]
fn js_super_skips_parent_ctor_body() {
    js_super_does_not_double_alloc();
}
