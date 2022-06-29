use serde::Serialize;

use crate::group::GroupId;

#[derive(Serialize)]
pub enum GeneralUpdateLocation {
	Home,
}

#[derive(Serialize)]
#[serde(tag = "Task")]
pub struct GeneralUpdate {
	#[serde(rename = "Location")]
	pub location: GeneralUpdateLocation,
	#[serde(rename = "Groups")]
	#[serde(skip_serializing_if="Vec::is_empty")]
	pub groups: Vec<GroupId>,
}