use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlElement, HtmlInputElement, MessageEvent, Worker};

/// A number evaluation struct
///
/// This struct will be the main object which responds to messages passed to the
/// worker. It stores the last number which it was passed to have a state. The
/// statefulness is not required in this example but should show how
/// larger, more complex scenarios with statefulness can be set up.
#[wasm_bindgen]
pub struct NumberEval {
    number: i32,
}

#[wasm_bindgen]
impl NumberEval {
    /// Create new instance.
    pub fn new() -> NumberEval {
        NumberEval { number: 0 }
    }

    /// Check if a number is even and store it as last processed number.
    ///
    /// # Arguments
    ///
    /// * `number` - The number to be checked for being even/odd.
    pub fn is_even(&mut self, number: i32) -> bool {
        self.number = number;
        self.number % 2 == 0
    }

    /// Get last number that was checked - this method is added to work with
    /// statefulness.
    pub fn get_last_number(&self) -> i32 {
        self.number
    }
}

/// Run entry point for the main thread.
#[wasm_bindgen]
pub fn startup() {
    // Here, we create our worker. In a larger app, multiple callbacks should be
    // able to interact with the code in the worker. Therefore, we wrap it in
    // `Rc<RefCell>` following the interior mutability pattern. Here, it would
    // not be needed but we include the wrapping anyway as example.
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within Wasm".into());

    // Pass the worker to the function which sets up the `oninput` callback.
    setup_input_oninput_callback(worker_handle);
}

fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    // If our `onmessage` callback should stay valid after exiting from the
    // `oninput` closure scope, we need to either forget it (so it is not
    // destroyed) or store it somewhere. To avoid leaking memory every time we
    // want to receive a response from the worker, we move a handle into the
    // `oninput` closure to which we will always attach the last `onmessage`
    // callback. The initial value will not be used and we silence the warning.
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::new(move || {
        console::log_1(&"oninput callback triggered".into());
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document
            .get_element_by_id("inputNumber")
            .expect("#inputNumber should exist");
        let input_field = input_field
            .dyn_ref::<HtmlInputElement>()
            .expect("#inputNumber should be a HtmlInputElement");

        // If the value in the field can be parsed to a `i32`, send it to the
        // worker. Otherwise clear the result field.
        match input_field.value().parse::<i32>() {
            Ok(number) => {
                // Access worker behind shared handle, following the interior
                // mutability pattern.
                let worker_handle = &*worker.borrow();
                let _ = worker_handle.post_message(&number.into());
                persistent_callback_handle = get_on_msg_callback();

                // Since the worker returns the message asynchronously, we
                // attach a callback to be triggered when the worker returns.
                worker_handle
                    .set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
            }
            Err(_) => {
                document
                    .get_element_by_id("resultField")
                    .expect("#resultField should exist")
                    .dyn_ref::<HtmlElement>()
                    .expect("#resultField should be a HtmlInputElement")
                    .set_inner_text("");
            }
        }
    });

    // Attach the closure as `oninput` callback to the input field.
    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

/// Create a closure to act on the message returned by the worker
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::new(move |event: MessageEvent| {
        console::log_2(&"Received response: ".into(), &event.data());

        let result = match event.data().as_bool().unwrap() {
            true => "even",
            false => "odd",
        };

        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("resultField")
            .expect("#resultField should exist")
            .dyn_ref::<HtmlElement>()
            .expect("#resultField should be a HtmlInputElement")
            .set_inner_text(result);
    })
}
