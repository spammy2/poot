#![feature(trait_alias)]

mod model;
mod subscriptions;

use async_trait::async_trait;
use lazy_static::lazy_static;
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
use url::Url;

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

#[derive(Serialize)]
pub struct CreatePostBody {
    #[serde(rename = "text")]
    content: String,

    #[serde(skip_serializing)]
    group_id: Option<GroupId>,

    #[serde(skip_serializing)]
    images: Vec<reqwest::multipart::Part>,
}

impl From<String> for CreatePostBody {
    fn from(content: String) -> CreatePostBody {
        CreatePostBody {
            content,
            group_id: None,
            images: Vec::new(),
        }
    }
}

impl Context {
    pub async fn me(&self) -> Result<ClientUser, reqwest::Error> {
        #[derive(Deserialize)]
        struct GetResponse {
            groups: HashMap<GroupId, RawGroup>,
            user: ClientUserRaw,
        }

        let mut url = BASE_API_URL.join("me").unwrap();
        url.query_pairs_mut()
            .append_pair("ss", &self.simplesocket.get_secure_id()[..]);
        let val = self
            .client
            .get(url)
            .header("auth", self.auth.to_string())
            .send()
            .await?
            .json::<GetResponse>()
            .await?;

        let GetResponse { user, groups } = val;

        Ok(ClientUser {
            id: user.id,
            created_at: user.creation_time,
            infractions: user.infractions,
            last_important_update: user.last_important_update,
            last_login: user.last_login,
            logins: user.logins,
            description: user.description,
            followers: user.followers,
            following: user.following,
            groups: groups.into_iter().map(|(k, v)| v.into_group(k)).collect(),
            username: user.username,
            status: user.status,
            email: user.email,
        })
    }

    pub async fn create_post(&self, body: CreatePostBody) -> Result<Post, reqwest::Error> {
        let CreatePostBody {
            content,
            group_id,
            mut images,
        } = body;

        let form = images.drain(..).enumerate().fold(
            reqwest::multipart::Form::new().text("data", json!({ "text": content }).to_string()),
            |form, (i, image)| form.part(format!("image_{}", i), image),
        );

        let mut post_url = BASE_API_URL.join("posts/new").unwrap();
        if let Some(group_id) = group_id {
            post_url
                .query_pairs_mut()
                .append_pair("groupid", &group_id.to_string());
			println!("{}", group_id.to_string());
        }

        let post_id = self
			.client
			.post(post_url)
			.header("auth", self.auth.to_string())
            .multipart(form)
            .send()
            .await?
            .json::<PostId>()
            .await?;
        Ok(self.get_post(post_id).await?)
    }
    pub async fn get_post(&self, id: PostId) -> Result<Post, reqwest::Error> {
        #[derive(Deserialize)]
        struct GetResponse {
            users: Vec<UserRaw>,
            posts: Vec<PostRaw>,
        }

        let mut url = BASE_API_URL.join("posts").unwrap();
        url.query_pairs_mut().append_pair(
            "postid",
            serde_json::to_value(id).unwrap().as_str().unwrap(),
        );

        let val = self
            .client
            .get(url)
            .send()
            .await?
            .json::<GetResponse>()
            .await?;

        let GetResponse {
            mut posts,
            mut users,
        } = val;
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
    async fn on_post(&self, context: Arc<Context>, post: Post);
    async fn on_ready(&self, context: Arc<Context>);
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
