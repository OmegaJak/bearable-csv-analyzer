use chrono::{NaiveDate, NaiveTime, Duration};
use timespan::Span;

use crate::{model::{data_manager::DataManager, date_map::OrderedNaiveDateTimeSpan}, view_model::scatter_plot::ScatterPlot};

pub struct Provider {
}

impl Provider {
    pub fn fetch_chart(data_manager: &Option<DataManager>, symptom: &Option<String>, start_date: &Option<NaiveDate>, end_date: &Option<NaiveDate>) -> Option<ScatterPlot> {
        let symptom_name = match symptom {
            Some(symptom) => symptom,
            None => data_manager.as_ref()?.get_symptom_names()[0],
        };
        let start_span = match start_date {
            Some(start_date) => {
                let naive_date_time = start_date.and_time(NaiveTime::from_hms(0, 0, 0));
                let span = Span::new(naive_date_time, naive_date_time.checked_add_signed(Duration::seconds(1)).unwrap()).unwrap();
                OrderedNaiveDateTimeSpan(span)
            }
            None => OrderedNaiveDateTimeSpan("2021-11-19T11:30:00 - 2021-11-19T11:31:00".parse().unwrap()),
        };
        let end_span = match end_date {
            Some(end_date) => {
                let naive_date_time = end_date.and_time(NaiveTime::from_hms(11, 59, 59));
                let span = Span::new(naive_date_time.checked_sub_signed(Duration::seconds(1)).unwrap(), naive_date_time).unwrap();
                OrderedNaiveDateTimeSpan(span)
            }
            None => OrderedNaiveDateTimeSpan("2021-11-25T11:30:00 - 2021-11-25T11:31:00".parse().unwrap()),
        };
        let range = start_span..end_span;
        return data_manager.as_ref()?.get_basic_symptoms_scatterplot(symptom_name, range);
    }
}
