const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.test_api_namespace = function() {
  assert.ok(wasm.api, "api namespace should exist");

  assert.strictEqual(wasm.api.add(2, 3), 5, "api.add should work");
  assert.strictEqual(wasm.api.multiply(4, 5), 20, "api.multiply should work");
};

exports.test_nested_namespace = function() {
  assert.ok(wasm.utils, "utils namespace should exist");
  assert.ok(wasm.utils.math, "utils.math namespace should exist");

  assert.strictEqual(wasm.utils.math.divide(10, 2), 5, "utils.math.divide should work");
  assert.strictEqual(wasm.utils.math.subtract(10, 3), 7, "utils.math.subtract should work");
};

exports.test_class_namespace = function() {
  assert.ok(wasm.models, "models namespace should exist");
  assert.ok(wasm.models.Counter, "models.Counter class should exist");

  const counter = new wasm.models.Counter(5);
  assert.strictEqual(counter.value, 5, "constructor should set initial value");

  counter.value = 10;
  assert.strictEqual(counter.value, 10, "setter should update value");

  counter.increment();
  assert.strictEqual(counter.value, 11, "increment should increase value by 1");

  counter.add(4);
  assert.strictEqual(counter.value, 15, "add should increase value by specified amount");
};

exports.test_enum_namespace = function() {
  assert.ok(wasm.types, "types namespace should exist");
  assert.ok(wasm.types.Status, "types.Status enum should exist");

  assert.strictEqual(wasm.types.Status.Pending, 0, "Status.Pending should be 0");
  assert.strictEqual(wasm.types.Status.Active, 1, "Status.Active should be 1");
  assert.strictEqual(wasm.types.Status.Complete, 2, "Status.Complete should be 2");

  assert.strictEqual(wasm.types.Status[0], "Pending", "Status[0] should be 'Pending'");
  assert.strictEqual(wasm.types.Status[1], "Active", "Status[1] should be 'Active'");
  assert.strictEqual(wasm.types.Status[2], "Complete", "Status[2] should be 'Complete'");
};

exports.test_nested_enum_namespace = function() {
  assert.ok(wasm.types, "types namespace should exist");
  assert.ok(wasm.types.http, "types.http namespace should exist");
  assert.ok(wasm.types.http.HttpStatus, "types.http.HttpStatus enum should exist");

  assert.strictEqual(wasm.types.http.HttpStatus.Ok, 200, "HttpStatus.Ok should be 200");
  assert.strictEqual(wasm.types.http.HttpStatus.NotFound, 404, "HttpStatus.NotFound should be 404");
  assert.strictEqual(wasm.types.http.HttpStatus.ServerError, 500, "HttpStatus.ServerError should be 500");

  assert.strictEqual(wasm.types.http.HttpStatus[200], "Ok", "HttpStatus[200] should be 'Ok'");
  assert.strictEqual(wasm.types.http.HttpStatus[404], "NotFound", "HttpStatus[404] should be 'NotFound'");
  assert.strictEqual(wasm.types.http.HttpStatus[500], "ServerError", "HttpStatus[500] should be 'ServerError'");
};

exports.test_struct_namespace = function() {
  assert.ok(wasm.shapes, "shapes namespace should exist");
  assert.ok(wasm.shapes.Point, "shapes.Point class should exist");

  assert.throws(() => new wasm.shapes.Point(), "Point constructor should be private");
};

exports.test_nested_struct_namespace = function() {
  assert.ok(wasm.shapes, "shapes namespace should exist");
  assert.ok(wasm.shapes["3d"], "shapes.3d namespace should exist");
  assert.ok(wasm.shapes["3d"].Point3D, "shapes.3d.Point3D class should exist");

  assert.throws(() => new wasm.shapes["3d"].Point3D(), "Point3D constructor should be private");
};
