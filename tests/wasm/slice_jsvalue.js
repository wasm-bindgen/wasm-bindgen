exports.js_receive_slice = function(values) {
    return values.length;
};

exports.js_receive_slice_and_sum = function(values) {
    return values.reduce((a, b) => a + b, 0);
};

exports.js_verify_slice_values = function(values) {
    if (values.length !== 3) return false;
    if (values[0] !== 42) return false;
    if (values[1] !== "hello") return false;
    if (values[2] !== true) return false;
    return true;
};
