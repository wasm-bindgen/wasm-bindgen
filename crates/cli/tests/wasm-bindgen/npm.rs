use crate::fixture;

macro_rules! assert_matches {
    ($haystack:expr, $needle:literal) => {
        let haystack = $haystack;
        let re = regex::Regex::new($needle).unwrap();

        assert!(
            re.is_match(haystack),
            "Expected\n{haystack:?}\nto match\n{re:?}"
        );
    };
}

#[test]
fn no_modules_rejects_npm() {
    let err = fixture("no_modules_rejects_npm")
        .wasm_bindgen("--no-modules")
        .unwrap_err()
        .to_string();

    assert_matches!(
        &err,
        "NPM dependencies have been specified in `.*` but this is incompatible with the `no-modules` target"
    );
}

#[test]
fn more_package_json_fields_ignored() {
    fixture("more_package_json_fields_ignored")
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn npm_conflict_rejected() {
    let err = fixture("npm_conflict_rejected")
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_matches!(&err, "dependency on NPM package `bar` specified in two");
}
