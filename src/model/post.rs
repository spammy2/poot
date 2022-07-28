use crate::context::{Context, create_chat::CreateChatBody};

use super::{
    id::{PostId, UserId, ChatId},
    user::User, chat::Chat,
};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize};
#[derive(Debug)]
pub struct Post {
    pub id: PostId,
    pub author: User,
    pub content: String,
}

impl Post {
	/// Panics if the client is not authenticated.
	pub async fn create_chat(&self, ctx: &Context, content: String) -> Result<ChatId, reqwest::Error> {
		ctx.create_chat(CreateChatBody {
			content,
			post_id: self.id.clone(),
		}).await
	}
}

fn default_false() -> bool {
	false
}

fn default_0() -> u64 {
	0
}

#[derive(Deserialize)]
pub(crate) struct PostRaw {
    #[serde(rename = "_id")]
    pub id: PostId,
    #[serde(rename = "Text")]
    pub content: String,

	#[serde(rename = "Chats", default="default_0")]
	pub chat_count: u64,

    #[serde(rename = "HasMentions", default="default_false")]
    pub has_mentions: bool,

    #[serde(rename = "UserID")]
    pub user: UserId,

    #[serde(rename = "Timestamp", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}
