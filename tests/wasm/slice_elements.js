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
