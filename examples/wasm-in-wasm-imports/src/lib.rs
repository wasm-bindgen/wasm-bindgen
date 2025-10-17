use js_sys::{Function, Map, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(a: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn error(a: &str);

}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("native_add.wasm");

async fn run_async() -> Result<(), JsValue> {
    console_log!("instantiating a new Wasm module directly");

    let imports = make_imports()?;
    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &imports)).await?;

    let instance: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let exports = instance.exports();

    let add = Reflect::get(&exports, &"add".into())?
        .dyn_into::<Function>()
        .expect("add export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    console_log!("1 + 2 = {:?}", three);

    Ok(())
}

fn bind(this: &JsValue, func_name: &str) -> Result<(), JsValue> {
    let property_key = JsValue::from(func_name);
    let orig_func = Reflect::get(this, &property_key)?.dyn_into::<Function>()?;
    let func = orig_func.bind0(this);
    if !Reflect::set(this, &property_key, &func)? {
        return Err(JsValue::from("failed to set property"));
    }
    Ok(())
}

pub fn make_imports() -> Result<Object, JsValue> {
    let map = Map::new();
    let imports: JsValue = Imports.into();

    bind(&imports, "native_add")?;

    map.set(&JsValue::from("env"), &imports);
    Object::from_entries(&map.into())
}

#[wasm_bindgen]
pub struct Imports;

#[wasm_bindgen]
impl Imports {
    pub fn native_add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[wasm_bindgen(start)]
fn run() {
    spawn_local(async {
        match run_async().await {
            Ok(_) => console_log!("Finished"),
            Err(e) => console_error!("{:?}", e),
        }
    });
}
