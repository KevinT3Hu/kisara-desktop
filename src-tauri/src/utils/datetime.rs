use chrono::{Datelike, NaiveDate, Weekday};

pub fn determine_current_season() -> (i32, i32) {
    let now = chrono::Local::now();
    let year = now.year();
    let month = now.month();

    let season = match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        _ => 4,
    };

    (year, season)
}

pub const fn get_season_start_end(
    (year, season): (i32, i32),
) -> Option<(chrono::NaiveDate, chrono::NaiveDate)> {
    let start_date = match season {
        1 => NaiveDate::from_weekday_of_month_opt(year - 1, 12, Weekday::Mon, 1),
        2 => NaiveDate::from_weekday_of_month_opt(year, 3, Weekday::Mon, 1),
        3 => NaiveDate::from_weekday_of_month_opt(year, 6, Weekday::Mon, 1),
        4 => NaiveDate::from_weekday_of_month_opt(year, 9, Weekday::Mon, 1),
        _ => return None,
    };

    let end_date = match season {
        1 => NaiveDate::from_weekday_of_month_opt(year, 3, Weekday::Mon, 1),
        2 => NaiveDate::from_weekday_of_month_opt(year, 6, Weekday::Mon, 1),
        3 => NaiveDate::from_weekday_of_month_opt(year, 9, Weekday::Mon, 1),
        4 => NaiveDate::from_weekday_of_month_opt(year + 1, 1, Weekday::Mon, 1),
        _ => return None,
    };

    if let (Some(start), Some(end)) = (start_date, end_date) {
        Some((start, end))
    } else {
        None
    }
}
