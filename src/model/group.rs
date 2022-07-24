use std::str::FromStr;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{serde::ts_seconds, DateTime, Utc};

use super::id::{Id, date_from_u64};
use super::user::{self, UserId};
use super::id::hex_id;

#[derive(Deserialize, Serialize,Debug)]
pub struct GroupId(
	#[serde(with = "hex_id")]
	pub(crate) u128
);

impl Id for GroupId {
	fn get_date(&self)-> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.0)
	}
}

#[derive(Deserialize,Debug)]
pub(crate) enum GroupInviteType {
	#[serde(rename = "Anyone")]
	Anyone,

	#[serde(rename = "Self")]
	SelfOnly,
}

#[derive(Deserialize)]
pub(crate) struct RawGroup {
	#[serde(rename = "Icon")]
	icon: String,

	#[serde(rename = "Invite")]
	invite: GroupInviteType,

	#[serde(rename = "Owner")]
	owner: UserId,

	#[serde(with = "ts_seconds", rename="LastChecked")]
    last_checked: DateTime<Utc>,

	#[serde(with = "ts_seconds", rename="LastContent")]
	last_content: DateTime<Utc>
}

#[derive(Debug)]
pub struct Group {
	id: GroupId,
	icon: String,
	invite: GroupInviteType,
	owner: UserId,
	last_checked: DateTime<Utc>,
	last_content: DateTime<Utc>,
}

#[async_trait]
trait GroupActions {
	async fn is_owner() -> bool;
	async fn delete() -> Result<(), ()>;
}

#[async_trait]
impl GroupActions for Group {
    async fn is_owner() -> bool {
        todo!()
    }

    async fn delete() -> Result<(), ()> {
        todo!()
    }
}

#[async_trait]
impl GroupActions for GroupId {
    async fn is_owner() -> bool {
        todo!()
    }

    async fn delete() -> Result<(), ()> {
        todo!()
    }
}