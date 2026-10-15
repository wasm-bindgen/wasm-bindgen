/* tslint:disable */
/* eslint-disable */

type Mode = "fast" | "slow";

type Mode2 = "fast" | "slow";
type ModeOrNumber = Mode | number;
type ModeOrNumber2 = Mode2 | number;

declare enum Level {
    Low = 1,
    High = 2,
}

declare enum Level2 {
    Low = 1,
    High = 2,
}

export function a_level(level: Level): Level;

export function a_mode(mode: Mode): Mode;

export function a_union(value: ModeOrNumber): ModeOrNumber;

export function b_level(level: Level2): Level2;

export function b_mode(mode: Mode2): Mode2;

export function b_union(value: ModeOrNumber2): ModeOrNumber2;
