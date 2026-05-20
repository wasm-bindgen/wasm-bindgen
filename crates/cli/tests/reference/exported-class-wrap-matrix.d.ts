/* tslint:disable */
/* eslint-disable */

export class Animal {
    free(): void;
    [Symbol.dispose](): void;
    constructor(legs: number);
    readonly legs: number;
}

export class Dog extends Animal {
    free(): void;
    [Symbol.dispose](): void;
    constructor(legs: number, breed: number);
    readonly breed: number;
}

export class Reserve extends wild__Habitat {
    free(): void;
    [Symbol.dispose](): void;
    constructor(area: number, rangers: number);
    readonly rangers: number;
}

export class Vehicle {
    free(): void;
    [Symbol.dispose](): void;
    constructor(wheels: number);
    readonly wheels: number;
}

declare class garden__Derived extends zoo__Base {
    free(): void;
    [Symbol.dispose](): void;
    constructor(id: number, tag: number);
    readonly tag: number;
}

export let garden: {
    Derived: typeof garden__Derived,
};

export function makeAnimal(legs: number): any;

export function makeBase(id: number): any;

export function makeCar(wheels: number, doors: number): any;

export function makeDerived(id: number, tag: number): any;

export function makeDog(legs: number, breed: number): any;

export function makeHabitat(area: number): any;

export function makeReserve(area: number, rangers: number): any;

export function makeVehicle(wheels: number): any;

export function makeWidget(v: number): any;

declare class nsa__Widget {
    free(): void;
    [Symbol.dispose](): void;
    constructor(v: number);
    readonly v: number;
}

export let nsa: {
    Widget: typeof nsa__Widget,
};

export function readDog(d: Dog): number;

export function readWidget(w: nsa__Widget): number;

declare class wild__Habitat {
    free(): void;
    [Symbol.dispose](): void;
    constructor(area: number);
    readonly area: number;
}

export let wild: {
    Habitat: typeof wild__Habitat,
};

declare class zoo__Base {
    free(): void;
    [Symbol.dispose](): void;
    constructor(id: number);
    readonly id: number;
}

export let zoo: {
    Base: typeof zoo__Base,
};

export class Car extends Vehicle {
    free(): void;
    [Symbol.dispose](): void;
    constructor(wheels: number, doors: number);
    readonly doors: number;
}
