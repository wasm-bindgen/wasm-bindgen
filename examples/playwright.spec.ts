/// <reference types="node" />
/// <reference lib="esnext" />

import { dirname, delimiter, join } from 'node:path';
import { test as baseTest, ConsoleMessage } from '@playwright/test';
import { globSync, existsSync } from 'node:fs';
import { chdir, env } from 'node:process';
import { exec as exec_ } from 'node:child_process';
import { promisify } from 'node:util';
const exec = promisify(exec_);

chdir(__dirname);

const { PREBUILT_EXAMPLES, PATH } = env;

const test = baseTest.extend({
  baseURL: 'http://localhost',
  context: async ({ context }, run) => {
    // A trick to serve files from the filesystem as-if they're HTTP without an actual server.
    // https://github.com/microsoft/playwright/issues/13968#issuecomment-1784041622
    await context.route('/**', (route, request) => {
      return route.fulfill({
        path: join('dist', new URL(request.url()).pathname),
        headers: {
          'Cross-Origin-Opener-Policy': 'same-origin',
          'Cross-Origin-Embedder-Policy': 'require-corp',
        },
      });
    });

    // Intercept the public websocket echo server to make test resilient to network issues.
    await context.routeWebSocket('wss://echo.websocket.org', (ws) => {
      ws.onMessage((message) => ws.send(message));
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

  function throwWithLogs(error: Error) {
    // Print the delayed logs before failing the test with the error.
    for (const msg of logs) {
      (console as any)[msg.type()](msg.text());
    }
    throw error;
  }

  page.on('console', msg => {
    logs.push(msg);
    // Treat logged errors as if they're page errors.
    if (msg.type() === 'error') {
      throwWithLogs(new Error('An error was logged to the console.'));
    }
  });

  // Fail test on any page errors (uncaught errors, unhandled rejections, network errors, etc.)
  page.once('pageerror', throwWithLogs);
});

if (!PREBUILT_EXAMPLES) {
  // Don't rely on the globally installed wasm-bindgen CLI to have the correct version.
  // Instead, build it locally (see `pretest` in `package.json`) and add it to the `PATH`.
  test.beforeAll(async () => {
    // Add the prebuilt wasm-bindgen.exe from npm `pretest` step to the `PATH`.
    const { stdout } = await exec('cargo metadata --format-version 1', {
      maxBuffer: Infinity
    });
    const { target_directory } = JSON.parse(stdout);
    env.PATH = join(target_directory, 'debug') + delimiter + PATH;
  });
}

for (const file of globSync('*/package.json')) {
  const dir = dirname(file);

  test(dir, async ({ page }) => {
    if (!PREBUILT_EXAMPLES) {
      await exec('npm run build', { cwd: dir, env });
    }

    if (existsSync(`dist/${dir}/index.html`)) {
      await page.goto(`${dir}/index.html`, {
        waitUntil: 'networkidle'
      });
    } else {
      // If index.html doesn't exist, this is not a browser test (e.g. deno).
      // Run its own test command.
      await exec('npm test', { cwd: dir });
    }
  });
}
