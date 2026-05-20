/* tslint:disable */
/* eslint-disable */

export class Renamed {
    free(): void;
    [Symbol.dispose](): void;
    constructor(value: number);
    readonly value: number;
}

export function makeRenamed(value: number): any;

export function readRenameds(renameds: Renamed[]): number;
