/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::formatter::{pattern::DateTimePattern, DateTimeFormatter, DefaultDateTimeFormatter};
use crate::TimeUnit;

// ---------------------------------------------------------------- date-time pattern

#[test]
fn test_date_time_formatter_format_default() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::default();

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
}

#[test]
fn test_date_time_formatter_format_new() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
}

#[test]
fn test_date_time_formatter_format_yyyy_mm_dd() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
}

#[test]
fn test_date_time_formatter_format_mm_dd_yyyy() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::MmDdYyyy);

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "03/01/2024"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::MmDdYyyy),
        "03/01/2024"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::MmDdYyyy),
        "03/01/2024"
    );
}

#[test]
fn test_date_time_formatter_format_dd_mm_yyyy() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::DdMmYyyy);

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "01-03-2024"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::DdMmYyyy),
        "01-03-2024"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::DdMmYyyy),
        "01-03-2024"
    );
}

#[test]
fn test_date_time_formatter_format_yyyy_mm_dd_hh_mm() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMm);

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01 02:03"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMm),
        "2024-03-01 02:03"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMm),
        "2024-03-01 02:03"
    );
}

#[test]
fn test_date_time_formatter_format_yyyy_mm_dd_hh_mm_ss() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSs);
    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
}

#[test]
fn test_date_time_formatter_format_yyyy_mm_dd_hh_mm_ss_sss() {
    let now = "2024-03-01 02:03:04.789";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S%.3f").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSsSss);
    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01 02:03:04.789"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSsSss),
        "2024-03-01 02:03:04.789"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMmSsSss),
        "2024-03-01 02:03:04.789"
    );
}

#[test]
fn test_date_time_formatter_format_hh_mm() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::HhMm);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "02:03");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::HhMm),
        "02:03"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::HhMm),
        "02:03"
    );
}

#[test]
fn test_date_time_formatter_format_hh_mm_ss() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::HhMmSs);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "02:03:04");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::HhMmSs),
        "02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::HhMmSs),
        "02:03:04"
    );
}

#[test]
fn test_date_time_formatter_format_month_full() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::MonthFull);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "March");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::MonthFull),
        "March"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::MonthFull),
        "March"
    );
}

#[test]
fn test_date_time_formatter_format_month_abbr() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::MonthAbbr);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "Mar");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::MonthAbbr),
        "Mar"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::MonthAbbr),
        "Mar"
    );
}

#[test]
fn test_date_time_formatter_format_weekday_full() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::WeekdayFull);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "Friday");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::WeekdayFull),
        "Friday"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::WeekdayFull),
        "Friday"
    );
}

#[test]
fn test_date_time_formatter_format_weekday_abbr() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::WeekdayAbbr);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "Fri");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::WeekdayAbbr),
        "Fri"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::WeekdayAbbr),
        "Fri"
    );
}

#[test]
fn test_date_time_formatter_format_am_pm_am() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::AmPm);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "AM");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::AmPm),
        "AM"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::AmPm),
        "AM"
    );
}

#[test]
fn test_date_time_formatter_format_am_pm_pm() {
    let now = "2024-03-01 12:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::AmPm);
    assert_eq!(dtf.default_format_date_time_utc(&datetime_utc), "PM");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::AmPm),
        "PM"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::AmPm),
        "PM"
    );
}

#[test]
fn test_date_time_formatter_format_timestamp() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::Timestamp);
    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "1709258584"
    ); // +8 -> 2024-03-01 10:03:04
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::Timestamp),
        "1709258584"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::Timestamp),
        "1709258584"
    );
}

// ----------------------------------------------------------------

#[test]
fn test_date_time_formatter_naive_date_time_format_yyyy_mm_dd() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

    assert_eq!(dtf.default_format_naive_date_time(&ndt), "2024-03-01");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
}

// ----------------------------------------------------------------

#[test]
fn test_date_time_formatter_of_pattern() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);
    let dtf = dtf.of_pattern(DateTimePattern::YyyyMmDdHhMmSs);

    assert_eq!(
        dtf.default_format_date_time_utc(&datetime_utc),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
}

// ---------------------------------------------------------------- time-unit

#[test]
fn test_time_unit_to_nanoseconds() {
    assert_eq!(TimeUnit::Nanoseconds.to_nanos(1024), 1024);
    assert_eq!(
        TimeUnit::Microseconds.to_nanos(1024),
        1024 * TimeUnit::NANOS_PER_MICROSECOND
    );
    assert_eq!(
        TimeUnit::Milliseconds.to_nanos(1024),
        1024 * TimeUnit::NANOS_PER_MILLISECOND
    );
    assert_eq!(
        TimeUnit::Seconds.to_nanos(1024),
        1024 * TimeUnit::NANOS_PER_SECOND
    );
    assert_eq!(
        TimeUnit::Minutes.to_nanos(1024),
        1024 * TimeUnit::NANOS_PER_MINUTE
    );
    assert_eq!(
        TimeUnit::Hours.to_nanos(1024),
        1024 * TimeUnit::NANOS_PER_HOUR
    );
    assert_eq!(
        TimeUnit::Days.to_nanos(1024),
        1024 * TimeUnit::NANOS_PER_DAY
    );
}
