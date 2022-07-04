#![feature(trait_alias)]
use std::{borrow::{Borrow, BorrowMut}, sync::Arc};
use async_trait::async_trait;
use serde_json::Value;
use simplesocket::{connect_socket, message::ConnectedResponse, context::Subscriber};
use model::{user::User, post::{Post, PostId}, id::hex_id};
use subscriptions::general_update::{GeneralUpdate, GeneralUpdateLocation, GeneralUpdateEvent};
use url::Url;
mod subscriptions;
mod model;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Context {
	simplesocket: Arc<simplesocket::Context>,
	pub client: Arc<reqwest::Client>,
}

lazy_static! {
    static ref BASE_API_URL: Url = Url::parse("https://photop.exotek.co").unwrap();
}

impl Context {
	pub fn me(&self) {
		
	}
	pub async fn get_post(&self, id: PostId) -> Result<Post, reqwest::Error> {
		let mut url = BASE_API_URL.join("posts").unwrap();
		url.query_pairs_mut().append_pair("postid", serde_json::to_value(id).unwrap().as_str().unwrap());
		let val = self.client.get(url).send().await?.json::<serde_json::Value>().await?;
		println!("{:#?}", val);
		todo!();
		// let post: Post = serde_json::from_value(val).unwrap();
		// return Ok(post);
	}
}

struct SimpleSocketEvents {
	auth: Auth,
	callback: Option<Box<dyn Fn(Context) + Send + Sync + 'static>>,
}

fn a(t: &mut Vec<Box<dyn Fn(Context) + Send + Sync + 'static>>, cb: impl Fn(Context) + Send + Sync + 'static) {
	t.push(Box::new(cb));
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
					println!("{:?}", post.content);
				});
			}
			_ => {}
		}
	}
}

#[async_trait]
impl simplesocket::Events for SimpleSocketEvents {
	async fn on_ready(&self, ctx: Arc<simplesocket::context::Context>, res: ConnectedResponse) {
		let ctx = Context {
			simplesocket: ctx,
			client: Arc::new(reqwest::Client::new()),
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

enum Auth {
	UsernameAndPassword{
		username: String,
		password: String,
	},
	Token {
		user_id: String,
		token: String,
	},
	None,
}

#[async_trait]
trait PhotopEvents {
	fn on_post(&mut self, context: Context, post: Post) {
		
	}
}

pub trait Callback = Fn(Context) + Send + Sync + 'static;

pub struct Client;
impl Client {
	pub async fn from_password(username: impl Into<String>, password: impl Into<String>, callback: impl Callback) {
		connect_socket("61b9724ea70f1912d5e0eb11", "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5", SimpleSocketEvents {
			callback: Some(Box::new(callback)),
			auth: Auth::UsernameAndPassword {
				username: username.into(),
				password: password.into(),
			}
        }).await;
	}
	pub async fn from_token(user_id: impl Into<String>, token: impl Into<String>, callback: impl Callback) {
		connect_socket("61b9724ea70f1912d5e0eb11", "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5", SimpleSocketEvents {
			callback: Some(Box::new(callback)),
			auth: Auth::Token {
				user_id: user_id.into(),
				token: token.into(),
			}
		}).await;
	}
	pub async fn from_guest(callback: impl Callback) {
		connect_socket("61b9724ea70f1912d5e0eb11", "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5", SimpleSocketEvents {
			callback: Some(Box::new(callback)),
			auth: Auth::None,
		}).await;
	}
}

#[tokio::test]
async fn test(){
	Client::from_guest(|ctx|{
		println!("Client is ready");
	}).await;
}
