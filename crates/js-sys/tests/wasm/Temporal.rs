use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/Temporal.js")]
extern "C" {
    fn is_temporal_supported() -> bool;
    fn is_temporal_required() -> bool;
    fn get_test_instant() -> Temporal::Instant;
    fn get_test_zoned_date_time() -> Temporal::ZonedDateTime;
    fn get_test_plain_date_time() -> Temporal::PlainDateTime;
    fn get_test_plain_date() -> Temporal::PlainDate;
    fn get_test_plain_time() -> Temporal::PlainTime;
    fn get_test_plain_year_month() -> Temporal::PlainYearMonth;
    fn get_test_plain_month_day() -> Temporal::PlainMonthDay;
    fn get_test_duration() -> Temporal::Duration;
    fn create_day_object(day: u32) -> Object;
    fn create_year_object(year: i32) -> Object;
}

#[wasm_bindgen_test]
fn temporal_available_where_required() {
    assert!(
        is_temporal_supported() || !is_temporal_required(),
        "Temporal is missing on a runtime that ships it, so the other tests are being skipped"
    );
}

#[wasm_bindgen_test]
fn to_locale_string() {
    if !is_temporal_supported() {
        return;
    }
    let en_us = [JsString::from("en-US")];
    let pd = get_test_plain_date();
    let opts = Intl::DateTimeFormatOptions::new();
    assert_eq!(
        pd.to_locale_string_with_options(&en_us, &opts).unwrap(),
        "6/15/2022"
    );

    opts.set_time_zone("UTC");
    assert_eq!(
        get_test_instant()
            .to_locale_string_with_options(&en_us, &opts)
            .unwrap(),
        "6/15/2022, 12:30:45 PM"
    );
    assert_eq!(
        get_test_plain_time().to_locale_string(&en_us).unwrap(),
        "12:30:45 PM"
    );

    // An ISO year-month cannot be formatted in the locale's Gregorian calendar
    let pym = get_test_plain_year_month();
    let err = pym.to_locale_string(&en_us).unwrap_err();
    assert!(err.is_instance_of::<RangeError>());
    assert!(
        pym.to_locale_string(&[JsString::from("en-US-u-ca-iso8601")])
            .unwrap()
            .length()
            > 0
    );

    // A `timeZone` option conflicts with a ZonedDateTime's own time zone
    let err = get_test_zoned_date_time()
        .to_locale_string_with_options(&en_us, &opts)
        .unwrap_err();
    assert!(err.is_instance_of::<TypeError>());

    // Invalid language tag
    let err = pd.to_locale_string(&[JsString::from("en_US")]).unwrap_err();
    assert!(err.is_instance_of::<RangeError>());

    let dur = get_test_duration();
    let s = dur.to_locale_string(&en_us).unwrap();
    assert!(s.length() > 0);
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
    let zdt_utc = Temporal::Now::zoned_date_time_iso_with_time_zone("UTC").unwrap();
    assert!(zdt_utc.year() > 2020);
    assert_eq!(zdt_utc.time_zone_id(), "UTC");

    assert!(Temporal::Now::zoned_date_time_iso_with_time_zone("Not/AZone").is_err());
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
    let pdt_utc = Temporal::Now::plain_date_time_iso_with_time_zone("UTC").unwrap();
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
    let pd_utc = Temporal::Now::plain_date_iso_with_time_zone("UTC").unwrap();
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
    let pt_utc = Temporal::Now::plain_time_iso_with_time_zone("UTC").unwrap();
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
    let instant2 = Temporal::Instant::from_with_str("2020-01-01T00:00:00Z").unwrap();
    assert!(instant2.epoch_milliseconds() > 0.0);

    // Test compare
    let cmp = Temporal::Instant::compare(&instant, &instant2).unwrap();
    assert!(cmp == -1 || cmp == 0 || cmp == 1);
}

#[wasm_bindgen_test]
fn instant_methods() {
    if !is_temporal_supported() {
        return;
    }
    let instant = get_test_instant();

    // Test equals
    let instant2 = Temporal::Instant::from_with_str(&String::from(instant.to_js_string())).unwrap();
    assert!(instant.equals(&instant2).unwrap());

    // Test add/subtract
    let later = instant
        .add(&Temporal::Duration::from_with_str("PT1H").unwrap())
        .unwrap();
    assert!(later.epoch_milliseconds() > instant.epoch_milliseconds());

    let earlier = instant
        .subtract(&Temporal::Duration::from_with_str("PT1H").unwrap())
        .unwrap();
    assert!(earlier.epoch_milliseconds() < instant.epoch_milliseconds());

    // Test until/since with typed options; the default largest unit is seconds
    let duration = instant.until(&later).unwrap();
    assert_eq!(duration.seconds(), 3600.0);

    let diff_opts = Temporal::DifferenceOptions::new();
    diff_opts.set_largest_unit(Temporal::LargestUnit::Hour);
    let duration2 = later.since_with_options(&instant, &diff_opts).unwrap();
    assert_eq!(duration2.hours(), 1.0);

    // Test round with typed options
    let round_opts = Temporal::RoundToOptions::new();
    round_opts.set_smallest_unit(Temporal::TemporalUnit::Hour);
    let rounded = instant.round(&round_opts).unwrap();
    // Instant rounded to hour should have milliseconds as 0
    assert_eq!(rounded.epoch_milliseconds() % 3600000.0, 0.0);

    // Test toZonedDateTimeISO
    let zdt = instant.to_zoned_date_time_iso("UTC").unwrap();
    assert_eq!(zdt.year(), 2022);
    assert_eq!(zdt.time_zone_id(), "UTC");
    assert!(instant.to_zoned_date_time_iso("Not/AZone").is_err());

    // Test toJSON and toString
    assert_eq!(instant.to_json(), "2022-06-15T12:30:45.123456789Z");
    assert_eq!(instant.to_js_string(), "2022-06-15T12:30:45.123456789Z");

    let opts = Temporal::InstantToStringOptions::new();
    opts.set_fractional_second_digits(3);
    assert_eq!(
        instant.to_js_string_with_options(&opts).unwrap(),
        "2022-06-15T12:30:45.123Z"
    );

    let opts = Temporal::InstantToStringOptions::new();
    opts.set_smallest_unit(Temporal::TemporalUnit::Minute);
    opts.set_time_zone("America/Los_Angeles");
    assert_eq!(
        opts.get_time_zone(),
        Some(JsString::from("America/Los_Angeles"))
    );
    assert_eq!(
        instant.to_js_string_with_options(&opts).unwrap(),
        "2022-06-15T05:30-07:00"
    );

    // Calendar units are rejected by Instant.toString
    let opts = Temporal::InstantToStringOptions::new();
    opts.set_smallest_unit(Temporal::TemporalUnit::Day);
    assert!(instant.to_js_string_with_options(&opts).is_err());
}

#[wasm_bindgen_test]
fn instant_constructor() {
    if !is_temporal_supported() {
        return;
    }
    let ns = BigInt::new(&JsValue::from_str("1655296245123456789")).unwrap();
    let instant = Temporal::Instant::new(&ns).unwrap();
    assert_eq!(instant.epoch_milliseconds(), 1655296245123.0);
    assert_eq!(instant.epoch_nanoseconds(), ns);
    assert!(instant
        .equals(&Temporal::Instant::from_epoch_nanoseconds(&ns).unwrap())
        .unwrap());

    let too_large = BigInt::new(&JsValue::from_str("9000000000000000000000")).unwrap();
    assert!(Temporal::Instant::new(&too_large).is_err());
}

#[wasm_bindgen_test]
fn date_to_temporal_instant() {
    if !is_temporal_supported() {
        return;
    }
    let date = Date::new(&"2022-06-15T12:30:45.123Z".into());
    let instant = date.to_temporal_instant().unwrap();
    assert_eq!(instant.epoch_milliseconds(), 1655296245123.0);

    let invalid = Date::new(&"not a date".into());
    assert!(invalid.to_temporal_instant().is_err());
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
    let zdt2 = Temporal::ZonedDateTime::from_with_str(&String::from(zdt.to_js_string())).unwrap();
    assert!(zdt.equals(&zdt2).unwrap());

    // Test withCalendar
    let with_calendar = zdt.with_calendar("iso8601").unwrap();
    assert_eq!(with_calendar.calendar_id(), "iso8601");

    // Test add/subtract with typed options
    let later = zdt
        .add(&Temporal::Duration::from_with_str("P1D").unwrap())
        .unwrap();
    let earlier = zdt
        .subtract(&Temporal::Duration::from_with_str("P1D").unwrap())
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

    // Test withTimeZone
    let zdt_utc = zdt.with_time_zone("UTC").unwrap();
    assert_eq!(zdt_utc.time_zone_id(), "UTC");
    assert_eq!(zdt_utc.hour(), 19);
    assert!(zdt.with_time_zone("Not/AZone").is_err());

    // Test toString/toJSON
    assert_eq!(
        zdt.to_js_string(),
        "2022-06-15T12:30:45.123456789-07:00[America/Los_Angeles]"
    );
    assert_eq!(zdt.to_json(), zdt.to_js_string());

    let opts = Temporal::ZonedDateTimeToStringOptions::new();
    opts.set_fractional_second_digits(0);
    opts.set_time_zone_name(Temporal::TimeZoneDisplay::Never);
    opts.set_offset(Temporal::OffsetDisplay::Never);
    opts.set_calendar_name(Temporal::CalendarDisplay::Always);
    assert_eq!(
        zdt.to_js_string_with_options(&opts).unwrap(),
        "2022-06-15T12:30:45[u-ca=iso8601]"
    );
}

#[wasm_bindgen_test]
fn zoned_date_time_offset_and_transitions() {
    if !is_temporal_supported() {
        return;
    }
    let zdt = get_test_zoned_date_time();
    assert_eq!(zdt.offset(), "-07:00");
    assert_eq!(zdt.offset_nanoseconds(), -25_200_000_000_000.0);
    assert_eq!(zdt.era(), None);
    assert_eq!(zdt.era_year(), None);
    assert_eq!(zdt.week_of_year(), Some(24));
    assert_eq!(zdt.year_of_week(), Some(2022));

    let opts = Temporal::TimeZoneTransitionOptions::new();
    opts.set_direction(Temporal::TransitionDirection::Next);
    assert_eq!(
        opts.get_direction(),
        Some(Temporal::TransitionDirection::Next)
    );
    let next = zdt.get_time_zone_transition(&opts).unwrap().unwrap();
    assert_eq!(next.month(), 11);
    assert_eq!(next.day(), 6);
    assert_eq!(next.offset(), "-08:00");

    opts.set_direction(Temporal::TransitionDirection::Previous);
    let previous = zdt.get_time_zone_transition(&opts).unwrap().unwrap();
    assert_eq!(previous.month(), 3);
    assert_eq!(previous.day(), 13);

    let utc = zdt.with_time_zone("UTC").unwrap();
    assert!(utc.get_time_zone_transition(&opts).unwrap().is_none());

    // `direction` is required
    let err = zdt
        .get_time_zone_transition(&Temporal::TimeZoneTransitionOptions::new())
        .unwrap_err();
    assert!(err.is_instance_of::<RangeError>());
}

#[wasm_bindgen_test]
fn zoned_date_time_constructor() {
    if !is_temporal_supported() {
        return;
    }
    let ns = BigInt::new(&JsValue::from_str("1655296245123456789")).unwrap();
    let zdt = Temporal::ZonedDateTime::new(&ns, "UTC").unwrap();
    assert_eq!(zdt.hour(), 12);
    assert_eq!(zdt.calendar_id(), "iso8601");
    assert_eq!(zdt.epoch_nanoseconds(), ns);

    let zdt =
        Temporal::ZonedDateTime::new_with_calendar(&ns, "America/Los_Angeles", "gregory").unwrap();
    assert_eq!(zdt.hour(), 5);
    assert_eq!(zdt.calendar_id(), "gregory");
    assert_eq!(zdt.era(), Some(JsString::from("ce")));
    assert_eq!(zdt.era_year(), Some(2022));

    assert!(Temporal::ZonedDateTime::new(&ns, "Not/AZone").is_err());
    assert!(Temporal::ZonedDateTime::new_with_calendar(&ns, "UTC", "nocalendar").is_err());
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
    let pdt2 = Temporal::PlainDateTime::from_with_str(&String::from(pdt.to_js_string())).unwrap();
    assert!(pdt.equals(&pdt2).unwrap());

    // Test add/subtract with typed options
    let later = pdt
        .add(&Temporal::Duration::from_with_str("P1D").unwrap())
        .unwrap();
    let earlier = pdt
        .subtract(&Temporal::Duration::from_with_str("P1D").unwrap())
        .unwrap();
    assert!(later.day() != earlier.day() || later.month() != earlier.month());

    // Test round with typed options
    let round_opts = Temporal::RoundToOptions::new();
    round_opts.set_smallest_unit(Temporal::TemporalUnit::Hour);
    let rounded = pdt.round(&round_opts).unwrap();
    assert_eq!(rounded.minute(), 0);
    assert_eq!(rounded.second(), 0);

    // Test conversions
    let pd = pdt.to_plain_date();
    assert_eq!(pd.year(), pdt.year());

    let pt = pdt.to_plain_time();
    assert_eq!(pt.hour(), pdt.hour());

    // Test withPlainTime
    let with_midnight = pdt.with_plain_time().unwrap();
    assert_eq!(with_midnight.hour(), 0);

    // Test toZonedDateTime
    let zdt = pdt.to_zoned_date_time("UTC").unwrap();
    assert_eq!(zdt.year(), pdt.year());
    assert_eq!(zdt.hour(), pdt.hour());
    assert_eq!(zdt.time_zone_id(), "UTC");

    // 2022-03-13T02:30 does not exist in Los Angeles (spring forward)
    let skipped = Temporal::PlainDateTime::new(2022, 3, 13, 2, 30, 0, 0, 0, 0).unwrap();
    let opts = Temporal::ToZonedDateTimeOptions::new();
    opts.set_disambiguation(Temporal::TemporalDisambiguation::Reject);
    assert!(skipped
        .to_zoned_date_time_with_options("America/Los_Angeles", &opts)
        .is_err());
    opts.set_disambiguation(Temporal::TemporalDisambiguation::Later);
    let later = skipped
        .to_zoned_date_time_with_options("America/Los_Angeles", &opts)
        .unwrap();
    assert_eq!(later.hour(), 3);

    // Test toString/toJSON
    assert_eq!(pdt.to_js_string(), "2022-06-15T12:30:45.123456789");
    assert_eq!(pdt.to_json(), pdt.to_js_string());

    let opts = Temporal::CalendarTypeToStringOptions::new();
    opts.set_smallest_unit(Temporal::TemporalUnit::Minute);
    opts.set_calendar_name(Temporal::CalendarDisplay::Always);
    assert_eq!(
        pdt.to_js_string_with_options(&opts).unwrap(),
        "2022-06-15T12:30[u-ca=iso8601]"
    );
}

#[wasm_bindgen_test]
fn plain_date_time_constructor() {
    if !is_temporal_supported() {
        return;
    }
    let pdt = Temporal::PlainDateTime::new(2022, 6, 15, 12, 30, 45, 123, 456, 789).unwrap();
    assert!(pdt.equals(&get_test_plain_date_time()).unwrap());
    assert_eq!(pdt.era(), None);
    assert_eq!(pdt.era_year(), None);
    assert_eq!(pdt.week_of_year(), Some(24));
    assert_eq!(pdt.year_of_week(), Some(2022));

    let pdt =
        Temporal::PlainDateTime::new_with_calendar(2022, 6, 15, 12, 30, 45, 0, 0, 0, "gregory")
            .unwrap();
    assert_eq!(pdt.calendar_id(), "gregory");
    assert_eq!(pdt.era(), Some(JsString::from("ce")));

    assert!(Temporal::PlainDateTime::new(2022, 13, 1, 0, 0, 0, 0, 0, 0).is_err());
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
    let pd2 = Temporal::PlainDate::from_with_str(&String::from(pd.to_js_string())).unwrap();
    assert!(pd.equals(&pd2).unwrap());

    // Test add/subtract with typed options
    let later = pd
        .add(&Temporal::Duration::from_with_str("P1M").unwrap())
        .unwrap();
    let earlier = pd
        .subtract(&Temporal::Duration::from_with_str("P1M").unwrap())
        .unwrap();
    assert!(later.month() != earlier.month() || later.year() != earlier.year());

    // Test until/since with typed options; the default largest unit is days
    let duration = pd.until(&later).unwrap();
    assert_eq!(duration.days(), 30.0);

    let diff_opts = Temporal::DifferenceOptions::new();
    diff_opts.set_largest_unit(Temporal::LargestUnit::Month);
    let duration2 = later.since_with_options(&pd, &diff_opts).unwrap();
    assert_eq!(duration2.months(), 1.0);
    assert_eq!(duration2.days(), 0.0);

    // Test conversions
    let pdt = pd.to_plain_date_time().unwrap();
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

    // Test toString/toJSON
    assert_eq!(pd.to_js_string(), "2022-06-15");
    assert_eq!(pd.to_json(), "2022-06-15");

    let opts = Temporal::ShowCalendarOptions::new();
    opts.set_calendar_name(Temporal::CalendarDisplay::Critical);
    assert_eq!(
        pd.to_js_string_with_options(&opts).unwrap(),
        "2022-06-15[!u-ca=iso8601]"
    );
}

#[wasm_bindgen_test]
fn plain_date_constructor() {
    if !is_temporal_supported() {
        return;
    }
    let pd = Temporal::PlainDate::new(2022, 6, 15).unwrap();
    assert!(pd.equals(&get_test_plain_date()).unwrap());
    assert_eq!(pd.era(), None);
    assert_eq!(pd.era_year(), None);
    assert_eq!(pd.week_of_year(), Some(24));
    assert_eq!(pd.year_of_week(), Some(2022));

    let pd = Temporal::PlainDate::new_with_calendar(2022, 6, 15, "hebrew").unwrap();
    assert_eq!(pd.calendar_id(), "hebrew");
    assert_eq!(pd.year(), 5782);
    assert_eq!(pd.week_of_year(), None);
    assert_eq!(pd.year_of_week(), None);

    assert!(Temporal::PlainDate::new(2022, 2, 30).is_err());
    assert!(Temporal::PlainDate::new_with_calendar(2022, 6, 15, "nocalendar").is_err());
}

#[wasm_bindgen_test]
fn plain_date_compare() {
    if !is_temporal_supported() {
        return;
    }
    let pd1 = Temporal::PlainDate::from_with_str("2022-01-01").unwrap();
    let pd2 = Temporal::PlainDate::from_with_str("2022-06-15").unwrap();

    assert_eq!(Temporal::PlainDate::compare(&pd1, &pd2).unwrap(), -1);
    assert_eq!(Temporal::PlainDate::compare(&pd2, &pd1).unwrap(), 1);
    assert_eq!(Temporal::PlainDate::compare(&pd1, &pd1).unwrap(), 0);
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
    let pt2 = Temporal::PlainTime::from_with_str(&String::from(pt.to_js_string())).unwrap();
    assert!(pt.equals(&pt2).unwrap());

    // Test add/subtract (wraps around midnight)
    let _later = pt
        .add(&Temporal::Duration::from_with_str("PT1H").unwrap())
        .unwrap();
    let _earlier = pt
        .subtract(&Temporal::Duration::from_with_str("PT1H").unwrap())
        .unwrap();

    // Test until/since with typed options
    let pt_start = Temporal::PlainTime::from_with_str("10:00:00").unwrap();
    let pt_end = Temporal::PlainTime::from_with_str("12:30:00").unwrap();
    let duration = pt_start.until(&pt_end).unwrap();
    assert_eq!(duration.hours(), 2.0);
    assert_eq!(duration.minutes(), 30.0);

    // Test round with typed options
    let round_opts = Temporal::RoundToOptions::new();
    round_opts.set_smallest_unit(Temporal::TemporalUnit::Hour);
    let rounded = pt.round(&round_opts).unwrap();
    assert_eq!(rounded.minute(), 0);
    assert_eq!(rounded.second(), 0);

    // Test PlainDate.toPlainDateTime with this time
    let pd = Temporal::PlainDate::from_with_str("2024-06-15").unwrap();
    let pdt = pd.to_plain_date_time_with_plain_time(&pt).unwrap();
    assert_eq!(pdt.year(), 2024);
    assert_eq!(pdt.month(), 6);
    assert_eq!(pdt.day(), 15);
    assert_eq!(pdt.hour(), pt.hour());
    assert_eq!(pdt.minute(), pt.minute());

    // Test toString/toJSON
    assert_eq!(pt.to_js_string(), "12:30:45.123456789");
    assert_eq!(pt.to_json(), pt.to_js_string());

    let opts = Temporal::ToStringPrecisionOptions::new();
    opts.set_fractional_second_digits(3);
    assert_eq!(pt.to_js_string_with_options(&opts).unwrap(), "12:30:45.123");

    opts.set_fractional_second_digits(0);
    opts.set_rounding_mode(Intl::RoundingMode::Ceil);
    assert_eq!(pt.to_js_string_with_options(&opts).unwrap(), "12:30:46");

    let new_pt = Temporal::PlainTime::new(12, 30, 45, 123, 456, 789).unwrap();
    assert!(new_pt.equals(&pt).unwrap());
    assert!(Temporal::PlainTime::new(24, 0, 0, 0, 0, 0).is_err());
}

#[wasm_bindgen_test]
fn plain_time_compare() {
    if !is_temporal_supported() {
        return;
    }
    let pt1 = Temporal::PlainTime::from_with_str("10:00:00").unwrap();
    let pt2 = Temporal::PlainTime::from_with_str("14:30:00").unwrap();

    assert_eq!(Temporal::PlainTime::compare(&pt1, &pt2).unwrap(), -1);
    assert_eq!(Temporal::PlainTime::compare(&pt2, &pt1).unwrap(), 1);
    assert_eq!(Temporal::PlainTime::compare(&pt1, &pt1).unwrap(), 0);
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
    let pym2 = Temporal::PlainYearMonth::from_with_str(&String::from(pym.to_js_string())).unwrap();
    assert!(pym.equals(&pym2).unwrap());

    // Test add/subtract with typed options
    let later = pym
        .add(&Temporal::Duration::from_with_str("P3M").unwrap())
        .unwrap();
    let earlier = pym
        .subtract(&Temporal::Duration::from_with_str("P3M").unwrap())
        .unwrap();
    assert!(later.month() != earlier.month() || later.year() != earlier.year());

    // Test until/since with typed options
    let duration = pym.until(&later).unwrap();
    assert_eq!(duration.months(), 3.0);

    // Test toPlainDate
    let pd = pym.to_plain_date(&create_day_object(15)).unwrap();
    assert_eq!(pd.year(), pym.year());
    assert_eq!(pd.month(), pym.month());

    // Test toString/toJSON
    assert_eq!(pym.to_js_string(), "2022-06");
    assert_eq!(pym.to_json(), "2022-06");

    let opts = Temporal::ShowCalendarOptions::new();
    opts.set_calendar_name(Temporal::CalendarDisplay::Always);
    assert_eq!(
        pym.to_js_string_with_options(&opts).unwrap(),
        "2022-06-01[u-ca=iso8601]"
    );

    // Test constructors
    let new_pym = Temporal::PlainYearMonth::new(2022, 6).unwrap();
    assert!(new_pym.equals(&pym).unwrap());
    assert_eq!(new_pym.era(), None);
    assert_eq!(new_pym.era_year(), None);

    let gregory = Temporal::PlainYearMonth::new_with_calendar(2022, 6, "gregory", 1).unwrap();
    assert_eq!(gregory.calendar_id(), "gregory");
    assert_eq!(gregory.era_year(), Some(2022));

    assert!(Temporal::PlainYearMonth::new(2022, 13).is_err());
}

#[wasm_bindgen_test]
fn plain_year_month_compare() {
    if !is_temporal_supported() {
        return;
    }
    let pym1 = Temporal::PlainYearMonth::from_with_str("2022-01").unwrap();
    let pym2 = Temporal::PlainYearMonth::from_with_str("2022-06").unwrap();

    assert_eq!(Temporal::PlainYearMonth::compare(&pym1, &pym2).unwrap(), -1);
    assert_eq!(Temporal::PlainYearMonth::compare(&pym2, &pym1).unwrap(), 1);
    assert_eq!(Temporal::PlainYearMonth::compare(&pym1, &pym1).unwrap(), 0);
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
    let pmd2 = Temporal::PlainMonthDay::from_with_str(&String::from(pmd.to_js_string())).unwrap();
    assert!(pmd.equals(&pmd2).unwrap());

    // Test toPlainDate
    let pd = pmd.to_plain_date(&create_year_object(2022)).unwrap();
    assert_eq!(pd.day(), pmd.day());

    // Test toString/toJSON
    assert_eq!(pmd.to_js_string(), "06-15");
    assert_eq!(pmd.to_json(), "06-15");

    let opts = Temporal::ShowCalendarOptions::new();
    opts.set_calendar_name(Temporal::CalendarDisplay::Always);
    assert_eq!(
        pmd.to_js_string_with_options(&opts).unwrap(),
        "1972-06-15[u-ca=iso8601]"
    );

    // Test constructors
    let new_pmd = Temporal::PlainMonthDay::new(6, 15).unwrap();
    assert!(new_pmd.equals(&pmd).unwrap());

    let gregory = Temporal::PlainMonthDay::new_with_calendar(6, 15, "gregory", 2022).unwrap();
    assert_eq!(gregory.calendar_id(), "gregory");
    assert_eq!(gregory.month_code(), "M06");

    assert!(Temporal::PlainMonthDay::new(2, 30).is_err());
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
    let dur = Temporal::Duration::from_with_str("P1Y2M3W4DT5H6M7.008009010S").unwrap();
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
    assert!(!dur.blank());
}

#[wasm_bindgen_test]
fn duration_methods() {
    if !is_temporal_supported() {
        return;
    }
    let dur = Temporal::Duration::from_with_str("PT1H30M").unwrap();

    // Test negated
    let negated = dur.negated();
    assert_eq!(negated.sign(), -1);
    assert_eq!(negated.hours(), -1.0);

    // Test abs
    let abs_dur = negated.abs();
    assert_eq!(abs_dur.sign(), 1);
    assert_eq!(abs_dur.hours(), 1.0);

    // Test add
    let dur2 = Temporal::Duration::from_with_str("PT30M").unwrap();
    let sum = dur.add(&dur2).unwrap();
    assert_eq!(sum.hours(), 2.0);
    assert_eq!(sum.minutes(), 0.0);

    // Test subtract
    let diff = dur.subtract(&dur2).unwrap();
    assert_eq!(diff.hours(), 1.0);
    assert_eq!(diff.minutes(), 0.0);

    // Test total with typed options
    let total_opts = Temporal::DurationTotalOptions::new();
    total_opts.set_unit(Temporal::TemporalUnit::Minute);
    let total_minutes = dur.total(&total_opts).unwrap();
    assert_eq!(total_minutes, 90.0);

    // Test round with typed options
    let round_opts = Temporal::DurationRoundToOptions::new();
    round_opts.set_largest_unit(Temporal::LargestUnit::Minute);
    assert_eq!(dur.round(&round_opts).unwrap().minutes(), 90.0);

    round_opts.set_largest_unit(Temporal::LargestUnit::Auto);
    round_opts.set_smallest_unit(Temporal::TemporalUnit::Hour);
    round_opts.set_rounding_mode(Intl::RoundingMode::Floor);
    assert_eq!(dur.round(&round_opts).unwrap().to_js_string(), "PT1H");

    // Test with
    let with = dur
        .with(&Temporal::Duration::from_with_str("PT2H").unwrap())
        .unwrap();
    assert_eq!(with.hours(), 2.0);
    assert_eq!(with.minutes(), 0.0);

    // Test toString/toJSON
    assert_eq!(dur.to_js_string(), "PT1H30M");
    assert_eq!(dur.to_json(), "PT1H30M");

    let opts = Temporal::ToStringPrecisionOptions::new();
    opts.set_fractional_second_digits(3);
    assert_eq!(
        dur.to_js_string_with_options(&opts).unwrap(),
        "PT1H30M0.000S"
    );

    let precise = Temporal::Duration::from_with_str("PT1.123456789S").unwrap();
    opts.set_fractional_second_digits(2);
    assert_eq!(precise.to_js_string_with_options(&opts).unwrap(), "PT1.12S");

    // Duration.toString only accepts seconds and smaller
    opts.set_smallest_unit(Temporal::TemporalUnit::Minute);
    assert!(precise.to_js_string_with_options(&opts).is_err());

    // Test constructor
    let new_dur =
        Temporal::Duration::new(0.0, 0.0, 0.0, 0.0, 1.0, 30.0, 0.0, 0.0, 0.0, 0.0).unwrap();
    assert_eq!(new_dur.to_js_string(), "PT1H30M");
    assert!(Temporal::Duration::new(1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_err());
}

#[wasm_bindgen_test]
fn duration_compare() {
    if !is_temporal_supported() {
        return;
    }
    let dur1 = Temporal::Duration::from_with_str("PT1H").unwrap();
    let dur2 = Temporal::Duration::from_with_str("PT2H").unwrap();

    assert_eq!(Temporal::Duration::compare(&dur1, &dur2).unwrap(), -1);
    assert_eq!(Temporal::Duration::compare(&dur2, &dur1).unwrap(), 1);
    assert_eq!(Temporal::Duration::compare(&dur1, &dur1).unwrap(), 0);
}

#[wasm_bindgen_test]
fn duration_blank() {
    if !is_temporal_supported() {
        return;
    }
    let zero = Temporal::Duration::from_with_str("PT0S").unwrap();
    assert!(zero.blank());
    assert_eq!(zero.sign(), 0);

    let non_zero = Temporal::Duration::from_with_str("PT1S").unwrap();
    assert!(!non_zero.blank());
}

#[wasm_bindgen_test]
fn duration_compare_relative_to() {
    if !is_temporal_supported() {
        return;
    }
    let one_month = Temporal::Duration::from_with_str("P1M").unwrap();
    let thirty_days = Temporal::Duration::from_with_str("P30D").unwrap();

    // Calendar units need a reference date
    assert!(Temporal::Duration::compare(&one_month, &thirty_days).is_err());

    let opts = Temporal::DurationCompareOptions::new();
    opts.set_relative_to(&JsValue::from_str("2022-02-01"));
    assert_eq!(
        Temporal::Duration::compare_with_options(&one_month, &thirty_days, &opts).unwrap(),
        -1
    );
    opts.set_relative_to(&Temporal::PlainDate::new(2022, 3, 1).unwrap());
    assert_eq!(
        Temporal::Duration::compare_with_options(&one_month, &thirty_days, &opts).unwrap(),
        1
    );
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

    opts.set_smallest_unit(Temporal::TemporalUnit::Day);
    assert_eq!(opts.get_smallest_unit(), Some(Temporal::TemporalUnit::Day));

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

    assert_eq!(opts.get_fractional_second_digits(), None);
    opts.set_fractional_second_digits(3);
    assert_eq!(opts.get_fractional_second_digits(), Some(3));

    opts.set_smallest_unit(Temporal::TemporalUnit::Millisecond);
    assert_eq!(
        opts.get_smallest_unit(),
        Some(Temporal::TemporalUnit::Millisecond)
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

    opts.set_smallest_unit(Temporal::TemporalUnit::Hour);
    assert_eq!(opts.get_smallest_unit(), Some(Temporal::TemporalUnit::Hour));

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

    opts.set_unit(Temporal::TemporalUnit::Hour);
    assert_eq!(opts.get_unit(), Some(Temporal::TemporalUnit::Hour));
}
