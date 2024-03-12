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

use std::thread;
use std::time::Duration;

// ----------------------------------------------------------------

pub mod formatter;

/// @since 0.3.0
#[macro_use]
pub mod macros;

#[cfg(test)]
mod macro_tests;
#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

/// [`TimeUnit`] time unit.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum TimeUnit {
    /// Time unit representing one thousandth of a microsecond.
    Nanoseconds,
    /// Time unit representing one thousandth of a millisecond.
    Microseconds,
    /// Time unit representing one thousandth of a second.
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
    /// 0
    pub const ZERO: u64 = 0;
    /// 1
    pub const ONE: u64 = 1;
    /// 1000
    pub const THOUSAND: u64 = 1000;

    // ----------------------------------------------------------------

    /// 60
    pub const SECONDS_PER_MINUTE: u64 = 60;
    /// 60
    pub const MINUTES_PER_HOUR: u64 = 60;
    /// 24
    pub const HOURS_PER_DAY: u64 = 24;

    pub const SECONDS_PER_HOUR: u64 = Self::SECONDS_PER_MINUTE * Self::MINUTES_PER_HOUR;
    pub const SECONDS_PER_DAY: u64 =
        Self::SECONDS_PER_MINUTE * Self::MINUTES_PER_HOUR * Self::HOURS_PER_DAY;

    // ----------------------------------------------------------------

    /// 1
    pub const NANOS_SCALE: u64 = Self::ONE;
    /// 1000
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

    pub const NANOSECONDS_NAME: &'static str = "Nanoseconds";
    pub const MICROSECONDS_NAME: &'static str = "Microseconds";
    pub const MILLISECONDS_NAME: &'static str = "Milliseconds";
    pub const SECONDS_NAME: &'static str = "Seconds";
    pub const MINUTES_NAME: &'static str = "Minutes";
    pub const HOURS_NAME: &'static str = "Hours";
    pub const DAYS_NAME: &'static str = "Days";

    pub const NANOSECONDS_LOWER_CASE_NAME: &'static str = "nanoseconds";
    pub const MICROSECONDS_LOWER_CASE_NAME: &'static str = "microseconds";
    pub const MILLISECONDS_LOWER_CASE_NAME: &'static str = "milliseconds";
    pub const SECONDS_LOWER_CASE_NAME: &'static str = "seconds";
    pub const MINUTES_LOWER_CASE_NAME: &'static str = "minutes";
    pub const HOURS_LOWER_CASE_NAME: &'static str = "hours";
    pub const DAYS_LOWER_CASE_NAME: &'static str = "days";

    // ----------------------------------------------------------------

    /// Converts the given time amount to nanoseconds.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in nanoseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_nanos(1024), 1024);
    /// assert_eq!(TimeUnit::Microseconds.to_nanos(1024), 1024 * 1000);
    /// assert_eq!(TimeUnit::Milliseconds.to_nanos(1024), 1024 * (1000 * 1000));
    /// assert_eq!( TimeUnit::Seconds.to_nanos(1024), 1024 * (1000 * 1000 * 1000));
    /// assert_eq!(TimeUnit::Minutes.to_nanos(1024), 1024 * (1000 * 1000 * 1000 * 60));
    /// assert_eq!(TimeUnit::Hours.to_nanos(1024), 1024 * (1000 * 1000 * 1000 * 60 * 60));
    /// assert_eq!(TimeUnit::Days.to_nanos(1024), 1024 * (1000 * 1000 * 1000 * 60 * 60 * 24));
    /// ```
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

    /// Converts the given time amount to microseconds.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in microseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_micros(1024), 1024 / 1000);
    /// assert_eq!(TimeUnit::Microseconds.to_micros(1024), 1024);
    /// assert_eq!(TimeUnit::Milliseconds.to_micros(1024), 1024 * 1000);
    /// assert_eq!(TimeUnit::Seconds.to_micros(1024), 1024 * (1000 * 1000));
    /// assert_eq!(TimeUnit::Minutes.to_micros(1024), 1024 * (1000 * 1000 * 60));
    /// assert_eq!(TimeUnit::Hours.to_micros(1024), 1024 * (1000 * 1000 * 60 * 60));
    /// assert_eq!( TimeUnit::Days.to_micros(1024), 1024 * (1000 * 1000 * 60 * 60 * 24));
    /// ```
    pub fn to_micros(&self, amount: u64) -> u64 {
        self.to_nanos(amount) / Self::NANOS_PER_MICROSECOND
    }

    /// Converts the given time amount to milliseconds.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in milliseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_millis(1024), 1024 / 1000 / 1000);
    /// assert_eq!(TimeUnit::Microseconds.to_millis(1024), 1024 / 1000);
    /// assert_eq!(TimeUnit::Milliseconds.to_millis(1024), 1024);
    /// assert_eq!(TimeUnit::Seconds.to_millis(1024), 1024 * 1000);
    /// assert_eq!(TimeUnit::Minutes.to_millis(1024), 1024 * (1000 * 60));
    /// assert_eq!(TimeUnit::Hours.to_millis(1024), 1024 * (1000 * 60 * 60));
    /// assert_eq!(TimeUnit::Days.to_millis(1024), 1024 * (1000 * 60 * 60 * 24));
    /// ```
    pub fn to_millis(&self, amount: u64) -> u64 {
        self.to_nanos(amount) / Self::NANOS_PER_MILLISECOND
    }

    /// Converts the given time amount to seconds.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in seconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_seconds(1024), 1024 / 1000 / 1000 / 1000);
    /// assert_eq!(TimeUnit::Microseconds.to_seconds(1024), 1024 / 1000 / 1000);
    /// assert_eq!(TimeUnit::Milliseconds.to_seconds(1024), 1024 / 1000);
    /// assert_eq!(TimeUnit::Seconds.to_seconds(1024), 1024);
    /// assert_eq!(TimeUnit::Minutes.to_seconds(1024), 1024 * 60);
    /// assert_eq!(TimeUnit::Hours.to_seconds(1024), 1024 * (60 * 60));
    /// assert_eq!(TimeUnit::Days.to_seconds(1024), 1024 * (60 * 60 * 24));
    /// ```
    pub fn to_seconds(&self, amount: u64) -> u64 {
        self.to_nanos(amount) / Self::NANOS_PER_SECOND
    }

    /// Converts the given time amount to minutes.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in minutes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_minutes(1024), 1024 / 1000 / 1000 / 1000 / 60);
    /// assert_eq!(TimeUnit::Microseconds.to_minutes(1024), 1024 / 1000 / 1000 / 60);
    /// assert_eq!(TimeUnit::Milliseconds.to_minutes(1024), 1024 / 1000 / 60);
    /// assert_eq!(TimeUnit::Seconds.to_minutes(1024), 1024 / 60);
    /// assert_eq!(TimeUnit::Minutes.to_minutes(1024), 1024);
    /// assert_eq!(TimeUnit::Hours.to_minutes(1024), 1024 * 60);
    /// assert_eq!(TimeUnit::Days.to_minutes(1024), 1024 * (60 * 24));
    /// ```
    pub fn to_minutes(&self, amount: u64) -> u64 {
        self.to_seconds(amount) / Self::SECONDS_PER_MINUTE
    }

    /// Converts the given time amount to hours.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_hours(1024), 1024 / 1000 / 1000 / 1000 / 60 / 60);
    /// assert_eq!(TimeUnit::Microseconds.to_hours(1024), 1024 / 1000 / 1000 / 60 / 60);
    /// assert_eq!(TimeUnit::Milliseconds.to_hours(1024), 1024 / 1000 / 60 / 60);
    /// assert_eq!(TimeUnit::Seconds.to_hours(1024), 1024 / 60 / 60);
    /// assert_eq!(TimeUnit::Minutes.to_hours(1024), 1024 / 60);
    /// assert_eq!(TimeUnit::Hours.to_hours(1024), 1024);
    /// assert_eq!(TimeUnit::Days.to_hours(1024), 1024 * 24);
    /// ```
    pub fn to_hours(&self, amount: u64) -> u64 {
        self.to_minutes(amount) / Self::MINUTES_PER_HOUR
    }

    /// Converts the given time amount to days.
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in days.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_days(1024), 1024 / 1000 / 1000 / 1000 / 60 / 60 / 24);
    /// assert_eq!(TimeUnit::Microseconds.to_days(1024), 1024 / 1000 / 1000 / 60 / 60 / 24);
    /// assert_eq!(TimeUnit::Milliseconds.to_days(1024), 1024 / 1000 / 60 / 60 / 24);
    /// assert_eq!(TimeUnit::Seconds.to_days(1024), 1024 / 60 / 60 / 24);
    /// assert_eq!(TimeUnit::Minutes.to_days(1024), 1024 / 60 / 24);
    /// assert_eq!(TimeUnit::Hours.to_days(1024), 1024 / 24);
    /// assert_eq!(TimeUnit::Days.to_days(1024), 1024);
    /// ```
    pub fn to_days(&self, amount: u64) -> u64 {
        self.to_hours(amount) / Self::HOURS_PER_DAY
    }

    /// Converts the given time amount to a `std` [`Duration`].
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in [`Duration`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_duration(100), Duration::from_nanos(100));
    /// assert_eq!(TimeUnit::Microseconds.to_duration(100), Duration::from_micros(100));
    /// assert_eq!(TimeUnit::Milliseconds.to_duration(100), Duration::from_millis(100));
    /// assert_eq!(TimeUnit::Seconds.to_duration(1), Duration::from_secs(1));
    /// assert_eq!(TimeUnit::Minutes.to_duration(1), Duration::from_secs(60));
    /// assert_eq!(TimeUnit::Hours.to_duration(1), Duration::from_secs(60 * 60));
    /// assert_eq!(TimeUnit::Days.to_duration(1), Duration::from_secs(60 * 60 * 24));
    /// ```
    pub fn to_duration(&self, amount: u64) -> Duration {
        match self {
            TimeUnit::Nanoseconds => Duration::from_nanos(amount),
            TimeUnit::Microseconds => Duration::from_micros(amount),
            TimeUnit::Milliseconds => Duration::from_millis(amount),
            TimeUnit::Seconds => Duration::from_secs(amount),
            TimeUnit::Minutes => Duration::from_secs(amount * TimeUnit::SECONDS_PER_MINUTE),
            TimeUnit::Hours => Duration::from_secs(amount * TimeUnit::SECONDS_PER_HOUR),
            TimeUnit::Days => Duration::from_secs(amount * TimeUnit::SECONDS_PER_DAY),
        }
    }

    /// Converts the given time amount to a `std` [`Duration`].
    ///
    /// # Arguments
    /// `amount` - The original time amount, with the unit specified by the caller.
    ///
    /// # Returns
    /// The converted time amount in [`Duration`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Duration;
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.to_chrono_duration(100), Duration::nanoseconds(100));
    /// assert_eq!(TimeUnit::Microseconds.to_chrono_duration(100), Duration::microseconds(100));
    /// assert_eq!(TimeUnit::Milliseconds.to_chrono_duration(100), Duration::milliseconds(100));
    /// assert_eq!(TimeUnit::Seconds.to_chrono_duration(1), Duration::seconds(1));
    /// assert_eq!(TimeUnit::Minutes.to_chrono_duration(1), Duration::minutes(1));
    /// assert_eq!(TimeUnit::Hours.to_chrono_duration(1), Duration::hours(1));
    /// assert_eq!(TimeUnit::Days.to_chrono_duration(1), Duration::days(1));
    /// ```
    pub fn to_chrono_duration(&self, amount: u64) -> chrono::Duration {
        match self {
            TimeUnit::Nanoseconds => chrono::Duration::nanoseconds(amount as i64),
            TimeUnit::Microseconds => chrono::Duration::microseconds(amount as i64),
            TimeUnit::Milliseconds => chrono::Duration::milliseconds(amount as i64),
            TimeUnit::Seconds => chrono::Duration::seconds(amount as i64),
            TimeUnit::Minutes => chrono::Duration::minutes(amount as i64),
            TimeUnit::Hours => chrono::Duration::hours(amount as i64),
            TimeUnit::Days => chrono::Duration::days(amount as i64),
        }
    }

    /// Retrieves the string representation of this [`TimeUnit`].
    ///
    /// # Returns
    /// A `String` which represents the formatted value of the object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Nanoseconds.value(), "Nanoseconds");
    /// assert_eq!(TimeUnit::Microseconds.value(), "Microseconds");
    /// assert_eq!(TimeUnit::Milliseconds.value(), "Milliseconds");
    /// assert_eq!(TimeUnit::Seconds.value(), "Seconds");
    /// assert_eq!(TimeUnit::Minutes.value(), "Minutes");
    /// assert_eq!(TimeUnit::Hours.value(), "Hours");
    /// assert_eq!(TimeUnit::Days.value(), "Days");
    /// ```
    pub fn value(&self) -> String {
        format!("{:?}", self)
    }

    /// Sleeps for a specified amount of time according to the [`TimeUnit`],
    ///
    /// # Arguments
    ///
    /// `amount` - A u64 parameter representing the amount of time to sleep, according to the [`TimeUnit`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use chronounit::TimeUnit;
    ///
    /// let start = std::time::Instant::now();
    /// TimeUnit::Milliseconds.sleep(1024);
    /// let duration = start.elapsed();
    /// assert!(duration >= Duration::from_millis(1024));
    /// ```
    pub fn sleep(&self, amount: u64) {
        let millis = self.to_millis(amount);
        let duration = Duration::from_millis(millis);
        thread::sleep(duration);
    }

    /// Sleeps for a specified amount of time according to the [`TimeUnit`],
    /// then executes a custom sleep function provided by the user/caller.
    ///
    /// This function converts the given `amount` of time to milliseconds based on the [`TimeUnit`] enum,
    /// constructs a Duration, and then passes it to the provided closure for custom sleep execution.
    ///
    /// # Arguments
    ///
    /// `amount` - A u64 parameter representing the amount of time to sleep, according to the [`TimeUnit`].
    /// `callback` - A closure that takes a [`std::time::Duration`] parameter and performs custom sleep actions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::thread;
    /// use std::time::Duration;
    /// use chronounit::TimeUnit;
    /// let start = std::time::Instant::now();
    ///
    /// TimeUnit::Milliseconds.closure_sleep(1024, |x| {
    ///     assert_eq!(x, Duration::from_millis(1024));
    ///     thread::sleep(x);
    /// });
    ///
    /// let duration = start.elapsed();
    /// assert!(duration >= Duration::from_millis(1024));
    /// ```
    pub fn closure_sleep<F>(&self, amount: u64, callback: F)
    where
        F: Fn(Duration),
    {
        let duration = self.to_duration(amount);
        callback(duration);
    }

    /// Sleeps for a specified amount of time according to the [`TimeUnit`],
    /// then executes a custom sleep function provided by the user/caller.
    ///
    /// This function converts the given `amount` of time to milliseconds based on the [`TimeUnit`] enum,
    /// constructs a Duration, and then passes it to the provided closure for custom sleep execution.
    ///
    /// # Arguments
    ///
    /// `amount` - A u64 parameter representing the amount of time to sleep, according to the [`TimeUnit`].
    /// `callback` - A closure that takes a [`chrono::Duration`] parameter and performs custom sleep actions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::thread;
    /// use std::time::Duration;
    /// use chronounit::TimeUnit;
    /// let start = std::time::Instant::now();
    ///
    /// TimeUnit::Milliseconds.closure_chrono_sleep(1024, |x| {
    ///     assert_eq!(x, chrono::Duration::milliseconds(1024));
    ///     thread::sleep(Duration::from_millis(x.num_milliseconds() as u64));
    /// });
    ///
    /// let duration = start.elapsed();
    /// assert!(duration >= Duration::from_millis(1024));
    /// ```
    pub fn closure_chrono_sleep<F>(&self, amount: u64, callback: F)
    where
        F: Fn(chrono::Duration),
    {
        let duration = self.to_chrono_duration(amount);
        callback(duration);
    }
}

impl TimeUnit {
    /// Returns the corresponding [`TimeUnit`] enum based on the provided [`TimeUnit`] value/name.
    ///
    /// # Arguments
    /// `value` - A string slice representing the value/name of the [`TimeUnit`].
    ///
    /// # Returns
    /// An `Option<Self>` containing the matched [`TimeUnit`] if a valid value/name is given, otherwise returns `None`.
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    /// assert_eq!(TimeUnit::value_of("Nanoseconds"), Some(TimeUnit::Nanoseconds));
    /// assert_eq!(TimeUnit::value_of("Microseconds"), Some(TimeUnit::Microseconds));
    /// assert_eq!(TimeUnit::value_of("Milliseconds"), Some(TimeUnit::Milliseconds));
    /// assert_eq!(TimeUnit::value_of("Seconds"), Some(TimeUnit::Seconds));
    /// assert_eq!(TimeUnit::value_of("Minutes"), Some(TimeUnit::Minutes));
    /// assert_eq!(TimeUnit::value_of("Hours"), Some(TimeUnit::Hours));
    /// assert_eq!(TimeUnit::value_of("Days"), Some(TimeUnit::Days));
    /// assert_eq!(TimeUnit::value_of("Invalid"), None);
    /// ```
    pub fn value_of(value: &str) -> Option<Self> {
        match value {
            TimeUnit::NANOSECONDS_NAME => Some(TimeUnit::Nanoseconds),
            TimeUnit::MICROSECONDS_NAME => Some(TimeUnit::Microseconds),
            TimeUnit::MILLISECONDS_NAME => Some(TimeUnit::Milliseconds),
            TimeUnit::SECONDS_NAME => Some(TimeUnit::Seconds),
            TimeUnit::MINUTES_NAME => Some(TimeUnit::Minutes),
            TimeUnit::HOURS_NAME => Some(TimeUnit::Hours),
            TimeUnit::DAYS_NAME => Some(TimeUnit::Days),
            _ => None,
        }
    }

    /// Returns the corresponding [`TimeUnit`] enum based on the provided [`TimeUnit`] value/name.
    /// performing a case-insensitive match.
    ///
    /// # Arguments
    /// `value` - A string slice representing the value/name of the [`TimeUnit`].
    ///
    /// # Returns
    /// An `Option<Self>` containing the matched [`TimeUnit`] if a valid value/name is given, otherwise returns `None`.
    ///
    /// ```rust
    /// use chronounit::TimeUnit;
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Nanoseconds"), Some(TimeUnit::Nanoseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Microseconds"), Some(TimeUnit::Microseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Milliseconds"), Some(TimeUnit::Milliseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Seconds"), Some(TimeUnit::Seconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Minutes"), Some(TimeUnit::Minutes));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Hours"), Some(TimeUnit::Hours));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Days"), Some(TimeUnit::Days));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("Invalid"), None);
    ///
    /// assert_eq!(TimeUnit::insensitive_case_value_of("NANOSECONDS"), Some(TimeUnit::Nanoseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("MICROSECONDS"), Some(TimeUnit::Microseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("MILLISECONDS"), Some(TimeUnit::Milliseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("SECONDS"), Some(TimeUnit::Seconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("MINUTES"), Some(TimeUnit::Minutes));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("HOURS"), Some(TimeUnit::Hours));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("DAYS"), Some(TimeUnit::Days));
    ///
    /// assert_eq!(TimeUnit::insensitive_case_value_of("nanoseconds"), Some(TimeUnit::Nanoseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("microseconds"), Some(TimeUnit::Microseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("milliseconds"), Some(TimeUnit::Milliseconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("seconds"), Some(TimeUnit::Seconds));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("minutes"), Some(TimeUnit::Minutes));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("hours"), Some(TimeUnit::Hours));
    /// assert_eq!(TimeUnit::insensitive_case_value_of("days"), Some(TimeUnit::Days));
    /// ```
    pub fn insensitive_case_value_of(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            TimeUnit::NANOSECONDS_LOWER_CASE_NAME => Some(TimeUnit::Nanoseconds),
            TimeUnit::MICROSECONDS_LOWER_CASE_NAME => Some(TimeUnit::Microseconds),
            TimeUnit::MILLISECONDS_LOWER_CASE_NAME => Some(TimeUnit::Milliseconds),
            TimeUnit::SECONDS_LOWER_CASE_NAME => Some(TimeUnit::Seconds),
            TimeUnit::MINUTES_LOWER_CASE_NAME => Some(TimeUnit::Minutes),
            TimeUnit::HOURS_LOWER_CASE_NAME => Some(TimeUnit::Hours),
            TimeUnit::DAYS_LOWER_CASE_NAME => Some(TimeUnit::Days),
            _ => None,
        }
    }
}
