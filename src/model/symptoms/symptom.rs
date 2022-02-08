use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use timespan::NaiveDateTimeSpan;

use crate::model::date_map::OrderedNaiveDateTimeSpan;

use super::super::time_of_day::TimeOfDay;

#[derive(Debug, Deserialize)]
pub struct Symptom {
	#[serde(rename = "detail")]
	pub name: String,

	pub date: NaiveDate,

	#[serde(rename = "time of day")]
	pub time_of_day: TimeOfDay,

	#[serde(rename = "rating/amount")]
	pub severity: u8
}

impl Symptom {
	pub fn date_time_span(&self) -> Result<OrderedNaiveDateTimeSpan, timespan::Error> {
		let time_span = self.time_of_day.span()?;
		let date_time_span = NaiveDateTimeSpan::new(self.date.and_time(time_span.start), self.date.and_time(time_span.end))?;
		Ok(OrderedNaiveDateTimeSpan(date_time_span))
	}
}