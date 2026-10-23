use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/DisposableStack.js")]
extern "C" {
    fn make_disposable(log: &Array<JsString>, name: &str) -> Object;
    fn make_throwing_disposable(message: &str) -> Object;
    fn make_async_disposable(log: &Array<JsString>, name: &str) -> Object;
    fn delayed_push(log: &Array<JsString>, name: &str) -> Promise<JsValue>;
}

fn has_disposable_stack() -> bool {
    Reflect::has_str(global().as_ref(), &"DisposableStack".into()).unwrap_or(false)
}

fn error_message(err: &JsValue) -> JsString {
    err.unchecked_ref::<Error>().message()
}

#[wasm_bindgen_test]
fn new_dispose_disposed() {
    if !has_disposable_stack() {
        return;
    }
    let stack = DisposableStack::new();
    assert!(stack.is_instance_of::<Object>());
    assert!(!stack.disposed());
    stack.dispose().unwrap();
    assert!(stack.disposed());
    stack.dispose().unwrap();
    assert!(stack.disposed());
}

#[wasm_bindgen_test]
fn use_() {
    if !has_disposable_stack() {
        return;
    }
    let log: Array<JsString> = Array::new_typed();
    let stack = DisposableStack::new();

    let a = make_disposable(&log, "a");
    assert_eq!(stack.use_(&a).unwrap(), a);
    let b = make_disposable(&log, "b");
    assert_eq!(stack.use_(&b).unwrap(), b);
    assert!(stack.use_(&JsValue::NULL).unwrap().is_null());

    let err = stack.use_(&Object::new()).unwrap_err();
    assert!(err.is_instance_of::<TypeError>());

    assert_eq!(log.length(), 0);
    stack.dispose().unwrap();
    assert_eq!(log.join(","), "b,a");
}

#[wasm_bindgen_test]
fn adopt_and_defer_lifo() {
    if !has_disposable_stack() {
        return;
    }
    let log: Array<JsString> = Array::new_typed();
    let stack = DisposableStack::new();

    let log2 = log.clone();
    let on_defer = ScopedClosure::<dyn FnMut()>::new(move || {
        log2.push(&JsString::from("defer"));
    });
    stack.defer(&on_defer).unwrap();

    let log2 = log.clone();
    let on_adopt = ScopedClosure::<dyn FnMut(JsString)>::new(move |v: JsString| {
        log2.push(&JsString::from(format!("adopt:{}", v)));
    });
    let value = JsString::from("x");
    assert_eq!(stack.adopt(&value, &on_adopt).unwrap(), value);

    stack.use_(&make_disposable(&log, "use")).unwrap();

    stack.dispose().unwrap();
    assert_eq!(log.join(","), "use,adopt:x,defer");

    stack.dispose().unwrap();
    assert_eq!(log.length(), 3);
}

#[wasm_bindgen_test]
fn defer_after_dispose_errors() {
    if !has_disposable_stack() {
        return;
    }
    let stack = DisposableStack::new();
    stack.dispose().unwrap();

    let on_defer = ScopedClosure::<dyn FnMut()>::new(|| {});
    let err = stack.defer(&on_defer).unwrap_err();
    assert!(err.is_instance_of::<ReferenceError>());

    let err = stack.move_().unwrap_err();
    assert!(err.is_instance_of::<ReferenceError>());
}

#[wasm_bindgen_test]
fn move_() {
    if !has_disposable_stack() {
        return;
    }
    let log: Array<JsString> = Array::new_typed();
    let stack = DisposableStack::new();
    stack.use_(&make_disposable(&log, "a")).unwrap();

    let moved = stack.move_().unwrap();
    assert!(stack.disposed());
    assert!(!moved.disposed());
    assert_eq!(log.length(), 0);

    stack.dispose().unwrap();
    assert_eq!(log.length(), 0);

    moved.dispose().unwrap();
    assert!(moved.disposed());
    assert_eq!(log.join(","), "a");
}

#[wasm_bindgen_test]
fn suppressed_error() {
    if !has_disposable_stack() {
        return;
    }
    let error = JsValue::from(Error::new("outer"));
    let suppressed = JsValue::from(Error::new("inner"));
    assert_eq!(SuppressedError::new(&error, &suppressed).message(), "");
    let err = SuppressedError::new_with_message(&error, &suppressed, "both");

    assert!(err.is_instance_of::<Error>());
    assert!(err.is_instance_of::<SuppressedError>());
    assert_eq!(err.error(), error);
    assert_eq!(err.suppressed(), suppressed);
    let base: &Error = err.as_ref();
    assert_eq!(base.message(), "both");
    assert_eq!(base.name(), "SuppressedError");
}

#[wasm_bindgen_test]
fn dispose_rethrows_suppressed_error() {
    if !has_disposable_stack() {
        return;
    }
    let stack = DisposableStack::new();
    stack.use_(&make_throwing_disposable("first")).unwrap();
    stack.use_(&make_throwing_disposable("second")).unwrap();

    let err = stack.dispose().unwrap_err();
    assert!(stack.disposed());
    let err: SuppressedError = err.dyn_into().unwrap();
    assert_eq!(error_message(&err.error()), "first");
    assert_eq!(error_message(&err.suppressed()), "second");
}

#[wasm_bindgen_test]
async fn async_new_dispose_disposed() {
    if !has_disposable_stack() {
        return;
    }
    let stack = AsyncDisposableStack::new();
    assert!(stack.is_instance_of::<Object>());
    assert!(!stack.disposed());
    JsFuture::from(stack.dispose_async()).await.unwrap();
    assert!(stack.disposed());
    JsFuture::from(stack.dispose_async()).await.unwrap();
    assert!(stack.disposed());
}

#[wasm_bindgen_test]
async fn async_use_adopt_defer_lifo() {
    if !has_disposable_stack() {
        return;
    }
    let log: Array<JsString> = Array::new_typed();
    let stack = AsyncDisposableStack::new();

    let log2 = log.clone();
    let on_defer_sync = ScopedClosure::<dyn FnMut()>::new(move || {
        log2.push(&JsString::from("defer"));
    });
    stack.defer(&on_defer_sync).unwrap();

    let log2 = log.clone();
    let on_defer_async = ScopedClosure::<dyn FnMut() -> Promise<JsValue>>::new(move || {
        delayed_push(&log2, "defer_async")
    });
    stack.defer(&on_defer_async).unwrap();

    let log2 = log.clone();
    let on_adopt_sync = ScopedClosure::<dyn FnMut(JsString)>::new(move |v: JsString| {
        log2.push(&JsString::from(format!("adopt:{}", v)));
    });
    let value = JsString::from("x");
    assert_eq!(stack.adopt(&value, &on_adopt_sync).unwrap(), value);

    let log2 = log.clone();
    let on_adopt_async =
        ScopedClosure::<dyn FnMut(JsString) -> Promise<JsValue>>::new(move |v: JsString| {
            delayed_push(&log2, &format!("adopt_async:{}", v))
        });
    let value = JsString::from("y");
    assert_eq!(stack.adopt(&value, &on_adopt_async).unwrap(), value);

    let sync = make_disposable(&log, "use");
    assert_eq!(stack.use_(&sync).unwrap(), sync);
    let a = make_async_disposable(&log, "use_async");
    assert_eq!(stack.use_(&a).unwrap(), a);

    let err = stack.use_(&Object::new()).unwrap_err();
    assert!(err.is_instance_of::<TypeError>());

    assert_eq!(log.length(), 0);
    JsFuture::from(stack.dispose_async()).await.unwrap();
    assert!(stack.disposed());
    assert_eq!(
        log.join(","),
        "use_async,use,adopt_async:y,adopt:x,defer_async,defer"
    );
}

#[wasm_bindgen_test]
async fn async_defer_after_dispose_errors() {
    if !has_disposable_stack() {
        return;
    }
    let stack = AsyncDisposableStack::new();
    JsFuture::from(stack.dispose_async()).await.unwrap();

    let on_defer = ScopedClosure::<dyn FnMut()>::new(|| {});
    let err = stack.defer(&on_defer).unwrap_err();
    assert!(err.is_instance_of::<ReferenceError>());

    let err = stack.move_().unwrap_err();
    assert!(err.is_instance_of::<ReferenceError>());
}

#[wasm_bindgen_test]
async fn async_move_() {
    if !has_disposable_stack() {
        return;
    }
    let log: Array<JsString> = Array::new_typed();
    let stack = AsyncDisposableStack::new();
    stack.use_(&make_async_disposable(&log, "a")).unwrap();

    let moved = stack.move_().unwrap();
    assert!(stack.disposed());
    assert!(!moved.disposed());

    JsFuture::from(stack.dispose_async()).await.unwrap();
    assert_eq!(log.length(), 0);

    JsFuture::from(moved.dispose_async()).await.unwrap();
    assert!(moved.disposed());
    assert_eq!(log.join(","), "a");
}

#[wasm_bindgen_test]
async fn async_dispose_rejects_with_suppressed_error() {
    if !has_disposable_stack() {
        return;
    }
    let stack = AsyncDisposableStack::new();
    stack.use_(&make_throwing_disposable("first")).unwrap();
    stack.use_(&make_throwing_disposable("second")).unwrap();

    let err = JsFuture::from(stack.dispose_async()).await.unwrap_err();
    assert!(stack.disposed());
    let err: SuppressedError = err.dyn_into().unwrap();
    assert_eq!(error_message(&err.error()), "first");
    assert_eq!(error_message(&err.suppressed()), "second");
}
