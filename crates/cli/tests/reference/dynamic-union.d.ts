/* tslint:disable */
/* eslint-disable */
export type ApiResponse = "loading" | "empty" | string | ExportedStruct | ImportedType;
export type FallbackUnion = "loading" | ImportedType;
type Status = "success" | "error" | string;
export type Wrapper = "plain" | Status | ExportedStruct;

export class ExportedStruct {
    free(): void;
    [Symbol.dispose](): void;
    constructor(value: number);
}

export function echo_fallback(u: FallbackUnion): FallbackUnion;

export function echo_optional_wrapper(w?: Wrapper | null): Wrapper | undefined;

export function echo_response(response: ApiResponse): ApiResponse;

export function echo_status(status: Status): Status;

export function echo_wrapper(w: Wrapper): Wrapper;
