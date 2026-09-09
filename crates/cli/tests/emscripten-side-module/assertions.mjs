import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { basename, dirname, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const [hostFile, sideFile, paddingFile] = process.argv.slice(2).map(path => resolve(path));
const side = new WebAssembly.Module(readFileSync(sideFile));
assert.equal(WebAssembly.Module.imports(side).filter(i => i.name.startsWith('__wbindgen_describe')).length, 0);
const files = new Map([sideFile, paddingFile].map(file => [basename(file), file]));
const { default: createHost } = await import(pathToFileURL(hostFile));
const host = await createHost({
    locateFile(name) { return files.get(basename(name)) ?? resolve(dirname(hostFile), basename(name)); },
});

assert.equal(host.drop_count(), 0);
assert.equal(host.call_from_rust(value => value * 3, 7), 21);
const thrown = new Error('callback failure');
assert.throws(() => host.call_from_rust(() => { throw thrown; }, 0), error => error === thrown);
assert.throws(() => host.panic_for_test(), error => error.name === 'PanicError' && /side module panic/.test(error.message));

const a = new host.Counter(40);
const callback = a.callback();
assert.equal(typeof callback, 'function');
assert.equal(callback(2), 42);
assert.equal(callback(5), 47);
assert.equal(host.call_from_rust(callback, -7), 40);
const b = new host.Counter(100);
const other = b.callback();
assert.equal(other(1), 101);
assert.equal(callback(1), 41);
a.free();
assert.equal(host.drop_count(), 1);
assert.throws(() => callback(1), /closure.*(drop|recursive|destroy|invoke)/i);
b.free();
assert.equal(host.drop_count(), 2);
console.log(`SIDE-BINDINGS-CALLBACKS-OK (${basename(hostFile)})`);
