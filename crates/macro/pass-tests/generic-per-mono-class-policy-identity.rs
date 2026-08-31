use wasm_bindgen::prelude::*;

mod erased {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Holder<T>;

        #[wasm_bindgen(method)]
        pub fn erased_get<T>(this: &Holder<T>) -> T;
    }
}

mod per_mono {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(generic_per_mono)]
        pub type Holder<T>;

        #[wasm_bindgen(method, generic_per_mono)]
        pub fn mono_get<T>(this: &Holder<T>) -> T;
    }
}

mod aliased {
    use super::*;
    use crate::per_mono::Holder as RenamedHolder;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(method, generic_per_mono, js_name = aliasGet)]
        pub fn alias_get<T>(this: &RenamedHolder<T>) -> T;
    }
}

mod qualified_target {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(generic_per_mono)]
        pub type Holder<T>;
    }
}

mod qualified_collision {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[cfg(any())]
        pub type Holder<T>;

        #[wasm_bindgen(method, generic_per_mono, js_name = qualifiedGet)]
        pub fn qualified_get<T>(this: &crate::qualified_target::Holder<T>) -> T;
    }
}

fn assert_policies(erased: &erased::Holder<JsValue>, mono: &per_mono::Holder<JsValue>) {
    let _: JsValue = erased.erased_get();
    let _: JsValue = mono.mono_get();
    let _: JsValue = mono.alias_get();
}

fn assert_qualified_policy(value: &qualified_target::Holder<JsValue>) {
    let _: JsValue = value.qualified_get();
}

fn main() {}
