#![feature(map_try_insert)]

use std::collections::HashMap;

use super::{
    date_map::{self, BTreeDateMap, OrderedNaiveDateTimeSpan},
    symptoms::symptom::Symptom,
};

pub struct DataManager {
    pub symptoms: HashMap<String, BTreeDateMap<Symptom>>,
}

impl DataManager {
    pub fn from(symptoms: Vec<Symptom>) -> DataManager {
        let mut categorized_symptoms = HashMap::<String, BTreeDateMap<Symptom>>::new();
        for symptom in symptoms {
            if !categorized_symptoms.contains_key(&symptom.name) {
                categorized_symptoms.insert(symptom.name.to_string(), BTreeDateMap::<Symptom>::new());
            }
            let date_map = categorized_symptoms.get_mut(&symptom.name).expect("Should have just added it");
            match symptom.date_time_span() {
                Ok(date_time_span) => {
                    date_map.insert(date_time_span, symptom);
                }
                Err(_) => {}
            }
        }

        DataManager {
            symptoms: categorized_symptoms,
        }
    }
}
