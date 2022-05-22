use chrono::{NaiveDateTime};
use serde::Serialize;

pub struct ScatterPlot {
	pub points: Vec<DateTimeValuePoint>
}


#[derive(Debug, Serialize)]
pub struct DateTimeValuePoint {
	pub x: NaiveDateTime,
	pub y: u8
}