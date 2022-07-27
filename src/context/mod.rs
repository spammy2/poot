pub mod create_post;
pub mod me;
pub mod get_post;

use std::{collections::HashMap, sync::Arc};

use crate::{
    model::{
        client_user::{ClientUser, ClientUserRaw},
        group::RawGroup,
        id::{GroupId, PostId},
        post::{Post, PostRaw},
        user::UserRaw,
    },
    Auth, Events,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
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