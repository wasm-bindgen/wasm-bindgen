const wbg = require('wasm-bindgen-test.js');

exports.js_instanceof_works = () => {
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    if (!(dog instanceof wbg.InheritanceDog)) {
        throw new Error('expected dog instanceof InheritanceDog');
    }
    if (!(dog instanceof wbg.InheritanceAnimal)) {
        throw new Error('expected dog instanceof InheritanceAnimal');
    }
    const animal = new wbg.InheritanceAnimal('Felix');
    if (animal instanceof wbg.InheritanceDog) {
        throw new Error('animal should not be instanceof InheritanceDog');
    }
    dog.free();
    animal.free();
};

// Verify the multi-pointer ABI: a Dog instance dispatching a parent method
// (Animal.prototype.name) via the JS prototype chain must pass a pointer
// that wasm can soundly interpret as an Animal.  Each ancestor is stored on
// the instance as `this.__wbg_ptr_<Ancestor>`, and the parent class's
// emitted method body reads from that per-class field.
exports.js_inherited_method_dispatch_works = () => {
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    if (dog.name() !== 'Rex') {
        throw new Error('expected dog.name() === Rex, got: ' + dog.name());
    }
    if (dog.breed() !== 'Labrador') {
        throw new Error('expected dog.breed() === Labrador, got: ' + dog.breed());
    }
    // Both the own-class pointer and the ancestor pointer must be non-zero.
    if (!dog.__wbg_ptr_InheritanceDog || !dog.__wbg_ptr_InheritanceAnimal) {
        throw new Error('expected both per-class pointers to be populated');
    }
    if (dog.__wbg_ptr_InheritanceDog === dog.__wbg_ptr_InheritanceAnimal) {
        throw new Error(
            'ancestor pointer must be a separate Rc allocation, got the same ptr as own'
        );
    }
    dog.free();
};

// Owned parent conversion of a wasm-bindgen subclass instance must throw,
// not silently hand the descendant pointer to the parent's wasm shim. The
// generated `Parent.__unwrap` rejects with a 0 return, which trips
// `assert_not_null` in the Rust shim and surfaces as a JS exception here.
exports.js_owned_parent_rejects_subclass_works = () => {
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    let threw = false;
    try {
        wbg.inheritance_take_animal_by_value(dog);
    } catch (_) {
        threw = true;
    }
    if (!threw) {
        throw new Error(
            'passing an InheritanceDog by value as InheritanceAnimal must throw'
        );
    }
    dog.free();

    // The legitimate path — passing an actual InheritanceAnimal — must still
    // succeed and consume the instance.
    const animal = new wbg.InheritanceAnimal('Felix');
    wbg.inheritance_take_animal_by_value(animal);
};

// `self`-by-value parent method dispatched via the JS prototype chain on a
// wasm-bindgen subclass would otherwise hand the descendant pointer to the
// parent's wasm shim. The cli-support guard now throws before the
// __destroy_into_raw call.
exports.js_owned_self_method_rejects_subclass_works = () => {
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    let threw = false;
    try {
        dog.into_name();
    } catch (_) {
        threw = true;
    }
    if (!threw) {
        throw new Error(
            'InheritanceAnimal.into_name() must throw when dispatched on an InheritanceDog'
        );
    }
    dog.free();

    // Direct invocation on a real parent must still consume and return.
    const animal = new wbg.InheritanceAnimal('Felix');
    if (animal.into_name() !== 'Felix') {
        throw new Error(
            'InheritanceAnimal.into_name() must still work on a real InheritanceAnimal'
        );
    }
};

// Renamed parent (js_name = "RenamedAnimal") with a child whose rust_name
// alphabetically precedes the parent's export name. Validates both the
// upcast wasm-symbol agreement and the topological emission order: if
// cli-support emits `class InheritanceBetaChild extends RenamedAnimal`
// before `class RenamedAnimal`, the module fails to load with a TDZ
// ReferenceError — so this test guards both concerns at once.
exports.js_renamed_parent_extends_works = () => {
    const child = new wbg.InheritanceBetaChild('Aristotle', 'extra-bit');
    if (!(child instanceof wbg.RenamedAnimal)) {
        throw new Error('child should be instanceof RenamedAnimal');
    }
    if (child.label() !== 'Aristotle') {
        throw new Error('inherited label() must work, got: ' + child.label());
    }
    if (child.extra() !== 'extra-bit') {
        throw new Error('child.extra() must work, got: ' + child.extra());
    }
    child.free();
};

// Parent with `skip_typescript`: the child's `.d.ts` must not say
// `extends Parent` (the parent has no declaration), but the runtime JS
// `class extends` and prototype-chain dispatch must still work.
exports.js_skip_typescript_parent_child_works = () => {
    const child = new wbg.InheritanceChildOfSkipped(7, 9);
    if (!(child instanceof wbg.InheritanceSkippedParent)) {
        throw new Error('child should still be instanceof InheritanceSkippedParent');
    }
    if (child.n() !== 7) {
        throw new Error('inherited n() must return 7, got: ' + child.n());
    }
    if (child.extra() !== 9) {
        throw new Error('child.extra() must return 9, got: ' + child.extra());
    }
    child.free();
};

// Pass a Dog (subclass) to a free function declared as `&Animal`. The
// `_assertClass` runtime check uses `instanceof`, so a Dog passes the
// Animal check. But the JS-side `I32FromExternrefRustBorrow` lowering
// emits `dog.__wbg_ptr` — a pointer to `WasmRefCell<Dog>` — which is then
// reinterpreted by the wasm shim as `WasmRefCell<Animal>`. The Dog's
// fields happen to align such that `Dog.breed` (a String) lands where
// `Animal.name` (also a String) lives, so the call returns the wrong
// data instead of throwing or returning the correct upcast.
//
// The fix: when the parameter class participates in inheritance, emit
// `dog.__wbg_ptr_<DefiningClass>` (which the constructor populated via
// upcast). For now this test asserts the SOUND behavior — either the
// call returns the correct name "Rex", or it throws. What it must NOT
// do is return the breed silently.
exports.js_borrowed_parent_with_subclass_works = () => {
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    let result;
    let threw = false;
    try {
        result = wbg.inheritance_take_animal_by_ref(dog);
    } catch (_) {
        threw = true;
    }
    if (!threw && result !== 'Rex') {
        throw new Error(
            'borrowed-parent dispatch on a subclass returned wrong data: ' +
            JSON.stringify(result) +
            ' — type confusion (dog.__wbg_ptr was reinterpreted as *Animal)'
        );
    }
    dog.free();

    // Real-Animal path must still work after the routing change.
    const animal = new wbg.InheritanceAnimal('Felix');
    if (wbg.inheritance_take_animal_by_ref(animal) !== 'Felix') {
        throw new Error('borrowed-parent on a real Animal must return its name');
    }
    animal.free();
};

// Same lowering in method position: another class's method takes
// `&InheritanceAnimal`. Pass a Dog (subclass) — must return the Animal's
// name "Rex", not the Dog's breed "Labrador" (the type-confusion symptom).
exports.js_borrowed_parent_method_with_subclass_works = () => {
    const observer = new wbg.InheritanceObserver('seen');
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    const result = observer.describe_animal(dog);
    if (result !== 'seen: Rex') {
        throw new Error(
            'borrowed-parent in method position returned wrong data: ' +
            JSON.stringify(result) +
            ' — expected "seen: Rex"'
        );
    }
    dog.free();

    const animal = new wbg.InheritanceAnimal('Felix');
    if (observer.describe_animal(animal) !== 'seen: Felix') {
        throw new Error('borrowed-parent method on a real Animal must work');
    }
    animal.free();
    observer.free();
};

exports.js_super_does_not_double_alloc = () => {
    wbg.inheritance_reset_counters();

    // new InheritanceDog should:
    //   - invoke InheritanceDog::new in Rust (dog ctor count +1)
    //   - which calls InheritanceAnimal::new once for the inner parent field
    //     (animal ctor count +1)
    //   - and trigger `super(__wbgSuperSkip)` in JS, which must SHORT-CIRCUIT
    //     the JS parent-class constructor so it does NOT allocate a second
    //     InheritanceAnimal.
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');

    const animalCount = wbg.inheritance_animal_ctor_count();
    const dogCount = wbg.inheritance_dog_ctor_count();
    if (dogCount !== 1) {
        throw new Error('expected 1 Dog ctor invocation, got ' + dogCount);
    }
    if (animalCount !== 1) {
        throw new Error(
            'super(sentinel) must short-circuit parent ctor; expected 1 ' +
            'Animal ctor invocation (from the Rust parent-field init), ' +
            'got ' + animalCount + '. This indicates the super-skip sentinel is not wired up.'
        );
    }
    dog.free();
};
