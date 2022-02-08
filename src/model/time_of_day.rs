use std::{str::FromStr};

use chrono::{NaiveTime, Duration, NaiveDateTime};
use serde::Deserialize;
use timespan::{NaiveTimeSpan};

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TimeOfDay {
	#[serde(rename = "")]
	None,
	Pre,
	AM,
	MID,
	PM,
	AllDay
}

fn span(time: NaiveTime) -> Result<NaiveTimeSpan, timespan::Error> {
	NaiveTimeSpan::new(time, time + Duration::hours(6))
}

impl TimeOfDay {
	pub fn span(&self) -> Result<NaiveTimeSpan, timespan::Error> {
		match self {
			TimeOfDay::Pre => span(NaiveTime::from_hms(0, 0, 0)),
			TimeOfDay::AM => span(NaiveTime::from_hms(6, 0, 0)),
			TimeOfDay::MID => span(NaiveTime::from_hms(12, 0, 0)),
			TimeOfDay::PM => span(NaiveTime::from_hms(18, 0, 0)),
			TimeOfDay::None => todo!(),
    	TimeOfDay::AllDay => NaiveTimeSpan::new(NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(23, 59, 59)),
		}
	}
}