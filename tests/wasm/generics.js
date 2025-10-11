const assert = require("assert");

exports.test_generic_ref = function(val, ty) {
    assert.strictEqual(typeof val, ty);
};

exports.get_test_val = function(ty) {
    switch (ty) {
        case 'number':
            return 42;
        case 'string':
            return 'test';
        case 'bigint':
            return 9007199254740991n;
        case 'boolean':
            return true;
    }
};
