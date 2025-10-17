exports.is_temporal_supported = function () {
    return typeof Temporal === 'object';
};

exports.get_test_instant = function () {
    return Temporal.Instant.from('2022-06-15T12:30:45.123456789Z');
};

exports.get_test_zoned_date_time = function () {
    return Temporal.ZonedDateTime.from('2022-06-15T12:30:45.123456789-07:00[America/Los_Angeles]');
};

exports.get_test_plain_date_time = function () {
    return Temporal.PlainDateTime.from('2022-06-15T12:30:45.123456789');
};

exports.get_test_plain_date = function () {
    return Temporal.PlainDate.from('2022-06-15');
};

exports.get_test_plain_time = function () {
    return Temporal.PlainTime.from('12:30:45.123456789');
};

exports.get_test_plain_year_month = function () {
    return Temporal.PlainYearMonth.from('2022-06');
};

exports.get_test_plain_month_day = function () {
    return Temporal.PlainMonthDay.from('06-15');
};

exports.get_test_duration = function () {
    return Temporal.Duration.from('P1Y2M3DT4H5M6.007008009S');
};

// Helper to create objects for with() methods
exports.create_day_object = function (day) {
    return { day: day };
};

exports.create_year_object = function (year) {
    return { year: year };
};
