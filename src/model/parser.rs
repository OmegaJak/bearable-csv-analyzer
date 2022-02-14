use std::iter::FromIterator;

use chrono::NaiveDate;
use serde::Deserialize;

use super::{data_manager::DataManager, date_map::BTreeDateMap, symptoms::symptom::Symptom, time_of_day::TimeOfDay};

#[derive(Debug, Deserialize, PartialEq)]
pub struct CsvRow {
    #[serde(with = "bearable_date_format")]
    pub date: NaiveDate,
    pub weekday: String,

    #[serde(rename = "time of day")]
    pub time_of_day: String,

    pub category: String,

    #[serde(rename = "rating/amount")]
    pub amount: String,

    pub detail: String,
    pub notes: String,
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
    create_data_manager(rows)
}

pub fn parse_into_data_manager_str(csv_text: &str) -> DataManager {
    let rows = parse(csv_text);
    create_data_manager(rows)
}

fn create_data_manager(rows: Vec<CsvRow>) -> DataManager {
    let symptom_rows = rows.iter().filter(|r| r.category == "Symptom");
    let mut symptoms = Vec::<Symptom>::new();
    for symptom_row in symptom_rows {
        let symptom = Symptom::from(&symptom_row);

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
    use assertables::*;
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
    fn Parse_ForCsvContainingOnlySymptoms_CorrectlyParsesSymptoms() {
        let text = r#"date,weekday,time of day,category,rating/amount,detail,notes
"5th Jan 2022","Wednesday","am","Symptom","1","Headache (Mild)",""
"5th Jan 2022","Wednesday","am","Symptom","1","Neck pain (Mild)",""
"5th Jan 2022","Wednesday","mid","Symptom","1","Neck pain (Mild)",""
"5th Jan 2022","Wednesday","pm","Symptom","1","Neck pain (Mild)",""
"5th Jan 2022","Wednesday","pre","Symptom","1","Back (lower) pain (Mild)",""
"5th Jan 2022","Wednesday","am","Symptom","2","Back (lower) pain (Moderate)",""
"5th Jan 2022","Wednesday","mid","Symptom","2","Back (lower) pain (Moderate)",""
"5th Jan 2022","Wednesday","pm","Symptom","2","Back (lower) pain (Moderate)",""
"5th Jan 2022","Wednesday","pre","Symptom","1","Back (mid) pain (Mild)",""
"5th Jan 2022","Wednesday","am","Symptom","3","Back (mid) pain (Severe)",""
"5th Jan 2022","Wednesday","mid","Symptom","3","Back (mid) pain (Severe)",""
"5th Jan 2022","Wednesday","pm","Symptom","4","Back (mid) pain (Unbearable)","""#;
        let date = NaiveDate::from_ymd(2022, 1, 5);

        let data_man = parse_into_data_manager_str(text);

        let expected_symptoms = vec!["Headache", "Neck pain", "Back (lower) pain", "Back (mid) pain"];
        let actual_symptoms = Vec::from_iter(data_man.get_symptom_names().into_iter().map(|s| s as &str));
        assert_bag_eq!(expected_symptoms, actual_symptoms);

        let expected_mid_pain = vec![
            Symptom {
                date: date,
                name: "Back (mid) pain".to_string(),
                severity: 1,
                time_of_day: TimeOfDay::Pre,
            },
            Symptom {
                date: date,
                name: "Back (mid) pain".to_string(),
                severity: 3,
                time_of_day: TimeOfDay::AM,
            },
            Symptom {
                date: date,
                name: "Back (mid) pain".to_string(),
                severity: 3,
                time_of_day: TimeOfDay::MID,
            },
            Symptom {
                date: date,
                name: "Back (mid) pain".to_string(),
                severity: 4,
                time_of_day: TimeOfDay::PM,
            },
        ];
        let expected_mid_pain = Vec::from_iter(expected_mid_pain.into_iter());
        let actual_mid_pain = Vec::from_iter(
            data_man
                .get_all_sorted_symptoms(expected_symptoms[3])
                .unwrap()
                .into_iter()
                .map(|s| s.to_owned()),
        );
        assert_eq!(expected_mid_pain, actual_mid_pain);
    }

    #[test]
    fn Parse_ForRealCsv_Works() {
        let mut reader = Reader::from_path(r#"C:\Users\JAK\Downloads\bearable-export-08-01-2022.csv"#).unwrap();

        let data_man = parse_into_data_manager(reader);

        assert_eq!(true, data_man.get_symptom_names().len() > 0)
    }
}
