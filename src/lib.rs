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

pub mod formatter;
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

    // ----------------------------------------------------------------

    /// [`to_nanos`]
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

    /// [`to_micros`]
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

    /// [`to_millis`]
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

    /// [`to_seconds`]
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

    /// [`to_minutes`]
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

    /// [`to_hours`]
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

    /// [`to_days`]
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

    /// [`value`]
    ///
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
}

impl TimeUnit {
    /// [`value_of`]
    ///
    /// Returns the corresponding time unit enum based on the provided [`TimeUnit`] name.
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
}
