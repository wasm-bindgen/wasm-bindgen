import { argv } from 'node:process';

const targetName = argv[2];

const { greet } = await import(`../dist/nodejs_and_deno/${targetName}/node_and_deno.js`);
greet(targetName);
