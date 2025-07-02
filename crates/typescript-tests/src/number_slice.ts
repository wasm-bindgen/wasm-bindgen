import * as wbg from "../pkg/typescript_tests";
import { expect, jest, test } from "@jest/globals";

test("function calls", () => {
  const fn_expects_number_vec = wbg.fn_expects_number_vec;
  const fn_expects_number_slice = wbg.fn_expects_number_slice;
  const fn_return_number_vec = wbg.fn_return_number_vec;

  expect(fn_expects_number_vec([0.1, 2.2, 3.4, 5])).toBeCloseTo(10.7, 5);
  expect(fn_expects_number_slice([0.1, 2.2, 3.4, 5])).toBeCloseTo(10.7, 5);
  expect(fn_return_number_vec(5)).toEqual(new Uint32Array([0, 1, 2, 3, 4]));
});

test("integer truncating", () => {
  const fn_u8_to_f32 = wbg.fn_u8_to_f32;
  expect(fn_u8_to_f32([0, 128, 255, 2.9, -0x10, 0x21f]))
    .toEqual(new Float32Array([0, 128, 255, 2, 0xf0, 0x1f]));
});
