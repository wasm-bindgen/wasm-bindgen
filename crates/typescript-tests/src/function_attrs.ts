import * as wbg from '../pkg/typescript_tests';
import * as wasm from '../pkg/typescript_tests_bg.wasm';
import { expect, test } from "@jest/globals";

const wasm_fn_with_attr: (a: number, b: number) => number = wasm.fn_with_attr;
const wbg_fn_with_attr: (a: number, b: boolean) => Promise<number> = wbg.fn_with_attr;
const wasm_holdsnumber_static_fn_with_attr: (a: number, b: number) => number = wasm.holdsnumber_static_fn_with_attr;
const wbg_holdsnumber_static_fn_with_attr: (a1: number, b: number) => wbg.HoldsNumber = wbg.HoldsNumber.static_fn_with_attr;
const wasm_holdsnumber_method_with_attr: (a: number, b: number, c: number) => number = wasm.holdsnumber_method_with_attr;
const wbg_holdsnumber_method_with_attr: (a: number, b: boolean) => number = wbg.HoldsNumber.static_fn_with_attr(1, undefined).method_with_attr;

test("async function fn_with_attr", async () => {
  let result = await wbg.fn_with_attr(4, undefined);
  expect(result).toEqual(4);

  result = await wbg.fn_with_attr(5, false);
  expect(result).toEqual(5);

  result = await wbg.fn_with_attr(6, true);
  expect(result).toEqual(1);
});

test("HoldsNumber methods", () => {
  const num1 = wbg.HoldsNumber.static_fn_with_attr(4, undefined).inner;
  const num2 = wbg.HoldsNumber.static_fn_with_attr(3, 4).inner;
  expect(num1).toEqual(num2);

  const holdsNumber = wbg.HoldsNumber.static_fn_with_attr(8, undefined);
  let result = holdsNumber.method_with_attr(4, undefined);
  expect(result).toEqual(8);

  result = holdsNumber.method_with_attr(5, false);
  expect(result).toEqual(8);

  result = holdsNumber.method_with_attr(6, true);
  expect(result).toEqual(6);
});

// Test unchecked_optional_param_type generates correct TypeScript signature
const fn_with_optional_param: (name?: string) => any = wbg.fn_with_optional_param;
const fn_with_optional_param_and_attrs: (userName?: string) => any = wbg.fn_with_optional_param_and_attrs;

test("fn_with_optional_param", () => {
  // Can be called without argument
  let result = wbg.fn_with_optional_param();
  expect(result).toEqual("Hello, World!");

  // Can be called with undefined
  result = wbg.fn_with_optional_param(undefined);
  expect(result).toEqual("Hello, World!");

  // Can be called with a string
  result = wbg.fn_with_optional_param("Alice");
  expect(result).toEqual("Hello, Alice!");
});

test("fn_with_optional_param_and_attrs", () => {
  // Can be called without argument
  let result = wbg.fn_with_optional_param_and_attrs();
  expect(result).toEqual("Hello, World!");

  // Can be called with undefined
  result = wbg.fn_with_optional_param_and_attrs(undefined);
  expect(result).toEqual("Hello, World!");

  // Can be called with a string
  result = wbg.fn_with_optional_param_and_attrs("Bob");
  expect(result).toEqual("Hello, Bob!");
});