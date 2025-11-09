use std::collections::HashSet;

use chrono::{Datelike, FixedOffset, NaiveDate, Utc};

pub fn get_current_academic_year(date: Option<NaiveDate>) -> i64 {
    let date = date.unwrap_or_else(|| Utc::now().naive_utc().date());
    let year = date.year();
    let month = date.month();

    // If the month is march or less than march, then it is the previous year
    if month <= 3 {
        (year - 1).into()
    } else {
        year.into()
    }
}

pub fn get_current_semester(date: Option<NaiveDate>) -> i64 {
    let month = date
        .unwrap_or_else(|| Utc::now().naive_utc().date())
        .month();

    // If the month is less than july, then it is the first semester
    if (4..10).contains(&month) { 1 } else { 2 }
}

pub fn get_current_date() -> NaiveDate {
    Utc::now()
        .with_timezone(&FixedOffset::east_opt(7 * 3600).unwrap()) // UTC+7: Asia/Bangkok
        .date_naive()
}

pub fn is_today_jaturamitr() -> bool {
    let today = get_current_date();
    let jaturamitr_dates = HashSet::from([
        NaiveDate::from_ymd_opt(2025, 11, 14).unwrap(),
        NaiveDate::from_ymd_opt(2025, 11, 15).unwrap(),
        NaiveDate::from_ymd_opt(2025, 11, 22).unwrap(),
    ]);

    jaturamitr_dates.contains(&today)
}
