use wasm_bindgen::prelude::*;

trait Projected {
    type Class;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(static_method_of = <T as Projected>::Class)]
    fn qualified_self<T: Projected>();
}

#[wasm_bindgen]
extern "C" {
    type Holder<T = JsValue>;
    type LtHolder<'a, T>;

    #[wasm_bindgen(static_method_of = Holder<T::Item>)]
    fn erased_projection<T: IntoIterator>();

    #[wasm_bindgen(static_method_of = Holder<T::Item>, experimental_generic_mono)]
    fn mono_projection<T: IntoIterator>();

    #[wasm_bindgen(static_method_of = Holder<_>)]
    fn erased_inferred<T>();

    #[wasm_bindgen(method)]
    fn erased_elided<T>(this: &LtHolder<'_, T>);
}

trait LifetimeBound<'a> {}

#[wasm_bindgen]
extern "C" {
    type HrtbConflict<'class, T>
    where
        for<'target> &'class T: LifetimeBound<'target>;

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn hrtb_conflict<'target, T>(this: &'target HrtbConflict<'target, T>);

    type HrtbTypeConflict<T>
    where
        for<'target> T: LifetimeBound<'target>;

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn hrtb_type_conflict<'target, T>(
        this: &HrtbTypeConflict<&'target T>,
    );

    #[wasm_bindgen(method, experimental_generic_mono)]
    fn hrtb_shim_conflict<'target, T>(
        this: &HrtbTypeConflict<T>,
        value: &'target T,
    );
}

fn main() {}
