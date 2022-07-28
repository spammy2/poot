use crate::context::{Context, create_chat::CreateChatBody, connect_post::{ConnectPostError, ConnectPostResponse}};

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
	/// Connect to the post, listen for new chat messages.
	/// Errors if too many posts are connected.
	/// If successful, returns `ConnectedPostResponse`, which contains raw versions of most recent chats.
	/// Most of the time this value is useless so it is best to just discard it.
	pub async fn connect(&self, context: &Context) -> Result<ConnectPostResponse, ConnectPostError> {
		context.connect_post(self.id).await
	}

	/// Similar to `connect`, but will unsubscribe from the already connected ones if there are too many subscribed.
	pub async fn connect_pop(&self, context: &Context) -> Result<ConnectPostResponse, ConnectPostError> {
		context.connect_post_pop(self.id).await
	}

	/// Panics if the client is not authenticated.
	pub async fn create_chat(&self, ctx: &Context, content: &str) -> Result<ChatId, reqwest::Error> {
		ctx.create_chat(CreateChatBody {
			content,
			post_id: self.id,
		}).await
	}
}

impl PostId {
	// can take self because Id is Copy
	pub async fn connect(self, context: &Context) -> Result<ConnectPostResponse, ConnectPostError> {
		context.connect_post(self).await
	}
	pub async fn connect_pop(self, context: &Context) -> Result<ConnectPostResponse, ConnectPostError> {
		context.connect_post_pop(self).await
	}
	pub async fn create_chat(self, ctx: &Context, content: &str) -> Result<ChatId, reqwest::Error> {
		ctx.create_chat(CreateChatBody {
			content,
			post_id: self,
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
