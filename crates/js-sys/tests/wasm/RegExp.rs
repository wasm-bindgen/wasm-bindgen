use js_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

fn has_regexp_escape() -> bool {
    let ctor: Object = Reflect::get_str(js_sys::global().as_ref(), &"RegExp".into())
        .unwrap()
        .unwrap()
        .unchecked_into();
    Reflect::has_str(&ctor, &"escape".into()).unwrap_or(false)
}

#[wasm_bindgen_test]
fn regexp_inheritance() {
    let re = RegExp::new(".", "");
    assert!(re.is_instance_of::<RegExp>());
    assert!(re.is_instance_of::<Object>());
    let _: &Object = re.as_ref();
}

#[wasm_bindgen_test]
fn exec() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let re = RegExp::new("quick\\s(brown).+?(jumps)", "ig");
        let result = re.exec("The Quick Brown Fox Jumps Over The Lazy Dog");

        let mut v = vec![];
        result.unwrap().for_each(&mut |x, _, _| v.push(x));

        assert_eq!(v[0], "Quick Brown Fox Jumps");
        assert_eq!(v[1], "Brown");
        assert_eq!(v[2], "Jumps");

        let result = re.exec("foo");
        assert!(result.is_none());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let re = RegExp::new("quick\\s(brown).+?(jumps)", "ig");
        let result = re.exec("The Quick Brown Fox Jumps Over The Lazy Dog");

        let m = result.unwrap();
        // Access via typed getters
        assert_eq!(m.get(0).unwrap(), "Quick Brown Fox Jumps");
        assert_eq!(m.get(1).unwrap(), "Brown");
        assert_eq!(m.get(2).unwrap(), "Jumps");
        assert_eq!(m.index(), 4);
        assert_eq!(m.input(), "The Quick Brown Fox Jumps Over The Lazy Dog");

        let result = re.exec("foo");
        assert!(result.is_null());
    }
}

#[wasm_bindgen_test]
fn flags() {
    let re = RegExp::new("foo", "ig");
    assert_eq!(re.flags(), "gi");
}

#[wasm_bindgen_test]
fn global() {
    let re = RegExp::new("foo", "g");
    assert!(re.global());

    let re = RegExp::new("bar", "i");
    assert!(!re.global());
}

#[wasm_bindgen_test]
fn ignore_case() {
    let re = RegExp::new("foo", "");
    assert!(!re.ignore_case());

    let re = RegExp::new("foo", "i");
    assert!(re.ignore_case());
}

#[wasm_bindgen_test]
fn input() {
    let re = RegExp::new("hi", "g");
    re.test("hi there!");
    assert_eq!(RegExp::input(), "hi there!");
}

#[wasm_bindgen_test]
fn last_index() {
    let re = RegExp::new("hi", "g");
    assert_eq!(re.last_index(), 0);

    re.set_last_index(42);
    assert_eq!(re.last_index(), 42);
}

#[wasm_bindgen_test]
fn last_match() {
    let re = RegExp::new("hi", "g");
    re.test("hi there!");
    assert_eq!(RegExp::last_match(), "hi");
}

#[wasm_bindgen_test]
fn last_paren() {
    let re = RegExp::new("(hi)", "g");
    re.test("hi there!");
    assert_eq!(RegExp::last_paren(), "hi");
}

#[wasm_bindgen_test]
fn left_context() {
    let re = RegExp::new("world", "g");
    re.test("hello world!");
    assert_eq!(RegExp::left_context(), "hello ");
}

#[wasm_bindgen_test]
fn multiline() {
    let re = RegExp::new("foo", "m");
    assert!(re.multiline());
}

#[wasm_bindgen_test]
fn n1_to_n9() {
    let re = RegExp::new(
        r"(\w+)\s(\w+)\s(\w+)\s(\w+)\s(\w+)\s(\w+)\s(\w+)\s(\w+)\s(\w+)",
        "",
    );
    re.test("The Quick Brown Fox Jumps Over The Lazy Dog");
    assert_eq!(RegExp::n1(), "The");
    assert_eq!(RegExp::n2(), "Quick");
    assert_eq!(RegExp::n3(), "Brown");
    assert_eq!(RegExp::n4(), "Fox");
    assert_eq!(RegExp::n5(), "Jumps");
    assert_eq!(RegExp::n6(), "Over");
    assert_eq!(RegExp::n7(), "The");
    assert_eq!(RegExp::n8(), "Lazy");
    assert_eq!(RegExp::n9(), "Dog");
}

#[wasm_bindgen_test]
fn new() {
    let re = RegExp::new("foo", "");
    let re = RegExp::new_regexp(&re, "g");
    assert_eq!(re.to_js_string(), "/foo/g");
}

#[wasm_bindgen_test]
fn right_context() {
    let re = RegExp::new("hello", "g");
    re.test("hello world!");
    assert_eq!(RegExp::right_context(), " world!");
}

#[wasm_bindgen_test]
fn source() {
    let re = RegExp::new("fooBar", "ig");
    assert_eq!(re.source(), "fooBar");

    let re = RegExp::new("", "ig");
    assert_eq!(re.source(), "(?:)");
}

#[wasm_bindgen_test]
fn sticky() {
    let re = RegExp::new("foo", "y");
    assert!(re.sticky());
}

#[wasm_bindgen_test]
fn test() {
    let re = RegExp::new("foo", "");
    assert!(re.test("football"));
    assert!(!re.test("bar"));
}

#[allow(deprecated)]
#[wasm_bindgen_test]
fn to_string() {
    let re = RegExp::new("a+b+c", "g");
    assert_eq!(re.to_string(), "/a+b+c/g");
}

#[wasm_bindgen_test]
fn unicode() {
    let re = RegExp::new("\u{61}", "u");
    assert!(re.unicode());
}

#[wasm_bindgen_test]
fn dot_all() {
    let re = RegExp::new("foo.bar", "s");
    assert!(re.dot_all());
    assert!(re.test("foo\nbar"));

    let re = RegExp::new("foo.bar", "");
    assert!(!re.dot_all());
    assert!(!re.test("foo\nbar"));
}

#[wasm_bindgen_test]
fn has_indices() {
    let re = RegExp::new("foo", "d");
    assert!(re.has_indices());

    let re = RegExp::new("foo", "g");
    assert!(!re.has_indices());
}

#[wasm_bindgen_test]
fn unicode_sets() {
    let re = RegExp::new(r"[\p{L}--[a-z]]", "v");
    assert!(re.unicode_sets());
    assert!(re.test("A"));
    assert!(!re.test("a"));

    let re = RegExp::new("foo", "u");
    assert!(!re.unicode_sets());
}

#[wasm_bindgen_test]
fn escape() {
    if !has_regexp_escape() {
        return;
    }

    let escaped = RegExp::escape("a.b*c");
    assert_eq!(escaped, r"\x61\.b\*c");
    let re = RegExp::new(&String::from(&escaped), "");
    assert!(re.test("a.b*c"));
    assert!(!re.test("axb*c"));
    assert!(!re.test("abbc"));

    assert_eq!(RegExp::escape(""), "");
    assert_eq!(RegExp::escape("-"), r"\x2d");
}
