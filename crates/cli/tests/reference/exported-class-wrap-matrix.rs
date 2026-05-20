use wasm_bindgen::prelude::*;

// Coverage matrix for the qualified-name keying of `WrapInExportedClass` /
// `UnwrapExportedClass`. Each struct here crosses the boundary via `JsValue`
// in both directions:
//
//   make_*   returns `Rust::new(..).into()`   -> WrapInExportedClass (`Class.__wrap`)
//   read_*   takes a single struct by value    -> `_assertClass` + flat pointer ABI
//   read_*s  takes a `Vec<Struct>` by value     -> UnwrapExportedClass (`Class.__unwrap`)
//
// Note which argument shape drives the unwrap import: a *single* exported
// struct argument lowers through `_assertClass` and the flat i32 pointer and
// never touches the unwrap import. A `Vec<Struct>` argument is marshaled as a
// JS array of class instances and unwrapped element-wise inside wasm via
// `Class.__unwrap` — that is the `UnwrapExportedClass` path. So the `read_*s`
// functions are the ones that exercise the keying fix on the unwrap side; the
// single-argument `read_*` functions only cover `_assertClass`.
//
// Both `Class.__wrap` and `Class.__unwrap` must reference the struct's
// qualified JS identity (the `exported_classes` key). A struct whose
// `rust_name` differs from its `qualified_name` (renamed via `js_name` and/or
// placed in a `js_namespace`) is where keying by `rust_name` used to mint a
// duplicate phantom class.
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

// `Vec<Struct>` arguments drive the `UnwrapExportedClass` path: each element
// is unwrapped inside wasm via `Class.__unwrap`, which must resolve to the
// qualified-name `exported_classes` key. One `read_*s` per struct in the
// matrix so the unwrap import is exercised for every rename / namespace /
// inheritance combination above (base and child alike).

// (A) namespace only.
#[wasm_bindgen(js_name = "readWidgets")]
pub fn read_widgets(widgets: Vec<Widget>) -> usize {
    widgets.len()
}

// (B) renamed base + non-renamed child.
#[wasm_bindgen(js_name = "readAnimals")]
pub fn read_animals(animals: Vec<RustAnimal>) -> usize {
    animals.len()
}

#[wasm_bindgen(js_name = "readDogs")]
pub fn read_dogs(dogs: Vec<Dog>) -> usize {
    dogs.len()
}

// (C) non-renamed base + renamed child.
#[wasm_bindgen(js_name = "readVehicles")]
pub fn read_vehicles(vehicles: Vec<Vehicle>) -> usize {
    vehicles.len()
}

#[wasm_bindgen(js_name = "readCars")]
pub fn read_cars(cars: Vec<RustCar>) -> usize {
    cars.len()
}

// (D) namespaced base + child inheriting the namespace.
#[wasm_bindgen(js_name = "readHabitats")]
pub fn read_habitats(habitats: Vec<Habitat>) -> usize {
    habitats.len()
}

#[wasm_bindgen(js_name = "readReserves")]
pub fn read_reserves(reserves: Vec<Reserve>) -> usize {
    reserves.len()
}

// (E) base and child renamed in two different namespaces.
#[wasm_bindgen(js_name = "readBases")]
pub fn read_bases(bases: Vec<RustBase>) -> usize {
    bases.len()
}

#[wasm_bindgen(js_name = "readDeriveds")]
pub fn read_deriveds(deriveds: Vec<RustDerived>) -> usize {
    deriveds.len()
}
