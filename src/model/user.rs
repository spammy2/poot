use serde::Deserialize;

use super::id::{Id, date_from_u64};
use super::id::hex_id;
pub struct User {
	id: UserId,
	username: String,

}

#[derive(Deserialize)]
pub struct UserId(
	#[serde(with = "hex_id")]
	pub(crate) u128
);

impl Id for UserId {
	fn get_date(&self)-> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.0)
	}
}
