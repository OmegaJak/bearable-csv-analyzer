use chrono::{NaiveDate};
use serde::Deserialize;

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
}