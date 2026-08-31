#![allow(dead_code)]

use wasm_bindgen::prelude::*;

trait HrtbBound<'a> {}

impl<'a, 'b, T> HrtbBound<'a> for &'b T {}

trait ActiveBound {}
trait InactiveBound {}

impl ActiveBound for JsValue {}
impl InactiveBound for JsValue {}

#[wasm_bindgen]
extern "C" {
    type Value;

    type ErasedBounded<T: Clone>;

    #[wasm_bindgen(static_method_of = ErasedBounded<T>, js_name = create)]
    fn erased_bounded_create<T>(value: T);

    #[wasm_bindgen(generic_per_mono)]
    type ExplicitBounded<T: Clone>;

    // The explicit U selects the impl even though the return uses T. Imported
    // class bounds must consequently be substituted with U, not T.
    #[wasm_bindgen(
        static_method_of = ExplicitBounded<U>,
        generic_per_mono,
        js_name = convert
    )]
    fn explicit_static_class_wins<T: Clone, U>(value: T) -> ExplicitBounded<T>;

    type ErasedHrtbHolder<'class, T>
    where
        for<'bound> &'bound T: HrtbBound<'class>;

    #[wasm_bindgen(method, js_name = erased)]
    fn hrtb_erased<'scope, T>(this: &'scope ErasedHrtbHolder<'scope, T>, value: T);

    #[wasm_bindgen(generic_per_mono)]
    type HrtbHolder<'class, T>
    where
        for<'bound> &'bound T: HrtbBound<'class>;

    #[wasm_bindgen(method, generic_per_mono, js_name = mono)]
    fn hrtb_mono<'scope, T>(this: &'scope HrtbHolder<'scope, T>, value: T);
}

#[wasm_bindgen]
extern "C" {
    #[cfg(not(any()))]
    #[cfg_attr(any(), doc = "not a compilation gate")]
    type ErasedConditional<T: ActiveBound>;

    #[cfg(any())]
    type ErasedConditional<T: InactiveBound>;

    #[wasm_bindgen(method, js_name = erased)]
    fn conditional_erased<T>(this: &ErasedConditional<T>) -> T;

    #[cfg(not(any()))]
    #[cfg_attr(any(), doc = "not a compilation gate")]
    #[wasm_bindgen(generic_per_mono)]
    type Conditional<T: ActiveBound>;

    #[cfg(any())]
    #[wasm_bindgen(generic_per_mono)]
    type Conditional<T: InactiveBound>;

    #[wasm_bindgen(method, generic_per_mono, js_name = mono)]
    fn conditional_mono<T>(this: &Conditional<T>) -> T;

    #[cfg_attr(all(), cfg(not(any())))]
    #[wasm_bindgen(generic_per_mono)]
    type AttrConditional<T: ActiveBound>;

    #[cfg_attr(all(), cfg(any()))]
    #[wasm_bindgen(generic_per_mono)]
    type AttrConditional<T: InactiveBound>;

    #[wasm_bindgen(method, generic_per_mono)]
    fn attr_conditional<T>(this: &AttrConditional<T>) -> T;

    #[cfg_attr(all(), cfg(any()))]
    type Hidden<T>;

    #[cfg(not(any()))]
    #[wasm_bindgen(generic_per_mono)]
    type ShapeConditional<T>;

    #[cfg(any())]
    #[wasm_bindgen(generic_per_mono)]
    type ShapeConditional<'a, T>;

    #[wasm_bindgen(method, generic_per_mono)]
    fn shape_conditional<T>(this: &ShapeConditional<T>);

    #[cfg(any())]
    type Absent<T>;

    #[wasm_bindgen(method)]
    fn absent<T>(this: &Absent<T>);
}

mod qualified {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        pub type ErasedQualified<T: Clone>;

        #[wasm_bindgen(constructor)]
        pub fn erased_new<T>(value: T) -> self::ErasedQualified<T>;

        #[wasm_bindgen(generic_per_mono)]
        pub type Qualified<T: Clone>;

        #[wasm_bindgen(constructor, generic_per_mono)]
        pub fn mono_new<T>(value: T) -> self::Qualified<T>;

        #[wasm_bindgen(
            static_method_of = self::Qualified,
            generic_per_mono,
            js_name = of
        )]
        pub fn qualified_static<T>(value: T) -> self::Qualified<T>;
    }

    pub fn assert_qualified(value1: JsValue, value2: JsValue, value3: JsValue) {
        let _: ErasedQualified<JsValue> = ErasedQualified::<JsValue>::erased_new(value1);
        let _: Qualified<JsValue> = Qualified::<JsValue>::mono_new(value2);
        let _: Qualified<JsValue> = Qualified::<JsValue>::qualified_static(value3);
    }
}

fn assert_regressions<'a>(
    erased_hrtb: &'a ErasedHrtbHolder<'a, JsValue>,
    hrtb: &'a HrtbHolder<'a, JsValue>,
    erased_conditional: &ErasedConditional<JsValue>,
    conditional: &Conditional<JsValue>,
    attr_conditional: &AttrConditional<JsValue>,
) {
    ErasedBounded::<JsValue>::erased_bounded_create(JsValue::NULL);
    let _: ExplicitBounded<String> =
        ExplicitBounded::<JsValue>::explicit_static_class_wins(String::new());
    erased_hrtb.hrtb_erased(JsValue::NULL);
    hrtb.hrtb_mono(JsValue::NULL);
    let _: JsValue = erased_conditional.conditional_erased();
    let _: JsValue = conditional.conditional_mono();
    let _: JsValue = attr_conditional.attr_conditional();
}

fn main() {}
