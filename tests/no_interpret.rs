const _: () = {
    unsafe extern "C" fn __ctor() {
        let _ = 1 + 1;
    }

    #[unsafe(link_section = ".init_array")]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};

#[cfg(test)]
#[wasm_bindgen_test::wasm_bindgen_test]
fn it_works() {}
