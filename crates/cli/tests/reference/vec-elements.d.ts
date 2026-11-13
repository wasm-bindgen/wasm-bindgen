/* tslint:disable */
/* eslint-disable */

export type Mode = "read" | "write";

export function bools(v: boolean[]): boolean[];

export function driver(): void;

export function modes(v: Mode[]): Mode[];

export function opt_chars(v?: string[] | null): string[] | undefined;

export function strs(): string[];
