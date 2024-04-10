use chrono::{Datelike, NaiveDate, Utc};

pub fn get_current_academic_year(date: Option<NaiveDate>) -> i64 {
    let date = date.unwrap_or_else(|| Utc::now().naive_utc().date());
    let year = date.year();
    let month = date.month();

    // if month is march or less than march, then it is the previous year
    if month <= 3 {
        (year - 1).into()
    } else {
        year.into()
    }
}

pub fn get_current_semester(date: Option<NaiveDate>) -> i64 {
    let date = date.unwrap_or_else(|| Utc::now().naive_utc().date());
    let month = date.month();

    // if month is less than july, then it is the first semester
    if month >= 4 && month < 10 {
        1
    } else {
        2
    }
}
