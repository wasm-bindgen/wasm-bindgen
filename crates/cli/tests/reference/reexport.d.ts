/* tslint:disable */
/* eslint-disable */

export let CustomType: unknown;

export let MY_CONSTANT: unknown;

declare let OriginalName: unknown;
export { OriginalName as RenamedClass }

declare function foo(): void;

export let Snippet: {
    foo: typeof foo,
};

declare let _default: unknown;
export default _default;

export let helperFunction: unknown;

declare let invalid_name: unknown;
// export { invalid_name as 'invalid-name' }

declare let original_config: unknown;
export { original_config as renamedConfig }

declare let original: unknown;
export { original as renamedFunction }
