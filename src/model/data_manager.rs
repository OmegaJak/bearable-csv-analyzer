#![feature(map_try_insert)]

use std::{collections::HashMap, iter::FromIterator};

use log::debug;

use super::{
    date_map::{self, BTreeDateMap, OrderedNaiveDateTimeSpan},
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

    pub fn get_all_sorted_symptoms(&self, symptom_name: &str) -> Option<Vec<&Symptom>> {
        let map = self.symptoms.get(symptom_name);
        Some(Vec::from_iter(map?.values().into_iter()))
    }
}
