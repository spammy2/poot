#![feature(trait_alias)]
use std::{borrow::{Borrow, BorrowMut}, sync::Arc};

use async_trait::async_trait;
use simplesocket::{connect_socket, message::ConnectedResponse};
use subs::{GeneralUpdate, GeneralUpdateLocation};
use user::UserRef;
mod subs;
mod user;
mod group;

#[derive(Clone)]
pub struct Context {
	simplesocket: Arc<simplesocket::Context>,
	pub client: Arc<reqwest::Client>,
}


impl Context {
	pub fn me(&self) {
		
	}
}


struct SimpleSocketEvents {
	auth: Auth,
	callback: Option<Box<dyn Fn(Context) + Send + Sync + 'static>>,
}

#[async_trait]
impl simplesocket::Events for SimpleSocketEvents {
	async fn on_ready(&mut self, ctx: simplesocket::context::Dispatch, res: ConnectedResponse) {
		let ctx = Context {
			simplesocket: ctx.0,
			client: Arc::new(reqwest::Client::new()),
		};
		let ss = ctx.clone().simplesocket;
		ss.subscribe(GeneralUpdate{
			location: GeneralUpdateLocation::Home,
			groups: vec![],
		}, |event|{
			println!("POST {:?}", event);
		}).await;
		(self.callback.take().unwrap())(ctx);
	}
	async fn on_close(&mut self, _ctx: simplesocket::context::Dispatch) {
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

struct Message {
	author: UserRef,
	content: String,

}

#[async_trait]
trait PhotopEvents {
	fn on_post(&mut self, context: Context, message: Message) {
		
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
