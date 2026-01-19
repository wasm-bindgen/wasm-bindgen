import { Snippet } from './snippets/reexport_reftest-7b4ae97ed571adca/inline0.js';
import { original_config } from 'config';
import { MY_CONSTANT } from 'constants';
import { default as _default } from 'default-export-lib';
import { helperFunction } from 'helpers';
import { OriginalName } from 'some-library';
import { CustomType } from 'types-lib';
import { original } from 'utils';
import { 'invalid-name' as invalid_name } from 'weird-exports';

export { CustomType }

export { MY_CONSTANT }

export { OriginalName as RenamedClass }

function foo() {
    wasm.Snippet_foo();
}


Snippet.foo = foo;

export { Snippet }

export { _default as default }

export { helperFunction }

export { invalid_name as 'invalid-name' }

export { original_config as renamedConfig }

export { original as renamedFunction }
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
