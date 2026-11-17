const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.makePoint = (x, y) => ({ x, y });
exports.pointX = point => point.x;
exports.pointY = point => point.y;

const shift = ({ x, y }) => ({ x: x + 1, y: y + 1 });

exports.shiftPoint = shift;
exports.shiftOptPoint = point => point === undefined ? undefined : shift(point);
exports.shiftPoints = points => {
    assert.ok(Array.isArray(points));
    return points.map(shift);
};
exports.shiftOptPoints = points => points === undefined ? undefined : points.map(shift);
exports.sumPointRef = ({ x, y }) => x + y;
exports.sumPointSlice = points => {
    assert.ok(Array.isArray(points));
    return points.reduce((sum, { x, y }) => sum + x + y, 0);
};

exports.callExports = () => {
    assert.deepStrictEqual(wasm.third_party_point({ x: 1, y: 2 }), { x: 2, y: 4 });
    assert.deepStrictEqual(wasm.third_party_opt_point({ x: 1, y: 2 }), { x: 2, y: 4 });
    assert.strictEqual(wasm.third_party_opt_point(undefined), undefined);
    assert.deepStrictEqual(
        wasm.third_party_points([{ x: 1, y: 2 }, { x: 3, y: 4 }]),
        [{ x: 2, y: 4 }, { x: 6, y: 8 }],
    );
    assert.deepStrictEqual(wasm.third_party_points([]), []);
    assert.deepStrictEqual(wasm.third_party_opt_points([{ x: 1, y: 2 }]), [{ x: 2, y: 4 }]);
    assert.strictEqual(wasm.third_party_opt_points(undefined), undefined);
    assert.strictEqual(wasm.third_party_point_ref({ x: 1, y: 2 }), 3);
};
