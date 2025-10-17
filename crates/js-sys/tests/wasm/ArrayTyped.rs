use js_sys::{Array, JsString, Number};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/ArrayTyped.js")]
extern "C" {
    #[derive(Clone)]
    pub type TestItem;

    #[wasm_bindgen(constructor)]
    fn new(id: u32, name: &JsString) -> TestItem;

    #[wasm_bindgen(method, getter)]
    fn id(this: &TestItem) -> u32;

    #[wasm_bindgen(method, getter)]
    fn name(this: &TestItem) -> JsString;

    #[wasm_bindgen(method)]
    fn with_prefix(this: &TestItem, prefix: &JsString) -> TestItem;
}

#[wasm_bindgen(module = "tests/wasm/ArrayTyped.js")]
extern "C" {
    #[wasm_bindgen(js_name = "createTestItemArray")]
    fn create_test_item_array() -> Array<TestItem>;

    #[wasm_bindgen(js_name = "processTestItemArray")]
    fn process_test_item_array(arr: &Array<TestItem>) -> u32;

    #[wasm_bindgen(js_name = "checkArrayType")]
    fn check_array_type(arr: &Array<TestItem>) -> bool;

    #[wasm_bindgen(js_name = "createThrowingIterable")]
    fn create_throwing_iterable() -> JsValue;
}

#[wasm_bindgen_test]
fn test_array_new_typed() {
    let arr: Array<TestItem> = Array::new_typed();
    assert_eq!(arr.length(), 0);
}

#[wasm_bindgen_test]
fn test_array_new_with_length_typed() {
    let arr: Array<TestItem> = Array::new_with_length_typed(5);
    assert_eq!(arr.length(), 5);
}

#[wasm_bindgen_test]
fn test_array_of() {
    let arr: Array<TestItem> = Array::of(&[
        TestItem::new(4, &JsString::from("d")),
        TestItem::new(5, &JsString::from("e")),
    ]);
    assert_eq!(arr.length(), 2);
    assert_eq!(arr.get(1).id(), 5);
}

#[wasm_bindgen_test]
fn test_array_get_set() {
    let arr: Array<TestItem> = Array::new_typed();
    let item = TestItem::new(1, &JsString::from("first"));

    arr.set(0, item);
    assert_eq!(arr.length(), 1);

    let retrieved: TestItem = arr.get(0);
    assert_eq!(retrieved.id(), 1);
    assert_eq!(retrieved.name(), "first");
}

#[wasm_bindgen_test]
fn test_array_get_checked() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let item: Option<TestItem> = arr.get_checked(0);
    assert!(item.is_some());
    assert_eq!(item.unwrap().id(), 1);

    let missing: Option<TestItem> = arr.get_checked(99);
    assert!(missing.is_none());

    let sparse: Array<TestItem> = Array::new_with_length_typed(5);
    sparse.set(2, TestItem::new(42, &JsString::from("middle")));
    assert!(sparse.get_checked(0).is_none());
    assert!(sparse.get_checked(2).is_some());
}

#[wasm_bindgen_test]
fn test_array_at() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    let item: TestItem = arr.at(1);
    assert_eq!(item.id(), 2);
    let last: TestItem = arr.at(-1);
    assert_eq!(last.id(), 3);
}

#[wasm_bindgen_test]
fn test_array_delete() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    arr.delete(0);

    assert_eq!(arr.length(), 2);
    assert!(arr.get(0).is_undefined());
}

#[wasm_bindgen_test]
fn test_array_push_pop() {
    let arr: Array<TestItem> = Array::new_typed();

    let len = arr.push(&TestItem::new(1, &JsString::from("first")));
    assert_eq!(len, 1);

    let len = arr.push(&TestItem::new(2, &JsString::from("second")));
    assert_eq!(len, 2);

    let more = [
        TestItem::new(3, &JsString::from("third")),
        TestItem::new(4, &JsString::from("fourth")),
    ];
    let len = arr.push_many(&more);
    assert_eq!(len, 4);

    let popped: TestItem = arr.pop();
    assert_eq!(popped.id(), 4);
    assert_eq!(arr.length(), 3);

    let popped: TestItem = arr.pop();
    assert_eq!(popped.id(), 3);

    let popped: TestItem = arr.pop();
    assert_eq!(popped.id(), 2);
    assert_eq!(arr.length(), 1);

    let popped: TestItem = arr.pop();
    assert_eq!(popped.id(), 1);
    assert_eq!(arr.length(), 0);
    assert!(arr.pop().is_undefined());
    assert!(arr.pop_checked().is_none());
}

#[wasm_bindgen_test]
fn test_array_shift() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("first")));
    arr.push(&TestItem::new(2, &JsString::from("second")));

    let shifted: TestItem = arr.shift();
    assert_eq!(shifted.id(), 1);
    assert_eq!(arr.length(), 1);

    let remaining: TestItem = arr.get(0);
    assert_eq!(remaining.id(), 2);
    arr.shift();
    assert!(arr.shift_checked().is_none());

    let items = [
        TestItem::new(1, &JsString::from("a")),
        TestItem::new(2, &JsString::from("b")),
    ];
    let len = arr.unshift_many(&items);
    assert_eq!(len, 2);
    assert_eq!(arr.get(0).id(), 1);
    assert_eq!(arr.get(1).id(), 2);
}

#[wasm_bindgen_test]
fn test_array_concat() {
    let arr1: Array<TestItem> = Array::new_typed();
    arr1.push(&TestItem::new(1, &JsString::from("a")));

    let arr2: Array<TestItem> = Array::new_typed();
    arr2.push(&TestItem::new(2, &JsString::from("b")));

    let combined = arr1.concat(&arr2);
    assert_eq!(combined.length(), 2);

    let first: TestItem = combined.get(0);
    let second: TestItem = combined.get(1);
    assert_eq!(first.id(), 1);
    assert_eq!(second.id(), 2);
}

#[wasm_bindgen_test]
fn test_array_reverse() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let reversed = arr.reverse();

    let first: TestItem = reversed.get(0);
    let last: TestItem = reversed.get(2);
    assert_eq!(first.id(), 3);
    assert_eq!(last.id(), 1);
}

#[wasm_bindgen_test]
fn test_array_copy_within() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));
    let result = arr.copy_within(2, 0, 2);

    let at2: TestItem = result.get(2);
    let at3: TestItem = result.get(3);
    assert_eq!(at2.id(), 1);
    assert_eq!(at3.id(), 2);
}

#[wasm_bindgen_test]
fn test_array_splice() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let items = [
        TestItem::new(10, &JsString::from("x")),
        TestItem::new(11, &JsString::from("y")),
    ];
    let removed = arr.splice_many(0, 1, &items);
    assert_eq!(removed.get(0).id(), 1);
    assert_eq!(arr.get(0).id(), 10);
    assert_eq!(arr.get(1).id(), 11);

    let spliced = arr.to_spliced(0, 2, &[]);
    assert_eq!(spliced.length(), 2);

    let items = [TestItem::new(99, &JsString::from("new"))];
    let spliced = arr.to_spliced(0, 1, &items);
    assert_eq!(spliced.get(0).id(), 99);

    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    arr.splice_many(1, 1, &[]);
}

#[wasm_bindgen_test]
fn test_array_iter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let ids: Vec<u32> = arr
        .iter()
        .map(|item| {
            let t: TestItem = item;
            t.id()
        })
        .collect();

    assert_eq!(ids, vec![1, 2, 3]);
}

#[wasm_bindgen_test]
fn test_array_into_iter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(10, &JsString::from("x")));
    arr.push(&TestItem::new(20, &JsString::from("y")));

    let mut sum = 0;
    for item in arr {
        let t: TestItem = item;
        sum += t.id();
    }
    assert_eq!(sum, 30);
}

#[wasm_bindgen_test]
fn test_array_to_vec() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let vec: Vec<TestItem> = arr.to_vec();
    assert_eq!(vec.len(), 2);

    let first = vec[0].clone();
    let second = vec[1].clone();
    assert_eq!(first.id(), 1);
    assert_eq!(second.id(), 2);
}

#[wasm_bindgen_test]
fn test_array_from_iter() {
    let items = vec![
        TestItem::new(1, &JsString::from("a")),
        TestItem::new(2, &JsString::from("b")),
        TestItem::new(3, &JsString::from("c")),
    ];

    let arr: Array<TestItem> = items.iter().map(|i| i).collect();

    assert_eq!(arr.length(), 3);
    let first: TestItem = arr.get(0);
    assert_eq!(first.id(), 1);
}

#[wasm_bindgen_test]
fn test_array_extend() {
    let mut arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let more = vec![
        TestItem::new(2, &JsString::from("b")),
        TestItem::new(3, &JsString::from("c")),
    ];
    arr.extend(more);

    assert_eq!(arr.length(), 3);
}

#[wasm_bindgen_test]
fn test_array_find() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("apple")));
    arr.push(&TestItem::new(2, &JsString::from("banana")));
    arr.push(&TestItem::new(3, &JsString::from("cherry")));

    let found: TestItem = arr.find(&mut |val: TestItem, _, _| val.id() == 2);
    assert_eq!(found.name(), "banana");
}

#[wasm_bindgen_test]
fn test_array_find_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let idx = arr.find_index(&mut |val: TestItem, _, _| val.id() == 2);
    assert_eq!(idx, 1);

    let not_found = arr.find_index(&mut |val: TestItem, _, _| val.id() == 99);
    assert_eq!(not_found, -1);
}

#[wasm_bindgen_test]
fn test_array_filter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let evens: Array<TestItem> = arr.filter(&mut |val: TestItem, _, _| val.id() % 2 == 0);
    assert_eq!(evens.length(), 2);
    assert_eq!(evens.get(0).id(), 2);
}

#[wasm_bindgen_test]
fn test_array_every() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(4, &JsString::from("b")));
    arr.push(&TestItem::new(6, &JsString::from("c")));

    let all_even = arr.every(&mut |val: TestItem, _, _| val.id() % 2 == 0);
    assert!(all_even);

    arr.push(&TestItem::new(7, &JsString::from("d")));

    let still_all_even = arr.every(&mut |val: TestItem, _, _| val.id() % 2 == 0);
    assert!(!still_all_even);
}

#[wasm_bindgen_test]
fn test_array_from_js() {
    let arr = create_test_item_array();

    assert_eq!(arr.length(), 3);

    let first: TestItem = arr.get(0);
    assert_eq!(first.id(), 1);
}

#[wasm_bindgen_test]
fn test_array_try_from_catches_error() {
    use wasm_bindgen::JsCast;

    let throwing_iterable = create_throwing_iterable();
    let result: Result<Array, JsValue> = Array::from_iterable(&throwing_iterable);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.is_instance_of::<js_sys::Error>());
    let error: &js_sys::Error = err.unchecked_ref();
    assert_eq!(error.message(), "iterator error");
}

#[wasm_bindgen_test]
fn test_array_to_js() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(10, &JsString::from("x")));
    arr.push(&TestItem::new(20, &JsString::from("y")));

    let sum = process_test_item_array(&arr);
    assert_eq!(sum, 30);
}

#[wasm_bindgen_test]
fn test_array_type_preserved() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    assert!(check_array_type(&arr));
}

#[wasm_bindgen]
pub fn rust_create_test_item_array() -> Array<TestItem> {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(100, &JsString::from("rust1")));
    arr.push(&TestItem::new(200, &JsString::from("rust2")));
    arr
}

#[wasm_bindgen_test]
fn test_rust_export_ArrayTyped() {
    let arr = rust_create_test_item_array();
    assert_eq!(arr.length(), 2);

    let first: TestItem = arr.get(0);
    assert_eq!(first.id(), 100);
    assert_eq!(first.name(), "rust1");
}

#[wasm_bindgen_test]
fn test_array_slice_vec() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let sliced: Vec<TestItem> = arr.slice(1, 3).to_vec();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
    assert_eq!(sliced[1].id(), 3);

    assert_eq!(arr.length(), 4);
}

#[wasm_bindgen_test]
fn test_array_slice_box() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let sliced: Box<[TestItem]> = arr.slice(1, 3).to_vec().into_boxed_slice();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
    assert_eq!(sliced[1].id(), 3);

    assert_eq!(arr.length(), 4);
}

#[wasm_bindgen_test]
fn test_array_to_reversed() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let reversed: Array<TestItem> = arr.to_reversed();
    assert_eq!(reversed.length(), 3);
    assert_eq!(reversed.get(0).id(), 3);
    assert_eq!(reversed.get(2).id(), 1);
    assert_eq!(arr.get(0).id(), 1);
}

#[wasm_bindgen_test]
fn test_array_to_sorted() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let sorted: Array<TestItem> = arr.to_sorted();
    assert_eq!(sorted.length(), 3);
    assert_eq!(arr.get(0).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_with() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let new_arr = arr.with(1, &TestItem::new(99, &JsString::from("replaced")));
    assert_eq!(new_arr.get(1).id(), 99);
    assert_eq!(arr.get(1).id(), 2);
}

#[wasm_bindgen_test]
fn test_array_slice_from() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let sliced: Array<TestItem> = arr.slice_from(2);
    assert_eq!(sliced.length(), 2);
    assert_eq!(sliced.get(0).id(), 3);
    assert_eq!(sliced.get(1).id(), 4);
}

#[wasm_bindgen_test]
fn test_array_slice_from_vec() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let sliced: Vec<TestItem> = arr.slice_from(1).to_vec();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
    assert_eq!(sliced[1].id(), 3);
}

#[wasm_bindgen_test]
fn test_array_slice_from_boxed_slice() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let sliced: Box<[TestItem]> = arr.slice_from(1).to_vec().into_boxed_slice();
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced[0].id(), 2);
}

#[wasm_bindgen_test]
fn test_array_to_boxed_slice() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let boxed: Box<[TestItem]> = arr.to_vec().into_boxed_slice();
    assert_eq!(boxed.len(), 2);
    assert_eq!(boxed[0].id(), 1);
    assert_eq!(boxed[1].id(), 2);
}

#[wasm_bindgen_test]
fn test_array_fill() {
    let arr: Array<TestItem> = Array::new_with_length_typed(3);
    let filler = TestItem::new(42, &JsString::from("fill"));
    arr.fill(&filler, 0, 3);
    assert_eq!(arr.get(0).id(), 42);
    assert_eq!(arr.get(1).id(), 42);
    assert_eq!(arr.get(2).id(), 42);
}

#[wasm_bindgen_test]
fn test_array_includes() {
    let arr: Array<TestItem> = Array::new_typed();
    let item1 = TestItem::new(1, &JsString::from("a"));
    let item2 = TestItem::new(2, &JsString::from("b"));
    arr.push(&item1);
    arr.push(&item2);

    assert!(arr.includes(&item1, 0));
    assert!(arr.includes(&item2, 0));
}

#[wasm_bindgen_test]
fn test_array_index_of() {
    let arr: Array<TestItem> = Array::new_typed();
    let item1 = TestItem::new(1, &JsString::from("a"));
    let item2 = TestItem::new(2, &JsString::from("b"));
    arr.push(&item1);
    arr.push(&item2);
    arr.push(&item1);

    assert_eq!(arr.index_of(&item1, 0), 0);
    assert_eq!(arr.index_of(&item1, 1), 2);
    assert_eq!(arr.index_of(&item2, 0), 1);
}

#[wasm_bindgen_test]
fn test_array_last_index_of() {
    let arr: Array<TestItem> = Array::new_typed();
    let item1 = TestItem::new(1, &JsString::from("a"));
    let item2 = TestItem::new(2, &JsString::from("b"));
    arr.push(&item1);
    arr.push(&item2);
    arr.push(&item1);

    assert_eq!(arr.last_index_of(&item1, 3), 2);
    assert_eq!(arr.last_index_of(&item1, 1), 0);
}

#[wasm_bindgen_test]
fn test_array_find_last() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let found: TestItem = arr.find_last(&mut |item: TestItem, _, _| item.id() > 1);
    assert_eq!(found.id(), 3);
}

#[wasm_bindgen_test]
fn test_array_find_last_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let idx = arr.find_last_index(&mut |item: TestItem, _, _| item.id() > 1);
    assert_eq!(idx, 2);
}

#[wasm_bindgen_test]
fn test_array_join() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("a"));
    arr.push(&JsString::from("b"));
    arr.push(&JsString::from("c"));

    let joined = arr.join(",");
    assert_eq!(joined, "a,b,c");
}

#[wasm_bindgen_test]
fn test_array_set_length() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    assert_eq!(arr.length(), 3);

    arr.set_length(2);
    assert_eq!(arr.length(), 2);
}

#[wasm_bindgen_test]
fn test_array_unshift() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let new_len = arr.unshift(&TestItem::new(1, &JsString::from("a")));
    assert_eq!(new_len, 3);
    assert_eq!(arr.get(0).id(), 1);
}

#[wasm_bindgen_test]
fn test_array_flat() {
    let inner1: Array<JsString> = Array::new_typed();
    inner1.push(&JsString::from("a"));
    inner1.push(&JsString::from("b"));

    let inner2: Array<JsString> = Array::new_typed();
    inner2.push(&JsString::from("c"));

    let outer: Array = Array::new_typed();
    outer.push(&inner1.into());
    outer.push(&inner2.into());

    let flat: Array = outer.flat(1);
    assert_eq!(flat.length(), 3);
}

#[wasm_bindgen_test]
fn test_array_to_spliced() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let spliced: Array<TestItem> = arr.to_spliced(1, 1, &[]);
    assert_eq!(spliced.length(), 2);
    assert_eq!(spliced.get(0).id(), 1);
    assert_eq!(spliced.get(1).id(), 3);
    assert_eq!(arr.length(), 3);
}

#[wasm_bindgen_test]
fn test_array_to_string() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("x"));
    arr.push(&JsString::from("y"));

    let s = arr.to_string();
    assert_eq!(s, "x,y");
}

#[wasm_bindgen_test]
fn test_array_push_many() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let items = [
        TestItem::new(2, &JsString::from("b")),
        TestItem::new(3, &JsString::from("c")),
    ];
    let new_len = arr.push_many(&items);
    assert_eq!(new_len, 3);
    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(1).id(), 2);
    assert_eq!(arr.get(2).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_sort_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let sorted = arr.sort_by(&mut |a, b| (a.id() as i32) - (b.id() as i32));
    assert_eq!(sorted.get(0).id(), 1);
    assert_eq!(sorted.get(1).id(), 2);
    assert_eq!(sorted.get(2).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_to_sorted_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    let sorted: Array<TestItem> = arr.to_sorted_by(&mut |a, b| (b.id() as i32) - (a.id() as i32));
    assert_eq!(sorted.get(0).id(), 3);
    assert_eq!(sorted.get(1).id(), 2);
    assert_eq!(sorted.get(2).id(), 1);
    assert_eq!(arr.get(0).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_splice_many() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let insert = [
        TestItem::new(10, &JsString::from("x")),
        TestItem::new(11, &JsString::from("y")),
    ];
    let removed = arr.splice_many(1, 1, &insert);
    assert_eq!(removed.length(), 1);
    assert_eq!(removed.get(0).id(), 2);
    assert_eq!(arr.length(), 4);
    assert_eq!(arr.get(1).id(), 10);
    assert_eq!(arr.get(2).id(), 11);
}

#[wasm_bindgen_test]
fn test_array_entries_typed() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(10, &JsString::from("a")));
    arr.push(&TestItem::new(20, &JsString::from("b")));
    arr.push(&TestItem::new(30, &JsString::from("c")));

    let entries = arr.entries_typed();
    let mut count = 0u32;
    for result in entries.into_iter() {
        let tuple: js_sys::ArrayTuple<u32, TestItem> = result.unwrap();
        let arr_ref: &js_sys::Array = wasm_bindgen::JsCast::unchecked_ref(&tuple);
        assert_eq!(arr_ref.get(0), count);
        count += 1;
    }
    assert_eq!(count, 3);
}

#[wasm_bindgen_test]
fn test_from_iterable_map() {
    let source: Array<Number> = Array::of(&[Number::from(1), Number::from(2), Number::from(3)]);

    let result: Array<Number> = Array::from_iterable_map(&source, &mut |val: Number, _idx: u32| {
        Ok(Number::from(val.value_of() * 2.0))
    })
    .unwrap();

    assert_eq!(result.length(), 3);
    assert_eq!(result.get(0).value_of(), 2.0);
    assert_eq!(result.get(1).value_of(), 4.0);
    assert_eq!(result.get(2).value_of(), 6.0);
}

#[wasm_bindgen_test]
fn test_array_try_every() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(2, &JsString::from("a")));
    arr.push(&TestItem::new(4, &JsString::from("b")));
    arr.push(&TestItem::new(6, &JsString::from("c")));

    let result = arr.try_every(&mut |val: TestItem, _| Ok(val.id() % 2 == 0));
    assert!(result.is_ok());
    assert!(result.unwrap());

    arr.push(&TestItem::new(7, &JsString::from("d")));
    let result = arr.try_every(&mut |val: TestItem, _| Ok(val.id() % 2 == 0));
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[wasm_bindgen_test]
fn test_array_try_every_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_every(&mut |val: TestItem, _| {
        if val.id() == 2 {
            Err(JsValue::from_str("error at 2"))
        } else {
            Ok(true)
        }
    });
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_filter() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(4, &JsString::from("d")));

    let result = arr.try_filter(&mut |val: TestItem, _| Ok(val.id() % 2 == 0));
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert_eq!(filtered.length(), 2);
    assert_eq!(filtered.get(0).id(), 2);
    assert_eq!(filtered.get(1).id(), 4);
}

#[wasm_bindgen_test]
fn test_array_try_filter_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_filter(&mut |val: TestItem, _| {
        if val.id() == 2 {
            Err(JsValue::from_str("filter error"))
        } else {
            Ok(true)
        }
    });
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("apple")));
    arr.push(&TestItem::new(2, &JsString::from("banana")));
    arr.push(&TestItem::new(3, &JsString::from("cherry")));

    let result = arr.try_find(&mut |val: TestItem, _| Ok(val.id() == 2));
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "banana");

    let result = arr.try_find(&mut |val: TestItem, _| Ok(val.id() == 99));
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[wasm_bindgen_test]
fn test_array_try_find_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_find(&mut |_val: TestItem, _| Err(JsValue::from_str("find error")));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_find_index(&mut |val: TestItem, _| Ok(val.id() == 2));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let result = arr.try_find_index(&mut |val: TestItem, _| Ok(val.id() == 99));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[wasm_bindgen_test]
fn test_array_try_find_index_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result =
        arr.try_find_index(&mut |_val: TestItem, _| Err(JsValue::from_str("find_index error")));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find_last() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_find_last(&mut |val: TestItem, _| Ok(val.id() > 1));
    assert!(result.is_ok());
    let found = result.unwrap();
    assert_eq!(found.id(), 3);
}

#[wasm_bindgen_test]
fn test_array_try_find_last_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result =
        arr.try_find_last(&mut |_val: TestItem, _| Err(JsValue::from_str("find_last error")));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_find_last_index() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_find_last_index(&mut |val: TestItem, _| Ok(val.id() > 1));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[wasm_bindgen_test]
fn test_array_try_find_last_index_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_find_last_index(&mut |_val: TestItem, _| {
        Err(JsValue::from_str("find_last_index error"))
    });
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_for_each() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let mut sum = 0;
    let result = arr.try_for_each(&mut |val: TestItem, _| {
        sum += val.id();
        Ok(())
    });
    assert!(result.is_ok());
    assert_eq!(sum, 6);
}

#[wasm_bindgen_test]
fn test_array_try_for_each_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_for_each(&mut |val: TestItem, _| {
        if val.id() == 2 {
            Err(JsValue::from_str("for_each error"))
        } else {
            Ok(())
        }
    });
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_map() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result: Result<Array<TestItem>, JsValue> =
        arr.try_map(&mut |val: TestItem, _| Ok(val.with_prefix(&JsString::from("pre_"))));
    assert!(result.is_ok());
    let mapped = result.unwrap();
    assert_eq!(mapped.length(), 3);
    assert_eq!(mapped.get(0).name(), "pre_a");
}

#[wasm_bindgen_test]
fn test_array_try_map_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result: Result<Array<TestItem>, JsValue> = arr.try_map(&mut |val: TestItem, _| {
        if val.id() == 2 {
            Err(JsValue::from_str("map error"))
        } else {
            Ok(val)
        }
    });
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_reduce() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));
    arr.push(&Number::from(3));

    let initial = Number::from(0);
    let result = arr.try_reduce(
        &mut |acc: Number, val: Number, _| Ok(Number::from(acc.value_of() + val.value_of())),
        &initial,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value_of(), 6.0);
}

#[wasm_bindgen_test]
fn test_array_try_reduce_error() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));

    let initial = Number::from(0);
    let result = arr.try_reduce(
        &mut |_acc: Number, val: Number, _| {
            if val.value_of() == 2.0 {
                Err(JsValue::from_str("reduce error"))
            } else {
                Ok(val)
            }
        },
        &initial,
    );
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_reduce_right() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));
    arr.push(&Number::from(3));

    let initial = Number::from(0);
    let result = arr.try_reduce_right(
        &mut |acc: JsValue, val: Number, _| {
            let acc_num: Number = acc.unchecked_into();
            Ok(Number::from(acc_num.value_of() + val.value_of()))
        },
        &initial,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value_of(), 6.0);
}

#[wasm_bindgen_test]
fn test_array_try_reduce_right_error() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1));
    arr.push(&Number::from(2));

    let initial = Number::from(0);
    let result = arr.try_reduce_right(
        &mut |_acc: JsValue, val: Number, _| {
            if val.value_of() == 1.0 {
                Err(JsValue::from_str("reduce_right error"))
            } else {
                Ok(val)
            }
        },
        &initial,
    );
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_some() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));
    arr.push(&TestItem::new(3, &JsString::from("c")));

    let result = arr.try_some(&mut |val: TestItem| Ok(val.id() == 2));
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = arr.try_some(&mut |val: TestItem| Ok(val.id() == 99));
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[wasm_bindgen_test]
fn test_array_try_some_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));

    let result = arr.try_some(&mut |_val: TestItem| Err(JsValue::from_str("some error")));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_sort_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result =
        arr.try_sort_by(&mut |a: TestItem, b: TestItem| Ok((a.id() as i32) - (b.id() as i32)));
    assert!(result.is_ok());
    let sorted = result.unwrap();
    assert_eq!(sorted.get(0).id(), 1);
    assert_eq!(sorted.get(1).id(), 2);
    assert_eq!(sorted.get(2).id(), 3);
}

#[wasm_bindgen_test]
fn test_array_try_sort_by_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result =
        arr.try_sort_by(&mut |_a: TestItem, _b: TestItem| Err(JsValue::from_str("sort error")));
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_array_try_to_sorted_by() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(3, &JsString::from("c")));
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result =
        arr.try_to_sorted_by(&mut |a: TestItem, b: TestItem| Ok((b.id() as i32) - (a.id() as i32)));
    assert!(result.is_ok());
    let sorted = result.unwrap();
    assert_eq!(sorted.get(0).id(), 3);
    assert_eq!(sorted.get(1).id(), 2);
    assert_eq!(sorted.get(2).id(), 1);
    assert_eq!(arr.get(0).id(), 3); // Original array unchanged
}

#[wasm_bindgen_test]
fn test_array_try_to_sorted_by_error() {
    let arr: Array<TestItem> = Array::new_typed();
    arr.push(&TestItem::new(1, &JsString::from("a")));
    arr.push(&TestItem::new(2, &JsString::from("b")));

    let result = arr.try_to_sorted_by(&mut |_a: TestItem, _b: TestItem| {
        Err(JsValue::from_str("to_sorted error"))
    });
    assert!(result.is_err());
}

// Exported functions with standard js-sys types
#[wasm_bindgen]
pub fn rust_create_number_array() -> Array<Number> {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(1.0));
    arr.push(&Number::from(2.0));
    arr.push(&Number::from(3.0));
    arr
}

#[wasm_bindgen]
pub fn rust_create_string_array() -> Array<JsString> {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("a"));
    arr.push(&JsString::from("b"));
    arr.push(&JsString::from("c"));
    arr
}

#[wasm_bindgen]
pub fn rust_sum_number_array(arr: Array<Number>) -> f64 {
    let mut sum = 0.0;
    for i in 0..arr.length() {
        let num: Number = arr.get(i);
        sum += num.value_of();
    }
    sum
}

#[wasm_bindgen]
pub fn rust_concat_string_array(arr: Array<JsString>, separator: &str) -> String {
    let mut result = String::new();
    for i in 0..arr.length() {
        if i > 0 {
            result.push_str(separator);
        }
        let s: JsString = arr.get(i);
        result.push_str(&String::from(s));
    }
    result
}

#[wasm_bindgen_test]
fn rust_export_number_array() {
    let arr = rust_create_number_array();
    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(0).value_of(), 1.0);
    assert_eq!(arr.get(1).value_of(), 2.0);
    assert_eq!(arr.get(2).value_of(), 3.0);
}

#[wasm_bindgen_test]
fn rust_export_string_array() {
    let arr = rust_create_string_array();
    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(0), JsString::from("a"));
    assert_eq!(arr.get(1), JsString::from("b"));
    assert_eq!(arr.get(2), JsString::from("c"));
}

#[wasm_bindgen_test]
fn rust_export_process_number_array() {
    let arr: Array<Number> = Array::new_typed();
    arr.push(&Number::from(10.0));
    arr.push(&Number::from(20.0));
    arr.push(&Number::from(30.0));

    let sum = rust_sum_number_array(arr);
    assert_eq!(sum, 60.0);
}

#[wasm_bindgen_test]
fn rust_export_process_string_array() {
    let arr: Array<JsString> = Array::new_typed();
    arr.push(&JsString::from("hello"));
    arr.push(&JsString::from("world"));

    let result = rust_concat_string_array(arr, " ");
    assert_eq!(result, "hello world");
}

#[wasm_bindgen_test]
fn rust_export_round_trip_arrays() {
    let num_arr = rust_create_number_array();
    let doubled: Array<Number> = Array::new_typed();
    for i in 0..num_arr.length() {
        let num = num_arr.get(i);
        doubled.push(&Number::from(num.value_of() * 2.0));
    }
    let sum = rust_sum_number_array(doubled);
    assert_eq!(sum, 12.0); // (1+2+3) * 2 = 12
}
