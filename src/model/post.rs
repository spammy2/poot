use serde::{Deserialize, Serialize};
use super::{user::User, id::{Id, date_from_u64}};
use super::id::hex_id;

#[derive(Deserialize, Serialize)]
pub struct PostId(
	#[serde(with = "hex_id")]
	pub(crate) u128
);

impl Id for PostId {		
	fn get_date(&self)-> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.0)
	}
}

pub struct Post {
	pub id: PostId,
	pub author: User,
	pub content: String,
}