use crate::{model::{data_manager::DataManager, date_map::OrderedNaiveDateTimeSpan}, view_model::scatter_plot::ScatterPlot};

pub struct Provider {
}

impl Provider {
    pub fn fetch_chart(data_manager: &Option<DataManager>, symptom: &Option<String>) -> Option<ScatterPlot> {
        let symptom_name = match symptom {
            Some(symptom) => symptom,
            None => data_manager.as_ref()?.get_symptom_names()[0],
        };
        let range = 
            OrderedNaiveDateTimeSpan("2021-11-19T11:30:00 - 2021-11-19T11:31:00".parse().unwrap())..
            OrderedNaiveDateTimeSpan("2021-11-25T11:30:00 - 2021-11-25T11:31:00".parse().unwrap());
        return data_manager.as_ref()?.get_basic_symptoms_scatterplot(symptom_name, range);
    }
}
