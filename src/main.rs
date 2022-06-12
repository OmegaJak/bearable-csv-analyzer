mod provider;

use chrono::{NaiveDateTime, NaiveDate};
use gloo_file::{callbacks::FileReader, File};
use log::{debug, info};
use model::{data_manager::DataManager};
use provider::{Provider};
use view_model::scatter_plot::ScatterPlot;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::model::{parser};

mod bindings;
mod model {
    pub mod data_manager;
    pub mod date_map;
    pub mod parser;
    pub mod time_of_day;
    pub mod symptoms {
        pub mod symptom;
    }
}
mod view_model {
    pub mod scatter_plot;
}

enum Msg {
    FetchSymptomScatterplot,
    SetFetchChartResult(ScatterPlot),
    ShowError(String),
    Files(Vec<File>),
    Loaded(String, String),
    SymptomSelectionUpdated(Option<String>),
    StartDateUpdated(Option<String>),
    EndDateUpdated(Option<String>)
}

struct Model {
    error_msg: String,
    readers: HashMap<String, FileReader>,
    csv_text: String,
    data_manager: Option<DataManager>,

    symptom_names: Vec<String>,
    selected_symptom: Option<String>,

    earliest_symptom_date: String,
    latest_symptom_date: String,

    selected_start_date: Option<NaiveDate>,
    selected_end_date: Option<NaiveDate>,
}

static HTML_INPUT_DATE_FORMAT: &str = "%Y-%m-%d";

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error_msg: String::new(),
            readers: HashMap::default(),
            csv_text: String::new(),
            data_manager: None,
            symptom_names: Vec::new(),
            selected_symptom: None,
            earliest_symptom_date: String::new(),
            latest_symptom_date: String::new(),
            selected_start_date: None,
            selected_end_date: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchSymptomScatterplot => {
                debug!("Fetching chart...");
                ctx.link().send_message(
                    match Provider::fetch_chart(&self.data_manager, &self.selected_symptom, &self.selected_start_date, &self.selected_end_date) {
                        Some(scatter_plot) => Msg::SetFetchChartResult(scatter_plot),
                        None => Msg::ShowError("returned null".to_string()),
                    }
                );
                true
            }
            Msg::SetFetchChartResult(data) => {
                Self::show_chart(data);
                true
            }
            Msg::ShowError(msg) => {
                info!("Error: {:?}", msg);
                self.error_msg = msg;
                true
            }
            Msg::Files(files) => {
                info!("Files");
                let first_file = files.first().expect("no files");
                let filename = first_file.name();
                let link = ctx.link().clone();
                let reader = {
                    let filename = filename.clone();
                    gloo_file::callbacks::read_as_text(&first_file, move |res| {
                        info!("Callback");
                        link.send_message(Msg::Loaded(filename, res.expect("failed to read file")))
                    })
                };

                self.readers.insert(filename, reader);

                true
            }
            Msg::Loaded(csv_name, csv_text) => {
                info!("{:?}", csv_text);
                self.csv_text = csv_text;
                self.readers.remove(&csv_name);
                self.data_manager = Some(parser::parse_into_data_manager_str(self.csv_text.as_str())); //TODO: Some async stuff here to avoid hanging?

                if let Some(data_manager) = &self.data_manager {
                    self.symptom_names = data_manager.get_symptom_names()
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect::<Vec<String>>();
                    let selected_symptom = self.symptom_names.first().and_then(|a| Some(a.to_string()));
                    ctx.link().clone().send_message(Msg::SymptomSelectionUpdated(selected_symptom));
                }

                true
            },
            Msg::SymptomSelectionUpdated(symptom) => {
                info!("Received symptom selection {:?}", symptom);
                self.selected_symptom = symptom;
                if let (Some(selection), Some(data_manager)) = (self.selected_symptom.to_owned(), &self.data_manager) {
                    let range = data_manager.get_symptom_date_range(selection.as_str()).expect("Symptom has dates");
                    self.earliest_symptom_date = format_date_for_html(range.start());
                    self.latest_symptom_date = format_date_for_html(range.end());
                }
                true
            }
            Msg::StartDateUpdated(start_date_str) => {
                self.selected_start_date = parse_html_date(start_date_str);
                info!("Parsed updated start date {:?}", self.selected_start_date);
                true
            },
            Msg::EndDateUpdated(end_date) => {
                self.selected_end_date = parse_html_date(end_date);
                info!("Parsed updated end date {:?}", self.selected_end_date);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input type="file" multiple=false accept=".csv" onchange={ctx.link().callback(move |e| Self::on_file_change(e))} />

                <select name="symptom_choice" id="symptom_choice" onchange={ctx.link().callback(move |e| Self::on_symptom_change(e))}>
                    { for self.symptom_names.iter().map(|e| self.view_option(e)) }
                </select>

                <input type="date" id="start_date" name="start_date" 
                    min={self.earliest_symptom_date.to_owned()}
                    max={self.latest_symptom_date.to_owned()}
                    onchange={ctx.link().callback(move |e| Self::on_start_date_change(e))}/>
                <input type="date" id="end_date" name="end_date"
                    min={self.earliest_symptom_date.to_owned()}
                    max={self.latest_symptom_date.to_owned()}
                    onchange={ctx.link().callback(move |e| Self::on_end_date_change(e))}/>

                <button onclick={ctx.link().callback(|_| Msg::FetchSymptomScatterplot)}>{ "Fetch" }</button>
                <p style="color: red;"> { self.error_msg.clone() }</p>
                <svg id="chart" width="960" height="500"></svg>
            </div>
        }
    }
}

impl Model {
    fn show_chart(scatter_plot: ScatterPlot) {
        debug!("Showing chart");
        // call js
        // the bindings are defined in bindings.rs
        bindings::show_chart(JsValue::from_serde(&scatter_plot.points).unwrap());
    }

    fn on_file_change(e: Event) -> Msg {
        info!("On file change");
        let mut result = Vec::new();
        let input: HtmlInputElement = e.target_unchecked_into();

        if let Some(files) = input.files() {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }

    fn on_symptom_change(e: Event) -> Msg {
        info!("On symptom change");
        let value = get_html_input_value(e);

        Msg::SymptomSelectionUpdated(value)
    }

    fn on_start_date_change(e: Event) -> Msg {
        info!("On start date change");
        let value = get_html_input_value(e);
        Msg::StartDateUpdated(value)
    }

    fn on_end_date_change(e: Event) -> Msg {
        //TODO: Holy duplication, Batman!
        info!("On end date change");
        let value = get_html_input_value(e);    
        Msg::EndDateUpdated(value)
    }

    fn view_option(&self, symptom: &str) -> Html {
        let owned_symptom = symptom.to_string();
        html! {
            <option value={owned_symptom}>{ symptom }</option>
        }
    }
}

fn get_html_input_value(inputOnChangeEvent: Event) -> Option<String> {
    let input: HtmlInputElement = inputOnChangeEvent.target_unchecked_into();
    let value = input.value();
    if value.is_empty() {
        return None;
    } else {
        return Some(value);
    }
}

fn format_date_for_html(dateTime: &NaiveDateTime) -> String {
    dateTime.date().format(HTML_INPUT_DATE_FORMAT).to_string()
}

fn parse_html_date(html_date_str: Option<String>) -> Option<NaiveDate> {
    Some(NaiveDate::parse_from_str(html_date_str?.as_str(), HTML_INPUT_DATE_FORMAT).expect("date parsing should work"))
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
