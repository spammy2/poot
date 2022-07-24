#![feature(trait_alias)]

mod subscriptions;
mod model;

use std::{sync::Arc, collections::HashMap};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use simplesocket::{connect_socket, message::ConnectedResponse};
use model::{user::UserRaw, client_user::ClientUser, post::{Post, PostId, PostRaw}, group::RawGroup};
use subscriptions::general_update::*;
use url::Url;
use lazy_static::lazy_static;

use crate::model::client_user::ClientUserRaw;


#[derive(Clone)]
pub struct Context {
	simplesocket: Arc<simplesocket::Context>,
	pub client: Arc<reqwest::Client>,
	events: Arc<dyn Events + Send + Sync>,
	auth: Auth,
}

lazy_static! {
    static ref BASE_API_URL: Url = Url::parse("https://photop.exotek.co").unwrap();
}

impl Context {
	pub async fn me(&self) -> Result<ClientUser, reqwest::Error> {
		#[derive(Deserialize)]
		struct GetResponse {
			groups: HashMap<String,RawGroup>,
			user: ClientUserRaw,
		}

		let mut url = BASE_API_URL.join("me").unwrap();
		url.query_pairs_mut().append_pair("ss", &self.simplesocket.get_secure_id()[..]);
		let val = self.client.get(url)
			.header("auth", self.auth.to_string())
			.send().await?.json::<GetResponse>().await?;
		todo!();
		// Ok(ClientUser{

		// })
	}
	pub async fn get_post(&self, id: PostId) -> Result<Post, reqwest::Error> {
		#[derive(Deserialize)]
		struct GetResponse {
			users: Vec<UserRaw>,
			posts: Vec<PostRaw>,
		}

		let mut url = BASE_API_URL.join("posts").unwrap();
		url.query_pairs_mut().append_pair("postid", serde_json::to_value(id).unwrap().as_str().unwrap());

		let val = self.client.get(url).send().await?.json::<GetResponse>().await?;

		let GetResponse {mut posts, mut users} = val;
		let post = posts.remove(0);
		let user = users.remove(0);
		let post = Post {
			id: post.id,
			author: user.into(),
			content: post.content,
		};
		return Ok(post);
	}
}

struct InitOptions {
	auth: Auth,
	events: Arc<dyn Events + Send + Sync>
}

#[async_trait]
impl simplesocket::Events for InitOptions {
	async fn on_ready(&self, ctx: Arc<simplesocket::context::Context>, res: ConnectedResponse) {
		let ctx = Arc::new(Context {
			simplesocket: ctx,
			client: Arc::new(reqwest::Client::new()),
			events: self.events.clone(),
			auth: self.auth.clone(),
		});

		let ss = ctx.simplesocket.clone();
		
		
		ss.subscribe(GeneralUpdate{
			location: GeneralUpdateLocation::Home,
			groups: vec![],
		}, GeneralUpdateSubscriber{ctx: ctx.clone()}).await;

		self.events.on_ready(ctx).await;
	}
	async fn on_close(&self, _ctx: Arc<simplesocket::context::Context>) {
        println!("closed");
    }
}

#[derive(Clone)]
pub enum Auth {
	// Username{
	// 	username: String,
	// 	password: String,
	// },
	Token {
		user_id: String,
		token: String,
	},
	None,
}

impl ToString for Auth {
	fn to_string(&self) -> String {
		match self {
			Auth::Token { user_id, token } => format!("{};{}", user_id, token),
			Auth::None => panic!("Auth none"),
		}
	}
}


#[async_trait]
pub trait Events {
	async fn on_post(&self, context: Arc<Context>, post: Post);
	async fn on_ready(&self, context: Arc<Context>);
}

pub struct Client;
impl Client {
	pub async fn new(auth: Auth, events: impl Events + Send + Sync + 'static) {
		connect_socket("61b9724ea70f1912d5e0eb11", "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5", InitOptions {
			events: Arc::new(events),
			auth,
		}).await;
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use dotenv::dotenv;
	use std::env;

	struct BotEvents;
	#[async_trait]
	impl Events for BotEvents {
		async fn on_post(&self, context: Arc<Context>, post: Post) {
			println!("{:?}", post);
		}
		async fn on_ready(&self, context: Arc<Context>) {
			println!("ready");
			let e = context.me().await;
			println!("finished");
		}
	}

	#[tokio::test]
	pub async fn test(){
		dotenv().ok();
		let client = Client::new(
			Auth::Token { user_id: env::var("USER_ID").unwrap(), token: env::var("TOKEN").unwrap() },
			BotEvents{},
		).await;
	}
}