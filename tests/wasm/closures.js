const assert = require('assert');
const wasm = require('wasm-bindgen-test');

exports.works_call = a => {
    a();
};

exports.works_thread = a => a(2);

let CANNOT_REUSE_CACHE = null;

exports.cannot_reuse_call = a => {
    CANNOT_REUSE_CACHE = a;
};

exports.cannot_reuse_call_again = () => {
    CANNOT_REUSE_CACHE();
};

exports.long_lived_call1 = a => {
    a();
};

exports.long_lived_call2 = a => a(2);

exports.many_arity_call1 = a => {
    a();
};
exports.many_arity_call2 = a => {
    a(1);
};
exports.many_arity_call3 = a => {
    a(1, 2);
};
exports.many_arity_call4 = a => {
    a(1, 2, 3);
};
exports.many_arity_call5 = a => {
    a(1, 2, 3, 4);
};
exports.many_arity_call6 = a => {
    a(1, 2, 3, 4, 5);
};
exports.many_arity_call7 = a => {
    a(1, 2, 3, 4, 5, 6);
};
exports.many_arity_call8 = a => {
    a(1, 2, 3, 4, 5, 6, 7);
};
exports.many_arity_call9 = a => {
    a(1, 2, 3, 4, 5, 6, 7, 8);
};

exports.option_call1 = a => {
    if (a) {
        a();
    }
};
exports.option_call2 = a => {
    if (a) {
        return a(2);
    }
};
exports.option_call3 = a => a == undefined;

let LONG_LIVED_DROPPING_CACHE = null;

exports.long_lived_dropping_cache = a => {
    LONG_LIVED_DROPPING_CACHE = a;
};
exports.long_lived_dropping_call = () => {
    LONG_LIVED_DROPPING_CACHE();
};

let LONG_LIVED_OPTION_DROPPING_CACHE = null;

exports.long_lived_option_dropping_cache = a => {
    if (a) {
        LONG_LIVED_OPTION_DROPPING_CACHE = a;
        return true;
    } else {
        return false;
    }
}
exports.long_lived_option_dropping_call = () => {
    LONG_LIVED_OPTION_DROPPING_CACHE();
}

let LONG_FNMUT_RECURSIVE_CACHE = null;

exports.long_fnmut_recursive_cache = a => {
    LONG_FNMUT_RECURSIVE_CACHE = a;
};
exports.long_fnmut_recursive_call = () => {
    LONG_FNMUT_RECURSIVE_CACHE();
};

exports.fnmut_call = a => {
    a();
};

exports.fnmut_thread = a => a(2);

let FNMUT_BAD_F = null;

exports.fnmut_bad_call = a => {
    FNMUT_BAD_F = a;
    a();
};

exports.fnmut_bad_again = x => {
    if (x) {
        FNMUT_BAD_F();
    }
};

exports.string_arguments_call = a => {
    a('foo');
};

exports.string_ret_call = a => {
    assert.strictEqual(a('foo'), 'foobar');
};

let DROP_DURING_CALL = null;
exports.drop_during_call_save = f => {
  DROP_DURING_CALL = f;
};
exports.drop_during_call_call = () => DROP_DURING_CALL();

exports.js_test_closure_returner = () => {
  wasm.closure_returner().someKey();
};

exports.calling_it_throws = a => {
  try {
    a();
    return false;
  } catch(_) {
    return true;
  }
};

exports.call_val = f => f();

exports.pass_reference_first_arg_twice = (a, b, c) => {
  b(a);
  c(a);
  a.free();
};

exports.call_destroyed = f => {
  assert.throws(f, /closure invoked.*after being dropped/);
};

let FORGOTTEN_CLOSURE = null;

exports.js_store_forgotten_closure = f => {
  FORGOTTEN_CLOSURE = f;
};

exports.js_call_forgotten_closure = () => {
  FORGOTTEN_CLOSURE();
};

// Test for RefClosure - closure works during callback, throws after
let CLOSURE_WITH_CACHE = null;

exports.closure_with_call = f => {
  f();
};

// Same as closure_with_call but used to test RefClosure -> &Closure deref
exports.closure_with_call_closure = f => {
  f();
};

exports.closure_with_cache = f => {
  CLOSURE_WITH_CACHE = f;
};

exports.closure_with_call_cached = () => {
  CLOSURE_WITH_CACHE();
};

// Test that calling a RefClosure closure after it's been invalidated throws
let CLOSURE_WITH_ARG_CACHE = null;

exports.closure_with_call_and_cache = f => {
  CLOSURE_WITH_ARG_CACHE = f;
  f(1);
  f(2);
  f(3);
};

exports.closure_with_call_cached_throws = () => {
  try {
    CLOSURE_WITH_ARG_CACHE(42);
    return false; // Should not reach here
  } catch (e) {
    // Expected: closure invoked after being dropped
    return true;
  }
};

// Test for passing Closure by value (ownership transfer)
let OWNED_CLOSURE_CACHE = null;

exports.closure_take_ownership = f => {
  // Store the closure and call it
  OWNED_CLOSURE_CACHE = f;
  f();
};

exports.closure_take_ownership_with_arg = (f, value) => {
  f(value);
};

exports.closure_call_stored = () => {
  // Call the previously stored closure
  OWNED_CLOSURE_CACHE();
};

// Test for ScopedClosure::borrow with Fn closures
exports.closure_fn_with_call = f => {
  f();
};

exports.closure_fn_with_call_arg = (f, value) => {
  f(value);
};

// Test for ImmediateClosure
exports.immediate_closure_call = f => {
  f();
};

exports.immediate_closure_call_arg = (f, value) => {
  f(value);
};

exports.immediate_closure_call_ret = (f, value) => {
  return f(value);
};

exports.immediate_closure_fn_call = f => {
  f();
};

exports.immediate_closure_catches_panic = f => {
  try {
    f();
    return false;
  } catch (e) {
    return true;
  }
};

// Calls the closure, which may call immediate_closure_fnmut_reentrant_invoke
// to trigger reentrancy
let IMMEDIATE_REENTRANT_CB = null;
exports.immediate_closure_fnmut_reentrant = f => {
  IMMEDIATE_REENTRANT_CB = f;
  f();
  IMMEDIATE_REENTRANT_CB = null;
};

// Called from inside the closure to attempt reentrant invocation
exports.immediate_closure_fnmut_reentrant_invoke = () => {
  IMMEDIATE_REENTRANT_CB();
};

// Same pattern for Fn (immutable) closures
let IMMEDIATE_FN_REENTRANT_CB = null;
exports.immediate_closure_fn_reentrant = f => {
  IMMEDIATE_FN_REENTRANT_CB = f;
  f();
  IMMEDIATE_FN_REENTRANT_CB = null;
};

exports.immediate_closure_fn_reentrant_invoke = () => {
  IMMEDIATE_FN_REENTRANT_CB();
};
