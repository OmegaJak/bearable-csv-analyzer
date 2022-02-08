use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimeOfDay {
	PRE,
	AM,
	MID,
	PM
}