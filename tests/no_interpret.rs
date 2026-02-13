const _: () = {
    unsafe extern "C" fn __ctor() {
        let _ = 1 + 1;
    }

    #[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
    #[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".init_array"))]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};

#[cfg(test)]
#[wasm_bindgen_test::wasm_bindgen_test]
fn it_works() {}
