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

use std::sync::{Arc, Mutex};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use lazy_static::lazy_static;

use crate::formatter::pattern::DateTimePattern;

// ----------------------------------------------------------------

pub mod pattern;

// ----------------------------------------------------------------

/// `DateTimeFormatter` date-time `formatter`
pub trait DateTimeFormatter {
    /// Activates the pattern of the formatter.
    ///
    /// # Examples
    /// ```rust
    /// use chrono::NaiveDateTime;
    /// use chronounit::formatter::DateTimeFormatter;
    /// use chronounit::formatter::pattern::DateTimePattern;
    /// use crate::chronounit::formatter::DefaultDateTimeFormatter;
    ///
    /// let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);
    /// let dtf = dtf.of_pattern(DateTimePattern::YyyyMmDdHhMmSs);
    ///
    /// assert_eq!(dtf.activated_pattern().value(), DateTimePattern::YyyyMmDdHhMmSs.value());
    /// assert_eq!(dtf.activated_pattern(), DateTimePattern::YyyyMmDdHhMmSs);
    /// ```
    fn of_pattern(&self, pattern: DateTimePattern) -> Box<dyn DateTimeFormatter>;

    /// Get the pattern associated with this formatter.
    fn activated_pattern(&self) -> DateTimePattern;

    /// Formats a [`DateTime<Utc>`] according to the formatter default pattern(new/or_pattern).
    fn format_date_time_utc_default(&self, datetime: &DateTime<Utc>) -> String {
        self.format_date_time_utc(datetime, self.activated_pattern())
    }

    /// Formats a [`DateTime<Utc>`] according to the specified pattern.
    ///
    /// This function takes a reference to a `DateTime<Utc>` object and a `DateTimePattern` enum value,
    /// then formats the datetime based on the provided pattern, returning a formatted string.
    fn format_date_time_utc(&self, datetime: &DateTime<Utc>, pattern: DateTimePattern) -> String {
        match pattern {
            DateTimePattern::YyyyMmDd => datetime.format(DateTimePattern::YYYY_MM_DD).to_string(), // Formats as "year-month-day"
            DateTimePattern::MmDdYyyy => datetime.format(DateTimePattern::MM_DD_YYYY).to_string(), // Formats as "month-day-year"
            DateTimePattern::DdMmYyyy => datetime.format(DateTimePattern::DD_MM_YYYY).to_string(), // Formats as "day-month-year"
            DateTimePattern::YyyyMmDdHhMm => datetime
                .format(DateTimePattern::YYYY_MM_DD_HH_MM)
                .to_string(), // Formats as "year-month-day hour-minute"
            DateTimePattern::YyyyMmDdHhMmSs => datetime
                .format(DateTimePattern::YYYY_MM_DD_HH_MM_SS)
                .to_string(), // Formats as "year-month-day hour-minute-second"
            DateTimePattern::YyyyMmDdHhMmSsSss => datetime
                .format(DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS)
                .to_string(), // Formats as "year-month-day hour-minute-second-millisecond
            DateTimePattern::HhMm => datetime.format(DateTimePattern::HH_MM).to_string(), // Formats as "hour-minute"
            DateTimePattern::HhMmSs => datetime.format(DateTimePattern::HH_MM_SS).to_string(), // Formats as "hour-minute-second"
            DateTimePattern::MonthFull => datetime.format(DateTimePattern::MONTH_FULL).to_string(), // Formats as "full month name"
            DateTimePattern::MonthAbbr => datetime.format(DateTimePattern::MONTH_ABBR).to_string(), // Formats as "abbreviated month name"
            DateTimePattern::WeekdayFull => {
                datetime.format(DateTimePattern::WEEKDAY_FULL).to_string()
            } // Formats as "full weekday name"
            DateTimePattern::WeekdayAbbr => {
                datetime.format(DateTimePattern::WEEKDAY_ABBR).to_string()
            } // Formats as "abbreviated weekday name"
            DateTimePattern::AmPm => datetime.format(DateTimePattern::AM_PM).to_string(), // Formats as "AM/PM"
            DateTimePattern::Timestamp => datetime.timestamp().to_string(), // Formats as "timestamp"
        }
    }

    // ----------------------------------------------------------------

    /// Formats a [`NaiveDateTime`] according to the formatter default pattern(new/or_pattern).
    ///
    /// This function takes a reference to a [`NaiveDateTime`] object,
    /// then formats the datetime based on the formatter activates pattern, returning a formatted string.
    fn format_naive_date_time_utc_default(&self, datetime: &NaiveDateTime) -> String {
        let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(datetime);
        self.format_date_time_utc(&datetime_utc, self.activated_pattern())
    }

    /// Formats a [`NaiveDateTime`] according to the specified pattern.
    ///
    /// This function takes a reference to a [`NaiveDateTime`] object and a [`DateTimePattern`] enum value,
    /// then formats the datetime based on the provided pattern, returning a formatted string.
    fn format_naive_date_time_utc(
        &self,
        datetime: &NaiveDateTime,
        pattern: DateTimePattern,
    ) -> String {
        let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(datetime);
        self.format_date_time_utc(&datetime_utc, pattern)
    }

    /// Formats a [`NaiveDateTime`] date and time according to the formatter default pattern(new/or_pattern).
    fn format_naive_date_time_default(&self, datetime: &NaiveDateTime) -> String {
        self.format_naive_date_time(datetime, self.activated_pattern())
    }

    /// Formats a [`NaiveDateTime`] according to the specified pattern.
    ///
    /// This function takes a reference to a [`NaiveDateTime`] object and a `DateTimePattern` enum value,
    /// then formats the datetime based on the provided pattern, returning a formatted string.
    fn format_naive_date_time(&self, datetime: &NaiveDateTime, pattern: DateTimePattern) -> String {
        match pattern {
            DateTimePattern::YyyyMmDd => datetime.format(DateTimePattern::YYYY_MM_DD).to_string(), // Formats as "year-month-day"
            DateTimePattern::MmDdYyyy => datetime.format(DateTimePattern::MM_DD_YYYY).to_string(), // Formats as "month-day-year"
            DateTimePattern::DdMmYyyy => datetime.format(DateTimePattern::DD_MM_YYYY).to_string(), // Formats as "day-month-year"
            DateTimePattern::YyyyMmDdHhMm => datetime
                .format(DateTimePattern::YYYY_MM_DD_HH_MM)
                .to_string(), // Formats as "year-month-day hour-minute"
            DateTimePattern::YyyyMmDdHhMmSs => datetime
                .format(DateTimePattern::YYYY_MM_DD_HH_MM_SS)
                .to_string(), // Formats as "year-month-day hour-minute-second"
            DateTimePattern::YyyyMmDdHhMmSsSss => datetime
                .format(DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS)
                .to_string(), // Formats as "year-month-day hour-minute-second-millisecond
            DateTimePattern::HhMm => datetime.format(DateTimePattern::HH_MM).to_string(), // Formats as "hour-minute"
            DateTimePattern::HhMmSs => datetime.format(DateTimePattern::HH_MM_SS).to_string(), // Formats as "hour-minute-second"
            DateTimePattern::MonthFull => datetime.format(DateTimePattern::MONTH_FULL).to_string(), // Formats as "full month name"
            DateTimePattern::MonthAbbr => datetime.format(DateTimePattern::MONTH_ABBR).to_string(), // Formats as "abbreviated month name"
            DateTimePattern::WeekdayFull => {
                datetime.format(DateTimePattern::WEEKDAY_FULL).to_string()
            } // Formats as "full weekday name"
            DateTimePattern::WeekdayAbbr => {
                datetime.format(DateTimePattern::WEEKDAY_ABBR).to_string()
            } // Formats as "abbreviated weekday name"
            DateTimePattern::AmPm => datetime.format(DateTimePattern::AM_PM).to_string(), // Formats as "AM/PM"
            DateTimePattern::Timestamp => datetime.timestamp().to_string(), // Formats as "timestamp"
        }
    }
}

/// [`DefaultDateTimeFormatter`] The default `impl` of [`DateTimeFormatter`]
pub struct DefaultDateTimeFormatter {
    /// [`pattern`] the activate pattern([`DateTimePattern`]) of formatter .
    pub pattern: DateTimePattern,
}

impl DateTimeFormatter for DefaultDateTimeFormatter {
    /// override
    fn of_pattern(&self, pattern: DateTimePattern) -> Box<dyn DateTimeFormatter> {
        Box::new(DefaultDateTimeFormatter::new(pattern))
    }

    /// override
    fn activated_pattern(&self) -> DateTimePattern {
        self.pattern.clone()
    }
}

#[allow(dead_code)]
impl DefaultDateTimeFormatter {
    pub fn builtin() -> Self {
        DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSs)
    }

    pub fn new(pattern: DateTimePattern) -> Self {
        DefaultDateTimeFormatter { pattern }
    }
}

// ----------------------------------------------------------------

lazy_static! {
    static ref BUILT_IN_FORMATTER: Arc<Mutex<Option<DefaultDateTimeFormatter>>> =
        Arc::new(Mutex::new(None));
}

fn formatter() -> Arc<Mutex<Option<DefaultDateTimeFormatter>>> {
    let mut instance = BUILT_IN_FORMATTER.lock().unwrap();
    if instance.is_none() {
        *instance = Some(DefaultDateTimeFormatter::builtin());
    }

    Arc::clone(&BUILT_IN_FORMATTER)
}

// ----------------------------------------------------------------

/// Formats a [`DateTime<Utc>`] date and time according to the builtin formatter default pattern([`DateTimePattern::YyyyMmDdHhMmSs`]).
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
/// use chronounit::formatter;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
/// let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);
///
/// assert_eq!(
///     formatter::format_date_time_utc_default(&datetime_utc),
///     "2024-03-12 22:55:00"
/// );
/// ```
pub fn format_date_time_utc_default(datetime: &DateTime<Utc>) -> String {
    formatter()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .format_date_time_utc_default(datetime)
}

/// Formats a [`NaiveDateTime`] according to the builtin formatter default pattern([`DateTimePattern::YyyyMmDdHhMmSs`]).
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::formatter;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     formatter::format_naive_date_time_utc_default(&ndt),
///     "2024-03-12 22:55:00"
/// );
/// ```
pub fn format_naive_date_time_utc_default(datetime: &NaiveDateTime) -> String {
    formatter()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .format_naive_date_time_utc_default(datetime)
}

/// Formats a [`NaiveDateTime`] according to the builtin formatter default pattern([`DateTimePattern::YyyyMmDdHhMmSs`]).
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::formatter;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     formatter::format_naive_date_time_utc_default(&ndt),
///     "2024-03-12 22:55:00"
/// );
/// ```
pub fn format_naive_date_time_default(datetime: &NaiveDateTime) -> String {
    formatter()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .format_naive_date_time_default(datetime)
}

/// Formats a [`DateTime<Utc>`] according to the specified pattern.
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
/// use chronounit::formatter;
/// use chronounit::formatter::pattern::DateTimePattern;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
/// let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);
///
/// assert_eq!(
///     formatter::format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDd),
///     "2024-03-12"
/// );
/// assert_eq!(
///     formatter::format_date_time_utc(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
///     "2024-03-12 22:55:00"
/// );
/// assert_eq!(
///     formatter::format_date_time_utc(&datetime_utc, DateTimePattern::HhMmSs),
///     "22:55:00"
/// );
/// ```
pub fn format_date_time_utc(datetime: &DateTime<Utc>, pattern: DateTimePattern) -> String {
    formatter()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .format_date_time_utc(datetime, pattern)
}

/// Formats a [`NaiveDateTime`] -> [`DateTime<Utc>`] according to the specified pattern.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::formatter;
/// use chronounit::formatter::pattern::DateTimePattern;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     formatter::format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDd),
///     "2024-03-12"
/// );
/// assert_eq!(
///     formatter::format_naive_date_time_utc(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
///     "2024-03-12 22:55:00"
/// );
/// assert_eq!(
///     formatter::format_naive_date_time_utc(&ndt, DateTimePattern::HhMmSs),
///     "22:55:00"
/// );
/// ```
pub fn format_naive_date_time_utc(datetime: &NaiveDateTime, pattern: DateTimePattern) -> String {
    formatter()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .format_naive_date_time_utc(datetime, pattern)
}

/// Formats a [`NaiveDateTime`] according to the specified pattern.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::formatter;
/// use chronounit::formatter::pattern::DateTimePattern;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     formatter::format_naive_date_time(&ndt, DateTimePattern::YyyyMmDd),
///     "2024-03-12"
/// );
/// assert_eq!(
///     formatter::format_naive_date_time(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
///     "2024-03-12 22:55:00"
/// );
/// assert_eq!(
///     formatter::format_naive_date_time(&ndt, DateTimePattern::HhMmSs),
///     "22:55:00"
/// );
/// ```
pub fn format_naive_date_time(datetime: &NaiveDateTime, pattern: DateTimePattern) -> String {
    formatter()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .format_naive_date_time(datetime, pattern)
}
