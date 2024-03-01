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

use std::string::ToString;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

// ----------------------------------------------------------------

/// [`DateTimePattern`] date & time string pattern.
pub enum DateTimePattern {
    YyyyMmDd,
    MmDdYyyy,
    DdMmYyyy,
    YyyyMmDdHhMm,
    YyyyMmDdHhMmSs,
    YyyyMmDdHhMmSsSss,
    HhMm,
    HhMmSs,
    MonthFull,
    MonthAbbr,
    WeekdayFull,
    WeekdayAbbr,
    AmPm,
    Timestamp,
}

impl DateTimePattern {
    /// [`YYYY_MM_DD`] `%Y-%m-%d` ...
    pub const YYYY_MM_DD: &'static str = "%Y-%m-%d";
    pub const MM_DD_YYYY: &'static str = "%m/%d/%Y";
    pub const DD_MM_YYYY: &'static str = "%d-%m-%Y";

    pub const YYYY_MM_DD_HH_MM: &'static str = "%Y-%m-%d %H:%M";
    pub const YYYY_MM_DD_HH_MM_SS: &'static str = "%Y-%m-%d %H:%M:%S";
    pub const YYYY_MM_DD_HH_MM_SS_SSS: &'static str = "%Y-%m-%d %H:%M:%S%.3f";

    pub const HH_MM: &'static str = "%H:%M";
    pub const HH_MM_SS: &'static str = "%H:%M:%S";

    pub const MONTH_FULL: &'static str = "%B";
    pub const MONTH_ABBR: &'static str = "%b";

    pub const WEEKDAY_FULL: &'static str = "%A";
    pub const WEEKDAY_ABBR: &'static str = "%a";

    pub const AM_PM: &'static str = "%p";

    /// [`format_date_time_utc`]
    ///
    /// Formats a UTC date and time according to the specified pattern.
    ///
    /// This function takes a reference to a `DateTime<Utc>` object and a `DateTimePattern` enum value,
    /// then formats the datetime based on the provided pattern, returning a formatted string.
    pub fn format_date_time_utc(datetime: &DateTime<Utc>, pattern: DateTimePattern) -> String {
        match pattern {
            DateTimePattern::YyyyMmDd => datetime.format(Self::YYYY_MM_DD).to_string(), // Formats as "year-month-day"
            DateTimePattern::MmDdYyyy => datetime.format(Self::MM_DD_YYYY).to_string(), // Formats as "month-day-year"
            DateTimePattern::DdMmYyyy => datetime.format(Self::DD_MM_YYYY).to_string(), // Formats as "day-month-year"
            DateTimePattern::YyyyMmDdHhMm => datetime.format(Self::YYYY_MM_DD_HH_MM).to_string(), // Formats as "year-month-day hour-minute"
            DateTimePattern::YyyyMmDdHhMmSs => {
                datetime.format(Self::YYYY_MM_DD_HH_MM_SS).to_string()
            } // Formats as "year-month-day hour-minute-second"
            DateTimePattern::YyyyMmDdHhMmSsSss => {
                datetime.format(Self::YYYY_MM_DD_HH_MM_SS_SSS).to_string()
            } // Formats as "year-month-day hour-minute-second-millisecond
            DateTimePattern::HhMm => datetime.format(Self::HH_MM).to_string(), // Formats as "hour-minute"
            DateTimePattern::HhMmSs => datetime.format(Self::HH_MM_SS).to_string(), // Formats as "hour-minute-second"
            DateTimePattern::MonthFull => datetime.format(Self::MONTH_FULL).to_string(), // Formats as "full month name"
            DateTimePattern::MonthAbbr => datetime.format(Self::MONTH_ABBR).to_string(), // Formats as "abbreviated month name"
            DateTimePattern::WeekdayFull => datetime.format(Self::WEEKDAY_FULL).to_string(), // Formats as "full weekday name"
            DateTimePattern::WeekdayAbbr => datetime.format(Self::WEEKDAY_ABBR).to_string(), // Formats as "abbreviated weekday name"
            DateTimePattern::AmPm => datetime.format(Self::AM_PM).to_string(), // Formats as "AM/PM"
            DateTimePattern::Timestamp => datetime.timestamp().to_string(), // Formats as "timestamp"
        }
    }

    /// [`format_naive_date_time`]
    ///
    /// Formats a [`NaiveDateTime`] according to the specified pattern.
    ///
    /// This function takes a reference to a [`NaiveDateTime`] object and a [`DateTimePattern`] enum value,
    /// then formats the datetime based on the provided pattern, returning a formatted string.
    pub fn format_naive_date_time(datetime: &NaiveDateTime, pattern: DateTimePattern) -> String {
        let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(datetime);
        Self::format_date_time_utc(&datetime_utc, pattern)
    }
}

// ----------------------------------------------------------------

/// [`TimeUnit`] time unit.
pub enum TimeUnit {
    /// Time unit representing one thousandth of a microsecond.
    Nanoseconds,
    /// Time unit representing one thousandth of a millisecond.
    Microseconds,
    // Time unit representing one thousandth of a second.
    Milliseconds,
    /// Time unit representing one second.
    Seconds,
    /// Time unit representing sixty seconds.
    Minutes,
    /// Time unit representing sixty minutes.
    Hours,
    /// Time unit representing twenty-four hours.
    Days,
}

impl TimeUnit {
    pub const ZERO: u64 = 0;
    pub const ONE: u64 = 1;
    pub const THOUSAND: u64 = 1000;

    // ----------------------------------------------------------------

    pub const SECONDS_PER_MINUTE: u64 = 60;
    pub const MINUTES_PER_HOUR: u64 = 60;
    pub const HOURS_PER_DAY: u64 = 24;

    // ----------------------------------------------------------------

    pub const NANOS_SCALE: u64 = Self::ONE;
    pub const THOUSAND_SCALE: u64 = Self::THOUSAND;

    // ----------------------------------------------------------------

    pub const MICROSECOND_SCALE: u64 = Self::THOUSAND_SCALE * Self::NANOS_SCALE;
    pub const MILLISECOND_SCALE: u64 = Self::THOUSAND_SCALE * Self::MICROSECOND_SCALE;
    pub const SECOND_SCALE: u64 = Self::THOUSAND_SCALE * Self::MILLISECOND_SCALE;
    pub const MINUTE_SCALE: u64 = Self::SECONDS_PER_MINUTE * Self::SECOND_SCALE;
    pub const HOUR_SCALE: u64 = Self::MINUTES_PER_HOUR * Self::MINUTE_SCALE;
    pub const DAY_SCALE: u64 = Self::HOURS_PER_DAY * Self::HOUR_SCALE;

    // ----------------------------------------------------------------

    pub const NANOS_PER_MICROSECOND: u64 = Self::MICROSECOND_SCALE;
    pub const NANOS_PER_MILLISECOND: u64 = Self::MILLISECOND_SCALE;
    pub const NANOS_PER_SECOND: u64 = Self::SECOND_SCALE;
    pub const NANOS_PER_MINUTE: u64 = Self::MINUTE_SCALE;
    pub const NANOS_PER_HOUR: u64 = Self::HOUR_SCALE;
    pub const NANOS_PER_DAY: u64 = Self::DAY_SCALE;

    // ----------------------------------------------------------------

    pub fn to_nanos(&self, amount: u64) -> u64 {
        match self {
            TimeUnit::Nanoseconds => amount,
            TimeUnit::Microseconds => amount * Self::NANOS_PER_MICROSECOND,
            TimeUnit::Milliseconds => amount * Self::NANOS_PER_MILLISECOND,
            TimeUnit::Seconds => amount * Self::NANOS_PER_SECOND,
            TimeUnit::Minutes => amount * Self::NANOS_PER_MINUTE,
            TimeUnit::Hours => amount * Self::NANOS_PER_HOUR,
            TimeUnit::Days => amount * Self::NANOS_PER_DAY,
        }
    }

    pub fn to_micros(&self, amount: u64) -> u64 {
        self.to_nanos(amount) / Self::NANOS_PER_MICROSECOND
    }

    pub fn to_millis(&self, amount: u64) -> u64 {
        self.to_nanos(amount) / Self::NANOS_PER_MILLISECOND
    }

    pub fn to_seconds(&self, amount: u64) -> u64 {
        self.to_nanos(amount) / Self::NANOS_PER_SECOND
    }

    pub fn to_minutes(&self, amount: u64) -> u64 {
        self.to_seconds(amount) / Self::SECONDS_PER_MINUTE
    }

    pub fn to_hours(&self, amount: u64) -> u64 {
        self.to_minutes(amount) / Self::MINUTES_PER_HOUR
    }

    pub fn to_days(&self, amount: u64) -> u64 {
        self.to_hours(amount) / Self::HOURS_PER_DAY
    }
}
