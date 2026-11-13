export class Animal {
    static __wrap(ptr) {
        const obj = Object.create(Animal.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_Animal = ptr;

        AnimalFinalization.register(obj, { __wbg_ptr_Animal: obj.__wbg_ptr_Animal }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_Animal = 0;
        AnimalFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_Animal) { throw new TypeError('Animal: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_animal_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get legs() {
        const ret = wasm.animal_legs(this.__wbg_ptr_Animal);
        return ret;
    }
    /**
     * @param {number} legs
     */
    constructor(legs) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.animal_new(legs);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_Animal = ret >>> 0;
        AnimalFinalization.register(this, { __wbg_ptr_Animal: ret >>> 0 }, this);
        return this;
    }
}
if (Symbol.dispose) Animal.prototype[Symbol.dispose] = Animal.prototype.free;

export class Dog extends Animal {
    static __wrap(ptr) {
        const obj = Object.create(Dog.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_Dog = ptr;
        const __wbg_anc_0 = wasm.__wbg_upcast_dog_to_animal(ptr) >>> 0;
        obj.__wbg_ptr_Animal = __wbg_anc_0;

        DogFinalization.register(obj, { __wbg_ptr_Dog: obj.__wbg_ptr_Dog, __wbg_ptr_Animal: obj.__wbg_ptr_Animal }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_Dog = 0;
        const __anc_Animal = this.__wbg_ptr_Animal;
        this.__wbg_ptr_Animal = 0;
        if (__anc_Animal !== 0) wasm.__wbg_animal_free(__anc_Animal >>> 0, 1);
        DogFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_Dog) { throw new TypeError('Dog: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dog_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get breed() {
        const ret = wasm.dog_breed(this.__wbg_ptr_Dog);
        return ret;
    }
    /**
     * @param {number} legs
     * @param {number} breed
     */
    constructor(legs, breed) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.dog_new(legs, breed);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_Dog = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_dog_to_animal(ret >>> 0) >>> 0;
        this.__wbg_ptr_Animal = __wbg_anc_0;
        DogFinalization.register(this, { __wbg_ptr_Dog: ret >>> 0, __wbg_ptr_Animal: __wbg_anc_0 }, this);
        return this;
    }
}
if (Symbol.dispose) Dog.prototype[Symbol.dispose] = Dog.prototype.free;

export class Reserve extends wild__Habitat {
    static __wrap(ptr) {
        const obj = Object.create(Reserve.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_Reserve = ptr;
        const __wbg_anc_0 = wasm.__wbg_upcast_reserve_to_wild__habitat(ptr) >>> 0;
        obj.__wbg_ptr_wild__Habitat = __wbg_anc_0;

        ReserveFinalization.register(obj, { __wbg_ptr_Reserve: obj.__wbg_ptr_Reserve, __wbg_ptr_wild__Habitat: obj.__wbg_ptr_wild__Habitat }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_Reserve = 0;
        const __anc_wild__Habitat = this.__wbg_ptr_wild__Habitat;
        this.__wbg_ptr_wild__Habitat = 0;
        if (__anc_wild__Habitat !== 0) wasm.__wbg_wild__habitat_free(__anc_wild__Habitat >>> 0, 1);
        ReserveFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_Reserve) { throw new TypeError('Reserve: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_reserve_free(ptr, 0);
    }
    /**
     * @param {number} area
     * @param {number} rangers
     */
    constructor(area, rangers) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.reserve_new(area, rangers);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_Reserve = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_reserve_to_wild__habitat(ret >>> 0) >>> 0;
        this.__wbg_ptr_wild__Habitat = __wbg_anc_0;
        ReserveFinalization.register(this, { __wbg_ptr_Reserve: ret >>> 0, __wbg_ptr_wild__Habitat: __wbg_anc_0 }, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get rangers() {
        const ret = wasm.reserve_rangers(this.__wbg_ptr_Reserve);
        return ret;
    }
}
if (Symbol.dispose) Reserve.prototype[Symbol.dispose] = Reserve.prototype.free;

export class Vehicle {
    static __wrap(ptr) {
        const obj = Object.create(Vehicle.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_Vehicle = ptr;

        VehicleFinalization.register(obj, { __wbg_ptr_Vehicle: obj.__wbg_ptr_Vehicle }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_Vehicle = 0;
        VehicleFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_Vehicle) { throw new TypeError('Vehicle: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vehicle_free(ptr, 0);
    }
    /**
     * @param {number} wheels
     */
    constructor(wheels) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.vehicle_new(wheels);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_Vehicle = ret >>> 0;
        VehicleFinalization.register(this, { __wbg_ptr_Vehicle: ret >>> 0 }, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get wheels() {
        const ret = wasm.vehicle_wheels(this.__wbg_ptr_Vehicle);
        return ret;
    }
}
if (Symbol.dispose) Vehicle.prototype[Symbol.dispose] = Vehicle.prototype.free;

class garden__Derived extends zoo__Base {
    static __wrap(ptr) {
        const obj = Object.create(garden__Derived.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_garden__Derived = ptr;
        const __wbg_anc_0 = wasm.__wbg_upcast_garden__derived_to_zoo__base(ptr) >>> 0;
        obj.__wbg_ptr_zoo__Base = __wbg_anc_0;

        garden__DerivedFinalization.register(obj, { __wbg_ptr_garden__Derived: obj.__wbg_ptr_garden__Derived, __wbg_ptr_zoo__Base: obj.__wbg_ptr_zoo__Base }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_garden__Derived = 0;
        const __anc_zoo__Base = this.__wbg_ptr_zoo__Base;
        this.__wbg_ptr_zoo__Base = 0;
        if (__anc_zoo__Base !== 0) wasm.__wbg_zoo__base_free(__anc_zoo__Base >>> 0, 1);
        garden__DerivedFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_garden__Derived) { throw new TypeError('garden__Derived: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_garden__derived_free(ptr, 0);
    }
    /**
     * @param {number} id
     * @param {number} tag
     */
    constructor(id, tag) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.garden__derived_new(id, tag);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_garden__Derived = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_garden__derived_to_zoo__base(ret >>> 0) >>> 0;
        this.__wbg_ptr_zoo__Base = __wbg_anc_0;
        garden__DerivedFinalization.register(this, { __wbg_ptr_garden__Derived: ret >>> 0, __wbg_ptr_zoo__Base: __wbg_anc_0 }, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get tag() {
        const ret = wasm.garden__derived_tag(this.__wbg_ptr_garden__Derived);
        return ret;
    }
}
if (Symbol.dispose) garden__Derived.prototype[Symbol.dispose] = garden__Derived.prototype.free;

export const garden = {};
garden.Derived = garden__Derived;

/**
 * @param {number} legs
 * @returns {any}
 */
export function makeAnimal(legs) {
    const ret = wasm.makeAnimal(legs);
    return ret;
}

/**
 * @param {number} id
 * @returns {any}
 */
export function makeBase(id) {
    const ret = wasm.makeBase(id);
    return ret;
}

/**
 * @param {number} wheels
 * @param {number} doors
 * @returns {any}
 */
export function makeCar(wheels, doors) {
    const ret = wasm.makeCar(wheels, doors);
    return ret;
}

/**
 * @param {number} id
 * @param {number} tag
 * @returns {any}
 */
export function makeDerived(id, tag) {
    const ret = wasm.makeDerived(id, tag);
    return ret;
}

/**
 * @param {number} legs
 * @param {number} breed
 * @returns {any}
 */
export function makeDog(legs, breed) {
    const ret = wasm.makeDog(legs, breed);
    return ret;
}

/**
 * @param {number} area
 * @returns {any}
 */
export function makeHabitat(area) {
    const ret = wasm.makeHabitat(area);
    return ret;
}

/**
 * @param {number} area
 * @param {number} rangers
 * @returns {any}
 */
export function makeReserve(area, rangers) {
    const ret = wasm.makeReserve(area, rangers);
    return ret;
}

/**
 * @param {number} wheels
 * @returns {any}
 */
export function makeVehicle(wheels) {
    const ret = wasm.makeVehicle(wheels);
    return ret;
}

/**
 * @param {number} v
 * @returns {any}
 */
export function makeWidget(v) {
    const ret = wasm.makeWidget(v);
    return ret;
}

class nsa__Widget {
    static __wrap(ptr) {
        const obj = Object.create(nsa__Widget.prototype);
        obj.__wbg_ptr = ptr;
        nsa__WidgetFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        nsa__WidgetFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_nsa__widget_free(ptr, 0);
    }
    /**
     * @param {number} v
     */
    constructor(v) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.nsa__widget_new(v);
        this.__wbg_ptr = ret;
        nsa__WidgetFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get v() {
        const ret = wasm.nsa__widget_v(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) nsa__Widget.prototype[Symbol.dispose] = nsa__Widget.prototype.free;

export const nsa = {};
nsa.Widget = nsa__Widget;

/**
 * @param {Animal[]} animals
 * @returns {number}
 */
export function readAnimals(animals) {
    for (let n0 = 0; n0 < animals.length; n0++) {
        const e0 = animals[n0];
        if (!(e0 instanceof Animal)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Animal);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_Animal) { throw new TypeError('expected exact instance of Animal; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = animals.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = animals[j0];
            if (!(e0 instanceof Animal)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, Animal);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_Animal) { throw new TypeError('expected exact instance of Animal; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_animal_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readAnimals(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {zoo__Base[]} bases
 * @returns {number}
 */
export function readBases(bases) {
    for (let n0 = 0; n0 < bases.length; n0++) {
        const e0 = bases[n0];
        if (!(e0 instanceof zoo__Base)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, zoo__Base);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_zoo__Base) { throw new TypeError('expected exact instance of zoo__Base; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = bases.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = bases[j0];
            if (!(e0 instanceof zoo__Base)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, zoo__Base);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_zoo__Base) { throw new TypeError('expected exact instance of zoo__Base; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_zoo__base_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readBases(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Car[]} cars
 * @returns {number}
 */
export function readCars(cars) {
    for (let n0 = 0; n0 < cars.length; n0++) {
        const e0 = cars[n0];
        if (!(e0 instanceof Car)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Car);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_Car) { throw new TypeError('expected exact instance of Car; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = cars.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = cars[j0];
            if (!(e0 instanceof Car)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, Car);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_Car) { throw new TypeError('expected exact instance of Car; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_car_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readCars(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {garden__Derived[]} deriveds
 * @returns {number}
 */
export function readDeriveds(deriveds) {
    for (let n0 = 0; n0 < deriveds.length; n0++) {
        const e0 = deriveds[n0];
        if (!(e0 instanceof garden__Derived)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, garden__Derived);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_garden__Derived) { throw new TypeError('expected exact instance of garden__Derived; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = deriveds.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = deriveds[j0];
            if (!(e0 instanceof garden__Derived)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, garden__Derived);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_garden__Derived) { throw new TypeError('expected exact instance of garden__Derived; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_garden__derived_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readDeriveds(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Dog} d
 * @returns {number}
 */
export function readDog(d) {
    _assertClass(d, Dog);
    if (d.__wbg_ptr !== d.__wbg_ptr_Dog) { throw new TypeError('expected exact instance of Dog; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
    var ptr0 = d.__destroy_into_raw();
    const ret = wasm.readDog(ptr0);
    return ret;
}

/**
 * @param {Dog[]} dogs
 * @returns {number}
 */
export function readDogs(dogs) {
    for (let n0 = 0; n0 < dogs.length; n0++) {
        const e0 = dogs[n0];
        if (!(e0 instanceof Dog)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Dog);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_Dog) { throw new TypeError('expected exact instance of Dog; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = dogs.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = dogs[j0];
            if (!(e0 instanceof Dog)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, Dog);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_Dog) { throw new TypeError('expected exact instance of Dog; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_dog_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readDogs(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {wild__Habitat[]} habitats
 * @returns {number}
 */
export function readHabitats(habitats) {
    for (let n0 = 0; n0 < habitats.length; n0++) {
        const e0 = habitats[n0];
        if (!(e0 instanceof wild__Habitat)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, wild__Habitat);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_wild__Habitat) { throw new TypeError('expected exact instance of wild__Habitat; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = habitats.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = habitats[j0];
            if (!(e0 instanceof wild__Habitat)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, wild__Habitat);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_wild__Habitat) { throw new TypeError('expected exact instance of wild__Habitat; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_wild__habitat_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readHabitats(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Reserve[]} reserves
 * @returns {number}
 */
export function readReserves(reserves) {
    for (let n0 = 0; n0 < reserves.length; n0++) {
        const e0 = reserves[n0];
        if (!(e0 instanceof Reserve)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Reserve);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_Reserve) { throw new TypeError('expected exact instance of Reserve; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = reserves.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = reserves[j0];
            if (!(e0 instanceof Reserve)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, Reserve);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_Reserve) { throw new TypeError('expected exact instance of Reserve; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_reserve_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readReserves(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Vehicle[]} vehicles
 * @returns {number}
 */
export function readVehicles(vehicles) {
    for (let n0 = 0; n0 < vehicles.length; n0++) {
        const e0 = vehicles[n0];
        if (!(e0 instanceof Vehicle)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, Vehicle);
        if (e0.__wbg_ptr !== e0.__wbg_ptr_Vehicle) { throw new TypeError('expected exact instance of Vehicle; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = vehicles.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = vehicles[j0];
            if (!(e0 instanceof Vehicle)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, Vehicle);
            if (e0.__wbg_ptr !== e0.__wbg_ptr_Vehicle) { throw new TypeError('expected exact instance of Vehicle; a wasm-bindgen descendant cannot be consumed by-value as its ancestor'); }
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_vehicle_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readVehicles(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {nsa__Widget} w
 * @returns {number}
 */
export function readWidget(w) {
    _assertClass(w, nsa__Widget);
    var ptr0 = w.__destroy_into_raw();
    const ret = wasm.readWidget(ptr0);
    return ret;
}

/**
 * @param {nsa__Widget[]} widgets
 * @returns {number}
 */
export function readWidgets(widgets) {
    for (let n0 = 0; n0 < widgets.length; n0++) {
        const e0 = widgets[n0];
        if (!(e0 instanceof nsa__Widget)) throw new Error('array contains a value of the wrong type');
        _assertClass(e0, nsa__Widget);
        if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
    }
    let j0 = 0, len0, ptr0;
    try {
        len0 = widgets.length;
        ptr0 = wasm.__wbindgen_malloc(len0 * 4, 4) >>> 0;
        for (; j0 < len0; j0++) {
            const e0 = widgets[j0];
            if (!(e0 instanceof nsa__Widget)) throw new Error('array contains a value of the wrong type');
            _assertClass(e0, nsa__Widget);
            if (e0.__wbg_ptr === 0) throw new Error('array contains a value of the wrong type');
            var ptr1 = e0.__destroy_into_raw();
            const view0 = getDataViewMemory0();
            const p0 = ptr0 + j0 * 4;
            view0.setInt32(p0 + 0, ptr1, true);
        }
    } catch (e) {
        if (ptr0) {
            for (let k2 = 0; k2 < j0; k2++) {
                const cell2 = ptr0 + k2 * 4;
                const w2_0 = getDataViewMemory0().getInt32(cell2 + 0, true) >>> 0;
                if (w2_0) wasm.__wbg_nsa__widget_free(w2_0, 1);
            }
            wasm.__wbindgen_free(ptr0, len0 * 4, 4);
        }
        throw e;
    }
    const ret = wasm.readWidgets(ptr0, len0);
    return ret >>> 0;
}

class wild__Habitat {
    static __wrap(ptr) {
        const obj = Object.create(wild__Habitat.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_wild__Habitat = ptr;

        wild__HabitatFinalization.register(obj, { __wbg_ptr_wild__Habitat: obj.__wbg_ptr_wild__Habitat }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_wild__Habitat = 0;
        wild__HabitatFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_wild__Habitat) { throw new TypeError('wild__Habitat: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wild__habitat_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get area() {
        const ret = wasm.wild__habitat_area(this.__wbg_ptr_wild__Habitat);
        return ret;
    }
    /**
     * @param {number} area
     */
    constructor(area) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.wild__habitat_new(area);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_wild__Habitat = ret >>> 0;
        wild__HabitatFinalization.register(this, { __wbg_ptr_wild__Habitat: ret >>> 0 }, this);
        return this;
    }
}
if (Symbol.dispose) wild__Habitat.prototype[Symbol.dispose] = wild__Habitat.prototype.free;

export const wild = {};
wild.Habitat = wild__Habitat;

class zoo__Base {
    static __wrap(ptr) {
        const obj = Object.create(zoo__Base.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_zoo__Base = ptr;

        zoo__BaseFinalization.register(obj, { __wbg_ptr_zoo__Base: obj.__wbg_ptr_zoo__Base }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_zoo__Base = 0;
        zoo__BaseFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_zoo__Base) { throw new TypeError('zoo__Base: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_zoo__base_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get id() {
        const ret = wasm.zoo__base_id(this.__wbg_ptr_zoo__Base);
        return ret;
    }
    /**
     * @param {number} id
     */
    constructor(id) {
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.zoo__base_new(id);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_zoo__Base = ret >>> 0;
        zoo__BaseFinalization.register(this, { __wbg_ptr_zoo__Base: ret >>> 0 }, this);
        return this;
    }
}
if (Symbol.dispose) zoo__Base.prototype[Symbol.dispose] = zoo__Base.prototype.free;

export const zoo = {};
zoo.Base = zoo__Base;

export class Car extends Vehicle {
    static __wrap(ptr) {
        const obj = Object.create(Car.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_Car = ptr;
        const __wbg_anc_0 = wasm.__wbg_upcast_car_to_vehicle(ptr) >>> 0;
        obj.__wbg_ptr_Vehicle = __wbg_anc_0;

        CarFinalization.register(obj, { __wbg_ptr_Car: obj.__wbg_ptr_Car, __wbg_ptr_Vehicle: obj.__wbg_ptr_Vehicle }, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        this.__wbg_ptr_Car = 0;
        const __anc_Vehicle = this.__wbg_ptr_Vehicle;
        this.__wbg_ptr_Vehicle = 0;
        if (__anc_Vehicle !== 0) wasm.__wbg_vehicle_free(__anc_Vehicle >>> 0, 1);
        CarFinalization.unregister(this);
        return ptr;
    }
    free() {
        if (this.__wbg_ptr !== this.__wbg_ptr_Car) { throw new TypeError('Car: free cannot be invoked through subclass prototype dispatch'); }
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_car_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get doors() {
        const ret = wasm.car_doors(this.__wbg_ptr_Car);
        return ret;
    }
    /**
     * @param {number} wheels
     * @param {number} doors
     */
    constructor(wheels, doors) {
        super(__wbgSuperSkip);
        if (arguments[0] === __wbgSuperSkip) return;
        const ret = wasm.car_new(wheels, doors);
        this.__wbg_ptr = ret >>> 0;
        this.__wbg_ptr_Car = ret >>> 0;
        const __wbg_anc_0 = wasm.__wbg_upcast_car_to_vehicle(ret >>> 0) >>> 0;
        this.__wbg_ptr_Vehicle = __wbg_anc_0;
        CarFinalization.register(this, { __wbg_ptr_Car: ret >>> 0, __wbg_ptr_Vehicle: __wbg_anc_0 }, this);
        return this;
    }
}
if (Symbol.dispose) Car.prototype[Symbol.dispose] = Car.prototype.free;
export function __wbg___wbindgen_throw_4c88d8bf264fd984(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_animal_new(arg0) {
    const ret = Animal.__wrap(arg0);
    return ret;
}
export function __wbg_car_new(arg0) {
    const ret = Car.__wrap(arg0);
    return ret;
}
export function __wbg_dog_new(arg0) {
    const ret = Dog.__wrap(arg0);
    return ret;
}
export function __wbg_garden__derived_new(arg0) {
    const ret = garden__Derived.__wrap(arg0);
    return ret;
}
export function __wbg_nsa__widget_new(arg0) {
    const ret = nsa__Widget.__wrap(arg0);
    return ret;
}
export function __wbg_reserve_new(arg0) {
    const ret = Reserve.__wrap(arg0);
    return ret;
}
export function __wbg_vehicle_new(arg0) {
    const ret = Vehicle.__wrap(arg0);
    return ret;
}
export function __wbg_wild__habitat_new(arg0) {
    const ret = wild__Habitat.__wrap(arg0);
    return ret;
}
export function __wbg_zoo__base_new(arg0) {
    const ret = zoo__Base.__wrap(arg0);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    table.set(0, undefined);
    table.set(1024, undefined);
    table.set(1025, null);
    table.set(1026, true);
    table.set(1027, false);
}
const __wbgSuperSkip = Symbol('wasm-bindgen.super-skip');
const AnimalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_animal_free(tok.__wbg_ptr_Animal >>> 0, 1);
});
const CarFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_car_free(tok.__wbg_ptr_Car >>> 0, 1);
wasm.__wbg_vehicle_free(tok.__wbg_ptr_Vehicle >>> 0, 1);
});
const DogFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_dog_free(tok.__wbg_ptr_Dog >>> 0, 1);
wasm.__wbg_animal_free(tok.__wbg_ptr_Animal >>> 0, 1);
});
const ReserveFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_reserve_free(tok.__wbg_ptr_Reserve >>> 0, 1);
wasm.__wbg_wild__habitat_free(tok.__wbg_ptr_wild__Habitat >>> 0, 1);
});
const VehicleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_vehicle_free(tok.__wbg_ptr_Vehicle >>> 0, 1);
});
const garden__DerivedFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_garden__derived_free(tok.__wbg_ptr_garden__Derived >>> 0, 1);
wasm.__wbg_zoo__base_free(tok.__wbg_ptr_zoo__Base >>> 0, 1);
});
const nsa__WidgetFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_nsa__widget_free(ptr, 1));
const wild__HabitatFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_wild__habitat_free(tok.__wbg_ptr_wild__Habitat >>> 0, 1);
});
const zoo__BaseFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry((tok) => { wasm.__wbg_zoo__base_free(tok.__wbg_ptr_zoo__Base >>> 0, 1);
});

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(bytes) {
    numBytesDecoded += bytes.length;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
        cachedTextDecoder.decode();
        numBytesDecoded = bytes.length;
    }
    return cachedTextDecoder.decode(bytes);
}


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
