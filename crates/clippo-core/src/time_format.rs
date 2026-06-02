use serde::{Deserialize, Serialize};

use crate::TimestampMillis;

const MILLIS_PER_SECOND: i128 = 1_000;
const SECONDS_PER_MINUTE: i128 = 60;
const MINUTES_PER_DAY: i128 = 24 * 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateTimeFormatStyle {
    EnUs,
    EnGb,
    Iso8601,
    JaJp,
}

impl DateTimeFormatStyle {
    #[must_use]
    pub fn from_locale(locale: &str) -> Self {
        let normalized = locale.to_ascii_lowercase();
        if normalized.starts_with("en-us") {
            Self::EnUs
        } else if normalized.starts_with("ja") {
            Self::JaJp
        } else if normalized.starts_with("en-gb")
            || normalized.starts_with("en-au")
            || normalized.starts_with("en-in")
        {
            Self::EnGb
        } else {
            Self::Iso8601
        }
    }
}

#[must_use]
pub fn format_timestamp_for_locale(
    timestamp: TimestampMillis,
    locale: &str,
    utc_offset_minutes: i32,
) -> String {
    let style = DateTimeFormatStyle::from_locale(locale);
    let timestamp_millis = i128::try_from(timestamp.0).unwrap_or(i128::MAX);
    let local_minutes =
        timestamp_millis / MILLIS_PER_SECOND / SECONDS_PER_MINUTE + i128::from(utc_offset_minutes);
    let days = div_floor(local_minutes, MINUTES_PER_DAY);
    let minute_of_day = local_minutes - days * MINUTES_PER_DAY;
    let hour = u32::try_from(minute_of_day / 60).unwrap_or_default();
    let minute = u32::try_from(minute_of_day % 60).unwrap_or_default();
    let days = i64::try_from(days).unwrap_or(if days.is_negative() {
        i64::MIN
    } else {
        i64::MAX
    });
    let (year, month, day) = civil_from_days(days);

    match style {
        DateTimeFormatStyle::EnUs => {
            let (display_hour, suffix) = hour_12(hour);
            format!(
                "{} {day}, {year}, {display_hour}:{minute:02} {suffix}",
                month_name_short(month)
            )
        }
        DateTimeFormatStyle::EnGb => {
            format!(
                "{day} {} {year}, {hour:02}:{minute:02}",
                month_name_short(month)
            )
        }
        DateTimeFormatStyle::JaJp => {
            format!("{year}/{month:02}/{day:02} {hour:02}:{minute:02}")
        }
        DateTimeFormatStyle::Iso8601 => {
            format!("{year}-{month:02}-{day:02} {hour:02}:{minute:02}")
        }
    }
}

fn div_floor(left: i128, right: i128) -> i128 {
    let quotient = left / right;
    let remainder = left % right;
    if remainder != 0 && (remainder > 0) != (right > 0) {
        quotient - 1
    } else {
        quotient
    }
}

fn hour_12(hour: u32) -> (u32, &'static str) {
    let suffix = if hour < 12 { "AM" } else { "PM" };
    let display_hour = match hour % 12 {
        0 => 12,
        value => value,
    };
    (display_hour, suffix)
}

fn month_name_short(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        _ => "Dec",
    }
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let adjusted_days = days_since_unix_epoch + 719_468;
    let era = if adjusted_days >= 0 {
        adjusted_days
    } else {
        adjusted_days - 146_096
    } / 146_097;
    let day_of_era = adjusted_days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);

    (
        i32::try_from(year).unwrap_or(if year.is_negative() {
            i32::MIN
        } else {
            i32::MAX
        }),
        u32::try_from(month).unwrap_or_default(),
        u32::try_from(day).unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const JAN_02_2026_15_04_UTC: TimestampMillis = TimestampMillis(1_767_366_240_000);

    #[test]
    fn formats_us_locale_with_twelve_hour_clock() {
        let formatted = format_timestamp_for_locale(JAN_02_2026_15_04_UTC, "en-US", 0);

        assert_eq!(formatted, "Jan 2, 2026, 3:04 PM");
    }

    #[test]
    fn formats_gb_locale_with_day_first_and_twenty_four_hour_clock() {
        let formatted = format_timestamp_for_locale(JAN_02_2026_15_04_UTC, "en-GB", 0);

        assert_eq!(formatted, "2 Jan 2026, 15:04");
    }

    #[test]
    fn formats_japanese_locale_with_numeric_order() {
        let formatted = format_timestamp_for_locale(JAN_02_2026_15_04_UTC, "ja-JP", 540);

        assert_eq!(formatted, "2026/01/03 00:04");
    }

    #[test]
    fn falls_back_to_iso_like_format_for_unknown_locale() {
        let formatted = format_timestamp_for_locale(JAN_02_2026_15_04_UTC, "de-DE", 60);

        assert_eq!(formatted, "2026-01-02 16:04");
    }
}
