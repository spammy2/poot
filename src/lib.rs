#![feature(trait_alias)]

pub mod context;
mod model;
mod subscriptions;

use async_trait::async_trait;
use context::Context;
use model::{
    post::{Post},
};


use serde::{Serialize};
use simplesocket::{connect_socket, message::ConnectedResponse};
use std::{sync::Arc};
use subscriptions::general_update::*;
struct InitOptions {
    auth: Auth,
    events: Arc<dyn Events + Send + Sync>,
}

#[async_trait]
impl simplesocket::Events for InitOptions {
    async fn on_ready(&self, ctx: Arc<simplesocket::context::Context>, _res: ConnectedResponse) {
        let ctx = Context {
            simplesocket: ctx,
            client: Arc::new(reqwest::Client::new()),
            events: self.events.clone(),
            auth: self.auth.clone(),
        };

        let ss = ctx.simplesocket.clone();

		println!("subscribing ss");
        ss.subscribe(
            GeneralUpdate {
                location: GeneralUpdateLocation::Home,
                groups: vec![],
				auth: self.auth.clone(),
            },
            GeneralUpdateSubscriber { ctx: ctx.clone() },
        )
        .await;

        self.events.on_ready(ctx).await;
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
            },
        )
        .await;
    }
}

#[cfg(test)]
mod test {
    use crate::context::create_post::CreatePostBody;

    use super::*;
    use dotenv::dotenv;
    use std::env;

    struct BotEvents;
    #[async_trait]
    impl Events for BotEvents {
        async fn on_post(&self, ctx: Context, post: Post) {
            if post.content == "test create post" {
				let p = post.create_chat(&ctx, "test receive message".to_owned()).await;
				match p {
					Ok(chat_id) => println!("Created chat {}", chat_id),
					Err(e) => println!("Error: {}", e),
				}
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
