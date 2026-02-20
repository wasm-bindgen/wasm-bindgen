use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/Temporal.js")]
extern "C" {
    fn is_temporal_supported() -> bool;
    fn get_test_instant() -> Temporal::Instant;
    fn get_test_zoned_date_time() -> Temporal::ZonedDateTime;
    fn get_test_plain_date_time() -> Temporal::PlainDateTime;
    fn get_test_plain_date() -> Temporal::PlainDate;
    fn get_test_plain_time() -> Temporal::PlainTime;
    fn get_test_plain_year_month() -> Temporal::PlainYearMonth;
    fn get_test_plain_month_day() -> Temporal::PlainMonthDay;
    fn get_test_duration() -> Temporal::Duration;
    fn create_day_object(day: u32) -> JsValue;
    fn create_year_object(year: i32) -> JsValue;
}

// ============================================================================
// Temporal.Now tests
// ============================================================================

#[wasm_bindgen_test]
fn now_instant() {
    if !is_temporal_supported() {
        return;
    }
    let instant = Temporal::Now::instant();
    assert!(instant.epoch_milliseconds() > 0.0);
}

#[wasm_bindgen_test]
fn now_time_zone_id() {
    if !is_temporal_supported() {
        return;
    }
    let tz_id = Temporal::Now::time_zone_id();
    assert!(tz_id.length() > 0);
}

#[wasm_bindgen_test]
fn now_zoned_date_time_iso() {
    if !is_temporal_supported() {
        return;
    }
    // Test no-arg version (uses system timezone)
    let zdt = Temporal::Now::zoned_date_time_iso();
    assert!(zdt.year() > 2020);

    // Test with timezone string
    let zdt_utc = Temporal::Now::zoned_date_time_iso_with_timezone_str("UTC").unwrap();
    assert!(zdt_utc.year() > 2020);
    assert_eq!(zdt_utc.time_zone_id(), "UTC");

    // Test with ZonedDateTime as timezone source
    let zdt_from_zdt = Temporal::Now::zoned_date_time_iso_with_timezone(&zdt);
    assert!(zdt_from_zdt.year() > 2020);
}

#[wasm_bindgen_test]
fn now_plain_date_time_iso() {
    if !is_temporal_supported() {
        return;
    }
    // Test no-arg version
    let pdt = Temporal::Now::plain_date_time_iso();
    assert!(pdt.year() > 2020);

    // Test with timezone string
    let pdt_utc = Temporal::Now::plain_date_time_iso_with_timezone_str("UTC").unwrap();
    assert!(pdt_utc.year() > 2020);
}

#[wasm_bindgen_test]
fn now_plain_date_iso() {
    if !is_temporal_supported() {
        return;
    }
    // Test no-arg version
    let pd = Temporal::Now::plain_date_iso();
    assert!(pd.year() > 2020);

    // Test with timezone string
    let pd_utc = Temporal::Now::plain_date_iso_with_timezone_str("UTC").unwrap();
    assert!(pd_utc.year() > 2020);
}

#[wasm_bindgen_test]
fn now_plain_time_iso() {
    if !is_temporal_supported() {
        return;
    }
    // Test no-arg version
    let pt = Temporal::Now::plain_time_iso();
    assert!(pt.hour() <= 23);

    // Test with timezone string
    let pt_utc = Temporal::Now::plain_time_iso_with_timezone_str("UTC").unwrap();
    assert!(pt_utc.hour() <= 23);
}

// ============================================================================
// Temporal.Instant tests
// ============================================================================

#[wasm_bindgen_test]
fn instant_constructor_and_static_methods() {
    if !is_temporal_supported() {
        return;
    }
    // Test fromEpochMilliseconds
    let instant = Temporal::Instant::from_epoch_milliseconds(1000000000000.0).unwrap();
    assert_eq!(instant.epoch_milliseconds(), 1000000000000.0);

    // Test from with string
    let instant2 = Temporal::Instant::from(&"2020-01-01T00:00:00Z".into()).unwrap();
    assert!(instant2.epoch_milliseconds() > 0.0);

    // Test compare
    let cmp = Temporal::Instant::compare(&instant.into(), &instant2.into()).unwrap();
    assert!(cmp == -1 || cmp == 0 || cmp == 1);
}

#[wasm_bindgen_test]
fn instant_methods() {
    if !is_temporal_supported() {
        return;
    }
    let instant = get_test_instant();

    // Test equals
    let instant2 = Temporal::Instant::from(
        &instant
            .to_js_string(&Temporal::InstantToStringOptions::new())
            .into(),
    )
    .unwrap();
    assert!(instant.equals(&instant2.into()).unwrap());

    // Test add/subtract
    let later = instant.add(&JsValue::from_str("PT1H")).unwrap();
    assert!(later.epoch_milliseconds() > instant.epoch_milliseconds());

    let earlier = instant.subtract(&JsValue::from_str("PT1H")).unwrap();
    assert!(earlier.epoch_milliseconds() < instant.epoch_milliseconds());

    // Test until/since with typed options
    let diff_opts = Temporal::DifferenceOptions::new();
    let duration = instant.until(&later.clone().into(), &diff_opts).unwrap();
    assert_eq!(duration.hours(), 1.0);

    let duration2 = later
        .since(&instant.clone().into(), &Temporal::DifferenceOptions::new())
        .unwrap();
    assert_eq!(duration2.hours(), 1.0);

    // Test round with typed options
    let round_opts = Temporal::RoundToOptions::new();
    round_opts.set_smallest_unit(Temporal::SmallestUnit::Hour);
    let rounded = instant.round(&round_opts).unwrap();
    // Instant rounded to hour should have milliseconds as 0
    assert_eq!(rounded.epoch_milliseconds() % 3600000.0, 0.0);

    // Test toZonedDateTimeISO with string
    let zdt = instant
        .to_zoned_date_time_iso_with_timezone_str("UTC")
        .unwrap();
    assert!(zdt.year() > 0);

    // Test toZonedDateTimeISO with ZonedDateTime
    let zdt2 = instant.to_zoned_date_time_iso_with_timezone(&zdt);
    assert!(zdt2.year() > 0);

    // Test toJSON and toString
    let json = instant.to_json();
    assert!(json.length() > 0);

    let str = instant.to_js_string(&Temporal::InstantToStringOptions::new());
    assert!(str.length() > 0);
}

// ============================================================================
// Temporal.ZonedDateTime tests
// ============================================================================

#[wasm_bindgen_test]
fn zoned_date_time_properties() {
    if !is_temporal_supported() {
        return;
    }
    let zdt = get_test_zoned_date_time();

    // Test basic date/time properties
    assert!(zdt.year() != 0);
    assert!(zdt.month() >= 1 && zdt.month() <= 12);
    assert!(zdt.day() >= 1 && zdt.day() <= 31);
    assert!(zdt.hour() <= 23);
    assert!(zdt.minute() <= 59);
    assert!(zdt.second() <= 59);
    assert!(zdt.millisecond() <= 999);
    assert!(zdt.microsecond() <= 999);
    assert!(zdt.nanosecond() <= 999);

    // Test calendar/timezone properties
    assert!(zdt.time_zone_id().length() > 0);
    assert!(zdt.calendar_id().length() > 0);
    assert!(zdt.month_code().length() > 0);

    // Test computed properties
    assert!(zdt.day_of_week() >= 1 && zdt.day_of_week() <= 7);
    assert!(zdt.day_of_year() >= 1 && zdt.day_of_year() <= 366);
    assert!(zdt.days_in_week() > 0);
    assert!(zdt.days_in_month() >= 28 && zdt.days_in_month() <= 31);
    assert!(zdt.days_in_year() >= 365 && zdt.days_in_year() <= 366);
    assert!(zdt.months_in_year() >= 12);
    assert!(zdt.hours_in_day() > 0.0);

    // Test offset
    assert!(zdt.offset().length() > 0);
    assert!(zdt.epoch_milliseconds() != 0.0);
}

#[wasm_bindgen_test]
fn zoned_date_time_methods() {
    if !is_temporal_supported() {
        return;
    }
    let zdt = get_test_zoned_date_time();

    // Test equals
    let zdt2 = Temporal::ZonedDateTime::from(
        &zdt.to_js_string(&Temporal::ZonedDateTimeToStringOptions::new())
            .into(),
        &Temporal::ZonedDateTimeAssignmentOptions::new(),
    )
    .unwrap();
    assert!(zdt.equals(&zdt2.into()).unwrap());

    // Test withCalendar
    let with_calendar = zdt.with_calendar("iso8601").unwrap();
    assert_eq!(with_calendar.calendar_id(), "iso8601");

    // Test add/subtract with typed options
    let later = zdt
        .add(
            &JsValue::from_str("P1D"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    let earlier = zdt
        .subtract(
            &JsValue::from_str("P1D"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    assert!(later.day() != earlier.day() || later.month() != earlier.month());

    // Test startOfDay
    let start = zdt.start_of_day();
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);

    // Test conversions
    let instant = zdt.to_instant();
    assert!(instant.epoch_milliseconds() > 0.0);

    let pdt = zdt.to_plain_date_time();
    assert_eq!(pdt.year(), zdt.year());

    let pd = zdt.to_plain_date();
    assert_eq!(pd.year(), zdt.year());

    let pt = zdt.to_plain_time();
    assert_eq!(pt.hour(), zdt.hour());

    // Test toPlainYearMonth and toPlainMonthDay
    let pym = zdt.to_plain_year_month();
    assert_eq!(pym.year(), zdt.year());
    assert_eq!(pym.month(), zdt.month());

    let pmd = zdt.to_plain_month_day();
    assert_eq!(pmd.day(), zdt.day());

    // Test withTimeZone with string
    let zdt_utc = zdt.with_time_zone_str("UTC").unwrap();
    assert_eq!(zdt_utc.time_zone_id(), "UTC");

    // Test withTimeZone with ZonedDateTime (infallible when using valid ZonedDateTime)
    let zdt_from_zdt = zdt.with_time_zone(&zdt_utc);
    assert_eq!(zdt_from_zdt.time_zone_id(), "UTC");

    // Test toString/toJSON with typed options
    let str = zdt.to_js_string(&Temporal::ZonedDateTimeToStringOptions::new());
    assert!(str.length() > 0);

    let json = zdt.to_json();
    assert!(json.length() > 0);
}

// ============================================================================
// Temporal.PlainDateTime tests
// ============================================================================

#[wasm_bindgen_test]
fn plain_date_time_properties() {
    if !is_temporal_supported() {
        return;
    }
    let pdt = get_test_plain_date_time();

    // Test basic properties
    assert!(pdt.year() != 0);
    assert!(pdt.month() >= 1 && pdt.month() <= 12);
    assert!(pdt.day() >= 1 && pdt.day() <= 31);
    assert!(pdt.hour() <= 23);
    assert!(pdt.minute() <= 59);
    assert!(pdt.second() <= 59);
    assert!(pdt.millisecond() <= 999);
    assert!(pdt.microsecond() <= 999);
    assert!(pdt.nanosecond() <= 999);

    // Test calendar properties
    assert!(pdt.calendar_id().length() > 0);
    assert!(pdt.month_code().length() > 0);

    // Test computed properties
    assert!(pdt.day_of_week() >= 1 && pdt.day_of_week() <= 7);
    assert!(pdt.day_of_year() >= 1 && pdt.day_of_year() <= 366);
    assert!(pdt.days_in_week() > 0);
    assert!(pdt.days_in_month() >= 28);
    assert!(pdt.days_in_year() >= 365);
    assert!(pdt.months_in_year() >= 12);
}

#[wasm_bindgen_test]
fn plain_date_time_methods() {
    if !is_temporal_supported() {
        return;
    }
    let pdt = get_test_plain_date_time();

    // Test equals
    let pdt2 = Temporal::PlainDateTime::from(
        &pdt.to_js_string(&Temporal::CalendarTypeToStringOptions::new())
            .into(),
        &Temporal::AssignmentOptions::new(),
    )
    .unwrap();
    assert!(pdt.equals(&pdt2.into()).unwrap());

    // Test add/subtract with typed options
    let later = pdt
        .add(
            &JsValue::from_str("P1D"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    let earlier = pdt
        .subtract(
            &JsValue::from_str("P1D"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    assert!(later.day() != earlier.day() || later.month() != earlier.month());

    // Test round with typed options
    let round_opts = Temporal::RoundToOptions::new();
    round_opts.set_smallest_unit(Temporal::SmallestUnit::Hour);
    let rounded = pdt.round(&round_opts).unwrap();
    assert_eq!(rounded.minute(), 0);
    assert_eq!(rounded.second(), 0);

    // Test conversions
    let pd = pdt.to_plain_date();
    assert_eq!(pd.year(), pdt.year());

    let pt = pdt.to_plain_time();
    assert_eq!(pt.hour(), pdt.hour());

    // Test withPlainTime
    let with_midnight = pdt.with_plain_time(&JsValue::UNDEFINED).unwrap();
    assert_eq!(with_midnight.hour(), 0);

    // Test toPlainYearMonth and toPlainMonthDay
    let pym = pdt.to_plain_year_month();
    assert_eq!(pym.year(), pdt.year());
    assert_eq!(pym.month(), pdt.month());

    let pmd = pdt.to_plain_month_day();
    assert_eq!(pmd.day(), pdt.day());

    // Test toZonedDateTime with timezone string
    let zdt = pdt
        .to_zoned_date_time_with_timezone_str("UTC", &Temporal::ToInstantOptions::new())
        .unwrap();
    assert_eq!(zdt.year(), pdt.year());
    assert_eq!(zdt.time_zone_id(), "UTC");

    // Test toZonedDateTime with ZonedDateTime as timezone source
    let zdt2 = pdt
        .to_zoned_date_time_with_timezone(&zdt, &Temporal::ToInstantOptions::new())
        .unwrap();
    assert_eq!(zdt2.time_zone_id(), "UTC");

    // Test toString/toJSON with typed options
    let str = pdt.to_js_string(&Temporal::CalendarTypeToStringOptions::new());
    assert!(str.length() > 0);

    let json = pdt.to_json();
    assert!(json.length() > 0);
}

// ============================================================================
// Temporal.PlainDate tests
// ============================================================================

#[wasm_bindgen_test]
fn plain_date_properties() {
    if !is_temporal_supported() {
        return;
    }
    let pd = get_test_plain_date();

    // Test basic properties
    assert!(pd.year() != 0);
    assert!(pd.month() >= 1 && pd.month() <= 12);
    assert!(pd.day() >= 1 && pd.day() <= 31);

    // Test calendar properties
    assert!(pd.calendar_id().length() > 0);
    assert!(pd.month_code().length() > 0);

    // Test computed properties
    assert!(pd.day_of_week() >= 1 && pd.day_of_week() <= 7);
    assert!(pd.day_of_year() >= 1 && pd.day_of_year() <= 366);
    assert!(pd.days_in_week() > 0);
    assert!(pd.days_in_month() >= 28);
    assert!(pd.days_in_year() >= 365);
    assert!(pd.months_in_year() >= 12);
}

#[wasm_bindgen_test]
fn plain_date_methods() {
    if !is_temporal_supported() {
        return;
    }
    let pd = get_test_plain_date();

    // Test equals
    let pd2 = Temporal::PlainDate::from(
        &pd.to_js_string(&Temporal::ShowCalendarOptions::new())
            .into(),
        &Temporal::AssignmentOptions::new(),
    )
    .unwrap();
    assert!(pd.equals(&pd2.into()).unwrap());

    // Test add/subtract with typed options
    let later = pd
        .add(
            &JsValue::from_str("P1M"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    let earlier = pd
        .subtract(
            &JsValue::from_str("P1M"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    assert!(later.month() != earlier.month() || later.year() != earlier.year());

    // Test until/since with typed options
    let duration = pd
        .until(&later.clone().into(), &Temporal::DifferenceOptions::new())
        .unwrap();
    assert_eq!(duration.months(), 1.0);

    let duration2 = later
        .since(&pd.clone().into(), &Temporal::DifferenceOptions::new())
        .unwrap();
    assert_eq!(duration2.months(), 1.0);

    // Test conversions
    let pdt = pd.to_plain_date_time(&JsValue::UNDEFINED).unwrap();
    assert_eq!(pdt.year(), pd.year());
    assert_eq!(pdt.hour(), 0); // Default to midnight

    let pym = pd.to_plain_year_month();
    assert_eq!(pym.year(), pd.year());
    assert_eq!(pym.month(), pd.month());

    let pmd = pd.to_plain_month_day();
    assert_eq!(pmd.day(), pd.day());

    // Test withCalendar
    let with_calendar = pd.with_calendar("iso8601").unwrap();
    assert_eq!(with_calendar.calendar_id(), "iso8601");

    // Test toString/toJSON with typed options
    let str = pd.to_js_string(&Temporal::ShowCalendarOptions::new());
    assert!(str.length() > 0);

    let json = pd.to_json();
    assert!(json.length() > 0);
}

#[wasm_bindgen_test]
fn plain_date_compare() {
    if !is_temporal_supported() {
        return;
    }
    let pd1 = Temporal::PlainDate::from(&"2022-01-01".into(), &Temporal::AssignmentOptions::new())
        .unwrap();
    let pd2 = Temporal::PlainDate::from(&"2022-06-15".into(), &Temporal::AssignmentOptions::new())
        .unwrap();

    assert_eq!(
        Temporal::PlainDate::compare(&pd1.clone().into(), &pd2.clone().into()).unwrap(),
        -1
    );
    assert_eq!(
        Temporal::PlainDate::compare(&pd2.clone().into(), &pd1.clone().into()).unwrap(),
        1
    );
    assert_eq!(
        Temporal::PlainDate::compare(&pd1.clone().into(), &pd1.into()).unwrap(),
        0
    );
}

// ============================================================================
// Temporal.PlainTime tests
// ============================================================================

#[wasm_bindgen_test]
fn plain_time_properties() {
    if !is_temporal_supported() {
        return;
    }
    let pt = get_test_plain_time();

    assert!(pt.hour() <= 23);
    assert!(pt.minute() <= 59);
    assert!(pt.second() <= 59);
    assert!(pt.millisecond() <= 999);
    assert!(pt.microsecond() <= 999);
    assert!(pt.nanosecond() <= 999);
}

#[wasm_bindgen_test]
fn plain_time_methods() {
    if !is_temporal_supported() {
        return;
    }
    let pt = get_test_plain_time();

    // Test equals
    let pt2 = Temporal::PlainTime::from(
        &pt.to_js_string(&Temporal::ToStringPrecisionOptions::new())
            .into(),
        &Temporal::AssignmentOptions::new(),
    )
    .unwrap();
    assert!(pt.equals(&pt2.into()).unwrap());

    // Test add/subtract (wraps around midnight)
    let _later = pt.add(&JsValue::from_str("PT1H")).unwrap();
    let _earlier = pt.subtract(&JsValue::from_str("PT1H")).unwrap();

    // Test until/since with typed options
    let pt_start =
        Temporal::PlainTime::from(&"10:00:00".into(), &Temporal::AssignmentOptions::new()).unwrap();
    let pt_end =
        Temporal::PlainTime::from(&"12:30:00".into(), &Temporal::AssignmentOptions::new()).unwrap();
    let duration = pt_start
        .until(&pt_end.into(), &Temporal::DifferenceOptions::new())
        .unwrap();
    assert_eq!(duration.hours(), 2.0);
    assert_eq!(duration.minutes(), 30.0);

    // Test round with typed options
    let round_opts = Temporal::RoundToOptions::new();
    round_opts.set_smallest_unit(Temporal::SmallestUnit::Hour);
    let rounded = pt.round(&round_opts).unwrap();
    assert_eq!(rounded.minute(), 0);
    assert_eq!(rounded.second(), 0);

    // Test toPlainDateTime
    let pd = Temporal::PlainDate::from(&"2024-06-15".into(), &Temporal::AssignmentOptions::new())
        .unwrap();
    let pdt = pt.to_plain_date_time(&pd.into()).unwrap();
    assert_eq!(pdt.year(), 2024);
    assert_eq!(pdt.month(), 6);
    assert_eq!(pdt.day(), 15);
    assert_eq!(pdt.hour(), pt.hour());
    assert_eq!(pdt.minute(), pt.minute());

    // Test toString/toJSON with typed options
    let str = pt.to_js_string(&Temporal::ToStringPrecisionOptions::new());
    assert!(str.length() > 0);

    let json = pt.to_json();
    assert!(json.length() > 0);
}

#[wasm_bindgen_test]
fn plain_time_compare() {
    if !is_temporal_supported() {
        return;
    }
    let pt1 =
        Temporal::PlainTime::from(&"10:00:00".into(), &Temporal::AssignmentOptions::new()).unwrap();
    let pt2 =
        Temporal::PlainTime::from(&"14:30:00".into(), &Temporal::AssignmentOptions::new()).unwrap();

    assert_eq!(
        Temporal::PlainTime::compare(&pt1.clone().into(), &pt2.clone().into()).unwrap(),
        -1
    );
    assert_eq!(
        Temporal::PlainTime::compare(&pt2.clone().into(), &pt1.clone().into()).unwrap(),
        1
    );
    assert_eq!(
        Temporal::PlainTime::compare(&pt1.clone().into(), &pt1.into()).unwrap(),
        0
    );
}

// ============================================================================
// Temporal.PlainYearMonth tests
// ============================================================================

#[wasm_bindgen_test]
fn plain_year_month_properties() {
    if !is_temporal_supported() {
        return;
    }
    let pym = get_test_plain_year_month();

    assert!(pym.year() != 0);
    assert!(pym.month() >= 1 && pym.month() <= 12);
    assert!(pym.calendar_id().length() > 0);
    assert!(pym.month_code().length() > 0);
    assert!(pym.days_in_month() >= 28);
    assert!(pym.days_in_year() >= 365);
    assert!(pym.months_in_year() >= 12);
}

#[wasm_bindgen_test]
fn plain_year_month_methods() {
    if !is_temporal_supported() {
        return;
    }
    let pym = get_test_plain_year_month();

    // Test equals
    let pym2 = Temporal::PlainYearMonth::from(
        &pym.to_js_string(&Temporal::ShowCalendarOptions::new())
            .into(),
        &Temporal::AssignmentOptions::new(),
    )
    .unwrap();
    assert!(pym.equals(&pym2.into()).unwrap());

    // Test add/subtract with typed options
    let later = pym
        .add(
            &JsValue::from_str("P3M"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    let earlier = pym
        .subtract(
            &JsValue::from_str("P3M"),
            &Temporal::ArithmeticOptions::new(),
        )
        .unwrap();
    assert!(later.month() != earlier.month() || later.year() != earlier.year());

    // Test until/since with typed options
    let duration = pym
        .until(&later.into(), &Temporal::DifferenceOptions::new())
        .unwrap();
    assert_eq!(duration.months(), 3.0);

    // Test toPlainDate
    let pd = pym.to_plain_date(&create_day_object(15)).unwrap();
    assert_eq!(pd.year(), pym.year());
    assert_eq!(pd.month(), pym.month());

    // Test toString/toJSON with typed options
    let str = pym.to_js_string(&Temporal::ShowCalendarOptions::new());
    assert!(str.length() > 0);

    let json = pym.to_json();
    assert!(json.length() > 0);
}

#[wasm_bindgen_test]
fn plain_year_month_compare() {
    if !is_temporal_supported() {
        return;
    }
    let pym1 =
        Temporal::PlainYearMonth::from(&"2022-01".into(), &Temporal::AssignmentOptions::new())
            .unwrap();
    let pym2 =
        Temporal::PlainYearMonth::from(&"2022-06".into(), &Temporal::AssignmentOptions::new())
            .unwrap();

    assert_eq!(
        Temporal::PlainYearMonth::compare(&pym1.clone().into(), &pym2.clone().into()).unwrap(),
        -1
    );
    assert_eq!(
        Temporal::PlainYearMonth::compare(&pym2.clone().into(), &pym1.clone().into()).unwrap(),
        1
    );
    assert_eq!(
        Temporal::PlainYearMonth::compare(&pym1.clone().into(), &pym1.into()).unwrap(),
        0
    );
}

// ============================================================================
// Temporal.PlainMonthDay tests
// ============================================================================

#[wasm_bindgen_test]
fn plain_month_day_properties() {
    if !is_temporal_supported() {
        return;
    }
    let pmd = get_test_plain_month_day();

    assert!(pmd.day() >= 1 && pmd.day() <= 31);
    assert!(pmd.calendar_id().length() > 0);
    assert!(pmd.month_code().length() > 0);
}

#[wasm_bindgen_test]
fn plain_month_day_methods() {
    if !is_temporal_supported() {
        return;
    }
    let pmd = get_test_plain_month_day();

    // Test equals
    let pmd2 = Temporal::PlainMonthDay::from(
        &pmd.to_js_string(&Temporal::ShowCalendarOptions::new())
            .into(),
        &Temporal::AssignmentOptions::new(),
    )
    .unwrap();
    assert!(pmd.equals(&pmd2.into()).unwrap());

    // Test toPlainDate
    let pd = pmd.to_plain_date(&create_year_object(2022)).unwrap();
    assert_eq!(pd.day(), pmd.day());

    // Test toString/toJSON with typed options
    let str = pmd.to_js_string(&Temporal::ShowCalendarOptions::new());
    assert!(str.length() > 0);

    let json = pmd.to_json();
    assert!(json.length() > 0);
}

// ============================================================================
// Temporal.Duration tests
// ============================================================================

#[wasm_bindgen_test]
fn duration_properties() {
    if !is_temporal_supported() {
        return;
    }
    let dur = get_test_duration();

    // Properties should be accessible (values depend on test data)
    let _ = dur.years();
    let _ = dur.months();
    let _ = dur.weeks();
    let _ = dur.days();
    let _ = dur.hours();
    let _ = dur.minutes();
    let _ = dur.seconds();
    let _ = dur.milliseconds();
    let _ = dur.microseconds();
    let _ = dur.nanoseconds();
    let _ = dur.sign();
    let _ = dur.blank();
}

#[wasm_bindgen_test]
fn duration_from_string() {
    if !is_temporal_supported() {
        return;
    }
    // Test parsing ISO 8601 duration strings
    let dur = Temporal::Duration::from(&"P1Y2M3W4DT5H6M7.008009010S".into()).unwrap();
    assert_eq!(dur.years(), 1.0);
    assert_eq!(dur.months(), 2.0);
    assert_eq!(dur.weeks(), 3.0);
    assert_eq!(dur.days(), 4.0);
    assert_eq!(dur.hours(), 5.0);
    assert_eq!(dur.minutes(), 6.0);
    assert_eq!(dur.seconds(), 7.0);
    assert_eq!(dur.milliseconds(), 8.0);
    assert_eq!(dur.microseconds(), 9.0);
    assert_eq!(dur.nanoseconds(), 10.0);
    assert_eq!(dur.sign(), 1);
    assert_eq!(dur.blank(), false);
}

#[wasm_bindgen_test]
fn duration_methods() {
    if !is_temporal_supported() {
        return;
    }
    let dur = Temporal::Duration::from(&"PT1H30M".into()).unwrap();

    // Test negated
    let negated = dur.negated();
    assert_eq!(negated.sign(), -1);
    assert_eq!(negated.hours(), -1.0);

    // Test abs
    let abs_dur = negated.abs();
    assert_eq!(abs_dur.sign(), 1);
    assert_eq!(abs_dur.hours(), 1.0);

    // Test add
    let dur2 = Temporal::Duration::from(&"PT30M".into()).unwrap();
    let sum = dur.add(&dur2.clone().into()).unwrap();
    assert_eq!(sum.hours(), 2.0);
    assert_eq!(sum.minutes(), 0.0);

    // Test subtract
    let diff = dur.subtract(&dur2.into()).unwrap();
    assert_eq!(diff.hours(), 1.0);
    assert_eq!(diff.minutes(), 0.0);

    // Test total with typed options
    let total_opts = Temporal::DurationTotalOptions::new();
    total_opts.set_unit(Temporal::TotalUnit::Minute);
    let total_minutes = dur.total(&total_opts).unwrap();
    assert_eq!(total_minutes, 90.0);

    // Test toString/toJSON with typed options
    let str = dur.to_js_string(&Temporal::ToStringPrecisionOptions::new());
    assert!(str.length() > 0);

    let json = dur.to_json();
    assert!(json.length() > 0);
}

#[wasm_bindgen_test]
fn duration_compare() {
    if !is_temporal_supported() {
        return;
    }
    let dur1 = Temporal::Duration::from(&"PT1H".into()).unwrap();
    let dur2 = Temporal::Duration::from(&"PT2H".into()).unwrap();

    assert_eq!(
        Temporal::Duration::compare(
            &dur1.clone().into(),
            &dur2.clone().into(),
            &Temporal::DurationArithmeticOptions::new()
        )
        .unwrap(),
        -1
    );
    assert_eq!(
        Temporal::Duration::compare(
            &dur2.clone().into(),
            &dur1.clone().into(),
            &Temporal::DurationArithmeticOptions::new()
        )
        .unwrap(),
        1
    );
    assert_eq!(
        Temporal::Duration::compare(
            &dur1.clone().into(),
            &dur1.into(),
            &Temporal::DurationArithmeticOptions::new()
        )
        .unwrap(),
        0
    );
}

#[wasm_bindgen_test]
fn duration_blank() {
    if !is_temporal_supported() {
        return;
    }
    let zero = Temporal::Duration::from(&"PT0S".into()).unwrap();
    assert_eq!(zero.blank(), true);
    assert_eq!(zero.sign(), 0);

    let non_zero = Temporal::Duration::from(&"PT1S".into()).unwrap();
    assert_eq!(non_zero.blank(), false);
}

// ============================================================================
// Temporal Options and Enums tests
// ============================================================================

#[wasm_bindgen_test]
fn test_assignment_options() {
    if !is_temporal_supported() {
        return;
    }
    let opts = Temporal::AssignmentOptions::new();
    assert!(opts.get_overflow().is_none());

    opts.set_overflow(Temporal::TemporalOverflow::Constrain);
    assert_eq!(
        opts.get_overflow(),
        Some(Temporal::TemporalOverflow::Constrain)
    );

    opts.set_overflow(Temporal::TemporalOverflow::Reject);
    assert_eq!(
        opts.get_overflow(),
        Some(Temporal::TemporalOverflow::Reject)
    );
}

#[wasm_bindgen_test]
fn test_difference_options() {
    if !is_temporal_supported() {
        return;
    }
    let opts = Temporal::DifferenceOptions::new();

    opts.set_smallest_unit(Temporal::SmallestUnit::Day);
    assert_eq!(opts.get_smallest_unit(), Some(Temporal::SmallestUnit::Day));

    opts.set_largest_unit(Temporal::LargestUnit::Year);
    assert_eq!(opts.get_largest_unit(), Some(Temporal::LargestUnit::Year));

    opts.set_rounding_increment(5);
    assert_eq!(opts.get_rounding_increment(), Some(5));

    opts.set_rounding_mode(Intl::RoundingMode::HalfExpand);
    assert_eq!(
        opts.get_rounding_mode(),
        Some(Intl::RoundingMode::HalfExpand)
    );
}

#[wasm_bindgen_test]
fn test_to_string_options() {
    if !is_temporal_supported() {
        return;
    }
    // Test ToStringPrecisionOptions
    let opts = Temporal::ToStringPrecisionOptions::new();

    opts.set_fractional_second_digits(Temporal::FractionalSecondDigits::Three);
    assert_eq!(
        opts.get_fractional_second_digits(),
        Some(Temporal::FractionalSecondDigits::Three)
    );

    opts.set_smallest_unit(Temporal::SmallestUnit::Millisecond);
    assert_eq!(
        opts.get_smallest_unit(),
        Some(Temporal::SmallestUnit::Millisecond)
    );

    // Test ShowCalendarOptions
    let cal_opts = Temporal::ShowCalendarOptions::new();
    cal_opts.set_calendar_name(Temporal::CalendarDisplay::Always);
    assert_eq!(
        cal_opts.get_calendar_name(),
        Some(Temporal::CalendarDisplay::Always)
    );

    // Test ZonedDateTimeToStringOptions
    let zdt_opts = Temporal::ZonedDateTimeToStringOptions::new();
    zdt_opts.set_time_zone_name(Temporal::TimeZoneDisplay::Never);
    assert_eq!(
        zdt_opts.get_time_zone_name(),
        Some(Temporal::TimeZoneDisplay::Never)
    );

    zdt_opts.set_offset(Temporal::OffsetDisplay::Auto);
    assert_eq!(zdt_opts.get_offset(), Some(Temporal::OffsetDisplay::Auto));
}

#[wasm_bindgen_test]
fn test_zoned_date_time_assignment_options() {
    if !is_temporal_supported() {
        return;
    }
    let opts = Temporal::ZonedDateTimeAssignmentOptions::new();

    opts.set_overflow(Temporal::TemporalOverflow::Reject);
    assert_eq!(
        opts.get_overflow(),
        Some(Temporal::TemporalOverflow::Reject)
    );

    opts.set_disambiguation(Temporal::TemporalDisambiguation::Earlier);
    assert_eq!(
        opts.get_disambiguation(),
        Some(Temporal::TemporalDisambiguation::Earlier)
    );

    opts.set_offset(Temporal::TemporalOffsetOption::Prefer);
    assert_eq!(
        opts.get_offset(),
        Some(Temporal::TemporalOffsetOption::Prefer)
    );
}

#[wasm_bindgen_test]
fn test_duration_round_options() {
    if !is_temporal_supported() {
        return;
    }
    let opts = Temporal::DurationRoundToOptions::new();

    opts.set_smallest_unit(Temporal::SmallestUnit::Hour);
    assert_eq!(opts.get_smallest_unit(), Some(Temporal::SmallestUnit::Hour));

    opts.set_largest_unit(Temporal::LargestUnit::Day);
    assert_eq!(opts.get_largest_unit(), Some(Temporal::LargestUnit::Day));

    opts.set_rounding_increment(1);
    assert_eq!(opts.get_rounding_increment(), Some(1));

    opts.set_rounding_mode(Intl::RoundingMode::Ceil);
    assert_eq!(opts.get_rounding_mode(), Some(Intl::RoundingMode::Ceil));
}

#[wasm_bindgen_test]
fn test_duration_total_options() {
    if !is_temporal_supported() {
        return;
    }
    let opts = Temporal::DurationTotalOptions::new();

    opts.set_unit(Temporal::TotalUnit::Hours);
    assert_eq!(opts.get_unit(), Some(Temporal::TotalUnit::Hours));
}
