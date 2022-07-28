use serde::Deserialize;
use serde_json::json;

use crate::model::{id::PostId, chat::{Chat, ChatRaw}, user::{User, UserRaw}};
use super::{Context, BASE_API_URL};

const MAX_CONNECTED_POSTS: usize = 10;

#[derive(Debug)]
pub enum ConnectPostError {
	MaxConnectedPostsReached,
	ReqwestError(reqwest::Error),
}

#[derive(Deserialize)]
pub struct ConnectPostResponse {
	pub chats: Vec<ChatRaw>,
	pub users: Vec<UserRaw>,
	pub replies: Vec<ChatRaw>,
}

async fn connect_post(ctx: &Context, posts: &Vec<PostId>) -> Result<ConnectPostResponse, reqwest::Error> {
	let res = ctx.client.post(BASE_API_URL.join("chats/connect").unwrap())
	.header("auth", ctx.auth.to_string())
	.header("content-type", "text/plain") // ????
	.body(serde_json::to_string(&json!({
		"ssid": ctx.simplesocket.get_secure_id(),
		"connect": posts,
		"posts": posts,
	})).unwrap())
	.send()
	.await?
	// .text()
	.json::<ConnectPostResponse>()
	.await?;
	// println!("response is {}", res);
	// todo!("not done");
	Ok(res)
}

impl Context {
	/// Errors if the connected posts exceeds 10
	pub(crate) async fn connect_post(&self, post: PostId) -> Result<ConnectPostResponse,ConnectPostError> {
		let posts = {
			let mut lock = self.posts.lock().unwrap();
			if lock.len() >= MAX_CONNECTED_POSTS {
				return Err(ConnectPostError::MaxConnectedPostsReached);
			}
			lock.push(post);
			lock.clone()
		};

		Ok(connect_post(&self, &posts).await.map_err(|e|ConnectPostError::ReqwestError(e))?)
	}
	pub(crate) async fn connect_post_pop(&self, post: PostId) -> Result<ConnectPostResponse, ConnectPostError> {
		let posts = {
			let mut lock = self.posts.lock().unwrap();
			// i know this is inefficient but who cares
			if lock.len() >= MAX_CONNECTED_POSTS {
				for _ in 0..(MAX_CONNECTED_POSTS - lock.len() + 1) {
					lock.remove(0);
				}
			}
			lock.push(post);
			lock.clone()
		};
		Ok(connect_post(&self, &posts).await.map_err(|e|ConnectPostError::ReqwestError(e))?)
	}
}