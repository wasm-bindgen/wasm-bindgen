const targetName = process.argv[2];

const { greet } = await import(`../dist/nodejs_and_deno/${targetName}.js`);
greet(targetName);
