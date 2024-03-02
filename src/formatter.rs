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

use crate::formatter::pattern::DateTimePattern;

// ----------------------------------------------------------------

pub mod pattern;

// ----------------------------------------------------------------

/// [`DateTimeFormatter`] date-time `formatter`
pub trait DateTimeFormatter {
    /// [`of_pattern`]
    ///
    /// Activates the pattern of the formatter.
    fn of_pattern(&self, pattern: DateTimePattern) -> Box<dyn DateTimeFormatter>;

    /// [`activated_pattern`]
    ///
    /// Get the pattern associated with this formatter.
    fn activated_pattern(&self) -> DateTimePattern;

    /// [`format_date_time_utc_default`]
    ///
    /// Formats a UTC date and time according to the formatter default pattern(new/or_pattern).
    fn format_date_time_utc_default(&self, datetime: &DateTime<Utc>) -> String {
        self.format_date_time_utc(datetime, self.activated_pattern())
    }

    /// [`format_date_time_utc`]
    ///
    /// Formats a UTC date and time according to the specified pattern.
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

    /// [`format_naive_date_time`]
    ///
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

    /// [`format_naive_date_time_default`]
    ///
    /// Formats a [`NaiveDateTime`] date and time according to the formatter default pattern(new/or_pattern).
    fn format_naive_date_time_default(&self, datetime: &NaiveDateTime) -> String {
        self.format_naive_date_time(datetime, self.activated_pattern())
    }

    /// [`format_naive_date_time`]
    ///
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

/// [`DefaultDateTimeFormatter`] The default `impl`s of [`DateTimeFormatter`]
pub struct DefaultDateTimeFormatter {
    // [`pattern`] the pattern of activates.
    pub pattern: DateTimePattern,
}

impl DateTimeFormatter for DefaultDateTimeFormatter {
    // override
    fn of_pattern(&self, pattern: DateTimePattern) -> Box<dyn DateTimeFormatter> {
        Box::new(DefaultDateTimeFormatter::new(pattern))
    }

    // override
    fn activated_pattern(&self) -> DateTimePattern {
        self.pattern.clone()
    }
}

#[allow(dead_code)]
impl DefaultDateTimeFormatter {
    pub fn default() -> Self {
        DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDdHhMmSs)
    }

    pub fn new(pattern: DateTimePattern) -> Self {
        DefaultDateTimeFormatter { pattern }
    }
}
