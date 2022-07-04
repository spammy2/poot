use std::str::FromStr;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{serde::ts_seconds, DateTime, Utc};

use super::id::{Id, date_from_u64};
use super::user::{self, UserId};
use super::id::hex_id;

#[derive(Deserialize, Serialize)]
pub struct GroupId(
	#[serde(with = "hex_id")]
	pub(crate) u128
);

impl Id for GroupId {
	fn get_date(&self)-> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.0)
	}
}

#[derive(Deserialize)]
struct Group {
	id: GroupId,
	owner: UserId,

	#[serde(with = "ts_seconds")]
    last_checked: DateTime<Utc>
}

#[async_trait]
trait GroupActions {
	async fn is_owner() -> bool;
	async fn delete() -> Result<(), ()>;
}