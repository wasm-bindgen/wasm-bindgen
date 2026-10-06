#[test]
fn excluded_primitive_union_types() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("ui-tests/primitive-union-excluded-types.rs");
}
