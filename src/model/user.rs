use std::fmt::Display;

use serde::Deserialize;

use super::id::{Id, date_from_u64};
use super::id::hex_id;
use super::role::{RoleOrRoles, Role};

#[derive(Debug)]
pub struct User {
	id: UserId,
	username: String,
	roles: Vec<Role>,
}

#[derive(Deserialize, Debug)]
pub struct UserId(
	#[serde(with = "hex_id")]
	pub(crate) u128
);

impl Id for UserId {
	fn get_date(&self)-> chrono::DateTime<chrono::Utc> {
		date_from_u64(self.0)
	}
}


#[derive(Deserialize)]
pub struct UserRaw {
	pub id: UserId,
	pub username: String,
	pub roles: RoleOrRoles,
}

impl Into<User> for UserRaw {
	fn into(self) -> User {
		User {
			id: self.id,
			username: self.username,
			roles: self.roles.into(),
		}
	}
}