use js_sys::*;
use wasm_bindgen::JsCast;
#[cfg(not(js_sys_unstable_apis))]
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn get_canonical_locales() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::new();
        locales.push(&"EN-US".into());
        locales.push(&"Fr".into());
        let locales = JsValue::from(locales);
        let canonical_locales = Intl::get_canonical_locales(&locales);
        assert_eq!(canonical_locales.length(), 2);
        canonical_locales.for_each(&mut |l, i, _| {
            if i == 0 {
                assert_eq!(l, "en-US");
            } else {
                assert_eq!(l, "fr");
            }
        });
        let canonical_locales = Intl::get_canonical_locales(&"EN-US".into());
        assert_eq!(canonical_locales.length(), 1);
        canonical_locales.for_each(&mut |l, _, _| {
            assert_eq!(l, "en-US");
        });
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("EN-US"), JsString::from("Fr")];
        let canonical_locales = Intl::get_canonical_locales(&locales).unwrap();
        assert_eq!(canonical_locales.length(), 2);
        canonical_locales.for_each(&mut |l, i, _| {
            if i == 0 {
                assert_eq!(l, "en-US");
            } else {
                assert_eq!(l, "fr");
            }
        });
        let canonical_locales = Intl::get_canonical_locales(&[JsString::from("EN-US")]).unwrap();
        assert_eq!(canonical_locales.length(), 1);
        canonical_locales.for_each(&mut |l, _, _| {
            assert_eq!(l, "en-US");
        });
    }
}

#[wasm_bindgen_test]
fn collator() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();

        let c = Intl::Collator::new(&locales, &opts);
        assert!(c.compare().is_instance_of::<Function>());
        assert!(c.resolved_options().is_instance_of::<Object>());

        let a = Intl::Collator::supported_locales_of(&locales, &opts);
        assert!(a.is_instance_of::<Array>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];

        let c = Intl::Collator::new(&locales, &Default::default()).unwrap();
        // compare is now a direct method, not a getter returning a function
        assert!(c.compare("a", "b") < 0);
        assert!(c.compare("b", "a") > 0);
        assert_eq!(c.compare("a", "a"), 0);
        assert!(c.resolved_options().is_instance_of::<Object>());

        let a = Intl::Collator::supported_locales_of(&locales, &Default::default()).unwrap();
        assert!(a.is_instance_of::<Array<JsString>>());

        // Test with custom options
        let opts = Intl::CollatorOptions::new();
        let c2 = Intl::Collator::new(&locales, &opts).unwrap();
        assert!(c2.resolved_options().is_instance_of::<Object>());
    }
}

#[wasm_bindgen_test]
fn collator_inheritance() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let c = Intl::Collator::new(&locales, &opts);

        assert!(c.is_instance_of::<Intl::Collator>());
        assert!(c.is_instance_of::<Object>());
        let _: &Object = c.as_ref();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let c = Intl::Collator::new(&locales, &Default::default()).unwrap();

        assert!(c.is_instance_of::<Intl::Collator>());
        assert!(c.is_instance_of::<Object>());
        let _: &Object = c.as_ref();
    }
}

#[wasm_bindgen_test]
fn date_time_format() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let epoch = Date::new(&JsValue::from(0));

        let c = Intl::DateTimeFormat::new(&locales, &opts);
        assert!(c.format().is_instance_of::<Function>());
        assert!(c.format_to_parts(&epoch).is_instance_of::<Array>());
        assert!(c.resolved_options().is_instance_of::<Object>());

        let a = Intl::DateTimeFormat::supported_locales_of(&locales, &opts);
        assert!(a.is_instance_of::<Array>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let epoch = Date::new(&0.into());

        let c = Intl::DateTimeFormat::new(&locales, &Default::default()).unwrap();
        // format is now a direct method, not a getter
        let formatted = c.format(&epoch);
        assert!(formatted.length() > 0);
        assert!(c.format_to_parts(&epoch).is_instance_of::<Array<Object>>());
        assert!(c.resolved_options().is_instance_of::<Object>());

        let a = Intl::DateTimeFormat::supported_locales_of(&locales, &Default::default()).unwrap();
        assert!(a.is_instance_of::<Array<JsString>>());

        // Test with custom options
        let opts = Intl::DateTimeFormatOptions::new();
        let c2 = Intl::DateTimeFormat::new(&locales, &opts).unwrap();
        assert!(c2.resolved_options().is_instance_of::<Object>());
    }
}

#[wasm_bindgen_test]
fn date_time_format_inheritance() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let c = Intl::DateTimeFormat::new(&locales, &opts);

        assert!(c.is_instance_of::<Intl::DateTimeFormat>());
        assert!(c.is_instance_of::<Object>());
        let _: &Object = c.as_ref();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let c = Intl::DateTimeFormat::new(&locales, &Default::default()).unwrap();

        assert!(c.is_instance_of::<Intl::DateTimeFormat>());
        assert!(c.is_instance_of::<Object>());
        let _: &Object = c.as_ref();
    }
}

#[wasm_bindgen_test]
fn number_format() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();

        let n = Intl::NumberFormat::new(&locales, &opts);
        assert!(n.format().is_instance_of::<Function>());
        assert!(n.format_to_parts(42.5).is_instance_of::<Array>());
        assert!(n.resolved_options().is_instance_of::<Object>());

        let a = Intl::NumberFormat::supported_locales_of(&locales, &opts);
        assert!(a.is_instance_of::<Array>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];

        let n = Intl::NumberFormat::new(&locales, &Default::default()).unwrap();
        // format is now a direct method accepting JsString
        let formatted = n.format(&JsString::from("42.5"));
        assert!(formatted.length() > 0);
        assert!(n
            .format_to_parts(&JsString::from("42.5"))
            .is_instance_of::<Array<Object>>());
        assert!(n.resolved_options().is_instance_of::<Object>());

        let a = Intl::NumberFormat::supported_locales_of(&locales, &Default::default()).unwrap();
        assert!(a.is_instance_of::<Array<JsString>>());

        // Test with custom options
        let opts = Intl::NumberFormatOptions::new();
        let n2 = Intl::NumberFormat::new(&locales, &opts).unwrap();
        assert!(n2.resolved_options().is_instance_of::<Object>());
    }
}

#[wasm_bindgen_test]
fn number_format_inheritance() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let n = Intl::NumberFormat::new(&locales, &opts);

        assert!(n.is_instance_of::<Intl::NumberFormat>());
        assert!(n.is_instance_of::<Object>());
        let _: &Object = n.as_ref();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let n = Intl::NumberFormat::new(&locales, &Default::default()).unwrap();

        assert!(n.is_instance_of::<Intl::NumberFormat>());
        assert!(n.is_instance_of::<Object>());
        let _: &Object = n.as_ref();
    }
}

#[wasm_bindgen_test]
fn date_time_format_range() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let c = Intl::DateTimeFormat::new(&locales, &opts);

        // Jan 1, 2020 and Jan 15, 2020
        let start = Date::new(&JsValue::from_f64(1577836800000.0));
        let end = Date::new(&JsValue::from_f64(1579046400000.0));

        let range_str = c.format_range(&start, &end).unwrap();
        assert!(range_str.length() > 0);

        let parts = c.format_range_to_parts(&start, &end).unwrap();
        assert!(parts.is_instance_of::<Array>());
        assert!(parts.length() > 0);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::{DateTimeRangeFormatPart, RangeSource};

        let locales = [JsString::from("en-US")];
        let c = Intl::DateTimeFormat::new(&locales, &Default::default()).unwrap();

        let start = Date::new(&wasm_bindgen::JsValue::from_f64(1577836800000.0));
        let end = Date::new(&wasm_bindgen::JsValue::from_f64(1579046400000.0));

        let range_str = c.format_range(&start, &end).unwrap();
        assert!(range_str.length() > 0);

        let parts = c.format_range_to_parts(&start, &end).unwrap();
        assert!(parts.length() > 0);

        // Verify source property exists and is valid
        let first: DateTimeRangeFormatPart = parts.get(0).unwrap().unchecked_into();
        let source = first.source();
        assert!(
            source == RangeSource::StartRange
                || source == RangeSource::EndRange
                || source == RangeSource::Shared
        );
        assert!(first.value().length() > 0);
    }
}

#[wasm_bindgen_test]
fn number_format_range() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let n = Intl::NumberFormat::new(&locales, &opts);

        let range_str = n
            .format_range(&JsString::from("100"), &JsString::from("200"))
            .unwrap();
        assert!(range_str.length() > 0);

        let parts = n
            .format_range_to_parts(&JsString::from("100"), &JsString::from("200"))
            .unwrap();
        assert!(parts.is_instance_of::<Array>());
        assert!(parts.length() > 0);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::{NumberRangeFormatPart, RangeSource};

        let locales = [JsString::from("en-US")];
        let n = Intl::NumberFormat::new(&locales, &Default::default()).unwrap();

        let range_str = n
            .format_range(&JsString::from("100"), &JsString::from("200"))
            .unwrap();
        assert!(range_str.length() > 0);

        let parts = n
            .format_range_to_parts(&JsString::from("100"), &JsString::from("200"))
            .unwrap();
        assert!(parts.length() > 0);

        // Verify source property
        let first: NumberRangeFormatPart = parts.get(0).unwrap().unchecked_into();
        let source = first.source();
        assert!(
            source == RangeSource::StartRange
                || source == RangeSource::EndRange
                || source == RangeSource::Shared
        );
        assert!(first.value().length() > 0);

        // Test BigInt-style string formatting
        let big_range = n
            .format_range(
                &JsString::from("1000000000000000110000"),
                &JsString::from("2000000000000000220000"),
            )
            .unwrap();
        assert!(big_range.length() > 0);
    }
}

#[wasm_bindgen_test]
fn plural_rules() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();

        let r = Intl::PluralRules::new(&locales, &opts);
        assert!(r.resolved_options().is_instance_of::<Object>());
        assert_eq!(r.select(1_f64), "one");

        let a = Intl::PluralRules::supported_locales_of(&locales, &opts);
        assert!(a.is_instance_of::<Array>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];

        let r = Intl::PluralRules::new(&locales, &Default::default()).unwrap();
        assert!(r.resolved_options().is_instance_of::<Object>());
        assert_eq!(r.select(1_f64), Intl::PluralCategory::One);

        let a = Intl::PluralRules::supported_locales_of(&locales, &Default::default()).unwrap();
        assert!(a.is_instance_of::<Array<JsString>>());

        // Test with custom options
        let opts = Intl::PluralRulesOptions::new();
        let r2 = Intl::PluralRules::new(&locales, &opts).unwrap();
        assert!(r2.resolved_options().is_instance_of::<Object>());
    }
}

#[wasm_bindgen_test]
fn plural_rules_inheritance() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let r = Intl::PluralRules::new(&locales, &opts);

        assert!(r.is_instance_of::<Intl::PluralRules>());
        assert!(r.is_instance_of::<Object>());
        let _: &Object = r.as_ref();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let r = Intl::PluralRules::new(&locales, &Default::default()).unwrap();

        assert!(r.is_instance_of::<Intl::PluralRules>());
        assert!(r.is_instance_of::<Object>());
        let _: &Object = r.as_ref();
    }
}

#[wasm_bindgen_test]
fn relative_time_format() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();

        let c = Intl::RelativeTimeFormat::new(&locales, &opts);
        assert!(c.format(1_f64, "seconds").is_string());
        assert!(c
            .format_to_parts(1_f64, "seconds")
            .is_instance_of::<Array>());
        assert!(c.resolved_options().is_instance_of::<Object>());

        assert_eq!(c.format(1_f64, "seconds"), "in 1 second");
        assert_eq!(c.format(1.5, "seconds"), "in 1.5 seconds");
        assert_eq!(c.format(-1.5, "seconds"), "1.5 seconds ago");

        let a = Intl::RelativeTimeFormat::supported_locales_of(&locales, &opts);
        assert!(a.is_instance_of::<Array>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::{
            RelativeTimeFormatNumeric, RelativeTimeFormatOptions, RelativeTimeFormatPart,
            RelativeTimeFormatPartType, RelativeTimeFormatStyle, RelativeTimeFormatUnit,
        };

        let locales = [JsString::from("en-US")];

        let c = Intl::RelativeTimeFormat::new(&locales).unwrap();
        assert!(c.format(1_f64, RelativeTimeFormatUnit::Second).is_string());
        assert!(c
            .format_to_parts(1_f64, RelativeTimeFormatUnit::Second)
            .is_instance_of::<Array<RelativeTimeFormatPart>>());
        // resolved_options returns a plain Object with typed accessors
        assert!(c.resolved_options().is_instance_of::<Object>());

        assert_eq!(
            c.format(1_f64, RelativeTimeFormatUnit::Second),
            "in 1 second"
        );
        assert_eq!(
            c.format(1.5, RelativeTimeFormatUnit::Seconds),
            "in 1.5 seconds"
        );
        assert_eq!(
            c.format(-1.5, RelativeTimeFormatUnit::Seconds),
            "1.5 seconds ago"
        );

        let locale_opts = Intl::LocaleMatcherOptions::new();
        let a = Intl::RelativeTimeFormat::supported_locales_of(&locales, &locale_opts).unwrap();
        assert!(a.is_instance_of::<Array<JsString>>());

        // Test with typed options
        let opts = RelativeTimeFormatOptions::new();
        opts.set_numeric(RelativeTimeFormatNumeric::Auto);
        opts.set_style(RelativeTimeFormatStyle::Long);
        let c2 = Intl::RelativeTimeFormat::new_with_options(&locales, &opts).unwrap();
        // resolved_options returns a plain Object with typed accessors
        assert!(c2.resolved_options().is_instance_of::<Object>());

        // Check resolved options have expected values
        let resolved = c2.resolved_options();
        assert_eq!(
            resolved.get_numeric(),
            Some(RelativeTimeFormatNumeric::Auto)
        );
        assert_eq!(resolved.get_style(), Some(RelativeTimeFormatStyle::Long));
        assert!(resolved.get_locale().length() > 0);

        // Test formatToParts returns typed parts
        let parts = c.format_to_parts(1_f64, RelativeTimeFormatUnit::Day);
        assert!(parts.length() > 0);
        let first_part: RelativeTimeFormatPart = parts.get(0).unwrap().unchecked_into();
        // First part should be "literal" with value "in "
        assert_eq!(first_part.type_(), RelativeTimeFormatPartType::Literal);
        assert!(first_part.value().length() > 0);

        // Test different units
        assert!(c
            .format(2_f64, RelativeTimeFormatUnit::Years)
            .includes("2", 0));
        assert!(c
            .format(3_f64, RelativeTimeFormatUnit::Months)
            .includes("3", 0));
        assert!(c
            .format(4_f64, RelativeTimeFormatUnit::Weeks)
            .includes("4", 0));
        assert!(c
            .format(5_f64, RelativeTimeFormatUnit::Days)
            .includes("5", 0));
        assert!(c
            .format(6_f64, RelativeTimeFormatUnit::Hours)
            .includes("6", 0));
        assert!(c
            .format(7_f64, RelativeTimeFormatUnit::Minutes)
            .includes("7", 0));
    }
}

#[wasm_bindgen_test]
fn relative_time_format_inheritance() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let c = Intl::RelativeTimeFormat::new(&locales, &opts);

        assert!(c.is_instance_of::<Intl::RelativeTimeFormat>());
        assert!(c.is_instance_of::<Object>());
        let _: &Object = c.as_ref();
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let c = Intl::RelativeTimeFormat::new(&locales).unwrap();

        assert!(c.is_instance_of::<Intl::RelativeTimeFormat>());
        assert!(c.is_instance_of::<Object>());
        let _: &Object = c.as_ref();
    }
}

#[wasm_bindgen_test]
fn supported_values_of() {
    use Intl::SupportedValuesKey;

    // Test that we can get supported calendars
    let calendars = Intl::supported_values_of(SupportedValuesKey::Calendar);
    assert!(calendars.length() > 0);

    // Test that we can get supported currencies
    let currencies = Intl::supported_values_of(SupportedValuesKey::Currency);
    assert!(currencies.length() > 0);

    // Test time zones
    let time_zones = Intl::supported_values_of(SupportedValuesKey::TimeZone);
    assert!(time_zones.length() > 0);
}

#[wasm_bindgen_test]
fn list_format() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        let list = Array::of(&[
            JsValue::from("Apple"),
            JsValue::from("Banana"),
            JsValue::from("Orange"),
        ]);

        let lf = Intl::ListFormat::new(&locales, &opts);
        let result = lf.format(&list);
        assert!(result.length() > 0);
        assert!(lf.format_to_parts(&list).is_instance_of::<Array>());
        assert!(lf.resolved_options().is_instance_of::<Object>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::{ListFormatOptions, ListFormatPart, ListFormatStyle, ListFormatType};

        let locales = [JsString::from("en-US")];
        let list = [
            JsString::from("Apple"),
            JsString::from("Banana"),
            JsString::from("Orange"),
        ];

        let lf = Intl::ListFormat::new(&locales, &Default::default()).unwrap();
        let result = lf.format(&list);
        assert!(result.includes("Apple", 0));
        assert!(result.includes("Banana", 0));
        assert!(result.includes("Orange", 0));

        // Test formatToParts
        let parts = lf.format_to_parts(&list);
        assert!(parts.length() > 0);
        let first: ListFormatPart = parts.get(0).unwrap().unchecked_into();
        assert!(first.value().length() > 0);

        // Test with options
        let opts = ListFormatOptions::new();
        opts.set_type(ListFormatType::Disjunction);
        opts.set_style(ListFormatStyle::Long);
        let lf2 = Intl::ListFormat::new(&locales, &opts).unwrap();
        let result2 = lf2.format(&list);
        // Disjunction should contain "or"
        assert!(result2.includes("or", 0));
    }
}

#[wasm_bindgen_test]
fn segmenter() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();

        let seg = Intl::Segmenter::new(&locales, &opts);
        let segments = seg.segment("Hello, world!");
        assert!(segments.is_instance_of::<Object>());
        assert!(seg.resolved_options().is_instance_of::<Object>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::{SegmenterGranularity, SegmenterOptions};

        let locales = [JsString::from("en-US")];

        // Test word segmentation
        let opts = SegmenterOptions::new();
        opts.set_granularity(SegmenterGranularity::Word);
        let seg = Intl::Segmenter::new(&locales, &opts).unwrap();
        let segments = seg.segment("Hello, world!");

        // Test containing method
        let data = segments.containing(0).unwrap();
        assert_eq!(data.segment(), "Hello");
        assert_eq!(data.index(), 0);
        assert_eq!(data.is_word_like(), Some(true));

        // Test resolved options
        let resolved = seg.resolved_options();
        assert_eq!(resolved.get_granularity(), Some(SegmenterGranularity::Word));

        // Test grapheme segmentation
        let opts2 = SegmenterOptions::new();
        opts2.set_granularity(SegmenterGranularity::Grapheme);
        let seg2 = Intl::Segmenter::new(&locales, &opts2).unwrap();
        let segments2 = seg2.segment("👨‍👩‍👧‍👦");
        let data2 = segments2.containing(0).unwrap();
        // Family emoji is a single grapheme cluster
        assert!(data2.segment().length() > 0);
    }
}

#[wasm_bindgen_test]
fn display_names() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();
        Reflect::set(&opts, &"type".into(), &"language".into()).unwrap();

        let dn = Intl::DisplayNames::new(&locales, &opts);
        let name = dn.of("en-US");
        assert!(name.is_some());
        assert!(dn.resolved_options().is_instance_of::<Object>());
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::{DisplayNamesOptions, DisplayNamesType};

        let locales = [JsString::from("en-US")];

        // Test language names
        let opts = DisplayNamesOptions::new();
        opts.set_type(DisplayNamesType::Language);
        let dn = Intl::DisplayNames::new(&locales, &opts).unwrap();
        let name = dn.of("fr");
        assert!(name.is_some());
        assert!(name.unwrap().includes("French", 0));

        // Test region names
        let opts2 = DisplayNamesOptions::new();
        opts2.set_type(DisplayNamesType::Region);
        let dn2 = Intl::DisplayNames::new(&locales, &opts2).unwrap();
        let region = dn2.of("US");
        assert!(region.is_some());
        assert!(region.unwrap().includes("United States", 0));

        // Test currency names
        let opts3 = DisplayNamesOptions::new();
        opts3.set_type(DisplayNamesType::Currency);
        let dn3 = Intl::DisplayNames::new(&locales, &opts3).unwrap();
        let currency = dn3.of("USD");
        assert!(currency.is_some());
    }
}

#[wasm_bindgen_test]
fn locale() {
    let locale = Intl::Locale::new("en-US").unwrap();
    assert_eq!(locale.language(), "en");
    assert_eq!(locale.region(), Some(JsString::from("US")));
    assert!(locale.base_name().length() > 0);

    // Test maximize/minimize
    let minimal = Intl::Locale::new("en").unwrap();
    let maximized = minimal.maximize();
    assert!(maximized.script().is_some());
    assert!(maximized.region().is_some());

    let minimized = maximized.minimize();
    assert_eq!(minimized.language(), "en");

    // Test getCalendars, etc. (not available in all environments)
    // Check if getCalendars method exists on the locale object
    if Reflect::get_str(&locale, &"getCalendars".into())
        .unwrap()
        .map(|v| v.is_function())
        .unwrap_or(false)
    {
        let calendars = locale.get_calendars();
        assert!(calendars.length() > 0);

        let numbering = locale.get_numbering_systems();
        assert!(numbering.length() > 0);
    }
}

#[wasm_bindgen_test]
fn plural_rules_select_range() {
    #[cfg(not(js_sys_unstable_apis))]
    {
        let locales = Array::of(&[JsValue::from("en-US")]);
        let opts = Object::new();

        let pr = Intl::PluralRules::new(&locales, &opts);
        let result = pr.select_range(1.0, 5.0);
        assert!(result.length() > 0);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        use Intl::PluralCategory;

        let locales = [JsString::from("en-US")];
        let pr = Intl::PluralRules::new(&locales, &Default::default()).unwrap();

        let result = pr.select_range(1.0, 5.0);
        assert!(result == PluralCategory::Other || result == PluralCategory::One);
    }
}

#[wasm_bindgen_test]
fn collator_compare_direct() {
    #[cfg(js_sys_unstable_apis)]
    {
        let locales = [JsString::from("en-US")];
        let collator = Intl::Collator::new(&locales, &Default::default()).unwrap();

        // Test direct comparison
        assert!(collator.compare("a", "b") < 0);
        assert!(collator.compare("b", "a") > 0);
        assert_eq!(collator.compare("a", "a"), 0);

        // Test locale-aware comparison
        let de_locales = [JsString::from("de")];
        let de_collator = Intl::Collator::new(&de_locales, &Default::default()).unwrap();
        // In German, ä sorts with a
        assert!(de_collator.compare("ä", "z") < 0);
    }
}

#[wasm_bindgen_test]
fn duration_format() {
    use Intl::{Duration, DurationFormat, DurationFormatOptions, DurationFormatStyle};

    // Check if Intl.DurationFormat exists (not available in all environments)
    let intl_val = Reflect::get_str(js_sys::global().as_ref(), &"Intl".into())
        .unwrap()
        .unwrap();
    let intl = Object::try_from(&intl_val).unwrap();
    if Reflect::get_str(intl, &"DurationFormat".into())
        .unwrap()
        .is_none()
    {
        return;
    }

    let locales = [JsString::from("en-US")];

    // Create a duration
    let duration = Duration::new();
    duration.set_hours(1.0);
    duration.set_minutes(46.0);
    duration.set_seconds(40.0);

    // Test with default options
    let df = DurationFormat::new(&locales, &Default::default()).unwrap();
    let result = df.format(&duration);
    assert!(result.length() > 0);
    assert!(result.includes("1", 0)); // Should contain "1" for hours

    // Test with long style
    let opts = DurationFormatOptions::new();
    opts.set_style(DurationFormatStyle::Long);
    let df_long = DurationFormat::new(&locales, &opts).unwrap();
    let result_long = df_long.format(&duration);
    assert!(result_long.includes("hour", 0)); // Long style should have "hour"

    // Test with short style
    let opts_short = DurationFormatOptions::new();
    opts_short.set_style(DurationFormatStyle::Short);
    let df_short = DurationFormat::new(&locales, &opts_short).unwrap();
    let result_short = df_short.format(&duration);
    assert!(result_short.length() > 0);

    // Test formatToParts
    let parts = df.format_to_parts(&duration);
    assert!(parts.length() > 0);

    // Test resolvedOptions
    let resolved = df_long.resolved_options();
    assert!(resolved.get_locale().length() > 0);
    assert_eq!(resolved.get_style(), Some(DurationFormatStyle::Long));

    // Test supportedLocalesOf
    let supported = DurationFormat::supported_locales_of(&locales, &Default::default()).unwrap();
    assert!(supported.length() > 0);

    // Test with French locale
    let fr_locales = [JsString::from("fr-FR")];
    let df_fr = DurationFormat::new(&fr_locales, &opts).unwrap();
    let result_fr = df_fr.format(&duration);
    assert!(result_fr.includes("heure", 0)); // French for "hour"
}
