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
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
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
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDd),
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
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDd),
        "2024-03-01"
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
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
}

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
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
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
