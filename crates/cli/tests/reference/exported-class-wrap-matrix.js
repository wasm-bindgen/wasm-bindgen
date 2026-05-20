/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Animal, Dog, Habitat, Reserve, RustAnimal, RustBase, RustCar, RustDerived, Vehicle, Widget, makeAnimal, makeBase, makeCar, makeDerived, makeDog, makeHabitat, makeReserve, makeVehicle, makeWidget, readDog, readWidget, Car
} from "./reference_test_bg.js";
