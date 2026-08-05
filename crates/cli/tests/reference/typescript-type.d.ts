/* tslint:disable */
/* eslint-disable */

export type Café = "espresso" | "crème";

export enum Größe {
    Klein = 1,
    Gross = 2,
}

export function accented(a: "Café" | "naïve"): void;

export function single(a: number | string): void;

export function slice(a: (number | string)[]): void;

export function take_cafe(c: Café): Café;

export function take_groesse(g: Größe): Größe;
