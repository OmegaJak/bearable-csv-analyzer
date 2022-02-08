mod provider;

use gloo_file::{callbacks::FileReader, File};
use log::{debug, info};
use provider::{Provider, Tick};
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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

enum Msg {
    FetchChart,
    SetFetchChartResult(Vec<Tick>),
    ShowError(String),
    Files(Vec<File>),
    Loaded(String, String),
}

struct Model {
    provider: Rc<Provider>,
    error_msg: String,
    readers: HashMap<String, FileReader>,
    csv: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            provider: Rc::new(Provider {}),
            error_msg: String::new(),
            readers: HashMap::default(),
            csv: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchChart => {
                debug!("Fetching chart...");
                let provider = self.provider.clone();
                ctx.link().send_future(async move {
                    match provider.fetch_chart().await {
                        Ok(entries) => Msg::SetFetchChartResult(entries),
                        Err(err) => Msg::ShowError(format!("{}", err)),
                    }
                });
                false
            }
            Msg::SetFetchChartResult(chart) => {
                Self::show_chart(chart);
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
                self.readers.remove(&csv_name);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input type="file" multiple=false accept=".csv" onchange={ctx.link().callback(move |e| Self::on_file_change(e))} />
                <button onclick={ctx.link().callback(|_| Msg::FetchChart)}>{ "Fetch" }</button>
                <p style="color: red;"> { self.error_msg.clone() }</p>
                <svg id="chart"></svg>
            </div>
        }
    }
}

impl Model {
    fn show_chart(ticks: Vec<Tick>) {
        debug!("Showing chart");
        // call js
        // the bindings are defined in bindings.rs
        bindings::show_chart(JsValue::from_serde(&ticks).unwrap());
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
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
