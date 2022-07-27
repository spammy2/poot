#![feature(trait_alias)]

pub mod context;
mod model;
mod subscriptions;

use async_trait::async_trait;
use context::Context;
use model::{
    client_user::{ClientUser, ClientUserRaw},
    group::RawGroup,
    id::{GroupId, PostId, UserId},
    post::{Post, PostRaw},
    user::UserRaw,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use simplesocket::{connect_socket, message::ConnectedResponse};
use std::{collections::HashMap, sync::Arc};
use subscriptions::general_update::*;
struct InitOptions {
    auth: Auth,
    events: Arc<dyn Events + Send + Sync>,
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

        ss.subscribe(
            GeneralUpdate {
                location: GeneralUpdateLocation::Home,
                groups: vec![],
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

#[derive(Clone)]
pub enum Auth {
    // Username{
    // 	username: String,
    // 	password: String,
    // },
    Token { user_id: String, token: String },
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
    async fn on_post(&self, context: Arc<Context>, post: Post) {
        // ...
    }
    async fn on_ready(&self, context: Arc<Context>) {
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
        async fn on_post(&self, context: Arc<Context>, post: Post) {
            println!("{:?}", post);
        }
        async fn on_ready(&self, context: Arc<Context>) {
            let post = context
                .create_post(CreatePostBody {
                    content: "it's like magic but also really shitty at the same time".to_string(),
                    // group_id: Some(GroupId::from("620722323f99d655b9afe2fa")),
                    group_id: None,
                    images: vec![],
                })
                .await;
            // tf it's not returning
            println!("FINISHED {:?}", post);
        }
    }

    #[tokio::test]
    pub async fn test() {
        dotenv().ok();
        let client = Client::new(
            Auth::Token {
                user_id: env::var("USER_ID").unwrap(),
                token: env::var("TOKEN").unwrap(),
            },
            BotEvents {},
        )
        .await;
    }
}
