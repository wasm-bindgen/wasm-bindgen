/* tslint:disable */
/* eslint-disable */

export enum Color {
    Green = 0,
    Yellow = 1,
    Red = 2,
}

declare enum HiddenErr {
    X = 0,
    Y = 1,
}

export enum RenamedErr {
    One = 0,
    Two = 1,
}

export function enum_vec_echo(values: Color[]): Color[];

export function hidden_err_vec_echo(values: HiddenErr[]): HiddenErr[];

declare enum ns__NsErr {
    A = 0,
    B = 1,
}

export let ns: {
    NsErr: typeof ns__NsErr,
};

export function ns_err_vec_echo(values: ns__NsErr[]): ns__NsErr[];

export function option_enum_vec_echo(values?: Color[] | null): Color[] | undefined;

export function renamed_err_vec_echo(values: RenamedErr[]): RenamedErr[];
