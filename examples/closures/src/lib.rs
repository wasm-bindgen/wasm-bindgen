use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    // Demonstrate ScopedClosure for immediate callbacks.
    // This is the recommended way to pass closures to JS for synchronous use.
    // The closure can capture local references and is automatically cleaned up.
    demonstrate_scoped_closure();

    // Note: js_sys::Array::for_each currently still uses the old `&mut dyn FnMut`
    // pattern, which will be migrated to Closure in a future release.
    let array = Array::new();
    array.push(&"Hello".into());
    array.push(&1.into());
    let mut first_item = None;
    array.for_each(&mut |obj, idx, _arr| match idx {
        0 => {
            assert_eq!(obj, "Hello");
            first_item = obj.as_string();
        }
        1 => assert_eq!(obj, 1),
        _ => panic!("unknown index: {idx}"),
    });
    assert_eq!(first_item, Some("Hello".to_string()));

    // Below are some more advanced usages of the `Closure` type for closures
    // that need to live beyond our function call.

    setup_clock(&window, &document)?;
    setup_clicker(&document);

    // And now that our demo is ready to go let's switch things up so
    // everything is displayed and our loading prompt is hidden.
    document
        .get_element_by_id("loading")
        .expect("should have #loading on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;
    document
        .get_element_by_id("script")
        .expect("should have #script on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#script should be an `HtmlElement`")
        .style()
        .set_property("display", "block")?;

    Ok(())
}

// Set up a clock on our page and update it each second to ensure it's got
// an accurate date.
//
// Note the usage of `Closure` here because the closure is "long lived",
// basically meaning it has to persist beyond the call to this one function.
// Also of note here is the `.as_ref().unchecked_ref()` chain, which is how
// you can extract `&Function`, what `web-sys` expects, from a `Closure`
// which only hands you `&JsValue` via `AsRef`.
fn setup_clock(window: &Window, document: &Document) -> Result<(), JsValue> {
    let current_time = document
        .get_element_by_id("current-time")
        .expect("should have #current-time on the page");
    update_time(&current_time);
    let a = Closure::<dyn Fn()>::new(move || update_time(&current_time));
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
    fn update_time(current_time: &Element) {
        current_time.set_inner_html(&String::from(
            Date::new_0().to_locale_string("en-GB", &JsValue::undefined()),
        ));
    }

    // The instance of `Closure` that we created will invalidate its
    // corresponding JS callback whenever it is dropped, so if we were to
    // normally return from `setup_clock` then our registered closure will
    // raise an exception when invoked.
    //
    // Normally we'd store the handle to later get dropped at an appropriate
    // time but for now we want it to be a global handler so we use the
    // `forget` method to drop it without invalidating the closure. Note that
    // this is leaking memory in Rust, so this should be done judiciously!
    a.forget();

    Ok(())
}

// Demonstrate ScopedClosure for immediate/synchronous callbacks.
//
// Use ScopedClosure::borrow (for FnMut) or ScopedClosure::borrow_immutable (for Fn) when
// JavaScript will call the closure immediately and won't retain it. Benefits:
// - Can capture non-'static references (like &mut local_var)
// - Automatic cleanup when ScopedClosure is dropped
// - Lifetime safety: ScopedClosure can't outlive the closure's captured data
// - Unwind safe (panics become JS exceptions)
fn demonstrate_scoped_closure() {
    #[wasm_bindgen(inline_js = r#"
        export function callThreeTimes(cb) {
            cb(1);
            cb(2);
            cb(3);
        }
    "#)]
    extern "C" {
        // This JS function calls the callback immediately, three times
        fn callThreeTimes(cb: &ScopedClosure<dyn FnMut(u32)>);
    }

    // Example: Using ScopedClosure::borrow to sum values
    // The closure captures &mut sum without requiring 'static
    let mut sum = 0u32;

    {
        let mut func = |value: u32| {
            sum += value;
        };
        let closure = ScopedClosure::borrow_mut(&mut func);
        // Pass the closure to JavaScript - it will be called synchronously
        // and then invalidated when closure is dropped
        callThreeTimes(&closure);
    }

    assert_eq!(sum, 6);
}

// We also want to count the number of times that our green square has been
// clicked. Our callback will update the `#num-clicks` div.
//
// This is pretty similar above, but showing how closures can also implement
// `FnMut()`.
fn setup_clicker(document: &Document) {
    let num_clicks = document
        .get_element_by_id("num-clicks")
        .expect("should have #num-clicks on the page");
    let mut clicks = 0;
    let a = Closure::<dyn FnMut()>::new(move || {
        clicks += 1;
        num_clicks.set_inner_html(&clicks.to_string());
    });
    document
        .get_element_by_id("green-square")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    // See comments in `setup_clock` above for why we use `a.forget()`.
    a.forget();
}
