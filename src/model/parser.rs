use std::iter::FromIterator;

use chrono::NaiveDate;
use serde::Deserialize;

use super::{data_manager::DataManager, date_map::BTreeDateMap, symptoms::symptom::Symptom, time_of_day::TimeOfDay};

#[derive(Debug, Deserialize, PartialEq)]
struct CsvRow {
    #[serde(with = "bearable_date_format")]
    date: NaiveDate,
    weekday: String,

    #[serde(rename = "time of day")]
    time_of_day: String,

    category: String,

    #[serde(rename = "rating/amount")]
    amount: String,

    detail: String,
    notes: String,
}

mod bearable_date_format {
    use ::regex::Regex;
    use chrono::NaiveDate;
    use lazy_static::lazy_static;
    use serde::{Deserialize, Deserializer};

    const FORMAT: &'static str = "%d%b%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
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

pub fn parse_into_data_manager<R: std::io::Read>(reader: csv::Reader<R>) -> DataManager {
    let rows = parse_rdr(reader);
    let symptom_rows = rows.iter().filter(|r| r.category == "Symptom");
    let mut symptoms = Vec::<Symptom>::new();
    for symptom_row in symptom_rows {
        let symptom = Symptom {
            name: symptom_row.detail.to_string(),
            time_of_day: serde_plain::from_str::<TimeOfDay>(&symptom_row.time_of_day).unwrap(),
            severity: str::parse::<u8>(&symptom_row.amount).expect("Failed to parse symptom amount"),
            date: symptom_row.date,
        };

        symptoms.push(symptom);
    }

    DataManager::from(symptoms)
}

fn parse(csv_text: &str) -> Vec<CsvRow> {
    let reader = csv::Reader::from_reader(csv_text.as_bytes());
    parse_rdr(reader)
}

fn parse_rdr<R: std::io::Read>(mut reader: csv::Reader<R>) -> Vec<CsvRow> {
    let iter = reader.deserialize::<CsvRow>();
    Vec::from_iter(iter.map(|s| s.expect("failed parsing line")))
}

#[cfg(test)]
mod tests {
    use csv::Reader;

    use super::*;

    #[test]
    fn Parse_ForBearableCsvString_ProducesCsvRow() {
        let data = r#"date,weekday,time of day,category,rating/amount,detail,notes
"8th Dec 2021","Wednesday","mid","Symptom","2","Neck pain (Moderate)","""#;

        let result = parse(data);

        let expected_row = CsvRow {
            date: NaiveDate::from_ymd(2021, 12, 8),
            weekday: "Wednesday".to_string(),
            time_of_day: "mid".to_string(),
            category: "Symptom".to_string(),
            amount: "2".to_string(),
            detail: "Neck pain (Moderate)".to_string(),
            notes: "".to_string(),
        };
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap(), &expected_row);
    }

    #[test]
    fn Parse_ForRealCsv_Works() {
        let mut reader = Reader::from_path(r#"C:\Users\JAK\Downloads\bearable-export-08-01-2022.csv"#).unwrap();

        let data_man = parse_into_data_manager(reader);

        assert_eq!(true, data_man.symptoms.len() > 0)
    }
}
