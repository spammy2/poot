use std::str::FromStr;

use async_trait::async_trait;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::id::{GroupId, UserId};

#[derive(Deserialize, Debug)]
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

	#[serde(with = "ts_seconds", rename = "LastChecked")]
	last_checked: DateTime<Utc>,

	#[serde(with = "ts_seconds", rename = "LastContent")]
	last_content: DateTime<Utc>,
}

impl RawGroup {
	pub(crate) fn into_group(self, id: GroupId) -> Group {
		Group {
			id,
			icon: self.icon,
			invite: self.invite,
			owner: self.owner,
			last_checked: self.last_checked,
			last_content: self.last_content,
		}
	}
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
