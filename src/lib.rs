#![feature(trait_alias)]

pub mod context;
mod model;
mod subscriptions;

use async_trait::async_trait;
use context::{Subscriptions};
use serde::{Serialize, Deserialize};
use simplesocket::{connect_socket, message::{ConnectedResponse, ServerResponse}};
use std::{sync::{Arc, Mutex, Weak, RwLock}, cell::RefCell};
use subscriptions::general_update::*;

use crate::model::{user::UserRaw, chat::ChatRaw};

pub use context::Context;
pub use model::{post::Post, chat::Chat, user::User, client_user::ClientUser, id::{Id,UserId,PostId,ChatId}};

struct InitOptions {
    auth: Auth,
    events: Arc<dyn Events + Send + Sync>,
	ctx: Arc<Mutex<Option<Context>>>,
}

#[async_trait]
impl simplesocket::Events for InitOptions {
    async fn on_ready(&self, ss: Arc<simplesocket::context::Context>, _res: ConnectedResponse) {
		let ctx_ref = Arc::new(RwLock::new(None));
		
        let ctx = Context {
			subscriptions: Subscriptions {
				general_update: ss.subscribe(
					GeneralUpdate {
						location: GeneralUpdateLocation::Home,
						groups: vec![],
						auth: self.auth.clone(),
					},
					GeneralUpdateSubscriber { ctx: ctx_ref.clone() },
				)
				.await
			},
            simplesocket: ss,
            client: reqwest::Client::new(),
            events: self.events.clone(),
            auth: self.auth.clone(),
			posts: Arc::new(Mutex::new(Vec::new())),
        };

		self.ctx.lock().unwrap().replace(ctx.clone()); // i think lock is immediatelly dropped
		ctx_ref.write().unwrap().replace(ctx.clone());

        self.events.on_ready(ctx).await;
    }
	async fn on_remote(&self, ss: Arc<simplesocket::context::Context>, res: simplesocket::message::RemoteResponse){
		let ctx = self.ctx.lock().unwrap().as_ref().expect("Not init???").clone();
		// https://stackoverflow.com/a/32790546/11309351 ??

		#[derive(Deserialize)]
		struct NewChat {
			chat: ChatRaw,
			users: Vec<UserRaw>,
		}

		#[derive(Deserialize)]
		#[serde(tag = "type")]
		enum StreamValue {
			#[serde(rename = "chat")]
			NewChat(NewChat)
		}

		match res.name.as_str() {
			"stream" => {
				let value: StreamValue = serde_json::from_value(res.data).unwrap();
				match value {
					StreamValue::NewChat(NewChat { chat, mut users }) => {
						let chat = Chat::from_raw(chat, users.remove(0).into());
						ctx.events.on_chat(ctx.clone(), chat).await;
					}
				}
			},
			_ => {
				println!("unknown {:?}", res)
			}
		}
	}
    async fn on_close(&self, _ctx: Arc<simplesocket::context::Context>) {
        println!("closed");
    }
}

#[derive(Clone,Serialize)]
#[serde(untagged)]
pub enum Auth {
	// too hard to implement, also discouraged by robot
    // Username{
    // 	username: String,
    // 	password: String,
    // },
    Token { user_id: String, token: String },
    None,
}
impl Auth {
	fn is_none(&self) -> bool {
		match self {
			Auth::None => true,
			_ => false,
		}
	}
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
    async fn on_post(&self, _context: Context, _post: Post) {
        // ...
    }
    async fn on_ready(&self, _context: Context) {
        // ...
    }
	async fn on_chat(&self, _context: Context, _chat: Chat) {
		// ...
	}
}

pub struct Client;
impl Client {
    pub async fn new(auth: Auth, events: impl Events + Send + Sync + 'static) {
        connect_socket(
            "61b9724ea70f1912d5e0eb11",
            "client_a05cd40e9f0d2b814249f06fbf97fe0f1d5",
            InitOptions {
                events: Arc::new(events),
                auth,
				ctx: Arc::new(Mutex::new(None)),
            },
        )
        .await;
    }
}

#[cfg(test)]
mod test {
    use crate::model::id::UserId;

    use super::*;
    use dotenv::dotenv;
    use std::env;

    struct BotEvents;
    #[async_trait]
    impl Events for BotEvents {
        async fn on_post(&self, ctx: Context, post: Post) {
			post.connect_pop(&ctx).await.unwrap();
			let p = post.create_chat(&ctx, "post event and connected to chat").await;
			match p {
				Ok(chat_id) => println!("Created chat {}", chat_id),
				Err(e) => println!("Error: {}", e),
			}
		}
		async fn on_chat(&self, ctx: Context, chat: Chat) {
			if !chat.author.is_self(&ctx) {
				chat.post_id.create_chat(&ctx, format!("Received chat from {}", chat.author.username).as_str()).await.unwrap();
			}
		}
        async fn on_ready(&self, ctx: Context) {
			println!("ready");
            // let post = ctx
            //     .create_post(CreatePostBody {
            //         content: "test create post".to_string(),
            //         // group_id: Some(GroupId::from("620722323f99d655b9afe2fa")),
            //         group_id: None,
            //         images: vec![],
            //     })
            //     .await;
            // tf it's not returning
            // println!("FINISHED {:?}", post);
        }
    }

    #[tokio::test]
    pub async fn test() {
        dotenv().ok();
        Client::new(
            Auth::Token {
                user_id: env::var("USER_ID").unwrap(),
                token: env::var("TOKEN").unwrap(),
            },
            BotEvents {},
        )
        .await;
    }
}
