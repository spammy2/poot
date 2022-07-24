use std::fmt::Display;

use serde::Deserialize;
use serde_json::Value;
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use super::{user::UserId, group::Group, status::Status};

#[derive(Deserialize,Debug)]
pub enum InfractionType {
	Ban,
}

#[derive(Deserialize,Debug)]
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
	id: UserId,
	created_at: DateTime<Utc>,
	infractions: Vec<Infraction>,
	last_important_update: DateTime<Utc>,
	last_login: DateTime<Utc>,
	logins: u32,
	description: String,
	followers: u32,
	following: u32,
	groups: Vec<Group>,
	username: String,
	status: Status,
	email: String,
}

#[derive(Deserialize)]
pub (crate) struct ClientUserRaw {
	#[serde(rename="_id")]
	id: UserId,
	#[serde(with = "ts_milliseconds", rename = "CreationTime")]
	creation_time: DateTime<Utc>,
	#[serde(rename="Email")]
	email: String,

}