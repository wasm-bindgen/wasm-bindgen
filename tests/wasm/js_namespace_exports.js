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

// Struct uses `js_name` + `js_namespace`; impl uses `js_class` + `js_namespace`.
// Constructor and method must be reachable through the namespace export.
exports.test_renamed_namespaced_class_methods = function() {
  assert.ok(wasm.renamed_models, "renamed_models namespace should exist");
  assert.ok(wasm.renamed_models.RenamedCounter, "renamed_models.RenamedCounter class should exist");

  const obj = new wasm.renamed_models.RenamedCounter(7);
  assert.strictEqual(typeof obj.increment, "function", "instance should expose `increment` method");
  assert.strictEqual(obj.value, 7, "constructor should set initial value through the namespace export");

  obj.increment();
  assert.strictEqual(obj.value, 8, "method call through the namespace export should mutate state");
};

// Struct uses `js_name` + `js_namespace`; impl repeats both `js_class` and
// `js_namespace`. The impl macro invocation cannot see the struct's attrs,
// so the namespace must be carried on the impl block to be folded into the
// emitted wasm shim symbol name and the cli-support `exported_classes` key.
exports.test_renamed_class_namespace_on_struct_only = function() {
  assert.ok(wasm.struct_only_ns, "struct_only_ns namespace should exist");
  assert.ok(wasm.struct_only_ns.RenamedOnlyStructNs, "struct_only_ns.RenamedOnlyStructNs class should exist");

  const obj = new wasm.struct_only_ns.RenamedOnlyStructNs(5);
  assert.strictEqual(typeof obj.double, "function", "instance should expose `double` method");
  assert.strictEqual(obj.double(), 10, "method through the namespace export should return value");
};

// No rename; both struct and impl carry the same `js_namespace`. Confirms
// whether the rename is necessary to trigger the regression.
exports.test_namespaced_class_methods_same_name = function() {
  assert.ok(wasm.same_name_ns, "same_name_ns namespace should exist");
  assert.ok(wasm.same_name_ns.SameNameNs, "same_name_ns.SameNameNs class should exist");

  const obj = new wasm.same_name_ns.SameNameNs(3);
  assert.strictEqual(typeof obj.triple, "function", "instance should expose `triple` method");
  assert.strictEqual(obj.triple(), 9, "method through the namespace export should return value");
};

// Two Rust structs share the same identifier (`Foo`) across different
// modules but have distinct `js_name`s. Qualified-name keying in
// cli-support and matching `js_class` on each impl let them coexist as
// distinct JS classes. With rust_name keying the two would have
// clobbered each other in `exported_classes`.
exports.test_same_rust_ident_distinct_js_names = function() {
  assert.ok(wasm.CrossModFooAlpha, "CrossModFooAlpha class should exist");
  assert.ok(wasm.CrossModFooBeta, "CrossModFooBeta class should exist");
  assert.notStrictEqual(wasm.CrossModFooAlpha, wasm.CrossModFooBeta, "must be distinct classes");

  const a = new wasm.CrossModFooAlpha(11);
  const b = new wasm.CrossModFooBeta(22);
  assert.strictEqual(a.a_method(), 11, "Alpha method should resolve to its own impl");
  assert.strictEqual(b.b_method(), 22, "Beta method should resolve to its own impl");
  assert.strictEqual(typeof a.b_method, "undefined", "Alpha must not expose Beta's method");
  assert.strictEqual(typeof b.a_method, "undefined", "Beta must not expose Alpha's method");
};

// Two classes share the same `js_name` ("CrossNs") in different namespaces.
// Without per-impl namespace participating in symbol naming, the wasm shim
// names for `CrossNs::new`/`p_value`/`q_value` would collide at wasm-ld.
exports.test_cross_namespace_same_js_name = function() {
  assert.ok(wasm.ns_p, "ns_p namespace should exist");
  assert.ok(wasm.ns_q, "ns_q namespace should exist");
  assert.ok(wasm.ns_p.CrossNs, "ns_p.CrossNs class should exist");
  assert.ok(wasm.ns_q.CrossNs, "ns_q.CrossNs class should exist");
  assert.notStrictEqual(wasm.ns_p.CrossNs, wasm.ns_q.CrossNs, "ns_p.CrossNs and ns_q.CrossNs must be distinct classes");

  const p = new wasm.ns_p.CrossNs(1);
  const q = new wasm.ns_q.CrossNs(2);
  assert.strictEqual(p.p_value(), 101, "P-namespaced method should resolve to its own impl");
  assert.strictEqual(q.q_value(), 202, "Q-namespaced method should resolve to its own impl");
  assert.strictEqual(typeof p.q_value, "undefined", "P instance must not expose Q's method");
  assert.strictEqual(typeof q.p_value, "undefined", "Q instance must not expose P's method");
};

exports.test_nested_struct_namespace = function() {
  assert.ok(wasm.shapes, "shapes namespace should exist");
  assert.ok(wasm.shapes["3d"], "shapes.3d namespace should exist");
  assert.ok(wasm.shapes["3d"].Point3D, "shapes.3d.Point3D class should exist");

  assert.throws(() => new wasm.shapes["3d"].Point3D(), "Point3D constructor should be private");
};
