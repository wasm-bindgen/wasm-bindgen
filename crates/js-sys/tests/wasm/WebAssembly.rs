use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/WebAssembly.js")]
extern "C" {
    #[wasm_bindgen(js_name = getWasmArray)]
    fn get_wasm_array() -> Uint8Array;

    #[wasm_bindgen(js_name = getTableObject)]
    fn get_table_object() -> Object;

    #[wasm_bindgen(js_name = getInvalidTableObject)]
    fn get_invalid_table_object() -> Object;

    #[wasm_bindgen(js_name = getImports)]
    fn get_imports() -> Object;
}

fn get_invalid_wasm() -> JsValue {
    ArrayBuffer::new(42).into()
}

fn get_bad_type_wasm() -> JsValue {
    2.into()
}

fn get_valid_wasm() -> JsValue {
    get_wasm_array().into()
}

#[wasm_bindgen_test]
fn validate() {
    assert!(!WebAssembly::validate(&get_invalid_wasm()).unwrap());

    assert!(WebAssembly::validate(&get_bad_type_wasm()).is_err());
}

#[wasm_bindgen_test]
async fn compile_compile_error() {
    let p = WebAssembly::compile(&get_invalid_wasm());
    let e = JsFuture::from(p).await.unwrap_err();
    assert!(e.is_instance_of::<WebAssembly::CompileError>());
}

#[wasm_bindgen_test]
async fn compile_type_error() {
    let p = WebAssembly::compile(&get_bad_type_wasm());
    let e = JsFuture::from(p).await.unwrap_err();
    assert!(e.is_instance_of::<TypeError>());
}

#[wasm_bindgen_test]
async fn compile_valid() {
    let p = WebAssembly::compile(&get_valid_wasm());
    let module = JsFuture::from(p).await.unwrap();
    assert!(module.is_instance_of::<WebAssembly::Module>());
}

#[wasm_bindgen_test]
fn module_inheritance() {
    let module = WebAssembly::Module::new(&get_valid_wasm()).unwrap();
    assert!(module.is_instance_of::<WebAssembly::Module>());
    assert!(module.is_instance_of::<Object>());

    let _: &Object = module.as_ref();
}

#[wasm_bindgen_test]
fn module_error() {
    let error = WebAssembly::Module::new(&get_invalid_wasm()).err().unwrap();
    assert!(error.is_instance_of::<WebAssembly::CompileError>());

    let error = WebAssembly::Module::new(&get_bad_type_wasm())
        .err()
        .unwrap();
    assert!(error.is_instance_of::<TypeError>());
}

#[wasm_bindgen_test]
fn module_custom_sections() {
    let module = WebAssembly::Module::new(&get_valid_wasm()).unwrap();
    let cust_sec = WebAssembly::Module::custom_sections(&module, "abcd");
    assert_eq!(cust_sec.length(), 0);
}

#[wasm_bindgen_test]
fn module_exports() {
    let module = WebAssembly::Module::new(&get_valid_wasm()).unwrap();
    let exports = WebAssembly::Module::exports(&module);
    assert_eq!(exports.length(), 1);
}

#[wasm_bindgen_test]
fn module_imports() {
    let module = WebAssembly::Module::new(&get_valid_wasm()).unwrap();
    let imports = WebAssembly::Module::imports(&module);
    assert_eq!(imports.length(), 1);
}

#[wasm_bindgen_test]
fn table_inheritance() {
    let table = WebAssembly::Table::new(&get_table_object()).unwrap();
    assert!(table.is_instance_of::<WebAssembly::Table>());
    assert!(table.is_instance_of::<Object>());

    let _: &Object = table.as_ref();
}

#[wasm_bindgen_test]
fn table_error() {
    let error = WebAssembly::Table::new(&get_invalid_table_object())
        .err()
        .unwrap();
    assert!(error.is_instance_of::<RangeError>());
}

#[wasm_bindgen_test]
fn table() {
    let table = WebAssembly::Table::new(&get_table_object()).unwrap();
    assert_eq!(table.length(), 1);

    assert!(table.get(0).is_ok());
    assert!(table.get(999).is_err());

    table.grow(1).unwrap();
    assert_eq!(table.length(), 2);

    let f = table.get(0).unwrap();
    table.set(1, &f).unwrap();
}

#[wasm_bindgen_test]
fn compile_error_inheritance() {
    let error = WebAssembly::CompileError::new("");
    assert!(error.is_instance_of::<WebAssembly::CompileError>());
    assert!(error.is_instance_of::<Error>());

    let _: &Error = error.as_ref();
}

#[wasm_bindgen_test]
fn link_error_inheritance() {
    let error = WebAssembly::LinkError::new("");
    assert!(error.is_instance_of::<WebAssembly::LinkError>());
    assert!(error.is_instance_of::<Error>());

    let _: &Error = error.as_ref();
}

#[wasm_bindgen_test]
fn runtime_error_inheritance() {
    let error = WebAssembly::RuntimeError::new("");
    assert!(error.is_instance_of::<WebAssembly::RuntimeError>());
    assert!(error.is_instance_of::<Error>());

    let _: &Error = error.as_ref();
}

#[wasm_bindgen_test]
fn webassembly_instance() {
    let module = WebAssembly::Module::new(&get_valid_wasm()).unwrap();
    let imports = get_imports();
    let instance = WebAssembly::Instance::new(&module, &imports).unwrap();

    // Inheritance chain is correct.
    assert!(instance.is_instance_of::<WebAssembly::Instance>());
    assert!(instance.is_instance_of::<Object>());
    let _: &Object = instance.as_ref();

    // Has expected exports.
    let exports = instance.exports();
    assert!(Reflect::has_str(exports.as_ref(), &"exported_func".into()).unwrap());
}

#[wasm_bindgen_test]
async fn instantiate_module() {
    let module = WebAssembly::Module::new(&get_valid_wasm()).unwrap();
    let imports = get_imports();
    let p = WebAssembly::instantiate_module(&module, &imports);
    let inst = JsFuture::from(p).await.unwrap();
    assert!(inst.is_instance_of::<WebAssembly::Instance>());
}

#[wasm_bindgen_test]
fn memory_works() {
    let obj = Object::new();
    Reflect::set(obj.as_ref(), &JsString::from("initial"), &1.into()).unwrap();
    let mem = WebAssembly::Memory::new(&obj).unwrap();
    assert!(mem.is_instance_of::<WebAssembly::Memory>());
    assert!(mem.is_instance_of::<Object>());
    assert!(mem.buffer().is_instance_of::<ArrayBuffer>());
    assert_eq!(mem.grow(1), 1);
    assert_eq!(mem.grow(2), 2);
    assert_eq!(mem.grow(3), 4);
    assert_eq!(
        mem.buffer()
            .dyn_into::<ArrayBuffer>()
            .unwrap()
            .byte_length(),
        7 * 64 * 1024,
    );
}

#[wasm_bindgen_test]
fn table_get_and_set_raw() {
    let obj = Object::new();
    Reflect::set(
        obj.as_ref(),
        &JsString::from("element"),
        &"externref".into(),
    )
    .unwrap();
    Reflect::set(obj.as_ref(), &JsString::from("initial"), &1.into()).unwrap();
    let tbl = WebAssembly::Table::new(&obj).unwrap();

    assert_eq!(tbl.length(), 1);
    tbl.set_raw(0, &JsValue::from(42)).unwrap();
    let value = tbl.get_raw(0).unwrap();
    assert_eq!(value, JsValue::from(42));
}

#[wasm_bindgen_test]
fn new_and_grow_with_value() {
    let obj = Object::new();
    Reflect::set(
        obj.as_ref(),
        &JsString::from("element"),
        &"externref".into(),
    )
    .unwrap();
    Reflect::set(obj.as_ref(), &JsString::from("initial"), &1.into()).unwrap();
    let tbl = WebAssembly::Table::new_with_value(&obj, JsValue::from(42)).unwrap();

    assert_eq!(tbl.length(), 1);
    let value = tbl.get_raw(0).unwrap();
    assert_eq!(value, JsValue::from(42));

    tbl.grow_with_value(1, JsValue::from(43)).unwrap();
    assert_eq!(tbl.length(), 2);
    let value = tbl.get_raw(1).unwrap();
    assert_eq!(value, JsValue::from(43));
}

fn wasm_has(name: &str) -> bool {
    let ns = Reflect::get_str(global().as_ref(), &"WebAssembly".into())
        .unwrap()
        .unwrap();
    Reflect::has_str(ns.unchecked_ref::<Object>(), &name.into()).unwrap_or(false)
}

fn compile_options() -> Object {
    let builtins = Array::<JsValue>::new();
    builtins.push(&"js-string".into());
    let options = Object::new();
    Reflect::set(options.as_ref(), &JsString::from("builtins"), &builtins).unwrap();
    options
}

#[wasm_bindgen_test]
fn js_tag() {
    if !wasm_has("JSTag") {
        return;
    }
    let tag = WebAssembly::JS_TAG.with(|t| t.clone());
    assert!(tag.is_instance_of::<WebAssembly::Tag>());
    let other = WebAssembly::JS_TAG.with(|t| t.clone());
    assert_eq!(JsValue::from(tag), JsValue::from(other));
}

#[wasm_bindgen_test]
fn exception_stack() {
    if !wasm_has("Exception") {
        return;
    }
    let desc = Object::new();
    Reflect::set(
        desc.as_ref(),
        &JsString::from("parameters"),
        &Array::<JsValue>::new(),
    )
    .unwrap();
    let tag = WebAssembly::Tag::new(&desc).unwrap();
    assert!(WebAssembly::Exception::new(&tag, &Array::<JsValue>::new())
        .unwrap()
        .stack()
        .is_none());

    let options = Object::new();
    Reflect::set(
        options.as_ref(),
        &JsString::from("traceStack"),
        &true.into(),
    )
    .unwrap();
    let traced =
        WebAssembly::Exception::new_with_options(&tag, &Array::<JsValue>::new(), &options).unwrap();
    assert!(traced.stack().is_some());
}

#[wasm_bindgen_test]
fn global_value_of() {
    let desc = Object::new();
    Reflect::set(desc.as_ref(), &JsString::from("value"), &"i32".into()).unwrap();
    Reflect::set(desc.as_ref(), &JsString::from("mutable"), &true.into()).unwrap();
    let global = WebAssembly::Global::new(&desc, &7.into()).unwrap();
    assert_eq!(global.value_of(), 7);
    global.set_value(&9.into());
    assert_eq!(global.value_of(), 9);
}

#[wasm_bindgen_test]
fn memory_resizable_and_fixed_length_buffers() {
    let desc = Object::new();
    Reflect::set(desc.as_ref(), &JsString::from("initial"), &1.into()).unwrap();
    Reflect::set(desc.as_ref(), &JsString::from("maximum"), &2.into()).unwrap();
    let mem = WebAssembly::Memory::new(&desc).unwrap();
    if !Reflect::has_str(mem.as_ref(), &"toResizableBuffer".into()).unwrap() {
        return;
    }

    let resizable: ArrayBuffer = mem.to_resizable_buffer().unwrap().unchecked_into();
    assert!(resizable.resizable());
    assert_eq!(resizable.max_byte_length(), 2 * 64 * 1024);
    assert_eq!(mem.buffer(), JsValue::from(&resizable));
    assert_eq!(mem.grow(1), 1);
    assert_eq!(resizable.byte_length(), 2 * 64 * 1024);

    let fixed: ArrayBuffer = mem.to_fixed_length_buffer().unwrap().unchecked_into();
    assert!(!fixed.resizable());
    assert!(resizable.detached());
    assert_eq!(mem.buffer(), JsValue::from(&fixed));

    let unbounded = Object::new();
    Reflect::set(unbounded.as_ref(), &JsString::from("initial"), &1.into()).unwrap();
    assert!(WebAssembly::Memory::new(&unbounded)
        .unwrap()
        .to_resizable_buffer()
        .is_err());
}

#[wasm_bindgen_test]
async fn compile_options_variants() {
    if !WebAssembly::validate_with_options(&get_valid_wasm(), &compile_options()).unwrap_or(false) {
        return;
    }
    let module =
        WebAssembly::Module::new_with_options(&get_valid_wasm(), &compile_options()).unwrap();
    assert!(module.is_instance_of::<WebAssembly::Module>());

    let compiled = JsFuture::from(WebAssembly::compile_with_options(
        &get_valid_wasm(),
        &compile_options(),
    ))
    .await
    .unwrap();
    assert!(compiled.is_instance_of::<WebAssembly::Module>());

    let source: WebAssembly::InstantiatedSource =
        JsFuture::from(WebAssembly::instantiate_buffer_with_options(
            &get_wasm_array().to_vec(),
            &get_imports(),
            &compile_options(),
        ))
        .await
        .unwrap()
        .unchecked_into();
    assert!(source.module().is_instance_of::<WebAssembly::Module>());
    assert!(source.instance().is_instance_of::<WebAssembly::Instance>());
    assert!(Reflect::has_str(
        source.instance().exports().as_ref(),
        &"exported_func".into()
    )
    .unwrap());

    assert!(
        WebAssembly::Module::new_with_options(&get_invalid_wasm(), &compile_options()).is_err()
    );
}
