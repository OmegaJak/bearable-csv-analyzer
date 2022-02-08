use chrono::{NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use timespan::NaiveDateTimeSpan;

use crate::model::{date_map::OrderedNaiveDateTimeSpan, parser::CsvRow};

use super::super::time_of_day::TimeOfDay;

#[derive(Debug, Deserialize)]
pub struct Symptom {
    #[serde(rename = "detail")]
    pub name: String,

    pub date: NaiveDate,

    #[serde(rename = "time of day")]
    pub time_of_day: TimeOfDay,

    #[serde(rename = "rating/amount")]
    pub severity: u8,
}

impl Symptom {
    pub fn from(row: &CsvRow) -> Symptom {
        Symptom {
            name: Symptom::parse_name(&row.detail),
            time_of_day: serde_plain::from_str::<TimeOfDay>(&row.time_of_day).unwrap(),
            severity: str::parse::<u8>(&row.amount).expect("Failed to parse symptom amount"),
            date: row.date,
        }
    }

    pub fn date_time_span(&self) -> Result<OrderedNaiveDateTimeSpan, timespan::Error> {
        let time_span = self.time_of_day.span()?;
        let date_time_span = NaiveDateTimeSpan::new(self.date.and_time(time_span.start), self.date.and_time(time_span.end))?;
        Ok(OrderedNaiveDateTimeSpan(date_time_span))
    }

    fn parse_name(name: &str) -> String {
        lazy_static! {
            static ref NAME_REGEX: Regex = Regex::new(r"(.*) (\(Mild\)|\(Moderate\)|\(Severe\)|\(Unbearable\))").unwrap();
        }

        let caps = NAME_REGEX.captures(name).expect("name contains severity");

        caps.get(1).unwrap().as_str().to_string()
    }
}
