//! Tests for cli-support diagnostic messages emitted when a user-supplied
//! name (impl `js_class`, struct `extends` parent, ...) fails to resolve.
//!
//! These exercises live alongside the rest of the CLI test suite rather than
//! in the macro UI-test directory because they assert behaviour of
//! `wasm-bindgen` post-macro-expansion: the user code compiles, the wasm is
//! emitted, and the failure surfaces when `wasm-bindgen` walks the encoded
//! `Aux*` data and tries to wire up class references.

use crate::fixture;

macro_rules! assert_contains {
    ($haystack:expr, $needle:literal) => {
        let haystack = $haystack;
        assert!(
            haystack.contains($needle),
            "Expected\n{haystack:?}\nto contain\n{:?}",
            $needle
        );
    };
}

/// When a struct declares `js_namespace = ns` but the impl block omits it,
/// the impl macro emits methods with no namespace prefix in the wasm shim
/// symbol and `Export.js_namespace = None`. The resulting class identity
/// (`Foo`) doesn't match the struct's registered `qualified_name`
/// (`ns__Foo`), so cli-support emits a targeted hint asking the user to
/// repeat `js_namespace` on the impl. The hint must reference the exact
/// namespace the struct uses so the fix is mechanical.
#[test]
fn missing_js_namespace_on_impl_suggests_struct_namespace() {
    let err = fixture("missing_js_namespace_on_impl_suggests_struct_namespace")
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "class `Foo` referenced by an impl block");
    assert_contains!(
        &err,
        "a struct with the same `js_name` exists in a different namespace"
    );
    assert_contains!(&err, "js_namespace = ns");
    assert_contains!(&err, "ns__Foo");
}

/// Same shape with a nested namespace (`["a", "b"]`). The hint must list
/// every segment so the user can copy-paste it onto the impl block.
#[test]
fn missing_js_namespace_on_impl_nested_namespace() {
    let err = fixture("missing_js_namespace_on_impl_nested_namespace")
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "js_namespace = a, b");
}

/// A typo in `js_class` produces a "did you mean ...?" fuzzy hint sourced
/// from the registered struct names. Ranking is by edit distance so the
/// closest candidate appears first.
#[test]
fn typo_in_js_class_suggests_nearest_struct() {
    let err = fixture("typo_in_js_class_suggests_nearest_struct")
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_contains!(&err, "class `Countr` referenced by an impl block");
    assert_contains!(&err, "did you mean `Counter`?");
}

// Note: the `extends = ParentPath` failure-with-suggestion path is wired
// in `write_class` (using the same suggestion helper as
// `validate_impl_class_references`) but it's awkward to trigger from an
// integration test without also tripping a Rust-level compilation error
// (a non-`#[wasm_bindgen]` parent type fails the `Parent<T>` trait bound
// the macro injects). The helper itself has unit coverage in
// `cli-support/src/suggest.rs::tests` and the wider hint-formatting
// behaviour is exercised by `typo_in_js_class_suggests_nearest_struct`
// above, so we deliberately don't duplicate the integration test here.
