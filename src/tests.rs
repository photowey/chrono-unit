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
fn test_date_time_formatter_format_builtin() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::builtin();

    assert_eq!(
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc_default(&ndt),
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "2024-03-01");
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDd),
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc_default(&ndt),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
}

// ----------------------------------------------------------------

#[test]
fn test_date_time_formatter_format_yyyy_mm_dd() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

    assert_eq!(
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "2024-03-01");
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "03/01/2024"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::MmDdYyyy),
        "03/01/2024"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "03/01/2024");
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "01-03-2024"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::DdMmYyyy),
        "01-03-2024"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "01-03-2024");
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01 02:03"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMm),
        "2024-03-01 02:03"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc_default(&ndt),
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc_default(&ndt),
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
        dtf.format_date_time_utc_default(&datetime_utc),
        "2024-03-01 02:03:04.789"
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSsSss),
        "2024-03-01 02:03:04.789"
    );
    assert_eq!(
        dtf.format_naive_date_time_utc_default(&ndt),
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "02:03");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::HhMm),
        "02:03"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "02:03");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "02:03:04");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::HhMmSs),
        "02:03:04"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "02:03:04");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "March");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::MonthFull),
        "March"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "March");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "Mar");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::MonthAbbr),
        "Mar"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "Mar");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "Friday");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::WeekdayFull),
        "Friday"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "Friday");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "Fri");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::WeekdayAbbr),
        "Fri"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "Fri");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "AM");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::AmPm),
        "AM"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "AM");
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
    assert_eq!(dtf.format_date_time_utc_default(&datetime_utc), "PM");
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::AmPm),
        "PM"
    );
    assert_eq!(dtf.format_naive_date_time_utc_default(&ndt), "PM");
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

    let timestamp_ndt = ndt.timestamp();
    let timestamp_utc = datetime_utc.timestamp();

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::Timestamp);
    assert_eq!(
        dtf.format_date_time_utc_default(&datetime_utc),
        timestamp_utc.to_string()
    );
    assert_eq!(
        dtf.format_date_time_utc(&datetime_utc, DateTimePattern::Timestamp),
        timestamp_utc.to_string()
    );
    assert_eq!(
        dtf.format_naive_date_time_utc_default(&ndt),
        timestamp_ndt.to_string()
    );
    assert_eq!(
        dtf.format_naive_date_time_utc(&ndt, DateTimePattern::Timestamp),
        timestamp_ndt.to_string()
    );
}

// ----------------------------------------------------------------

#[test]
fn test_date_time_formatter_naive_date_time_format_yyyy_mm_dd() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "2024-03-01");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDd),
        "2024-03-01"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_mm_dd_yyyy() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::MmDdYyyy);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "03/01/2024");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::MmDdYyyy),
        "03/01/2024"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_dd_mm_yyyy() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::DdMmYyyy);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "01-03-2024");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::DdMmYyyy),
        "01-03-2024"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_yyyy_mm_dd_hh_mm() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMm);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "2024-03-01 02:03");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMm),
        "2024-03-01 02:03"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_yyyy_mm_dd_hh_mm_ss() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSs);

    assert_eq!(
        dtf.format_naive_date_time_default(&ndt),
        "2024-03-01 02:03:04"
    );
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
        "2024-03-01 02:03:04"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_yyyy_mm_dd_hh_mm_ss_sss() {
    let now = "2024-03-01 02:03:04.789";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S%.3f").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSsSss);

    assert_eq!(
        dtf.format_naive_date_time_default(&ndt),
        "2024-03-01 02:03:04.789"
    );
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSsSss),
        "2024-03-01 02:03:04.789"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_hh_mm() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::HhMm);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "02:03");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::HhMm),
        "02:03"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_hh_mm_ss() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::HhMmSs);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "02:03:04");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::HhMmSs),
        "02:03:04"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_month_full() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::MonthFull);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "March");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::MonthFull),
        "March"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_month_abbr() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::MonthAbbr);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "Mar");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::MonthAbbr),
        "Mar"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_weekday_full() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::WeekdayFull);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "Friday");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::WeekdayFull),
        "Friday"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_weekday_abbr() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::WeekdayAbbr);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "Fri");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::WeekdayAbbr),
        "Fri"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_am_pm_am() {
    let now = "2024-03-01 02:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::AmPm);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "AM");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::AmPm),
        "AM"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_am_pm_pm() {
    let now = "2024-03-01 12:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::AmPm);

    assert_eq!(dtf.format_naive_date_time_default(&ndt), "PM");
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::AmPm),
        "PM"
    );
}

#[test]
fn test_date_time_formatter_naive_date_time_format_timestamp() {
    let now = "2024-03-01 12:03:04";
    let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
    let timestamp_ndt = ndt.timestamp();

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::Timestamp);

    assert_eq!(
        dtf.format_naive_date_time_default(&ndt),
        timestamp_ndt.to_string()
    );
    assert_eq!(
        dtf.format_naive_date_time(&ndt, DateTimePattern::Timestamp),
        timestamp_ndt.to_string()
    );
}

// ----------------------------------------------------------------

#[test]
fn test_date_time_pattern_pattern_of() {
    assert_eq!(
        DateTimePattern::YyyyMmDd.pattern_of(),
        DateTimePattern::YYYY_MM_DD
    );
    assert_eq!(
        DateTimePattern::MmDdYyyy.pattern_of(),
        DateTimePattern::MM_DD_YYYY
    );
    assert_eq!(
        DateTimePattern::DdMmYyyy.pattern_of(),
        DateTimePattern::DD_MM_YYYY
    );
    assert_eq!(
        DateTimePattern::YyyyMmDdHhMm.pattern_of(),
        DateTimePattern::YYYY_MM_DD_HH_MM
    );
    assert_eq!(
        DateTimePattern::YyyyMmDdHhMmSs.pattern_of(),
        DateTimePattern::YYYY_MM_DD_HH_MM_SS
    );
    assert_eq!(
        DateTimePattern::YyyyMmDdHhMmSsSss.pattern_of(),
        DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS
    );
    assert_eq!(DateTimePattern::HhMm.pattern_of(), DateTimePattern::HH_MM);
    assert_eq!(
        DateTimePattern::HhMmSs.pattern_of(),
        DateTimePattern::HH_MM_SS
    );
    assert_eq!(
        DateTimePattern::MonthFull.pattern_of(),
        DateTimePattern::MONTH_FULL
    );
    assert_eq!(
        DateTimePattern::MonthAbbr.pattern_of(),
        DateTimePattern::MONTH_ABBR
    );
    assert_eq!(
        DateTimePattern::WeekdayFull.pattern_of(),
        DateTimePattern::WEEKDAY_FULL
    );
    assert_eq!(
        DateTimePattern::WeekdayAbbr.pattern_of(),
        DateTimePattern::WEEKDAY_ABBR
    );
    assert_eq!(DateTimePattern::AmPm.pattern_of(), DateTimePattern::AM_PM);
    assert_eq!(
        DateTimePattern::Timestamp.pattern_of(),
        DateTimePattern::TIMESTAMP
    );
}

#[test]
fn test_date_time_pattern_value_of() {
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::YYYY_MM_DD),
        Some(DateTimePattern::YyyyMmDd)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::MM_DD_YYYY),
        Some(DateTimePattern::MmDdYyyy)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::DD_MM_YYYY),
        Some(DateTimePattern::DdMmYyyy)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::YYYY_MM_DD_HH_MM),
        Some(DateTimePattern::YyyyMmDdHhMm)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::YYYY_MM_DD_HH_MM_SS),
        Some(DateTimePattern::YyyyMmDdHhMmSs)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS),
        Some(DateTimePattern::YyyyMmDdHhMmSsSss)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::HH_MM),
        Some(DateTimePattern::HhMm)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::HH_MM_SS),
        Some(DateTimePattern::HhMmSs)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::MONTH_FULL),
        Some(DateTimePattern::MonthFull)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::MONTH_ABBR),
        Some(DateTimePattern::MonthAbbr)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::WEEKDAY_FULL),
        Some(DateTimePattern::WeekdayFull)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::WEEKDAY_ABBR),
        Some(DateTimePattern::WeekdayAbbr)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::AM_PM),
        Some(DateTimePattern::AmPm)
    );
    assert_eq!(
        DateTimePattern::value_of(DateTimePattern::TIMESTAMP),
        Some(DateTimePattern::Timestamp)
    );
    assert_eq!(DateTimePattern::value_of("Invalid"), None);
}

#[test]
fn test_date_time_pattern_name_of() {
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::YYYY_MM_DD_NAME),
        Some(DateTimePattern::YyyyMmDd)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::MM_DD_YYYY_NAME),
        Some(DateTimePattern::MmDdYyyy)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::DD_MM_YYYY_NAME),
        Some(DateTimePattern::DdMmYyyy)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::YYYY_MM_DD_HH_MM_NAME),
        Some(DateTimePattern::YyyyMmDdHhMm)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::YYYY_MM_DD_HH_MM_SS_NAME),
        Some(DateTimePattern::YyyyMmDdHhMmSs)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS_NAME),
        Some(DateTimePattern::YyyyMmDdHhMmSsSss)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::HH_MM_NAME),
        Some(DateTimePattern::HhMm)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::HH_MM_SS_NAME),
        Some(DateTimePattern::HhMmSs)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::MONTH_FULL_NAME),
        Some(DateTimePattern::MonthFull)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::MONTH_ABBR_NAME),
        Some(DateTimePattern::MonthAbbr)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::WEEKDAY_FULL_NAME),
        Some(DateTimePattern::WeekdayFull)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::WEEKDAY_ABBR_NAME),
        Some(DateTimePattern::WeekdayAbbr)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::AM_PM_NAME),
        Some(DateTimePattern::AmPm)
    );
    assert_eq!(
        DateTimePattern::name_of(DateTimePattern::TIMESTAMP_NAME),
        Some(DateTimePattern::Timestamp)
    );
    assert_eq!(DateTimePattern::name_of("Invalid"), None);
}

// ---------------------------------------------------------------- time-unit

#[test]
fn test_time_unit_to_nanos() {
    assert_eq!(TimeUnit::Nanoseconds.to_nanos(1024), 1024);
    assert_eq!(TimeUnit::Microseconds.to_nanos(1024), 1024 * 1000);
    assert_eq!(TimeUnit::Milliseconds.to_nanos(1024), 1024 * (1000 * 1000));
    assert_eq!(
        TimeUnit::Seconds.to_nanos(1024),
        1024 * (1000 * 1000 * 1000)
    );
    assert_eq!(
        TimeUnit::Minutes.to_nanos(1024),
        1024 * (1000 * 1000 * 1000 * 60)
    );
    assert_eq!(
        TimeUnit::Hours.to_nanos(1024),
        1024 * (1000 * 1000 * 1000 * 60 * 60)
    );
    assert_eq!(
        TimeUnit::Days.to_nanos(1024),
        1024 * (1000 * 1000 * 1000 * 60 * 60 * 24)
    );
}

#[test]
fn test_time_unit_to_micros() {
    assert_eq!(TimeUnit::Nanoseconds.to_micros(1024), 1024 / 1000);
    assert_eq!(TimeUnit::Microseconds.to_micros(1024), 1024);
    assert_eq!(TimeUnit::Milliseconds.to_micros(1024), 1024 * 1000);
    assert_eq!(TimeUnit::Seconds.to_micros(1024), 1024 * (1000 * 1000));
    assert_eq!(TimeUnit::Minutes.to_micros(1024), 1024 * (1000 * 1000 * 60));
    assert_eq!(
        TimeUnit::Hours.to_micros(1024),
        1024 * (1000 * 1000 * 60 * 60)
    );
    assert_eq!(
        TimeUnit::Days.to_micros(1024),
        1024 * (1000 * 1000 * 60 * 60 * 24)
    );
}

#[test]
fn test_time_unit_to_millis() {
    assert_eq!(TimeUnit::Nanoseconds.to_millis(1024), 1024 / 1000 / 1000);
    assert_eq!(TimeUnit::Microseconds.to_millis(1024), 1024 / 1000);
    assert_eq!(TimeUnit::Milliseconds.to_millis(1024), 1024);
    assert_eq!(TimeUnit::Seconds.to_millis(1024), 1024 * 1000);
    assert_eq!(TimeUnit::Minutes.to_millis(1024), 1024 * (1000 * 60));
    assert_eq!(TimeUnit::Hours.to_millis(1024), 1024 * (1000 * 60 * 60));
    assert_eq!(TimeUnit::Days.to_millis(1024), 1024 * (1000 * 60 * 60 * 24));
}

#[test]
fn test_time_unit_to_seconds() {
    assert_eq!(
        TimeUnit::Nanoseconds.to_seconds(1024),
        1024 / 1000 / 1000 / 1000
    );
    assert_eq!(TimeUnit::Microseconds.to_seconds(1024), 1024 / 1000 / 1000);
    assert_eq!(TimeUnit::Milliseconds.to_seconds(1024), 1024 / 1000);
    assert_eq!(TimeUnit::Seconds.to_seconds(1024), 1024);
    assert_eq!(TimeUnit::Minutes.to_seconds(1024), 1024 * 60);
    assert_eq!(TimeUnit::Hours.to_seconds(1024), 1024 * (60 * 60));
    assert_eq!(TimeUnit::Days.to_seconds(1024), 1024 * (60 * 60 * 24));
}

#[test]
fn test_time_unit_to_minutes() {
    assert_eq!(
        TimeUnit::Nanoseconds.to_minutes(1024),
        1024 / 1000 / 1000 / 1000 / 60
    );
    assert_eq!(
        TimeUnit::Microseconds.to_minutes(1024),
        1024 / 1000 / 1000 / 60
    );
    assert_eq!(TimeUnit::Milliseconds.to_minutes(1024), 1024 / 1000 / 60);
    assert_eq!(TimeUnit::Seconds.to_minutes(1024), 1024 / 60);
    assert_eq!(TimeUnit::Minutes.to_minutes(1024), 1024);
    assert_eq!(TimeUnit::Hours.to_minutes(1024), 1024 * 60);
    assert_eq!(TimeUnit::Days.to_minutes(1024), 1024 * (60 * 24));
}

#[test]
fn test_time_unit_to_hours() {
    assert_eq!(
        TimeUnit::Nanoseconds.to_hours(1024),
        1024 / 1000 / 1000 / 1000 / 60 / 60
    );
    assert_eq!(
        TimeUnit::Microseconds.to_hours(1024),
        1024 / 1000 / 1000 / 60 / 60
    );
    assert_eq!(TimeUnit::Milliseconds.to_hours(1024), 1024 / 1000 / 60 / 60);
    assert_eq!(TimeUnit::Seconds.to_hours(1024), 1024 / 60 / 60);
    assert_eq!(TimeUnit::Minutes.to_hours(1024), 1024 / 60);
    assert_eq!(TimeUnit::Hours.to_hours(1024), 1024);
    assert_eq!(TimeUnit::Days.to_hours(1024), 1024 * 24);
}

#[test]
fn test_time_unit_to_days() {
    assert_eq!(
        TimeUnit::Nanoseconds.to_days(1024),
        1024 / 1000 / 1000 / 1000 / 60 / 60 / 24
    );
    assert_eq!(
        TimeUnit::Microseconds.to_days(1024),
        1024 / 1000 / 1000 / 60 / 60 / 24
    );
    assert_eq!(
        TimeUnit::Milliseconds.to_days(1024),
        1024 / 1000 / 60 / 60 / 24
    );
    assert_eq!(TimeUnit::Seconds.to_days(1024), 1024 / 60 / 60 / 24);
    assert_eq!(TimeUnit::Minutes.to_days(1024), 1024 / 60 / 24);
    assert_eq!(TimeUnit::Hours.to_days(1024), 1024 / 24);
    assert_eq!(TimeUnit::Days.to_days(1024), 1024);
}

// ----------------------------------------------------------------

#[test]
fn test_time_unit_nanoseconds() {
    assert_eq!(TimeUnit::Nanoseconds.to_nanos(1024), 1024);
    assert_eq!(TimeUnit::Nanoseconds.to_micros(1024), 1024 / 1000);
    assert_eq!(TimeUnit::Nanoseconds.to_millis(1024), 1024 / 1000 / 1000);
    assert_eq!(
        TimeUnit::Nanoseconds.to_seconds(1024),
        1024 / 1000 / 1000 / 1000
    );
    assert_eq!(
        TimeUnit::Nanoseconds.to_minutes(1024),
        1024 / 1000 / 1000 / 1000 / 60
    );
    assert_eq!(
        TimeUnit::Nanoseconds.to_hours(1024),
        1024 / 1000 / 1000 / 1000 / 60 / 60
    );
    assert_eq!(
        TimeUnit::Nanoseconds.to_days(1024),
        1024 / 1000 / 1000 / 1000 / 60 / 60 / 24
    );
}

#[test]
fn test_time_unit_microseconds() {
    assert_eq!(TimeUnit::Microseconds.to_nanos(1024), 1024 * 1000);
    assert_eq!(TimeUnit::Microseconds.to_micros(1024), 1024);
    assert_eq!(TimeUnit::Microseconds.to_millis(1024), 1024 / 1000);
    assert_eq!(TimeUnit::Microseconds.to_seconds(1024), 1024 / 1000 / 1000);
    assert_eq!(
        TimeUnit::Microseconds.to_minutes(1024),
        1024 / 1000 / 1000 / 60
    );
    assert_eq!(
        TimeUnit::Microseconds.to_hours(1024),
        1024 / 1000 / 1000 / 60 / 60
    );
    assert_eq!(
        TimeUnit::Microseconds.to_days(1024),
        1024 / 1000 / 1000 / 60 / 60 / 24
    );
}

#[test]
fn test_time_unit_milliseconds() {
    assert_eq!(TimeUnit::Milliseconds.to_nanos(1024), 1024 * 1000 * 1000);
    assert_eq!(TimeUnit::Milliseconds.to_micros(1024), 1024 * 1000);
    assert_eq!(TimeUnit::Milliseconds.to_millis(1024), 1024);
    assert_eq!(TimeUnit::Milliseconds.to_seconds(1024), 1024 / 1000);
    assert_eq!(TimeUnit::Milliseconds.to_minutes(1024), 1024 / 1000 / 60);
    assert_eq!(TimeUnit::Milliseconds.to_hours(1024), 1024 / 1000 / 60 / 60);
    assert_eq!(
        TimeUnit::Milliseconds.to_days(1024),
        1024 / 1000 / 60 / 60 / 24
    );
}

#[test]
fn test_time_unit_seconds() {
    assert_eq!(TimeUnit::Seconds.to_nanos(1024), 1024 * 1000 * 1000 * 1000);
    assert_eq!(TimeUnit::Seconds.to_micros(1024), 1024 * 1000 * 1000);
    assert_eq!(TimeUnit::Seconds.to_millis(1024), 1024 * 1000);
    assert_eq!(TimeUnit::Seconds.to_seconds(1024), 1024);
    assert_eq!(TimeUnit::Seconds.to_minutes(1024), 1024 / 60);
    assert_eq!(TimeUnit::Seconds.to_hours(1024), 1024 / 60 / 60);
    assert_eq!(TimeUnit::Seconds.to_days(1024), 1024 / 60 / 60 / 24);
}

#[test]
fn test_time_unit_minutes() {
    assert_eq!(
        TimeUnit::Minutes.to_nanos(1024),
        1024 * 1000 * 1000 * 1000 * 60
    );
    assert_eq!(TimeUnit::Minutes.to_micros(1024), 1024 * 1000 * 1000 * 60);
    assert_eq!(TimeUnit::Minutes.to_millis(1024), 1024 * 1000 * 60);
    assert_eq!(TimeUnit::Minutes.to_seconds(1024), 1024 * 60);
    assert_eq!(TimeUnit::Minutes.to_minutes(1024), 1024);
    assert_eq!(TimeUnit::Minutes.to_hours(1024), 1024 / 60);
    assert_eq!(TimeUnit::Minutes.to_days(1024), 1024 / 60 / 24);
}

#[test]
fn test_time_unit_hours() {
    assert_eq!(
        TimeUnit::Hours.to_nanos(1024),
        1024 * 1000 * 1000 * 1000 * 60 * 60
    );
    assert_eq!(
        TimeUnit::Hours.to_micros(1024),
        1024 * 1000 * 1000 * 60 * 60
    );
    assert_eq!(TimeUnit::Hours.to_millis(1024), 1024 * 1000 * 60 * 60);
    assert_eq!(TimeUnit::Hours.to_seconds(1024), 1024 * 60 * 60);
    assert_eq!(TimeUnit::Hours.to_minutes(1024), 1024 * 60);
    assert_eq!(TimeUnit::Hours.to_hours(1024), 1024);
    assert_eq!(TimeUnit::Hours.to_days(1024), 1024 / 24);
}

#[test]
fn test_time_unit_days() {
    assert_eq!(
        TimeUnit::Days.to_nanos(1024),
        1024 * 1000 * 1000 * 1000 * 60 * 60 * 24
    );
    assert_eq!(
        TimeUnit::Days.to_micros(1024),
        1024 * 1000 * 1000 * 60 * 60 * 24
    );
    assert_eq!(TimeUnit::Days.to_millis(1024), 1024 * 1000 * 60 * 60 * 24);
    assert_eq!(TimeUnit::Days.to_seconds(1024), 1024 * 60 * 60 * 24);
    assert_eq!(TimeUnit::Days.to_minutes(1024), 1024 * 60 * 24);
    assert_eq!(TimeUnit::Days.to_hours(1024), 1024 * 24);
    assert_eq!(TimeUnit::Days.to_days(1024), 1024);
}

#[test]
fn test_time_unit_value_of() {
    assert_eq!(
        TimeUnit::value_of("Nanoseconds"),
        Some(TimeUnit::Nanoseconds)
    );
    assert_eq!(
        TimeUnit::value_of("Microseconds"),
        Some(TimeUnit::Microseconds)
    );
    assert_eq!(
        TimeUnit::value_of("Milliseconds"),
        Some(TimeUnit::Milliseconds)
    );
    assert_eq!(TimeUnit::value_of("Seconds"), Some(TimeUnit::Seconds));
    assert_eq!(TimeUnit::value_of("Minutes"), Some(TimeUnit::Minutes));
    assert_eq!(TimeUnit::value_of("Hours"), Some(TimeUnit::Hours));
    assert_eq!(TimeUnit::value_of("Days"), Some(TimeUnit::Days));
    assert_eq!(TimeUnit::value_of("Invalid"), None);
}
