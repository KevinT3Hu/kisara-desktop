use chrono::{Datelike, NaiveDate};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Season {
    pub year: i32,
    pub season: i32,
}

impl Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let season_str = format!("{},{}", self.year, self.season);
        serializer.serialize_str(&season_str)
    }
}

impl Season {
    pub const fn new(year: i32, season: i32) -> Self {
        Self { year, season }
    }

    pub fn determine_season(date: NaiveDate) -> Self {
        let year = date.year();
        let month = date.month();

        let season = match month {
            1..=2 | 12 => 1, // Winter
            3..=5 => 2,      // Spring
            6..=8 => 3,      // Summer
            9..=11 => 4,     // Fall
            _ => unreachable!(),
        };

        let adjusted_year = if month == 12 { year + 1 } else { year };

        Self::new(adjusted_year, season)
    }
}
