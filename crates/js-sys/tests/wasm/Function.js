// Used for `Function.rs` tests
exports.get_function_to_bind = function() {
  return function() { return this.x || 1; }
};
exports.get_value_to_bind_to = function() {
  return { x: 2 };
};
exports.list = function() {
  return function() {return Array.prototype.slice.call(arguments);}
};
exports.add_arguments = function() {
    return function(arg1, arg2) {return arg1 + arg2}
};
exports.call_function = function(f) {
  return f();
};
exports.call_function_arg =  function(f, arg1) {
  return f(arg1);
};
exports.sum_many_arguments = function() {
    return function(a, b, c, d, e, f, g, h) {
        return (a || 0) + (b || 0) + (c || 0) + (d || 0) + (e || 0) + (f || 0) + (g || 0) + (h || 0)
    };
};
exports.test_context = function() {
    return { multiplier: 10 };
};
exports.multiply_sum = function() {
    return function(a, b, c, d, e, f, g, h) {
        return this.multiplier * ((a || 0) + (b || 0) + (c || 0) + (d || 0) + (e || 0) + (f || 0) + (g || 0) + (h || 0));
    };
};
// Simulates a for_each method that calls a callback with (string, index) pairs
// Similar to DOMTokenList.forEach or Array.forEach
exports.invoke_for_each_callback = function(callback, items) {
    items.forEach(function(item, index) {
        callback(item, index);
    });
};