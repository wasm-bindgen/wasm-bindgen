import { execFileSync } from 'node:child_process';
import * as assert from 'node:assert/strict';

let { reason, filenames } = JSON.parse(execFileSync(
  'cargo',
  [
    'build',
    '--target',
    'wasm32-unknown-unknown',
    '--release',
    '--message-format=json'
  ],
  {
    stdio: ['inherit', 'pipe', 'inherit'],
    encoding: 'utf-8'
  }
)
  .trimEnd()
  .split('\n')
  .at(-2));

assert.equal(reason, 'compiler-artifact', 'the message before the build-finished message should be a compiler-artifact');
assert.equal(filenames.length, 1, 'there should be only one output filename');

execFileSync('wasm-bindgen', [...filenames, ...process.argv.slice(2)], {
	stdio: 'inherit'
});
