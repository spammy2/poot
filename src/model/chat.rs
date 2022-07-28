use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;

use super::{id::{ChatId, UserId, PostId}, user::User};

pub struct Chat {
	pub id: ChatId,
	pub content: String,
	pub author: User,
	pub created_at: DateTime<Utc>,
	pub post_id: PostId,
}

impl Chat {
	pub fn from_raw(raw: ChatRaw, user: User) -> Chat {
		Chat {
			id: raw.id,
			content: raw.content,
			author: user,
			created_at: raw.created_at,
			post_id: raw.post_id,
		}
	}
}

impl PartialEq for Chat {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

#[derive(Deserialize)]
pub struct ChatRaw {
	#[serde(rename = "_id")]
	pub id: ChatId,

	#[serde(rename = "Text")]
	pub content: String,

	#[serde(rename = "UserID")]
	pub author: UserId,

	#[serde(rename = "Timestamp", with = "ts_milliseconds")]
	pub created_at: DateTime<Utc>,

	#[serde(rename = "PostID")]
	pub post_id: PostId,
}

