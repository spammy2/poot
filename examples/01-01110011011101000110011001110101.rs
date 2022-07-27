use poot::*;
use std::{sync::Arc};
use async_trait::async_trait;

struct BotEvents;
#[async_trait]
impl Events for BotEvents {
	async fn on_ready(&self, context: Arc<Context>) {
		let result = context.create_post(CreatePostBody::from(String::from("bread"))).await;
		match result {
			Ok(post) => {
				println!("Post successfully creative")
			},
			Err(err) => {
				println!("not successful :(")
			},
		}
	}
}

#[tokio::main]
async fn main(){
	Client::new(
		Auth::Token {
			user_id: String::from("YER USERID HERE"),
			token: String::from("YER TOKEN HERE"),
		},
		BotEvents,
	).await;
}