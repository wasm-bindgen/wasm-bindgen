use crate::oscillator::Params;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{AudioContext, HtmlInputElement, HtmlLabelElement};

pub fn create_gui(params: &'static Params, ctx: AudioContext) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let volume = add_slider(&document, &body, "Volume:").unwrap();
    let frequency = add_slider(&document, &body, "Frequency:").unwrap();
    volume.set_value("0");
    frequency.set_min("20");
    frequency.set_value("60");

    let listener = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
        params.set_frequency(frequency.value().parse().unwrap());
        params.set_volume(volume.value().parse().unwrap());
        drop(ctx.resume().unwrap());
    })
    .into_js_value();

    body.add_event_listener_with_callback("input", listener.as_ref().unchecked_ref())
        .unwrap();
}

fn add_slider(
    document: &web_sys::Document,
    body: &web_sys::HtmlElement,
    name: &str,
) -> Result<HtmlInputElement, JsValue> {
    let input: HtmlInputElement = document.create_element("input")?.unchecked_into();
    let label: HtmlLabelElement = document.create_element("label")?.unchecked_into();
    input.set_type("range");
    label.set_text_content(Some(name));
    label.append_child(&input)?;
    body.append_child(&label)?;
    Ok(input)
}
