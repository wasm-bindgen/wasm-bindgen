/* @ts-self-types="./reference_test.d.ts" */
import * as wasm from "./reference_test_bg.wasm";
import { __wbg_set_wasm } from "./reference_test_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Animal, Dog, Reserve, Vehicle, makeAnimal, makeBase, makeCar, makeDerived, makeDog, makeHabitat, makeReserve, makeVehicle, makeWidget, readAnimals, readBases, readCars, readDeriveds, readDog, readDogs, readHabitats, readReserves, readVehicles, readWidget, readWidgets, Car
} from "./reference_test_bg.js";
