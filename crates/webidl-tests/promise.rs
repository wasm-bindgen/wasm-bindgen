use crate::generated::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn return_promise() {
    let f = TestPromises::new().unwrap();

    #[cfg(feature = "idl-generics-compat")]
    {
        let v = JsFuture::from(f.string_promise()).await.unwrap();
        let v = v.as_string().unwrap();
        assert_eq!(v, "abc");
    }

    #[cfg(not(feature = "idl-generics-compat"))]
    {
        let v: String = JsFuture::from(f.string_promise()).await.unwrap().into();
        assert_eq!(v, "abc");
    }
}
