// `Kind:[values]`, where `Kind` is `Array` or the typed array's constructor.
// An `undefined` element is spelled out, since `JSON.stringify` would write
// it as `null`.
const describe = (v) => {
    if (v === undefined) return 'undefined';
    const kind = Array.isArray(v) ? 'Array' : v.constructor.name;
    const json = JSON.stringify(Array.from(v), (_, x) => x === undefined ? 'undefined' : x);
    return `${kind}:${json}`;
};

exports.describe = describe;
exports.echo = (v) => v;
exports.callNullable = (f) => {
    f(null);
    f('a');
};
exports.nullableArray = () => ['a', null];
exports.nullableMap = () => new Map([['a', 'b'], ['c', null]]);
