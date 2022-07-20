use std::fmt::Display;

use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};
use super::{user::{User, UserId}, id::{Id, date_from_u64}};
use super::id::hex_id;

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct PostId(
	#[serde(with = "hex_id")]
	pub(crate) u128
);

impl Id for PostId {		
	fn get_date(&self)-> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.0)
	}
}

#[derive(Debug)]
pub struct Post {
	pub id: PostId,
	pub author: User,
	pub content: String,
}


#[derive(Deserialize)]
pub (crate) struct PostRaw {
	#[serde(rename = "_id")]
	pub id: PostId,
	#[serde(rename = "Text")]
	pub content: String,

	#[serde(rename = "HasMentions")]
	pub has_mentions: bool,

	#[serde(rename = "UserID")]
	pub user: UserId,

	#[serde(rename = "Timestamp", with="ts_milliseconds")]
	pub created_at: DateTime<Utc>
}