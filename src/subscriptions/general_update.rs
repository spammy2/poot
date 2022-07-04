use serde::{Serialize, Deserialize, de::Error};
use crate::model::{group::GroupId, user::UserId, post::PostId};

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

#[derive(Deserialize)]
pub struct NewPostAdded {
	#[serde(rename = "Timestamp")]
	pub timestamp: u64,
	#[serde(rename = "UserID")]
	pub user_id: UserId,
	#[serde(rename = "_id")]
	pub post_id: PostId,
}

pub enum GeneralUpdateEvent {
	NewPostAdded(NewPostAdded)
}

impl<'de> Deserialize<'de> for GeneralUpdateEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
		let v = serde_json::Value::deserialize(deserializer)?;
		let v = v.as_object().ok_or(Error::custom("shit"))?;

		match v.get("Type").unwrap().as_str().unwrap() {
			"NewPostAdded" => {
				Ok(GeneralUpdateEvent::NewPostAdded(serde_json::from_value(v.get("NewPostData").unwrap().to_owned()).unwrap()))	
			},
			_ => Err(Error::custom("Not a valid type"))
		}
    }
}
