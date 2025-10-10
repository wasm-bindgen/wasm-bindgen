/* tslint:disable */
/* eslint-disable */
export function echo_status(status: Status): Status;
export function echo_response(response: ApiResponse): ApiResponse;
type ApiResponse = "loading" | "empty" | string | ExportedStruct | ImportedType;
type Status = "success" | "error" | string;
export class ExportedStruct {
  free(): void;
  [Symbol.dispose](): void;
  constructor(value: number);
}
