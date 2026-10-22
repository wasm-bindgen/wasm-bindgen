use super::*;

/// Overflow handling mode for Temporal assignment operations.
///
/// Used by `from()` and `with()` methods on date/time types.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalOverflow {
    /// Out-of-range values are clamped to the nearest in-range value.
    Constrain = "constrain",
    /// Out-of-range values will cause the function to throw a RangeError.
    Reject = "reject",
}

/// Disambiguation mode for handling ambiguous times during DST transitions.
///
/// Used when converting `PlainDateTime` to `Instant` or `ZonedDateTime`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalDisambiguation {
    /// Equivalent to `'earlier'` for backward transitions and `'later'` for forward transitions.
    /// This matches the behavior of legacy `Date` and libraries like moment.js.
    Compatible = "compatible",
    /// The earlier time of two possible times.
    Earlier = "earlier",
    /// The later of two possible times.
    Later = "later",
    /// Throw a RangeError instead of resolving ambiguity.
    Reject = "reject",
}

/// Offset disambiguation option for ZonedDateTime parsing and conversion.
///
/// Controls how to handle time zone offset changes when parsing or converting dates.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalOffsetOption {
    /// Always use the offset to calculate the instant.
    Use = "use",
    /// Use the offset if it's valid for the date/time, otherwise use the time zone.
    Prefer = "prefer",
    /// Disregard any provided offset and use the time zone.
    Ignore = "ignore",
    /// Throw a RangeError if the offset is not valid for the time zone.
    Reject = "reject",
}

/// A date or time unit, as accepted by the `smallestUnit` and `unit` options.
///
/// Each method accepts only a subset of these; for example `Instant` methods
/// reject calendar units and `toString()` accepts `Minute` and smaller only.
/// Out-of-range units are reported as a `RangeError` by the engine.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalUnit {
    Year = "year",
    Month = "month",
    Week = "week",
    Day = "day",
    Hour = "hour",
    Minute = "minute",
    Second = "second",
    Millisecond = "millisecond",
    Microsecond = "microsecond",
    Nanosecond = "nanosecond",
}

/// A date or time unit, or `Auto`, as accepted by the `largestUnit` option.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LargestUnit {
    Auto = "auto",
    Year = "year",
    Month = "month",
    Week = "week",
    Day = "day",
    Hour = "hour",
    Minute = "minute",
    Second = "second",
    Millisecond = "millisecond",
    Microsecond = "microsecond",
    Nanosecond = "nanosecond",
}

/// Calendar display option for `toString()` methods.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarDisplay {
    /// Show calendar annotation only when it's not ISO 8601.
    Auto = "auto",
    /// Always show the calendar annotation.
    Always = "always",
    /// Never show the calendar annotation.
    Never = "never",
    /// Always show the calendar annotation with the critical flag.
    Critical = "critical",
}

/// Time zone display option for `ZonedDateTime.toString()`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeZoneDisplay {
    /// Show time zone annotation.
    Auto = "auto",
    /// Never show the time zone annotation.
    Never = "never",
    /// Always show the time zone annotation with the critical flag.
    Critical = "critical",
}

/// Offset display option for `ZonedDateTime.toString()`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetDisplay {
    /// Show the offset.
    Auto = "auto",
    /// Never show the offset.
    Never = "never",
}

/// Direction for `getTimeZoneTransition()`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionDirection {
    /// Find the next time zone transition.
    Next = "next",
    /// Find the previous time zone transition.
    Previous = "previous",
}

// AssignmentOptions - for from() and with() methods
#[wasm_bindgen]
extern "C" {
    /// Options for assigning fields using `with()` or entire objects with `from()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type AssignmentOptions;

    /// Get the overflow handling mode.
    #[wasm_bindgen(method, getter = overflow)]
    pub fn get_overflow(this: &AssignmentOptions) -> Option<TemporalOverflow>;

    /// Set the overflow handling mode.
    ///
    /// - `'constrain'` (default): Out-of-range values are clamped to the nearest in-range value.
    /// - `'reject'`: Out-of-range values will cause the function to throw a RangeError.
    #[wasm_bindgen(method, setter = overflow)]
    pub fn set_overflow(this: &AssignmentOptions, value: TemporalOverflow);
}

impl AssignmentOptions {
    /// Creates a new `AssignmentOptions` object.
    pub fn new() -> AssignmentOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for AssignmentOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ToZonedDateTimeOptions - for PlainDateTime.toZonedDateTime()
#[wasm_bindgen]
extern "C" {
    /// Options for `PlainDateTime.toZonedDateTime()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toZonedDateTime#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ToZonedDateTimeOptions;

    /// Get the disambiguation mode.
    #[wasm_bindgen(method, getter = disambiguation)]
    pub fn get_disambiguation(this: &ToZonedDateTimeOptions) -> Option<TemporalDisambiguation>;

    /// Set the disambiguation mode for handling ambiguous times.
    ///
    /// - `'compatible'` (default): Equivalent to `'earlier'` for backward transitions
    ///   and `'later'` for forward transitions.
    /// - `'earlier'`: The earlier time of two possible times.
    /// - `'later'`: The later of two possible times.
    /// - `'reject'`: Throw a RangeError instead.
    #[wasm_bindgen(method, setter = disambiguation)]
    pub fn set_disambiguation(this: &ToZonedDateTimeOptions, value: TemporalDisambiguation);
}

impl ToZonedDateTimeOptions {
    /// Creates a new `ToZonedDateTimeOptions` object.
    pub fn new() -> ToZonedDateTimeOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for ToZonedDateTimeOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ZonedDateTimeAssignmentOptions - for ZonedDateTime.from() and ZonedDateTime.with()
#[wasm_bindgen]
extern "C" {
    /// Options for `ZonedDateTime.from()` and `ZonedDateTime.with()`.
    ///
    /// The `offset` option defaults to `'reject'` for `from()` and to
    /// `'prefer'` for `with()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ZonedDateTimeAssignmentOptions;

    /// Get the overflow handling mode.
    #[wasm_bindgen(method, getter = overflow)]
    pub fn get_overflow(this: &ZonedDateTimeAssignmentOptions) -> Option<TemporalOverflow>;

    /// Set the overflow handling mode.
    #[wasm_bindgen(method, setter = overflow)]
    pub fn set_overflow(this: &ZonedDateTimeAssignmentOptions, value: TemporalOverflow);

    /// Get the disambiguation mode.
    #[wasm_bindgen(method, getter = disambiguation)]
    pub fn get_disambiguation(
        this: &ZonedDateTimeAssignmentOptions,
    ) -> Option<TemporalDisambiguation>;

    /// Set the disambiguation mode.
    #[wasm_bindgen(method, setter = disambiguation)]
    pub fn set_disambiguation(this: &ZonedDateTimeAssignmentOptions, value: TemporalDisambiguation);

    /// Get the offset option.
    #[wasm_bindgen(method, getter = offset)]
    pub fn get_offset(this: &ZonedDateTimeAssignmentOptions) -> Option<TemporalOffsetOption>;

    /// Set the offset option.
    #[wasm_bindgen(method, setter = offset)]
    pub fn set_offset(this: &ZonedDateTimeAssignmentOptions, value: TemporalOffsetOption);
}

impl ZonedDateTimeAssignmentOptions {
    /// Creates a new `ZonedDateTimeAssignmentOptions` object.
    pub fn new() -> ZonedDateTimeAssignmentOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for ZonedDateTimeAssignmentOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ArithmeticOptions - for add() and subtract()
#[wasm_bindgen]
extern "C" {
    /// Options for arithmetic operations like `add()` and `subtract()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/add#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ArithmeticOptions;

    /// Get the overflow handling mode.
    #[wasm_bindgen(method, getter = overflow)]
    pub fn get_overflow(this: &ArithmeticOptions) -> Option<TemporalOverflow>;

    /// Set the overflow handling mode.
    ///
    /// - `'constrain'` (default): Clamp the result to the allowed range.
    /// - `'reject'`: Throw a RangeError if the result is out of range.
    #[wasm_bindgen(method, setter = overflow)]
    pub fn set_overflow(this: &ArithmeticOptions, value: TemporalOverflow);
}

impl ArithmeticOptions {
    /// Creates a new `ArithmeticOptions` object.
    pub fn new() -> ArithmeticOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for ArithmeticOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ToStringPrecisionOptions - base options for toString
#[wasm_bindgen]
extern "C" {
    /// Options for outputting precision in `toString()` on types with seconds.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toString#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ToStringPrecisionOptions;

    /// Get the fractional second digits option.
    #[wasm_bindgen(method, getter = fractionalSecondDigits)]
    pub fn get_fractional_second_digits(this: &ToStringPrecisionOptions) -> Option<u8>;

    /// Set the number of fractional second digits to display, from 0 to 9.
    ///
    /// When unset, as many digits as needed to represent the value are shown.
    #[wasm_bindgen(method, setter = fractionalSecondDigits)]
    pub fn set_fractional_second_digits(this: &ToStringPrecisionOptions, value: u8);

    /// Get the smallest unit option.
    #[wasm_bindgen(method, getter = smallestUnit)]
    pub fn get_smallest_unit(this: &ToStringPrecisionOptions) -> Option<TemporalUnit>;

    /// Set the smallest unit to display.
    #[wasm_bindgen(method, setter = smallestUnit)]
    pub fn set_smallest_unit(this: &ToStringPrecisionOptions, value: TemporalUnit);

    /// Get the rounding mode.
    /// Note: Reuses `Intl::RoundingMode` which has identical values.
    #[wasm_bindgen(method, getter = roundingMode)]
    pub fn get_rounding_mode(this: &ToStringPrecisionOptions) -> Option<super::Intl::RoundingMode>;

    /// Set the rounding mode.
    #[wasm_bindgen(method, setter = roundingMode)]
    pub fn set_rounding_mode(this: &ToStringPrecisionOptions, value: super::Intl::RoundingMode);
}

impl ToStringPrecisionOptions {
    /// Creates a new `ToStringPrecisionOptions` object.
    pub fn new() -> ToStringPrecisionOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for ToStringPrecisionOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ShowCalendarOptions
#[wasm_bindgen]
extern "C" {
    /// Options for showing calendar in `toString()` output.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toString#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type ShowCalendarOptions;

    /// Get the calendar name display option.
    #[wasm_bindgen(method, getter = calendarName)]
    pub fn get_calendar_name(this: &ShowCalendarOptions) -> Option<CalendarDisplay>;

    /// Set the calendar name display option.
    ///
    /// - `'auto'` (default): Show calendar only when it's not ISO 8601.
    /// - `'always'`: Always show the calendar annotation.
    /// - `'never'`: Never show the calendar annotation.
    /// - `'critical'`: Always show with the critical flag.
    #[wasm_bindgen(method, setter = calendarName)]
    pub fn set_calendar_name(this: &ShowCalendarOptions, value: CalendarDisplay);
}

impl ShowCalendarOptions {
    /// Creates a new `ShowCalendarOptions` object.
    pub fn new() -> ShowCalendarOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for ShowCalendarOptions {
    fn default() -> Self {
        Self::new()
    }
}

// CalendarTypeToStringOptions - extends ToStringPrecisionOptions with calendar option
#[wasm_bindgen]
extern "C" {
    /// Options for `toString()` on calendar types (PlainDateTime, PlainDate, etc.).
    ///
    /// Extends `ToStringPrecisionOptions` with calendar display option.
    #[wasm_bindgen(extends = ToStringPrecisionOptions)]
    #[derive(Clone, Debug)]
    pub type CalendarTypeToStringOptions;

    /// Get the calendar name display option.
    #[wasm_bindgen(method, getter = calendarName)]
    pub fn get_calendar_name(this: &CalendarTypeToStringOptions) -> Option<CalendarDisplay>;

    /// Set the calendar name display option.
    #[wasm_bindgen(method, setter = calendarName)]
    pub fn set_calendar_name(this: &CalendarTypeToStringOptions, value: CalendarDisplay);
}

impl CalendarTypeToStringOptions {
    /// Creates a new `CalendarTypeToStringOptions` object.
    pub fn new() -> CalendarTypeToStringOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for CalendarTypeToStringOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ZonedDateTimeToStringOptions - extends CalendarTypeToStringOptions
#[wasm_bindgen]
extern "C" {
    /// Options for `ZonedDateTime.toString()`.
    ///
    /// Extends `CalendarTypeToStringOptions` with time zone and offset display options.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toString#options)
    #[wasm_bindgen(extends = CalendarTypeToStringOptions)]
    #[derive(Clone, Debug)]
    pub type ZonedDateTimeToStringOptions;

    /// Get the time zone name display option.
    #[wasm_bindgen(method, getter = timeZoneName)]
    pub fn get_time_zone_name(this: &ZonedDateTimeToStringOptions) -> Option<TimeZoneDisplay>;

    /// Set the time zone name display option.
    ///
    /// - `'auto'` (default): Show the time zone annotation.
    /// - `'never'`: Never show the time zone annotation.
    /// - `'critical'`: Always show with the critical flag.
    #[wasm_bindgen(method, setter = timeZoneName)]
    pub fn set_time_zone_name(this: &ZonedDateTimeToStringOptions, value: TimeZoneDisplay);

    /// Get the offset display option.
    #[wasm_bindgen(method, getter = offset)]
    pub fn get_offset(this: &ZonedDateTimeToStringOptions) -> Option<OffsetDisplay>;

    /// Set the offset display option.
    ///
    /// - `'auto'` (default): Show the offset.
    /// - `'never'`: Never show the offset.
    #[wasm_bindgen(method, setter = offset)]
    pub fn set_offset(this: &ZonedDateTimeToStringOptions, value: OffsetDisplay);
}

impl ZonedDateTimeToStringOptions {
    /// Creates a new `ZonedDateTimeToStringOptions` object.
    pub fn new() -> ZonedDateTimeToStringOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for ZonedDateTimeToStringOptions {
    fn default() -> Self {
        Self::new()
    }
}

// InstantToStringOptions
#[wasm_bindgen]
extern "C" {
    /// Options for `Instant.toString()`.
    ///
    /// Extends `ToStringPrecisionOptions` with optional time zone.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toString#options)
    #[wasm_bindgen(extends = ToStringPrecisionOptions)]
    #[derive(Clone, Debug)]
    pub type InstantToStringOptions;

    /// Get the time zone for display.
    #[wasm_bindgen(method, getter = timeZone)]
    pub fn get_time_zone(this: &InstantToStringOptions) -> Option<JsString>;

    /// Set the time zone for display.
    ///
    /// If provided, the output will show the wall-clock time in that time zone
    /// with its offset, instead of just 'Z'.
    #[wasm_bindgen(method, setter = timeZone)]
    pub fn set_time_zone(this: &InstantToStringOptions, value: &str);
}

impl InstantToStringOptions {
    /// Creates a new `InstantToStringOptions` object.
    pub fn new() -> InstantToStringOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for InstantToStringOptions {
    fn default() -> Self {
        Self::new()
    }
}

// DifferenceOptions - for until() and since()
#[wasm_bindgen]
extern "C" {
    /// Options to control the result of `until()` and `since()` methods.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/until#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type DifferenceOptions;

    /// Get the smallest unit option.
    #[wasm_bindgen(method, getter = smallestUnit)]
    pub fn get_smallest_unit(this: &DifferenceOptions) -> Option<TemporalUnit>;

    /// Set the smallest unit to round to.
    #[wasm_bindgen(method, setter = smallestUnit)]
    pub fn set_smallest_unit(this: &DifferenceOptions, value: TemporalUnit);

    /// Get the largest unit option.
    #[wasm_bindgen(method, getter = largestUnit)]
    pub fn get_largest_unit(this: &DifferenceOptions) -> Option<LargestUnit>;

    /// Set the largest unit to allow in the result.
    ///
    /// Larger units will be "balanced" into smaller units.
    #[wasm_bindgen(method, setter = largestUnit)]
    pub fn set_largest_unit(this: &DifferenceOptions, value: LargestUnit);

    /// Get the rounding increment.
    #[wasm_bindgen(method, getter = roundingIncrement)]
    pub fn get_rounding_increment(this: &DifferenceOptions) -> Option<u32>;

    /// Set the rounding increment.
    ///
    /// Allows rounding to an integer number of units.
    #[wasm_bindgen(method, setter = roundingIncrement)]
    pub fn set_rounding_increment(this: &DifferenceOptions, value: u32);

    /// Get the rounding mode.
    #[wasm_bindgen(method, getter = roundingMode)]
    pub fn get_rounding_mode(this: &DifferenceOptions) -> Option<super::Intl::RoundingMode>;

    /// Set the rounding mode.
    #[wasm_bindgen(method, setter = roundingMode)]
    pub fn set_rounding_mode(this: &DifferenceOptions, value: super::Intl::RoundingMode);
}

impl DifferenceOptions {
    /// Creates a new `DifferenceOptions` object.
    pub fn new() -> DifferenceOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for DifferenceOptions {
    fn default() -> Self {
        Self::new()
    }
}

// RoundToOptions - for round() methods
#[wasm_bindgen]
extern "C" {
    /// Options for `round()` methods on Temporal types.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/round#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type RoundToOptions;

    /// Get the smallest unit option.
    #[wasm_bindgen(method, getter = smallestUnit)]
    pub fn get_smallest_unit(this: &RoundToOptions) -> Option<TemporalUnit>;

    /// Set the unit to round to. This option is required.
    #[wasm_bindgen(method, setter = smallestUnit)]
    pub fn set_smallest_unit(this: &RoundToOptions, value: TemporalUnit);

    /// Get the rounding increment.
    #[wasm_bindgen(method, getter = roundingIncrement)]
    pub fn get_rounding_increment(this: &RoundToOptions) -> Option<u32>;

    /// Set the rounding increment.
    #[wasm_bindgen(method, setter = roundingIncrement)]
    pub fn set_rounding_increment(this: &RoundToOptions, value: u32);

    /// Get the rounding mode.
    #[wasm_bindgen(method, getter = roundingMode)]
    pub fn get_rounding_mode(this: &RoundToOptions) -> Option<super::Intl::RoundingMode>;

    /// Set the rounding mode. Default is `'halfExpand'`.
    #[wasm_bindgen(method, setter = roundingMode)]
    pub fn set_rounding_mode(this: &RoundToOptions, value: super::Intl::RoundingMode);
}

impl RoundToOptions {
    /// Creates a new `RoundToOptions` object.
    pub fn new() -> RoundToOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for RoundToOptions {
    fn default() -> Self {
        Self::new()
    }
}

// DurationRoundToOptions - for Duration.round()
#[wasm_bindgen]
extern "C" {
    /// Options for `Duration.round()`.
    ///
    /// Either `smallestUnit` or `largestUnit` must be provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/round#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type DurationRoundToOptions;

    /// Get the smallest unit option.
    #[wasm_bindgen(method, getter = smallestUnit)]
    pub fn get_smallest_unit(this: &DurationRoundToOptions) -> Option<TemporalUnit>;

    /// Set the unit to round to.
    #[wasm_bindgen(method, setter = smallestUnit)]
    pub fn set_smallest_unit(this: &DurationRoundToOptions, value: TemporalUnit);

    /// Get the largest unit option.
    #[wasm_bindgen(method, getter = largestUnit)]
    pub fn get_largest_unit(this: &DurationRoundToOptions) -> Option<LargestUnit>;

    /// Set the largest unit to allow in the result.
    #[wasm_bindgen(method, setter = largestUnit)]
    pub fn set_largest_unit(this: &DurationRoundToOptions, value: LargestUnit);

    /// Get the rounding increment.
    #[wasm_bindgen(method, getter = roundingIncrement)]
    pub fn get_rounding_increment(this: &DurationRoundToOptions) -> Option<u32>;

    /// Set the rounding increment.
    #[wasm_bindgen(method, setter = roundingIncrement)]
    pub fn set_rounding_increment(this: &DurationRoundToOptions, value: u32);

    /// Get the rounding mode.
    #[wasm_bindgen(method, getter = roundingMode)]
    pub fn get_rounding_mode(this: &DurationRoundToOptions) -> Option<super::Intl::RoundingMode>;

    /// Set the rounding mode.
    #[wasm_bindgen(method, setter = roundingMode)]
    pub fn set_rounding_mode(this: &DurationRoundToOptions, value: super::Intl::RoundingMode);

    /// Get the relativeTo option (returns JsValue as it can be various types).
    #[wasm_bindgen(method, getter = relativeTo)]
    pub fn get_relative_to(this: &DurationRoundToOptions) -> JsValue;

    /// Set the relativeTo option.
    ///
    /// Required when using variable-length units (years, months, weeks).
    /// Can be a `PlainDate`, `ZonedDateTime`, or a string/object that
    /// can be converted to one of those types.
    #[wasm_bindgen(method, setter = relativeTo)]
    pub fn set_relative_to(this: &DurationRoundToOptions, value: &JsValue);
}

impl DurationRoundToOptions {
    /// Creates a new `DurationRoundToOptions` object.
    pub fn new() -> DurationRoundToOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for DurationRoundToOptions {
    fn default() -> Self {
        Self::new()
    }
}

// DurationTotalOptions - for Duration.total()
#[wasm_bindgen]
extern "C" {
    /// Options for `Duration.total()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/total#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type DurationTotalOptions;

    /// Get the unit option.
    #[wasm_bindgen(method, getter = unit)]
    pub fn get_unit(this: &DurationTotalOptions) -> Option<TemporalUnit>;

    /// Set the unit to convert the duration to. This option is required.
    #[wasm_bindgen(method, setter = unit)]
    pub fn set_unit(this: &DurationTotalOptions, value: TemporalUnit);

    /// Get the relativeTo option.
    #[wasm_bindgen(method, getter = relativeTo)]
    pub fn get_relative_to(this: &DurationTotalOptions) -> JsValue;

    /// Set the relativeTo option.
    ///
    /// Required when using variable-length units (years, months, weeks).
    #[wasm_bindgen(method, setter = relativeTo)]
    pub fn set_relative_to(this: &DurationTotalOptions, value: &JsValue);
}

impl DurationTotalOptions {
    /// Creates a new `DurationTotalOptions` object.
    pub fn new() -> DurationTotalOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for DurationTotalOptions {
    fn default() -> Self {
        Self::new()
    }
}

// DurationCompareOptions - for Duration.compare()
#[wasm_bindgen]
extern "C" {
    /// Options for `Duration.compare()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/compare#options)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type DurationCompareOptions;

    /// Get the relativeTo option.
    #[wasm_bindgen(method, getter = relativeTo)]
    pub fn get_relative_to(this: &DurationCompareOptions) -> JsValue;

    /// Set the relativeTo option.
    ///
    /// Required when either duration has nonzero weeks or larger units.
    #[wasm_bindgen(method, setter = relativeTo)]
    pub fn set_relative_to(this: &DurationCompareOptions, value: &JsValue);
}

impl DurationCompareOptions {
    /// Creates a new `DurationCompareOptions` object.
    pub fn new() -> DurationCompareOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for DurationCompareOptions {
    fn default() -> Self {
        Self::new()
    }
}

// TimeZoneTransitionOptions - for getTimeZoneTransition()
#[wasm_bindgen]
extern "C" {
    /// Options for `ZonedDateTime.getTimeZoneTransition()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/getTimeZoneTransition)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type TimeZoneTransitionOptions;

    /// Get the direction option.
    #[wasm_bindgen(method, getter = direction)]
    pub fn get_direction(this: &TimeZoneTransitionOptions) -> Option<TransitionDirection>;

    /// Set the direction to search for transitions.
    ///
    /// - `'next'`: Find the next time zone transition.
    /// - `'previous'`: Find the previous time zone transition.
    #[wasm_bindgen(method, setter = direction)]
    pub fn set_direction(this: &TimeZoneTransitionOptions, value: TransitionDirection);
}

impl TimeZoneTransitionOptions {
    /// Creates a new `TimeZoneTransitionOptions` object.
    pub fn new() -> TimeZoneTransitionOptions {
        JsCast::unchecked_into(Object::new())
    }
}

impl Default for TimeZoneTransitionOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.Instant` is an exact point in time, with a precision in
    /// nanoseconds. No time zone or calendar information is present. Therefore,
    /// `Temporal.Instant` has no concept of days, months, or even hours.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type Instant;

    /// The `Temporal.Instant()` constructor creates `Temporal.Instant` objects.
    ///
    /// # Errors
    /// Throws a `RangeError` if `epoch_nanoseconds` is outside the valid range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/Instant)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(epoch_nanoseconds: &BigInt) -> Result<Instant, JsValue>;

    /// The `Temporal.Instant.from()` static method creates a new `Temporal.Instant`
    /// object from another `Temporal.Instant` or an RFC 9557 string.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is not a valid RFC 9557 format.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/from)
    #[wasm_bindgen(js_name = from, static_method_of = Instant, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<Instant, JsValue>;

    /// The `Temporal.Instant.from()` static method creates a new `Temporal.Instant`
    /// object from another `Temporal.Instant` or an RFC 9557 string.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is not a valid RFC 9557 format.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/from)
    #[wasm_bindgen(js_name = from, static_method_of = Instant, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<Instant, JsValue>;

    /// The `Temporal.Instant.fromEpochMilliseconds()` static method creates a new
    /// `Temporal.Instant` object from the number of milliseconds since the Unix epoch.
    ///
    /// # Errors
    /// Throws a `RangeError` if `epoch_milliseconds` is outside the valid range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/fromEpochMilliseconds)
    #[wasm_bindgen(static_method_of = Instant, js_namespace = Temporal, js_name = fromEpochMilliseconds, catch)]
    pub fn from_epoch_milliseconds(epoch_milliseconds: f64) -> Result<Instant, JsValue>;

    /// The `Temporal.Instant.fromEpochNanoseconds()` static method creates a new
    /// `Temporal.Instant` object from the number of nanoseconds since the Unix epoch.
    ///
    /// # Errors
    /// Throws a `RangeError` if `epoch_nanoseconds` is outside the valid range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/fromEpochNanoseconds)
    #[wasm_bindgen(static_method_of = Instant, js_namespace = Temporal, js_name = fromEpochNanoseconds, catch)]
    pub fn from_epoch_nanoseconds(epoch_nanoseconds: &BigInt) -> Result<Instant, JsValue>;

    /// The `Temporal.Instant.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` comes before, is equal to, or comes after `two`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/compare)
    #[wasm_bindgen(static_method_of = Instant, js_namespace = Temporal, catch)]
    pub fn compare(one: &Instant, two: &Instant) -> Result<i32, JsValue>;

    /// The `epochMilliseconds` accessor property returns an integer representing the
    /// number of milliseconds since the Unix epoch.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/epochMilliseconds)
    #[wasm_bindgen(method, getter, structural, js_name = epochMilliseconds)]
    pub fn epoch_milliseconds(this: &Instant) -> f64;

    /// The `epochNanoseconds` accessor property returns a BigInt representing the
    /// number of nanoseconds since the Unix epoch.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/epochNanoseconds)
    #[wasm_bindgen(method, getter, structural, js_name = epochNanoseconds)]
    pub fn epoch_nanoseconds(this: &Instant) -> BigInt;

    /// The `equals()` method returns `true` if this instant is equivalent to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &Instant, other: &Instant) -> Result<bool, JsValue>;

    /// The `add()` method returns a new `Temporal.Instant` object with the duration
    /// added to this instant. The years, months, weeks, and days fields of the
    /// duration must be zero.
    ///
    /// # Errors
    /// Throws a `RangeError` if the duration has non-zero calendar units (years, months, weeks, days)
    /// or if the result is out of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &Instant, duration: &Duration) -> Result<Instant, JsValue>;

    /// The `subtract()` method returns a new `Temporal.Instant` object with the
    /// duration subtracted from this instant. The years, months, weeks, and days
    /// fields of the duration must be zero.
    ///
    /// # Errors
    /// Throws a `RangeError` if the duration has non-zero calendar units (years, months, weeks, days)
    /// or if the result is out of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &Instant, duration: &Duration) -> Result<Instant, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this instant until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify invalid units (days and calendar units are not allowed).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/until)
    #[wasm_bindgen(method, catch)]
    pub fn until(this: &Instant, other: &Instant) -> Result<Duration, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this instant until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify invalid units (days and calendar units are not allowed).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/until)
    #[wasm_bindgen(js_name = until, method, catch)]
    pub fn until_with_options(
        this: &Instant,
        other: &Instant,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this instant.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify invalid units (days and calendar units are not allowed).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/since)
    #[wasm_bindgen(method, catch)]
    pub fn since(this: &Instant, other: &Instant) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this instant.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify invalid units (days and calendar units are not allowed).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/since)
    #[wasm_bindgen(js_name = since, method, catch)]
    pub fn since_with_options(
        this: &Instant,
        other: &Instant,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `round()` method returns a new `Temporal.Instant` object with the instant
    /// rounded to the given unit.
    ///
    /// # Errors
    /// Throws a `RangeError` if `smallestUnit` is not provided, specifies calendar units,
    /// or if `roundingIncrement` is invalid for the unit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/round)
    #[wasm_bindgen(method, catch)]
    pub fn round(this: &Instant, round_to: &RoundToOptions) -> Result<Instant, JsValue>;

    /// The `toZonedDateTimeISO()` method returns a `Temporal.ZonedDateTime`
    /// representing this instant in the ISO 8601 calendar and the given time zone.
    ///
    /// # Errors
    /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toZonedDateTimeISO)
    #[wasm_bindgen(method, js_name = toZonedDateTimeISO, catch)]
    pub fn to_zoned_date_time_iso(
        this: &Instant,
        time_zone: &str,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this instant.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(this: &Instant, locales: &[JsString]) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this instant.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &Instant,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this instant suitable
    /// for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &Instant) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this instant.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &Instant) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this instant,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &Instant,
        options: &InstantToStringOptions,
    ) -> Result<JsString, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.PlainDateTime` represents a calendar date and wall-clock time, with
    /// a precision in nanoseconds, and without any time zone.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type PlainDateTime;

    /// The `Temporal.PlainDateTime()` constructor creates `Temporal.PlainDateTime` objects
    /// in the ISO 8601 calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(
        iso_year: i32,
        iso_month: u32,
        iso_day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millisecond: u32,
        microsecond: u32,
        nanosecond: u32,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `Temporal.PlainDateTime()` constructor creates `Temporal.PlainDateTime` objects
    /// in the given calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    /// Throws a `RangeError` if `calendar` is not a supported calendar identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new_with_calendar(
        iso_year: i32,
        iso_month: u32,
        iso_day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millisecond: u32,
        microsecond: u32,
        nanosecond: u32,
        calendar: &str,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `Temporal.PlainDateTime.from()` static method creates a new `Temporal.PlainDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<PlainDateTime, JsValue>;

    /// The `Temporal.PlainDateTime.from()` static method creates a new `Temporal.PlainDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<PlainDateTime, JsValue>;

    /// The `Temporal.PlainDateTime.from()` static method creates a new `Temporal.PlainDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_str_and_options(
        item: &str,
        options: &AssignmentOptions,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `Temporal.PlainDateTime.from()` static method creates a new `Temporal.PlainDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_object_and_options(
        item: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `Temporal.PlainDateTime.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` comes before, is equal to, or comes after `two`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/compare)
    #[wasm_bindgen(static_method_of = PlainDateTime, js_namespace = Temporal, catch)]
    pub fn compare(one: &PlainDateTime, two: &PlainDateTime) -> Result<i32, JsValue>;

    /// The `era` accessor property returns a calendar-specific lowercase string
    /// representing the era of this date, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/era)
    #[wasm_bindgen(method, getter, structural)]
    pub fn era(this: &PlainDateTime) -> Option<JsString>;

    /// The `eraYear` accessor property returns a non-negative integer representing the
    /// year of this date within the era, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/eraYear)
    #[wasm_bindgen(method, getter, structural, js_name = eraYear)]
    pub fn era_year(this: &PlainDateTime) -> Option<i32>;

    /// The `year` accessor property returns an integer representing the number of years
    /// of this date relative to the start of a calendar-specific epoch year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/year)
    #[wasm_bindgen(method, getter, structural)]
    pub fn year(this: &PlainDateTime) -> i32;

    /// The `month` accessor property returns a positive integer representing the
    /// 1-based month index in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/month)
    #[wasm_bindgen(method, getter, structural)]
    pub fn month(this: &PlainDateTime) -> u32;

    /// The `monthCode` accessor property returns a calendar-specific string representing
    /// the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/monthCode)
    #[wasm_bindgen(method, getter, structural, js_name = monthCode)]
    pub fn month_code(this: &PlainDateTime) -> JsString;

    /// The `day` accessor property returns a positive integer representing the
    /// 1-based day index in the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/day)
    #[wasm_bindgen(method, getter, structural)]
    pub fn day(this: &PlainDateTime) -> u32;

    /// The `hour` accessor property returns an integer from 0 to 23 representing
    /// the hour component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/hour)
    #[wasm_bindgen(method, getter, structural)]
    pub fn hour(this: &PlainDateTime) -> u32;

    /// The `minute` accessor property returns an integer from 0 to 59 representing
    /// the minute component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/minute)
    #[wasm_bindgen(method, getter, structural)]
    pub fn minute(this: &PlainDateTime) -> u32;

    /// The `second` accessor property returns an integer from 0 to 59 representing
    /// the second component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/second)
    #[wasm_bindgen(method, getter, structural)]
    pub fn second(this: &PlainDateTime) -> u32;

    /// The `millisecond` accessor property returns an integer from 0 to 999 representing
    /// the millisecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/millisecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn millisecond(this: &PlainDateTime) -> u32;

    /// The `microsecond` accessor property returns an integer from 0 to 999 representing
    /// the microsecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/microsecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn microsecond(this: &PlainDateTime) -> u32;

    /// The `nanosecond` accessor property returns an integer from 0 to 999 representing
    /// the nanosecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/nanosecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn nanosecond(this: &PlainDateTime) -> u32;

    /// The `calendarId` accessor property returns a string representing the calendar
    /// used to interpret the internal ISO 8601 date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/calendarId)
    #[wasm_bindgen(method, getter, structural, js_name = calendarId)]
    pub fn calendar_id(this: &PlainDateTime) -> JsString;

    /// The `dayOfWeek` accessor property returns a positive integer representing the
    /// day of the week of this date (1 = Monday through 7 = Sunday for ISO 8601).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/dayOfWeek)
    #[wasm_bindgen(method, getter, structural, js_name = dayOfWeek)]
    pub fn day_of_week(this: &PlainDateTime) -> u32;

    /// The `dayOfYear` accessor property returns a positive integer representing the
    /// 1-based day index of this date in the year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/dayOfYear)
    #[wasm_bindgen(method, getter, structural, js_name = dayOfYear)]
    pub fn day_of_year(this: &PlainDateTime) -> u32;

    /// The `weekOfYear` accessor property returns a positive integer representing the
    /// 1-based week index in the `yearOfWeek` of this date, or `None` if the
    /// calendar does not have a well-defined week system.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/weekOfYear)
    #[wasm_bindgen(method, getter, structural, js_name = weekOfYear)]
    pub fn week_of_year(this: &PlainDateTime) -> Option<u32>;

    /// The `yearOfWeek` accessor property returns an integer representing the year
    /// to be paired with `weekOfYear`, or `None` if the calendar does not have
    /// a well-defined week system.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/yearOfWeek)
    #[wasm_bindgen(method, getter, structural, js_name = yearOfWeek)]
    pub fn year_of_week(this: &PlainDateTime) -> Option<i32>;

    /// The `daysInWeek` accessor property returns a positive integer representing the
    /// number of days in the week of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/daysInWeek)
    #[wasm_bindgen(method, getter, structural, js_name = daysInWeek)]
    pub fn days_in_week(this: &PlainDateTime) -> u32;

    /// The `daysInMonth` accessor property returns a positive integer representing the
    /// number of days in the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/daysInMonth)
    #[wasm_bindgen(method, getter, structural, js_name = daysInMonth)]
    pub fn days_in_month(this: &PlainDateTime) -> u32;

    /// The `daysInYear` accessor property returns a positive integer representing the
    /// number of days in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/daysInYear)
    #[wasm_bindgen(method, getter, structural, js_name = daysInYear)]
    pub fn days_in_year(this: &PlainDateTime) -> u32;

    /// The `monthsInYear` accessor property returns a positive integer representing the
    /// number of months in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/monthsInYear)
    #[wasm_bindgen(method, getter, structural, js_name = monthsInYear)]
    pub fn months_in_year(this: &PlainDateTime) -> u32;

    /// The `inLeapYear` accessor property returns `true` if this date is in a leap year,
    /// and `false` otherwise.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/inLeapYear)
    #[wasm_bindgen(method, getter, structural, js_name = inLeapYear)]
    pub fn in_leap_year(this: &PlainDateTime) -> bool;

    /// The `equals()` method returns `true` if this date-time is equivalent to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &PlainDateTime, other: &PlainDateTime) -> Result<bool, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainDateTime` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &PlainDateTime, info: &Object) -> Result<PlainDateTime, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainDateTime` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/with)
    #[wasm_bindgen(js_name = with, method, catch)]
    pub fn with_with_options(
        this: &PlainDateTime,
        info: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `withPlainTime()` method returns a new `Temporal.PlainDateTime` object with
    /// the time part replaced by the given time (or midnight if not provided).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/withPlainTime)
    #[wasm_bindgen(method, js_name = withPlainTime, catch)]
    pub fn with_plain_time(this: &PlainDateTime) -> Result<PlainDateTime, JsValue>;

    /// The `withPlainTime()` method returns a new `Temporal.PlainDateTime` object with
    /// the time part replaced by the given time (or midnight if not provided).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/withPlainTime)
    #[wasm_bindgen(method, js_name = withPlainTime, catch)]
    pub fn with_plain_time_with_plain_time(
        this: &PlainDateTime,
        plain_time: &PlainTime,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `withCalendar()` method returns a new `Temporal.PlainDateTime` object with
    /// the calendar replaced.
    ///
    /// # Errors
    /// Throws a `RangeError` if the calendar identifier is invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/withCalendar)
    #[wasm_bindgen(method, js_name = withCalendar, catch)]
    pub fn with_calendar(this: &PlainDateTime, calendar: &str) -> Result<PlainDateTime, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainDateTime` object with the
    /// duration added to this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &PlainDateTime, duration: &Duration) -> Result<PlainDateTime, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainDateTime` object with the
    /// duration added to this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/add)
    #[wasm_bindgen(js_name = add, method, catch)]
    pub fn add_with_options(
        this: &PlainDateTime,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainDateTime` object with the
    /// duration subtracted from this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &PlainDateTime, duration: &Duration) -> Result<PlainDateTime, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainDateTime` object with the
    /// duration subtracted from this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/subtract)
    #[wasm_bindgen(js_name = subtract, method, catch)]
    pub fn subtract_with_options(
        this: &PlainDateTime,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this date-time until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if datetimes have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/until)
    #[wasm_bindgen(method, catch)]
    pub fn until(this: &PlainDateTime, other: &PlainDateTime) -> Result<Duration, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this date-time until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if datetimes have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/until)
    #[wasm_bindgen(js_name = until, method, catch)]
    pub fn until_with_options(
        this: &PlainDateTime,
        other: &PlainDateTime,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if datetimes have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/since)
    #[wasm_bindgen(method, catch)]
    pub fn since(this: &PlainDateTime, other: &PlainDateTime) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if datetimes have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/since)
    #[wasm_bindgen(js_name = since, method, catch)]
    pub fn since_with_options(
        this: &PlainDateTime,
        other: &PlainDateTime,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `round()` method returns a new `Temporal.PlainDateTime` object with the
    /// date-time rounded to the given unit.
    ///
    /// # Errors
    /// Throws a `RangeError` if `smallestUnit` is not provided, specifies invalid units,
    /// or if `roundingIncrement` is invalid for the unit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/round)
    #[wasm_bindgen(method, catch)]
    pub fn round(this: &PlainDateTime, round_to: &RoundToOptions)
        -> Result<PlainDateTime, JsValue>;

    /// The `toZonedDateTime()` method returns a `Temporal.ZonedDateTime` with the
    /// same calendar date and wall-clock time in the given time zone.
    ///
    /// # Errors
    /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier,
    /// or if `disambiguation` is `'reject'` and the wall-clock time is ambiguous
    /// or does not exist in that time zone.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toZonedDateTime)
    #[wasm_bindgen(method, js_name = toZonedDateTime, catch)]
    pub fn to_zoned_date_time(
        this: &PlainDateTime,
        time_zone: &str,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `toZonedDateTime()` method returns a `Temporal.ZonedDateTime` with the
    /// same calendar date and wall-clock time in the given time zone.
    ///
    /// # Errors
    /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier,
    /// or if `disambiguation` is `'reject'` and the wall-clock time is ambiguous
    /// or does not exist in that time zone.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toZonedDateTime)
    #[wasm_bindgen(method, js_name = toZonedDateTime, catch)]
    pub fn to_zoned_date_time_with_options(
        this: &PlainDateTime,
        time_zone: &str,
        options: &ToZonedDateTimeOptions,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `toPlainDate()` method returns a `Temporal.PlainDate` object representing
    /// the date part of this date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toPlainDate)
    #[wasm_bindgen(method, js_name = toPlainDate)]
    pub fn to_plain_date(this: &PlainDateTime) -> PlainDate;

    /// The `toPlainTime()` method returns a `Temporal.PlainTime` object representing
    /// the time part of this date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toPlainTime)
    #[wasm_bindgen(method, js_name = toPlainTime)]
    pub fn to_plain_time(this: &PlainDateTime) -> PlainTime;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(
        this: &PlainDateTime,
        locales: &[JsString],
    ) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &PlainDateTime,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this date-time suitable
    /// for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &PlainDateTime) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &PlainDateTime) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this date-time,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &PlainDateTime,
        options: &CalendarTypeToStringOptions,
    ) -> Result<JsString, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.ZonedDateTime` is a timezone-aware, calendar-aware date/time object
    /// that represents a real event that has happened (or will happen) at a particular
    /// exact time from the perspective of a particular region on Earth.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type ZonedDateTime;

    /// The `Temporal.ZonedDateTime()` constructor creates `Temporal.ZonedDateTime` objects
    /// in the ISO 8601 calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if `epoch_nanoseconds` is outside the valid range or
    /// `time_zone` is not a valid time zone identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/ZonedDateTime)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(epoch_nanoseconds: &BigInt, time_zone: &str) -> Result<ZonedDateTime, JsValue>;

    /// The `Temporal.ZonedDateTime()` constructor creates `Temporal.ZonedDateTime` objects
    /// in the given calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if `epoch_nanoseconds` is outside the valid range or
    /// `time_zone` is not a valid time zone identifier.
    /// Throws a `RangeError` if `calendar` is not a supported calendar identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/ZonedDateTime)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new_with_calendar(
        epoch_nanoseconds: &BigInt,
        time_zone: &str,
        calendar: &str,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `Temporal.ZonedDateTime.from()` static method creates a new `Temporal.ZonedDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid, values are out of range
    /// (when `overflow` is `'reject'`), or the offset is invalid (when `offset` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = ZonedDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<ZonedDateTime, JsValue>;

    /// The `Temporal.ZonedDateTime.from()` static method creates a new `Temporal.ZonedDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid, values are out of range
    /// (when `overflow` is `'reject'`), or the offset is invalid (when `offset` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = ZonedDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<ZonedDateTime, JsValue>;

    /// The `Temporal.ZonedDateTime.from()` static method creates a new `Temporal.ZonedDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid, values are out of range
    /// (when `overflow` is `'reject'`), or the offset is invalid (when `offset` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = ZonedDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_str_and_options(
        item: &str,
        options: &ZonedDateTimeAssignmentOptions,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `Temporal.ZonedDateTime.from()` static method creates a new `Temporal.ZonedDateTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid, values are out of range
    /// (when `overflow` is `'reject'`), or the offset is invalid (when `offset` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = ZonedDateTime, js_namespace = Temporal, catch)]
    pub fn from_with_object_and_options(
        item: &Object,
        options: &ZonedDateTimeAssignmentOptions,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `Temporal.ZonedDateTime.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` comes before, is equal to, or comes after `two`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/compare)
    #[wasm_bindgen(static_method_of = ZonedDateTime, js_namespace = Temporal, catch)]
    pub fn compare(one: &ZonedDateTime, two: &ZonedDateTime) -> Result<i32, JsValue>;

    /// The `era` accessor property returns a calendar-specific lowercase string
    /// representing the era of this date, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/era)
    #[wasm_bindgen(method, getter, structural)]
    pub fn era(this: &ZonedDateTime) -> Option<JsString>;

    /// The `eraYear` accessor property returns a non-negative integer representing the
    /// year of this date within the era, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/eraYear)
    #[wasm_bindgen(method, getter, structural, js_name = eraYear)]
    pub fn era_year(this: &ZonedDateTime) -> Option<i32>;

    /// The `year` accessor property returns an integer representing the number of years
    /// of this date relative to the start of a calendar-specific epoch year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/year)
    #[wasm_bindgen(method, getter, structural)]
    pub fn year(this: &ZonedDateTime) -> i32;

    /// The `month` accessor property returns a positive integer representing the
    /// 1-based month index in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/month)
    #[wasm_bindgen(method, getter, structural)]
    pub fn month(this: &ZonedDateTime) -> u32;

    /// The `monthCode` accessor property returns a calendar-specific string representing
    /// the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/monthCode)
    #[wasm_bindgen(method, getter, structural, js_name = monthCode)]
    pub fn month_code(this: &ZonedDateTime) -> JsString;

    /// The `day` accessor property returns a positive integer representing the
    /// 1-based day index in the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/day)
    #[wasm_bindgen(method, getter, structural)]
    pub fn day(this: &ZonedDateTime) -> u32;

    /// The `hour` accessor property returns an integer from 0 to 23 representing
    /// the hour component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/hour)
    #[wasm_bindgen(method, getter, structural)]
    pub fn hour(this: &ZonedDateTime) -> u32;

    /// The `minute` accessor property returns an integer from 0 to 59 representing
    /// the minute component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/minute)
    #[wasm_bindgen(method, getter, structural)]
    pub fn minute(this: &ZonedDateTime) -> u32;

    /// The `second` accessor property returns an integer from 0 to 59 representing
    /// the second component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/second)
    #[wasm_bindgen(method, getter, structural)]
    pub fn second(this: &ZonedDateTime) -> u32;

    /// The `millisecond` accessor property returns an integer from 0 to 999 representing
    /// the millisecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/millisecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn millisecond(this: &ZonedDateTime) -> u32;

    /// The `microsecond` accessor property returns an integer from 0 to 999 representing
    /// the microsecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/microsecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn microsecond(this: &ZonedDateTime) -> u32;

    /// The `nanosecond` accessor property returns an integer from 0 to 999 representing
    /// the nanosecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/nanosecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn nanosecond(this: &ZonedDateTime) -> u32;

    /// The `timeZoneId` accessor property returns a string representing the time zone
    /// identifier used to interpret the internal instant.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/timeZoneId)
    #[wasm_bindgen(method, getter, structural, js_name = timeZoneId)]
    pub fn time_zone_id(this: &ZonedDateTime) -> JsString;

    /// The `calendarId` accessor property returns a string representing the calendar
    /// used to interpret the internal ISO 8601 date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/calendarId)
    #[wasm_bindgen(method, getter, structural, js_name = calendarId)]
    pub fn calendar_id(this: &ZonedDateTime) -> JsString;

    /// The `dayOfWeek` accessor property returns a positive integer representing the
    /// day of the week of this date (1 = Monday through 7 = Sunday for ISO 8601).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/dayOfWeek)
    #[wasm_bindgen(method, getter, structural, js_name = dayOfWeek)]
    pub fn day_of_week(this: &ZonedDateTime) -> u32;

    /// The `dayOfYear` accessor property returns a positive integer representing the
    /// 1-based day index of this date in the year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/dayOfYear)
    #[wasm_bindgen(method, getter, structural, js_name = dayOfYear)]
    pub fn day_of_year(this: &ZonedDateTime) -> u32;

    /// The `weekOfYear` accessor property returns a positive integer representing the
    /// 1-based week index in the `yearOfWeek` of this date, or `None` if the
    /// calendar does not have a well-defined week system.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/weekOfYear)
    #[wasm_bindgen(method, getter, structural, js_name = weekOfYear)]
    pub fn week_of_year(this: &ZonedDateTime) -> Option<u32>;

    /// The `yearOfWeek` accessor property returns an integer representing the year
    /// to be paired with `weekOfYear`, or `None` if the calendar does not have
    /// a well-defined week system.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/yearOfWeek)
    #[wasm_bindgen(method, getter, structural, js_name = yearOfWeek)]
    pub fn year_of_week(this: &ZonedDateTime) -> Option<i32>;

    /// The `hoursInDay` accessor property returns a positive number representing the
    /// number of hours in the current day. Usually 24, but may differ due to DST.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/hoursInDay)
    #[wasm_bindgen(method, getter, structural, js_name = hoursInDay)]
    pub fn hours_in_day(this: &ZonedDateTime) -> f64;

    /// The `daysInWeek` accessor property returns a positive integer representing the
    /// number of days in the week of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/daysInWeek)
    #[wasm_bindgen(method, getter, structural, js_name = daysInWeek)]
    pub fn days_in_week(this: &ZonedDateTime) -> u32;

    /// The `daysInMonth` accessor property returns a positive integer representing the
    /// number of days in the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/daysInMonth)
    #[wasm_bindgen(method, getter, structural, js_name = daysInMonth)]
    pub fn days_in_month(this: &ZonedDateTime) -> u32;

    /// The `daysInYear` accessor property returns a positive integer representing the
    /// number of days in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/daysInYear)
    #[wasm_bindgen(method, getter, structural, js_name = daysInYear)]
    pub fn days_in_year(this: &ZonedDateTime) -> u32;

    /// The `monthsInYear` accessor property returns a positive integer representing the
    /// number of months in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/monthsInYear)
    #[wasm_bindgen(method, getter, structural, js_name = monthsInYear)]
    pub fn months_in_year(this: &ZonedDateTime) -> u32;

    /// The `inLeapYear` accessor property returns `true` if this date is in a leap year,
    /// and `false` otherwise.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/inLeapYear)
    #[wasm_bindgen(method, getter, structural, js_name = inLeapYear)]
    pub fn in_leap_year(this: &ZonedDateTime) -> bool;

    /// The `offsetNanoseconds` accessor property returns an integer representing the
    /// offset from UTC in nanoseconds.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/offsetNanoseconds)
    #[wasm_bindgen(method, getter, structural, js_name = offsetNanoseconds)]
    pub fn offset_nanoseconds(this: &ZonedDateTime) -> f64;

    /// The `offset` accessor property returns a string representing the offset from UTC
    /// (e.g., '-08:00' or '+05:30').
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/offset)
    #[wasm_bindgen(method, getter, structural)]
    pub fn offset(this: &ZonedDateTime) -> JsString;

    /// The `epochMilliseconds` accessor property returns an integer representing the
    /// number of milliseconds since the Unix epoch.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/epochMilliseconds)
    #[wasm_bindgen(method, getter, structural, js_name = epochMilliseconds)]
    pub fn epoch_milliseconds(this: &ZonedDateTime) -> f64;

    /// The `epochNanoseconds` accessor property returns a BigInt representing the
    /// number of nanoseconds since the Unix epoch.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/epochNanoseconds)
    #[wasm_bindgen(method, getter, structural, js_name = epochNanoseconds)]
    pub fn epoch_nanoseconds(this: &ZonedDateTime) -> BigInt;

    /// The `equals()` method returns `true` if this zoned date-time is equivalent to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &ZonedDateTime, other: &ZonedDateTime) -> Result<bool, JsValue>;

    /// The `with()` method returns a new `Temporal.ZonedDateTime` object with some fields
    /// replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`),
    /// or if the offset is invalid (when `offset` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &ZonedDateTime, info: &Object) -> Result<ZonedDateTime, JsValue>;

    /// The `with()` method returns a new `Temporal.ZonedDateTime` object with some fields
    /// replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`),
    /// or if the offset is invalid (when `offset` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/with)
    #[wasm_bindgen(js_name = with, method, catch)]
    pub fn with_with_options(
        this: &ZonedDateTime,
        info: &Object,
        options: &ZonedDateTimeAssignmentOptions,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `withPlainTime()` method returns a new `Temporal.ZonedDateTime` object with
    /// the time part replaced by the given time (or midnight if not provided).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withPlainTime)
    #[wasm_bindgen(method, js_name = withPlainTime, catch)]
    pub fn with_plain_time(this: &ZonedDateTime) -> Result<ZonedDateTime, JsValue>;

    /// The `withPlainTime()` method returns a new `Temporal.ZonedDateTime` object with
    /// the time part replaced by the given time (or midnight if not provided).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withPlainTime)
    #[wasm_bindgen(method, js_name = withPlainTime, catch)]
    pub fn with_plain_time_with_plain_time(
        this: &ZonedDateTime,
        plain_time: &PlainTime,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `withCalendar()` method returns a new `Temporal.ZonedDateTime` object with
    /// the calendar replaced.
    ///
    /// # Errors
    /// Throws a `RangeError` if the calendar identifier is invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withCalendar)
    #[wasm_bindgen(method, js_name = withCalendar, catch)]
    pub fn with_calendar(this: &ZonedDateTime, calendar: &str) -> Result<ZonedDateTime, JsValue>;

    /// The `withTimeZone()` method returns a new `Temporal.ZonedDateTime` object
    /// representing the same instant in the given time zone.
    ///
    /// # Errors
    /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withTimeZone)
    #[wasm_bindgen(method, js_name = withTimeZone, catch)]
    pub fn with_time_zone(this: &ZonedDateTime, time_zone: &str) -> Result<ZonedDateTime, JsValue>;

    /// The `add()` method returns a new `Temporal.ZonedDateTime` object with the
    /// duration added to this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &ZonedDateTime, duration: &Duration) -> Result<ZonedDateTime, JsValue>;

    /// The `add()` method returns a new `Temporal.ZonedDateTime` object with the
    /// duration added to this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/add)
    #[wasm_bindgen(js_name = add, method, catch)]
    pub fn add_with_options(
        this: &ZonedDateTime,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `subtract()` method returns a new `Temporal.ZonedDateTime` object with the
    /// duration subtracted from this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &ZonedDateTime, duration: &Duration) -> Result<ZonedDateTime, JsValue>;

    /// The `subtract()` method returns a new `Temporal.ZonedDateTime` object with the
    /// duration subtracted from this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/subtract)
    #[wasm_bindgen(js_name = subtract, method, catch)]
    pub fn subtract_with_options(
        this: &ZonedDateTime,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this zoned date-time until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different time zones or calendars, or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/until)
    #[wasm_bindgen(method, catch)]
    pub fn until(this: &ZonedDateTime, other: &ZonedDateTime) -> Result<Duration, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this zoned date-time until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different time zones or calendars, or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/until)
    #[wasm_bindgen(js_name = until, method, catch)]
    pub fn until_with_options(
        this: &ZonedDateTime,
        other: &ZonedDateTime,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different time zones or calendars, or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/since)
    #[wasm_bindgen(method, catch)]
    pub fn since(this: &ZonedDateTime, other: &ZonedDateTime) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different time zones or calendars, or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/since)
    #[wasm_bindgen(js_name = since, method, catch)]
    pub fn since_with_options(
        this: &ZonedDateTime,
        other: &ZonedDateTime,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `round()` method returns a new `Temporal.ZonedDateTime` object with the
    /// zoned date-time rounded to the given unit.
    ///
    /// # Errors
    /// Throws a `RangeError` if `smallestUnit` is not provided, specifies invalid units,
    /// or if `roundingIncrement` is invalid for the unit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/round)
    #[wasm_bindgen(method, catch)]
    pub fn round(this: &ZonedDateTime, round_to: &RoundToOptions)
        -> Result<ZonedDateTime, JsValue>;

    /// The `startOfDay()` method returns a `Temporal.ZonedDateTime` object representing
    /// the start of the day (usually midnight) of this zoned date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/startOfDay)
    #[wasm_bindgen(method, js_name = startOfDay)]
    pub fn start_of_day(this: &ZonedDateTime) -> ZonedDateTime;

    // Next major: `-> Result<Nullable<ZonedDateTime>, JsValue>`
    /// The `getTimeZoneTransition()` method returns a `Temporal.ZonedDateTime`
    /// representing the closest UTC offset transition in this time zone in the
    /// given direction, or `None` if there is none.
    ///
    /// # Errors
    /// Throws a `RangeError` if `direction` is not set or is invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/getTimeZoneTransition)
    #[wasm_bindgen(method, js_name = getTimeZoneTransition, catch)]
    pub fn get_time_zone_transition(
        this: &ZonedDateTime,
        options: &TimeZoneTransitionOptions,
    ) -> Result<Option<ZonedDateTime>, JsValue>;

    /// The `toInstant()` method returns a `Temporal.Instant` object representing the
    /// same instant as this zoned date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toInstant)
    #[wasm_bindgen(method, js_name = toInstant)]
    pub fn to_instant(this: &ZonedDateTime) -> Instant;

    /// The `toPlainDateTime()` method returns a `Temporal.PlainDateTime` object representing
    /// the date and time parts of this zoned date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toPlainDateTime)
    #[wasm_bindgen(method, js_name = toPlainDateTime)]
    pub fn to_plain_date_time(this: &ZonedDateTime) -> PlainDateTime;

    /// The `toPlainDate()` method returns a `Temporal.PlainDate` object representing
    /// the date part of this zoned date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toPlainDate)
    #[wasm_bindgen(method, js_name = toPlainDate)]
    pub fn to_plain_date(this: &ZonedDateTime) -> PlainDate;

    /// The `toPlainTime()` method returns a `Temporal.PlainTime` object representing
    /// the time part of this zoned date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toPlainTime)
    #[wasm_bindgen(method, js_name = toPlainTime)]
    pub fn to_plain_time(this: &ZonedDateTime) -> PlainTime;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(
        this: &ZonedDateTime,
        locales: &[JsString],
    ) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this zoned date-time.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &ZonedDateTime,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this zoned date-time
    /// suitable for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &ZonedDateTime) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this date-time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &ZonedDateTime) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this date-time,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &ZonedDateTime,
        options: &ZonedDateTimeToStringOptions,
    ) -> Result<JsString, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.Duration` represents an immutable duration of time which can be
    /// used in date/time arithmetic.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type Duration;

    /// The `Temporal.Duration()` constructor creates `Temporal.Duration` objects.
    ///
    /// # Errors
    /// Throws a `RangeError` if any component is non-integer, infinite, exceeds
    /// safe integer range, or if components have mixed signs.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(
        years: f64,
        months: f64,
        weeks: f64,
        days: f64,
        hours: f64,
        minutes: f64,
        seconds: f64,
        milliseconds: f64,
        microseconds: f64,
        nanoseconds: f64,
    ) -> Result<Duration, JsValue>;

    /// The `Temporal.Duration.from()` static method creates a new `Temporal.Duration`
    /// object from another Duration, an object with duration properties, or an ISO 8601 string.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is not a valid ISO 8601 duration format,
    /// or if any numeric values are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/from)
    #[wasm_bindgen(js_name = from, static_method_of = Duration, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<Duration, JsValue>;

    /// The `Temporal.Duration.from()` static method creates a new `Temporal.Duration`
    /// object from another Duration, an object with duration properties, or an ISO 8601 string.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is not a valid ISO 8601 duration format,
    /// or if any numeric values are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/from)
    #[wasm_bindgen(js_name = from, static_method_of = Duration, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<Duration, JsValue>;

    /// The `Temporal.Duration.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` is shorter than, equal to, or longer than `two`.
    ///
    /// # Errors
    /// Throws a `RangeError` if `relativeTo` is required (when either duration has
    /// calendar units like years, months, or weeks) but not provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/compare)
    #[wasm_bindgen(static_method_of = Duration, js_namespace = Temporal, catch)]
    pub fn compare(one: &Duration, two: &Duration) -> Result<i32, JsValue>;

    /// The `Temporal.Duration.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` is shorter than, equal to, or longer than `two`.
    ///
    /// # Errors
    /// Throws a `RangeError` if `relativeTo` is required (when either duration has
    /// calendar units like years, months, or weeks) but not provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/compare)
    #[wasm_bindgen(js_name = compare, static_method_of = Duration, js_namespace = Temporal, catch)]
    pub fn compare_with_options(
        one: &Duration,
        two: &Duration,
        options: &DurationCompareOptions,
    ) -> Result<i32, JsValue>;

    /// The `sign` accessor property returns -1, 0, or 1 depending on whether
    /// the duration is negative, zero, or positive.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/sign)
    #[wasm_bindgen(method, getter, structural)]
    pub fn sign(this: &Duration) -> i32;

    /// The `blank` accessor property returns `true` if the duration represents
    /// zero length of time, and `false` otherwise.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/blank)
    #[wasm_bindgen(method, getter, structural)]
    pub fn blank(this: &Duration) -> bool;

    /// The `years` accessor property returns the number of years in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/years)
    #[wasm_bindgen(method, getter, structural)]
    pub fn years(this: &Duration) -> f64;

    /// The `months` accessor property returns the number of months in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/months)
    #[wasm_bindgen(method, getter, structural)]
    pub fn months(this: &Duration) -> f64;

    /// The `weeks` accessor property returns the number of weeks in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/weeks)
    #[wasm_bindgen(method, getter, structural)]
    pub fn weeks(this: &Duration) -> f64;

    /// The `days` accessor property returns the number of days in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/days)
    #[wasm_bindgen(method, getter, structural)]
    pub fn days(this: &Duration) -> f64;

    /// The `hours` accessor property returns the number of hours in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/hours)
    #[wasm_bindgen(method, getter, structural)]
    pub fn hours(this: &Duration) -> f64;

    /// The `minutes` accessor property returns the number of minutes in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/minutes)
    #[wasm_bindgen(method, getter, structural)]
    pub fn minutes(this: &Duration) -> f64;

    /// The `seconds` accessor property returns the number of seconds in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/seconds)
    #[wasm_bindgen(method, getter, structural)]
    pub fn seconds(this: &Duration) -> f64;

    /// The `milliseconds` accessor property returns the number of milliseconds in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/milliseconds)
    #[wasm_bindgen(method, getter, structural)]
    pub fn milliseconds(this: &Duration) -> f64;

    /// The `microseconds` accessor property returns the number of microseconds in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/microseconds)
    #[wasm_bindgen(method, getter, structural)]
    pub fn microseconds(this: &Duration) -> f64;

    /// The `nanoseconds` accessor property returns the number of nanoseconds in the duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/nanoseconds)
    #[wasm_bindgen(method, getter, structural)]
    pub fn nanoseconds(this: &Duration) -> f64;

    /// The `negated()` method returns a new `Temporal.Duration` object with the
    /// opposite sign (positive becomes negative and vice versa).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/negated)
    #[wasm_bindgen(method)]
    pub fn negated(this: &Duration) -> Duration;

    /// The `abs()` method returns a new `Temporal.Duration` object with the
    /// absolute value of the duration (all negative values become positive).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/abs)
    #[wasm_bindgen(method)]
    pub fn abs(this: &Duration) -> Duration;

    /// The `with()` method returns a new `Temporal.Duration` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if any numeric values are invalid or result in mixed signs.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &Duration, duration_like: &Object) -> Result<Duration, JsValue>;

    /// The `add()` method returns a new `Temporal.Duration` object with the
    /// sum of this duration and another duration.
    ///
    /// # Errors
    /// Throws a `RangeError` if either duration has nonzero years, months or weeks,
    /// or if the result would exceed the valid range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &Duration, other: &Duration) -> Result<Duration, JsValue>;

    /// The `subtract()` method returns a new `Temporal.Duration` object with the
    /// difference of this duration and another duration.
    ///
    /// # Errors
    /// Throws a `RangeError` if either duration has nonzero years, months or weeks,
    /// or if the result would exceed the valid range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &Duration, other: &Duration) -> Result<Duration, JsValue>;

    /// The `round()` method returns a new `Temporal.Duration` object with the
    /// duration rounded and/or balanced to the given options.
    ///
    /// # Errors
    /// Throws a `RangeError` if neither `smallestUnit` nor `largestUnit` is provided,
    /// if `relativeTo` is required for calendar units but not provided, or if options
    /// are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/round)
    #[wasm_bindgen(method, catch)]
    pub fn round(this: &Duration, round_to: &DurationRoundToOptions) -> Result<Duration, JsValue>;

    /// The `total()` method returns the total number of the specified unit that
    /// fits in the duration as a floating point number.
    ///
    /// # Errors
    /// Throws a `RangeError` if `unit` is not provided, or if `relativeTo` is required
    /// for calendar units (years, months, weeks) but not provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/total)
    #[wasm_bindgen(method, catch)]
    pub fn total(this: &Duration, total_of: &DurationTotalOptions) -> Result<f64, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this duration.
    ///
    /// `options` are `Intl.DurationFormat` options.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(this: &Duration, locales: &[JsString]) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this duration.
    ///
    /// `options` are `Intl.DurationFormat` options.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &Duration,
        locales: &[JsString],
        options: &super::Intl::DurationFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this duration
    /// suitable for JSON serialization (same as `toString()`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &Duration) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this duration.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &Duration) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this duration,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &Duration,
        options: &ToStringPrecisionOptions,
    ) -> Result<JsString, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.PlainTime` represents a wall-clock time, with a precision in
    /// nanoseconds, and without any time zone or calendar information.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type PlainTime;

    /// The `Temporal.PlainTime()` constructor creates `Temporal.PlainTime` objects.
    ///
    /// # Errors
    /// Throws a `RangeError` if any component is out of range (hour: 0-23, minute/second: 0-59,
    /// millisecond/microsecond/nanosecond: 0-999).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(
        hour: u32,
        minute: u32,
        second: u32,
        millisecond: u32,
        microsecond: u32,
        nanosecond: u32,
    ) -> Result<PlainTime, JsValue>;

    /// The `Temporal.PlainTime.from()` static method creates a new `Temporal.PlainTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainTime, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<PlainTime, JsValue>;

    /// The `Temporal.PlainTime.from()` static method creates a new `Temporal.PlainTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainTime, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<PlainTime, JsValue>;

    /// The `Temporal.PlainTime.from()` static method creates a new `Temporal.PlainTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainTime, js_namespace = Temporal, catch)]
    pub fn from_with_str_and_options(
        item: &str,
        options: &AssignmentOptions,
    ) -> Result<PlainTime, JsValue>;

    /// The `Temporal.PlainTime.from()` static method creates a new `Temporal.PlainTime`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainTime, js_namespace = Temporal, catch)]
    pub fn from_with_object_and_options(
        item: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainTime, JsValue>;

    /// The `Temporal.PlainTime.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` comes before, is equal to, or comes after `two`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/compare)
    #[wasm_bindgen(static_method_of = PlainTime, js_namespace = Temporal, catch)]
    pub fn compare(one: &PlainTime, two: &PlainTime) -> Result<i32, JsValue>;

    /// The `hour` accessor property returns an integer from 0 to 23 representing
    /// the hour component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/hour)
    #[wasm_bindgen(method, getter, structural)]
    pub fn hour(this: &PlainTime) -> u32;

    /// The `minute` accessor property returns an integer from 0 to 59 representing
    /// the minute component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/minute)
    #[wasm_bindgen(method, getter, structural)]
    pub fn minute(this: &PlainTime) -> u32;

    /// The `second` accessor property returns an integer from 0 to 59 representing
    /// the second component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/second)
    #[wasm_bindgen(method, getter, structural)]
    pub fn second(this: &PlainTime) -> u32;

    /// The `millisecond` accessor property returns an integer from 0 to 999 representing
    /// the millisecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/millisecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn millisecond(this: &PlainTime) -> u32;

    /// The `microsecond` accessor property returns an integer from 0 to 999 representing
    /// the microsecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/microsecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn microsecond(this: &PlainTime) -> u32;

    /// The `nanosecond` accessor property returns an integer from 0 to 999 representing
    /// the nanosecond component of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/nanosecond)
    #[wasm_bindgen(method, getter, structural)]
    pub fn nanosecond(this: &PlainTime) -> u32;

    /// The `equals()` method returns `true` if this time is equal to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &PlainTime, other: &PlainTime) -> Result<bool, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainTime` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &PlainTime, time_like: &Object) -> Result<PlainTime, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainTime` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/with)
    #[wasm_bindgen(js_name = with, method, catch)]
    pub fn with_with_options(
        this: &PlainTime,
        time_like: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainTime, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainTime` object with the
    /// duration added to this time (wrapping around midnight if necessary).
    ///
    /// # Errors
    /// Throws a `RangeError` if the duration has non-zero calendar units (years, months, weeks, days).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &PlainTime, duration: &Duration) -> Result<PlainTime, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainTime` object with the
    /// duration subtracted from this time (wrapping around midnight if necessary).
    ///
    /// # Errors
    /// Throws a `RangeError` if the duration has non-zero calendar units (years, months, weeks, days).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &PlainTime, duration: &Duration) -> Result<PlainTime, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this time until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify calendar units or are otherwise invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/until)
    #[wasm_bindgen(method, catch)]
    pub fn until(this: &PlainTime, other: &PlainTime) -> Result<Duration, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this time until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify calendar units or are otherwise invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/until)
    #[wasm_bindgen(js_name = until, method, catch)]
    pub fn until_with_options(
        this: &PlainTime,
        other: &PlainTime,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this time.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify calendar units or are otherwise invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/since)
    #[wasm_bindgen(method, catch)]
    pub fn since(this: &PlainTime, other: &PlainTime) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this time.
    ///
    /// # Errors
    /// Throws a `RangeError` if options specify calendar units or are otherwise invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/since)
    #[wasm_bindgen(js_name = since, method, catch)]
    pub fn since_with_options(
        this: &PlainTime,
        other: &PlainTime,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `round()` method returns a new `Temporal.PlainTime` object with the
    /// time rounded to the given unit.
    ///
    /// # Errors
    /// Throws a `RangeError` if `smallestUnit` is not provided or specifies calendar units,
    /// or if `roundingIncrement` is invalid for the unit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/round)
    #[wasm_bindgen(method, catch)]
    pub fn round(this: &PlainTime, round_to: &RoundToOptions) -> Result<PlainTime, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this time.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(this: &PlainTime, locales: &[JsString]) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this time.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &PlainTime,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this time
    /// suitable for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &PlainTime) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this time.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &PlainTime) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this time,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &PlainTime,
        options: &ToStringPrecisionOptions,
    ) -> Result<JsString, JsValue>;

}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.PlainYearMonth` represents a particular month on the calendar. For example,
    /// it could be used to represent a particular instance of a monthly recurring event, like
    /// "the June 2019 meeting".
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type PlainYearMonth;

    /// The `Temporal.PlainYearMonth()` constructor creates `Temporal.PlainYearMonth` objects
    /// in the ISO 8601 calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/PlainYearMonth)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(iso_year: i32, iso_month: u32) -> Result<PlainYearMonth, JsValue>;

    /// The `Temporal.PlainYearMonth()` constructor creates `Temporal.PlainYearMonth` objects
    /// in the given calendar.
    /// `reference_iso_day` selects the ISO day used to represent the month in
    /// non-ISO calendars.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    /// Throws a `RangeError` if `calendar` is not a supported calendar identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/PlainYearMonth)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new_with_calendar(
        iso_year: i32,
        iso_month: u32,
        calendar: &str,
        reference_iso_day: u32,
    ) -> Result<PlainYearMonth, JsValue>;

    /// The `Temporal.PlainYearMonth.from()` static method creates a new `Temporal.PlainYearMonth`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainYearMonth, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<PlainYearMonth, JsValue>;

    /// The `Temporal.PlainYearMonth.from()` static method creates a new `Temporal.PlainYearMonth`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainYearMonth, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<PlainYearMonth, JsValue>;

    /// The `Temporal.PlainYearMonth.from()` static method creates a new `Temporal.PlainYearMonth`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainYearMonth, js_namespace = Temporal, catch)]
    pub fn from_with_str_and_options(
        item: &str,
        options: &AssignmentOptions,
    ) -> Result<PlainYearMonth, JsValue>;

    /// The `Temporal.PlainYearMonth.from()` static method creates a new `Temporal.PlainYearMonth`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainYearMonth, js_namespace = Temporal, catch)]
    pub fn from_with_object_and_options(
        item: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainYearMonth, JsValue>;

    /// The `Temporal.PlainYearMonth.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` comes before, is equal to, or comes after `two`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/compare)
    #[wasm_bindgen(static_method_of = PlainYearMonth, js_namespace = Temporal, catch)]
    pub fn compare(one: &PlainYearMonth, two: &PlainYearMonth) -> Result<i32, JsValue>;

    /// The `era` accessor property returns a calendar-specific lowercase string
    /// representing the era of this year-month, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/era)
    #[wasm_bindgen(method, getter, structural)]
    pub fn era(this: &PlainYearMonth) -> Option<JsString>;

    /// The `eraYear` accessor property returns a non-negative integer representing the
    /// year of this year-month within the era, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/eraYear)
    #[wasm_bindgen(method, getter, structural, js_name = eraYear)]
    pub fn era_year(this: &PlainYearMonth) -> Option<i32>;

    /// The `year` accessor property returns an integer representing the number of years
    /// of this year-month relative to the start of a calendar-specific epoch year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/year)
    #[wasm_bindgen(method, getter, structural)]
    pub fn year(this: &PlainYearMonth) -> i32;

    /// The `month` accessor property returns a positive integer representing the
    /// 1-based month index in the year of this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/month)
    #[wasm_bindgen(method, getter, structural)]
    pub fn month(this: &PlainYearMonth) -> u32;

    /// The `monthCode` accessor property returns a calendar-specific string representing
    /// the month of this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/monthCode)
    #[wasm_bindgen(method, getter, structural, js_name = monthCode)]
    pub fn month_code(this: &PlainYearMonth) -> JsString;

    /// The `calendarId` accessor property returns a string representing the calendar
    /// used to interpret the internal ISO 8601 date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/calendarId)
    #[wasm_bindgen(method, getter, structural, js_name = calendarId)]
    pub fn calendar_id(this: &PlainYearMonth) -> JsString;

    /// The `daysInMonth` accessor property returns a positive integer representing the
    /// number of days in the month of this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/daysInMonth)
    #[wasm_bindgen(method, getter, structural, js_name = daysInMonth)]
    pub fn days_in_month(this: &PlainYearMonth) -> u32;

    /// The `daysInYear` accessor property returns a positive integer representing the
    /// number of days in the year of this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/daysInYear)
    #[wasm_bindgen(method, getter, structural, js_name = daysInYear)]
    pub fn days_in_year(this: &PlainYearMonth) -> u32;

    /// The `monthsInYear` accessor property returns a positive integer representing the
    /// number of months in the year of this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/monthsInYear)
    #[wasm_bindgen(method, getter, structural, js_name = monthsInYear)]
    pub fn months_in_year(this: &PlainYearMonth) -> u32;

    /// The `inLeapYear` accessor property returns `true` if this year-month is in a leap year,
    /// and `false` otherwise.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/inLeapYear)
    #[wasm_bindgen(method, getter, structural, js_name = inLeapYear)]
    pub fn in_leap_year(this: &PlainYearMonth) -> bool;

    /// The `equals()` method returns `true` if this year-month is equivalent to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &PlainYearMonth, other: &PlainYearMonth) -> Result<bool, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainYearMonth` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &PlainYearMonth, info: &Object) -> Result<PlainYearMonth, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainYearMonth` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/with)
    #[wasm_bindgen(js_name = with, method, catch)]
    pub fn with_with_options(
        this: &PlainYearMonth,
        info: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainYearMonth, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainYearMonth` object with the
    /// duration added to this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &PlainYearMonth, duration: &Duration) -> Result<PlainYearMonth, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainYearMonth` object with the
    /// duration added to this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/add)
    #[wasm_bindgen(js_name = add, method, catch)]
    pub fn add_with_options(
        this: &PlainYearMonth,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<PlainYearMonth, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainYearMonth` object with the
    /// duration subtracted from this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &PlainYearMonth, duration: &Duration) -> Result<PlainYearMonth, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainYearMonth` object with the
    /// duration subtracted from this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/subtract)
    #[wasm_bindgen(js_name = subtract, method, catch)]
    pub fn subtract_with_options(
        this: &PlainYearMonth,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<PlainYearMonth, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this year-month until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/until)
    #[wasm_bindgen(method, catch)]
    pub fn until(this: &PlainYearMonth, other: &PlainYearMonth) -> Result<Duration, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this year-month until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/until)
    #[wasm_bindgen(js_name = until, method, catch)]
    pub fn until_with_options(
        this: &PlainYearMonth,
        other: &PlainYearMonth,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/since)
    #[wasm_bindgen(method, catch)]
    pub fn since(this: &PlainYearMonth, other: &PlainYearMonth) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/since)
    #[wasm_bindgen(js_name = since, method, catch)]
    pub fn since_with_options(
        this: &PlainYearMonth,
        other: &PlainYearMonth,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `toPlainDate()` method returns a `Temporal.PlainDate` object representing
    /// this year-month with the given day.
    ///
    /// # Errors
    /// Throws a `RangeError` if the day doesn't exist in this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toPlainDate)
    #[wasm_bindgen(method, js_name = toPlainDate, catch)]
    pub fn to_plain_date(this: &PlainYearMonth, day: &Object) -> Result<PlainDate, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(
        this: &PlainYearMonth,
        locales: &[JsString],
    ) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this year-month.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &PlainYearMonth,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this year-month
    /// suitable for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &PlainYearMonth) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this year-month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &PlainYearMonth) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this year-month,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &PlainYearMonth,
        options: &ShowCalendarOptions,
    ) -> Result<JsString, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.PlainMonthDay` represents a particular day on the calendar, but without
    /// a year. For example, it could be used to represent a yearly recurring event, like
    /// "Bastille Day is on the 14th of July."
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type PlainMonthDay;

    /// The `Temporal.PlainMonthDay()` constructor creates `Temporal.PlainMonthDay` objects
    /// in the ISO 8601 calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/PlainMonthDay)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(iso_month: u32, iso_day: u32) -> Result<PlainMonthDay, JsValue>;

    /// The `Temporal.PlainMonthDay()` constructor creates `Temporal.PlainMonthDay` objects
    /// in the given calendar.
    /// `reference_iso_year` selects the ISO year used to represent the month-day
    /// in non-ISO calendars.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    /// Throws a `RangeError` if `calendar` is not a supported calendar identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/PlainMonthDay)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new_with_calendar(
        iso_month: u32,
        iso_day: u32,
        calendar: &str,
        reference_iso_year: i32,
    ) -> Result<PlainMonthDay, JsValue>;

    /// The `Temporal.PlainMonthDay.from()` static method creates a new `Temporal.PlainMonthDay`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainMonthDay, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<PlainMonthDay, JsValue>;

    /// The `Temporal.PlainMonthDay.from()` static method creates a new `Temporal.PlainMonthDay`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainMonthDay, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<PlainMonthDay, JsValue>;

    /// The `Temporal.PlainMonthDay.from()` static method creates a new `Temporal.PlainMonthDay`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainMonthDay, js_namespace = Temporal, catch)]
    pub fn from_with_str_and_options(
        item: &str,
        options: &AssignmentOptions,
    ) -> Result<PlainMonthDay, JsValue>;

    /// The `Temporal.PlainMonthDay.from()` static method creates a new `Temporal.PlainMonthDay`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainMonthDay, js_namespace = Temporal, catch)]
    pub fn from_with_object_and_options(
        item: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainMonthDay, JsValue>;

    /// The `monthCode` accessor property returns a calendar-specific string representing
    /// the month of this month-day.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/monthCode)
    #[wasm_bindgen(method, getter, structural, js_name = monthCode)]
    pub fn month_code(this: &PlainMonthDay) -> JsString;

    /// The `day` accessor property returns a positive integer representing the
    /// day of the month.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/day)
    #[wasm_bindgen(method, getter, structural)]
    pub fn day(this: &PlainMonthDay) -> u32;

    /// The `calendarId` accessor property returns a string representing the calendar
    /// used to interpret the internal ISO 8601 date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/calendarId)
    #[wasm_bindgen(method, getter, structural, js_name = calendarId)]
    pub fn calendar_id(this: &PlainMonthDay) -> JsString;

    /// The `equals()` method returns `true` if this month-day is equivalent to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &PlainMonthDay, other: &PlainMonthDay) -> Result<bool, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainMonthDay` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &PlainMonthDay, info: &Object) -> Result<PlainMonthDay, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainMonthDay` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/with)
    #[wasm_bindgen(js_name = with, method, catch)]
    pub fn with_with_options(
        this: &PlainMonthDay,
        info: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainMonthDay, JsValue>;

    /// The `toPlainDate()` method returns a `Temporal.PlainDate` object representing
    /// this month-day in the given year.
    ///
    /// # Errors
    /// Throws a `RangeError` if the date doesn't exist in the given year (e.g., Feb 29 in non-leap year).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toPlainDate)
    #[wasm_bindgen(method, js_name = toPlainDate, catch)]
    pub fn to_plain_date(this: &PlainMonthDay, year: &Object) -> Result<PlainDate, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this month-day.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(
        this: &PlainMonthDay,
        locales: &[JsString],
    ) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this month-day.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &PlainMonthDay,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this month-day
    /// suitable for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &PlainMonthDay) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this month-day.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &PlainMonthDay) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this month-day,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &PlainMonthDay,
        options: &ShowCalendarOptions,
    ) -> Result<JsString, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A `Temporal.PlainDate` represents a calendar date without time or time zone information.
    /// For example, it could be used to represent an event on a calendar which happens during
    /// the whole day no matter which time zone it's happening in.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate)
    #[wasm_bindgen(js_namespace = Temporal, extends = Object)]
    #[derive(Clone, Debug)]
    pub type PlainDate;

    /// The `Temporal.PlainDate()` constructor creates `Temporal.PlainDate` objects
    /// in the ISO 8601 calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/PlainDate)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new(iso_year: i32, iso_month: u32, iso_day: u32) -> Result<PlainDate, JsValue>;

    /// The `Temporal.PlainDate()` constructor creates `Temporal.PlainDate` objects
    /// in the given calendar.
    ///
    /// # Errors
    /// Throws a `RangeError` if any field is out of range.
    /// Throws a `RangeError` if `calendar` is not a supported calendar identifier.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/PlainDate)
    #[wasm_bindgen(constructor, js_namespace = Temporal, catch)]
    pub fn new_with_calendar(
        iso_year: i32,
        iso_month: u32,
        iso_day: u32,
        calendar: &str,
    ) -> Result<PlainDate, JsValue>;

    /// The `Temporal.PlainDate.from()` static method creates a new `Temporal.PlainDate`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDate, js_namespace = Temporal, catch)]
    pub fn from_with_str(item: &str) -> Result<PlainDate, JsValue>;

    /// The `Temporal.PlainDate.from()` static method creates a new `Temporal.PlainDate`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDate, js_namespace = Temporal, catch)]
    pub fn from_with_object(item: &Object) -> Result<PlainDate, JsValue>;

    /// The `Temporal.PlainDate.from()` static method creates a new `Temporal.PlainDate`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDate, js_namespace = Temporal, catch)]
    pub fn from_with_str_and_options(
        item: &str,
        options: &AssignmentOptions,
    ) -> Result<PlainDate, JsValue>;

    /// The `Temporal.PlainDate.from()` static method creates a new `Temporal.PlainDate`
    /// object from another value.
    ///
    /// # Errors
    /// Throws a `RangeError` if the string is invalid or values are out of range
    /// (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from)
    #[wasm_bindgen(js_name = from, static_method_of = PlainDate, js_namespace = Temporal, catch)]
    pub fn from_with_object_and_options(
        item: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainDate, JsValue>;

    /// The `Temporal.PlainDate.compare()` static method returns -1, 0, or 1 depending
    /// on whether `one` comes before, is equal to, or comes after `two`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/compare)
    #[wasm_bindgen(static_method_of = PlainDate, js_namespace = Temporal, catch)]
    pub fn compare(one: &PlainDate, two: &PlainDate) -> Result<i32, JsValue>;

    /// The `era` accessor property returns a calendar-specific lowercase string
    /// representing the era of this date, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/era)
    #[wasm_bindgen(method, getter, structural)]
    pub fn era(this: &PlainDate) -> Option<JsString>;

    /// The `eraYear` accessor property returns a non-negative integer representing the
    /// year of this date within the era, or `None` if the calendar does not use eras.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/eraYear)
    #[wasm_bindgen(method, getter, structural, js_name = eraYear)]
    pub fn era_year(this: &PlainDate) -> Option<i32>;

    /// The `year` accessor property returns an integer representing the number of years
    /// of this date relative to the start of a calendar-specific epoch year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/year)
    #[wasm_bindgen(method, getter, structural)]
    pub fn year(this: &PlainDate) -> i32;

    /// The `month` accessor property returns a positive integer representing the
    /// 1-based month index in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/month)
    #[wasm_bindgen(method, getter, structural)]
    pub fn month(this: &PlainDate) -> u32;

    /// The `monthCode` accessor property returns a calendar-specific string representing
    /// the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/monthCode)
    #[wasm_bindgen(method, getter, structural, js_name = monthCode)]
    pub fn month_code(this: &PlainDate) -> JsString;

    /// The `day` accessor property returns a positive integer representing the
    /// 1-based day index in the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/day)
    #[wasm_bindgen(method, getter, structural)]
    pub fn day(this: &PlainDate) -> u32;

    /// The `calendarId` accessor property returns a string representing the calendar
    /// used to interpret the internal ISO 8601 date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/calendarId)
    #[wasm_bindgen(method, getter, structural, js_name = calendarId)]
    pub fn calendar_id(this: &PlainDate) -> JsString;

    /// The `dayOfWeek` accessor property returns a positive integer representing the
    /// day of the week of this date (1 = Monday through 7 = Sunday for ISO 8601).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/dayOfWeek)
    #[wasm_bindgen(method, getter, structural, js_name = dayOfWeek)]
    pub fn day_of_week(this: &PlainDate) -> u32;

    /// The `dayOfYear` accessor property returns a positive integer representing the
    /// 1-based day index of this date in the year.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/dayOfYear)
    #[wasm_bindgen(method, getter, structural, js_name = dayOfYear)]
    pub fn day_of_year(this: &PlainDate) -> u32;

    /// The `weekOfYear` accessor property returns a positive integer representing the
    /// 1-based week index in the `yearOfWeek` of this date, or `None` if the
    /// calendar does not have a well-defined week system.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/weekOfYear)
    #[wasm_bindgen(method, getter, structural, js_name = weekOfYear)]
    pub fn week_of_year(this: &PlainDate) -> Option<u32>;

    /// The `yearOfWeek` accessor property returns an integer representing the year
    /// to be paired with `weekOfYear`, or `None` if the calendar does not have
    /// a well-defined week system.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/yearOfWeek)
    #[wasm_bindgen(method, getter, structural, js_name = yearOfWeek)]
    pub fn year_of_week(this: &PlainDate) -> Option<i32>;

    /// The `daysInWeek` accessor property returns a positive integer representing the
    /// number of days in the week of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/daysInWeek)
    #[wasm_bindgen(method, getter, structural, js_name = daysInWeek)]
    pub fn days_in_week(this: &PlainDate) -> u32;

    /// The `daysInMonth` accessor property returns a positive integer representing the
    /// number of days in the month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/daysInMonth)
    #[wasm_bindgen(method, getter, structural, js_name = daysInMonth)]
    pub fn days_in_month(this: &PlainDate) -> u32;

    /// The `daysInYear` accessor property returns a positive integer representing the
    /// number of days in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/daysInYear)
    #[wasm_bindgen(method, getter, structural, js_name = daysInYear)]
    pub fn days_in_year(this: &PlainDate) -> u32;

    /// The `monthsInYear` accessor property returns a positive integer representing the
    /// number of months in the year of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/monthsInYear)
    #[wasm_bindgen(method, getter, structural, js_name = monthsInYear)]
    pub fn months_in_year(this: &PlainDate) -> u32;

    /// The `inLeapYear` accessor property returns `true` if this date is in a leap year,
    /// and `false` otherwise.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/inLeapYear)
    #[wasm_bindgen(method, getter, structural, js_name = inLeapYear)]
    pub fn in_leap_year(this: &PlainDate) -> bool;

    /// The `equals()` method returns `true` if this date is equivalent to `other`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/equals)
    #[wasm_bindgen(method, catch)]
    pub fn equals(this: &PlainDate, other: &PlainDate) -> Result<bool, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainDate` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/with)
    #[wasm_bindgen(method, catch)]
    pub fn with(this: &PlainDate, info: &Object) -> Result<PlainDate, JsValue>;

    /// The `with()` method returns a new `Temporal.PlainDate` object with some
    /// fields replaced by new values.
    ///
    /// # Errors
    /// Throws a `RangeError` if values are out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/with)
    #[wasm_bindgen(js_name = with, method, catch)]
    pub fn with_with_options(
        this: &PlainDate,
        info: &Object,
        options: &AssignmentOptions,
    ) -> Result<PlainDate, JsValue>;

    /// The `withCalendar()` method returns a new `Temporal.PlainDate` object with
    /// the calendar replaced.
    ///
    /// # Errors
    /// Throws a `RangeError` if the calendar identifier is invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/withCalendar)
    #[wasm_bindgen(method, js_name = withCalendar, catch)]
    pub fn with_calendar(this: &PlainDate, calendar: &str) -> Result<PlainDate, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainDate` object with the
    /// duration added to this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/add)
    #[wasm_bindgen(method, catch)]
    pub fn add(this: &PlainDate, duration: &Duration) -> Result<PlainDate, JsValue>;

    /// The `add()` method returns a new `Temporal.PlainDate` object with the
    /// duration added to this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/add)
    #[wasm_bindgen(js_name = add, method, catch)]
    pub fn add_with_options(
        this: &PlainDate,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<PlainDate, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainDate` object with the
    /// duration subtracted from this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/subtract)
    #[wasm_bindgen(method, catch)]
    pub fn subtract(this: &PlainDate, duration: &Duration) -> Result<PlainDate, JsValue>;

    /// The `subtract()` method returns a new `Temporal.PlainDate` object with the
    /// duration subtracted from this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if the result is out of range (when `overflow` is `'reject'`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/subtract)
    #[wasm_bindgen(js_name = subtract, method, catch)]
    pub fn subtract_with_options(
        this: &PlainDate,
        duration: &Duration,
        options: &ArithmeticOptions,
    ) -> Result<PlainDate, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this date until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/until)
    #[wasm_bindgen(method, catch)]
    pub fn until(this: &PlainDate, other: &PlainDate) -> Result<Duration, JsValue>;

    /// The `until()` method returns a `Temporal.Duration` representing the duration
    /// from this date until `other`.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/until)
    #[wasm_bindgen(js_name = until, method, catch)]
    pub fn until_with_options(
        this: &PlainDate,
        other: &PlainDate,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/since)
    #[wasm_bindgen(method, catch)]
    pub fn since(this: &PlainDate, other: &PlainDate) -> Result<Duration, JsValue>;

    /// The `since()` method returns a `Temporal.Duration` representing the duration
    /// from `other` until this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if dates have different calendars or options are invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/since)
    #[wasm_bindgen(js_name = since, method, catch)]
    pub fn since_with_options(
        this: &PlainDate,
        other: &PlainDate,
        options: &DifferenceOptions,
    ) -> Result<Duration, JsValue>;

    /// The `toPlainDateTime()` method returns a `Temporal.PlainDateTime` object representing
    /// this date at the given time (or midnight if not provided).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainDateTime)
    #[wasm_bindgen(method, js_name = toPlainDateTime, catch)]
    pub fn to_plain_date_time(this: &PlainDate) -> Result<PlainDateTime, JsValue>;

    /// The `toPlainDateTime()` method returns a `Temporal.PlainDateTime` object representing
    /// this date at the given time (or midnight if not provided).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainDateTime)
    #[wasm_bindgen(method, js_name = toPlainDateTime, catch)]
    pub fn to_plain_date_time_with_plain_time(
        this: &PlainDate,
        plain_time: &PlainTime,
    ) -> Result<PlainDateTime, JsValue>;

    /// The `toZonedDateTime()` method returns a `Temporal.ZonedDateTime` object representing
    /// this date at the given time in the given time zone.
    ///
    /// `item` is either a time zone identifier string, or an object with a
    /// `timeZone` property and an optional `plainTime` property.
    ///
    /// # Errors
    /// Throws a `RangeError` if the time zone identifier is invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toZonedDateTime)
    #[wasm_bindgen(method, js_name = toZonedDateTime, catch)]
    pub fn to_zoned_date_time_with_str(
        this: &PlainDate,
        time_zone: &str,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `toZonedDateTime()` method returns a `Temporal.ZonedDateTime` object representing
    /// this date at the given time in the given time zone.
    ///
    /// `item` is either a time zone identifier string, or an object with a
    /// `timeZone` property and an optional `plainTime` property.
    ///
    /// # Errors
    /// Throws a `RangeError` if the time zone identifier is invalid.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toZonedDateTime)
    #[wasm_bindgen(method, js_name = toZonedDateTime, catch)]
    pub fn to_zoned_date_time_with_object(
        this: &PlainDate,
        item: &Object,
    ) -> Result<ZonedDateTime, JsValue>;

    /// The `toPlainYearMonth()` method returns a `Temporal.PlainYearMonth` object representing
    /// the year and month of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainYearMonth)
    #[wasm_bindgen(method, js_name = toPlainYearMonth)]
    pub fn to_plain_year_month(this: &PlainDate) -> PlainYearMonth;

    /// The `toPlainMonthDay()` method returns a `Temporal.PlainMonthDay` object representing
    /// the month and day of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainMonthDay)
    #[wasm_bindgen(method, js_name = toPlainMonthDay)]
    pub fn to_plain_month_day(this: &PlainDate) -> PlainMonthDay;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string(this: &PlainDate, locales: &[JsString]) -> Result<JsString, JsValue>;

    /// The `toLocaleString()` method returns a language-sensitive string representation
    /// of this date.
    ///
    /// # Errors
    /// Throws a `RangeError` if `locales` is invalid or the locale's calendar does
    /// not match this value's calendar, and a `TypeError` for options that do not
    /// apply to this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toLocaleString)
    #[wasm_bindgen(method, js_name = toLocaleString, catch)]
    pub fn to_locale_string_with_options(
        this: &PlainDate,
        locales: &[JsString],
        options: &super::Intl::DateTimeFormatOptions,
    ) -> Result<JsString, JsValue>;

    /// The `toJSON()` method returns a string representation of this date
    /// suitable for JSON serialization.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toJSON)
    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &PlainDate) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this date.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toString)
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_js_string(this: &PlainDate) -> JsString;

    /// The `toString()` method returns an RFC 9557 string representation of this date,
    /// formatted according to `options`.
    ///
    /// # Errors
    /// Throws a `RangeError` if the options are invalid for this type.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toString)
    #[wasm_bindgen(method, js_name = toString, catch)]
    pub fn to_js_string_with_options(
        this: &PlainDate,
        options: &ShowCalendarOptions,
    ) -> Result<JsString, JsValue>;
}

/// The `Temporal.Now` object has several methods which give information about
/// the current date, time, and time zone.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now)
pub mod Now {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        /// The `Temporal.Now.instant()` static method returns the current exact time
        /// as a `Temporal.Instant`.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/instant)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"])]
        pub fn instant() -> Instant;

        /// The `Temporal.Now.timeZoneId()` static method returns the identifier of
        /// the system's current time zone.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/timeZoneId)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = timeZoneId)]
        pub fn time_zone_id() -> JsString;

        /// The `Temporal.Now.zonedDateTimeISO()` static method returns the current date and time as a
        /// `Temporal.ZonedDateTime` in the ISO 8601 calendar and
        /// the system's current time zone.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/zonedDateTimeISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = zonedDateTimeISO)]
        pub fn zoned_date_time_iso() -> ZonedDateTime;

        /// The `Temporal.Now.zonedDateTimeISO()` static method returns the current date and time as a
        /// `Temporal.ZonedDateTime` in the ISO 8601 calendar and
        /// the given time zone.
        ///
        /// # Errors
        /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/zonedDateTimeISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = zonedDateTimeISO, catch)]
        pub fn zoned_date_time_iso_with_time_zone(
            time_zone: &str,
        ) -> Result<ZonedDateTime, JsValue>;

        /// The `Temporal.Now.plainDateTimeISO()` static method returns the current date and wall-clock time
        /// as a `Temporal.PlainDateTime` in the ISO 8601 calendar and
        /// the system's current time zone.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateTimeISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = plainDateTimeISO)]
        pub fn plain_date_time_iso() -> PlainDateTime;

        /// The `Temporal.Now.plainDateTimeISO()` static method returns the current date and wall-clock time
        /// as a `Temporal.PlainDateTime` in the ISO 8601 calendar and
        /// the given time zone.
        ///
        /// # Errors
        /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateTimeISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = plainDateTimeISO, catch)]
        pub fn plain_date_time_iso_with_time_zone(
            time_zone: &str,
        ) -> Result<PlainDateTime, JsValue>;

        /// The `Temporal.Now.plainDateISO()` static method returns the current date as a `Temporal.PlainDate`
        /// in the ISO 8601 calendar and
        /// the system's current time zone.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = plainDateISO)]
        pub fn plain_date_iso() -> PlainDate;

        /// The `Temporal.Now.plainDateISO()` static method returns the current date as a `Temporal.PlainDate`
        /// in the ISO 8601 calendar and
        /// the given time zone.
        ///
        /// # Errors
        /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = plainDateISO, catch)]
        pub fn plain_date_iso_with_time_zone(time_zone: &str) -> Result<PlainDate, JsValue>;

        /// The `Temporal.Now.plainTimeISO()` static method returns the current wall-clock time as a
        /// `Temporal.PlainTime`
        /// in the system's current time zone.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainTimeISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = plainTimeISO)]
        pub fn plain_time_iso() -> PlainTime;

        /// The `Temporal.Now.plainTimeISO()` static method returns the current wall-clock time as a
        /// `Temporal.PlainTime`
        /// in the given time zone.
        ///
        /// # Errors
        /// Throws a `RangeError` if `time_zone` is not a valid time zone identifier.
        ///
        /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainTimeISO)
        #[wasm_bindgen(js_namespace = ["Temporal", "Now"], js_name = plainTimeISO, catch)]
        pub fn plain_time_iso_with_time_zone(time_zone: &str) -> Result<PlainTime, JsValue>;
    }
}
