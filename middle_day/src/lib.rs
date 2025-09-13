use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if is_leap_year(year) {
        return None;
    }

    let middle_date = NaiveDate::from_yo_opt(year as i32, 183)?;
    Some(middle_date.weekday())
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
