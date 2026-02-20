use js_sys::Number;
use std::cell::{Cell, RefCell};
use std::panic::AssertUnwindSafe;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, sys::Undefined};
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    fn works_call(a: &dyn Fn());
    fn works_thread(a: &dyn Fn(u32) -> u32) -> u32;

    fn cannot_reuse_call(a: &dyn Fn());
    #[wasm_bindgen(catch)]
    fn cannot_reuse_call_again() -> Result<(), JsValue>;

    fn long_lived_call1(a: &Closure<dyn Fn()>);
    fn long_lived_call2(a: &Closure<dyn FnMut(u32) -> u32>) -> u32;

    fn many_arity_call1(a: &Closure<dyn Fn()>);
    fn many_arity_call2(a: &Closure<dyn Fn(u32)>);
    fn many_arity_call3(a: &Closure<dyn Fn(u32, u32)>);
    fn many_arity_call4(a: &Closure<dyn Fn(u32, u32, u32)>);
    fn many_arity_call5(a: &Closure<dyn Fn(u32, u32, u32, u32)>);
    fn many_arity_call6(a: &Closure<dyn Fn(u32, u32, u32, u32, u32)>);
    fn many_arity_call7(a: &Closure<dyn Fn(u32, u32, u32, u32, u32, u32)>);
    fn many_arity_call8(a: &Closure<dyn Fn(u32, u32, u32, u32, u32, u32, u32)>);
    fn many_arity_call9(a: &Closure<dyn Fn(u32, u32, u32, u32, u32, u32, u32, u32)>);

    #[wasm_bindgen(js_name = many_arity_call1)]
    fn many_arity_call_mut1(a: &Closure<dyn FnMut()>);
    #[wasm_bindgen(js_name = many_arity_call2)]
    fn many_arity_call_mut2(a: &Closure<dyn FnMut(u32)>);
    #[wasm_bindgen(js_name = many_arity_call3)]
    fn many_arity_call_mut3(a: &Closure<dyn FnMut(u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call4)]
    fn many_arity_call_mut4(a: &Closure<dyn FnMut(u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call5)]
    fn many_arity_call_mut5(a: &Closure<dyn FnMut(u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call6)]
    fn many_arity_call_mut6(a: &Closure<dyn FnMut(u32, u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call7)]
    fn many_arity_call_mut7(a: &Closure<dyn FnMut(u32, u32, u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call8)]
    fn many_arity_call_mut8(a: &Closure<dyn FnMut(u32, u32, u32, u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call9)]
    fn many_arity_call_mut9(a: &Closure<dyn FnMut(u32, u32, u32, u32, u32, u32, u32, u32)>);

    fn option_call1(a: Option<&Closure<dyn Fn()>>);
    fn option_call2(a: Option<&Closure<dyn FnMut(u32) -> u32>>) -> u32;
    fn option_call3(a: Option<&Closure<dyn Fn()>>) -> bool;

    #[wasm_bindgen(js_name = many_arity_call1)]
    fn many_arity_stack1(a: &dyn Fn());
    #[wasm_bindgen(js_name = many_arity_call2)]
    fn many_arity_stack2(a: &dyn Fn(u32));
    #[wasm_bindgen(js_name = many_arity_call3)]
    fn many_arity_stack3(a: &dyn Fn(u32, u32));
    #[wasm_bindgen(js_name = many_arity_call4)]
    fn many_arity_stack4(a: &dyn Fn(u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call5)]
    fn many_arity_stack5(a: &dyn Fn(u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call6)]
    fn many_arity_stack6(a: &dyn Fn(u32, u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call7)]
    fn many_arity_stack7(a: &dyn Fn(u32, u32, u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call8)]
    fn many_arity_stack8(a: &dyn Fn(u32, u32, u32, u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call9)]
    fn many_arity_stack9(a: &dyn Fn(u32, u32, u32, u32, u32, u32, u32, u32));

    fn long_lived_dropping_cache(a: &Closure<dyn Fn()>);
    #[wasm_bindgen(catch)]
    fn long_lived_dropping_call() -> Result<(), JsValue>;

    fn long_lived_option_dropping_cache(a: Option<&Closure<dyn Fn()>>) -> bool;
    #[wasm_bindgen(catch)]
    fn long_lived_option_dropping_call() -> Result<(), JsValue>;

    fn long_fnmut_recursive_cache(a: &Closure<dyn FnMut()>);
    #[wasm_bindgen(catch)]
    fn long_fnmut_recursive_call() -> Result<(), JsValue>;

    fn fnmut_call(a: &mut dyn FnMut());
    fn fnmut_thread(a: &mut dyn FnMut(u32) -> u32) -> u32;

    fn fnmut_bad_call(a: &mut dyn FnMut());
    #[wasm_bindgen(catch)]
    fn fnmut_bad_again(a: bool) -> Result<(), JsValue>;

    fn string_arguments_call(a: &mut dyn FnMut(String));

    fn string_ret_call(a: &mut dyn FnMut(String) -> String);

    fn drop_during_call_save(a: &Closure<dyn Fn()>);
    fn drop_during_call_call();

    fn js_test_closure_returner();

    fn calling_it_throws(a: &Closure<dyn FnMut()>) -> bool;

    fn call_val(f: &JsValue);

    #[wasm_bindgen(js_name = calling_it_throws)]
    fn call_val_throws(f: &JsValue) -> bool;

    fn pass_reference_first_arg_twice(
        a: RefFirstArgument,
        b: &Closure<dyn FnMut(&RefFirstArgument)>,
        c: &Closure<dyn FnMut(&RefFirstArgument)>,
    );
    #[wasm_bindgen(js_name = pass_reference_first_arg_twice)]
    fn pass_reference_first_arg_twice2(
        a: RefFirstArgument,
        b: &mut dyn FnMut(&RefFirstArgument),
        c: &mut dyn FnMut(&RefFirstArgument),
    );
    fn call_destroyed(a: &JsValue);

    fn js_store_forgotten_closure(closure: &Closure<dyn Fn()>);
    fn js_call_forgotten_closure();

    #[wasm_bindgen(js_name = many_arity_call2)]
    fn externref_call(a: &Closure<dyn Fn(JsValue)>);
    #[wasm_bindgen(js_name = many_arity_call2)]
    fn named_externref_call(a: &Closure<dyn Fn(Number)>);
}

#[wasm_bindgen_test]
fn works() {
    let a = Cell::new(false);
    works_call(&|| a.set(true));
    assert!(a.get());

    assert_eq!(works_thread(&|a| a + 1), 3);
}

#[wasm_bindgen_test]
fn cannot_reuse() {
    cannot_reuse_call(&|| {});
    assert!(cannot_reuse_call_again().is_err());
}

#[wasm_bindgen_test]
fn debug() {
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {}));
    assert_eq!(&format!("{:?}", closure), "Closure { ... }");
}

#[wasm_bindgen_test]
fn long_lived() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::own_aborting(move || hit2.set(true));
    assert!(!hit.get());
    long_lived_call1(&a);
    assert!(hit.get());

    let hit = Rc::new(Cell::new(false));
    {
        let hit = hit.clone();
        let a = Closure::own_aborting(move |x| {
            hit.set(true);
            x + 3
        });
        assert_eq!(long_lived_call2(&a), 5);
    }
    assert!(hit.get());
}

#[wasm_bindgen_test]
fn many_arity() {
    many_arity_call1(&Closure::new(|| {}));
    many_arity_call2(&ScopedClosure::new(|a| assert_eq!(a, 1)));
    many_arity_call3(&Closure::new(|a, b| assert_eq!((a, b), (1, 2))));
    many_arity_call4(&Closure::new(|a, b, c| assert_eq!((a, b, c), (1, 2, 3))));
    many_arity_call5(&Closure::new(|a, b, c, d| {
        assert_eq!((a, b, c, d), (1, 2, 3, 4))
    }));
    many_arity_call6(&Closure::new(|a, b, c, d, e| {
        assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5))
    }));
    many_arity_call7(&Closure::new(|a, b, c, d, e, f| {
        assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6))
    }));
    many_arity_call8(&Closure::new(|a, b, c, d, e, f, g| {
        assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7))
    }));
    many_arity_call9(&Closure::new(|a, b, c, d, e, f, g, h| {
        assert_eq!((a, b, c, d, e, f, g, h), (1, 2, 3, 4, 5, 6, 7, 8))
    }));

    let s = String::new();
    many_arity_call_mut1(&Closure::once(move || drop(s)));
    let s = String::new();
    many_arity_call_mut2(&Closure::once(move |a| {
        drop(s);
        assert_eq!(a, 1);
    }));
    let s = String::new();
    many_arity_call_mut3(&Closure::once(move |a, b| {
        drop(s);
        assert_eq!((a, b), (1, 2));
    }));
    let s = String::new();
    many_arity_call_mut4(&Closure::once(move |a, b, c| {
        drop(s);
        assert_eq!((a, b, c), (1, 2, 3));
    }));
    let s = String::new();
    many_arity_call_mut5(&Closure::once(move |a, b, c, d| {
        drop(s);
        assert_eq!((a, b, c, d), (1, 2, 3, 4));
    }));
    let s = String::new();
    many_arity_call_mut6(&Closure::once(move |a, b, c, d, e| {
        drop(s);
        assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5));
    }));
    let s = String::new();
    many_arity_call_mut7(&Closure::once(move |a, b, c, d, e, f| {
        drop(s);
        assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6));
    }));
    let s = String::new();
    many_arity_call_mut8(&Closure::once(move |a, b, c, d, e, f, g| {
        drop(s);
        assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7));
    }));
    let s = String::new();
    many_arity_call_mut9(&Closure::once(move |a, b, c, d, e, f, g, h| {
        drop(s);
        assert_eq!((a, b, c, d, e, f, g, h), (1, 2, 3, 4, 5, 6, 7, 8));
    }));

    many_arity_stack1(&(|| {}));
    many_arity_stack2(&(|a| assert_eq!(a, 1)));
    many_arity_stack3(&(|a, b| assert_eq!((a, b), (1, 2))));
    many_arity_stack4(&(|a, b, c| assert_eq!((a, b, c), (1, 2, 3))));
    many_arity_stack5(&(|a, b, c, d| assert_eq!((a, b, c, d), (1, 2, 3, 4))));
    many_arity_stack6(&(|a, b, c, d, e| assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5))));
    many_arity_stack7(&(|a, b, c, d, e, f| assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6))));
    many_arity_stack8(
        &(|a, b, c, d, e, f, g| assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7))),
    );
    many_arity_stack9(
        &(|a, b, c, d, e, f, g, h| assert_eq!((a, b, c, d, e, f, g, h), (1, 2, 3, 4, 5, 6, 7, 8))),
    );
}

#[wasm_bindgen_test]
fn option() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::own_aborting(move || hit2.set(true));
    assert!(!hit.get());
    option_call1(Some(&a));
    assert!(hit.get());

    let hit = Rc::new(Cell::new(false));
    {
        let hit = hit.clone();
        let a = Closure::own_aborting(move |x| {
            hit.set(true);
            x + 3
        });
        assert_eq!(option_call2(Some(&a)), 5);
    }
    assert!(hit.get());

    assert!(option_call3(None));
}

struct Dropper(Rc<Cell<bool>>);
impl Drop for Dropper {
    fn drop(&mut self) {
        assert!(!self.0.get());
        self.0.set(true);
    }
}

#[wasm_bindgen_test]
fn call_fn_once_twice() {
    let dropped = Rc::new(Cell::new(false));
    let dropper = Dropper(dropped.clone());
    let called = Rc::new(Cell::new(false));

    let c = Closure::once_aborting({
        let called = called.clone();
        move || {
            assert!(!called.get());
            called.set(true);
            drop(dropper);
        }
    });

    many_arity_call_mut1(&c);
    assert!(called.get());
    assert!(dropped.get());

    assert!(calling_it_throws(&c));
}

#[wasm_bindgen_test]
fn once_into_js() {
    use std::panic::AssertUnwindSafe;

    let dropped = Rc::new(Cell::new(false));
    let dropper = Dropper(dropped.clone());
    let called = Rc::new(Cell::new(false));

    let f = Closure::once_into_js(AssertUnwindSafe({
        let called = called.clone();
        move || {
            assert!(!called.get());
            called.set(true);
            drop(dropper);
        }
    }));

    call_val(&f);
    assert!(called.get());
    assert!(dropped.get());

    assert!(call_val_throws(&f));
}

/// Reproduce: Closure::once(AssertUnwindSafe(Box<dyn FnOnce(T) -> R>)) should compile.
#[wasm_bindgen_test]
fn once_with_boxed_trait_object() {
    let boxed: Box<dyn FnOnce(u32) -> u32> = Box::new(|x| x * 2);
    let c: Closure<dyn FnMut(u32) -> u32> = Closure::once_assert_unwind_safe(boxed);
    let result = long_lived_call2(&c);
    assert_eq!(result, 4);
}

#[wasm_bindgen_test]
fn long_lived_dropping() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::own_aborting(move || hit2.set(true));
    long_lived_dropping_cache(&a);
    assert!(!hit.get());
    assert!(long_lived_dropping_call().is_ok());
    assert!(hit.get());
    drop(a);
    assert!(long_lived_dropping_call().is_err());
}

#[wasm_bindgen_test]
fn long_lived_option_dropping() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();

    let a = Closure::own_aborting(move || hit2.set(true));

    assert!(!long_lived_option_dropping_cache(None));
    assert!(long_lived_option_dropping_cache(Some(&a)));

    assert!(!hit.get());
    assert!(long_lived_option_dropping_call().is_ok());
    assert!(hit.get());

    drop(a);
    assert!(long_lived_option_dropping_call().is_err());
}

#[wasm_bindgen_test]
fn long_fnmut_recursive() {
    let a = Closure::new(|| {
        assert!(long_fnmut_recursive_call().is_err());
    });
    long_fnmut_recursive_cache(&a);
    assert!(long_fnmut_recursive_call().is_ok());
}

#[wasm_bindgen_test]
fn fnmut() {
    let mut a = false;
    fnmut_call(&mut || a = true);
    assert!(a);

    let mut x = false;
    assert_eq!(
        fnmut_thread(&mut |a| {
            x = true;
            a + 1
        }),
        3
    );
    assert!(x);
}

#[wasm_bindgen_test]
fn fnmut_bad() {
    let mut x = true;
    let mut hits = 0;
    fnmut_bad_call(&mut || {
        hits += 1;
        if fnmut_bad_again(hits == 1).is_err() {
            return;
        }
        x = false;
    });
    assert_eq!(hits, 1);
    assert!(x);

    assert!(fnmut_bad_again(true).is_err());
}

#[wasm_bindgen_test]
fn string_arguments() {
    let mut x = false;
    string_arguments_call(&mut |s| {
        assert_eq!(s, "foo");
        x = true;
    });
    assert!(x);
}

#[wasm_bindgen_test]
fn string_ret() {
    let mut x = false;
    string_ret_call(&mut |mut s| {
        assert_eq!(s, "foo");
        s.push_str("bar");
        x = true;
        s
    });
    assert!(x);
}

#[wasm_bindgen_test]
fn drop_drops() {
    static mut HIT: bool = false;

    struct A;

    impl Drop for A {
        fn drop(&mut self) {
            unsafe {
                HIT = true;
            }
        }
    }
    let a = A;
    let x: Closure<dyn Fn()> = Closure::new(move || {
        let _ = &a;
    });
    drop(x);
    unsafe {
        assert!(HIT);
    }
}

#[wasm_bindgen_test]
fn drop_during_call_ok() {
    static mut HIT: bool = false;
    struct A;
    impl Drop for A {
        fn drop(&mut self) {
            unsafe {
                HIT = true;
            }
        }
    }

    let rc = Rc::new(RefCell::new(None));
    let rc2 = rc.clone();
    let x = 3;
    let a = A;
    let x: Closure<dyn Fn()> = Closure::own_aborting(move || {
        // "drop ourselves"
        drop(rc2.borrow_mut().take().unwrap());

        // `A` should not have been destroyed as a result
        unsafe {
            assert!(!HIT);
        }

        // allocate some heap memory to try to paper over our `3`
        drop(String::from("1234567890"));

        // make sure our closure memory is still valid
        assert_eq!(x, 3);

        // make sure `A` is bound to our closure environment.
        let _a = &a;
        unsafe {
            assert!(!HIT);
        }
    });
    drop_during_call_save(&x);
    *rc.borrow_mut() = Some(x);
    drop(rc);
    unsafe {
        assert!(!HIT);
    }
    drop_during_call_call();
    unsafe {
        assert!(HIT);
    }
}

#[wasm_bindgen_test]
fn test_closure_returner() {
    type ClosureType = dyn FnMut() -> BadStruct;

    use js_sys::{Object, Reflect};

    js_test_closure_returner();

    #[wasm_bindgen]
    pub struct ClosureHandle {
        _closure: Closure<ClosureType>,
    }

    #[wasm_bindgen]
    pub struct BadStruct {}

    #[wasm_bindgen]
    pub fn closure_returner() -> Result<Object, JsValue> {
        let o = Object::new();

        let some_fn = Closure::<ClosureType>::wrap(Box::new(move || BadStruct {}));
        Reflect::set(
            &o,
            &JsValue::from("someKey"),
            some_fn.as_ref().unchecked_ref(),
        )
        .unwrap();
        Reflect::set(
            &o,
            &JsValue::from("handle"),
            &JsValue::from(ClosureHandle { _closure: some_fn }),
        )
        .unwrap();

        Ok(o)
    }
}

#[wasm_bindgen]
pub struct RefFirstArgument {
    contents: u32,
}

#[wasm_bindgen_test]
fn reference_as_first_argument_builds_at_all() {
    #[wasm_bindgen]
    extern "C" {
        fn ref_first_arg1(a: &dyn Fn(&JsValue));
        fn ref_first_arg2(a: &mut dyn FnMut(&JsValue));
        fn ref_first_arg3(a: &Closure<dyn Fn(&JsValue)>);
        fn ref_first_arg4(a: &Closure<dyn FnMut(&JsValue)>);
        fn ref_first_custom1(a: &dyn Fn(&RefFirstArgument));
        fn ref_first_custom2(a: &mut dyn FnMut(&RefFirstArgument));
        fn ref_first_custom3(a: &Closure<dyn Fn(&RefFirstArgument)>);
        fn ref_first_custom4(a: &Closure<dyn FnMut(&RefFirstArgument)>);
    }

    Closure::<dyn Fn(&JsValue)>::wrap(Box::new(|_: &JsValue| ()));
    Closure::<dyn FnMut(&JsValue)>::wrap(Box::new(|_: &JsValue| ()));
    Closure::once(|_: &JsValue| ());
    Closure::once_into_js(|_: &JsValue| ());
    Closure::<dyn Fn(&RefFirstArgument)>::wrap(Box::new(|_: &RefFirstArgument| ()));
    Closure::<dyn FnMut(&RefFirstArgument)>::wrap(Box::new(|_: &RefFirstArgument| ()));
    Closure::once(|_: &RefFirstArgument| ());
    Closure::once_into_js(|_: &RefFirstArgument| ());
}

#[wasm_bindgen_test]
fn reference_as_first_argument_works() {
    let a = Rc::new(Cell::new(0));
    let b = {
        let a = a.clone();
        Closure::once_aborting(move |x: &RefFirstArgument| {
            assert_eq!(a.get(), 0);
            assert_eq!(x.contents, 3);
            a.set(a.get() + 1);
        })
    };
    let c = {
        let a = a.clone();
        Closure::once_aborting(move |x: &RefFirstArgument| {
            assert_eq!(a.get(), 1);
            assert_eq!(x.contents, 3);
            a.set(a.get() + 1);
        })
    };
    pass_reference_first_arg_twice(RefFirstArgument { contents: 3 }, &b, &c);
    assert_eq!(a.get(), 2);
}

#[wasm_bindgen_test]
fn reference_as_first_argument_works2() {
    let a = Cell::new(0);
    pass_reference_first_arg_twice2(
        RefFirstArgument { contents: 3 },
        &mut |x: &RefFirstArgument| {
            assert_eq!(a.get(), 0);
            assert_eq!(x.contents, 3);
            a.set(a.get() + 1);
        },
        &mut |x: &RefFirstArgument| {
            assert_eq!(a.get(), 1);
            assert_eq!(x.contents, 3);
            a.set(a.get() + 1);
        },
    );
    assert_eq!(a.get(), 2);
}

#[wasm_bindgen_test]
fn call_destroyed_doesnt_segfault() {
    struct A(i32, i32);
    impl Drop for A {
        fn drop(&mut self) {
            assert_eq!(self.0, self.1);
        }
    }

    let a = A(1, 1);
    let a = Closure::<dyn Fn()>::wrap(Box::new(move || {
        let _ = a;
    }));
    let b = a.as_ref().clone();
    drop(a);
    call_destroyed(&b);

    let a = A(2, 2);
    let a = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        let _ = a;
    }));
    let b = a.as_ref().clone();
    drop(a);
    call_destroyed(&b);

    let a = A(1, 1);
    let a = Closure::<dyn Fn(&JsValue)>::wrap(Box::new(move |_: &JsValue| {
        let _ = a;
    }));
    let b = a.as_ref().clone();
    drop(a);
    call_destroyed(&b);

    let a = A(2, 2);
    let a = Closure::<dyn FnMut(&JsValue)>::wrap(Box::new(move |_: &JsValue| {
        let _ = a;
    }));
    let b = a.as_ref().clone();
    drop(a);
    call_destroyed(&b);
}

#[wasm_bindgen_test]
fn forget_works() {
    let a = Closure::<dyn Fn()>::wrap(Box::new(|| {}));
    js_store_forgotten_closure(&a);
    a.forget();
    js_call_forgotten_closure();
}

#[wasm_bindgen_test]
fn named_externref_no_duplicate_adapter() {
    externref_call(&Closure::new(|a| assert_eq!(a, 1)));
    named_externref_call(&Closure::new(|a| assert_eq!(a, 1)));
}

#[wasm_bindgen_test]
fn closure_does_not_leak() {
    let initial = wasm_bindgen::externref_heap_live_count();
    let dropped = Rc::new(Cell::new(false));
    let mut dropper = Dropper(dropped.clone());
    drop(Closure::own_aborting(move || {
        // just ensure that `dropper` is moved into the closure environment
        // (we can't use it by value because it's not a FnOnce closure)
        let _ = &mut dropper;
    }));
    assert_eq!(
        wasm_bindgen::externref_heap_live_count(),
        initial,
        "JS closure not dropped"
    );
    assert!(dropped.get(), "Rust closure not dropped");
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    #[wasm_bindgen(js_name = many_arity_call1)]
    fn abort_closure_call1(a: &Closure<dyn Fn()>);
    #[wasm_bindgen(js_name = many_arity_call2)]
    fn abort_closure_call2(a: &Closure<dyn Fn(u32)>);
    #[wasm_bindgen(js_name = long_lived_call2)]
    fn abort_closure_call_mut(a: &Closure<dyn FnMut(u32) -> u32>) -> u32;
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_basic() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::own_aborting(move || hit2.set(true));
    assert!(!hit.get());
    abort_closure_call1(&a);
    assert!(hit.get());
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_with_non_unwind_safe() {
    use std::cell::RefCell;

    // RefCell is not UnwindSafe, but Closure::own_aborting doesn't require it
    let rc = Rc::new(RefCell::new(0));
    let rc2 = rc.clone();
    let a = Closure::own_aborting(move || {
        *rc2.borrow_mut() += 1;
    });
    abort_closure_call1(&a);
    assert_eq!(*rc.borrow(), 1);
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_debug() {
    let closure = Closure::<dyn FnMut()>::wrap(Box::new(|| {}));
    assert_eq!(&format!("{:?}", closure), "Closure { ... }");
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_once() {
    let s = String::from("test");
    let closure = Closure::once(move || {
        drop(s);
    });
    many_arity_call_mut1(&closure);
    // Calling again should throw, but we can't easily test that from Rust
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_fnmut() {
    let hit = Rc::new(Cell::new(0));
    let hit2 = hit.clone();
    let a = Closure::own_aborting(move |x| {
        hit2.set(hit2.get() + 1);
        x + 3
    });
    assert_eq!(abort_closure_call_mut(&a), 5);
    assert_eq!(hit.get(), 1);
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_forget() {
    let a = Closure::<dyn Fn()>::wrap(Box::new(|| {}));
    js_store_forgotten_closure(&a);
    a.forget();
    js_call_forgotten_closure();
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn closure_unwind_safe_catches_panic() {
    // Closure::new should catch panics when invoked from JS
    let a = Closure::new(|| {
        panic!("test panic");
    });
    // JS should catch this as a PanicError
    assert!(calling_it_throws(&a));
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn closure_with_assert_unwind_safe() {
    use std::panic::AssertUnwindSafe;

    // Rc<Cell> is not UnwindSafe, but we can wrap with AssertUnwindSafe
    let rc = Rc::new(Cell::new(0));
    let rc2 = AssertUnwindSafe(rc.clone());
    let a = Closure::new(move || {
        rc2.set(rc2.get() + 1);
    });
    many_arity_call1(&a);
    assert_eq!(rc.get(), 1);
}

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    fn closure_with_call(f: &ScopedClosure<dyn FnMut()>);
    #[wasm_bindgen(js_name = closure_with_call)]
    fn closure_with_call_immutable(f: &ScopedClosure<dyn Fn()>);
    fn closure_with_cache(f: &ScopedClosure<dyn FnMut()>);
    #[wasm_bindgen(catch)]
    fn closure_with_call_cached() -> Result<(), JsValue>;
    fn closure_with_call_and_cache<'a>(f: &ScopedClosure<'a, dyn FnMut(u32) + 'a>);
    fn closure_with_call_cached_throws() -> bool;
}

/// Test that ScopedClosure::borrow works correctly during the callback body
#[wasm_bindgen_test]
fn closure_with_works_during_body() {
    let called = Cell::new(false);
    {
        let mut func = AssertUnwindSafe(|| {
            called.set(true);
        });
        let closure = ScopedClosure::borrow_mut(&mut func);
        closure_with_call(&closure);
    }
    assert!(called.get());
}

/// Test that ScopedClosure::borrow_immutable allows capturing non-'static references
/// with proper unwind safety using AssertUnwindSafe + Cell
#[wasm_bindgen_test]
fn closure_with_captures_non_static() {
    let value = AssertUnwindSafe(Cell::new(0u32));
    {
        let mut func = || {
            value.set(value.get() + 1);
        };
        {
            let closure = ScopedClosure::borrow(&func);
            closure_with_call_immutable(&closure);
            closure_with_call_immutable(&closure);
        }
        let closure = ScopedClosure::borrow_mut(&mut func);
        closure_with_call(&closure);
    }
    assert_eq!(value.get(), 3);
}

/// Test that using a ScopedClosure closure after the borrow ends throws an error
#[wasm_bindgen_test]
fn closure_with_use_after_free_throws() {
    // Cache the closure's JS function during the borrowed scope
    {
        let mut func = || {
            // This closure body doesn't matter - we just want to cache the JS function
        };
        let closure = ScopedClosure::borrow_mut(&mut func);
        closure_with_cache(&closure);
    }

    // After the borrow ends, the closure has been invalidated.
    // Calling it should throw an error.
    let result = closure_with_call_cached();
    let _ = result.expect_err("calling closure after ScopedClosure should throw");
}

/// Test that a ScopedClosure closure throws when JS retains and calls it after invalidation
#[wasm_bindgen_test]
fn closure_with_cached_throws_after_drop() {
    let mut sum = 0u32;
    {
        // Test inference: value type should be inferred from closure_with_call_and_cache signature
        let mut func = AssertUnwindSafe(|value| {
            sum += value;
        });
        let closure = ScopedClosure::borrow_mut(&mut func);
        // JS will cache the closure AND call it 3 times during this callback
        closure_with_call_and_cache(&closure);
    }
    // Closure worked during the callback
    assert_eq!(sum, 6); // 1 + 2 + 3

    // Now the closure has been invalidated. JS tries to call the cached reference
    // and should get an exception.
    assert!(
        closure_with_call_cached_throws(),
        "calling cached ScopedClosure closure after drop should throw"
    );
}

/// Test that ScopedClosure can be used where &Closure is expected (same type)
#[wasm_bindgen_test]
fn scoped_closure_is_closure() {
    #[wasm_bindgen(module = "tests/wasm/closures.js")]
    extern "C" {
        // This function takes &Closure (which is ScopedClosure<'static, T>)
        fn closure_with_call_closure(f: &Closure<dyn FnMut()>);
    }

    let called = Cell::new(false);
    // Create a 'static closure using Closure::new
    let closure = Closure::new(|| {
        // Note: Can't capture `called` by reference here since Closure::new requires 'static
    });
    closure_with_call_closure(&closure);

    // For non-'static captures, use ScopedClosure::borrow
    {
        let mut func = || {
            called.set(true);
        };
        let scoped = ScopedClosure::borrow_mut_aborting(&mut func);
        closure_with_call(&scoped);
    }
    assert!(called.get());
}

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    // Takes ownership of the closure (passed by value)
    fn closure_take_ownership(cb: Closure<dyn FnMut()>);
    fn closure_take_ownership_with_arg(cb: Closure<dyn FnMut(u32)>, value: u32);
    #[wasm_bindgen(catch)]
    fn closure_call_stored() -> Result<(), JsValue>;
}

/// Test that Closure can be passed by value, transferring ownership to JS
#[wasm_bindgen_test]
fn closure_pass_by_value() {
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    // Create a closure and pass it by value to JS
    let closure = Closure::own_assert_unwind_safe(move || {
        called_clone.set(true);
    });

    // Pass ownership to JS - closure is consumed here
    closure_take_ownership(closure);

    // The closure should have been called
    assert!(called.get());
}

/// Test that Closure passed by value with arguments works
#[wasm_bindgen_test]
fn closure_pass_by_value_with_arg() {
    use std::rc::Rc;

    let sum = Rc::new(Cell::new(0u32));
    let sum_clone = sum.clone();

    // Test inference: value type should be inferred from closure_take_ownership_with_arg signature
    let closure = Closure::own_assert_unwind_safe(move |value| {
        sum_clone.set(sum_clone.get() + value);
    });

    closure_take_ownership_with_arg(closure, 42);

    assert_eq!(sum.get(), 42);
}

/// Test that JS can store a closure passed by value and call it later
#[wasm_bindgen_test]
fn closure_pass_by_value_stored() {
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    // Pass closure by value - JS will store it
    let closure = Closure::own_assert_unwind_safe(move || {
        called_clone.set(true);
    });
    closure_take_ownership(closure);

    // First call should succeed (closure was stored and called)
    assert!(called.get());

    // JS can call the stored closure again
    let result = closure_call_stored();
    assert!(result.is_ok(), "calling stored closure should work");
}

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    fn closure_fn_with_call<'a>(f: &'a ScopedClosure<'a, dyn Fn() + 'a>);
    fn closure_fn_with_call_arg(f: &ScopedClosure<dyn Fn(u32)>, value: u32);
}

/// Test that ScopedClosure::borrow works for Fn closures
#[wasm_bindgen_test]
fn scoped_closure_borrow_fn() {
    let called = Cell::new(false);
    {
        let func = || {
            called.set(true);
        };
        let closure = ScopedClosure::borrow_aborting(&func);
        closure_fn_with_call(&closure);
    }
    assert!(called.get());
}

/// Test that ScopedClosure::borrow can capture non-'static references (Fn)
#[wasm_bindgen_test]
fn scoped_closure_borrow_fn_captures_non_static() {
    let data = vec![1, 2, 3, 4, 5];
    let sum = Cell::new(0u32);
    {
        let func = || {
            // Read-only access to captured data
            sum.set(data.iter().sum());
        };
        let closure = ScopedClosure::borrow_aborting(&func);
        closure_fn_with_call(&closure);
    }
    assert_eq!(sum.get(), 15);
    // data is still accessible after closure is dropped
    assert_eq!(data.len(), 5);
}

/// Test that ScopedClosure::borrow works with arguments
#[wasm_bindgen_test]
fn scoped_closure_borrow_fn_with_arg() {
    let received = Cell::new(0u32);
    {
        // Test inference: value type should be inferred from closure_fn_with_call_arg signature
        let func = |value| {
            received.set(value);
        };
        let closure = ScopedClosure::borrow_aborting(&func);
        closure_fn_with_call_arg(&closure, 42);
    }
    assert_eq!(received.get(), 42);
}

/// Test that ScopedClosure::own works the same as Closure::new
#[wasm_bindgen_test]
fn scoped_closure_own() {
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    // Use ScopedClosure::own instead of Closure::new
    let closure = ScopedClosure::own_aborting(move || {
        called_clone.set(true);
    });

    closure_take_ownership(closure);
    assert!(called.get());
}

/// Test that ScopedClosure::borrow_aborting works with FnMut
#[wasm_bindgen_test]
fn scoped_closure_borrow_aborting_fnmut() {
    use std::rc::Rc;

    // Rc<Cell<T>> is not UnwindSafe, so we need _aborting variant
    let counter = Rc::new(Cell::new(0u32));
    {
        let mut func = || {
            counter.set(counter.get() + 1);
        };
        let closure = ScopedClosure::borrow_mut_aborting(&mut func);
        closure_with_call(&closure);
        closure_with_call(&closure);
    }
    assert_eq!(counter.get(), 2);
}

/// Test that ScopedClosure::borrow_immutable_aborting works with Fn
#[wasm_bindgen_test]
fn scoped_closure_borrow_immutable_aborting() {
    use std::rc::Rc;

    // Rc<Cell<T>> is not UnwindSafe, so we need _aborting variant
    let counter = Rc::new(Cell::new(0u32));
    {
        let func = || {
            counter.set(counter.get() + 1);
            Undefined::UNDEFINED
        };
        let closure = ScopedClosure::borrow_aborting(&func);
        closure_fn_with_call(closure.upcast());
        closure_fn_with_call(closure.upcast());
    }
    assert_eq!(counter.get(), 2);
}

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    fn immediate_closure_call<'a>(f: ImmediateClosure<'a, dyn FnMut() + 'a>);
    fn immediate_closure_call_arg<'a>(f: ImmediateClosure<'a, dyn FnMut(u32) + 'a>, value: u32);
    fn immediate_closure_call_ret<'a>(
        f: ImmediateClosure<'a, dyn FnMut(u32) -> u32>,
        value: u32,
    ) -> u32;
    fn immediate_closure_fn_call<'a>(f: ImmediateClosure<'a, dyn Fn() + 'a>);
    fn immediate_closure_catches_panic<'a>(f: ImmediateClosure<'a, dyn FnMut() + 'a>) -> bool;
    fn immediate_closure_fnmut_reentrant<'a>(f: ImmediateClosure<'a, dyn FnMut() + 'a>);
    #[wasm_bindgen(catch)]
    fn immediate_closure_fnmut_reentrant_invoke() -> Result<(), JsValue>;
    fn immediate_closure_fn_reentrant<'a>(f: ImmediateClosure<'a, dyn Fn() + 'a>);
    #[wasm_bindgen(catch)]
    fn immediate_closure_fn_reentrant_invoke() -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
fn immediate_closure_basic() {
    let mut called = false;
    // Use wrap_mut_aborting for closures capturing &mut (not UnwindSafe)
    immediate_closure_call(ImmediateClosure::new_mut_aborting(&mut || {
        called = true;
    }));
    assert!(called);
}

#[wasm_bindgen_test]
fn immediate_closure_new_with_assert_unwind_safe() {
    let mut called = false;
    // Use new_mut with AssertUnwindSafe for closures capturing &mut
    // This enables panic catching while asserting unwind safety
    let mut closure = AssertUnwindSafe(|| {
        called = true;
    });
    immediate_closure_call(ImmediateClosure::new_mut(&mut closure));
    assert!(called);
}

#[wasm_bindgen_test]
fn immediate_closure_with_return() {
    // Test inference: x and return type should be inferred from the function signature
    // Using wrap_mut_aborting since it enables inference and this closure is simple
    let result = immediate_closure_call_ret(ImmediateClosure::new_mut_aborting(&mut |x| x * 2), 21);
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
fn immediate_closure_immutable() {
    let data = vec![1, 2, 3];
    // Use wrap_aborting for Fn closures without UnwindSafe requirement
    immediate_closure_fn_call(ImmediateClosure::new_aborting(&|| {
        assert_eq!(data.len(), 3);
    }));
    // data is still accessible after
    assert_eq!(data.len(), 3);
}

#[wasm_bindgen_test]
fn immediate_closure_fn_to_fnmut_upcast() {
    let sum = Cell::new(0u32);

    // Create an immutable Fn closure that takes no args
    // Use wrap_assert_unwind_safe for Fn closures capturing Cell (not RefUnwindSafe)
    let func: &dyn Fn() = &|| {
        sum.set(sum.get() + 1);
    };
    let closure = ImmediateClosure::new_assert_unwind_safe(func);
    // Upcast dyn Fn() -> dyn FnMut() and pass to function expecting FnMut
    immediate_closure_call(closure.as_mut());
    assert_eq!(sum.get(), 1);

    // Upcast with args: dyn Fn(u32) -> dyn FnMut(u32)
    // Note: ImmediateClosure can only upcast Fn->FnMut with same arg types
    // (unlike ScopedClosure which wraps a JsValue and can do full variance)
    let func_with_arg: &dyn Fn(u32) = &|x: u32| {
        sum.set(sum.get() + x);
    };
    let closure_with_arg = ImmediateClosure::new_assert_unwind_safe(func_with_arg);
    immediate_closure_call_arg(closure_with_arg.as_mut(), 41);
    assert_eq!(sum.get(), 42);
}

#[wasm_bindgen_test]
fn immediate_closure_debug() {
    // Type annotation needed when no context provides the expected dyn type
    let mut f = || {};
    // Use wrap_mut_aborting since we're using type annotation
    let closure: ImmediateClosure<dyn FnMut()> = ImmediateClosure::new_mut_aborting(&mut f);
    assert_eq!(&format!("{:?}", closure), "ImmediateClosure { .. }");
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn immediate_closure_catches_panic_test() {
    // Use new_mut with a closure that doesn't capture &mut to test panic catching
    // The closure || panic!() is UnwindSafe since it captures nothing
    let mut closure = || {
        panic!("test panic");
    };
    let caught = immediate_closure_catches_panic(ImmediateClosure::new_mut(&mut closure));
    assert!(
        caught,
        "panic should be caught and converted to JS exception"
    );
}

/// Test that FnMut ImmediateClosure has a reentrancy guard.
/// JS caches the closure and tries to call it from within itself — the
/// reentrant call should throw.
#[wasm_bindgen_test]
fn immediate_closure_fnmut_reentrancy_guard() {
    let mut call_count = 0u32;
    let mut func = || {
        call_count += 1;
        if call_count == 1 {
            // First call: try to invoke ourselves reentrantly via JS
            let result = immediate_closure_fnmut_reentrant_invoke();
            // The reentrant call should fail (JS exception from the guard)
            assert!(result.is_err(), "reentrant FnMut call should be rejected");
        }
    };
    immediate_closure_fnmut_reentrant(ImmediateClosure::new_mut_aborting(&mut func));
    // Only the outer call should have succeeded
    assert_eq!(call_count, 1);
}

/// Test that Fn (immutable) ImmediateClosure CAN be called reentrantly.
/// Unlike FnMut, Fn closures are safe to call concurrently, so no guard.
#[wasm_bindgen_test]
fn immediate_closure_fn_reentrancy_allowed() {
    let call_count = Cell::new(0u32);
    let func = || {
        call_count.set(call_count.get() + 1);
        if call_count.get() == 1 {
            // First call: invoke ourselves reentrantly via JS — should succeed
            let result = immediate_closure_fn_reentrant_invoke();
            assert!(result.is_ok(), "reentrant Fn call should be allowed");
        }
    };
    immediate_closure_fn_reentrant(ImmediateClosure::new_aborting(&func));
    // Both the outer and reentrant call should have succeeded
    assert_eq!(call_count.get(), 2);
}

/// Test that ImmediateClosure::wrap_mut_aborting works with closures capturing RefCell (not UnwindSafe).
#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn immediate_closure_wrap_allows_unwind_unsafe() {
    let data = RefCell::new(0);
    // wrap_mut_aborting does NOT require UnwindSafe, so this compiles
    let _closure: ImmediateClosure<dyn FnMut()> = ImmediateClosure::new_mut_aborting(&mut || {
        *data.borrow_mut() += 1;
    });
}

// Test closure upcasting
mod closure_variance {
    use super::*;
    use js_sys::Undefined;
    use js_sys::{JsString, Number};
    use wasm_bindgen::prelude::Upcast;

    #[wasm_bindgen_test]
    fn return_covariance_i32_to_number() {
        let closure: Closure<dyn Fn() -> i32> = Closure::new(|| 42i32);
        let _wider: &Closure<dyn Fn() -> Number> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn return_covariance_number_to_jsvalue() {
        let closure: Closure<dyn Fn() -> Number> = Closure::new(|| Number::from(42));
        let _wider: &Closure<dyn Fn() -> JsValue> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn return_covariance_i32_to_jsvalue() {
        let closure: Closure<dyn Fn() -> i32> = Closure::new(|| 42i32);
        let _wider: &Closure<dyn Fn() -> JsValue> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn return_covariance_fnmut() {
        let closure: Closure<dyn FnMut() -> i32> = Closure::new(|| 42i32);
        let _wider: &Closure<dyn FnMut() -> Number> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_jsvalue_to_number() {
        let closure: Closure<dyn Fn(JsValue)> = Closure::new(|_: JsValue| {});
        let _narrower: &Closure<dyn Fn(Number)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_number_to_i32() {
        let closure: Closure<dyn Fn(Number)> = Closure::new(|_: Number| {});
        let _narrower: &Closure<dyn Fn(i32)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_jsvalue_to_i32() {
        let closure: Closure<dyn Fn(JsValue)> = Closure::new(|_: JsValue| {});
        let _narrower: &Closure<dyn Fn(i32)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_fnmut() {
        let closure: Closure<dyn FnMut(JsValue)> = Closure::new(|_: JsValue| {});
        let _narrower: &Closure<dyn FnMut(Number)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arg_contravariance_multiple_args() {
        let closure: Closure<dyn Fn(JsValue, JsValue)> = Closure::new(|_: JsValue, _: JsValue| {});
        let _narrower: &Closure<dyn Fn(Number, JsString)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn combined_variance() {
        let closure: Closure<dyn Fn(JsValue) -> i32> = Closure::new(|_: JsValue| 42i32);
        let _upcast: &Closure<dyn Fn(Number) -> Number> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn combined_variance_complex() {
        let closure: Closure<dyn Fn(JsValue, JsValue) -> i32> =
            Closure::new(|_: JsValue, _: JsValue| 42i32);
        let _upcast: &Closure<dyn Fn(Number, JsString) -> JsValue> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_extend_zero_to_one() {
        let closure: Closure<dyn Fn()> = Closure::new(|| {});
        let _extended: &Closure<dyn Fn(Undefined)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_extend_zero_to_two() {
        let closure: Closure<dyn Fn()> = Closure::new(|| {});
        let _extended: &Closure<dyn Fn(Undefined, Undefined)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_extend_one_to_two() {
        let closure: Closure<dyn Fn(i32)> = Closure::new(|_: i32| {});
        let _extended: &Closure<dyn Fn(i32, Undefined)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_extend_with_contravariance() {
        let closure: Closure<dyn Fn(JsValue)> = Closure::new(|_: JsValue| {});
        let _extended: &Closure<dyn Fn(Number, Undefined)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_extend_fnmut() {
        let closure: Closure<dyn FnMut()> = Closure::new(|| {});
        let _extended: &Closure<dyn FnMut(Undefined)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_shrink_one_to_zero() {
        let closure: Closure<dyn Fn(Undefined)> = Closure::new(|_: Undefined| {});
        let _shrunk: &Closure<dyn Fn()> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_shrink_two_to_zero() {
        let closure: Closure<dyn Fn(Undefined, Undefined)> =
            Closure::new(|_: Undefined, _: Undefined| {});
        let _shrunk: &Closure<dyn Fn()> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_shrink_two_to_one() {
        let closure: Closure<dyn Fn(i32, Undefined)> = Closure::new(|_: i32, _: Undefined| {});
        let _shrunk: &Closure<dyn Fn(i32)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_shrink_with_contravariance() {
        let closure: Closure<dyn Fn(JsValue, Undefined)> =
            Closure::new(|_: JsValue, _: Undefined| {});
        let _shrunk: &Closure<dyn Fn(Number)> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn arity_shrink_fnmut() {
        let closure: Closure<dyn FnMut(Undefined)> = Closure::new(|_: Undefined| {});
        let _shrunk: &Closure<dyn FnMut()> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn full_variance_extend() {
        let closure: Closure<dyn Fn(JsValue) -> i32> = Closure::new(|_: JsValue| 42i32);
        let _upcast: &Closure<dyn Fn(Number, Undefined) -> JsValue> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn full_variance_shrink() {
        let closure: Closure<dyn Fn(JsValue, Undefined) -> i32> =
            Closure::new(|_: JsValue, _: Undefined| 42i32);
        let _upcast: &Closure<dyn Fn(Number) -> JsValue> = closure.upcast();
    }

    #[wasm_bindgen_test]
    fn immediate_closure_arg_contravariance() {
        let mut func = |_: JsValue| {};
        let closure: ImmediateClosure<dyn FnMut(JsValue)> = ImmediateClosure::new_mut(&mut func);
        let _narrower: ImmediateClosure<dyn FnMut(Number)> = closure.upcast_into();
    }
}
