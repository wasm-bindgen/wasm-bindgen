// Simple cross-platform copy command for build scripts.
import { copyFileSync } from 'node:fs';
import { basename } from 'node:path';

for (const file of process.argv.slice(2)) {
	const dst = `../dist/${basename(process.cwd())}/${file}`;
	console.log('Copying', file, 'to', dst);
	copyFileSync(file, dst);
}
