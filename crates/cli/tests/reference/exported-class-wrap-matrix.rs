use wasm_bindgen::prelude::*;

// Coverage matrix for the qualified-name keying of `WrapInExportedClass` /
// `UnwrapExportedClass`. Every struct here crosses the boundary via `JsValue`:
//
//   make_*  returns `Rust::new(..).into()`  -> WrapInExportedClass
//   read_*  takes the struct by value       -> UnwrapExportedClass
//
// Both imports must reference the struct's qualified JS identity (the
// `exported_classes` key). A struct whose `rust_name` differs from its
// `qualified_name` (renamed via `js_name` and/or placed in a `js_namespace`)
// is where keying by `rust_name` used to mint a duplicate phantom class.
//
// The plain rename / no-namespace case lives in `exported-class-rename-wrap`;
// the rename + namespace case in `js-namespace-export-same-name`. This fixture
// adds the remaining axes: namespace-only, and inheritance crossing the
// JsValue path with assorted rename / namespace combinations on parent vs
// child.

// (A) Namespace only, no rename. qualified `nsa__Widget` != rust `Widget`.
#[wasm_bindgen(js_namespace = nsa)]
pub struct Widget {
    v: i32,
}

#[wasm_bindgen(js_namespace = nsa)]
impl Widget {
    #[wasm_bindgen(constructor)]
    pub fn new(v: i32) -> Widget {
        Widget { v }
    }
    #[wasm_bindgen(getter)]
    pub fn v(&self) -> i32 {
        self.v
    }
}

#[wasm_bindgen(js_name = "makeWidget")]
pub fn make_widget(v: i32) -> JsValue {
    Widget::new(v).into()
}

#[wasm_bindgen(js_name = "readWidget")]
pub fn read_widget(w: Widget) -> i32 {
    w.v()
}

// (B) Inheritance: renamed parent (no namespace), non-renamed child.
//     Covers "base renamed, child not".
#[wasm_bindgen(js_name = "Animal")]
pub struct RustAnimal {
    legs: i32,
}

#[wasm_bindgen(js_class = "Animal")]
impl RustAnimal {
    #[wasm_bindgen(constructor)]
    pub fn new(legs: i32) -> RustAnimal {
        RustAnimal { legs }
    }
    #[wasm_bindgen(getter)]
    pub fn legs(&self) -> i32 {
        self.legs
    }
}

#[wasm_bindgen(extends = RustAnimal, extends_js_class = "Animal")]
pub struct Dog {
    breed: i32,
}

#[wasm_bindgen]
impl Dog {
    #[wasm_bindgen(constructor)]
    pub fn new(legs: i32, breed: i32) -> Dog {
        Dog {
            parent: RustAnimal::new(legs).into(),
            breed,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn breed(&self) -> i32 {
        self.breed
    }
}

#[wasm_bindgen(js_name = "makeAnimal")]
pub fn make_animal(legs: i32) -> JsValue {
    RustAnimal::new(legs).into()
}

#[wasm_bindgen(js_name = "makeDog")]
pub fn make_dog(legs: i32, breed: i32) -> JsValue {
    Dog::new(legs, breed).into()
}

#[wasm_bindgen(js_name = "readDog")]
pub fn read_dog(d: Dog) -> i32 {
    d.breed()
}

// (C) Inheritance: non-renamed parent, renamed child.
//     Covers "child renamed, base not".
#[wasm_bindgen]
pub struct Vehicle {
    wheels: i32,
}

#[wasm_bindgen]
impl Vehicle {
    #[wasm_bindgen(constructor)]
    pub fn new(wheels: i32) -> Vehicle {
        Vehicle { wheels }
    }
    #[wasm_bindgen(getter)]
    pub fn wheels(&self) -> i32 {
        self.wheels
    }
}

#[wasm_bindgen(js_name = "Car", extends = Vehicle)]
pub struct RustCar {
    doors: i32,
}

#[wasm_bindgen(js_class = "Car")]
impl RustCar {
    #[wasm_bindgen(constructor)]
    pub fn new(wheels: i32, doors: i32) -> RustCar {
        RustCar {
            parent: Vehicle::new(wheels).into(),
            doors,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn doors(&self) -> i32 {
        self.doors
    }
}

#[wasm_bindgen(js_name = "makeVehicle")]
pub fn make_vehicle(wheels: i32) -> JsValue {
    Vehicle::new(wheels).into()
}

#[wasm_bindgen(js_name = "makeCar")]
pub fn make_car(wheels: i32, doors: i32) -> JsValue {
    RustCar::new(wheels, doors).into()
}

// (D) Inheritance: namespaced parent, non-namespaced child.
//     Covers "base in js_namespace, child not".
#[wasm_bindgen(js_namespace = wild)]
pub struct Habitat {
    area: i32,
}

#[wasm_bindgen(js_namespace = wild)]
impl Habitat {
    #[wasm_bindgen(constructor)]
    pub fn new(area: i32) -> Habitat {
        Habitat { area }
    }
    #[wasm_bindgen(getter)]
    pub fn area(&self) -> i32 {
        self.area
    }
}

#[wasm_bindgen(extends = Habitat, extends_js_namespace = wild)]
pub struct Reserve {
    rangers: i32,
}

#[wasm_bindgen]
impl Reserve {
    #[wasm_bindgen(constructor)]
    pub fn new(area: i32, rangers: i32) -> Reserve {
        Reserve {
            parent: Habitat::new(area).into(),
            rangers,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn rangers(&self) -> i32 {
        self.rangers
    }
}

#[wasm_bindgen(js_name = "makeHabitat")]
pub fn make_habitat(area: i32) -> JsValue {
    Habitat::new(area).into()
}

#[wasm_bindgen(js_name = "makeReserve")]
pub fn make_reserve(area: i32, rangers: i32) -> JsValue {
    Reserve::new(area, rangers).into()
}

// (E) Inheritance across two different namespaces, both renamed.
//     Covers "base and child in different namespaces".
#[wasm_bindgen(js_name = "Base", js_namespace = zoo)]
pub struct RustBase {
    id: i32,
}

#[wasm_bindgen(js_class = "Base", js_namespace = zoo)]
impl RustBase {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32) -> RustBase {
        RustBase { id }
    }
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[wasm_bindgen(
    js_name = "Derived",
    js_namespace = garden,
    extends = RustBase,
    extends_js_class = "Base",
    extends_js_namespace = zoo
)]
pub struct RustDerived {
    tag: i32,
}

#[wasm_bindgen(js_class = "Derived", js_namespace = garden)]
impl RustDerived {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, tag: i32) -> RustDerived {
        RustDerived {
            parent: RustBase::new(id).into(),
            tag,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> i32 {
        self.tag
    }
}

#[wasm_bindgen(js_name = "makeBase")]
pub fn make_base(id: i32) -> JsValue {
    RustBase::new(id).into()
}

#[wasm_bindgen(js_name = "makeDerived")]
pub fn make_derived(id: i32, tag: i32) -> JsValue {
    RustDerived::new(id, tag).into()
}
