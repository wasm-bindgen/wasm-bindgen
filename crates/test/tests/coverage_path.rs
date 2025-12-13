#[wasm_bindgen_test::wasm_bindgen_test]
fn test_coverage_path() {
    fn asssert<'a>(env: impl Into<Option<&'a str>>, result: &str) {
        let env: Option<&str> = env.into();
        assert_eq!(
            wasm_bindgen_test::__rt::node::__wbgtest_coverage_path(
                env.map(|x| x.to_string()),
                123,
                "tmp",
                456
            ),
            result
        );
    }

    asssert(None, "default_456_0_123.profraw");
    asssert("", "");
    asssert("%p", "123");
    asssert("%h", "wbgt");
    asssert("%t", "tmp");
    asssert("%m", "456_0");
    asssert("%0123456789m", "456_0");
    asssert("%", "%");
    asssert("%%", "%%");
    asssert("%a", "%a");
    asssert("%0123456789", "%0123456789");
    asssert("%0123456789p", "%0123456789p");
    asssert("%%p", "%123");
    asssert("%%%p", "%%123");
    asssert("%a%p", "%a123");
    asssert("%0123456789%p", "%0123456789123");
    asssert("%p%", "123%");
    asssert("%p%%", "123%%");
    asssert("%p%a", "123%a");
    asssert("%p%0123456789", "123%0123456789");
    asssert("%p%0123456789p", "123%0123456789p");
    asssert("%m%a", "456_0%a");
    asssert("%m%0123456789", "456_0%0123456789");
    asssert("%m%0123456789p", "456_0%0123456789p");
    asssert("%0123456789m%a", "456_0%a");
    asssert("%0123456789m%0123456789", "456_0%0123456789");
    asssert("%0123456789m%0123456789p", "456_0%0123456789p");
}
