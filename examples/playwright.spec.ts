/// <reference types="node" />
/// <reference lib="esnext" />

import { dirname, delimiter } from 'node:path';
import { test as baseTest, ConsoleMessage } from '@playwright/test';
import { globSync, existsSync } from 'node:fs';
import { platform, chdir, env } from 'node:process';
import { exec as exec_ } from 'node:child_process';
import { promisify } from 'node:util';
const exec = promisify(exec_);

chdir(__dirname);

const { EXBUILD } = env;

const test = baseTest.extend({
  baseURL: 'http://localhost',
  context: async ({ context }, run) => {
    // A trick to serve files from the filesystem as-if they're HTTP without an actual server.
    // https://github.com/microsoft/playwright/issues/13968#issuecomment-1784041622
    await context.route('/**', (route, request) => {
      return route.fulfill({
        path: new URL(request.url()).pathname.slice(1)
      });
    });

    await run(context);
  },
  channel: 'chrome'
});

test.describe.configure({ mode: 'parallel', timeout: 2 * 60 * 1000 });

test.beforeEach(async ({ page }) => {
  // We don't print logs right away because they might be noisy, instead
  // follow the Cargo approach of buffering until a failure.
  let logs: ConsoleMessage[] = [];
  page.on('console', msg => {
    logs.push(msg);
    // Treat logged errors as if they're page errors.
    if (msg.type() === 'error') {
      throw new Error('An error was logged to the console.');
    }
  });

  // Fail test on any page errors (uncaught errors, unhandled rejections, network errors, etc.)
  page.once('pageerror', error => {
    // Print the delayed logs before failing the test with the error.
    for (const msg of logs) {
      (console as any)[msg.type()](msg.text());
    }
    throw error;
  });
});

// Don't rely on the globally installed wasm-bindgen CLI to have the correct version.
// Instead, build it locally and supply its path via PATH.

let childEnv = { ...env };

test.beforeAll(async () => {
  const { stdout } = await exec(
    'cargo build -p wasm-bindgen-cli --bin wasm-bindgen --message-format json'
  );

  // Parse the last compiler-artifact message to get the path to the built wasm-bindgen.
  // This way it works even if `CARGO_TARGET_DIR` is in a custom directory (like it is for me).
  const { executable } = stdout
    .trimEnd()
    .split('\n')
    .map(msg => JSON.parse(msg))
    .findLast(m => m.reason === 'compiler-artifact');

  childEnv.PATH = dirname(executable) + delimiter + env.PATH;
});

function exampleTest(dir: string, buildCmd: string, dist: string = dir) {
  test(dir, async ({ page }) => {
    if (EXBUILD) {
      dist = `${EXBUILD}/${dir}`;
    } else {
      await exec(buildCmd, { cwd: dir, env: childEnv });
    }

    await page.goto(`${dist}/index.html`, {
      waitUntil: 'networkidle'
    });
  });
}

test.describe('shell', () => {
  test.skip(
    platform === 'win32',
    'build.sh tests are not supported on Windows'
  );

  for (const file of globSync('*/build.sh')) {
    let dir = dirname(file);

    // If index.html doesn't exist, this is not a browser test (e.g. deno), skip it.
    if (!existsSync(`${dir}/index.html`)) continue;

    exampleTest(dir, './build.sh');
  }
});

test.describe('webpack', () => {
  for (const file of globSync('*/webpack.config.js')) {
    let dir = dirname(file);

    exampleTest(dir, 'npm run build', `${dir}/dist`);
  }
});
