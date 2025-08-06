#![cfg(web_sys_unstable_apis)]

#[macro_use]
mod utils;

use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn request_animation_frame(session: &XrSession, f: &Closure<dyn FnMut(f64, XrFrame)>) -> u32 {
    // This turns the Closure into a js_sys::Function
    // See https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html#casting-a-closure-to-a-js_sysfunction
    session.request_animation_frame(f.as_ref().unchecked_ref())
}

pub fn create_webgl_context(xr_mode: bool) -> Result<WebGl2RenderingContext, JsValue> {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let gl: WebGl2RenderingContext = if xr_mode {
        let gl_attribs = Object::new();
        Reflect::set(
            &gl_attribs,
            &JsValue::from_str("xrCompatible"),
            &JsValue::TRUE,
        )
        .unwrap();

        canvas
            .get_context_with_context_options("webgl2", &gl_attribs)?
            .unwrap()
            .dyn_into()?
    } else {
        canvas.get_context("webgl2")?.unwrap().dyn_into()?
    };

    Ok(gl)
}

#[wasm_bindgen]
pub struct XrApp {
    session: Rc<RefCell<Option<XrSession>>>,
    gl: Rc<WebGl2RenderingContext>,
}

#[wasm_bindgen]
impl XrApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> XrApp {
        set_panic_hook();

        let session = Rc::new(RefCell::new(None));

        let xr_mode = true;
        let gl = Rc::new(create_webgl_context(xr_mode).unwrap());

        XrApp { session, gl }
    }

    pub fn init(&self) -> Promise {
        log!("Starting WebXR...");
        let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
        let xr = navigator.xr();
        let session_mode = XrSessionMode::Inline;
        let session_supported_promise = xr.is_session_supported(session_mode);

        // Note: &self is on the stack so we can't use it in a future (which will
        // run after the &self reference is out or scope). Clone ref to the parts
        // of self we'll need, then move them into the Future
        // See https://github.com/wasm-bindgen/wasm-bindgen/issues/1858#issuecomment-552095511
        let session = self.session.clone();
        let gl = self.gl.clone();

        let future_ = async move {
            let supports_session =
                wasm_bindgen_futures::JsFuture::from(session_supported_promise).await;
            let supports_session = supports_session.unwrap();
            if supports_session == false {
                log!("XR session not supported");
                return Ok(JsValue::from("XR session not supported"));
            }

            let xr_session_promise = xr.request_session(session_mode);
            let xr_session = wasm_bindgen_futures::JsFuture::from(xr_session_promise).await;
            let xr_session: XrSession = xr_session.unwrap().into();

            let xr_gl_layer = XrWebGlLayer::new_with_web_gl2_rendering_context(&xr_session, &gl)?;
            let mut render_state_init = XrRenderStateInit::new();
            render_state_init.base_layer(Some(&xr_gl_layer));
            xr_session.update_render_state_with_state(&render_state_init);

            let mut session = session.borrow_mut();
            session.replace(xr_session);

            Ok(JsValue::from("Session set"))
        };

        future_to_promise(future_)
    }

    pub fn start(&self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut i = 0;
        *g.borrow_mut() = Some(Closure::new(move |_time: f64, frame: XrFrame| {
            log!("Frame rendering...");
            if i > 2 {
                log!("All done!");

                // Drop our handle to this closure so that it will get cleaned
                // up once we return.
                let _ = f.borrow_mut().take();
                return;
            }

            let sess: XrSession = frame.session();
            i += 1;

            // Schedule ourself for another requestAnimationFrame callback.
            // TODO: WebXR Samples call this at top of request_animation_frame - should this be moved?
            request_animation_frame(&sess, f.borrow().as_ref().unwrap());
        }));

        let session: &Option<XrSession> = &self.session.borrow();
        let sess: &XrSession = if let Some(sess) = session {
            sess
        } else {
            return ();
        };

        request_animation_frame(sess, g.borrow().as_ref().unwrap());
    }
}
