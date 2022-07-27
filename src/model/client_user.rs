use super::{group::Group, id::UserId, status::Status};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub enum InfractionType {
    Ban,
}

#[derive(Deserialize, Debug)]
pub struct Infraction {
    #[serde(rename = "ModID")]
    mod_id: UserId,

    #[serde(rename = "BanReason")]
    reason: String,

    // not gonna do jack about it for now
    #[serde(rename = "BanLength")]
    ban_length: Value,

    #[serde(with = "ts_milliseconds", rename = "Issued")]
    issued: DateTime<Utc>,

    #[serde(rename = "Type")]
    infraction_type: InfractionType,
}

#[derive(Debug)]
pub struct ClientUser {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
    pub infractions: Vec<Infraction>,
    pub last_important_update: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub logins: u32,
    pub description: String,
    pub followers: u32,
    pub following: u32,
    pub groups: Vec<Group>,
    pub username: String,
    pub status: Status,
    pub email: String,
}

#[derive(Deserialize)]
pub(crate) struct ClientUserRaw {
    #[serde(rename = "_id")]
    pub id: UserId,
    #[serde(with = "ts_milliseconds", rename = "CreationTime")]
    pub creation_time: DateTime<Utc>,
    #[serde(rename = "Email")]
    pub email: String,

    #[serde(rename = "Infractions")]
    pub infractions: Vec<Infraction>,

    #[serde(rename = "LastImportantUpdate", with = "ts_milliseconds")]
    pub last_important_update: DateTime<Utc>,

    #[serde(rename = "Logins")]
    pub logins: u32,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Followers")]
    pub followers: u32,

    #[serde(rename = "User")]
    pub username: String,

    #[serde(rename = "LastLogin")]
    pub last_login: DateTime<Utc>,

    #[serde(rename = "Following")]
    pub following: u32,

    #[serde(rename = "Status")]
    pub status: Status,
}
