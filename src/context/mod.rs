pub mod create_post;
pub mod me;
pub mod get_post;
pub mod create_chat;
pub mod get_chat;

use std::{sync::Arc};

use crate::{
    Auth, Events,
};
use lazy_static::lazy_static;
use url::Url;

#[derive(Clone)]
pub struct Context {
    pub(crate) simplesocket: Arc<simplesocket::Context>,
    pub client: Arc<reqwest::Client>,
    pub(crate) events: Arc<dyn Events + Send + Sync>,
    pub(crate) auth: Auth,
}

lazy_static! {
    pub static ref BASE_API_URL: Url = Url::parse("https://photop.exotek.co").unwrap();
}