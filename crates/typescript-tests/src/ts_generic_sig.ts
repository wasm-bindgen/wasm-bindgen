import * as wbg from '../pkg/typescript_tests';
import * as wasm from '../pkg/typescript_tests_bg.wasm';
import { expect, test } from "@jest/globals";

const wasm_someFn: (a: number, b: number, c: number) => number = wasm.someFn;
const wbg_someFn: (
    arg1: wbg.OtherGenericType<wbg.SomeGenericType<Uint16Array>, string[] | null | undefined>,
    arg2: wbg.SomeGenericType<wbg.SomeType[]>[]
) => Promise<
    wbg.OtherGenericType<
        wbg.SomeGenericType<number | undefined>[] | undefined,
        string[] | undefined
    >
> = wbg.someFn;

const wasm_someOtherFn: (a: number) => [number, number] = wasm.someOtherFn;
const wbg_someOtherFn: (
    arg1: wbg.SomeGenericType<wbg.SomeType | null | undefined>
) => wbg.OtherGenericType<
    wbg.SomeType,
    wbg.SomeGenericType<wbg.SomeType | undefined>
>[] | undefined = wbg.someOtherFn;

const wasm_anotherFn: (a: number) => [number, number] = wasm.anotherFn;
const wbg_anotherFn: (
    arg1: wbg.OtherGenericType<wbg.SomeType, Uint32Array>
) => Promise<
    wbg.OtherGenericType<
        wbg.SomeType,
        wbg.SomeGenericType<wbg.SomeType | undefined>
    >
> = wbg.anotherFn;

const wasm_teststruct_method1: (a: any, b: any, c: any) => any = wasm.teststruct_method1;
const wbg_teststruct_method1: (
    arg1: wbg.SomeGenericType<number>,
    arg2: wbg.OtherGenericType<boolean, string>,
    arg3: wbg.OtherGenericType<bigint, wbg.SomeType>
) => Promise<
    wbg.OtherGenericType<
        wbg.SomeGenericType<number | undefined>[],
        string[] | undefined
    >
> = wbg.TestStruct.method1;

const wasm_teststruct_method2: (a: number) => [number,  number, number] = wasm.teststruct_method2;
const wbg_teststruct_method2: wbg.OtherGenericType<
    wbg.SomeGenericType<Uint8Array>[],
    undefined
> = new wbg.TestStruct().someProperty;

test("test generics in function signatures", async() => {
    expect(wbg_teststruct_method2).toStrictEqual({
        field1: [{ field: [0] }],
        field2: undefined,
    })

    expect(await wbg_teststruct_method1(
        {field: 1},
        {field1: true, field2: "abcd"},
        {field1: BigInt(8), field2: {prop: "zxc"}}
    )).toStrictEqual({
        field1: [{ field: 0 }],
        field2: [""],
    })

    expect(await wbg_anotherFn(
        {field1: {prop: "abcd"}, field2: Uint32Array.from([1, 2, 3])},
    )).toStrictEqual({
        field1: { prop: "" },
        field2: { field: { prop: "" } }
    })

    expect(wbg_someOtherFn(
        {field: {prop: "zxc"}}
    )).toStrictEqual([{
        field1: { prop: "" },
        field2: { field: { prop: "" } }
    }])

    expect(await wbg_someFn(
        {field1: {field: Uint16Array.from([1, 2, 3])}, field2: ["zxc", "abcd"]},
        [{field: [{prop: "abcd"}]}],
    )).toStrictEqual({
        field1: [{ field: 0 }],
        field2: [""],
    })
})
