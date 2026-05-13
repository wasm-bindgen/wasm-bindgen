// Post-build: copy HTML/JS assets to dist/.
import { copyFileSync } from 'node:fs';
import { resolve, dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const dist = resolve(__dirname, '../dist/jspi-opfs');

for (const f of ['index.html']) {
    copyFileSync(join(__dirname, f), join(dist, f));
    console.log('  copied', f);
}
