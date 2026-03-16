//! @ts-gen --lib-name es-module-lexer ../ts-gen/tests/fixtures/es-module-lexer.d.ts
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use ts_gen_integration_tests::es_module_lexer::*;

/// Helper: await the init promise before calling parse.
async fn ensure_init() {
    let promise = init.with(|v| v.clone());
    JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_parse_static_import() {
    ensure_init().await;

    // parse(source) returns ArrayTuple directly (no catch).
    // All types infer through get0/get1/get.
    let result = parse("import { foo } from 'bar'");

    let imports = result.get0();
    let exports = result.get1();

    assert_eq!(imports.length(), 1);
    assert_eq!(exports.length(), 0);

    // Access the first import specifier — get() returns ImportSpecifier
    let spec = imports.get(0);

    // Module name should be "bar" — JsString implements PartialEq<str>
    // Getters have no catch, so n() returns Option<JsString> directly
    let module_name = spec.n().unwrap();
    assert_eq!(module_name, "bar");

    // Import type should be Static (1)
    let import_type = spec.t();
    assert_eq!(import_type, ImportType::Static);

    // d should be -1 for static imports
    let d = spec.d();
    assert_eq!(d, -1.0);
}

#[wasm_bindgen_test]
async fn test_parse_dynamic_import() {
    ensure_init().await;

    let result = parse("import('foo')");
    let imports = result.get0();

    assert_eq!(imports.length(), 1);

    let spec = imports.get(0);
    let module_name = spec.n().unwrap();
    assert_eq!(module_name, "foo");

    let import_type = spec.t();
    assert_eq!(import_type, ImportType::Dynamic);

    // d >= 0 for dynamic imports (start of the import expression)
    let d = spec.d();
    assert!(d >= 0.0);
}

#[wasm_bindgen_test]
async fn test_parse_exports() {
    ensure_init().await;

    let result = parse("export const answer = 42");
    let exports = result.get1();

    assert_eq!(exports.length(), 1);

    let spec = exports.get(0);
    let name = spec.n();
    assert_eq!(name, "answer");
}

#[wasm_bindgen_test]
async fn test_parse_reexport() {
    ensure_init().await;

    let source = "export { foo as bar } from 'baz'";
    let result = parse(source);

    let imports = result.get0();
    let exports = result.get1();

    assert_eq!(imports.length(), 1);
    assert_eq!(exports.length(), 1);

    let imp = imports.get(0);
    assert_eq!(imp.n().unwrap(), "baz");

    let exp = exports.get(0);
    assert_eq!(exp.n(), "bar");
}

#[wasm_bindgen_test]
async fn test_parse_multiple_imports() {
    ensure_init().await;

    let source = r#"
        import { a } from 'mod-a'
        import { b } from 'mod-b'
        import('mod-c')
    "#;

    let result = parse(source);
    let imports = result.get0();

    assert_eq!(imports.length(), 3);

    let spec0 = imports.get(0);
    let spec1 = imports.get(1);
    let spec2 = imports.get(2);

    assert_eq!(spec0.n().unwrap(), "mod-a");
    assert_eq!(spec0.t(), ImportType::Static);

    assert_eq!(spec1.n().unwrap(), "mod-b");
    assert_eq!(spec1.t(), ImportType::Static);

    assert_eq!(spec2.n().unwrap(), "mod-c");
    assert_eq!(spec2.t(), ImportType::Dynamic);
}

#[wasm_bindgen_test]
async fn test_import_specifier_offsets() {
    ensure_init().await;

    let source = "import { a } from 'asdf'";
    let result = parse(source);
    let imports = result.get0();

    let spec = imports.get(0);

    // s and e are the start/end of the module specifier string
    let s = spec.s() as usize;
    let e = spec.e() as usize;
    assert_eq!(&source[s..e], "asdf");

    // ss and se are the start/end of the full import statement
    let ss = spec.ss() as usize;
    let se = spec.se() as usize;
    // The full statement includes "import { a } from 'asdf'"
    assert!(source[ss..se].starts_with("import"));
}
