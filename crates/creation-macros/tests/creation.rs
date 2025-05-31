#![cfg(target_arch = "wasm32")]

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_creation_macros::{array, json};
use wasm_bindgen_test::wasm_bindgen_test;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

const JOHN: &str = "John";
const JOHN_AGE: i32 = 30;
const IS_STUDENT: bool = true;

const JANE: &str = "Jane";
const JANE_AGE: i32 = 25;

const JIM: &str = "Jim";
const JIM_AGE: i32 = 56;

const STREET: &str = "123 Main St";
const CITY: &str = "Anytown";
const STATE: &str = "CA";
const ZIP: &str = "12345";

#[cfg(test)]
macro_rules! assert_js_eq {
    ($left:expr, $right:expr) => {
        assert_eq!(format!("{:#?}", $left), format!("{:#?}", $right));
    };
}

#[wasm_bindgen_test]
fn sanity_check() {
    assert_js_eq!(Object::new(), Object::new());
}

#[wasm_bindgen_test]
fn empty_json() {
    let obj = json! {};
    let expected = Object::new();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn simple_json() {
    let obj = json! {
        name: "John",
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_all_literals() {
    let obj = json! {
        name: "John",
        age: 30,
        favorite_float: 3.14,
        is_student: true,
        hobbies: ["reading", "traveling", "coding"],
        address: {
            street: "123 Main St",
            city: "Anytown",
            state: "CA",
            zip: "12345",
        },
        empty: null,
        empty2: undefined,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    Reflect::set(&expected, &"favorite_float".into(), &3.14.into()).unwrap();
    Reflect::set(&expected, &"is_student".into(), &IS_STUDENT.into()).unwrap();
    Reflect::set(
        &expected,
        &"hobbies".into(),
        &Array::of3(&"reading".into(), &"traveling".into(), &"coding".into()).into(),
    )
    .unwrap();

    let address = Object::new();
    Reflect::set(&address, &"street".into(), &STREET.into()).unwrap();
    Reflect::set(&address, &"city".into(), &CITY.into()).unwrap();
    Reflect::set(&address, &"state".into(), &STATE.into()).unwrap();
    Reflect::set(&address, &"zip".into(), &ZIP.into()).unwrap();
    Reflect::set(&expected, &"address".into(), &address.into()).unwrap();

    Reflect::set(&expected, &"empty".into(), &JsValue::NULL).unwrap();
    Reflect::set(&expected, &"empty2".into(), &JsValue::UNDEFINED).unwrap();

    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn level_1_nested() {
    let obj = json! {
        name: JOHN,
        address: {
            street: STREET,
            city: CITY,
            state: STATE,
            zip: ZIP
        }
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    let address = Object::new();
    Reflect::set(&address, &"street".into(), &STREET.into()).unwrap();
    Reflect::set(&address, &"city".into(), &CITY.into()).unwrap();
    Reflect::set(&address, &"state".into(), &STATE.into()).unwrap();
    Reflect::set(&address, &"zip".into(), &ZIP.into()).unwrap();
    Reflect::set(&expected, &"address".into(), &address.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_array() {
    let obj = json! {
        name: JOHN,
        friends: [
            {
                name: JANE,
                age: JANE_AGE,
            },
            {
                name: JIM,
                age: JIM_AGE,
            }
        ],
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    let array = {
        let friend1 = Object::new();
        Reflect::set(&friend1, &"name".into(), &JANE.into()).unwrap();
        Reflect::set(&friend1, &"age".into(), &JANE_AGE.into()).unwrap();

        let friend2 = Object::new();
        Reflect::set(&friend2, &"name".into(), &JIM.into()).unwrap();
        Reflect::set(&friend2, &"age".into(), &JIM_AGE.into()).unwrap();
        Array::of2(&friend1, &friend2)
    };
    Reflect::set(&expected, &"friends".into(), &array.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn complex_json() {
    let obj = json! {
        name: JOHN,
        age: JOHN_AGE,
        is_student: IS_STUDENT,
        hobbies: ["reading", "traveling", "coding"],
        address: {
            street: STREET,
            city: CITY,
            state: STATE,
            zip: ZIP,
        },
        friends: [
            {
                name: JANE,
                age: JANE_AGE,
            },
            {
                name: JIM,
                age: JIM_AGE,
            }
        ],
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    Reflect::set(&expected, &"is_student".into(), &IS_STUDENT.into()).unwrap();
    Reflect::set(
        &expected,
        &"hobbies".into(),
        &Array::of3(&"reading".into(), &"traveling".into(), &"coding".into()).into(),
    )
    .unwrap();
    let address = Object::new();
    Reflect::set(&address, &"street".into(), &STREET.into()).unwrap();
    Reflect::set(&address, &"city".into(), &CITY.into()).unwrap();
    Reflect::set(&address, &"state".into(), &STATE.into()).unwrap();
    Reflect::set(&address, &"zip".into(), &ZIP.into()).unwrap();
    Reflect::set(&expected, &"address".into(), &address.into()).unwrap();
    let friends = {
        let friend1 = Object::new();
        Reflect::set(&friend1, &"name".into(), &JANE.into()).unwrap();
        Reflect::set(&friend1, &"age".into(), &JANE_AGE.into()).unwrap();

        let friend2 = Object::new();
        Reflect::set(&friend2, &"name".into(), &JIM.into()).unwrap();
        Reflect::set(&friend2, &"age".into(), &JIM_AGE.into()).unwrap();
        Array::of2(&friend1, &friend2)
    };
    Reflect::set(&expected, &"friends".into(), &friends.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn json_with_local_var() {
    let local = Object::new();
    let local_clone = local.clone();
    Reflect::set(&local, &"property".into(), &"value".into()).unwrap();

    let local_array = Array::of2(&"local".into(), &"local2".into());
    let local_array_clone = local_array.clone();

    let obj = json! {
        name: JOHN,
        age: JOHN_AGE,
        is_student: IS_STUDENT,
        hobbies: ["reading", "traveling", "coding"],
        locals: [local],
        local_array: local_array,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    Reflect::set(&expected, &"is_student".into(), &true.into()).unwrap();
    Reflect::set(
        &expected,
        &"hobbies".into(),
        &Array::of3(&"reading".into(), &"traveling".into(), &"coding".into()).into(),
    )
    .unwrap();
    Reflect::set(
        &expected,
        &"locals".into(),
        &Array::of1(&local_clone.into()),
    )
    .unwrap();
    Reflect::set(&expected, &"local_array".into(), &local_array_clone.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn json_with_local_vars() {
    const HOBBIES: [&str; 3] = ["reading", "traveling", "coding"];
    let jane = Object::new();
    Reflect::set(&jane, &"name".into(), &JANE.into()).unwrap();
    Reflect::set(&jane, &"age".into(), &JANE_AGE.into()).unwrap();
    let friends: Array = Array::of1(&jane.clone().into());

    let friends_clone = friends.clone();
    let obj = json! {
        name: JOHN,
        age: JOHN_AGE,
        is_student: IS_STUDENT,
        hobbies: ["reading", "traveling", "coding"], // [&str; 3] does not impl Into<JsValue>
        address: {
            street: STREET,
            city: CITY,
            state: STATE,
            zip: ZIP,
        },
        friends: friends,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    Reflect::set(&expected, &"is_student".into(), &IS_STUDENT.into()).unwrap();
    Reflect::set(
        &expected,
        &"hobbies".into(),
        &Array::of3(&"reading".into(), &"traveling".into(), &"coding".into()).into(),
    )
    .unwrap();
    let address = Object::new();
    Reflect::set(&address, &"street".into(), &STREET.into()).unwrap();
    Reflect::set(&address, &"city".into(), &CITY.into()).unwrap();
    Reflect::set(&address, &"state".into(), &STATE.into()).unwrap();
    Reflect::set(&address, &"zip".into(), &ZIP.into()).unwrap();
    Reflect::set(&expected, &"address".into(), &address.into()).unwrap();
    Reflect::set(&expected, &"friends".into(), &friends_clone.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn chain_json() {
    let one = json! {
        num: 1
    };
    let one_clone = one.clone();

    let two = json! {
        num: 2
    };
    let two_clone = two.clone();

    let three = json! {
        num: 3
    };
    let three_clone = three.clone();

    let address = json! {
        street: STREET,
        city: CITY,
        state: STATE,
        zip: ZIP,
    };
    let address_clone = address.clone();

    let obj = json! {
        name: JOHN,
        age: JOHN_AGE,
        address: address,
        numbers: [one, two, three],
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    Reflect::set(&expected, &"address".into(), &address_clone.into()).unwrap();
    Reflect::set(
        &expected,
        &"numbers".into(),
        &Array::of3(&one_clone.into(), &two_clone.into(), &three_clone.into()).into(),
    )
    .unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_null_and_undefined() {
    let obj = json! {
        name: JOHN,
        age: null,
        is_student: undefined,
        friends: [null, undefined],
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JsValue::NULL).unwrap();
    Reflect::set(&expected, &"is_student".into(), &JsValue::UNDEFINED).unwrap();
    Reflect::set(
        &expected,
        &"friends".into(),
        &Array::of2(&JsValue::NULL, &JsValue::UNDEFINED).into(),
    )
    .unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_string() {
    let john = JOHN.to_string();
    let obj = json! {
        name: john,
        age: JOHN_AGE,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_string_borrow() {
    let john = &JOHN.to_string();
    let obj = json! {
        name: john,
        age: JOHN_AGE,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_str() {
    let john = "John";
    let obj = json! {
        name: john,
        age: JOHN_AGE,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_vecs() {
    // Test both Vec<JsValue> and Vec<String>
    let hobbies: Vec<JsValue> = vec!["reading".into(), "traveling".into(), "coding".into()];
    let vec2 = vec![
        "reading".to_string(),
        "traveling".to_string(),
        "coding".to_string(),
    ];
    let vec2_clone = vec2.clone();
    let hobbies_clone = hobbies.clone();
    let obj = json! {
        name: JOHN,
        age: JOHN_AGE,
        hobbies: hobbies,
        vec2: vec2,
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    Reflect::set(
        &expected,
        &"hobbies".into(),
        &Array::from_iter(&hobbies_clone),
    )
    .unwrap();
    Reflect::set(&expected, &"vec2".into(), &vec2_clone.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn with_comments() {
    let obj = json! {
        name: JOHN, // name: "John"
        age: JOHN_AGE, // age: 30
    };

    let expected = Object::new();
    Reflect::set(&expected, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&expected, &"age".into(), &JOHN_AGE.into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn obj_with_custom_js_value() {
    struct CustomJsValue(u32);
    impl Into<JsValue> for CustomJsValue {
        fn into(self) -> JsValue {
            self.0.into()
        }
    }

    let custom = CustomJsValue(42);
    let obj = json! {
        custom: custom
    };

    let expected = Object::new();
    Reflect::set(&expected, &"custom".into(), &CustomJsValue(42).into()).unwrap();
    assert_js_eq!(obj, expected);
}

#[wasm_bindgen_test]
fn simple_array() {
    let arr = array![1, 2, 3];
    let expected = Array::of3(&1.into(), &2.into(), &3.into());
    assert_js_eq!(arr, expected);
}

#[wasm_bindgen_test]
fn array_with_local_var() {
    let string1 = "string1".to_string();
    let string2 = "string2".to_string();
    let string1_clone = string1.clone();
    let string2_clone = string2.clone();

    let arr = array![string1, string2];
    let expected = Array::of2(&string1_clone.into(), &string2_clone.into());
    assert_js_eq!(arr, expected);
}

#[wasm_bindgen_test]
fn with_objects() {
    let john = Object::new();
    Reflect::set(&john, &"name".into(), &JOHN.into()).unwrap();
    Reflect::set(&john, &"age".into(), &JOHN_AGE.into()).unwrap();
    let john_clone = john.clone();

    let jane = Object::new();
    Reflect::set(&jane, &"name".into(), &JANE.into()).unwrap();
    Reflect::set(&jane, &"age".into(), &JANE_AGE.into()).unwrap();
    let jane_clone = jane.clone();

    let arr = array![john, jane];
    let expected = Array::of2(&john_clone.into(), &jane_clone.into());
    assert_js_eq!(arr, expected);
}

#[wasm_bindgen_test]
fn array_of_arrays() {
    let arr = array![[1, 2, 3], [4, 5, 6]];
    let expected = Array::of2(
        &Array::of3(&1.into(), &2.into(), &3.into()).into(),
        &Array::of3(&4.into(), &5.into(), &6.into()).into(),
    );
    assert_js_eq!(arr, expected);
}

#[wasm_bindgen_test]
fn array_with_comments() {
    let arr = array![
        1, // 1
        2, // 2
        3, // 3
    ];
    let expected = Array::of3(&1.into(), &2.into(), &3.into());
    assert_js_eq!(arr, expected);
}

#[wasm_bindgen_test]
fn array_with_custom_js_value() {
    struct CustomJsValue(u32);
    impl Into<JsValue> for CustomJsValue {
        fn into(self) -> JsValue {
            self.0.into()
        }
    }

    let custom = CustomJsValue(42);
    let array = array![custom];
    let expected = Array::of1(&CustomJsValue(42).into());
    assert_js_eq!(array, expected);
}

#[wasm_bindgen_test]
fn mix_and_match() {
    let evens = array![2, 4, 6, 8];
    let odds = array![1, 3, 6, 7];

    let rust = json! {
        language: "Rust",
        mascot: "Crab"
    };

    let go = json! {
        language: "Go",
        mascot: "Gopher"
    };

    let languages_array = array![rust, go, { language: "Python", mascot: "Snakes" } ];

    let obj = json! {
        evens: evens,
        odds: odds,
        languages: languages_array
    };

    let rust_expected = Object::new();
    Reflect::set(&rust_expected, &"language".into(), &"Rust".into()).unwrap();
    Reflect::set(&rust_expected, &"mascot".into(), &"Crab".into()).unwrap();

    let go_expected = Object::new();
    Reflect::set(&go_expected, &"language".into(), &"Go".into()).unwrap();
    Reflect::set(&go_expected, &"mascot".into(), &"Gopher".into()).unwrap();

    let python_expected = Object::new();
    Reflect::set(&python_expected, &"language".into(), &"Python".into()).unwrap();
    Reflect::set(&python_expected, &"mascot".into(), &"Snakes".into()).unwrap();

    let expected = Object::new();
    Reflect::set(
        &expected,
        &"evens".into(),
        &Array::of4(&2.into(), &4.into(), &6.into(), &8.into()).into(),
    )
    .unwrap();
    Reflect::set(
        &expected,
        &"odds".into(),
        &Array::of4(&1.into(), &3.into(), &6.into(), &7.into()).into(),
    )
    .unwrap();
    Reflect::set(
        &expected,
        &"languages".into(),
        &Array::of3(
            &rust_expected.into(),
            &go_expected.into(),
            &python_expected.into(),
        )
        .into(),
    )
    .unwrap();

    assert_js_eq!(obj, expected);
}
