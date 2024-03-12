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

/// Formats a [`DateTime<Utc>`] date and time according to the builtin formatter default pattern([`DateTimePattern::YyyyMmDdHhMmSs`]).
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
/// use chronounit::format_date_time_utc_default;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
/// let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);
///
/// assert_eq!(
///     format_date_time_utc_default!(&datetime_utc),
///     "2024-03-12 22:55:00"
/// );
/// ```
#[macro_export]
macro_rules! format_date_time_utc_default {
    ($datetime:expr) => {
        $crate::formatter::format_date_time_utc_default($datetime)
    };
}

/// Formats a [`NaiveDateTime`] according to the builtin formatter default pattern([`DateTimePattern::YyyyMmDdHhMmSs`]).
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::format_naive_date_time_utc_default;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     format_naive_date_time_utc_default!(&ndt),
///     "2024-03-12 22:55:00"
/// );
/// ```
#[macro_export]
macro_rules! format_naive_date_time_utc_default {
    ($datetime:expr) => {
        $crate::formatter::format_naive_date_time_utc_default($datetime)
    };
}

/// Formats a [`NaiveDateTime`] according to the builtin formatter default pattern([`DateTimePattern::YyyyMmDdHhMmSs`]).
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::format_naive_date_time_utc_default;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     format_naive_date_time_utc_default!(&ndt),
///     "2024-03-12 22:55:00"
/// );
/// ```
#[macro_export]
macro_rules! format_naive_date_time_default {
    ($datetime:expr) => {
        $crate::formatter::format_naive_date_time_default($datetime)
    };
}

// ----------------------------------------------------------------

/// Formats a [`DateTime<Utc>`] according to the specified pattern.
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
/// use chronounit::format_date_time_utc;
/// use chronounit::formatter::pattern::DateTimePattern;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
/// let datetime_utc: DateTime<Utc> = Utc.from_utc_datetime(&ndt);
///
/// assert_eq!(
///     format_date_time_utc!(&datetime_utc, DateTimePattern::YyyyMmDd),
///     "2024-03-12"
/// );
/// assert_eq!(
///     format_date_time_utc!(&datetime_utc, DateTimePattern::YyyyMmDdHhMmSs),
///     "2024-03-12 22:55:00"
/// );
/// assert_eq!(
///     format_date_time_utc!(&datetime_utc, DateTimePattern::HhMmSs),
///     "22:55:00"
/// );
/// ```
#[macro_export]
macro_rules! format_date_time_utc {
    ($datetime:expr, $pattern:expr) => {
        $crate::formatter::format_date_time_utc($datetime, $pattern)
    };
}

/// Formats a [`NaiveDateTime`] -> [`DateTime<Utc>`] according to the specified pattern.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::format_naive_date_time_utc;
/// use chronounit::formatter::pattern::DateTimePattern;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     format_naive_date_time_utc!(&ndt, DateTimePattern::YyyyMmDd),
///     "2024-03-12"
/// );
/// assert_eq!(
///     format_naive_date_time_utc!(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
///     "2024-03-12 22:55:00"
/// );
/// assert_eq!(
///     format_naive_date_time_utc!(&ndt, DateTimePattern::HhMmSs),
///     "22:55:00"
/// );
/// ```
#[macro_export]
macro_rules! format_naive_date_time_utc {
    ($datetime:expr, $pattern:expr) => {
        $crate::formatter::format_naive_date_time_utc($datetime, $pattern)
    };
}

/// Formats a [`NaiveDateTime`] according to the specified pattern.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDateTime;
/// use chronounit::format_naive_date_time;
/// use chronounit::formatter::pattern::DateTimePattern;
///
/// let now = "2024-03-12 22:55:00";
/// let ndt = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").expect("Parse error");
///
/// assert_eq!(
///     format_naive_date_time!(&ndt, DateTimePattern::YyyyMmDd),
///     "2024-03-12"
/// );
/// assert_eq!(
///     format_naive_date_time!(&ndt, DateTimePattern::YyyyMmDdHhMmSs),
///     "2024-03-12 22:55:00"
/// );
/// assert_eq!(
///     format_naive_date_time!(&ndt, DateTimePattern::HhMmSs),
///     "22:55:00"
/// );
/// ```
#[macro_export]
macro_rules! format_naive_date_time {
    ($datetime:expr, $pattern:expr) => {
        $crate::formatter::format_naive_date_time($datetime, $pattern)
    };
}
