use crate::{Project, REPO_ROOT};

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
    let err = Project::new("no_modules_rejects_npm")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "foo")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen(start)]
                fn main() {
                    foo();
                }
            "#,
        )
        .file("package.json", "")
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
    Project::new("more_package_json_fields_ignored")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "foo")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen(start)]
                fn main() {
                    foo();
                }
            "#,
        )
        .file(
            "package.json",
            r#"
                {
                    "name": "foo",
                    "dependencies": {}
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap();
}

#[test]
fn npm_conflict_rejected() {
    let err = Project::new("npm_conflict_rejected")
        .dep("bar = { path = 'bar' }")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "bar")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen(start)]
                fn main() {
                    foo();
                    bar::foo();
                }
            "#,
        )
        .file(
            "package.json",
            r#"
                {
                    "dependencies": {"bar": "0.0.0"}
                }
            "#,
        )
        .file(
            "bar/Cargo.toml",
            &format!(
                r#"
                [package]
                name = "bar"
                authors = []
                version = "1.0.0"
                edition = '2021'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}
            "#,
                REPO_ROOT.display()
            ),
        )
        .file(
            "bar/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "bar")]
                extern {
                    pub fn foo();
                }
            "#,
        )
        .file(
            "bar/package.json",
            r#"
                {
                    "dependencies": {"bar": "1.0.0"}
                }
            "#,
        )
        .wasm_bindgen("")
        .unwrap_err()
        .to_string();

    assert_matches!(&err, "dependency on NPM package `bar` specified in two");
}
