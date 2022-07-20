#![feature(trait_alias)]
use std::{borrow::{Borrow, BorrowMut}, sync::Arc};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use simplesocket::{connect_socket, message::ConnectedResponse, context::Subscriber};
use model::{user::{User, UserRaw}, post::{Post, PostId, PostRaw}, id::hex_id};
use subscriptions::general_update::{GeneralUpdate, GeneralUpdateLocation, GeneralUpdateEvent};
use url::Url;
mod subscriptions;
use lazy_static::lazy_static;

mod model;

#[derive(Clone)]
pub struct Context {
	simplesocket: Arc<simplesocket::Context>,
	pub client: Arc<reqwest::Client>,
	events: Arc<dyn Events + Send + Sync>,
}

lazy_static! {
    static ref BASE_API_URL: Url = Url::parse("https://photop.exotek.co").unwrap();
}

impl Context {
	pub fn me(&self) {
		let mut url = BASE_API_URL.join("temp/signin").unwrap();
		url.query_pairs_mut().append_pair("ss", &self.simplesocket.get_secure_id()[..]);
	}
	pub async fn get_post(&self, id: PostId) -> Result<Post, reqwest::Error> {
		let mut url = BASE_API_URL.join("posts").unwrap();
		url.query_pairs_mut().append_pair("postid", serde_json::to_value(id).unwrap().as_str().unwrap());
		#[derive(Deserialize)]
		struct GetResponse {
			users: Vec<UserRaw>,
			posts: Vec<PostRaw>,
		}

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

struct GeneralUpdateSubscriber {
	ctx: Arc<Context>,
}

impl Subscriber for GeneralUpdateSubscriber {
	fn callback(&self, event: Value){
		let event: GeneralUpdateEvent = serde_json::from_value(event).unwrap();
		match event {
			GeneralUpdateEvent::NewPostAdded(post) => {
				let ctx = self.ctx.clone();
				tokio::spawn(async move {
					let post = ctx.get_post(post.post_id).await.unwrap();
					ctx.events.on_post(ctx.clone(), post);
				});
			}
			_ => {println!("Unknnown event: {:?}", event);}
		}
	}
}

#[async_trait]
impl simplesocket::Events for InitOptions {
	async fn on_ready(&self, ctx: Arc<simplesocket::context::Context>, res: ConnectedResponse) {
		let ctx = Context {
			simplesocket: ctx,
			client: Arc::new(reqwest::Client::new()),
			events: self.events.clone(),
		};
		let ss = ctx.clone().simplesocket;
		
		ss.subscribe(GeneralUpdate{
			location: GeneralUpdateLocation::Home,
			groups: vec![],
		}, GeneralUpdateSubscriber{ctx: Arc::new(ctx)}).await;
		// (self.callback.as_ref().unwrap())(ctx);
	}
	async fn on_close(&self, _ctx: Arc<simplesocket::context::Context>) {
        println!("closed");
    }
}

pub enum Auth {
	Username{
		username: String,
		password: String,
	},
	Token {
		user_id: String,
		token: String,
	},
	None,
}


pub trait Events {
	fn on_post(&self, context: Arc<Context>, post: Post);
	fn on_ready(&self, context: Arc<Context>);
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

struct BotEvents;
impl Events for BotEvents {
	fn on_post(&self, context: Arc<Context>, post: Post) {
		println!("{:?}", post);
	}
	fn on_ready(&self, context: Arc<Context>) {
		println!("ready");
	}
}

#[tokio::test]
async fn test(){
	Client::new(
		Auth::Token { user_id: "".to_owned(), token: "".to_owned() },
		BotEvents{},
	).await;
}
