import * as wbg from "../pkg/typescript_tests";
import { expect, test } from "@jest/globals";

test("string literal variant typechecks", () => {
  const a: wbg.ApiResponse = "loading";
  const b: wbg.ApiResponse = "empty";
  // The catch-all `string` variant accepts any string.
  const c: wbg.ApiResponse = "anything else";
  // Round-trip preserves literals.
  expect(wbg.echo_api_response(a)).toStrictEqual("loading");
  expect(wbg.echo_api_response(b)).toStrictEqual("empty");
  expect(wbg.echo_api_response(c)).toStrictEqual("anything else");
});

test("exported struct variant typechecks", () => {
  const shape = new wbg.ExportedShape(42);
  const r: wbg.ApiResponse = shape;
  const out = wbg.echo_api_response(r);
  // Round-tripping an exported struct through Rust consumes the original
  // wrapper and returns a fresh one (standard wasm-bindgen ownership). We
  // verify the variant survived discrimination by checking instanceof and
  // a field value.
  expect(out).toBeInstanceOf(wbg.ExportedShape);
  expect((out as wbg.ExportedShape).size).toBe(42);
});

test("function signatures expose the union type", () => {
  // These assignments only succeed if the generated `.d.ts` exposes
  // `ApiResponse` as a usable type alias and the function signatures
  // are typed in terms of it.
  const fn: (r: wbg.ApiResponse) => wbg.ApiResponse = wbg.echo_api_response;
  expect(typeof fn).toBe("function");
});

test("status union without fallback variant", () => {
  const ok: wbg.Status = "success";
  const bad: wbg.Status = "failure";
  const detail: wbg.Status = "with details";
  expect(wbg.echo_status(ok)).toStrictEqual("success");
  expect(wbg.echo_status(bad)).toStrictEqual("failure");
  expect(wbg.echo_status(detail)).toStrictEqual("with details");
});

test("nested union: variant payload is another union", () => {
  // Outer literal variant.
  const plain: wbg.Wrapped = "plain";
  expect(wbg.echo_wrapped(plain)).toStrictEqual("plain");

  // Nested literal: "success" must dispatch through Wrapped into Status.
  const status: wbg.Wrapped = "success";
  expect(wbg.echo_wrapped(status)).toStrictEqual("success");

  // Nested fallback: any unknown string lands in Status::Detail.
  const detail: wbg.Wrapped = "anything";
  expect(wbg.echo_wrapped(detail)).toStrictEqual("anything");

  // Outer struct variant.
  const shape = new wbg.ExportedShape(11);
  const back = wbg.echo_wrapped(shape);
  expect(back).toBeInstanceOf(wbg.ExportedShape);
  expect((back as wbg.ExportedShape).size).toBe(11);
});

test("Option<DynamicUnion> typechecks and round-trips", () => {
  const fn: (w?: wbg.Wrapped | null) => wbg.Wrapped | undefined =
    wbg.echo_optional_wrapped;
  expect(fn(undefined)).toBeUndefined();
  expect(fn(null)).toBeUndefined();
  expect(fn("plain")).toStrictEqual("plain");
  expect(fn("success")).toStrictEqual("success");
});
