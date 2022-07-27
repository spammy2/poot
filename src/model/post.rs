use super::{
    id::{PostId, UserId},
    user::User,
};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize};
#[derive(Debug)]
pub struct Post {
    pub id: PostId,
    pub author: User,
    pub content: String,
}

#[derive(Deserialize)]
pub(crate) struct PostRaw {
    #[serde(rename = "_id")]
    pub id: PostId,
    #[serde(rename = "Text")]
    pub content: String,

    #[serde(rename = "HasMentions")]
    pub has_mentions: bool,

    #[serde(rename = "UserID")]
    pub user: UserId,

    #[serde(rename = "Timestamp", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}
