/* tslint:disable */
/* eslint-disable */
type ApiResponse = "loading" | "empty" | string | ExportedStruct | any;
type Status = "success" | "error" | string;

export class ExportedStruct {
  free(): void;
  [Symbol.dispose](): void;
  constructor(value: number);
}

export function echo_response(response: ApiResponse): ApiResponse;

export function echo_status(status: Status): Status;
