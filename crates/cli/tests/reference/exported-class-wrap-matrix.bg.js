export class Animal {
    static __wrap(ptr) {
        const obj = Object.create(Animal.prototype);
        obj.__wbg_ptr = ptr;
        obj.__wbg_ptr_Animal = ptr;

        AnimalFinalization.register(obj, { __wbg_ptr_Animal: obj.__wbg_ptr_Animal }, obj);
        return obj;
    }
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_Animal) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_Dog) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_Reserve) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_Vehicle) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_garden__Derived) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (!(jsValue instanceof nsa__Widget)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    const ptr0 = passArrayJsValueToWasm0(animals, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.readAnimals(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {zoo__Base[]} bases
 * @returns {number}
 */
export function readBases(bases) {
    const ptr0 = passArrayJsValueToWasm0(bases, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.readBases(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Car[]} cars
 * @returns {number}
 */
export function readCars(cars) {
    const ptr0 = passArrayJsValueToWasm0(cars, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.readCars(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {garden__Derived[]} deriveds
 * @returns {number}
 */
export function readDeriveds(deriveds) {
    const ptr0 = passArrayJsValueToWasm0(deriveds, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
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
    const ptr0 = passArrayJsValueToWasm0(dogs, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.readDogs(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {wild__Habitat[]} habitats
 * @returns {number}
 */
export function readHabitats(habitats) {
    const ptr0 = passArrayJsValueToWasm0(habitats, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.readHabitats(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Reserve[]} reserves
 * @returns {number}
 */
export function readReserves(reserves) {
    const ptr0 = passArrayJsValueToWasm0(reserves, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.readReserves(ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {Vehicle[]} vehicles
 * @returns {number}
 */
export function readVehicles(vehicles) {
    const ptr0 = passArrayJsValueToWasm0(vehicles, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
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
    const ptr0 = passArrayJsValueToWasm0(widgets, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_wild__Habitat) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_zoo__Base) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
    static __unwrap(jsValue) {
        if (jsValue.__wbg_ptr !== jsValue.__wbg_ptr_Car) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
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
export function __wbg___wbindgen_debug_string_56c147eb1a51f0c4(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_bbadd78c1bac3a77(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_animal_new(arg0) {
    const ret = Animal.__wrap(arg0);
    return ret;
}
export function __wbg_animal_unwrap(arg0) {
    const ret = Animal.__unwrap(arg0);
    return ret;
}
export function __wbg_car_new(arg0) {
    const ret = Car.__wrap(arg0);
    return ret;
}
export function __wbg_car_unwrap(arg0) {
    const ret = Car.__unwrap(arg0);
    return ret;
}
export function __wbg_dog_new(arg0) {
    const ret = Dog.__wrap(arg0);
    return ret;
}
export function __wbg_dog_unwrap(arg0) {
    const ret = Dog.__unwrap(arg0);
    return ret;
}
export function __wbg_garden__derived_new(arg0) {
    const ret = garden__Derived.__wrap(arg0);
    return ret;
}
export function __wbg_garden__derived_unwrap(arg0) {
    const ret = garden__Derived.__unwrap(arg0);
    return ret;
}
export function __wbg_nsa__widget_new(arg0) {
    const ret = nsa__Widget.__wrap(arg0);
    return ret;
}
export function __wbg_nsa__widget_unwrap(arg0) {
    const ret = nsa__Widget.__unwrap(arg0);
    return ret;
}
export function __wbg_reserve_new(arg0) {
    const ret = Reserve.__wrap(arg0);
    return ret;
}
export function __wbg_reserve_unwrap(arg0) {
    const ret = Reserve.__unwrap(arg0);
    return ret;
}
export function __wbg_vehicle_new(arg0) {
    const ret = Vehicle.__wrap(arg0);
    return ret;
}
export function __wbg_vehicle_unwrap(arg0) {
    const ret = Vehicle.__unwrap(arg0);
    return ret;
}
export function __wbg_wild__habitat_new(arg0) {
    const ret = wild__Habitat.__wrap(arg0);
    return ret;
}
export function __wbg_wild__habitat_unwrap(arg0) {
    const ret = wild__Habitat.__unwrap(arg0);
    return ret;
}
export function __wbg_zoo__base_new(arg0) {
    const ret = zoo__Base.__wrap(arg0);
    return ret;
}
export function __wbg_zoo__base_unwrap(arg0) {
    const ret = zoo__Base.__unwrap(arg0);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
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

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
