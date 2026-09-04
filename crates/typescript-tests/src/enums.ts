import * as wbg from "../pkg/typescript_tests";
import { expect, jest, test } from "@jest/globals";

test("construction", () => {
  const a1: wbg.Foo = wbg.Foo.A;
  const a2: wbg.Foo.A = wbg.Foo.A;
  expect(a1).toStrictEqual(a2);
  const a3: wbg.Foo.A = 1;
  expect(a1).toStrictEqual(a3);

  const b1: wbg.Foo = wbg.Foo.B;
  const b2: wbg.Foo.B = wbg.Foo.B;
  expect(b1).toStrictEqual(b2);
  const b3: wbg.Foo.B = 3;
  expect(b1).toStrictEqual(b3);
  expect(a1).not.toStrictEqual(b1);
});

test("function calls", () => {
  const fn_expects_enum: (_: wbg.Foo) => void = wbg.fn_expects_enum;
  const fn_returns_enum: () => wbg.Foo = wbg.fn_returns_enum;
  const fn_expects_option_enum: (_?: wbg.Foo) => void = wbg.fn_expects_option_enum;
  const fn_returns_option_enum: () => wbg.Foo | undefined = wbg.fn_returns_option_enum;
  const fn_expects_enum_vec: (_: wbg.Foo[]) => void = wbg.fn_expects_enum_vec;
  const fn_returns_enum_vec: () => wbg.Foo[] = wbg.fn_returns_enum_vec;

  fn_expects_enum(wbg.Foo.B);
  expect(fn_returns_enum()).toStrictEqual(wbg.Foo.A);
  expect(fn_returns_option_enum()).toStrictEqual(wbg.Foo.A);
  fn_expects_enum_vec([wbg.Foo.B]);
  expect(fn_returns_enum_vec()).toStrictEqual([wbg.Foo.A]);
});
