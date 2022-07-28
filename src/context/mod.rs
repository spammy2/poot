pub mod create_post;
pub mod me;
pub mod get_post;
pub mod create_chat;
pub mod get_chat;
pub mod connect_post;

use std::{sync::{Arc, Mutex}};

use crate::{
    Auth, Events, model::id::PostId,
};
use lazy_static::lazy_static;
use simplesocket::subscription::SubscriptionHandle;
use url::Url;

#[derive(Clone)]
pub struct Context {
    pub(crate) simplesocket: Arc<simplesocket::Context>,
    pub client: reqwest::Client,
    pub(crate) events: Arc<dyn Events + Send + Sync>,
    pub(crate) auth: Auth,
	pub(crate) posts: Arc<Mutex<Vec<PostId>>>,
	pub(crate) subscriptions: Subscriptions,
}

#[derive(Clone)]
pub(crate) struct Subscriptions {
	pub general_update: SubscriptionHandle,
}

lazy_static! {
    pub static ref BASE_API_URL: Url = Url::parse("https://photop.exotek.co").unwrap();
}