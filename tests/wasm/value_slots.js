const assert = require("assert");

// Test functions that receive generic references via value slots
exports.test_generic_i32_ref = function(val) {
    assert.strictEqual(typeof val, "number");
    assert.strictEqual(val, 42);
};

exports.test_generic_u32_ref = function(val) {
    assert.strictEqual(typeof val, "number");
    assert.strictEqual(val, 123);
};

exports.test_generic_f32_ref = function(val) {
    assert.strictEqual(typeof val, "number");
    assert(Math.abs(val - 3.14) < 0.01, `Expected ~3.14, got ${val}`);
};

exports.test_generic_f64_ref = function(val) {
    assert.strictEqual(typeof val, "number");
    assert(Math.abs(val - 2.718) < 0.001, `Expected ~2.718, got ${val}`);
};

exports.test_generic_i64_ref = function(val) {
    assert.strictEqual(typeof val, "bigint");
    assert.strictEqual(val, 9223372036854775807n);
};

exports.test_generic_u64_ref = function(val) {
    assert.strictEqual(typeof val, "bigint");
    assert.strictEqual(val, 18446744073709551615n);
};

exports.test_generic_char_ref = function(val) {
    assert.strictEqual(typeof val, "string");
    assert.strictEqual(val, "🦀");
};

exports.test_generic_bool_ref = function(val) {
    assert.strictEqual(typeof val, "boolean");
    assert.strictEqual(val, true);
};

// Test functions that return values for JsValueCast testing
exports.get_test_number = function() {
    return 42;
};

exports.get_test_string = function() {
    return "test";
};

exports.get_test_bigint = function() {
    return 9007199254740991n;
};

exports.get_test_boolean = function() {
    return true;
};