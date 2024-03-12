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

/// [`DateTimePattern`] date & time pattern.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DateTimePattern {
    /// `YyyyMmDd` `%Y-%m-%d`
    YyyyMmDd,
    /// `MmDdYyyy` `%m/%d/%Y`
    MmDdYyyy,
    /// `DdMmYyyy` `%d-%m-%Y`
    DdMmYyyy,

    /// `YyyyMmDdHhMm` `%Y-%m-%d %H:%M`
    YyyyMmDdHhMm,
    /// `YyyyMmDdHhMmSs` `%Y-%m-%d %H:%M:%S`
    YyyyMmDdHhMmSs,
    /// `YyyyMmDdHhMmSsSss` `%Y-%m-%d %H:%M:%S%.3f`
    YyyyMmDdHhMmSsSss,

    /// `HhMm` `%H:%M`
    HhMm,
    /// `HhMmSs` `%H:%M:%S`
    HhMmSs,

    /// `MonthFull` `%B`
    MonthFull,
    /// `MonthAbbr` `%b`
    MonthAbbr,

    /// `WeekdayFull` `%A`
    WeekdayFull,
    /// `WeekdayAbbr` `%a`
    WeekdayAbbr,

    /// `AmPm` `%p`
    AmPm,

    /// `Timestamp` `unix timestamp`
    Timestamp,
}

#[allow(dead_code)]
impl DateTimePattern {
    /// `YYYY_MM_DD` `%Y-%m-%d`
    pub const YYYY_MM_DD: &'static str = "%Y-%m-%d";
    /// `MM_DD_YYYY` `%m/%d/%Y`
    pub const MM_DD_YYYY: &'static str = "%m/%d/%Y";
    /// `DD_MM_YYYY` `%d-%m-%Y`
    pub const DD_MM_YYYY: &'static str = "%d-%m-%Y";

    /// `YYYY_MM_DD_HH_MM` `%Y-%m-%d %H:%M`
    pub const YYYY_MM_DD_HH_MM: &'static str = "%Y-%m-%d %H:%M";
    /// `YYYY_MM_DD_HH_MM_SS` `%Y-%m-%d %H:%M:%S`
    pub const YYYY_MM_DD_HH_MM_SS: &'static str = "%Y-%m-%d %H:%M:%S";
    /// `YYYY_MM_DD_HH_MM_SS_SSS` `%Y-%m-%d %H:%M:%S%.3f`
    pub const YYYY_MM_DD_HH_MM_SS_SSS: &'static str = "%Y-%m-%d %H:%M:%S%.3f";

    /// `HH_MM` `%H:%M`
    pub const HH_MM: &'static str = "%H:%M";
    /// `HH_MM_SS` `%H:%M:%S`
    pub const HH_MM_SS: &'static str = "%H:%M:%S";

    /// `MONTH_FULL` `%B`
    pub const MONTH_FULL: &'static str = "%B";
    /// `MONTH_ABBR` `%b`
    pub const MONTH_ABBR: &'static str = "%b";

    /// `WEEKDAY_FULL` `%A`
    pub const WEEKDAY_FULL: &'static str = "%A";
    /// `WEEKDAY_ABBR` `%a`
    pub const WEEKDAY_ABBR: &'static str = "%a";

    /// `AM_PM` `%p`
    pub const AM_PM: &'static str = "%p";

    /// `TIMESTAMP` `timestamp`
    pub const TIMESTAMP: &'static str = "timestamp";

    // ----------------------------------------------------------------

    /// `YYYY_MM_DD_NAME` `YyyyMmDd`
    pub const YYYY_MM_DD_NAME: &'static str = "YyyyMmDd";
    /// `MM_DD_YYYY_NAME` `MmDdYyyy`
    pub const MM_DD_YYYY_NAME: &'static str = "MmDdYyyy";
    /// `DD_MM_YYYY_NAME` `DdMmYyyy`
    pub const DD_MM_YYYY_NAME: &'static str = "DdMmYyyy";

    /// `YYYY_MM_DD_HH_MM_NAME` `YyyyMmDdHhMm`
    pub const YYYY_MM_DD_HH_MM_NAME: &'static str = "YyyyMmDdHhMm";
    /// `YYYY_MM_DD_HH_MM_SS_NAME` `YyyyMmDdHhMmSs`
    pub const YYYY_MM_DD_HH_MM_SS_NAME: &'static str = "YyyyMmDdHhMmSs";
    /// `YYYY_MM_DD_HH_MM_SS_SSS_NAME` `YyyyMmDdHhMmSsSss`
    pub const YYYY_MM_DD_HH_MM_SS_SSS_NAME: &'static str = "YyyyMmDdHhMmSsSss";

    /// `HH_MM_NAME` `HhMm`
    pub const HH_MM_NAME: &'static str = "HhMm";
    /// `HH_MM_SS_NAME` `HhMmSs`
    pub const HH_MM_SS_NAME: &'static str = "HhMmSs";

    /// `MONTH_FULL_NAME` `MonthFull`
    pub const MONTH_FULL_NAME: &'static str = "MonthFull";
    /// `MONTH_ABBR_NAME` `MonthAbbr`
    pub const MONTH_ABBR_NAME: &'static str = "MonthAbbr";

    /// `WEEKDAY_FULL_NAME` `WeekdayFull`
    pub const WEEKDAY_FULL_NAME: &'static str = "WeekdayFull";
    /// `WEEKDAY_ABBR_NAME` `WeekdayAbbr`
    pub const WEEKDAY_ABBR_NAME: &'static str = "WeekdayAbbr";

    /// `AM_PM_NAME` `AmPm`
    pub const AM_PM_NAME: &'static str = "AmPm";

    /// `TIMESTAMP_NAME` `Timestamp`
    pub const TIMESTAMP_NAME: &'static str = "Timestamp";

    // ----------------------------------------------------------------

    /// Retrieves the string representation of a [`DateTimePattern`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::formatter::pattern::DateTimePattern;
    ///
    /// assert_eq!(DateTimePattern::YyyyMmDdHhMmSs.value(), "YyyyMmDdHhMmSs");
    /// ```
    pub fn value(&self) -> String {
        format!("{:?}", self)
    }
}

impl DateTimePattern {
    /// Retrieves the static string representation of a date and time pattern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::formatter::pattern::DateTimePattern;
    ///
    /// let pattern = DateTimePattern::YyyyMmDd;
    /// assert_eq!(pattern.pattern_of(), DateTimePattern::YYYY_MM_DD);
    /// ```
    pub fn pattern_of(&self) -> &'static str {
        match self {
            DateTimePattern::YyyyMmDd => DateTimePattern::YYYY_MM_DD,
            DateTimePattern::MmDdYyyy => DateTimePattern::MM_DD_YYYY,
            DateTimePattern::DdMmYyyy => DateTimePattern::DD_MM_YYYY,
            DateTimePattern::YyyyMmDdHhMm => DateTimePattern::YYYY_MM_DD_HH_MM,
            DateTimePattern::YyyyMmDdHhMmSs => DateTimePattern::YYYY_MM_DD_HH_MM_SS,
            DateTimePattern::YyyyMmDdHhMmSsSss => DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS,
            DateTimePattern::HhMm => DateTimePattern::HH_MM,
            DateTimePattern::HhMmSs => DateTimePattern::HH_MM_SS,
            DateTimePattern::MonthFull => DateTimePattern::MONTH_FULL,
            DateTimePattern::MonthAbbr => DateTimePattern::MONTH_ABBR,
            DateTimePattern::WeekdayFull => DateTimePattern::WEEKDAY_FULL,
            DateTimePattern::WeekdayAbbr => DateTimePattern::WEEKDAY_ABBR,
            DateTimePattern::AmPm => DateTimePattern::AM_PM,
            DateTimePattern::Timestamp => DateTimePattern::TIMESTAMP,
        }
    }

    /// Returns the corresponding date-time pattern based on the provided pattern string.
    ///
    /// # Parameters
    /// `pattern`: A reference to a string representing a date-time pattern, e.g., [`%Y-%m-%d`].
    ///
    /// # Return Value
    /// An `Option<Self>` containing the matched pattern if one is found; otherwise, returns [`None`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::formatter::pattern::DateTimePattern;
    ///
    /// let pattern = DateTimePattern::YyyyMmDd;
    /// assert_eq!(DateTimePattern::value_of("%Y-%m-%d"), Some(DateTimePattern::YyyyMmDd));
    /// assert_eq!(DateTimePattern::value_of("Invalid"), None);
    /// ```
    pub fn value_of(pattern: &str) -> Option<DateTimePattern> {
        match pattern {
            DateTimePattern::YYYY_MM_DD => Some(DateTimePattern::YyyyMmDd),
            DateTimePattern::MM_DD_YYYY => Some(DateTimePattern::MmDdYyyy),
            DateTimePattern::DD_MM_YYYY => Some(DateTimePattern::DdMmYyyy),
            DateTimePattern::YYYY_MM_DD_HH_MM => Some(DateTimePattern::YyyyMmDdHhMm),
            DateTimePattern::YYYY_MM_DD_HH_MM_SS => Some(DateTimePattern::YyyyMmDdHhMmSs),
            DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS => Some(DateTimePattern::YyyyMmDdHhMmSsSss),
            DateTimePattern::HH_MM => Some(DateTimePattern::HhMm),
            DateTimePattern::HH_MM_SS => Some(DateTimePattern::HhMmSs),
            DateTimePattern::MONTH_FULL => Some(DateTimePattern::MonthFull),
            DateTimePattern::MONTH_ABBR => Some(DateTimePattern::MonthAbbr),
            DateTimePattern::WEEKDAY_FULL => Some(DateTimePattern::WeekdayFull),
            DateTimePattern::WEEKDAY_ABBR => Some(DateTimePattern::WeekdayAbbr),
            DateTimePattern::AM_PM => Some(DateTimePattern::AmPm),
            DateTimePattern::TIMESTAMP => Some(DateTimePattern::Timestamp),
            _ => None,
        }
    }

    /// Returns the corresponding date-time pattern based on the provided name string.
    ///
    /// # Parameters
    /// `pattern`: A reference to a string representing a date-time name, e.g., [`YyyyMmDd`].
    ///
    /// # Return Value
    /// An `Option<Self>` containing the matched name if one is found; otherwise, returns [`None`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronounit::formatter::pattern::DateTimePattern;
    ///
    /// let pattern = DateTimePattern::YyyyMmDd;
    /// assert_eq!(DateTimePattern::name_of("YyyyMmDd"), Some(DateTimePattern::YyyyMmDd));
    /// assert_eq!(DateTimePattern::value_of("Invalid"), None);
    /// ```
    pub fn name_of(name: &str) -> Option<DateTimePattern> {
        match name {
            DateTimePattern::YYYY_MM_DD_NAME => Some(DateTimePattern::YyyyMmDd),
            DateTimePattern::MM_DD_YYYY_NAME => Some(DateTimePattern::MmDdYyyy),
            DateTimePattern::DD_MM_YYYY_NAME => Some(DateTimePattern::DdMmYyyy),
            DateTimePattern::YYYY_MM_DD_HH_MM_NAME => Some(DateTimePattern::YyyyMmDdHhMm),
            DateTimePattern::YYYY_MM_DD_HH_MM_SS_NAME => Some(DateTimePattern::YyyyMmDdHhMmSs),
            DateTimePattern::YYYY_MM_DD_HH_MM_SS_SSS_NAME => {
                Some(DateTimePattern::YyyyMmDdHhMmSsSss)
            }
            DateTimePattern::HH_MM_NAME => Some(DateTimePattern::HhMm),
            DateTimePattern::HH_MM_SS_NAME => Some(DateTimePattern::HhMmSs),
            DateTimePattern::MONTH_FULL_NAME => Some(DateTimePattern::MonthFull),
            DateTimePattern::MONTH_ABBR_NAME => Some(DateTimePattern::MonthAbbr),
            DateTimePattern::WEEKDAY_FULL_NAME => Some(DateTimePattern::WeekdayFull),
            DateTimePattern::WEEKDAY_ABBR_NAME => Some(DateTimePattern::WeekdayAbbr),
            DateTimePattern::AM_PM_NAME => Some(DateTimePattern::AmPm),
            DateTimePattern::TIMESTAMP_NAME => Some(DateTimePattern::Timestamp),
            _ => None,
        }
    }
}
