use std::{collections::HashMap, iter::FromIterator, ops::{RangeBounds, RangeInclusive}};

use chrono::NaiveDateTime;
use log::debug;

use crate::view_model::scatter_plot::{ScatterPlot, DateTimeValuePoint};

use super::{
    date_map::{BTreeDateMap, OrderedNaiveDateTimeSpan},
    symptoms::symptom::Symptom,
};

pub struct DataManager {
    symptoms: HashMap<String, BTreeDateMap<Symptom>>,
}

impl DataManager {
    pub fn from(symptoms: Vec<Symptom>) -> DataManager {
        let mut categorized_symptoms = HashMap::<String, BTreeDateMap<Symptom>>::new();
        for symptom in symptoms {
            debug!("Processing {:?}", symptom);
            if !categorized_symptoms.contains_key(&symptom.name) {
                categorized_symptoms.insert(symptom.name.to_string(), BTreeDateMap::<Symptom>::new());
                debug!("Inserting {}", &symptom.name);
            }
            let date_map = categorized_symptoms.get_mut(&symptom.name).expect("Should have just added it");
            date_map.insert(symptom.date_time_span().unwrap(), symptom);
        }

        DataManager {
            symptoms: categorized_symptoms,
        }
    }

    pub fn get_symptom_names(&self) -> Vec<&String> {
        Vec::from_iter(self.symptoms.keys().into_iter())
    }

    pub fn get_symptom_date_range(&self, symptom_name: &str) -> Option<RangeInclusive<NaiveDateTime>> {
        let map = self.symptoms.get(symptom_name)?;
        let min = map.min()?.0.start;
        let max = map.max()?.0.start;
        return Some(min..=max)
    }

    pub fn get_all_sorted_symptoms(&self, symptom_name: &str) -> Option<Vec<&Symptom>> {
        let map = self.symptoms.get(symptom_name)?;
        Some(Vec::from_iter(map.values().into_iter()))
    }

    pub fn get_basic_symptoms_scatterplot<R>(&self, symptom_name: &str, range: R) -> Option<ScatterPlot>
        where R: RangeBounds<OrderedNaiveDateTimeSpan>
    {
        let map = self.symptoms.get(symptom_name)?;
        let values = map.range(range)
            .into_iter()
            .map(|(k, v)| DateTimeValuePoint {
                x: k.start,
                y: v.severity
            })
            .collect::<Vec<DateTimeValuePoint>>();
        return Some(ScatterPlot { points: values });
    }
}

#[cfg(test)]
mod tests {
    use assertables::*;
    use chrono::NaiveDate;
    use wasm_bindgen::__rt::assert_not_null;

    use crate::model::time_of_day::TimeOfDay;

    use super::*;

    #[test]
    fn GetSymptomDateRange_DoesThingsIdk() {
        let symptom_name = "Back (mid) pain";
        let symptoms = vec![
            Symptom {
                date: NaiveDate::from_ymd(2022, 1, 5),
                name: symptom_name.to_string(),
                severity: 1,
                time_of_day: TimeOfDay::Pre,
            },
            Symptom {
                date: NaiveDate::from_ymd(2022, 1, 6),
                name: symptom_name.to_string(),
                severity: 3,
                time_of_day: TimeOfDay::Pre,
            },
        ];
        let data_man = DataManager::from(symptoms.clone());

        let range = data_man.get_symptom_date_range(symptom_name);

        assert!(range.is_some());
        assert_eq!(range.as_ref().expect("").start().date(), symptoms[0].date);
        assert_eq!(range.as_ref().expect("").end().date(), symptoms[1].date);
    }
}
