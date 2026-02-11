use js_sys::Number;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
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
    let a = Closure::new_aborting(move || hit2.set(true));
    assert!(!hit.get());
    long_lived_call1(&a);
    assert!(hit.get());

    let hit = Rc::new(Cell::new(false));
    {
        let hit = hit.clone();
        let a = Closure::new_aborting(move |x| {
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
    many_arity_call3(&StaticClosure::new(|a, b| assert_eq!((a, b), (1, 2))));
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
    let a = Closure::new_aborting(move || hit2.set(true));
    assert!(!hit.get());
    option_call1(Some(&a));
    assert!(hit.get());

    let hit = Rc::new(Cell::new(false));
    {
        let hit = hit.clone();
        let a = Closure::new_aborting(move |x| {
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

#[wasm_bindgen_test]
fn long_lived_dropping() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::new_aborting(move || hit2.set(true));
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

    let a = Closure::new_aborting(move || hit2.set(true));

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
    let x: Closure<dyn Fn()> = Closure::new_aborting(move || {
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
    drop(Closure::new_aborting(move || {
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
    let a = Closure::new_aborting(move || hit2.set(true));
    assert!(!hit.get());
    abort_closure_call1(&a);
    assert!(hit.get());
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn abort_closure_with_non_unwind_safe() {
    use std::cell::RefCell;

    // RefCell is not UnwindSafe, but Closure::new_aborting doesn't require it
    let rc = Rc::new(RefCell::new(0));
    let rc2 = rc.clone();
    let a = Closure::new_aborting(move || {
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
    let a = Closure::new_aborting(move |x| {
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
    fn closure_with_cache(f: &ScopedClosure<dyn FnMut()>);
    #[wasm_bindgen(catch)]
    fn closure_with_call_cached() -> Result<(), JsValue>;
    fn closure_with_call_and_cache(f: &ScopedClosure<dyn FnMut(u32)>);
    fn closure_with_call_cached_throws() -> bool;
}

/// Test that ScopedClosure::borrow_mut works correctly during the callback body
#[wasm_bindgen_test]
fn closure_with_works_during_body() {
    let called = Cell::new(false);
    {
        let mut func = || {
            called.set(true);
        };
        let closure = ScopedClosure::borrow_mut(&mut func);
        closure_with_call(&closure);
    }
    assert!(called.get());
}

/// Test that ScopedClosure::borrow_mut allows capturing non-'static references
#[wasm_bindgen_test]
fn closure_with_captures_non_static() {
    let mut value = 0u32;
    {
        let mut func = || {
            value += 1;
        };
        let closure = ScopedClosure::borrow_mut(&mut func);
        closure_with_call(&closure);
        closure_with_call(&closure);
        closure_with_call(&closure);
    }
    assert_eq!(value, 3);
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
        let mut func = |value| {
            sum += value;
        };
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

    // For non-'static captures, use ScopedClosure::borrow_mut
    {
        let mut func = || {
            called.set(true);
        };
        let scoped = ScopedClosure::borrow_mut(&mut func);
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
    let closure = Closure::new(move || {
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
    let closure = Closure::new(move |value| {
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
    let closure = Closure::new(move || {
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
    fn closure_fn_with_call(f: &ScopedClosure<dyn Fn()>);
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
        let closure = ScopedClosure::borrow(&func);
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
        let closure = ScopedClosure::borrow(&func);
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
        let closure = ScopedClosure::borrow(&func);
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
    let closure = ScopedClosure::own(move || {
        called_clone.set(true);
    });

    closure_take_ownership(closure);
    assert!(called.get());
}

/// Test that ScopedClosure::borrow_mut_aborting works
#[wasm_bindgen_test]
#[allow(deprecated)]
fn scoped_closure_borrow_mut_aborting() {
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

/// Test that ScopedClosure::borrow_aborting works
#[wasm_bindgen_test]
fn scoped_closure_borrow_aborting() {
    use std::rc::Rc;

    // Rc<Cell<T>> is not UnwindSafe, so we need _aborting variant
    let counter = Rc::new(Cell::new(0u32));
    {
        let func = || {
            counter.set(counter.get() + 1);
        };
        let closure = ScopedClosure::borrow_aborting(&func);
        closure_fn_with_call(&closure);
        closure_fn_with_call(&closure);
    }
    assert_eq!(counter.get(), 2);
}

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    fn immediate_closure_call(f: &ImmediateClosure<dyn FnMut()>);
    fn immediate_closure_call_arg(f: &ImmediateClosure<dyn FnMut(u32)>, value: u32);
    fn immediate_closure_call_ret(f: &ImmediateClosure<dyn FnMut(u32) -> u32>, value: u32) -> u32;
    fn immediate_closure_fn_call(f: &ImmediateClosure<dyn Fn()>);
    fn immediate_closure_catches_panic(f: &ImmediateClosure<dyn FnMut()>) -> bool;
}

#[wasm_bindgen_test]
fn immediate_closure_basic() {
    let mut called = false;
    immediate_closure_call(&ImmediateClosure::new(&mut || {
        called = true;
    }));
    assert!(called);
}

#[wasm_bindgen_test]
fn immediate_closure_with_args() {
    let mut sum = 0u32;
    // Test inference: x should be inferred as u32 from the function signature
    immediate_closure_call_arg(
        &ImmediateClosure::new(&mut |x| {
            sum += x;
        }),
        42,
    );
    assert_eq!(sum, 42);
}

#[wasm_bindgen_test]
fn immediate_closure_with_return() {
    // Test inference: x and return type should be inferred from the function signature
    let result = immediate_closure_call_ret(&ImmediateClosure::new(&mut |x| x * 2), 21);
    assert_eq!(result, 42);
}

#[wasm_bindgen_test]
fn immediate_closure_immutable() {
    let data = vec![1, 2, 3];
    immediate_closure_fn_call(&ImmediateClosure::new_immutable(&|| {
        assert_eq!(data.len(), 3);
    }));
    // data is still accessible after
    assert_eq!(data.len(), 3);
}

#[wasm_bindgen_test]
fn immediate_closure_debug() {
    let mut f = || {};
    let closure = ImmediateClosure::new(&mut f);
    assert_eq!(&format!("{:?}", closure), "ImmediateClosure { .. }");
}

#[cfg(all(feature = "std", target_arch = "wasm32", panic = "unwind"))]
#[wasm_bindgen_test]
fn immediate_closure_catches_panic_test() {
    let caught = immediate_closure_catches_panic(&ImmediateClosure::new(&mut || {
        panic!("test panic");
    }));
    assert!(
        caught,
        "panic should be caught and converted to JS exception"
    );
}

#[wasm_bindgen_test]
fn immediate_closure_to_scoped_closure() {
    let mut sum = 0u32;
    {
        let mut func = |value| {
            sum += value;
        };
        let immediate = ImmediateClosure::new(&mut func);
        // Convert ImmediateClosure to ScopedClosure
        let scoped: ScopedClosure<dyn FnMut(u32)> = (&immediate).into();
        closure_with_call_and_cache(&scoped);
    }
    assert_eq!(sum, 6); // 1 + 2 + 3
}
