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
#[derive(Clone)]
#[allow(dead_code)]
pub enum DateTimePattern {
    /// [`YyyyMmDd`] `%Y-%m-%d`
    YyyyMmDd,
    /// [`MmDdYyyy`] `%m/%d/%Y`
    MmDdYyyy,
    /// [`DdMmYyyy`] `%d-%m-%Y`
    DdMmYyyy,

    /// [`YyyyMmDdHhMm`] `%Y-%m-%d %H:%M`
    YyyyMmDdHhMm,
    /// [`YyyyMmDdHhMmSs`] `%Y-%m-%d %H:%M:%S`
    YyyyMmDdHhMmSs,
    /// [`YyyyMmDdHhMmSsSss`] `%Y-%m-%d %H:%M:%S%.3f`
    YyyyMmDdHhMmSsSss,

    /// [`HhMm`] `%H:%M`
    HhMm,
    /// [`HhMmSs`] `%H:%M:%S`
    HhMmSs,

    /// [`MonthFull`] `%B`
    MonthFull,
    /// [`MonthAbbr`] `%b`
    MonthAbbr,

    /// [`WeekdayFull`] `%A`
    WeekdayFull,
    /// [`WeekdayAbbr`] `%a`
    WeekdayAbbr,

    /// [`AmPm`] `%p`
    AmPm,

    /// [`Timestamp`] `unix timestamp`
    Timestamp,
}

#[allow(dead_code)]
impl DateTimePattern {
    /// [`YYYY_MM_DD`] `%Y-%m-%d`
    pub const YYYY_MM_DD: &'static str = "%Y-%m-%d";
    /// [`MM_DD_YYYY`] `%m/%d/%Y`
    pub const MM_DD_YYYY: &'static str = "%m/%d/%Y";
    /// [`DD_MM_YYYY`] `%d-%m-%Y`
    pub const DD_MM_YYYY: &'static str = "%d-%m-%Y";

    /// [`YYYY_MM_DD_HH_MM`] `%Y-%m-%d %H:%M`
    pub const YYYY_MM_DD_HH_MM: &'static str = "%Y-%m-%d %H:%M";
    /// [`YYYY_MM_DD_HH_MM_SS`] `%Y-%m-%d %H:%M:%S`
    pub const YYYY_MM_DD_HH_MM_SS: &'static str = "%Y-%m-%d %H:%M:%S";
    /// [`YYYY_MM_DD_HH_MM_SS_SSS`] `%Y-%m-%d %H:%M:%S%.3f`
    pub const YYYY_MM_DD_HH_MM_SS_SSS: &'static str = "%Y-%m-%d %H:%M:%S%.3f";

    /// [`HH_MM`] `%H:%M`
    pub const HH_MM: &'static str = "%H:%M";
    /// [`HH_MM_SS`] `%H:%M:%S`
    pub const HH_MM_SS: &'static str = "%H:%M:%S";

    /// [`MONTH_FULL`] `%B`
    pub const MONTH_FULL: &'static str = "%B";
    /// [`MONTH_ABBR`] `%b`
    pub const MONTH_ABBR: &'static str = "%b";

    /// [`WEEKDAY_FULL`] `%A`
    pub const WEEKDAY_FULL: &'static str = "%A";
    /// [`WEEKDAY_ABBR`] `%a`
    pub const WEEKDAY_ABBR: &'static str = "%a";

    /// [`AM_PM`] `%p`
    pub const AM_PM: &'static str = "%p";
}
