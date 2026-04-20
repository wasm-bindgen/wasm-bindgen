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
};

exports.js_inherited_via_deref_works = () => {
    const dog = new wbg.InheritanceDog('Rex', 'Labrador');
    // Child re-exports `name` (Deref-based one-liner on the Rust side).
    if (dog.name() !== 'Rex') {
        throw new Error('expected dog.name() === Rex, got: ' + dog.name());
    }
    if (dog.breed() !== 'Labrador') {
        throw new Error('expected dog.breed() === Labrador, got: ' + dog.breed());
    }
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
