use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("add.wasm");

async fn run_async() -> Result<(), JsValue> {
    console_log!("instantiating a new Wasm module directly");

    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &Object::new())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let c = b.exports();

    let add = Reflect::get(c.as_ref(), &"add".into())?
        .dyn_into::<Function>()
        .expect("add export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    console_log!("1 + 2 = {:?}", three);
    let mem = Reflect::get(c.as_ref(), &"memory".into())?
        .dyn_into::<WebAssembly::Memory>()
        .expect("memory export wasn't a `WebAssembly.Memory`");
    console_log!(
        "created module has {} pages of memory",
        mem.grow(0).unwrap()
    );
    console_log!("giving the module 4 more pages of memory");
    let new_pages = mem.grow(4).unwrap();
    console_log!("now the module has {} pages of memory", new_pages);

    Ok(())
}

#[wasm_bindgen(start)]
fn run() {
    spawn_local(async {
        run_async().await.unwrap_throw();
    });
}
