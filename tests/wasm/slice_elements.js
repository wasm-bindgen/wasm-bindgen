// `Kind:[values]`, where `Kind` is `Array` or the typed array's constructor.
const describe = (v) => {
    if (v === undefined) return 'undefined';
    const kind = Array.isArray(v) ? 'Array' : v.constructor.name;
    return `${kind}:${JSON.stringify(Array.from(v))}`;
};

exports.describe = describe;

exports.describeAll = (...args) => args.map(describe).join(' ');

exports.errorMessages = (v) => v.map((e) => e.message).join(',');

let retained = null;
exports.retain = (v) => {
    retained = v;
};
exports.retained = () => describe(retained);

exports.makeThing = (id) => ({ id });
exports.thingIds = (v) =>
    v === undefined ? 'undefined' : `${Array.isArray(v) ? 'Array' : v.constructor.name}:${v.map((t) => t.id).join(',')}`;

// Each level's own kind: a typed array by its constructor, an `Array` bare.
const shape = (v) =>
    v === undefined ? 'undefined'
    : v === null ? 'null'
    : typeof v === 'bigint' ? `${v}n`
    : ArrayBuffer.isView(v) ? `${v.constructor.name}[${Array.from(v, (e) => shape(e))}]`
    : Array.isArray(v) ? `[${v.map((e) => shape(e))}]`
    : JSON.stringify(v);

exports.shape = shape;
