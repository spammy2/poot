use serde::Serialize;
use serde_json::json;

use crate::model::{id::{PostId, ChatId}, post::Post, chat::Chat};

use super::BASE_API_URL;

pub struct CreateChatBody {
    pub content: String,
    pub post_id: PostId,
}

impl super::Context {
    pub(crate) async fn create_chat(&self, body: CreateChatBody) -> Result<ChatId, reqwest::Error> {
        let mut chat_url = BASE_API_URL.join("chats/new").unwrap();
		chat_url
			.query_pairs_mut()
			.append_pair("postid", &body.post_id.to_string());

        let chat_id = self
            .client
            .post(chat_url)
            .header("auth", self.auth.to_string())
            .body(serde_json::to_string(&json!({
				"text": body.content,
			})).unwrap())
            .send()
            .await?
            .text()
            .await?;
		Ok(ChatId::from(&chat_id[..]))
    }
}
