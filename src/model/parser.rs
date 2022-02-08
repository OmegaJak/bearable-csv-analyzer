use std::iter::FromIterator;

use chrono::NaiveDate;
use serde::Deserialize;

use super::time_of_day::TimeOfDay;

#[derive(Debug, Deserialize, PartialEq)]
struct CsvRow {
	#[serde(with = "bearable_date_format")]
	date: NaiveDate,
	weekday: String,

	#[serde(rename = "time of day")]
	time_of_day: TimeOfDay,

	category: String,

	#[serde(rename = "rating/amount")]
	amount: String,

	detail: String,
	notes: String
}

mod bearable_date_format {
	use chrono::NaiveDate;
	use ::regex::Regex;
	use serde::{Deserializer, Deserialize};
	use lazy_static::lazy_static;

	const FORMAT: &'static str = "%d%b%Y";

	pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
		where D: Deserializer<'de>
	{
		lazy_static! {
			static ref DATE_REGEX: Regex = Regex::new(r"(\d*)(\w*) (\w\w\w) (\d\d\d\d)").unwrap();
		}

		let s = String::deserialize(deserializer)?;
		let caps = DATE_REGEX.captures(&s).expect("has captures");

		let day = caps.get(1).unwrap().as_str();
		let month = caps.get(3).unwrap().as_str();
		let year = caps.get(4).unwrap().as_str();
		
		let reformatted = format!("{}{}{}", day, month, year);
		Ok(NaiveDate::parse_from_str(&reformatted, FORMAT).unwrap())
	}
}

fn parse(csv_text: &str) -> Vec<CsvRow> {
	let mut reader = csv::Reader::from_reader(csv_text.as_bytes());
	let iter = reader.deserialize::<CsvRow>();
	Vec::from_iter(iter.map(|s| s.expect("failed parsing line")))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_forBearableCsvString_ProducesCsvRow() {
		let data = r#"date,weekday,time of day,category,rating/amount,detail,notes
"8th Dec 2021","Wednesday","mid","Symptom","2","Neck pain (Moderate)","""#;

		let result = parse(data);

		let expected_row = CsvRow {
			date: NaiveDate::from_ymd(2021, 12, 8),
			weekday: "Wednesday".to_string(),
			time_of_day: TimeOfDay::MID,
			category: "Symptom".to_string(),
			amount: "2".to_string(),
			detail: "Neck pain (Moderate)".to_string(),
			notes: "".to_string(),
		};
		assert_eq!(result.len(), 1);
		assert_eq!(result.first().unwrap(), &expected_row);
	}
}