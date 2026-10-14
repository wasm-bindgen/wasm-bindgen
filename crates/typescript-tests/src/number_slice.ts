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

test("mutable and clamped slices take typed arrays only", () => {
  const fn_fill_number_slice = wbg.fn_fill_number_slice;
  const fn_expects_clamped_vec = wbg.fn_expects_clamped_vec;

  const filled = new Float64Array(2);
  fn_fill_number_slice(filled);
  expect(filled).toEqual(new Float64Array([1, 1]));
  expect(fn_expects_clamped_vec(new Uint8ClampedArray([1, 2]))).toBe(2);

  // The results are copied back into the buffer of the typed array.
  // @ts-expect-error
  const _fill_plain_array = () => fn_fill_number_slice([1, 2]);
  // Numbers in a plain array would wrap around instead of being clamped.
  // @ts-expect-error
  const _clamped_plain_array = () => fn_expects_clamped_vec([1, 2]);
});
